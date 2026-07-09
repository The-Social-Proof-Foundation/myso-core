// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::discovery_resolve::{self, DiscoveryResolveCtx};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct CoingeckoAdapter {
    discovery_ctx: DiscoveryResolveCtx,
}

impl CoingeckoAdapter {
    pub fn new(discovery_ctx: DiscoveryResolveCtx) -> Self {
        Self { discovery_ctx }
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
        discovery_resolve::fetch_price(
            &self.discovery_ctx,
            self.id(),
            "coingecko-simple-price",
            asset,
            quote,
        )
        .await
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: self.discovery_ctx.uses_discovery(),
            message: if self.discovery_ctx.uses_discovery() {
                "coingecko via Discovery".to_string()
            } else {
                "discovery client not configured".to_string()
            },
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "CoinGecko simple price via Discovery /v1/prices".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
