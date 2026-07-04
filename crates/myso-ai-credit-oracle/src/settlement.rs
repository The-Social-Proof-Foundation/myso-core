// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Submits batched `settle_signed_usage` PTBs via MySo RPC when settlement keys are configured.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use myso_json_rpc_types::MySoObjectDataOptions;
use myso_json_rpc_types::MySoTransactionBlockEffectsAPI;
use myso_json_rpc_types::MySoTransactionBlockResponseOptions;
use myso_sdk::MySoClientBuilder;
use myso_types::base_types::{MySoAddress, ObjectID};
use myso_types::crypto::MySoKeyPair;
use myso_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_types::transaction::{
    CallArg, Command, ObjectArg, ProgrammableMoveCall, ProgrammableTransaction,
    SharedObjectMutability, Transaction, TransactionData,
};
use myso_types::transaction_driver_types::ExecuteTransactionRequestType;

use crate::config::{OracleArgs, SOCIAL_PACKAGE_ID};
use crate::receipt::{ReceiptStore, UsageLine};
use crate::signing::{parse_object_id_hex, ReceiptSigner, UsageReceipt};

const CLOCK_OBJECT_ID: &str = "0x0000000000000000000000000000000000000000000000000000000000000006";
const MAX_BATCH: usize = 16;
const SETTLE_SIGNED_USAGE: &str = "settle_signed_usage";
const SETTLE_SIGNED_USAGE_ARG_COUNT: usize = 11;

/// Move abort codes in `social_contracts::ai_credit` that mean "allowance no longer
/// covers this receipt": EApprovalRequired=18, EApprovalExpired=19, EApprovalInsufficient=20.
const APPROVAL_ABORT_CODES: [&str; 3] = [", 18)", ", 19)", ", 20)"];

#[derive(Hash, Eq, PartialEq, Clone)]
struct BatchKey {
    balance_id: String,
    memory_account_id: String,
    agent_object_id: String,
}

/// Scalar receipt fields passed as PTB pure args (no struct serialization).
#[derive(Debug, Clone)]
pub(crate) struct SettlementLinePure {
    pub receipt_id: u128,
    pub amount_mist: u64,
    pub usage_kind: u8,
    pub timestamp_ms: u64,
    pub settlement_nonce: u64,
    pub signature: Vec<u8>,
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

    let balance_filter: HashMap<String, ()> =
        balance_ids.iter().map(|id| (id.clone(), ())).collect();
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
    let mut store_dirty = false;
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
                    let err_str = err.to_string();
                    // Revocation race recovery: the allowance was revoked or expired
                    // between signing and settlement. Void the aborting receipt (it can
                    // never settle) and re-sign the balance's remaining pending lines
                    // with a contiguous nonce sequence so the queue is unblocked.
                    if let Some(cmd_idx) = parse_approval_abort_command(&err_str) {
                        if let Some(failed) = chunk.get(cmd_idx) {
                            let voided_receipt = failed.receipt_id;
                            store.mark_void(voided_receipt);
                            let recovered =
                                resign_pending_for_balance(args, store, &key.balance_id).await;
                            match recovered {
                                Ok(resigned) => {
                                    store_dirty = true;
                                    tracing::warn!(
                                        balance_id = %key.balance_id,
                                        agent_object_id = %key.agent_object_id,
                                        voided_receipt = %voided_receipt,
                                        resigned,
                                        trigger,
                                        "voided unapprovable receipt and re-signed pending queue"
                                    );
                                }
                                Err(resign_err) => {
                                    tracing::warn!(
                                        balance_id = %key.balance_id,
                                        error = %resign_err,
                                        "void succeeded but resign failed; will retry next cycle"
                                    );
                                }
                            }
                        }
                        break;
                    }
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

    if settled_count > 0 || store_dirty {
        store.save(store_path)?;
        tracing::info!(
            settled = settled_count,
            trigger,
            "settlement cycle complete"
        );
    }
    Ok(settled_count)
}

