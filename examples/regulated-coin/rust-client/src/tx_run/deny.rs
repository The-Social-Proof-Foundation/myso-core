// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::{anyhow, Result};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use shared_crypto::intent::{Intent, IntentMessage};
use myso_sdk::rpc_types::{
    MySoObjectDataFilter, MySoObjectDataOptions, MySoObjectResponseQuery, MySoTransactionBlockResponse,
    MySoTransactionBlockResponseOptions,
};
use myso_sdk::types::base_types::{ObjectID, ObjectRef, SequenceNumber, MySoAddress};
use myso_sdk::types::coin::COIN_MODULE_NAME;
use myso_sdk::types::crypto::{Signature, MySoKeyPair};
use myso_sdk::types::object::Owner;
use myso_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_sdk::types::transaction_driver_types::ExecuteTransactionRequestType;
use myso_sdk::types::transaction::{Command, ObjectArg, Transaction, TransactionData};
use myso_sdk::types::{
    TypeTag, MYSO_DENY_LIST_OBJECT_ID, MYSO_FRAMEWORK_ADDRESS, MYSO_FRAMEWORK_PACKAGE_ID,
};
use myso_sdk::MySoClient;
use tracing::info;

use super::AppCommand;
use crate::gas::select_gas;

pub async fn get_deny_list(client: &MySoClient) -> Result<(ObjectID, SequenceNumber)> {
    let resp = client
        .read_api()
        .get_object_with_options(
            MYSO_DENY_LIST_OBJECT_ID,
            MySoObjectDataOptions {
                show_type: true,
                show_owner: true,
                show_previous_transaction: false,
                show_display: false,
                show_content: false,
                show_bcs: false,
                show_storage_rebate: false,
            },
        )
        .await?;
    let deny_list = resp.data.ok_or(anyhow!("No deny-list found!"))?;
    let Some(Owner::Shared {
        initial_shared_version,
    }) = deny_list.owner
    else {
        return Err(anyhow!("Invalid deny-list owner!"));
    };
    Ok((MYSO_DENY_LIST_OBJECT_ID, initial_shared_version))
}

pub async fn get_deny_cap(
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
                    name: Identifier::from_str("DenyCap")?,
                    type_params: vec![type_tag],
                })),
                options: None,
            }),
            None,
            None,
        )
        .await?;

    let deny_cap = resp
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("No deny-cap found!"))?;
    Ok(deny_cap.data.ok_or(anyhow!("DenyCap empty!"))?.object_ref())
}

#[derive(Debug, Copy, Clone)]
pub enum DenyListCommand {
    Add(MySoAddress),
    Remove(MySoAddress),
}

impl TryFrom<AppCommand> for DenyListCommand {
    type Error = anyhow::Error;

    fn try_from(cmd: AppCommand) -> Result<Self> {
        match cmd {
            AppCommand::DenyListAdd(address) => Ok(DenyListCommand::Add(address)),
            AppCommand::DenyListRemove(address) => Ok(DenyListCommand::Remove(address)),
            _ => Err(anyhow!("Invalid command for deny list")),
        }
    }
}

impl DenyListCommand {
    pub fn address(&self) -> MySoAddress {
        match self {
            DenyListCommand::Add(addr) => *addr,
            DenyListCommand::Remove(addr) => *addr,
        }
    }
}

impl ToString for DenyListCommand {
    fn to_string(&self) -> String {
        match self {
            DenyListCommand::Add(_) => "deny_list_add",
            DenyListCommand::Remove(_) => "deny_list_remove",
        }
        .to_string()
    }
}
// docs::#deny
pub async fn deny_list_add(
    client: &MySoClient,
    signer: &MySoKeyPair,
    otw_type: TypeTag,
    deny_list: (ObjectID, SequenceNumber),
    deny_cap: ObjectRef,
    addr: MySoAddress,
) -> Result<MySoTransactionBlockResponse> {
    info!("ADDING {addr} TO DENY_LIST");
    deny_list_cmd(
        client,
        signer,
        DenyListCommand::Add(addr),
        otw_type,
        deny_list,
        deny_cap,
    )
    .await
}

pub async fn deny_list_remove(
    client: &MySoClient,
    signer: &MySoKeyPair,
    otw_type: TypeTag,
    deny_list: (ObjectID, SequenceNumber),
    deny_cap: ObjectRef,
    addr: MySoAddress,
) -> Result<MySoTransactionBlockResponse> {
    info!("REMOVING {addr} FROM DENY_LIST");
    deny_list_cmd(
        client,
        signer,
        DenyListCommand::Remove(addr),
        otw_type,
        deny_list,
        deny_cap,
    )
    .await
}
// docs::/#deny
async fn deny_list_cmd(
    client: &MySoClient,
    signer: &MySoKeyPair,
    cmd: DenyListCommand,
    otw_type: TypeTag,
    deny_list: (ObjectID, SequenceNumber),
    deny_cap: ObjectRef,
) -> Result<MySoTransactionBlockResponse> {
    let signer_addr = MySoAddress::from(&signer.public());
    let gas_data = select_gas(client, signer_addr, None, None, vec![], None).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();

    let deny_list = ptb.obj(ObjectArg::SharedObject {
        id: deny_list.0,
        initial_shared_version: deny_list.1,
        mutable: true,
    })?;
    let deny_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(deny_cap))?;
    let address = ptb.pure(cmd.address())?;
    ptb.command(Command::move_call(
        MYSO_FRAMEWORK_PACKAGE_ID,
        Identifier::from(COIN_MODULE_NAME),
        Identifier::from_str(&cmd.to_string())?,
        vec![otw_type],
        vec![deny_list, deny_cap, address],
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
                .with_object_changes()
                .with_input(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(res)
}
