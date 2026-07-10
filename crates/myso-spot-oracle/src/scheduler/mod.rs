// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use tracing::{error, info};

use crate::api::AppState;
use crate::resolver::engine::resolve_market;
use crate::store::jobs::SpotJob;

pub async fn run_scheduler_loop(state: Arc<AppState>) -> anyhow::Result<()> {
    loop {
        if state.cancel.is_cancelled() {
            break;
        }
        let job = match state.store.claim_next_job().await {
            Ok(Some(job)) => job,
            Ok(None) => {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
            Err(err) => {
                error!(error = %err, "claim spot job failed");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
        };
        dispatch_job(state.clone(), job).await;
    }
    Ok(())
}

async fn dispatch_job(state: Arc<AppState>, job: SpotJob) {
    let job_id = job.id;
    let job_type = job.job_type.clone();
    let result = match job_type.as_str() {
        "ReviewPost" => crate::review::worker::run_review_job_loop(state.clone(), job).await,
        "ResolveMarket" => resolve_market_job(state.clone(), job).await,
        "SubmitChainTx" => crate::blockchain::worker::process_chain_job(state.clone(), job).await,
        other => {
            error!(job_type = other, "unknown job type");
            state
                .store
                .complete_job(job_id, "failed", Some("unknown job type"))
                .await
                .ok();
            return;
        }
    };
    if let Err(err) = result {
        error!(job_id = %job_id, job_type, error = %err, "job failed");
    }
}

async fn resolve_market_job(state: Arc<AppState>, job: SpotJob) -> anyhow::Result<()> {
    let job_id = job.id;
    match resolve_market(state.clone(), &job).await {
        Ok(()) => {
            state.store.complete_job(job_id, "completed", None).await?;
            info!(job_id = %job_id, "resolve market completed");
        }
        Err(err) => {
            let backoff = chrono::Duration::seconds(60 * 2_i64.pow(job.attempts.min(5) as u32));
            state
                .store
                .requeue_job(job_id, chrono::Utc::now() + backoff, &err.to_string())
                .await?;
            return Err(err);
        }
    }
    Ok(())
}