/// Extracts the failing command index from an execution-failure debug string when the
/// abort is an ai_credit approval error (codes 18/19/20). Command index maps 1:1 to the
/// chunk line order in [`build_settlement_ptb`].
pub(crate) fn parse_approval_abort_command(err: &str) -> Option<usize> {
    if !err.contains("ai_credit") {
        return None;
    }
    if !APPROVAL_ABORT_CODES.iter().any(|code| err.contains(code)) {
        return None;
    }
    let marker = "command_index: ";
    let idx_str = if let Some(pos) = err.rfind(marker) {
        &err[pos + marker.len()..]
    } else if let Some(pos) = err.rfind(" in command ") {
        &err[pos + " in command ".len()..]
    } else {
        return None;
    };
    let digits: String = idx_str.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

/// Re-signs all pending (unsettled, non-void) lines for a balance with a contiguous
/// nonce sequence anchored to the on-chain settlement nonce, refreshing receipt
/// timestamps so they stay inside the receipt TTL. Returns the number of re-signed lines.
async fn resign_pending_for_balance(
    args: &OracleArgs,
    store: &mut ReceiptStore,
    balance_id: &str,
) -> Result<usize> {
    let on_chain_nonce =
        crate::chain_balance::fetch_on_chain_settlement_nonce(&args.myso_rpc, balance_id)
            .await
            .unwrap_or(0);
    let signer = ReceiptSigner::from_hex(&args.private_key_hex)?;
    let now_ms = chrono::Utc::now().timestamp_millis() as u64;
    let indices = store.renumber_pending_for_balance(balance_id, on_chain_nonce, now_ms);
    for i in &indices {
        let line = &store.lines[*i];
        let receipt = UsageReceipt {
            balance_id: parse_object_id_hex(&line.balance_id)?,
            agent_object_id: parse_object_id_hex(&line.agent_object_id)?,
            receipt_id: line.receipt_id,
            amount_mist: line.amount_mist,
            usage_kind: line.usage_kind,
            timestamp_ms: line.timestamp_ms,
            settlement_nonce: line.settlement_nonce,
        };
        let signature = signer.sign_receipt(&receipt)?;
        store.lines[*i].signature_hex = hex::encode(signature);
    }
    Ok(indices.len())
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

    let config_arg =
        shared_object_arg(client, config_object, SharedObjectMutability::Immutable).await?;
    let balance_arg =
        shared_object_arg(client, balance_id, SharedObjectMutability::Mutable).await?;
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

    let pure_lines: Vec<SettlementLinePure> = lines
        .iter()
        .map(|line| {
            let sig = hex::decode(&line.signature_hex).context("invalid signature hex in store")?;
            Ok(SettlementLinePure {
                receipt_id: line.receipt_id,
                amount_mist: line.amount_mist,
                usage_kind: line.usage_kind,
                timestamp_ms: line.timestamp_ms,
                settlement_nonce: line.settlement_nonce,
                signature: sig,
            })
        })
        .collect::<Result<_>>()?;
    let receipt_ids: Vec<u128> = lines.iter().map(|l| l.receipt_id).collect();

    let pt = build_settlement_ptb(
        package,
        config_arg,
        balance_arg,
        account_arg,
        agent_arg,
        clock_arg,
        &pure_lines,
    )?;
    let rgp = client.read_api().get_reference_gas_price().await?;
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_obj_ref], pt, 1_000_000_000, rgp);
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
    if let Some(effects) = response.effects.as_ref() {
        if !effects.status().is_ok() {
            anyhow::bail!("settlement tx failed: {:?}", effects.status());
        }
    } else if response.status_ok() == Some(false) {
        anyhow::bail!("settlement tx failed: missing effects");
    }

    tracing::info!(
        digest = %response.digest,
        count = receipt_ids.len(),
        "settlement batch submitted"
    );
    Ok(receipt_ids)
}

