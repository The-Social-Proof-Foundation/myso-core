// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Reads AiCreditBalance scalar fields directly from chain object BCS.

use anyhow::{Context, Result};
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::ObjectID;

use crate::ai_credit_object::{parse_reservation_state, parse_settlement_nonce};

async fn fetch_balance_bcs(rpc_url: &str, balance_id: &str) -> Result<Vec<u8>> {
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
        {
            Ok(data) => return Ok(data),
            Err(err) => last_err = Some(err),
        }
    }
    Err(last_err.unwrap_or_else(|| anyhow::anyhow!("fetch on-chain AiCreditBalance failed")))
}

pub async fn fetch_on_chain_settlement_nonce(rpc_url: &str, balance_id: &str) -> Result<u64> {
    let data = fetch_balance_bcs(rpc_url, balance_id).await?;
    parse_settlement_nonce(&data).context("parse AiCreditBalance settlement_nonce")
}

/// Return the canonical reservation nonce and currently locked MIST directly
/// from chain state. The next reservation must use `reservation_nonce + 1`.
pub async fn fetch_on_chain_reservation_state(
    rpc_url: &str,
    balance_id: &str,
) -> Result<(u64, u64)> {
    let data = fetch_balance_bcs(rpc_url, balance_id).await?;
    parse_reservation_state(&data).context("parse AiCreditBalance reservation state")
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
