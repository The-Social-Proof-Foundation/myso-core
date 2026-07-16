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
use crate::blockchain::{chain_configured, parse_object_id, shared_object_arg, CLOCK_OBJECT_ID};
use crate::claim::lifecycle::{default_context_for, LifecycleEvent};
use crate::config::SOCIAL_PACKAGE_ID;
use crate::store::jobs::SpotJob;
use crate::types::OnChainSpotStatus;

pub async fn submit_refund_unresolved(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let market_id = job.market_id.context("refund missing market_id")?;
    let market = state
        .store
        .get_market(market_id)
        .await?
        .context("market not found")?;

    if matches!(
        crate::types::MarketStatus::from_str(&market.status),
        Some(
            crate::types::MarketStatus::Resolved
                | crate::types::MarketStatus::Refunded
                | crate::types::MarketStatus::Rejected
        )
    ) {
        return Ok(());
    }

    if !chain_configured(&state.args) {
        tracing::warn!(market_id = %market_id, "chain not configured — marking refunded off-chain");
        let mut ctx = default_context_for(&LifecycleEvent::RefundTxConfirmed);
        ctx.job_id = Some(job.id);
        ctx.on_chain_status = Some(OnChainSpotStatus::Refundable as i16);
        state
            .store
            .apply_market_transition(market_id, &LifecycleEvent::RefundTxConfirmed, &ctx)
            .await?;
        return Ok(());
    }

    let spot_market_object_id = market
        .spot_market_object_id
        .as_deref()
        .context("market missing spot_market_object_id for refund")?;

    let nonce = format!("refund-{market_id}");
    let tx_id = state
        .store
        .insert_transaction(Some(market_id), "refund_unresolved", &nonce)
        .await?;

    match build_and_submit_refund(&state.args, spot_market_object_id, &market.post_id).await {
        Ok(digest) => {
            state
                .store
                .update_transaction_status(tx_id, "confirmed", Some(&digest), None)
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["refund_unresolved", "confirmed"])
                .inc();
            let mut ctx = default_context_for(&LifecycleEvent::RefundTxConfirmed);
            ctx.job_id = Some(job.id);
            ctx.tx_digest = Some(digest.clone());
            ctx.on_chain_status = Some(OnChainSpotStatus::Refundable as i16);
            state
                .store
                .apply_market_transition(market_id, &LifecycleEvent::RefundTxConfirmed, &ctx)
                .await?;
            info!(market_id = %market_id, digest = %digest, "refund_unresolved confirmed");
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
                .with_label_values(&["refund_unresolved", "failed"])
                .inc();
            Err(err)
        }
    }
}

async fn build_and_submit_refund(
    args: &crate::config::OracleArgs,
    spot_market_object_id: &str,
    post_id: &str,
) -> anyhow::Result<String> {
    let key_hex = args
        .private_key_hex
        .as_ref()
        .context("missing private key")?;
    let key_bytes = hex::decode(key_hex.trim())?;
    let key_pair =
        MySoKeyPair::from_bytes(&key_bytes).map_err(|e| anyhow::anyhow!("invalid key: {:?}", e))?;
    let sender = MySoAddress::from(&key_pair.public());

    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;

    let gas_coins = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let gas_obj = gas_coins.data.first().context("sender has no gas coins")?;

    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let admin_cap = parse_object_id(args.admin_cap_object_id.as_ref().unwrap())?;
    let spot_config = parse_object_id(args.spot_config_object_id.as_ref().unwrap())?;
    let registry_id = parse_object_id(
        args.spot_registry_object_id
            .as_ref()
            .context("SPOT_ORACLE_REGISTRY_OBJECT_ID required for refund_unresolved")?,
    )?;
    let market = parse_object_id(spot_market_object_id)?;
    let post_obj = parse_object_id(post_id)?;
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
    let market_arg = shared_object_arg(&client, market, SharedObjectMutability::Mutable).await?;
    let post_arg = shared_object_arg(&client, post_obj, SharedObjectMutability::Immutable).await?;
    let clock_arg = shared_object_arg(&client, clock, SharedObjectMutability::Immutable).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let admin_input = ptb.obj(admin_arg)?;
    let config_input = ptb.obj(config_arg)?;
    let registry_input = ptb.obj(registry_arg)?;
    let market_input = ptb.obj(market_arg)?;
    let post_input = ptb.obj(post_arg)?;
    let clock_input = ptb.obj(clock_arg)?;
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "social_proof_of_truth".to_string(),
        function: "refund_unresolved".to_string(),
        type_arguments: vec![],
        arguments: vec![
            admin_input,
            config_input,
            registry_input,
            market_input,
            post_input,
            clock_input,
        ],
    })));

    let pt = ptb.finish();
    let rgp = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_obj.object_ref()], pt, 50_000_000, rgp);
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
