// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Stub provider for time-series metric observations (price, standings, etc.).

use async_trait::async_trait;
use chrono::Utc;

use crate::events::types::ProviderHealth;
use crate::knowledge::types::{
    DiscoveredKnowledge, DiscoveredMetric, DiscoveredObservation, KnowledgeProvider,
    ProviderContext,
};

pub struct MetricsObservationProvider;

#[async_trait]
impl KnowledgeProvider for MetricsObservationProvider {
    fn id(&self) -> &str {
        "metrics_observation_feed"
    }

    async fn discover(&self, _ctx: &ProviderContext) -> anyhow::Result<DiscoveredKnowledge> {
        Ok(DiscoveredKnowledge {
            metrics: vec![
                DiscoveredMetric {
                    id: "price_usd".to_string(),
                    key: "price_usd".to_string(),
                    unit: Some("usd".to_string()),
                    domain: "crypto".to_string(),
                    aggregation: Some("last".to_string()),
                    schema: serde_json::json!({"type": "number"}),
                    provenance: serde_json::json!({"source": "stub"}),
                },
                DiscoveredMetric {
                    id: "passing_yards".to_string(),
                    key: "passing_yards".to_string(),
                    unit: Some("yards".to_string()),
                    domain: "sports".to_string(),
                    aggregation: Some("sum".to_string()),
                    schema: serde_json::json!({"type": "integer"}),
                    provenance: serde_json::json!({"source": "stub"}),
                },
            ],
            observations: vec![DiscoveredObservation {
                metric_id: "price_usd".to_string(),
                entity_id: Some("sui".to_string()),
                event_id: None,
                observed_at: Utc::now(),
                value: serde_json::json!(null),
                provenance: serde_json::json!({"source": "stub", "note": "no live feed in localnet"}),
            }],
            ..Default::default()
        })
    }

    async fn health(&self) -> ProviderHealth {
        ProviderHealth::ok("metrics_observation_feed stub ready")
    }
}
