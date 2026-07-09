// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! SubscribeCheckpoints gRPC ingest — filter `PostCreatedEvent.enable_spot` and enqueue review.

use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use futures::StreamExt;
use myso_rpc::proto::myso::rpc::v2::subscription_service_client::SubscriptionServiceClient;
use myso_rpc::proto::myso::rpc::v2::SubscribeCheckpointsRequest;
use myso_types::full_checkpoint_content::Checkpoint;
use tonic::transport::Endpoint;
use myso_types::base_types::ObjectID;
use tracing::{error, info, warn};

use crate::api::AppState;
use crate::ingest::post_created::parse_post_created;
use crate::store::checkpoint::CheckpointIngestStore;

const MAX_GRPC_MESSAGE_SIZE_BYTES: usize = 128 * 1024 * 1024;

pub async fn run_checkpoint_ingest_loop(state: Arc<AppState>) -> anyhow::Result<()> {
    let Some(streaming_url) = &state.args.streaming_url else {
        warn!("SPOT_ORACLE_STREAMING_URL unset; checkpoint ingest disabled");
        return Ok(());
    };

    let mut backoff_secs = 1u64;
    loop {
        if state.cancel.is_cancelled() {
            break;
        }
        match run_stream_once(state.clone(), streaming_url).await {
            Ok(()) => {
                backoff_secs = 1;
                info!("checkpoint stream ended; reconnecting");
            }
            Err(err) => {
                error!(error = %err, backoff_secs, "checkpoint stream error");
                state
                    .metrics
                    .checkpoint_ingest_total
                    .with_label_values(&["error"])
                    .inc();
            }
        }
        tokio::select! {
            _ = state.cancel.cancelled() => break,
            _ = tokio::time::sleep(Duration::from_secs(backoff_secs)) => {}
        }
        backoff_secs = (backoff_secs * 2).min(60);
    }
    Ok(())
}

async fn run_stream_once(state: Arc<AppState>, streaming_url: &str) -> anyhow::Result<()> {
    let endpoint = Endpoint::from_shared(streaming_url.to_string())?
        .connect_timeout(Duration::from_secs(10));
    let mut client = SubscriptionServiceClient::connect(endpoint)
        .await?
        .max_decoding_message_size(MAX_GRPC_MESSAGE_SIZE_BYTES);

    let mut request = SubscribeCheckpointsRequest::default();
    request.read_mask = Some(Checkpoint::proto_field_mask());

    let mut stream = client.subscribe_checkpoints(request).await?.into_inner();
    let ingest_store = CheckpointIngestStore::new(state.store.pool().clone());
    let mut watermark = ingest_store.get_watermark().await.unwrap_or(0);

    info!(watermark, "checkpoint ingest connected");

    while let Some(item) = stream.next().await {
        if state.cancel.is_cancelled() {
            break;
        }
        let response = item?;
        let proto_cp = response
            .checkpoint
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing checkpoint in stream item"))?;
        let checkpoint: Checkpoint = proto_cp.try_into()?;

        let seq = checkpoint.summary.sequence_number;
        if seq <= watermark {
            continue;
        }

        let ingested = process_checkpoint(&state, &checkpoint).await?;
        watermark = seq;
        ingest_store.set_watermark(seq).await?;
        state.metrics.checkpoint_lag.set(seq as i64);
        if ingested > 0 {
            info!(checkpoint = seq, ingested, "checkpoint ingest enqueued posts");
            state
                .metrics
                .checkpoint_ingest_total
                .with_label_values(&["enqueued"])
                .inc_by(ingested as u64);
        } else {
            state
                .metrics
                .checkpoint_ingest_total
                .with_label_values(&["skipped"])
                .inc();
        }
    }
    Ok(())
}

async fn process_checkpoint(state: &AppState, checkpoint: &Checkpoint) -> anyhow::Result<usize> {
    let mut enqueued = 0usize;
    let social_package =
        ObjectID::from_hex_literal(&state.args.social_package_id).unwrap_or_else(|_| ObjectID::ZERO);

    for tx in &checkpoint.transactions {
        let Some(events) = &tx.events else {
            continue;
        };
        for ev in &events.data {
            if ev.package_id != social_package {
                continue;
            }
            if ev.type_.module.as_str() != "post" {
                continue;
            }
            if ev.type_.name.as_str() != "PostCreatedEvent" {
                continue;
            }

            let parsed = match parse_post_created(&ev.contents) {
                Ok(p) => p,
                Err(err) => {
                    warn!(error = %err, "PostCreatedEvent parse failed");
                    state
                        .metrics
                        .checkpoint_ingest_total
                        .with_label_values(&["parse_error"])
                        .inc();
                    continue;
                }
            };

            if !parsed.enable_spot {
                state.metrics.posts_filtered_enable_spot.inc();
                continue;
            }
            if parsed.spot_id.is_some() {
                continue;
            }
            if state.store.market_exists(&parsed.post_id).await? {
                continue;
            }

            let market_id = crate::store::markets::insert_market(
                state.store.pool(),
                &parsed.post_id,
                &parsed.owner,
                &parsed.content,
            )
            .await?;
            crate::store::jobs::enqueue_job(
                state.store.pool(),
                "ReviewPost",
                Some(market_id),
                None,
                parsed.created_at_ms,
                Utc::now(),
                serde_json::json!({
                    "post_id": parsed.post_id,
                    "owner": parsed.owner,
                    "content": parsed.content,
                    "created_at_ms": parsed.created_at_ms,
                    "post_type": parsed.post_type,
                }),
            )
            .await?;
            enqueued += 1;
        }
    }
    Ok(enqueued)
}
