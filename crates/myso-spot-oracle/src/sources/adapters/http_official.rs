// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverSpec};
use crate::sources::discovery_resolve::{self, DiscoveryResolveCtx};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct HttpOfficialAdapter {
    discovery_ctx: DiscoveryResolveCtx,
}

impl HttpOfficialAdapter {
    pub fn new(discovery_ctx: DiscoveryResolveCtx) -> Self {
        Self { discovery_ctx }
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
        match &def.spec {
            ResolverSpec::PriceThreshold { asset, quote, .. } => {
                discovery_resolve::fetch_price(
                    &self.discovery_ctx,
                    self.id(),
                    "coingecko-simple-price",
                    asset,
                    quote,
                )
                .await
            }
            ResolverSpec::CustomHttp { source_id, .. } => {
                discovery_resolve::fetch_events(&self.discovery_ctx, self.id(), source_id, source_id)
                    .await
            }
            _ => anyhow::bail!("http_official: unsupported spec"),
        }
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: self.discovery_ctx.uses_discovery(),
            message: if self.discovery_ctx.uses_discovery() {
                "http_official via Discovery".to_string()
            } else {
                "discovery client not configured".to_string()
            },
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "HTTP official sources via Discovery factual API".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
