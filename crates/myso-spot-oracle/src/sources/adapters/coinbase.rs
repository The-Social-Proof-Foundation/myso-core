// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::discovery_resolve::{self, DiscoveryResolveCtx};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct CoinbaseAdapter {
    discovery_ctx: DiscoveryResolveCtx,
}

impl CoinbaseAdapter {
    pub fn new(discovery_ctx: DiscoveryResolveCtx) -> Self {
        Self { discovery_ctx }
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
        let ResolverSpec::PriceThreshold { asset, quote, .. } = &def.spec else {
            anyhow::bail!("coinbase: expected PriceThreshold spec");
        };
        let source_id = match asset.as_str() {
            "bitcoin" => "coinbase-btc-spot",
            "ethereum" => "coinbase-eth-spot",
            other => return Err(anyhow::anyhow!("coinbase: unsupported asset {other}")),
        };
        discovery_resolve::fetch_price(&self.discovery_ctx, self.id(), source_id, asset, quote).await
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: self.discovery_ctx.uses_discovery(),
            message: if self.discovery_ctx.uses_discovery() {
                "coinbase via Discovery".to_string()
            } else {
                "discovery client not configured".to_string()
            },
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "Coinbase spot price via Discovery /v1/prices".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
