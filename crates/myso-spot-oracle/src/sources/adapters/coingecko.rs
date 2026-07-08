// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use chrono::Utc;
use myso_discovery_service_core::sources::http_client::HttpFetchClient;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct CoingeckoAdapter {
    client: HttpFetchClient,
}

impl CoingeckoAdapter {
    pub fn new() -> Self {
        Self {
            client: HttpFetchClient::new(),
        }
    }
}

impl Default for CoingeckoAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TrustedSource for CoingeckoAdapter {
    fn id(&self) -> &str {
        "coingecko"
    }

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        def.resolver_kind == ResolverKind::PriceThreshold
            && matches!(&def.spec, ResolverSpec::PriceThreshold { source_id, .. } if source_id == "coingecko")
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let ResolverSpec::PriceThreshold { asset, quote, .. } = &def.spec else {
            anyhow::bail!("coingecko: expected PriceThreshold spec");
        };
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={asset}&vs_currencies={quote}"
        );
        let fetched = self.client.get_text(&url).await?;
        let payload: serde_json::Value = serde_json::from_str(&fetched.body)?;
        Ok(SourceEvidence {
            adapter_id: self.id().to_string(),
            source_url: url,
            content_hash: fetched.content_hash,
            raw_response: Some(fetched.body),
            fetched_at: Utc::now(),
            payload,
        })
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "coingecko live".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "CoinGecko simple price API".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
