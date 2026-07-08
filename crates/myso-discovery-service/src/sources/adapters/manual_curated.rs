// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::sources::{
    DiscoveryDomain, DiscoverySource, RawDiscoveryRecord, SourceConfig, SourceHealth,
    SourceMetadata,
};

pub struct ManualCuratedAdapter;

#[async_trait]
impl DiscoverySource for ManualCuratedAdapter {
    fn id(&self) -> &str {
        "manual_curated"
    }

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Creative
    }

    fn supports(&self, config: &SourceConfig) -> bool {
        config.adapter_type == "manual_curated" && config.enabled
    }

    async fn discover(&self, config: &SourceConfig) -> anyhow::Result<Vec<RawDiscoveryRecord>> {
        let mut records = Vec::new();
        for entry in &config.entries {
            records.push(RawDiscoveryRecord {
                external_source_url: entry.url.clone(),
                media_type: entry.media_type.clone(),
                title: entry.title.clone(),
                creator_x_handle: entry.creator_x_handle.clone(),
                trust_score: entry.trust_score.unwrap_or(config.trust_score),
                metadata: serde_json::json!({
                    "title": entry.title,
                    "source": "manual_curated",
                }),
            });
        }
        Ok(records)
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "manual_curated ready".into(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().into(),
            description: "YAML/JSON curated trusted source list".into(),
            domain: DiscoveryDomain::Creative,
        }
    }
}