pub(crate) fn build_settlement_ptb(
    package: ObjectID,
    config_arg: ObjectArg,
    balance_arg: ObjectArg,
    account_arg: ObjectArg,
    agent_arg: ObjectArg,
    clock_arg: ObjectArg,
    lines: &[SettlementLinePure],
) -> Result<ProgrammableTransaction> {
    anyhow::ensure!(!lines.is_empty(), "settlement batch must not be empty");

    let mut ptb = ProgrammableTransactionBuilder::new();
    let config_input = ptb.input(CallArg::Object(config_arg))?;
    let balance_input = ptb.input(CallArg::Object(balance_arg))?;
    let account_input = ptb.input(CallArg::Object(account_arg))?;
    let agent_input = ptb.input(CallArg::Object(agent_arg))?;
    let clock_input = ptb.input(CallArg::Object(clock_arg))?;

    for line in lines {
        let receipt_id_input = ptb.input(CallArg::Pure(bcs::to_bytes(&line.receipt_id)?))?;
        let amount_input = ptb.input(CallArg::Pure(bcs::to_bytes(&line.amount_mist)?))?;
        let usage_kind_input = ptb.input(CallArg::Pure(bcs::to_bytes(&line.usage_kind)?))?;
        let timestamp_input = ptb.input(CallArg::Pure(bcs::to_bytes(&line.timestamp_ms)?))?;
        let nonce_input = ptb.input(CallArg::Pure(bcs::to_bytes(&line.settlement_nonce)?))?;
        let signature_input = ptb.input(CallArg::Pure(bcs::to_bytes(&line.signature)?))?;

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package,
            module: "ai_credit".into(),
            function: SETTLE_SIGNED_USAGE.into(),
            type_arguments: vec![],
            arguments: {
                let args = vec![
                    config_input,
                    balance_input,
                    account_input,
                    agent_input,
                    receipt_id_input,
                    amount_input,
                    usage_kind_input,
                    timestamp_input,
                    nonce_input,
                    signature_input,
                    clock_input,
                ];
                debug_assert_eq!(args.len(), SETTLE_SIGNED_USAGE_ARG_COUNT);
                args
            },
        })));
    }

    Ok(ptb.finish())
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

#[cfg(test)]
mod tests {
    use super::*;
    use myso_types::transaction::CallArg;

    fn dummy_shared(literal: &str) -> ObjectArg {
        ObjectArg::SharedObject {
            id: ObjectID::from_hex_literal(literal).unwrap(),
            initial_shared_version: 1.into(),
            mutability: SharedObjectMutability::Immutable,
        }
    }

    #[test]
    fn settlement_ptb_uses_settle_signed_usage_with_primitive_pure_args() {
        let package = ObjectID::from_hex_literal("0x50c1").unwrap();
        let lines = vec![
            SettlementLinePure {
                receipt_id: 1,
                amount_mist: 500_000_000,
                usage_kind: 1,
                timestamp_ms: 1_700_000_000_000,
                settlement_nonce: 1,
                signature: vec![0u8; 64],
            },
            SettlementLinePure {
                receipt_id: 2,
                amount_mist: 100_000_000,
                usage_kind: 2,
                timestamp_ms: 1_700_000_000_001,
                settlement_nonce: 2,
                signature: vec![1u8; 64],
            },
        ];

        let pt = build_settlement_ptb(
            package,
            dummy_shared("0x1"),
            dummy_shared("0x2"),
            dummy_shared("0x3"),
            dummy_shared("0x4"),
            dummy_shared("0x6"),
            &lines,
        )
        .unwrap();

        assert_eq!(pt.commands.len(), 2);
        for command in &pt.commands {
            let Command::MoveCall(call) = command else {
                panic!("expected MoveCall command");
            };
            assert_eq!(call.function, SETTLE_SIGNED_USAGE);
            assert_eq!(call.arguments.len(), SETTLE_SIGNED_USAGE_ARG_COUNT);
        }

        let pure_inputs: Vec<_> = pt
            .inputs
            .iter()
            .filter_map(|input| match input {
                CallArg::Pure(bytes) => Some(bytes.clone()),
                _ => None,
            })
            .collect();
        // 5 shared objects + 6 pure args per receipt × 2 receipts
        assert_eq!(pure_inputs.len(), 12);

        for bytes in pure_inputs {
            assert!(
                !bytes.is_empty(),
                "pure args must not be empty BCS payloads"
            );
        }
    }

    #[test]
    fn approval_abort_parser_extracts_command_index() {
        let err = "settlement tx failed: Failure { error: MoveAbort(MoveLocation { module: ModuleId { address: 50c1, name: Identifier(\"ai_credit\") }, function: 42, instruction: 7, function_name: Some(\"maybe_consume_spend_approval\") }, 18) in command 3 }";
        assert_eq!(parse_approval_abort_command(err), Some(3));

        let expired = err.replace(", 18)", ", 19)");
        assert_eq!(parse_approval_abort_command(&expired), Some(3));

        // Non-approval abort code from ai_credit is not recovered.
        let other = err.replace(", 18)", ", 4)");
        assert_eq!(parse_approval_abort_command(&other), None);

        // Approval-like code from another module is not recovered.
        let other_module = err.replace("ai_credit", "post");
        assert_eq!(parse_approval_abort_command(&other_module), None);
    }
}
