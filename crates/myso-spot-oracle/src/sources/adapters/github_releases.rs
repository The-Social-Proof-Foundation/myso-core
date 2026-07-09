// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::discovery_resolve::{self, DiscoveryResolveCtx};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct GithubReleasesAdapter {
    discovery_ctx: DiscoveryResolveCtx,
}

impl GithubReleasesAdapter {
    pub fn new(discovery_ctx: DiscoveryResolveCtx) -> Self {
        Self { discovery_ctx }
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
        let source_id = format!("{repo}-releases");
        discovery_resolve::fetch_release(&self.discovery_ctx, self.id(), &source_id, owner, repo).await
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: self.discovery_ctx.uses_discovery(),
            message: if self.discovery_ctx.uses_discovery() {
                "github_releases via Discovery".to_string()
            } else {
                "discovery client not configured".to_string()
            },
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "GitHub releases via Discovery /v1/releases".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
