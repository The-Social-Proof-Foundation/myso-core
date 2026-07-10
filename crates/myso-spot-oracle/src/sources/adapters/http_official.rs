// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::direct_fetch;
use crate::sources::source_config::{SourceDomain, SourceHealth, SourceMetadata};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct HttpOfficialAdapter;

impl HttpOfficialAdapter {
    pub fn new() -> Self {
        Self
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

    fn domain(&self) -> SourceDomain {
        SourceDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        matches!(
            def.resolver_kind,
            ResolverKind::CustomHttp | ResolverKind::PriceThreshold | ResolverKind::EventOccurrence
        ) && matches!(
            &def.spec,
            ResolverSpec::CustomHttp { source_id, .. }
                | ResolverSpec::PriceThreshold { source_id, .. }
                | ResolverSpec::EventOccurrence { source_id, .. }
                if source_id == "http_official" || source_id.is_empty()
        )
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        match &def.spec {
            ResolverSpec::CustomHttp { url, .. } => {
                if url.is_empty() {
                    anyhow::bail!("http_official: missing url");
                }
                direct_fetch::fetch_http_json(self.id(), url).await
            }
            ResolverSpec::PriceThreshold { asset, quote, .. } => {
                direct_fetch::fetch_coingecko_price(self.id(), asset, quote).await
            }
            ResolverSpec::EventOccurrence { feed_url, .. } => {
                if feed_url.is_empty() {
                    anyhow::bail!("http_official: missing feed_url");
                }
                direct_fetch::fetch_rss_events(self.id(), feed_url).await
            }
            _ => anyhow::bail!("http_official: unsupported resolver spec"),
        }
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "http_official direct HTTP".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "HTTP official sources via direct fetch".to_string(),
            domain: SourceDomain::Factual,
        }
    }
}
