// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use chrono::Utc;
use myso_discovery_service_core::api::EventsQuery;
use tracing::info;

use crate::api::AppState;

pub async fn run_rss_watcher_loop(state: Arc<AppState>) -> anyhow::Result<()> {
    let rows = if let Some(client) = &state.discovery_client {
        let summaries = client.list_sources().await?;
        summaries
            .into_iter()
            .filter(|s| s.enabled && s.adapter_type == "rss")
            .map(|s| (s.id, String::new()))
            .collect::<Vec<_>>()
    } else {
        state
            .store
            .list_enabled_sources()
            .await?
            .into_iter()
            .filter(|r| r.adapter_type == "rss")
            .filter_map(|r| {
                r.config
                    .get("feed_urls")
                    .and_then(|v| v.as_array())
                    .and_then(|a| a.first())
                    .and_then(|u| u.as_str())
                    .map(|url| (r.adapter_type.clone(), url.to_string()))
            })
            .collect()
    };

    if rows.is_empty() {
        return Ok(());
    }

    for (source_id, feed_url_hint) in rows {
        if state.cancel.is_cancelled() {
            break;
        }
        let feed_url = if !feed_url_hint.is_empty() {
            feed_url_hint
        } else if let Some(client) = &state.discovery_client {
            match client
                .get_events(&EventsQuery {
                    source_id: Some(source_id.clone()),
                    feed: None,
                    since: None,
                    query: None,
                    refresh: true,
                })
                .await
            {
                Ok(events) => events
                    .first()
                    .map(|e| e.provenance.source_url.clone())
                    .unwrap_or_default(),
                Err(err) => {
                    state.metrics.discovery_client_errors.inc();
                    tracing::warn!(source_id = %source_id, error = %err, "discovery RSS refresh failed");
                    continue;
                }
            }
        } else {
            continue;
        };

        if feed_url.is_empty() {
            continue;
        }

        let markets = state.store.list_markets(Some("waiting"), 100).await?;
        for market in markets {
            if let Some(def_id) = market.resolver_definition_id {
                let _ = state
                    .store
                    .enqueue_job(
                        "RssWake",
                        Some(market.id),
                        Some(def_id),
                        10,
                        Utc::now(),
                        serde_json::json!({ "feed_url": feed_url, "source_id": source_id }),
                    )
                    .await;
            }
        }
        state.metrics.rss_wake_total.inc();
        info!(feed = %feed_url, source_id = %source_id, "rss wake enqueued resolver jobs");
    }
    Ok(())
}
