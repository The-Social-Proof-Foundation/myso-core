// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::discovery_resolve::{self, DiscoveryResolveCtx};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct RssEventAdapter {
    discovery_ctx: DiscoveryResolveCtx,
}

impl RssEventAdapter {
    pub fn new(discovery_ctx: DiscoveryResolveCtx) -> Self {
        Self { discovery_ctx }
    }
}

#[async_trait]
impl TrustedSource for RssEventAdapter {
    fn id(&self) -> &str {
        "rss_event"
    }

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        def.resolver_kind == ResolverKind::EventOccurrence
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let ResolverSpec::EventOccurrence { feed_url, source_id, .. } = &def.spec else {
            anyhow::bail!("rss_event: expected EventOccurrence spec");
        };
        if feed_url.is_empty() {
            anyhow::bail!("rss_event: missing feed_url");
        }
        discovery_resolve::fetch_events(&self.discovery_ctx, self.id(), source_id, feed_url).await
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: self.discovery_ctx.uses_discovery(),
            message: if self.discovery_ctx.uses_discovery() {
                "rss_event via Discovery".to_string()
            } else {
                "discovery client not configured".to_string()
            },
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "RSS feed events via Discovery /v1/events".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
