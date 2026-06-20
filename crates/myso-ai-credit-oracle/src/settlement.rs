// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Submits batched `settle_usage_batch` PTBs via MySo RPC when settlement keys are configured.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use myso_json_rpc_types::MySoObjectDataOptions;
use myso_json_rpc_types::MySoTransactionBlockResponseOptions;
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::{MySoAddress, ObjectID};
use myso_types::crypto::MySoKeyPair;
use myso_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_types::transaction::{
    CallArg, Command, ObjectArg, ProgrammableMoveCall, SharedObjectMutability, Transaction,
    TransactionData,
};
use myso_types::transaction_driver_types::ExecuteTransactionRequestType;

use crate::config::{OracleArgs, SOCIAL_PACKAGE_ID};
use crate::receipt::{ReceiptStore, UsageLine};
use crate::signing::{parse_object_id_hex, UsageReceipt};

const CLOCK_OBJECT_ID: &str = "0x0000000000000000000000000000000000000000000000000000000000000006";
const MAX_BATCH: usize = 16;

#[derive(Hash, Eq, PartialEq, Clone)]
struct BatchKey {
    balance_id: String,
    memory_account_id: String,
    agent_object_id: String,
}

pub async fn run_settlement_cycle(
    args: &OracleArgs,
    store: &mut ReceiptStore,
    store_path: &Path,
    balance_ids: &[String],
    trigger: &str,
) -> Result<usize> {
    let package_id = SOCIAL_PACKAGE_ID;
    let config_id = args
        .config_object_id
        .as_ref()
        .context("AI_CREDIT_CONFIG_OBJECT_ID required for settlement")?;
    let settlement_key_hex = args
        .settlement_key_hex
        .as_ref()
        .context("AI_CREDIT_SETTLEMENT_KEY_HEX required for settlement")?;

    let balance_filter: HashMap<String, ()> = balance_ids.iter().map(|id| (id.clone(), ())).collect();
    let pending: Vec<UsageLine> = store
        .lines
        .iter()
        .filter(|l| !l.settled && balance_filter.contains_key(&l.balance_id))
        .cloned()
        .collect();
    if pending.is_empty() {
        return Ok(0);
    }

    tracing::info!(
        trigger,
        balance_count = balance_ids.len(),
        pending_lines = pending.len(),
        pending_mist = pending.iter().map(|l| l.amount_mist).sum::<u64>(),
        "starting settlement cycle"
    );

    let key_bytes = hex::decode(settlement_key_hex.trim())?;
    let key_pair = MySoKeyPair::from_bytes(&key_bytes)
        .map_err(|e| anyhow::anyhow!("invalid settlement key: {:?}", e))?;
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
        .context("settlement sender has no gas coins")?;
    let gas_obj_ref = gas_obj.object_ref();

    let package = ObjectID::from_hex_literal(package_id)?;
    let config_object = ObjectID::from_hex_literal(config_id)?;

    let mut groups: HashMap<BatchKey, Vec<UsageLine>> = HashMap::new();
    for line in pending {
        let key = BatchKey {
            balance_id: line.balance_id.clone(),
            memory_account_id: line.memory_account_id.clone(),
            agent_object_id: line.agent_object_id.clone(),
        };
        groups.entry(key).or_default().push(line);
    }

    let mut settled_count = 0usize;
    for (key, lines) in groups {
        let mut remaining = lines;
        while !remaining.is_empty() {
            let chunk: Vec<UsageLine> = remaining.drain(..remaining.len().min(MAX_BATCH)).collect();
            let chunk_refs: Vec<&UsageLine> = chunk.iter().collect();
            match submit_batch(
                &client,
                &key_pair,
                sender,
                gas_obj_ref,
                package,
                config_object,
                &chunk_refs,
            )
            .await
            {
                Ok(receipt_ids) => {
                    store.mark_settled(&receipt_ids);
                    settled_count += receipt_ids.len();
                }
                Err(err) => {
                    tracing::warn!(
                        balance_id = %key.balance_id,
                        agent_object_id = %key.agent_object_id,
                        error = %err,
                        trigger,
                        "settlement batch failed"
                    );
                    break;
                }
            }
        }
    }

    if settled_count > 0 {
        store.save(store_path)?;
        tracing::info!(settled = settled_count, trigger, "settlement cycle complete");
    }
    Ok(settled_count)
}

