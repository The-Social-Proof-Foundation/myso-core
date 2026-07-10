// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::direct_fetch;
use crate::sources::source_config::{SourceDomain, SourceHealth, SourceMetadata};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct GithubReleasesAdapter;

impl GithubReleasesAdapter {
    pub fn new() -> Self {
        Self
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

    fn domain(&self) -> SourceDomain {
        SourceDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        def.resolver_kind == ResolverKind::ReleasePublished
            && matches!(
                &def.spec,
                ResolverSpec::ReleasePublished { source_id, .. }
                    if source_id.is_empty() || source_id == "github_releases"
            )
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let ResolverSpec::ReleasePublished { owner, repo, .. } = &def.spec else {
            anyhow::bail!("github_releases: expected ReleasePublished spec");
        };
        direct_fetch::fetch_github_release(self.id(), owner, repo).await
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "github_releases direct HTTP".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "GitHub releases via direct HTTP".to_string(),
            domain: SourceDomain::Factual,
        }
    }
}
