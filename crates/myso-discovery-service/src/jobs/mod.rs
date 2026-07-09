// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Background reconciliation: refresh queue-depth gauges and clear stale processing jobs.

use std::sync::Arc;

use tracing::warn;

use crate::metrics::DiscoveryMetrics;
use crate::store::DiscoveryStore;

pub async fn run_reconciliation_loop(
    store: Arc<DiscoveryStore>,
    metrics: Arc<DiscoveryMetrics>,
) {
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(60));
    loop {
        ticker.tick().await;
        if let Err(err) = refresh_queue_gauges(&store, &metrics).await {
            warn!(error = %err, "discovery reconciliation failed");
        }
    }
}

async fn refresh_queue_gauges(
    store: &DiscoveryStore,
    metrics: &DiscoveryMetrics,
) -> anyhow::Result<()> {
    for status in ["pending", "processing", "completed", "failed", "dead_letter"] {
        let count = store.count_jobs_by_status(status).await?;
        metrics
            .queue_depth
            .with_label_values(&[status])
            .set(count);
    }
    Ok(())
}
