// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize};

/// True once the usage line has been successfully ingested into social-server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageLine {
    pub receipt_id: u128,
    pub balance_id: String,
    pub memory_account_id: String,
    pub agent_object_id: String,
    pub amount_mist: u64,
    pub usage_kind: u8,
    pub model_id: Option<String>,
    pub tool_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub signature_hex: String,
    pub settlement_nonce: u64,
    pub timestamp_ms: u64,
    pub settled: bool,
    pub created_at_ms: u64,
    /// Voided by the approval-abort recovery path (revoked/expired allowance between
    /// signing and settlement). Voided lines never settle and are excluded from pending.
    #[serde(default)]
    pub void: bool,
    /// Org attribution resolved from the sub-agent at record time.
    #[serde(default)]
    pub organization_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    /// False until social-server acknowledges the usage line ingest.
    #[serde(default)]
    pub ingest_synced: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReceiptStore {
    pub lines: Vec<UsageLine>,
    pub settled_ids: HashSet<u128>,
}

#[derive(Debug, Clone, Default)]
pub struct BalancePendingSummary {
    pub pending_mist: u64,
    pub pending_count: u64,
    pub oldest_timestamp_ms: Option<u64>,
}

struct StoreFileLock {
    _file: File,
}

impl StoreFileLock {
    fn acquire(path: &Path) -> Result<Self> {
        let lock_path_str = format!("{}.lock", path.display());
        let lock_path = Path::new(&lock_path_str);
        if let Some(parent) = lock_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(lock_path)
            .context("open receipt store lock file")?;
        file.lock_exclusive().context("acquire receipt store lock")?;
        Ok(Self { _file: file })
    }
}

impl ReceiptStore {
    pub fn probe_writable(path: &Path) -> bool {
        StoreFileLock::acquire(path).is_ok()
    }

