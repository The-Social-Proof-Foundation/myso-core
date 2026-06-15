// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Example PTB for paid DM policy and send flow.
//!
//! Shared-object requirements are documented in `crates/myso-framework/docs/messaging_ptb.md`.
//! Before submitting, clients should gate off-chain: block check → follow check → min-cost policy.

mod utils;
use anyhow::anyhow;
use myso_config::{MYSO_KEYSTORE_FILENAME, myso_config_dir};
use myso_keys::keystore::{AccountKeystore, FileBasedKeystore};
use myso_sdk::{
    rpc_types::MySoTransactionBlockResponseOptions,
    types::{
        base_types::ObjectID,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{
            Argument, CallArg, Command, ObjectArg, SharedObjectMutability, Transaction,
            TransactionData,
        },
        transaction_driver_types::ExecuteTransactionRequestType,
        Identifier,
    },
};
use myso_types::MYSO_MESSAGING_PACKAGE_ID;
use shared_crypto::intent::Intent;
use utils::setup_for_write;

/// Placeholder object IDs — replace with live shared object IDs from your network.
/// Fetch via RPC (`getObject`) or indexer; see messaging_ptb.md for object roles.
struct MessagingSharedObjects {
    version: ObjectID,
    namespace: ObjectID,
    group_manager: ObjectID,
    paid_messaging_registry: ObjectID,
    social_graph: ObjectID,
    block_list_registry: ObjectID,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (myso, sender, _recipient) = setup_for_write().await?;

    let coins = myso
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?;
    let coin = coins.data.into_iter().next().unwrap();

    // TODO: resolve these from chain state for your deployment.
    let shared = MessagingSharedObjects {
        version: ObjectID::ZERO,
        namespace: ObjectID::ZERO,
        group_manager: ObjectID::ZERO,
        paid_messaging_registry: ObjectID::ZERO,
        social_graph: ObjectID::ZERO,
        block_list_registry: ObjectID::ZERO,
    };

    let package = (*MYSO_MESSAGING_PACKAGE_ID).into();
    let module = Identifier::new("paid_messaging_policy").map_err(|e| anyhow!(e))?;

    let mut ptb = ProgrammableTransactionBuilder::new();

    // set_paid_messaging_policy(registry, enabled, min_cost)
    ptb.input(CallArg::Object(ObjectArg::SharedObject {
        id: shared.paid_messaging_registry,
        initial_shared_version: 1u64.into(),
        mutability: SharedObjectMutability::Mutable,
    }))?;
    ptb.input(CallArg::Pure(bcs::to_bytes(&true)?))?;
    ptb.input(CallArg::Pure(bcs::to_bytes(&Some(1_000_000u64))?))?;
    ptb.command(Command::move_call(
        package,
        module.clone(),
        Identifier::new("set_paid_messaging_policy").map_err(|e| anyhow!(e))?,
        vec![],
        vec![
            Argument::Input(0),
            Argument::Input(1),
            Argument::Input(2),
        ],
    ));

    // send_paid_message_digest requires Version, MessagingNamespace, GroupManager,
    // PaidMessagingRegistry, SocialGraph, BlockListRegistry, plus group/message args.
    let _ = (
        shared.version,
        shared.namespace,
        shared.group_manager,
        shared.social_graph,
        shared.block_list_registry,
    );

    let builder = ptb.finish();
    let gas_budget = 10_000_000;
    let gas_price = myso.read_api().get_reference_gas_price().await?;
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![coin.object_ref()],
        builder,
        gas_budget,
        gas_price,
    );

    let keystore =
        FileBasedKeystore::load_or_create(&myso_config_dir()?.join(MYSO_KEYSTORE_FILENAME))?;
    let signature = keystore
        .sign_secure(&sender, &tx_data, Intent::myso_transaction())
        .await?;

    print!("Executing set_paid_messaging_policy...");
    let transaction_response = myso
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            MySoTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    println!("{}", transaction_response);
    Ok(())
}
