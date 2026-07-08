// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::sources::{
    DiscoveryDomain, DiscoverySource, RawDiscoveryRecord, SourceConfig, SourceHealth,
    SourceMetadata,
};

/// Disabled-by-default placeholder for creative/out-of-scope factual sources.
/// Returns empty polls and reports disabled health. Never used in V1 E2E.
pub struct StubAdapter {
    adapter_id: String,
    domain: DiscoveryDomain,
}

impl StubAdapter {
    pub fn new(adapter_id: impl Into<String>, domain: DiscoveryDomain) -> Self {
        Self {
            adapter_id: adapter_id.into(),
            domain,
        }
    }
}

#[async_trait]
impl DiscoverySource for StubAdapter {
    fn id(&self) -> &str {
        &self.adapter_id
    }

    fn domain(&self) -> DiscoveryDomain {
        self.domain
    }

    fn supports(&self, config: &SourceConfig) -> bool {
        config.adapter_type == self.adapter_id && config.enabled
    }

    async fn discover(&self, _config: &SourceConfig) -> anyhow::Result<Vec<RawDiscoveryRecord>> {
        Ok(vec![])
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: false,
            message: format!("{} adapter disabled (stub)", self.adapter_id),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.adapter_id.clone(),
            description: format!("Stub adapter for {}", self.adapter_id),
            domain: self.domain,
        }
    }
}
