// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Spend-approval lookup with a short TTL cache (mirrors the `BalanceLedger` pattern).
//!
//! Reject-before-sign contract: an over-threshold usage request is rejected until a live,
//! sufficient allowance is indexed — no receipt (and no settlement nonce) is created for an
//! unapprovable spend, so approvals can never block the strictly-sequential nonce queue.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;

use crate::social_client::{SocialClient, SpendApprovalRow};

#[derive(Debug, Clone)]
struct CachedApproval {
    row: Option<SpendApprovalRow>,
    fetched_at: Instant,
}

#[derive(Clone)]
pub struct ApprovalsCache {
    social: SocialClient,
    ttl: Duration,
    cache: Arc<tokio::sync::Mutex<HashMap<(String, String), CachedApproval>>>,
}

impl ApprovalsCache {
    pub fn new(social: SocialClient, ttl_secs: u64) -> Self {
        Self {
            social,
            ttl: Duration::from_secs(ttl_secs.max(1)),
            cache: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Latest `approved` row for the agent, if any (cached; validity not checked here).
    pub async fn fetch_approved(
        &self,
        owner: &str,
        balance_id: &str,
        agent_object_id: &str,
    ) -> Result<Option<SpendApprovalRow>> {
        let key = (balance_id.to_string(), agent_object_id.to_string());
        {
            let cache = self.cache.lock().await;
            if let Some(entry) = cache.get(&key) {
                if entry.fetched_at.elapsed() < self.ttl {
                    return Ok(entry.row.clone());
                }
            }
        }
        let rows = self
            .social
            .get_spend_approvals(owner, Some(agent_object_id), Some("approved"))
            .await?;
        let row = rows
            .into_iter()
            .find(|r| r.balance_id == balance_id && r.agent_object_id == agent_object_id);
        let mut cache = self.cache.lock().await;
        cache.insert(
            key,
            CachedApproval {
                row: row.clone(),
                fetched_at: Instant::now(),
            },
        );
        Ok(row)
    }

    /// Drop the cached entry (used right after ingesting a requested row so a fresh
    /// approval is observed promptly on retry).
    pub async fn invalidate(&self, balance_id: &str, agent_object_id: &str) {
        let mut cache = self.cache.lock().await;
        cache.remove(&(balance_id.to_string(), agent_object_id.to_string()));
    }
}

/// A live allowance covers `amount_mist` when it is approved, unexpired for at least
/// `min_remaining_ms` (so it cannot expire before the settlement window), and large enough.
pub fn approval_covers(
    row: &SpendApprovalRow,
    amount_mist: u64,
    now_ms: i64,
    min_remaining_ms: i64,
) -> bool {
    if row.status != "approved" {
        return false;
    }
    let Some(expires_at_ms) = row.expires_at_ms else {
        return false;
    };
    if expires_at_ms < now_ms + min_remaining_ms {
        return false;
    }
    let Some(max_amount) = row.max_amount_mist else {
        return false;
    };
    max_amount >= 0 && (max_amount as u64) >= amount_mist
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(status: &str, max_amount: Option<i64>, expires_at: Option<i64>) -> SpendApprovalRow {
        SpendApprovalRow {
            balance_id: "0xbal".into(),
            agent_object_id: "0xagent".into(),
            status: status.into(),
            requested_amount_mist: None,
            threshold_mist: Some(100),
            approval_nonce: Some(1),
            max_amount_mist: max_amount,
            expires_at_ms: expires_at,
            approved_by: Some("0xowner".into()),
            approved_by_agent_id: None,
            organization_id: None,
        }
    }

    #[test]
    fn valid_approval_covers_amount() {
        let now = 1_000_000;
        assert!(approval_covers(
            &row("approved", Some(500), Some(now + 60_000)),
            500,
            now,
            30_000,
        ));
    }

    #[test]
    fn expired_or_soon_expiring_approval_rejected() {
        let now = 1_000_000;
        // Already expired.
        assert!(!approval_covers(
            &row("approved", Some(500), Some(now - 1)),
            500,
            now,
            0,
        ));
        // Expires inside the minimum remaining window.
        assert!(!approval_covers(
            &row("approved", Some(500), Some(now + 10_000)),
            500,
            now,
            30_000,
        ));
    }

    #[test]
    fn insufficient_or_wrong_status_rejected() {
        let now = 1_000_000;
        assert!(!approval_covers(
            &row("approved", Some(499), Some(now + 60_000)),
            500,
            now,
            0,
        ));
        assert!(!approval_covers(
            &row("requested", Some(500), Some(now + 60_000)),
            500,
            now,
            0,
        ));
        assert!(!approval_covers(
            &row("approved", None, Some(now + 60_000)),
            500,
            now,
            0
        ));
    }
}
