// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;

use crate::receipt::ReceiptStore;
use crate::social_client::{AiCreditBalanceResponse, SocialClient};

const BALANCE_CACHE_TTL: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
struct CachedBalance {
    response: AiCreditBalanceResponse,
    fetched_at: Instant,
}

#[derive(Clone)]
pub struct BalanceLedger {
    social: SocialClient,
    cache: Arc<tokio::sync::Mutex<Option<CachedBalance>>>,
    cache_owner: Arc<tokio::sync::Mutex<Option<String>>>,
}

impl BalanceLedger {
    pub fn new(social: SocialClient) -> Self {
        Self {
            social,
            cache: Arc::new(tokio::sync::Mutex::new(None)),
            cache_owner: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    pub async fn fetch_balance(&self, owner: &str) -> Result<Option<AiCreditBalanceResponse>> {
        let mut cached_owner = self.cache_owner.lock().await;
        let mut cache = self.cache.lock().await;
        if cached_owner.as_deref() == Some(owner) {
            if let Some(entry) = cache.as_ref() {
                if entry.fetched_at.elapsed() < BALANCE_CACHE_TTL {
                    return Ok(Some(entry.response.clone()));
                }
            }
        }
        let response = self.social.get_ai_credit_balance(owner).await?;
        if let Some(ref r) = response {
            *cache = Some(CachedBalance {
                response: r.clone(),
                fetched_at: Instant::now(),
            });
            *cached_owner = Some(owner.to_string());
        }
        Ok(response)
    }

    pub fn pending_mist(store: &ReceiptStore, balance_id: &str) -> u64 {
        store
            .lines
            .iter()
            .filter(|l| l.balance_id == balance_id && !l.settled && !l.void)
            .map(|l| l.amount_mist)
            .sum()
    }

    pub fn pending_count(store: &ReceiptStore, balance_id: &str) -> u64 {
        store
            .lines
            .iter()
            .filter(|l| l.balance_id == balance_id && !l.settled && !l.void)
            .count() as u64
    }

    pub fn effective_available_mist(balance: &AiCreditBalanceResponse, store: &ReceiptStore) -> u64 {
        let on_chain = balance.balance.balance_mist.max(0) as u64;
        let reserved = balance.balance.reserved_mist.max(0) as u64;
        let pending = Self::pending_mist(store, &balance.balance.balance_id);
        on_chain
            .saturating_sub(reserved)
            .saturating_sub(pending)
    }

    pub fn next_settlement_nonce(
        balance: &AiCreditBalanceResponse,
        store: &ReceiptStore,
        on_chain_nonce: Option<u64>,
    ) -> u64 {
        let indexed = balance.balance.settlement_nonce.max(0) as u64;
        let on_chain = on_chain_nonce.unwrap_or(0).max(indexed);
        let pending = Self::pending_count(store, &balance.balance.balance_id);
        on_chain + pending + 1
    }
}
