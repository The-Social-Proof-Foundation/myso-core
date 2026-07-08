// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use sha2::{Digest, Sha256};

use crate::sources::http_client::HttpFetchClient;
use crate::sources::{
    DiscoveryDomain, DiscoverySource, RawDiscoveryRecord, SourceConfig, SourceHealth,
    SourceMetadata,
};

/// Real `DiscoverySource` for any official HTTP JSON endpoint: `GET {api_base_url}{poll_path}`.
/// Emits a single record capturing the fetched URL, the raw JSON snapshot, and its content hash.
pub struct HttpOfficialAdapter {
    client: HttpFetchClient,
}

impl HttpOfficialAdapter {
    pub fn new() -> Self {
        Self {
            client: HttpFetchClient::new(),
        }
    }

    pub fn with_client(client: HttpFetchClient) -> Self {
        Self { client }
    }

    fn fetch_url(config: &SourceConfig) -> anyhow::Result<String> {
        let base = config
            .config
            .api_base_url
            .as_deref()
            .ok_or_else(|| anyhow!("http_official: missing config.api_base_url"))?;
        let path = config
            .config
            .poll_path
            .as_deref()
            .ok_or_else(|| anyhow!("http_official: missing config.poll_path"))?;
        // poll_path may already start with '/', and api_base_url may end with '/'.
        let joined = match (base.ends_with('/'), path.starts_with('/')) {
            (true, true) => format!("{}{}", base, &path[1..]),
            (false, false) => format!("{base}/{path}"),
            _ => format!("{base}{path}"),
        };
        Ok(joined)
    }
}

impl Default for HttpOfficialAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DiscoverySource for HttpOfficialAdapter {
    fn id(&self) -> &str {
        "http_official"
    }

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Factual
    }

    fn supports(&self, config: &SourceConfig) -> bool {
        config.adapter_type == "http_official"
            && config.enabled
            && config.config.api_base_url.is_some()
            && config.config.poll_path.is_some()
    }

    async fn discover(&self, config: &SourceConfig) -> anyhow::Result<Vec<RawDiscoveryRecord>> {
        let url = Self::fetch_url(config)?;
        let value = self
            .client
            .get_json(&url)
            .await
            .with_context(|| format!("http_official fetch {url}"))?;
        let body = serde_json::to_string(&value).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(body.as_bytes());
        let content_hash = hex::encode(hasher.finalize());
        Ok(vec![RawDiscoveryRecord {
            external_source_url: url.clone(),
            media_type: "application/json".to_string(),
            title: Some(config.id.clone()),
            creator_x_handle: None,
            trust_score: config.trust_score,
            content_hash: Some(content_hash),
            metadata: value,
        }])
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "http_official adapter ready".into(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().into(),
            description: "Real official HTTP JSON endpoint discovery (live fetch)".into(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