    /// Load the receipt store. Corrupt JSON fails unless `recover` is true (explicit opt-in
    /// to reset after manual backup). Deploy a single oracle instance per receipt file path.
    pub fn load(path: &Path, recover: bool) -> Result<Self> {
        let _lock = StoreFileLock::acquire(path)?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let bytes = fs::read(path)?;
        match serde_json::from_slice(&bytes) {
            Ok(store) => Ok(store),
            Err(err) if recover => {
                tracing::warn!(
                    error = %err,
                    path = %path.display(),
                    "receipt store corrupt; AI_CREDIT_RECEIPT_STORE_RECOVER=true, starting empty"
                );
                Ok(Self::default())
            }
            Err(err) => Err(err).context(format!(
                "receipt store at {} is corrupt; set AI_CREDIT_RECEIPT_STORE_RECOVER=true after backup to reset",
                path.display()
            )),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let _lock = StoreFileLock::acquire(path)?;
        let bytes = serde_json::to_vec_pretty(self)?;
        let tmp = path.with_extension("json.tmp");
        let mut file = File::create(&tmp).context("failed to create receipt store tmp")?;
        file.write_all(&bytes)
            .context("failed to write receipt store tmp")?;
        file.sync_all().context("failed to fsync receipt store tmp")?;
        drop(file);
        if path.exists() {
            let backup = path.with_extension("json.bak");
            fs::copy(path, &backup).ok();
        }
        fs::rename(&tmp, path).context("failed to rename receipt store")?;
        Ok(())
    }

    pub fn total_pending_mist(&self) -> u64 {
        self.lines
            .iter()
            .filter(|l| !l.settled && !l.void)
            .map(|l| l.amount_mist)
            .sum()
    }

    pub fn ingest_backlog_count(&self) -> u64 {
        self.lines
            .iter()
            .filter(|l| !l.ingest_synced && !l.settled && !l.void)
            .count() as u64
    }

    pub fn oldest_ingest_backlog_ms(&self) -> Option<u64> {
        self.lines
            .iter()
            .filter(|l| !l.ingest_synced && !l.settled && !l.void)
            .map(|l| l.created_at_ms)
            .min()
    }

    pub fn balance_ids_with_pending(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .lines
            .iter()
            .filter(|l| !l.settled && !l.void)
            .map(|l| l.balance_id.clone())
            .collect();
        ids.sort();
        ids.dedup();
        ids
    }

    pub fn balance_pending_summary(&self, balance_id: &str) -> BalancePendingSummary {
        let pending: Vec<&UsageLine> = self.pending_for_balance(balance_id);
        let pending_mist = pending.iter().map(|l| l.amount_mist).sum();
        let oldest_timestamp_ms = pending.iter().map(|l| l.timestamp_ms).min();
        BalancePendingSummary {
            pending_mist,
            pending_count: pending.len() as u64,
            oldest_timestamp_ms,
        }
    }

    pub fn find_by_idempotency(
        &self,
        balance_id: &str,
        agent_object_id: &str,
        key: &str,
    ) -> Option<&UsageLine> {
        self.lines.iter().find(|l| {
            l.balance_id == balance_id
                && l.agent_object_id == agent_object_id
                && l.idempotency_key.as_deref() == Some(key)
        })
    }

    pub fn insert_pending(&mut self, line: UsageLine) -> Result<()> {
        if self.settled_ids.contains(&line.receipt_id) {
            anyhow::bail!("receipt_id already settled");
        }
        if self.lines.iter().any(|l| l.receipt_id == line.receipt_id) {
            anyhow::bail!("duplicate receipt_id");
        }
        self.lines.push(line);
        Ok(())
    }

    pub fn mark_ingest_synced(&mut self, receipt_id: u128) -> bool {
        if let Some(line) = self.lines.iter_mut().find(|l| l.receipt_id == receipt_id) {
            line.ingest_synced = true;
            true
        } else {
            false
        }
    }

    pub fn pending_for_balance(&self, balance_id: &str) -> Vec<&UsageLine> {
        self.lines
            .iter()
            .filter(|l| l.balance_id == balance_id && !l.settled && !l.void)
            .collect()
    }

    pub fn mark_settled(&mut self, receipt_ids: &[u128]) {
        for id in receipt_ids {
            self.settled_ids.insert(*id);
            if let Some(line) = self.lines.iter_mut().find(|l| l.receipt_id == *id) {
                line.settled = true;
            }
        }
    }

    /// Void a receipt that can no longer settle (approval revoked/expired after signing).
    pub fn mark_void(&mut self, receipt_id: u128) -> bool {
        if let Some(line) = self
            .lines
            .iter_mut()
            .find(|l| l.receipt_id == receipt_id && !l.settled)
        {
            line.void = true;
            true
        } else {
            false
        }
    }

    /// Renumber and update timestamps of the balance's pending lines so the sequence is
    /// contiguous starting at `base_nonce + 1`. Returns the lines (in order) that must be
    /// re-signed by the caller.
    pub fn renumber_pending_for_balance(
        &mut self,
        balance_id: &str,
        base_nonce: u64,
        now_ms: u64,
    ) -> Vec<usize> {
        let mut indices: Vec<usize> = self
            .lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.balance_id == balance_id && !l.settled && !l.void)
            .map(|(i, _)| i)
            .collect();
        indices.sort_by_key(|i| self.lines[*i].created_at_ms);
        let mut nonce = base_nonce;
        for i in &indices {
            nonce += 1;
            let line = &mut self.lines[*i];
            line.settlement_nonce = nonce;
            line.timestamp_ms = now_ms;
        }
        indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn sample_line(receipt_id: u128, key: &str) -> UsageLine {
        UsageLine {
            receipt_id,
            balance_id: "0xbal".to_string(),
            memory_account_id: "0xacc".to_string(),
            agent_object_id: "0xagent".to_string(),
            amount_mist: 100,
            usage_kind: 1,
            model_id: None,
            tool_id: None,
            metadata: None,
            signature_hex: "ab".to_string(),
            settlement_nonce: 1,
            timestamp_ms: 1,
            settled: false,
            created_at_ms: 1,
            void: false,
            organization_id: None,
            idempotency_key: Some(key.to_string()),
            ingest_synced: false,
        }
    }

    #[test]
    fn corrupt_load_fails_without_recover() {
        let path = std::env::temp_dir().join(format!(
            "myso-ai-credit-receipt-test-{}",
            std::process::id()
        ));
        let mut file = File::create(&path).unwrap();
        write!(file, "{{not valid json").unwrap();
        let err = ReceiptStore::load(&path, false).unwrap_err();
        let _ = fs::remove_file(&path);
        assert!(err.to_string().contains("corrupt"));
    }

    #[test]
    fn corrupt_load_recovers_when_enabled() {
        let path = std::env::temp_dir().join(format!(
            "myso-ai-credit-receipt-recover-{}",
            std::process::id()
        ));
        let mut file = File::create(&path).unwrap();
        write!(file, "{{not valid json").unwrap();
        let store = ReceiptStore::load(&path, true).unwrap();
        let _ = fs::remove_file(&path);
        assert!(store.lines.is_empty());
    }

    #[test]
    fn find_by_idempotency_returns_matching_line() {
        let store = ReceiptStore {
            lines: vec![sample_line(42, "idem-1")],
            settled_ids: HashSet::new(),
        };
        let found = store
            .find_by_idempotency("0xbal", "0xagent", "idem-1")
            .unwrap();
        assert_eq!(found.receipt_id, 42);
        assert!(store
            .find_by_idempotency("0xbal", "0xagent", "missing")
            .is_none());
    }
}
