// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::events::types::{EventEntity, ProviderHealth};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredEntity {
    pub id: String,
    pub kind: String,
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub external_refs: serde_json::Value,
    #[serde(default)]
    pub provenance: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredCompetition {
    pub id: String,
    pub kind: String,
    pub label: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub recurrence_rule: Option<String>,
    #[serde(default)]
    pub provenance: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredEvent {
    pub id: String,
    #[serde(default)]
    pub competition_id: Option<String>,
    pub label: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: DateTime<Utc>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub entities: Vec<EventEntity>,
    #[serde(default)]
    pub feed_url: Option<String>,
    #[serde(default)]
    pub match_predicate: Option<String>,
    #[serde(default)]
    pub preferred_source_keys: Vec<String>,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub provenance: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredMetric {
    pub id: String,
    pub key: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub aggregation: Option<String>,
    #[serde(default)]
    pub schema: serde_json::Value,
    #[serde(default)]
    pub provenance: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredObservation {
    pub metric_id: String,
    #[serde(default)]
    pub entity_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
    pub observed_at: DateTime<Utc>,
    pub value: serde_json::Value,
    #[serde(default)]
    pub provenance: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscoveredRelationship {
    pub subject_id: String,
    pub object_id: String,
    pub rel_type: String,
    #[serde(default)]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(default)]
    pub valid_to: Option<DateTime<Utc>>,
    #[serde(default)]
    pub provenance: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscoveredKnowledge {
    #[serde(default)]
    pub entities: Vec<DiscoveredEntity>,
    #[serde(default)]
    pub competitions: Vec<DiscoveredCompetition>,
    #[serde(default)]
    pub events: Vec<DiscoveredEvent>,
    #[serde(default)]
    pub metrics: Vec<DiscoveredMetric>,
    #[serde(default)]
    pub observations: Vec<DiscoveredObservation>,
    #[serde(default)]
    pub relationships: Vec<DiscoveredRelationship>,
}

#[derive(Debug, Clone)]
pub struct ProviderContext {
    pub provider_key: String,
    pub config: serde_json::Value,
    pub live_fetch: bool,
}

#[async_trait]
pub trait KnowledgeProvider: Send + Sync {
    fn id(&self) -> &str;
    async fn discover(&self, ctx: &ProviderContext) -> anyhow::Result<DiscoveredKnowledge>;
    async fn health(&self) -> ProviderHealth;
}
