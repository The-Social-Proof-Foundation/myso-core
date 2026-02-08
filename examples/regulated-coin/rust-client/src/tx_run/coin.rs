// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::{anyhow, Result};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use shared_crypto::intent::{Intent, IntentMessage};
use myso_sdk::rpc_types::{
    MySoObjectDataFilter, MySoObjectDataOptions, MySoObjectResponseQuery, MySoTransactionBlockResponse,
    MySoTransactionBlockResponseOptions,
};
use myso_sdk::types::base_types::{ObjectID, ObjectRef, MySoAddress};
use myso_sdk::types::coin::{COIN_MODULE_NAME, COIN_TREASURE_CAP_NAME};
use myso_sdk::types::crypto::{Signature, MySoKeyPair};
use myso_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_sdk::types::transaction_driver_types::ExecuteTransactionRequestType;
use myso_sdk::types::transaction::{Argument, Command, ObjectArg, Transaction, TransactionData};
use myso_sdk::types::{MYSO_FRAMEWORK_ADDRESS, MYSO_FRAMEWORK_PACKAGE_ID};
use myso_sdk::MySoClient;
use tracing::info;

use crate::gas::select_gas;

pub async fn get_treasury_cap(
    client: &MySoClient,
    owner_addr: MySoAddress,
    type_tag: TypeTag,
) -> Result<ObjectRef> {
    let resp = client
        .read_api()
        .get_owned_objects(
            owner_addr,
            Some(MySoObjectResponseQuery {
                filter: Some(MySoObjectDataFilter::StructType(StructTag {
                    address: MYSO_FRAMEWORK_ADDRESS,
                    module: Identifier::from(COIN_MODULE_NAME),
                    name: Identifier::from(COIN_TREASURE_CAP_NAME),
                    type_params: vec![type_tag],
                })),
                options: None,
            }),
            None,
            None,
        )
        .await?;

    let treasury_cap = resp
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("No deny-cap found!"))?;
    Ok(treasury_cap
        .data
        .ok_or(anyhow!("DenyCap empty!"))?
        .object_ref())
}

pub async fn get_coin(client: &MySoClient, id: ObjectID) -> Result<ObjectRef> {
    let resp = client
        .read_api()
        .get_object_with_options(
            id,
            MySoObjectDataOptions {
                // Note that we could have the type-tag from here and transfer in a moment
                show_type: false,
                show_owner: false,
                show_previous_transaction: false,
                show_display: false,
                show_content: false,
                show_bcs: false,
                show_storage_rebate: false,
            },
        )
        .await?;

    Ok(resp.data.ok_or(anyhow!("No coin found"))?.object_ref())
}

// docs::#mint
pub async fn mint_and_transfer(
    client: &MySoClient,
    signer: &MySoKeyPair,
    type_tag: TypeTag,
    treasury_cap: ObjectRef,
    to_address: MySoAddress,
    balance: u64,
) -> Result<MySoTransactionBlockResponse> {
    info!("MINTING COIN OF BALANCE {balance} TO ADDRESS {to_address}");
    let signer_addr = MySoAddress::from(&signer.public());
    let gas_data = select_gas(client, signer_addr, None, None, vec![], None).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();

    let treasury_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(treasury_cap))?;
    let balance = ptb.pure(balance)?;
    ptb.command(Command::move_call(
        MYSO_FRAMEWORK_PACKAGE_ID,
        Identifier::from(COIN_MODULE_NAME),
        Identifier::from_str("mint")?,
        vec![type_tag],
        vec![treasury_cap, balance],
    ));
    ptb.transfer_arg(to_address, Argument::Result(0));

    let builder = ptb.finish();

    // Sign transaction
    let msg = IntentMessage {
        intent: Intent::myso_transaction(),
        value: TransactionData::new_programmable(
            signer_addr,
            vec![gas_data.object],
            builder,
            gas_data.budget,
            gas_data.price,
        ),
    };
    let sig = Signature::new_secure(&msg, signer);

    let res = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(msg.value, vec![sig]),
            MySoTransactionBlockResponseOptions::new()
                .with_effects()
                .with_object_changes()
                .with_input(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(res)
}
// docs::/#mint

pub async fn transfer(
    client: &MySoClient,
    signer: &MySoKeyPair,
    coin: ObjectRef,
    to_address: MySoAddress,
) -> Result<MySoTransactionBlockResponse> {
    info!("TRANSFERRING COIN {} TO ADDRESS {to_address}", coin.0);
    let signer_addr = MySoAddress::from(&signer.public());
    let gas_data = select_gas(client, signer_addr, None, None, vec![], None).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();

    let coin = ptb.obj(ObjectArg::ImmOrOwnedObject(coin))?;
    ptb.transfer_arg(to_address, coin);

    let builder = ptb.finish();

    // Sign transaction
    let msg = IntentMessage {
        intent: Intent::myso_transaction(),
        value: TransactionData::new_programmable(
            signer_addr,
            vec![gas_data.object],
            builder,
            gas_data.budget,
            gas_data.price,
        ),
    };
    let sig = Signature::new_secure(&msg, signer);

    let res = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(msg.value, vec![sig]),
            MySoTransactionBlockResponseOptions::new()
                .with_effects()
                .with_object_changes()
                .with_input(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(res)
}

pub(crate) async fn burn(
    client: &MySoClient,
    signer: &MySoKeyPair,
    type_tag: TypeTag,
    treasury_cap: ObjectRef,
    coin: ObjectRef,
) -> Result<MySoTransactionBlockResponse> {
    info!("BURNING COIN {}", coin.0);
    let signer_addr = MySoAddress::from(&signer.public());
    let gas_data = select_gas(client, signer_addr, None, None, vec![], None).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();

    let treasury_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(treasury_cap))?;
    let coin = ptb.obj(ObjectArg::ImmOrOwnedObject(coin))?;
    ptb.command(Command::move_call(
        MYSO_FRAMEWORK_PACKAGE_ID,
        Identifier::from(COIN_MODULE_NAME),
        Identifier::from_str("burn")?,
        vec![type_tag],
        vec![treasury_cap, coin],
    ));

    let builder = ptb.finish();

    // Sign transaction
    let msg = IntentMessage {
        intent: Intent::myso_transaction(),
        value: TransactionData::new_programmable(
            signer_addr,
            vec![gas_data.object],
            builder,
            gas_data.budget,
            gas_data.price,
        ),
    };
    let sig = Signature::new_secure(&msg, signer);

    let res = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(msg.value, vec![sig]),
            MySoTransactionBlockResponseOptions::new()
                .with_effects()
                .with_input(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(res)

}
