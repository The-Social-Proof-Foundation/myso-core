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
use crate::blockchain::{chain_configured, parse_object_id};
use crate::config::SOCIAL_PACKAGE_ID;
use crate::store::jobs::SpotJob;

const CLOCK_OBJECT_ID: &str = "0x0000000000000000000000000000000000000000000000000000000000000006";

pub async fn submit_create_market(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let market_id = job.market_id.context("create_market missing market_id")?;
    let market = state
        .store
        .get_market(market_id)
        .await?
        .context("market not found")?;

    if !chain_configured(&state.args) {
        tracing::warn!(market_id = %market_id, "chain not configured — activating market off-chain");
        state
            .store
            .update_market_status(market_id, "active", None, None, None)
            .await?;
        return Ok(());
    }

    let options: Vec<String> = serde_json::from_value(market.betting_options.clone())?;
    let digest = build_and_submit_create(
        &state.args,
        &market.post_id,
        &options,
        market.resolution_window_ms as u64,
        market.max_resolution_window_ms as u64,
    )
    .await?;

    let nonce = format!("create-{market_id}");
    let tx_id = state
        .store
        .insert_transaction(Some(market_id), "create_market", &nonce)
        .await?;
    state
        .store
        .update_transaction_status(tx_id, "confirmed", Some(&digest), None)
        .await?;
    state
        .metrics
        .chain_tx_total
        .with_label_values(&["create_market", "confirmed"])
        .inc();
    state
        .store
        .update_market_status(market_id, "active", None, None, None)
        .await?;
    info!(market_id = %market_id, digest, "create_spot_record_for_post submitted");
    Ok(())
}

async fn build_and_submit_create(
    args: &crate::config::OracleArgs,
    post_id: &str,
    betting_options: &[String],
    resolution_window_ms: u64,
    max_resolution_window_ms: u64,
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
    let post_obj = parse_object_id(post_id)?;
    let clock = ObjectID::from_hex_literal(CLOCK_OBJECT_ID)?;

    let admin_obj = client
        .read_api()
        .get_object_with_options(admin_cap, MySoObjectDataOptions::default())
        .await?
        .into_object()?;
    let admin_arg = ObjectArg::ImmOrOwnedObject(admin_obj.object_ref());

    let config_arg = shared_object_arg(&client, spot_config, SharedObjectMutability::Immutable).await?;
    let post_arg = shared_object_arg(&client, post_obj, SharedObjectMutability::Mutable).await?;
    let clock_arg = shared_object_arg(&client, clock, SharedObjectMutability::Immutable).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let admin_input = ptb.obj(admin_arg)?;
    let config_input = ptb.obj(config_arg)?;
    let post_input = ptb.obj(post_arg)?;
    let options_input = ptb.pure(betting_options.to_vec())?;
    let rw_input = ptb.pure(Some(resolution_window_ms))?;
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
            post_input,
            options_input,
            rw_input,
            max_rw_input,
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

async fn shared_object_arg(
    client: &myso_sdk::MySoClient,
    object_id: ObjectID,
    mutability: SharedObjectMutability,
) -> anyhow::Result<ObjectArg> {
    let object = client
        .read_api()
        .get_object_with_options(object_id, MySoObjectDataOptions::new().with_owner())
        .await?;
    let data = object.data.as_ref().context("object missing data")?;
    let initial_shared_version = match data.owner.as_ref() {
        Some(myso_types::object::Owner::Shared {
            initial_shared_version,
        }) => *initial_shared_version,
        other => anyhow::bail!("object {:?} is not shared: {:?}", object_id, other),
    };
    Ok(ObjectArg::SharedObject {
        id: object_id,
        initial_shared_version,
        mutability,
    })
}
