// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::direct_fetch;
use crate::sources::source_config::{SourceDomain, SourceHealth, SourceMetadata};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct CoingeckoAdapter;

impl CoingeckoAdapter {
    pub fn new() -> Self {
        Self
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

    fn domain(&self) -> SourceDomain {
        SourceDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        def.resolver_kind == ResolverKind::PriceThreshold
            && matches!(
                &def.spec,
                ResolverSpec::PriceThreshold { source_id, .. } if source_id == "coingecko"
            )
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let ResolverSpec::PriceThreshold { asset, quote, .. } = &def.spec else {
            anyhow::bail!("coingecko: expected PriceThreshold spec");
        };
        direct_fetch::fetch_coingecko_price(self.id(), asset, quote).await
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "coingecko direct HTTP".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "CoinGecko simple price via direct HTTP".to_string(),
            domain: SourceDomain::Factual,
        }
    }
}
