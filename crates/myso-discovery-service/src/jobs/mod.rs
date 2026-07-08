// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Background reconciliation jobs.

pub async fn run_reconciliation_loop(_store: std::sync::Arc<crate::store::DiscoveryStore>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
