// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use tracing::info;

use crate::api::AppState;

pub async fn run_reconcile_loop(state: Arc<AppState>) -> anyhow::Result<()> {
    let pending_txs = state.store.list_pending_transactions(50).await?;
    if !pending_txs.is_empty() {
        info!(
            count = pending_txs.len(),
            "reconcile: pending chain transactions"
        );
    }

    let dead_jobs = state.store.list_jobs(Some("dead_letter"), 20).await?;
    if !dead_jobs.is_empty() {
        info!(count = dead_jobs.len(), "reconcile: dead-letter jobs");
    }
    Ok(())
}
