// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Reads AiCreditBalance scalar fields directly from chain object BCS.

use anyhow::{Context, Result};
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::ObjectID;

use crate::ai_credit_object::parse_settlement_nonce;

pub async fn fetch_on_chain_settlement_nonce(rpc_url: &str, balance_id: &str) -> Result<u64> {
    let object_id = ObjectID::from_hex_literal(balance_id)?;
    let client = MySoClientBuilder::default().build(rpc_url).await?;
    let mut last_err = None;
    for attempt in 0..5 {
        if attempt > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
        match client
            .read_api()
            .get_move_object_bcs(object_id)
            .await
            .context("fetch AiCreditBalance BCS")
            .and_then(|data| {
                parse_settlement_nonce(&data).context("parse AiCreditBalance settlement_nonce")
            }) {
            Ok(nonce) => return Ok(nonce),
            Err(err) => last_err = Some(err),
        }
    }
    Err(last_err.unwrap_or_else(|| anyhow::anyhow!("fetch on-chain settlement_nonce failed")))
}

/// Resolve settlement nonce: on-chain BCS first, then indexed social-server value.
pub async fn resolve_settlement_nonce(
    rpc_url: &str,
    balance_id: &str,
    indexed_nonce: Option<u64>,
) -> Result<u64> {
    match fetch_on_chain_settlement_nonce(rpc_url, balance_id).await {
        Ok(nonce) => Ok(nonce),
        Err(chain_err) => {
            if let Some(indexed) = indexed_nonce {
                tracing::warn!(
                    balance_id = %balance_id,
                    error = %chain_err,
                    indexed_nonce = indexed,
                    "on-chain settlement_nonce fetch failed; using indexed value"
                );
                Ok(indexed)
            } else {
                Err(chain_err.context(format!(
                    "cannot resolve settlement_nonce for balance {balance_id}"
                )))
            }
        }
    }
}
