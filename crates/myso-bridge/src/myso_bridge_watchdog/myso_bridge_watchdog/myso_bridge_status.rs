// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! The MySoBridgeStatus observable monitors whether the MySo Bridge is paused.

use crate::myso_bridge_watchdog::Observable;
use crate::myso_client::MySoBridgeClient;
use async_trait::async_trait;
use prometheus::IntGauge;
use std::sync::Arc;

use tokio::time::Duration;
use tracing::{error, info};

pub struct MySoBridgeStatus {
    myso_client: Arc<MySoBridgeClient>,
    metric: IntGauge,
}

impl MySoBridgeStatus {
    pub fn new(myso_client: Arc<MySoBridgeClient>, metric: IntGauge) -> Self {
        Self {
            myso_client,
            metric,
        }
    }
}

#[async_trait]
impl Observable for MySoBridgeStatus {
    fn name(&self) -> &str {
        "MySoBridgeStatus"
    }

    async fn observe_and_report(&self) {
        let status = self.myso_client.is_bridge_paused().await;
        match status {
            Ok(status) => {
                self.metric.set(status as i64);
                info!("MySo Bridge Status: {:?}", status);
            }
            Err(e) => {
                error!("Error getting myso bridge status: {:?}", e);
            }
        }
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(10)
    }
}
