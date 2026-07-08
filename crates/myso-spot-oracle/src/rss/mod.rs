// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use chrono::Utc;
use tracing::info;

use crate::api::AppState;

pub async fn run_rss_watcher_loop(state: Arc<AppState>) -> anyhow::Result<()> {
    let rows = state.store.list_enabled_sources().await?;
    let feed_urls: Vec<String> = rows
        .iter()
        .filter(|r| r.adapter_type == "rss")
        .filter_map(|r| {
            r.config
                .get("feed_urls")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
                .and_then(|u| u.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    if feed_urls.is_empty() {
        return Ok(());
    }

    let client = myso_discovery_service_core::sources::http_client::HttpFetchClient::new();
    for url in feed_urls {
        if state.cancel.is_cancelled() {
            break;
        }
        match client.get_text(&url).await {
            Ok(body) => {
                let markets = state.store.list_markets(Some("active"), 100).await?;
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
                                serde_json::json!({ "feed_url": url, "content_hash": body.content_hash }),
                            )
                            .await;
                    }
                }
                state.metrics.rss_wake_total.inc();
                info!(feed = %url, "rss wake enqueued resolver jobs");
            }
            Err(err) => tracing::warn!(feed = %url, error = %err, "rss fetch failed"),
        }
    }
    Ok(())
}
