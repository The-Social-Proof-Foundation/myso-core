// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use chrono::{Duration, Utc};
use tokio::sync::Semaphore;
use tracing::{debug, error, info, warn};

use crate::config::DiscoveryArgs;
use crate::embed_client::{EmbedClient, EmbedRequest};
use crate::identity::{identity_hash_from_x_handle, resolve_or_create_candidate};
use crate::lifecycle::{AssetLifecycleState, LifecycleEvent};
use crate::metrics::DiscoveryMetrics;
use crate::normalizer::normalize_record;
use crate::prioritizer::{score_priority, signals_for_asset, PriorityWeights};
use crate::sources::{ContentKind, DiscoveryDomain, DiscoveryRegistry, SourceConfig};
use crate::store::DiscoveryStore;

pub async fn run_scheduler_loop(
    store: Arc<DiscoveryStore>,
    registry: Arc<DiscoveryRegistry>,
    sources: Vec<SourceConfig>,
    poll_interval_secs: u64,
    embed_enabled: bool,
    max_retries: i32,
    metrics: Arc<DiscoveryMetrics>,
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
                    let source_db_id = match upsert_source_row(&store, source).await {
                        Ok(id) => {
                            let _ = store.mark_source_polled(id).await;
                            id
                        }
                        Err(err) => {
                            error!(source = %source.id, error = %err, "failed to upsert discovery source; skipping assets");
                            metrics
                                .source_poll_total
                                .with_label_values(&[&source.id, "error"])
                                .inc();
                            continue;
                        }
                    };
                    metrics
                        .source_poll_total
                        .with_label_values(&[&source.id, "ok"])
                        .inc();
                    for raw in raw_records {
                        let normalized = normalize_record(&raw);
                        let signals = signals_for_asset(&normalized, false);
                        let priority = score_priority(&signals, &PriorityWeights::default());
                        let asset_id = match store
                            .upsert_asset(
                                Some(source_db_id),
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
                        metrics.assets_upserted_total.inc();
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
                        // PoC embed is creative media only — never enqueue factual/text assets.
                        let should_embed = embed_enabled
                            && source.domain == DiscoveryDomain::Creative
                            && normalized.content_kind == ContentKind::Media;
                        if should_embed {
                            let _ = store
                                .transition_asset(asset_id, LifecycleEvent::Enqueue)
                                .await;
                            let _ = store
                                .enqueue_job(
                                    "embed_asset",
                                    asset_id,
                                    priority,
                                    serde_json::json!({}),
                                    max_retries,
                                )
                                .await;
                        } else if embed_enabled {
                            debug!(
                                asset_id = %asset_id,
                                domain = source.domain.as_str(),
                                content_kind = normalized.content_kind.as_str(),
                                "skip embed enqueue (not creative media)"
                            );
                            metrics
                                .embed_jobs_total
                                .with_label_values(&["skipped_non_media"])
                                .inc();
                        }
                    }
                }
                Err(err) => {
                    metrics
                        .source_poll_total
                        .with_label_values(&[&source.id, "error"])
                        .inc();
                    error!(source = %source.id, error = %err, "adapter poll failed");
                }
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
    metrics: Arc<DiscoveryMetrics>,
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
        let metrics = metrics.clone();
        let permit = sem.clone().acquire_owned().await.unwrap();
        tokio::spawn(async move {
            let _permit = permit;
            let asset_id = job.discovery_asset_id;
            if let Err(err) = process_embed_job(&store, &embed_client, &args, &job).await {
                error!(job_id = %job.id, error = %err, "embed job failed");
                if let Some(aid) = asset_id {
                    let _ = store.transition_asset(aid, LifecycleEvent::Fail).await;
                }
                let backoff =
                    Duration::seconds(30 * 2_i64.pow(job.attempts.min(5) as u32));
                if let Err(requeue_err) = store
                    .requeue_job(job.id, Utc::now() + backoff, &err.to_string())
                    .await
                {
                    error!(job_id = %job.id, error = %requeue_err, "requeue embed job failed");
                    let _ = store
                        .complete_job(job.id, "failed", Some(&err.to_string()))
                        .await;
                    metrics
                        .embed_jobs_total
                        .with_label_values(&["failed"])
                        .inc();
                } else if job.attempts >= job.max_attempts {
                    metrics
                        .embed_jobs_total
                        .with_label_values(&["dead_letter"])
                        .inc();
                } else {
                    metrics
                        .embed_jobs_total
                        .with_label_values(&["requeued"])
                        .inc();
                }
            } else {
                let _ = store.complete_job(job.id, "completed", None).await;
                metrics
                    .embed_jobs_total
                    .with_label_values(&["completed"])
                    .inc();
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

    store
        .transition_asset(asset_id, LifecycleEvent::StartAcquire)
        .await?;

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

    store
        .transition_asset(asset_id, LifecycleEvent::EmbedComplete)
        .await?;
    store
        .transition_asset(asset_id, LifecycleEvent::IndexComplete)
        .await?;

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
async fn upsert_source_row(
    store: &DiscoveryStore,
    source: &SourceConfig,
) -> anyhow::Result<uuid::Uuid> {
    let config_json = serde_json::to_value(&source.config)?;
    let source_url = source
        .config
        .feed_urls
        .first()
        .cloned()
        .or_else(|| {
            source.config.owner.as_ref().and_then(|o| {
                source
                    .config
                    .repo
                    .as_ref()
                    .map(|r| format!("https://github.com/{o}/{r}"))
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

/// Single scheduler poll (no sleep) for cold-start bootstrap.
pub async fn poll_sources_once(
    store: Arc<DiscoveryStore>,
    registry: Arc<DiscoveryRegistry>,
    sources: Vec<SourceConfig>,
    embed_enabled: bool,
    max_retries: i32,
    metrics: Arc<DiscoveryMetrics>,
) {
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
                let source_db_id = match upsert_source_row(&store, source).await {
                    Ok(id) => {
                        let _ = store.mark_source_polled(id).await;
                        id
                    }
                    Err(err) => {
                        error!(source = %source.id, error = %err, "failed to upsert discovery source; skipping assets");
                        metrics
                            .source_poll_total
                            .with_label_values(&[&source.id, "error"])
                            .inc();
                        continue;
                    }
                };
                metrics
                    .source_poll_total
                    .with_label_values(&[&source.id, "ok"])
                    .inc();
                for raw in raw_records {
                    let normalized = normalize_record(&raw);
                    let signals = signals_for_asset(&normalized, false);
                    let priority = score_priority(&signals, &PriorityWeights::default());
                    let asset_id = match store
                        .upsert_asset(
                            Some(source_db_id),
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
                    metrics.assets_upserted_total.inc();
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
                    let should_embed = embed_enabled
                        && source.domain == DiscoveryDomain::Creative
                        && normalized.content_kind == ContentKind::Media;
                    if should_embed {
                        let _ = store
                            .transition_asset(asset_id, LifecycleEvent::Enqueue)
                            .await;
                        let _ = store
                            .enqueue_job(
                                "embed_asset",
                                asset_id,
                                priority,
                                serde_json::json!({}),
                                max_retries,
                            )
                            .await;
                    }
                }
            }
            Err(err) => {
                metrics
                    .source_poll_total
                    .with_label_values(&[&source.id, "error"])
                    .inc();
                error!(source = %source.id, error = %err, "adapter poll failed");
            }
        }
    }
}

/// Process up to `max_jobs` embed jobs synchronously (bootstrap drain).
pub async fn drain_embed_jobs(
    store: Arc<DiscoveryStore>,
    embed_client: Arc<EmbedClient>,
    args: Arc<DiscoveryArgs>,
    metrics: Arc<DiscoveryMetrics>,
    max_jobs: usize,
) {
    for _ in 0..max_jobs {
        let job = match store.claim_next_job().await {
            Ok(Some(job)) => job,
            Ok(None) => break,
            Err(err) => {
                error!(error = %err, "claim job failed");
                break;
            }
        };
        if let Err(err) = process_embed_job(&store, &embed_client, &args, &job).await {
            error!(job_id = %job.id, error = %err, "embed job failed");
            let _ = store
                .complete_job(job.id, "failed", Some(&err.to_string()))
                .await;
            metrics
                .embed_jobs_total
                .with_label_values(&["failed"])
                .inc();
        } else {
            let _ = store.complete_job(job.id, "completed", None).await;
            metrics
                .embed_jobs_total
                .with_label_values(&["completed"])
                .inc();
        }
    }
}
