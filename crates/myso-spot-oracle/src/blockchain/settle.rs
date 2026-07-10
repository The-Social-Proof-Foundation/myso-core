// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Context;
use myso_json_rpc_types::{MySoObjectDataOptions, MySoTransactionBlockResponseOptions};
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::{MySoAddress, ObjectID};
use myso_types::crypto::MySoKeyPair;
use myso_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_types::transaction::{
    Command, ObjectArg, ProgrammableMoveCall, SharedObjectMutability, Transaction, TransactionData,
};
use myso_types::transaction_driver_types::ExecuteTransactionRequestType;
use tracing::info;

use crate::api::AppState;
use crate::blockchain::{
    chain_configured, parse_object_id, shared_object_arg, CLOCK_OBJECT_ID,
};
use crate::claim::lifecycle::{default_context_for, LifecycleEvent};
use crate::config::SOCIAL_PACKAGE_ID;
use crate::resolver::engine::{is_high_confidence, on_chain_status_for_resolve};
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
        .unwrap_or(9500);
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

    if evidence_urls.is_empty() {
        anyhow::bail!("oracle_resolve requires at least one evidence URL");
    }

    let high_confidence = is_high_confidence(confidence_bps, state.args.confidence_threshold_bps);

    if let Some(def_id) = market.resolver_definition_id {
        if let Some(def) = state.store.get_resolver_definition(def_id).await? {
            if chrono::Utc::now() < def.maturity_schedule.deadline {
                state
                    .store
                    .requeue_job(
                        job.id,
                        def.maturity_schedule.deadline,
                        "before resolution_at for chain submit",
                    )
                    .await?;
                return Ok(());
            }
        }
    }

    if !chain_configured(&state.args) {
        tracing::warn!(market_id = %market_id, "chain not configured — marking resolved off-chain");
        let event = LifecycleEvent::ResolveTxConfirmed { high_confidence };
        let mut ctx = default_context_for(&event);
        ctx.job_id = Some(job.id);
        ctx.on_chain_status = Some(on_chain_status_for_resolve(high_confidence));
        state
            .store
            .apply_market_transition(market_id, &event, &ctx)
            .await?;
        return Ok(());
    }

    let spot_market_object_id = market
        .spot_market_object_id
        .as_deref()
        .context("market missing spot_market_object_id for on-chain resolve")?;
    let claim_object_id = resolve_claim_object_id(&state, &market.post_id).await?;
    let platform_id = state
        .args
        .platform_object_id
        .as_deref()
        .context("SPOT_ORACLE_PLATFORM_OBJECT_ID required for oracle_resolve")?;
    let treasury_id = state
        .args
        .ecosystem_treasury_object_id
        .as_deref()
        .context("SPOT_ORACLE_ECOSYSTEM_TREASURY_OBJECT_ID required for oracle_resolve")?;

    let nonce = format!("resolve-{market_id}-{outcome_option_id}");
    let tx_id = state
        .store
        .insert_transaction(Some(market_id), "oracle_resolve", &nonce)
        .await?;

    match build_and_submit_resolve(
        &state.args,
        &claim_object_id,
        spot_market_object_id,
        &market.post_id,
        platform_id,
        treasury_id,
        outcome_option_id,
        confidence_bps,
        &reasoning,
        &evidence_urls,
    )
    .await
    {
        Ok(digest) => {
            state
                .store
                .update_transaction_status(tx_id, "confirmed", Some(&digest), None)
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["oracle_resolve", "confirmed"])
                .inc();
            let event = LifecycleEvent::ResolveTxConfirmed { high_confidence };
            let mut ctx = default_context_for(&event);
            ctx.job_id = Some(job.id);
            ctx.tx_digest = Some(digest);
            ctx.on_chain_status = Some(on_chain_status_for_resolve(high_confidence));
            state
                .store
                .apply_market_transition(market_id, &event, &ctx)
                .await?;
            info!(
                market_id = %market_id,
                digest = ctx.tx_digest.as_deref().unwrap_or(""),
                outcome_option_id,
                high_confidence,
                "oracle_resolve submitted"
            );
            Ok(())
        }
        Err(err) => {
            state
                .store
                .update_transaction_status(tx_id, "failed", None, Some(&err.to_string()))
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["oracle_resolve", "failed"])
                .inc();
            Err(err)
        }
    }
}

async fn resolve_claim_object_id(state: &AppState, post_id: &str) -> anyhow::Result<String> {
    let row: Option<(Option<String>,)> = sqlx::query_as(
        r#"
        SELECT c.spot_claim_object_id
        FROM post_claim_links l
        JOIN spot_claims c ON c.id = l.claim_id
        WHERE l.post_id = $1
        "#,
    )
    .bind(post_id)
    .fetch_optional(state.store.pool())
    .await?;
    row.and_then(|r| r.0)
        .context("missing on-chain claim object id for post")
}

