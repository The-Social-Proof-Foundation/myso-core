// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use chrono::Utc;
use myso_discovery_service_core::sources::http_client::HttpFetchClient;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverSpec};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct HttpOfficialAdapter {
    client: HttpFetchClient,
}

impl HttpOfficialAdapter {
    pub fn new() -> Self {
        Self {
            client: HttpFetchClient::new(),
        }
    }
}

impl Default for HttpOfficialAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TrustedSource for HttpOfficialAdapter {
    fn id(&self) -> &str {
        "http_official"
    }

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        matches!(
            &def.spec,
            ResolverSpec::CustomHttp { source_id, .. }
                | ResolverSpec::PriceThreshold { source_id, .. }
                if source_id == "http_official"
        )
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let url = match &def.spec {
            ResolverSpec::CustomHttp { url, .. } => url.clone(),
            ResolverSpec::PriceThreshold { asset, quote, .. } => format!(
                "https://api.coingecko.com/api/v3/simple/price?ids={asset}&vs_currencies={quote}"
            ),
            _ => anyhow::bail!("http_official: unsupported spec"),
        };
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
            message: "http_official live".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "Generic HTTP official JSON endpoint".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
