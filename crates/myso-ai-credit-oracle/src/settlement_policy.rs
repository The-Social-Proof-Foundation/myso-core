// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::config::OracleArgs;
use crate::receipt::ReceiptStore;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettleTrigger {
    ThresholdMist,
    MaxAge,
    MinCount,
    IntervalSweep,
}

pub fn should_settle_balance(
    balance_id: &str,
    store: &ReceiptStore,
    args: &OracleArgs,
    now_ms: u64,
) -> Option<SettleTrigger> {
    let summary = store.balance_pending_summary(balance_id);
    if summary.pending_count == 0 {
        return None;
    }
    if summary.pending_mist >= args.settle_threshold_mist {
        return Some(SettleTrigger::ThresholdMist);
    }
    if summary.pending_count >= args.settle_min_count {
        return Some(SettleTrigger::MinCount);
    }
    if let Some(oldest) = summary.oldest_timestamp_ms {
        let age_secs = (now_ms.saturating_sub(oldest)) / 1000;
        if age_secs >= args.settle_max_age_secs {
            return Some(SettleTrigger::MaxAge);
        }
    }
    None
}

pub fn balances_due_for_settlement(
    store: &ReceiptStore,
    args: &OracleArgs,
    now_ms: u64,
) -> Vec<String> {
    let mut due = Vec::new();
    for balance_id in store.balance_ids_with_pending() {
        if should_settle_balance(&balance_id, store, args, now_ms).is_some() {
            due.push(balance_id);
        }
    }
    due
}

/// On interval sweep, settle every balance that has any pending receipts.
pub fn balances_for_interval_sweep(store: &ReceiptStore) -> Vec<String> {
    store.balance_ids_with_pending()
}

pub fn log_pending_age_warnings(store: &ReceiptStore, args: &OracleArgs, now_ms: u64) {
    for balance_id in store.balance_ids_with_pending() {
        let summary = store.balance_pending_summary(&balance_id);
        if let Some(oldest) = summary.oldest_timestamp_ms {
            let age_secs = (now_ms.saturating_sub(oldest)) / 1000;
            if age_secs >= args.settle_warn_age_secs {
                tracing::warn!(
                    balance_id = %balance_id,
                    pending_mist = summary.pending_mist,
                    pending_count = summary.pending_count,
                    pending_age_secs = age_secs,
                    warn_age_secs = args.settle_warn_age_secs,
                    "pending receipts approaching receipt TTL — settlement overdue"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use crate::receipt::UsageLine;

    fn test_args() -> OracleArgs {
        OracleArgs {
            listen_addr: "0.0.0.0:8095".into(),
            private_key_hex: "00".repeat(32),
            settlement_secret: None,
            myso_rpc: "http://127.0.0.1:9000".into(),
            receipt_store_path: std::path::PathBuf::from("test.json"),
            config_object_id: None,
            settlement_key_hex: None,
            settlement_interval_secs: 60,
            settle_threshold_mist: 10_000_000_000,
            settle_max_age_secs: 180,
            settle_min_count: 8,
            settle_warn_age_secs: 240,
            social_server_url: "http://127.0.0.1:9126".into(),
            pricing_catalog_path: std::path::PathBuf::from("config/pricing_catalog.toml"),
            ecosystem_margin_pct: 0.125,
            usage_sync_secret: None,
            strict_catalog: false,
        }
    }

    fn line(balance_id: &str, amount_mist: u64, timestamp_ms: u64, receipt_id: u128) -> UsageLine {
        UsageLine {
            receipt_id,
            balance_id: balance_id.to_string(),
            memory_account_id: "0xacc".to_string(),
            agent_object_id: "0xagent".to_string(),
            amount_mist,
            usage_kind: 1,
            model_id: None,
            tool_id: None,
            metadata: None,
            signature_hex: "ab".to_string(),
            settlement_nonce: 1,
            timestamp_ms,
            settled: false,
            created_at_ms: timestamp_ms,
        }
    }

    #[test]
    fn threshold_triggers_settlement() {
        let args = test_args();
        let store = ReceiptStore {
            lines: vec![line("0xbal", 10_000_000_000, 1_000_000, 1)],
            settled_ids: HashSet::new(),
        };
        let trigger = should_settle_balance("0xbal", &store, &args, 2_000_000);
        assert_eq!(trigger, Some(SettleTrigger::ThresholdMist));
    }

    #[test]
    fn count_triggers_settlement() {
        let args = test_args();
        let lines: Vec<UsageLine> = (0..8)
            .map(|i| line("0xbal", 1_000_000, 1_000_000, i as u128))
            .collect();
        let store = ReceiptStore {
            lines,
            settled_ids: HashSet::new(),
        };
        let trigger = should_settle_balance("0xbal", &store, &args, 2_000_000);
        assert_eq!(trigger, Some(SettleTrigger::MinCount));
    }

    #[test]
    fn age_triggers_settlement() {
        let args = test_args();
        let now_ms = 200_000u64;
        let old_ts = now_ms - (args.settle_max_age_secs * 1000);
        let store = ReceiptStore {
            lines: vec![line("0xbal", 1_000_000, old_ts, 1)],
            settled_ids: HashSet::new(),
        };
        let trigger = should_settle_balance("0xbal", &store, &args, now_ms);
        assert_eq!(trigger, Some(SettleTrigger::MaxAge));
    }
}
