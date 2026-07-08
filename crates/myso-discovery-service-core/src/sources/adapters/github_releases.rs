// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use async_trait::async_trait;

use crate::sources::http_client::HttpFetchClient;
use crate::sources::{
    DiscoveryDomain, DiscoverySource, RawDiscoveryRecord, SourceConfig, SourceHealth,
    SourceMetadata,
};

/// Real `DiscoverySource` that calls `GET https://api.github.com/repos/{owner}/{repo}/releases`
/// and emits one record per release (tag + html_url + content hash of the API response).
pub struct GithubReleasesAdapter {
    client: HttpFetchClient,
}

impl GithubReleasesAdapter {
    pub fn new() -> Self {
        Self {
            client: HttpFetchClient::new(),
        }
    }

    pub fn with_client(client: HttpFetchClient) -> Self {
        Self { client }
    }

    fn releases_url(config: &SourceConfig) -> anyhow::Result<String> {
        let owner = config
            .config
            .owner
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("github_releases: missing config.owner"))?;
        let repo = config
            .config
            .repo
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("github_releases: missing config.repo"))?;
        Ok(format!("https://api.github.com/repos/{owner}/{repo}/releases"))
    }
}

impl Default for GithubReleasesAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DiscoverySource for GithubReleasesAdapter {
    fn id(&self) -> &str {
        "github_releases"
    }

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Factual
    }

    fn supports(&self, config: &SourceConfig) -> bool {
        config.adapter_type == "github_releases"
            && config.enabled
            && config.config.owner.is_some()
            && config.config.repo.is_some()
    }

    async fn discover(&self, config: &SourceConfig) -> anyhow::Result<Vec<RawDiscoveryRecord>> {
        let url = Self::releases_url(config)?;
        let fetched = self
            .client
            .get_text_authed(&url, config.config.api_key_env.as_deref())
            .await?;
        let releases: Vec<serde_json::Value> = serde_json::from_str(&fetched.body)
            .with_context(|| format!("decoding github releases JSON from {url}"))?;
        let mut records = Vec::with_capacity(releases.len());
        for release in releases {
            let tag = release
                .get("tag_name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let html_url = release
                .get("html_url")
                .and_then(|v| v.as_str())
                .unwrap_or(&url)
                .to_string();
            let name = release.get("name").and_then(|v| v.as_str()).map(String::from);
            let published_at = release
                .get("published_at")
                .and_then(|v| v.as_str())
                .map(String::from);
            records.push(RawDiscoveryRecord {
                external_source_url: html_url.clone(),
                media_type: "text/html".to_string(),
                title: name.or(Some(tag.clone())),
                creator_x_handle: None,
                trust_score: config.trust_score,
                content_hash: Some(fetched.content_hash.clone()),
                metadata: serde_json::json!({
                    "tag": tag,
                    "html_url": html_url,
                    "published_at": published_at,
                    "api_url": url,
                }),
            });
        }
        Ok(records)
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "github_releases adapter ready".into(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().into(),
            description: "Real GitHub releases discovery (live REST API)".into(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
