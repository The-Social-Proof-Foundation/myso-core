// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::sources::{
    normalize_media_type, ContentKind, DiscoveryDomain, DiscoverySource, RawDiscoveryRecord,
    SourceConfig, SourceHealth, SourceMetadata,
};

/// Replays static YAML `entries`. **Test/seed only** — gated behind
/// `DISCOVERY_USE_MANUAL_CURATED=1` at use sites; never the default E2E path.
/// Real factual sources use `rss` / `github_releases` / `http_official`.
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
        if config.adapter_type != "manual_curated" || !config.enabled {
            return false;
        }
        // Test/seed only — refuse unless explicitly opted in.
        matches!(
            std::env::var("DISCOVERY_USE_MANUAL_CURATED").as_deref(),
            Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
        )
    }

    async fn discover(&self, config: &SourceConfig) -> anyhow::Result<Vec<RawDiscoveryRecord>> {
        let mut records = Vec::new();
        for entry in &config.entries {
            let (media_type, derived_kind) = normalize_media_type(&entry.media_type);
            let content_kind = config.content_kind.unwrap_or(derived_kind);
            // Prefer media when entry looks like media even if source override says text.
            let content_kind = if derived_kind == ContentKind::Media {
                ContentKind::Media
            } else {
                content_kind
            };
            records.push(RawDiscoveryRecord {
                external_source_url: entry.url.clone(),
                media_type,
                content_kind,
                title: entry.title.clone(),
                creator_x_handle: entry.creator_x_handle.clone(),
                trust_score: entry.trust_score.unwrap_or(config.trust_score),
                content_hash: None,
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
            description: "YAML/JSON curated trusted source list (test/seed only)".into(),
            domain: DiscoveryDomain::Creative,
        }
    }
}
