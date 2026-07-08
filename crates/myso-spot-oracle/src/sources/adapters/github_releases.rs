// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use chrono::Utc;
use myso_discovery_service_core::sources::http_client::HttpFetchClient;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct GithubReleasesAdapter {
    client: HttpFetchClient,
}

impl GithubReleasesAdapter {
    pub fn new() -> Self {
        Self {
            client: HttpFetchClient::new(),
        }
    }
}

impl Default for GithubReleasesAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TrustedSource for GithubReleasesAdapter {
    fn id(&self) -> &str {
        "github_releases"
    }

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        def.resolver_kind == ResolverKind::ReleasePublished
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let ResolverSpec::ReleasePublished { owner, repo, .. } = &def.spec else {
            anyhow::bail!("github_releases: expected ReleasePublished spec");
        };
        let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
        let fetched = self.client.get_text(&url).await?;
        let payload: serde_json::Value = serde_json::from_str(&fetched.body)?;
        Ok(SourceEvidence {
            adapter_id: self.id().to_string(),
            source_url: url,
            content_hash: fetched.content_hash,
            raw_response: Some(fetched.body),
            fetched_at: Utc::now(),
            payload,
        })
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "github_releases live".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "GitHub releases REST API".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
