// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deterministic event provider for unit tests.

use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::events::types::normalize_discovered_event;
use crate::events::{
    DiscoveredEvent, EventCategory, EventEntity, EventProvider, EventResolverHints,
    ProviderContext, ProviderHealth,
};

pub struct StubEventProvider {
    pub events: Vec<DiscoveredEvent>,
}

impl StubEventProvider {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn with_events(events: Vec<DiscoveredEvent>) -> Self {
        Self { events }
    }

    pub fn default_test_events() -> Self {
        let end_at = Utc::now() + Duration::days(365);
        Self::with_events(vec![DiscoveredEvent {
            external_id: "stub_fifa_2026".to_string(),
            label: "Stub FIFA World Cup 2026".to_string(),
            category: EventCategory::Sports,
            start_at: None,
            end_at,
            keywords: vec![
                "fifa".to_string(),
                "world cup".to_string(),
                "messi".to_string(),
                "messy".to_string(),
                "mbappe".to_string(),
                "muppet".to_string(),
            ],
            entities: vec![
                EventEntity {
                    name: "lionel messi".to_string(),
                    aliases: vec!["messi".to_string(), "messy".to_string()],
                    role: Some("player".to_string()),
                },
                EventEntity {
                    name: "kylian mbappe".to_string(),
                    aliases: vec!["mbappe".to_string(), "muppet".to_string()],
                    role: Some("player".to_string()),
                },
            ],
            resolver_hints: EventResolverHints {
                feed_url: Some("https://example.com/rss.xml".to_string()),
                match_predicate: Some("world cup".to_string()),
                preferred_source_keys: vec!["rss_event".to_string()],
            },
            provenance: serde_json::json!({"source": "stub"}),
            priority: 50,
            enabled: true,
        }])
    }
}

impl Default for StubEventProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventProvider for StubEventProvider {
    fn id(&self) -> &str {
        "stub"
    }

    async fn discover(&self, _ctx: &ProviderContext) -> anyhow::Result<Vec<DiscoveredEvent>> {
        Ok(self
            .events
            .iter()
            .cloned()
            .filter_map(normalize_discovered_event)
            .collect())
    }

    async fn health(&self) -> ProviderHealth {
        ProviderHealth::ok("stub ready")
    }
}
