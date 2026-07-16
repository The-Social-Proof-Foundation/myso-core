// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Background sync loop for event providers.

use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use tracing::{error, info, warn};

use crate::api::AppState;
use crate::events::{
    build_default_event_provider_registry, ProviderContext,
};
use crate::store::events::EventProviderRow;

pub async fn bootstrap_event_registry(state: &AppState) -> anyhow::Result<()> {
    sync_all_due_providers(state, true).await?;
    reload_registry(state).await?;
    Ok(())
}

pub async fn run_event_provider_sync_loop(state: Arc<AppState>) -> anyhow::Result<()> {
    let tick = Duration::from_secs(state.args.event_sync_interval_secs);
    loop {
        if state.cancel.is_cancelled() {
            break;
        }
        if let Err(err) = sync_all_due_providers(&state, false).await {
            error!(error = %err, "event provider sync round failed");
        }
        if let Err(err) = reload_registry(&state).await {
            error!(error = %err, "event registry reload failed");
        }
        tokio::time::sleep(tick).await;
    }
    Ok(())
}

pub async fn sync_provider(state: &AppState, provider_key: &str) -> anyhow::Result<usize> {
    let row = state
        .store
        .get_event_provider(provider_key)
        .await?
        .ok_or_else(|| anyhow::anyhow!("event provider {provider_key} not found"))?;
    sync_provider_row(state, &row).await
}

async fn sync_all_due_providers(state: &AppState, force: bool) -> anyhow::Result<()> {
    let providers = state.store.list_enabled_event_providers().await?;
    let registry = build_default_event_provider_registry();
    for row in providers {
        if !force && !is_due(&row) {
            continue;
        }
        if let Err(err) = sync_provider_row_with_registry(state, &row, &registry).await {
            warn!(provider = %row.provider_key, error = %err, "event provider sync failed");
        }
    }
    Ok(())
}

fn is_due(row: &EventProviderRow) -> bool {
    let Some(last) = row.last_sync_at else {
        return true;
    };
    let elapsed = Utc::now().signed_duration_since(last);
    elapsed.num_seconds() >= row.poll_interval_secs as i64
}

async fn sync_provider_row(state: &AppState, row: &EventProviderRow) -> anyhow::Result<usize> {
    let registry = build_default_event_provider_registry();
    sync_provider_row_with_registry(state, row, &registry).await
}

async fn sync_provider_row_with_registry(
    state: &AppState,
    row: &EventProviderRow,
    registry: &crate::events::EventProviderRegistry,
) -> anyhow::Result<usize> {
    let provider = registry
        .get(&row.provider_type)
        .ok_or_else(|| anyhow::anyhow!("unknown event provider type {}", row.provider_type))?;
    let ctx = ProviderContext {
        provider_key: row.provider_key.clone(),
        config: row.config.clone(),
        live_fetch: state.args.live_sources,
    };
    match provider.discover(&ctx).await {
        Ok(events) => {
            let count = state
                .store
                .upsert_discovered_events(&row.provider_key, &events)
                .await?;
            state
                .store
                .update_event_provider_sync_status(
                    &row.provider_key,
                    "ok",
                    true,
                    &format!("synced {count} events"),
                )
                .await?;
            state
                .metrics
                .event_provider_sync_total
                .with_label_values(&[&row.provider_key, "ok"])
                .inc();
            info!(provider = %row.provider_key, count, "event provider sync ok");
            Ok(count)
        }
        Err(err) => {
            let msg = err.to_string();
            state
                .store
                .update_event_provider_sync_status(&row.provider_key, "error", false, &msg)
                .await?;
            state
                .metrics
                .event_provider_sync_total
                .with_label_values(&[&row.provider_key, "error"])
                .inc();
            Err(err)
        }
    }
}

pub async fn reload_registry(state: &AppState) -> anyhow::Result<()> {
    let rows = state.store.list_active_scheduled_events().await?;
    let count = rows.len() as i64;
    state.event_registry.reload(rows);
    state.metrics.scheduled_events_active.set(count);
    crate::knowledge::sync::reload_knowledge_graph(state).await?;
    Ok(())
}
