// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use tokio::sync::Semaphore;
use tracing::{error, info, warn};

use crate::config::DiscoveryArgs;
use crate::embed_client::{EmbedClient, EmbedRequest};
use crate::identity::{identity_hash_from_x_handle, resolve_or_create_candidate};
use crate::lifecycle::{AssetLifecycleState, LifecycleEvent};
use crate::normalizer::normalize_record;
use crate::prioritizer::{score_priority, signals_for_asset, PriorityWeights};
use crate::sources::{DiscoveryRegistry, SourceConfig};
use crate::store::DiscoveryStore;

pub async fn run_scheduler_loop(
    store: Arc<DiscoveryStore>,
    registry: Arc<DiscoveryRegistry>,
    sources: Vec<SourceConfig>,
    poll_interval_secs: u64,
    embed_enabled: bool,
) {
    loop {
        for source in &sources {
            if !source.enabled {
                continue;
            }
            let Some(adapter) = registry.get(&source.adapter_type) else {
                warn!(adapter = %source.adapter_type, "adapter not registered");
                continue;
            };
            if !adapter.supports(source) {
                continue;
            }
            match adapter.discover(source).await {
                Ok(raw_records) => {
                    if let Ok(source_db_id) = upsert_source_row(&store, source).await {
                        let _ = store.mark_source_polled(source_db_id).await;
                    }
                    for raw in raw_records {
                        let normalized = normalize_record(&raw);
                        let signals = signals_for_asset(&normalized, false);
                        let priority = score_priority(&signals, &PriorityWeights::default());
                        let asset_id = match store
                            .upsert_asset(
                                None,
                                &normalized,
                                priority,
                                AssetLifecycleState::Normalized,
                            )
                            .await
                        {
                            Ok(id) => id,
                            Err(err) => {
                                error!(error = %err, "failed to upsert discovery asset");
                                continue;
                            }
                        };
                        if let Some(handle) = &normalized.creator_x_handle {
                            if let Ok(candidate_id) = resolve_or_create_candidate(
                                store.pool(),
                                handle,
                                normalized.creator_confidence,
                            )
                            .await
                            {
                                let _ = sqlx::query(
                                    "UPDATE discovery_assets SET creator_candidate_id = $1 WHERE id = $2",
                                )
                                .bind(candidate_id)
                                .bind(asset_id)
                                .execute(store.pool())
                                .await;
                            }
                        }
                        if embed_enabled {
                            let _ = store
                                .transition_asset(asset_id, LifecycleEvent::Enqueue)
                                .await;
                            let _ = store
                                .enqueue_job(
                                    "embed_asset",
                                    asset_id,
                                    priority,
                                    serde_json::json!({}),
                                )
                                .await;
                        }
                    }
                }
                Err(err) => error!(source = %source.id, error = %err, "adapter poll failed"),
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(poll_interval_secs)).await;
    }
}

pub async fn run_worker_loop(
    store: Arc<DiscoveryStore>,
    embed_client: Arc<EmbedClient>,
    args: Arc<DiscoveryArgs>,
    concurrency: usize,
) {
    let sem = Arc::new(Semaphore::new(concurrency));
    loop {
        let job = match store.claim_next_job().await {
            Ok(Some(job)) => job,
            Ok(None) => {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
            Err(err) => {
                error!(error = %err, "claim job failed");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
        };

        let store = store.clone();
        let embed_client = embed_client.clone();
        let args = args.clone();
        let permit = sem.clone().acquire_owned().await.unwrap();
        tokio::spawn(async move {
            let _permit = permit;
            if let Err(err) = process_embed_job(&store, &embed_client, &args, &job).await {
                error!(job_id = %job.id, error = %err, "embed job failed");
                let _ = store
                    .complete_job(job.id, "failed", Some(&err.to_string()))
                    .await;
            } else {
                let _ = store.complete_job(job.id, "completed", None).await;
            }
        });
    }
}

async fn process_embed_job(
    store: &DiscoveryStore,
    embed_client: &EmbedClient,
    args: &DiscoveryArgs,
    job: &crate::store::DiscoveryJob,
) -> anyhow::Result<()> {
    let asset_id = job
        .discovery_asset_id
        .ok_or_else(|| anyhow::anyhow!("missing discovery_asset_id"))?;

    store.transition_asset(asset_id, LifecycleEvent::StartAcquire).await?;

    let asset: (String, String, Option<String>, f64) = sqlx::query_as(
        r#"
        SELECT external_source_url, media_type, canonical_metadata->>'creator_x_handle', creator_confidence
        FROM discovery_assets WHERE id = $1
        "#,
    )
    .bind(asset_id)
    .fetch_one(store.pool())
    .await?;

    let response = embed_client
        .embed(EmbedRequest {
            discovery_asset_id: asset_id,
            external_source_url: asset.0.clone(),
            media_type: asset.1.clone(),
            embedding_version: args.active_embedding_version.clone(),
            creator_x_handle: asset.2.clone(),
            creator_confidence: Some(asset.3),
        })
        .await?;

    sqlx::query(
        r#"
        UPDATE discovery_assets SET
            work_confidence = $1,
            active_embedding_version = $2,
            updated_at = NOW()
        WHERE id = $3
        "#,
    )
    .bind(response.work_confidence)
    .bind(&response.embedding_version)
    .bind(asset_id)
    .execute(store.pool())
    .await?;

    store.transition_asset(asset_id, LifecycleEvent::EmbedComplete).await?;
    store.transition_asset(asset_id, LifecycleEvent::IndexComplete).await?;

    if let Some(handle) = asset.2 {
        let ih = response
            .identity_hash
            .unwrap_or_else(|| identity_hash_from_x_handle(&handle));
        info!(asset_id = %asset_id, media_id = %response.media_id, identity_hash = %ih, "asset indexed");
    }

    Ok(())
}

/// Register/refresh a `discovery_sources` row from the loaded YAML `SourceConfig`.
/// Source URL is derived from fetch config (first feed URL / GitHub repo / API base).
async fn upsert_source_row(store: &DiscoveryStore, source: &SourceConfig) -> anyhow::Result<uuid::Uuid> {
    let config_json = serde_json::to_value(&source.config)?;
    let source_url = source
        .config
        .feed_urls
        .first()
        .cloned()
        .or_else(|| {
            source.config.owner.as_ref().and_then(|o| {
                source.config.repo.as_ref().map(|r| {
                    format!("https://github.com/{o}/{r}")
                })
            })
        })
        .or_else(|| source.config.api_base_url.clone());
    store
        .upsert_source(
            &source.id,
            &source.adapter_type,
            source.domain.as_str(),
            source.trust_score,
            source.enabled,
            source_url.as_deref(),
            &config_json,
        )
        .await
}
