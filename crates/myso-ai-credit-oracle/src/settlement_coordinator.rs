// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;

use crate::config::OracleArgs;
use crate::receipt::ReceiptStore;
use crate::settlement::run_settlement_cycle;
use crate::settlement_policy::{
    balances_due_for_settlement, balances_for_interval_sweep, log_pending_age_warnings,
    should_settle_balance,
};

const SETTLE_DEBOUNCE_MS: u64 = 2000;

#[derive(Debug, Clone)]
pub enum SettlementMode {
    IntervalSweep,
    BalanceTriggered(String),
    DueBalances,
}

impl SettlementMode {
    pub fn label(&self) -> &'static str {
        match self {
            SettlementMode::IntervalSweep => "interval_sweep",
            SettlementMode::BalanceTriggered(_) => "balance_triggered",
            SettlementMode::DueBalances => "due_balances",
        }
    }
}

pub struct SettlementCoordinator {
    args: Arc<OracleArgs>,
    store: Arc<Mutex<ReceiptStore>>,
    store_path: PathBuf,
    cycle_lock: Mutex<()>,
    debounce: Mutex<Option<tokio::task::JoinHandle<()>>>,
}

pub fn spawn_settlement_worker(coordinator: Arc<SettlementCoordinator>) {
    coordinator.spawn_interval_worker();
}

impl SettlementCoordinator {
    pub fn new(
        args: Arc<OracleArgs>,
        store: Arc<Mutex<ReceiptStore>>,
        store_path: PathBuf,
    ) -> Self {
        Self {
            args,
            store,
            store_path,
            cycle_lock: Mutex::new(()),
            debounce: Mutex::new(None),
        }
    }

    pub fn spawn_interval_worker(self: Arc<Self>) {
        if self.args.settlement_key_hex.is_none() {
            tracing::info!("settlement worker disabled (no AI_CREDIT_SETTLEMENT_KEY_HEX)");
            return;
        }
        let interval = self.args.settlement_interval_secs;
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(interval));
            loop {
                ticker.tick().await;
                let settled = self.run_cycle(SettlementMode::IntervalSweep).await;
                if settled > 0 {
                    tracing::info!(settled, "settlement interval sweep flush");
                }
            }
        });
    }

    /// Debounced settlement after usage recording.
    pub async fn request_settle_for_balance(self: &Arc<Self>, balance_id: String) {
        if self.args.settlement_key_hex.is_none() {
            return;
        }
        let store = self.store.lock().await;
        let now_ms = chrono::Utc::now().timestamp_millis() as u64;
        if should_settle_balance(&balance_id, &store, &self.args, now_ms).is_none() {
            return;
        }
        drop(store);

        let mut debounce_guard = self.debounce.lock().await;
        if let Some(handle) = debounce_guard.as_ref() {
            handle.abort();
        }
        let coordinator = Arc::clone(self);
        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(SETTLE_DEBOUNCE_MS)).await;
            let settled = coordinator
                .run_cycle(SettlementMode::BalanceTriggered(balance_id))
                .await;
            if settled > 0 {
                tracing::info!(settled, "settlement triggered by usage threshold");
            }
        });
        *debounce_guard = Some(handle);
    }

    pub async fn run_cycle(&self, mode: SettlementMode) -> usize {
        let _cycle_guard = match self.cycle_lock.try_lock() {
            Ok(guard) => guard,
            Err(_) => {
                tracing::debug!("settlement cycle skipped — another cycle in progress");
                return 0;
            }
        };

        let now_ms = chrono::Utc::now().timestamp_millis() as u64;
        let balance_ids: Vec<String> = {
            let store = self.store.lock().await;
            log_pending_age_warnings(&store, &self.args, now_ms);
            match mode {
                SettlementMode::IntervalSweep => balances_for_interval_sweep(&store),
                SettlementMode::BalanceTriggered(ref id) => {
                    if should_settle_balance(id, &store, &self.args, now_ms).is_some() {
                        vec![id.clone()]
                    } else {
                        vec![]
                    }
                }
                SettlementMode::DueBalances => {
                    balances_due_for_settlement(&store, &self.args, now_ms)
                }
            }
        };

        if balance_ids.is_empty() {
            return 0;
        }

        let mut store = self.store.lock().await;
        match run_settlement_cycle(
            &self.args,
            &mut store,
            &self.store_path,
            &balance_ids,
            mode.label(),
        )
        .await
        {
            Ok(n) => n,
            Err(err) => {
                tracing::warn!(error = %err, trigger = mode.label(), "settlement cycle failed");
                0
            }
        }
    }
}
