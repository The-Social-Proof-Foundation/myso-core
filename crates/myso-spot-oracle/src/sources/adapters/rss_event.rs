// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::direct_fetch;
use crate::sources::source_config::{SourceDomain, SourceHealth, SourceMetadata};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct RssEventAdapter;

impl RssEventAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RssEventAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TrustedSource for RssEventAdapter {
    fn id(&self) -> &str {
        "rss_event"
    }

    fn domain(&self) -> SourceDomain {
        SourceDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        def.resolver_kind == ResolverKind::EventOccurrence
            && matches!(
                &def.spec,
                ResolverSpec::EventOccurrence { source_id, .. }
                    if source_id.is_empty() || source_id == "rss_event"
            )
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let ResolverSpec::EventOccurrence { feed_url, .. } = &def.spec else {
            anyhow::bail!("rss_event: expected EventOccurrence spec");
        };
        if feed_url.is_empty() {
            anyhow::bail!("rss_event: missing feed_url");
        }
        direct_fetch::fetch_rss_events(self.id(), feed_url).await
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "rss_event direct HTTP".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "RSS feed events via direct HTTP".to_string(),
            domain: SourceDomain::Factual,
        }
    }
}
