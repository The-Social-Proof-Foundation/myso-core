// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

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

impl ReceiptStore {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let bytes = fs::read(path)?;
        Ok(serde_json::from_slice(&bytes).unwrap_or_default())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let bytes = serde_json::to_vec_pretty(self)?;
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, &bytes).context("failed to write receipt store tmp")?;
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
