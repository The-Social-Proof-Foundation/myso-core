// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::api::AppState;
use crate::blockchain::{create_market, settle};
use crate::store::jobs::SpotJob;

pub async fn process_chain_job(state: Arc<AppState>, job: SpotJob) -> anyhow::Result<()> {
    let job_id = job.id;
    let tx_kind = job
        .payload
        .get("tx_kind")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let result = match tx_kind {
        "create_market" => create_market::submit_create_market(state.clone(), &job).await,
        "oracle_resolve" => settle::submit_oracle_resolve(state.clone(), &job).await,
        other => Err(anyhow::anyhow!("unknown tx_kind: {other}")),
    };
    match result {
        Ok(()) => {
            state.store.complete_job(job_id, "completed", None).await?;
        }
        Err(err) => {
            state
                .store
                .complete_job(job_id, "failed", Some(&err.to_string()))
                .await?;
            return Err(err);
        }
    }
    Ok(())
}
