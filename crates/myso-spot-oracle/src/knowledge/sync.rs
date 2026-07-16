// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Knowledge provider sync — dual-writes graph objects alongside event providers.

use std::sync::Arc;

use tracing::{info, warn};

use crate::api::AppState;
use crate::knowledge::{
    build_default_knowledge_provider_registry, DiscoveredKnowledge, KnowledgeProvider,
};

pub async fn bootstrap_knowledge_graph(state: &AppState) -> anyhow::Result<()> {
    sync_default_knowledge_providers(state).await?;
    reload_knowledge_graph(state).await?;
    Ok(())
}

pub async fn sync_default_knowledge_providers(state: &AppState) -> anyhow::Result<()> {
    let registry = build_default_knowledge_provider_registry();
    let configs: Vec<(String, serde_json::Value)> = vec![
        (
            "entity-seed".to_string(),
            serde_json::json!({
                "entities_file": "crates/myso-spot-oracle/config/entity_seed.localnet.yaml"
            }),
        ),
        (
            "relationship-seed".to_string(),
            serde_json::json!({
                "relationships_file": "crates/myso-spot-oracle/config/relationship_seed.localnet.yaml"
            }),
        ),
        (
            "metrics-observation-stub".to_string(),
            serde_json::json!({}),
        ),
    ];

    let mut failures = Vec::new();
    for (provider_key, config) in configs {
        let provider_type = match provider_key.as_str() {
            "entity-seed" => "entity_seed",
            "relationship-seed" => "relationship_seed",
            _ => "metrics_observation_feed",
        };
        let Some(provider) = registry.get(provider_type) else {
            continue;
        };
        if let Err(err) = sync_knowledge_provider(state, provider, &provider_key, &config).await {
            warn!(provider = %provider_key, error = %err, "knowledge provider sync failed");
            failures.push((provider_key, err));
        }
    }
    if !failures.is_empty() {
        let summary: Vec<String> = failures
            .into_iter()
            .map(|(k, e)| format!("{k}: {e}"))
            .collect();
        anyhow::bail!(
            "knowledge provider bootstrap failed: {}",
            summary.join("; ")
        );
    }
    Ok(())
}

async fn sync_knowledge_provider(
    state: &AppState,
    provider: Arc<dyn KnowledgeProvider>,
    provider_key: &str,
    config: &serde_json::Value,
) -> anyhow::Result<usize> {
    let ctx = crate::knowledge::ProviderContext {
        provider_key: provider_key.to_string(),
        config: config.clone(),
        live_fetch: state.args.live_sources,
    };
    let knowledge = provider.discover(&ctx).await?;
    let count = upsert_with_metrics(state, provider_key, &knowledge).await?;
    info!(provider = %provider_key, count, "knowledge provider sync ok");
    Ok(count)
}

async fn upsert_with_metrics(
    state: &AppState,
    provider_key: &str,
    knowledge: &DiscoveredKnowledge,
) -> anyhow::Result<usize> {
    let count = crate::store::knowledge::upsert_discovered_knowledge(
        state.store.pool(),
        provider_key,
        knowledge,
    )
    .await?;
    for object_type in [
        ("entities", knowledge.entities.len()),
        ("competitions", knowledge.competitions.len()),
        ("events", knowledge.events.len()),
        ("metrics", knowledge.metrics.len()),
        ("observations", knowledge.observations.len()),
        ("relationships", knowledge.relationships.len()),
    ] {
        if object_type.1 > 0 {
            state
                .metrics
                .knowledge_sync_total
                .with_label_values(&[provider_key, object_type.0])
                .inc_by(object_type.1 as u64);
        }
    }
    for obs in &knowledge.observations {
        state
            .metrics
            .observation_ingest_total
            .with_label_values(&[&obs.metric_id, "unknown"])
            .inc();
    }
    Ok(count)
}

pub async fn reload_knowledge_graph(state: &AppState) -> anyhow::Result<()> {
    let mut graph = crate::knowledge::KnowledgeGraph::new();
    graph.reload_from_event_registry(&state.event_registry);
    if let Ok(mut guard) = state.knowledge_graph.write() {
        *guard = graph;
    }
    Ok(())
}
