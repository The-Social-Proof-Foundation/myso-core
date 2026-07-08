// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use chrono::Utc;
use myso_discovery_service_core::sources::http_client::HttpFetchClient;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct CoinbaseAdapter {
    client: HttpFetchClient,
}

impl CoinbaseAdapter {
    pub fn new() -> Self {
        Self {
            client: HttpFetchClient::new(),
        }
    }
}

impl Default for CoinbaseAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TrustedSource for CoinbaseAdapter {
    fn id(&self) -> &str {
        "coinbase"
    }

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        def.resolver_kind == ResolverKind::PriceThreshold
            && matches!(&def.spec, ResolverSpec::PriceThreshold { source_id, .. } if source_id == "coinbase")
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let ResolverSpec::PriceThreshold { asset, .. } = &def.spec else {
            anyhow::bail!("coinbase: expected PriceThreshold spec");
        };
        let product = match asset.as_str() {
            "bitcoin" => "BTC-USD",
            "ethereum" => "ETH-USD",
            other => return Err(anyhow::anyhow!("coinbase: unsupported asset {other}")),
        };
        let url = format!("https://api.coinbase.com/v2/prices/{product}/spot");
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
            message: "coinbase live".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "Coinbase public spot price API".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
