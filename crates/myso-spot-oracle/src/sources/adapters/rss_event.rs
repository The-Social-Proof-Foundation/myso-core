// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use chrono::Utc;
use myso_discovery_service_core::sources::http_client::HttpFetchClient;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct RssEventAdapter {
    client: HttpFetchClient,
}

impl RssEventAdapter {
    pub fn new() -> Self {
        Self {
            client: HttpFetchClient::new(),
        }
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

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        def.resolver_kind == ResolverKind::EventOccurrence
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        let ResolverSpec::EventOccurrence { feed_url, .. } = &def.spec else {
            anyhow::bail!("rss_event: expected EventOccurrence spec");
        };
        if feed_url.is_empty() {
            anyhow::bail!("rss_event: missing feed_url");
        }
        let fetched = self.client.get_text(feed_url).await?;
        Ok(SourceEvidence {
            adapter_id: self.id().to_string(),
            source_url: feed_url.clone(),
            content_hash: fetched.content_hash,
            raw_response: Some(fetched.body.clone()),
            fetched_at: Utc::now(),
            payload: serde_json::json!({ "feed_len": fetched.body.len() }),
        })
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "rss_event live".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "RSS feed event verification at maturity".to_string(),
            domain: DiscoveryDomain::Factual,
        }
    }
}
