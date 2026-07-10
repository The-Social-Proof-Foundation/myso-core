// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Chainlink V1 settlement uses CoinGecko as the localnet price feed stand-in.

use async_trait::async_trait;

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::direct_fetch;
use crate::sources::source_config::{SourceDomain, SourceHealth, SourceMetadata};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct ChainlinkAdapter;

impl ChainlinkAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ChainlinkAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TrustedSource for ChainlinkAdapter {
    fn id(&self) -> &str {
        "chainlink"
    }

    fn domain(&self) -> SourceDomain {
        SourceDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        def.resolver_kind == ResolverKind::PriceThreshold
            && matches!(
                &def.spec,
                ResolverSpec::PriceThreshold { source_id, .. } if source_id == "chainlink"
            )
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let ResolverSpec::PriceThreshold { asset, quote, .. } = &def.spec else {
            anyhow::bail!("chainlink: expected PriceThreshold spec");
        };
        // V1 localnet: CoinGecko-backed price feed stand-in for Chainlink.
        direct_fetch::fetch_coingecko_price(self.id(), asset, quote).await
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "chainlink via CoinGecko stand-in".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "Chainlink price oracle (CoinGecko stand-in in V1)".to_string(),
            domain: SourceDomain::Factual,
        }
    }
}
