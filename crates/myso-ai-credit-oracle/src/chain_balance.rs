// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Reads AiCreditBalance scalar fields directly from chain object BCS.

use anyhow::{Context, Result};
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::ObjectID;

/// Fixed byte offset to `settlement_nonce` in `AiCreditBalance` BCS (before `agent_budgets`).
fn settlement_nonce_offset(data: &[u8]) -> Result<usize> {
    let mut off = 32 + 32 + 32 + 32 + 8; // uid, ids, Balance<MYSO>.value
    off += 8 * 5; // spent_total, spent_day, spent_month, day_anchor, month_anchor
    off = read_option_u64_skip(data, off)?;
    off = read_option_u64_skip(data, off)?;
    Ok(off)
}

fn read_option_u64_skip(data: &[u8], off: usize) -> Result<usize> {
    let tag = *data.get(off).context("truncated AiCreditBalance BCS")?;
    if tag == 0 {
        Ok(off + 1)
    } else {
        Ok(off + 1 + 8)
    }
}

fn read_u64_le(data: &[u8], off: usize) -> Result<u64> {
    let bytes: [u8; 8] = data
        .get(off..off + 8)
        .context("truncated AiCreditBalance u64 field")?
        .try_into()
        .context("invalid u64 slice")?;
    Ok(u64::from_le_bytes(bytes))
}

pub fn parse_settlement_nonce(data: &[u8]) -> Result<u64> {
    let off = settlement_nonce_offset(data)?;
    read_u64_le(data, off)
}

pub async fn fetch_on_chain_settlement_nonce(rpc_url: &str, balance_id: &str) -> Result<u64> {
    let object_id = ObjectID::from_hex_literal(balance_id)?;
    let client = MySoClientBuilder::default().build(rpc_url).await?;
    let data = client
        .read_api()
        .get_move_object_bcs(object_id)
        .await
        .context("fetch AiCreditBalance BCS")?;
    parse_settlement_nonce(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_settlement_nonce_from_known_layout() {
        let mut data = vec![0u8; 186];
        // Both Option<u64> caps are None (tag 0); settlement_nonce follows at offset 178.
        data[178] = 3;
        assert_eq!(parse_settlement_nonce(&data).unwrap(), 3);
    }
}
