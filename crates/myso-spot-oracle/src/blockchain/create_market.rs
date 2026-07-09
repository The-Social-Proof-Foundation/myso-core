// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Context;
use myso_json_rpc_types::{
    MySoObjectDataOptions, MySoTransactionBlockResponseOptions, ObjectChange,
};
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::{MySoAddress, ObjectID};
use myso_types::crypto::MySoKeyPair;
use myso_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_types::transaction::{
    Command, ObjectArg, ProgrammableMoveCall, SharedObjectMutability, Transaction, TransactionData,
};
use myso_types::transaction_driver_types::ExecuteTransactionRequestType;
use tracing::info;
use uuid::Uuid;

use crate::api::AppState;
use crate::blockchain::{chain_configured, parse_object_id, shared_object_arg, CLOCK_OBJECT_ID};
use crate::config::SOCIAL_PACKAGE_ID;
use crate::store::jobs::SpotJob;

pub async fn submit_create_claim_market(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let oracle_market_id = job
        .market_id
        .context("create_claim_market missing market_id")?;
    let spot_claim_id = job
        .payload
        .get("spot_claim_id")
        .and_then(|v| v.as_str())
        .context("create_claim_market missing spot_claim_id")?
        .parse::<Uuid>()?;
    let spot_market_id = job
        .payload
        .get("spot_market_id")
        .and_then(|v| v.as_str())
        .context("create_claim_market missing spot_market_id")?
        .parse::<Uuid>()?;

    let market = state
        .store
        .get_market(oracle_market_id)
        .await?
        .context("oracle market not found")?;
    let spot_claim = state
        .store
        .get_claim_by_id(spot_claim_id)
        .await?
        .context("spot claim row not found")?;

    if !chain_configured(&state.args) {
        tracing::warn!(market_id = %oracle_market_id, "chain not configured — activating market off-chain");
        let mut ctx = crate::claim::lifecycle::default_context_for(
            &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
        );
        ctx.job_id = Some(job.id);
        ctx.on_chain_status = Some(1);
        state
            .store
            .apply_market_transition(
                oracle_market_id,
                &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
                &ctx,
            )
            .await?;
        return Ok(());
    }

    let options: Vec<String> = serde_json::from_value(market.betting_options.clone())?;

    let nonce = format!("create-claim-market-{oracle_market_id}");
    let tx_id = state
        .store
        .insert_transaction(Some(oracle_market_id), "create_claim_market", &nonce)
        .await?;

    match build_and_submit_create(
        &state.args,
        &market.post_id,
        &options,
        market.max_resolution_window_ms as u64,
    )
    .await
    {
        Ok((digest, claim_object_id, market_object_id)) => {
            state
                .store
                .update_transaction_status(tx_id, "confirmed", Some(&digest), None)
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["create_claim_market", "confirmed"])
                .inc();
            if spot_claim.spot_claim_object_id.is_none() {
                state
                    .store
                    .set_spot_claim_object_id(spot_claim_id, &claim_object_id)
                    .await?;
            }
            state
                .store
                .set_spot_market_object_id(spot_market_id, &market_object_id)
                .await?;
            state
                .store
                .set_spot_record_id(oracle_market_id, &market_object_id)
                .await?;
            let mut ctx = crate::claim::lifecycle::default_context_for(
                &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
            );
            ctx.job_id = Some(job.id);
            ctx.tx_digest = Some(digest.clone());
            ctx.on_chain_status = Some(1);
            state
                .store
                .apply_market_transition(
                    oracle_market_id,
                    &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
                    &ctx,
                )
                .await?;
            info!(
                market_id = %oracle_market_id,
                digest,
                claim_object_id,
                market_object_id,
                "create_claim_market submitted"
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
                .with_label_values(&["create_claim_market", "failed"])
                .inc();
            Err(err)
        }
    }
}

pub async fn submit_link_post(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let oracle_market_id = job.market_id.context("link_post missing market_id")?;
    let spot_claim_id = job
        .payload
        .get("spot_claim_id")
        .and_then(|v| v.as_str())
        .context("link_post missing spot_claim_id")?
        .parse::<Uuid>()?;

    let market = state
        .store
        .get_market(oracle_market_id)
        .await?
        .context("oracle market not found")?;
    let spot_claim = state
        .store
        .get_claim_by_id(spot_claim_id)
        .await?
        .context("spot claim row not found")?;
    let claim_object_id = spot_claim
        .spot_claim_object_id
        .as_deref()
        .context("link_post requires on-chain claim object id")?;

    if !chain_configured(&state.args) {
        tracing::warn!(market_id = %oracle_market_id, "chain not configured — link_post off-chain only");
        return Ok(());
    }

    let nonce = format!("link-post-{oracle_market_id}");
    let tx_id = state
        .store
        .insert_transaction(Some(oracle_market_id), "link_post", &nonce)
        .await?;

    match build_and_submit_link_post(&state.args, claim_object_id, &market.post_id).await {
        Ok(digest) => {
            state
                .store
                .update_transaction_status(tx_id, "confirmed", Some(&digest), None)
                .await?;
            state
                .metrics
                .chain_tx_total
                .with_label_values(&["link_post", "confirmed"])
                .inc();
            info!(market_id = %oracle_market_id, digest, "link_post submitted");
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
                .with_label_values(&["link_post", "failed"])
                .inc();
            Err(err)
        }
    }
}

