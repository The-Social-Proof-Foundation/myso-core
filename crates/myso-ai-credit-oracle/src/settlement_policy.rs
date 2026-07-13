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
    use crate::receipt::UsageLine;
    use std::collections::HashSet;

    fn test_args() -> OracleArgs {
        OracleArgs {
            listen_addr: "0.0.0.0:8095".into(),
            database_url: "postgres://localhost/test".into(),
            database_max_connections: 2,
            outbox_lease_secs: 60,
            replica_count: 1,
            legacy_usage_enabled: true,
            private_key_hex: "00".repeat(32),
            settlement_secret: None,
            myso_rpc: "http://127.0.0.1:9000".into(),
            receipt_store_path: std::path::PathBuf::from("test.json"),
            config_object_id: None,
            settlement_key_hex: None,
            reservation_price_buffer_bps: 2500,
            reservation_capture_window_secs: 600,
            reservation_hard_expiry_secs: 1800,
            settlement_interval_secs: 60,
            settle_threshold_mist: 10_000_000_000,
            settle_max_age_secs: 180,
            settle_min_count: 8,
            settle_warn_age_secs: 240,
            social_server_url: "http://127.0.0.1:9126".into(),
            pricing_catalog_path: std::path::PathBuf::from("config/pricing_catalog.toml"),
            ecosystem_margin_pct: 0.125,
            graphql_url: "http://127.0.0.1:9125/graphql".into(),
            markup_refresh_interval_secs: 300,
            markup_graphql_enabled: false,
            usage_sync_secret: None,
            strict_catalog: false,
            myso_price_oracle_url: "https://myso-price-oracle-testnet.up.railway.app".into(),
            price_refresh_interval_secs: 60,
            myso_price_max_stale_secs: 300,
            myso_price_enabled: false,
            openrouter_api_key: None,
            catalog_sync_enabled: false,
            catalog_sync_interval_secs: 86400,
            catalog_sync_on_startup: true,
            openrouter_api_url: "https://openrouter.ai/api/v1/models".into(),
            openrouter_chat_url: "https://openrouter.ai/api/v1/chat/completions".into(),
            inference_enabled: false,
            catalog_max_drift_pct: 50.0,
            approvals_enabled: false,
            approval_lookup_ttl_secs: 5,
            approval_min_remaining_secs: 180,
            workflow_relayer_url: None,
            workflow_sync_secret: None,
            audit_sync_secret: None,
            oracle_api_secret: None,
            require_secrets: false,
            agent_auth_enabled: false,
            agent_auth_ttl_secs: 300,
            require_settlement_secret: false,
            receipt_store_recover: false,
            ingest_reconcile_interval_secs: 30,
            ingest_backlog_warn_age_secs: 300,
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
            void: false,
            organization_id: None,
            idempotency_key: None,
            ingest_synced: true,
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

    #[test]
    fn void_lines_excluded_from_pending_and_settlement() {
        let args = test_args();
        let mut voided = line("0xbal", 10_000_000_000, 1_000_000, 1);
        voided.void = true;
        let store = ReceiptStore {
            lines: vec![voided],
            settled_ids: HashSet::new(),
        };
        assert_eq!(store.total_pending_mist(), 0);
        assert!(store.pending_for_balance("0xbal").is_empty());
        assert_eq!(
            should_settle_balance("0xbal", &store, &args, 2_000_000),
            None
        );
    }

    #[test]
    fn renumber_pending_assigns_contiguous_nonces_ordered_by_creation() {
        let mut voided = line("0xbal", 100, 2_000, 2);
        voided.void = true;
        let mut store = ReceiptStore {
            lines: vec![
                line("0xbal", 100, 3_000, 3),
                voided,
                line("0xbal", 100, 1_000, 1),
                line("0xother", 100, 1_500, 9),
            ],
            settled_ids: HashSet::new(),
        };
        let indices = store.renumber_pending_for_balance("0xbal", 7, 99_000);
        assert_eq!(indices.len(), 2);
        // Oldest first: receipt 1 gets nonce 8, receipt 3 gets nonce 9.
        let by_receipt = |id: u128| store.lines.iter().find(|l| l.receipt_id == id).unwrap();
        assert_eq!(by_receipt(1).settlement_nonce, 8);
        assert_eq!(by_receipt(3).settlement_nonce, 9);
        assert_eq!(by_receipt(1).timestamp_ms, 99_000);
        // Voided and foreign-balance lines untouched.
        assert_eq!(by_receipt(2).settlement_nonce, 1);
        assert_eq!(by_receipt(9).settlement_nonce, 1);
    }
}