async fn build_and_submit_resolve(
    args: &crate::config::OracleArgs,
    claim_object_id: &str,
    spot_market_id: &str,
    post_id: &str,
    platform_id: &str,
    treasury_id: &str,
    outcome_option_id: u8,
    confidence_bps: u64,
    reasoning: &str,
    evidence_urls: &[String],
) -> anyhow::Result<String> {
    let key_hex = args.private_key_hex.as_ref().context("missing private key")?;
    let key_bytes = hex::decode(key_hex.trim())?;
    let key_pair = MySoKeyPair::from_bytes(&key_bytes)
        .map_err(|e| anyhow::anyhow!("invalid key: {:?}", e))?;
    let sender = MySoAddress::from(&key_pair.public());

    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;

    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let gas_obj = gas_coins
        .data
        .first()
        .context("sender has no gas coins")?;

    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let admin_cap = parse_object_id(args.admin_cap_object_id.as_ref().unwrap())?;
    let spot_config = parse_object_id(args.spot_config_object_id.as_ref().unwrap())?;
    let registry_id = parse_object_id(args.spot_registry_object_id.as_ref().unwrap())?;
    let claim_obj = parse_object_id(claim_object_id)?;
    let market_obj = parse_object_id(spot_market_id)?;
    let post_obj = parse_object_id(post_id)?;
    let platform = parse_object_id(platform_id)?;
    let treasury = parse_object_id(treasury_id)?;
    let clock = ObjectID::from_hex_literal(CLOCK_OBJECT_ID)?;

    let admin_obj = client
        .read_api()
        .get_object_with_options(admin_cap, MySoObjectDataOptions::default())
        .await?
        .into_object()?;
    let admin_arg = ObjectArg::ImmOrOwnedObject(admin_obj.object_ref());

    let config_arg =
        shared_object_arg(&client, spot_config, SharedObjectMutability::Immutable).await?;
    let registry_arg =
        shared_object_arg(&client, registry_id, SharedObjectMutability::Mutable).await?;
    let claim_arg =
        shared_object_arg(&client, claim_obj, SharedObjectMutability::Immutable).await?;
    let market_arg =
        shared_object_arg(&client, market_obj, SharedObjectMutability::Mutable).await?;
    let post_arg = shared_object_arg(&client, post_obj, SharedObjectMutability::Immutable).await?;
    let platform_arg =
        shared_object_arg(&client, platform, SharedObjectMutability::Mutable).await?;
    let treasury_arg =
        shared_object_arg(&client, treasury, SharedObjectMutability::Immutable).await?;
    let clock_arg = shared_object_arg(&client, clock, SharedObjectMutability::Immutable).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let admin_input = ptb.obj(admin_arg)?;
    let config_input = ptb.obj(config_arg)?;
    let registry_input = ptb.obj(registry_arg)?;
    let claim_input = ptb.obj(claim_arg)?;
    let market_input = ptb.obj(market_arg)?;
    let post_input = ptb.obj(post_arg)?;
    let platform_input = ptb.obj(platform_arg)?;
    let treasury_input = ptb.obj(treasury_arg)?;
    let outcome_input = ptb.pure(outcome_option_id)?;
    let confidence_input = ptb.pure(confidence_bps)?;
    let reasoning_input = ptb.pure(reasoning.to_string())?;
    let evidence_input = ptb.pure(evidence_urls.to_vec())?;
    let clock_input = ptb.obj(clock_arg)?;
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "social_proof_of_truth".to_string(),
        function: "oracle_resolve".to_string(),
        type_arguments: vec![],
        arguments: vec![
            admin_input,
            config_input,
            registry_input,
            claim_input,
            market_input,
            post_input,
            platform_input,
            treasury_input,
            outcome_input,
            confidence_input,
            reasoning_input,
            evidence_input,
            clock_input,
        ],
    })));

    let pt = ptb.finish();
    let rgp = client.read_api().get_reference_gas_price().await?;
    let tx_data = TransactionData::new_programmable(sender, vec![gas_obj.object_ref()], pt, 50_000_000, rgp);
    let signed = Transaction::from_data_and_signer(tx_data, vec![&key_pair]);
    let response = client
        .quorum_driver_api()
        .execute_transaction_block(
            signed,
            MySoTransactionBlockResponseOptions::new(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    Ok(response.digest.to_string())
}