pub async fn submit_create_market(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    submit_create_claim_market(state, job).await
}

async fn build_and_submit_create(
    args: &crate::config::OracleArgs,
    post_id: &str,
    betting_options: &[String],
    max_resolution_window_ms: u64,
) -> anyhow::Result<(String, String, String)> {
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
    let post_arg = shared_object_arg(&client, post_obj, SharedObjectMutability::Mutable).await?;
    let clock_arg = shared_object_arg(&client, clock, SharedObjectMutability::Immutable).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let admin_input = ptb.obj(admin_arg)?;
    let config_input = ptb.obj(config_arg)?;
    let registry_input = ptb.obj(registry_arg)?;
    let post_input = ptb.obj(post_arg)?;
    let options_input = ptb.pure(betting_options.to_vec())?;
    let rw_input = ptb.pure(Option::<u64>::None)?;
    let max_rw_input = ptb.pure(Some(max_resolution_window_ms))?;
    let clock_input = ptb.obj(clock_arg)?;
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "social_proof_of_truth".to_string(),
        function: "create_spot_record_for_post".to_string(),
        type_arguments: vec![],
        arguments: vec![
            admin_input,
            config_input,
            registry_input,
            post_input,
            options_input,
            rw_input,
            max_rw_input,
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
            MySoTransactionBlockResponseOptions::new().with_object_changes(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let digest = response.digest.to_string();
    let claim_object_id = find_created_type(&response.object_changes, "SpotClaim")
        .context("create_spot_record_for_post did not create SpotClaim")?;
    let market_object_id = find_created_type(&response.object_changes, "SpotMarket")
        .context("create_spot_record_for_post did not create SpotMarket")?;

    Ok((digest, claim_object_id, market_object_id))
}

async fn build_and_submit_link_post(
    args: &crate::config::OracleArgs,
    claim_object_id: &str,
    post_id: &str,
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
    let registry_id = parse_object_id(args.spot_registry_object_id.as_ref().unwrap())?;
    let claim_obj = parse_object_id(claim_object_id)?;
    let post_obj = parse_object_id(post_id)?;
    let clock = ObjectID::from_hex_literal(CLOCK_OBJECT_ID)?;

    let admin_obj = client
        .read_api()
        .get_object_with_options(admin_cap, MySoObjectDataOptions::default())
        .await?
        .into_object()?;
    let admin_arg = ObjectArg::ImmOrOwnedObject(admin_obj.object_ref());
    let registry_arg =
        shared_object_arg(&client, registry_id, SharedObjectMutability::Mutable).await?;
    let claim_arg =
        shared_object_arg(&client, claim_obj, SharedObjectMutability::Mutable).await?;
    let post_arg = shared_object_arg(&client, post_obj, SharedObjectMutability::Mutable).await?;
    let clock_arg = shared_object_arg(&client, clock, SharedObjectMutability::Immutable).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let admin_input = ptb.obj(admin_arg)?;
    let registry_input = ptb.obj(registry_arg)?;
    let claim_input = ptb.obj(claim_arg)?;
    let post_input = ptb.obj(post_arg)?;
    let clock_input = ptb.obj(clock_arg)?;

    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "social_proof_of_truth".to_string(),
        function: "link_post_to_spot_claim".to_string(),
        type_arguments: vec![],
        arguments: vec![
            admin_input,
            registry_input,
            claim_input,
            post_input,
            clock_input,
        ],
    })));

    let pt = ptb.finish();
    let rgp = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_obj.object_ref()], pt, 30_000_000, rgp);
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

fn find_created_type(
    changes: &Option<Vec<ObjectChange>>,
    type_name: &str,
) -> Option<String> {
    changes.as_ref().into_iter().flatten().find_map(|change| match change {
        ObjectChange::Created { object_type, object_id, .. }
            if object_type.name.as_str() == type_name =>
        {
            Some(object_id.to_canonical_string(true))
        }
        _ => None,
    })
}
