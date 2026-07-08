// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Context;

use crate::api::AppState;
use crate::blockchain::chain_configured;
use crate::store::jobs::SpotJob;

pub async fn submit_oracle_resolve(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let market_id = job.market_id.context("oracle_resolve missing market_id")?;
    let outcome_label = job
        .payload
        .get("outcome_label")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let confidence_bps = job
        .payload
        .get("confidence_bps")
        .and_then(|v| v.as_u64())
        .unwrap_or(9500) as u16;
    let reasoning = job
        .payload
        .get("reasoning")
        .and_then(|v| v.as_str())
        .unwrap_or("deterministic resolver outcome")
        .to_string();
    let evidence_urls: Vec<String> = job
        .payload
        .get("evidence_urls")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    let market = state
        .store
        .get_market(market_id)
        .await?
        .context("market not found")?;
    let options: Vec<String> = serde_json::from_value(market.betting_options.clone())?;
    let outcome_label = outcome_label.context("missing outcome_label")?;
    let outcome_option_id = options
        .iter()
        .position(|o| o == &outcome_label)
        .unwrap_or(0) as u8;

    if !chain_configured(&state.args) {
        tracing::warn!(market_id = %market_id, "chain not configured — marking resolved off-chain");
        state
            .store
            .update_market_status(market_id, "resolved", None, None, None)
            .await?;
        return Ok(());
    }

    let nonce = format!("resolve-{market_id}-{outcome_option_id}");
    let tx_id = state
        .store
        .insert_transaction(Some(market_id), "oracle_resolve", &nonce)
        .await?;

    // Full on-chain PTB requires spot_record, platform, treasury object IDs from session env.
    // V1 records the intent in `transactions` and marks resolved when chain objects are wired.
    tracing::info!(
        market_id = %market_id,
        outcome_option_id,
        confidence_bps,
        reasoning = %reasoning,
        evidence_count = evidence_urls.len(),
        "oracle_resolve PTB pending chain object wiring"
    );
    state
        .store
        .update_transaction_status(tx_id, "pending", None, Some("chain object ids not configured"))
        .await?;
    state
        .metrics
        .chain_tx_total
        .with_label_values(&["oracle_resolve", "pending"])
        .inc();
    Ok(())
}