async fn submit_batch(
    client: &myso_sdk::MySoClient,
    key_pair: &MySoKeyPair,
    sender: myso_types::base_types::MySoAddress,
    gas_obj_ref: myso_types::base_types::ObjectRef,
    package: ObjectID,
    config_object: ObjectID,
    lines: &[&UsageLine],
) -> Result<Vec<u128>> {
    let balance_id = ObjectID::from_hex_literal(&lines[0].balance_id)?;
    let memory_account_id = ObjectID::from_hex_literal(&lines[0].memory_account_id)?;
    let agent_object_id = ObjectID::from_hex_literal(&lines[0].agent_object_id)?;

    let config_arg = shared_object_arg(client, config_object, SharedObjectMutability::Immutable).await?;
    let balance_arg = shared_object_arg(client, balance_id, SharedObjectMutability::Mutable).await?;
    let account_arg =
        shared_object_arg(client, memory_account_id, SharedObjectMutability::Immutable).await?;
    let agent_arg =
        shared_object_arg(client, agent_object_id, SharedObjectMutability::Immutable).await?;
    let clock_arg = shared_object_arg(
        client,
        ObjectID::from_hex_literal(CLOCK_OBJECT_ID)?,
        SharedObjectMutability::Immutable,
    )
    .await?;

    let mut receipts = Vec::with_capacity(lines.len());
    let mut signatures = Vec::with_capacity(lines.len());
    let mut receipt_ids = Vec::with_capacity(lines.len());

    for line in lines {
        let receipt = UsageReceipt {
            balance_id: parse_object_id_hex(&line.balance_id)?,
            agent_object_id: parse_object_id_hex(&line.agent_object_id)?,
            receipt_id: line.receipt_id,
            amount_mist: line.amount_mist,
            usage_kind: line.usage_kind,
            timestamp_ms: line.timestamp_ms,
            settlement_nonce: line.settlement_nonce,
        };
        let sig = hex::decode(&line.signature_hex).context("invalid signature hex in store")?;
        receipts.push(receipt);
        signatures.push(sig);
        receipt_ids.push(line.receipt_id);
    }

    let mut ptb = ProgrammableTransactionBuilder::new();
    let config_input = ptb.input(CallArg::Object(config_arg))?;
    let balance_input = ptb.input(CallArg::Object(balance_arg))?;
    let account_input = ptb.input(CallArg::Object(account_arg))?;
    let agent_input = ptb.input(CallArg::Object(agent_arg))?;
    let receipts_input = ptb.input(CallArg::Pure(bcs::to_bytes(&receipts)?))?;
    let signatures_input = ptb.input(CallArg::Pure(bcs::to_bytes(&signatures)?))?;
    let clock_input = ptb.input(CallArg::Object(clock_arg))?;

    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "ai_credit".into(),
        function: "settle_usage_batch".into(),
        type_arguments: vec![],
        arguments: vec![
            config_input,
            balance_input,
            account_input,
            agent_input,
            receipts_input,
            signatures_input,
            clock_input,
        ],
    })));

    let pt = ptb.finish();
    let rgp = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_obj_ref], pt, 50_000_000_000, rgp);
    let signed = Transaction::from_data_and_signer(tx_data, vec![key_pair]);
    let response = client
        .quorum_driver_api()
        .execute_transaction_block(
            signed,
            MySoTransactionBlockResponseOptions::new(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    if !response.errors.is_empty() {
        anyhow::bail!("settlement tx errors: {:?}", response.errors);
    }
    if response.status_ok() == Some(false) {
        anyhow::bail!("settlement tx failed: {:?}", response.effects);
    }

    tracing::info!(
        digest = %response.digest,
        count = receipt_ids.len(),
        "settlement batch submitted"
    );
    Ok(receipt_ids)
}

async fn shared_object_arg(
    client: &myso_sdk::MySoClient,
    object_id: ObjectID,
    mutability: SharedObjectMutability,
) -> Result<ObjectArg> {
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
