// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Finalized on-chain reserve/capture/cancel transactions for gateway-owned inference.
//!
//! This module deliberately accepts typed values only. Callers cannot provide a Move
//! target or arbitrary programmable transaction, which keeps the public inference path
//! inside the audited `ai_credit` state machine.

use anyhow::{Context, Result};
use myso_json_rpc_types::{MySoTransactionBlockEffectsAPI, MySoTransactionBlockResponseOptions};
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
use crate::settlement::shared_object_arg;
use crate::signing::{
    CancelSpendIntent, CaptureSpendIntent, ReceiptSigner, SpendReservationIntent,
};

const CLOCK_OBJECT_ID: &str = "0x0000000000000000000000000000000000000000000000000000000000000006";
const GAS_BUDGET_MIST: u64 = 1_000_000_000;

#[derive(Debug, Clone)]
pub struct ReserveSpendRequest {
    pub balance_id: String,
    pub memory_account_id: String,
    pub agent_object_id: String,
    pub reservation_nonce: u64,
    pub max_amount_mist: u64,
    pub provider_envelope_hash: Vec<u8>,
    pub request_hash: Vec<u8>,
    pub fx_quote_id: Vec<u8>,
    pub myso_usd_e8: u64,
    pub markup_bps: u64,
    pub timestamp_ms: u64,
    pub capture_deadline_ms: u64,
    pub hard_expiry_ms: u64,
}

#[derive(Debug, Clone)]
pub struct CaptureSpendRequest {
    pub balance_id: String,
    pub agent_object_id: String,
    pub reservation_nonce: u64,
    pub amount_mist: u64,
    pub provider_cost_usd_micros: u64,
    pub provider_generation_hash: Vec<u8>,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct CancelSpendRequest {
    pub balance_id: String,
    pub agent_object_id: String,
    pub reservation_nonce: u64,
    pub timestamp_ms: u64,
}

pub async fn reserve_spend(
    args: &OracleArgs,
    oracle_signer: &ReceiptSigner,
    request: &ReserveSpendRequest,
) -> Result<String> {
    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;
    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let config = configured_object_id(args)?;
    let balance = ObjectID::from_hex_literal(&request.balance_id)?;
    let account = ObjectID::from_hex_literal(&request.memory_account_id)?;
    let agent = ObjectID::from_hex_literal(&request.agent_object_id)?;

    let intent = SpendReservationIntent {
        balance_id: object_id_bytes(&request.balance_id)?,
        agent_object_id: object_id_bytes(&request.agent_object_id)?,
        reservation_nonce: request.reservation_nonce,
        max_amount_mist: request.max_amount_mist,
        provider_envelope_hash: request.provider_envelope_hash.clone(),
        request_hash: request.request_hash.clone(),
        fx_quote_id: request.fx_quote_id.clone(),
        myso_usd_e8: request.myso_usd_e8,
        markup_bps: request.markup_bps,
        timestamp_ms: request.timestamp_ms,
        capture_deadline_ms: request.capture_deadline_ms,
        hard_expiry_ms: request.hard_expiry_ms,
    };
    let signature = oracle_signer.sign_reservation(&intent)?;

    let pt = build_reserve_ptb(
        package,
        shared_object_arg(&client, config, SharedObjectMutability::Immutable).await?,
        shared_object_arg(&client, balance, SharedObjectMutability::Mutable).await?,
        shared_object_arg(&client, account, SharedObjectMutability::Immutable).await?,
        shared_object_arg(&client, agent, SharedObjectMutability::Immutable).await?,
        shared_object_arg(
            &client,
            ObjectID::from_hex_literal(CLOCK_OBJECT_ID)?,
            SharedObjectMutability::Immutable,
        )
        .await?,
        request,
        signature,
    )?;
    submit_finalized(args, &client, pt, "reserve").await
}

pub async fn capture_spend(
    args: &OracleArgs,
    oracle_signer: &ReceiptSigner,
    request: &CaptureSpendRequest,
) -> Result<String> {
    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;
    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let config = configured_object_id(args)?;
    let balance = ObjectID::from_hex_literal(&request.balance_id)?;
    let intent = CaptureSpendIntent {
        balance_id: object_id_bytes(&request.balance_id)?,
        agent_object_id: object_id_bytes(&request.agent_object_id)?,
        reservation_nonce: request.reservation_nonce,
        amount_mist: request.amount_mist,
        provider_cost_usd_micros: request.provider_cost_usd_micros,
        provider_generation_hash: request.provider_generation_hash.clone(),
        timestamp_ms: request.timestamp_ms,
    };
    let signature = oracle_signer.sign_capture(&intent)?;
    let pt = build_capture_ptb(
        package,
        shared_object_arg(&client, config, SharedObjectMutability::Immutable).await?,
        shared_object_arg(&client, balance, SharedObjectMutability::Mutable).await?,
        shared_object_arg(
            &client,
            ObjectID::from_hex_literal(CLOCK_OBJECT_ID)?,
            SharedObjectMutability::Immutable,
        )
        .await?,
        request,
        signature,
    )?;
    submit_finalized(args, &client, pt, "capture").await
}

pub async fn cancel_spend(
    args: &OracleArgs,
    oracle_signer: &ReceiptSigner,
    request: &CancelSpendRequest,
) -> Result<String> {
    let client = MySoClientBuilder::default()
        .build(args.myso_rpc.clone())
        .await?;
    let package = ObjectID::from_hex_literal(SOCIAL_PACKAGE_ID)?;
    let config = configured_object_id(args)?;
    let balance = ObjectID::from_hex_literal(&request.balance_id)?;
    let intent = CancelSpendIntent {
        balance_id: object_id_bytes(&request.balance_id)?,
        agent_object_id: object_id_bytes(&request.agent_object_id)?,
        reservation_nonce: request.reservation_nonce,
        timestamp_ms: request.timestamp_ms,
    };
    let signature = oracle_signer.sign_cancel(&intent)?;
    let pt = build_cancel_ptb(
        package,
        shared_object_arg(&client, config, SharedObjectMutability::Immutable).await?,
        shared_object_arg(&client, balance, SharedObjectMutability::Mutable).await?,
        shared_object_arg(
            &client,
            ObjectID::from_hex_literal(CLOCK_OBJECT_ID)?,
            SharedObjectMutability::Immutable,
        )
        .await?,
        request,
        signature,
    )?;
    submit_finalized(args, &client, pt, "cancel").await
}

fn configured_object_id(args: &OracleArgs) -> Result<ObjectID> {
    ObjectID::from_hex_literal(
        args.config_object_id
            .as_deref()
            .context("AI_CREDIT_CONFIG_OBJECT_ID is required for inference reservations")?,
    )
    .context("invalid AI_CREDIT_CONFIG_OBJECT_ID")
}

fn object_id_bytes(value: &str) -> Result<[u8; 32]> {
    crate::signing::parse_object_id_hex(value)
}

async fn submit_finalized(
    args: &OracleArgs,
    client: &myso_sdk::MySoClient,
    pt: ProgrammableTransaction,
    operation: &'static str,
) -> Result<String> {
    let encoded = args
        .settlement_key_hex
        .as_deref()
        .context("AI_CREDIT_SETTLEMENT_KEY_HEX is required for inference reservations")?;
    let key_bytes = hex::decode(encoded.trim()).context("invalid settlement key hex")?;
    let key_pair = MySoKeyPair::from_bytes(&key_bytes)
        .map_err(|e| anyhow::anyhow!("invalid settlement key: {e:?}"))?;
    let sender = MySoAddress::from(&key_pair.public());
    let gas = client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?
        .data
        .into_iter()
        .next()
        .context("reservation transaction sender has no gas coins")?
        .object_ref();
    let rgp = client.read_api().get_reference_gas_price().await?;
    let data = TransactionData::new_programmable(sender, vec![gas], pt, GAS_BUDGET_MIST, rgp);
    let signed = Transaction::from_data_and_signer(data, vec![&key_pair]);
    let response = client
        .quorum_driver_api()
        .execute_transaction_block(
            signed,
            MySoTransactionBlockResponseOptions::new().with_effects(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    if !response.errors.is_empty() {
        anyhow::bail!("{operation} transaction errors: {:?}", response.errors);
    }
    let effects = response
        .effects
        .as_ref()
        .context("transaction response missing effects")?;
    if !effects.status().is_ok() {
        anyhow::bail!("{operation} transaction failed: {:?}", effects.status());
    }
    Ok(response.digest.to_string())
}

pub(crate) fn build_reserve_ptb(
    package: ObjectID,
    config: ObjectArg,
    balance: ObjectArg,
    account: ObjectArg,
    agent: ObjectArg,
    clock: ObjectArg,
    request: &ReserveSpendRequest,
    signature: Vec<u8>,
) -> Result<ProgrammableTransaction> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    let config = ptb.input(CallArg::Object(config))?;
    let balance = ptb.input(CallArg::Object(balance))?;
    let account = ptb.input(CallArg::Object(account))?;
    let agent = ptb.input(CallArg::Object(agent))?;
    let clock = ptb.input(CallArg::Object(clock))?;
    let pure = [
        bcs::to_bytes(&request.reservation_nonce)?,
        bcs::to_bytes(&request.max_amount_mist)?,
        bcs::to_bytes(&request.provider_envelope_hash)?,
        bcs::to_bytes(&request.request_hash)?,
        bcs::to_bytes(&request.fx_quote_id)?,
        bcs::to_bytes(&request.myso_usd_e8)?,
        bcs::to_bytes(&request.markup_bps)?,
        bcs::to_bytes(&request.timestamp_ms)?,
        bcs::to_bytes(&request.capture_deadline_ms)?,
        bcs::to_bytes(&request.hard_expiry_ms)?,
        bcs::to_bytes(&signature)?,
    ]
    .into_iter()
    .map(|value| ptb.input(CallArg::Pure(value)))
    .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut arguments = vec![config, balance, account, agent];
    arguments.extend(pure);
    arguments.push(clock);
    debug_assert_eq!(arguments.len(), 16);
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "ai_credit".into(),
        function: "reserve_signed_spend".into(),
        type_arguments: vec![],
        arguments,
    })));
    Ok(ptb.finish())
}

pub(crate) fn build_capture_ptb(
    package: ObjectID,
    config: ObjectArg,
    balance: ObjectArg,
    clock: ObjectArg,
    request: &CaptureSpendRequest,
    signature: Vec<u8>,
) -> Result<ProgrammableTransaction> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    let config = ptb.input(CallArg::Object(config))?;
    let balance = ptb.input(CallArg::Object(balance))?;
    let clock = ptb.input(CallArg::Object(clock))?;
    let pure = [
        bcs::to_bytes(&request.reservation_nonce)?,
        bcs::to_bytes(&request.amount_mist)?,
        bcs::to_bytes(&request.provider_cost_usd_micros)?,
        bcs::to_bytes(&request.provider_generation_hash)?,
        bcs::to_bytes(&request.timestamp_ms)?,
        bcs::to_bytes(&signature)?,
    ]
    .into_iter()
    .map(|value| ptb.input(CallArg::Pure(value)))
    .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut arguments = vec![config, balance];
    arguments.extend(pure);
    arguments.push(clock);
    debug_assert_eq!(arguments.len(), 9);
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "ai_credit".into(),
        function: "capture_reserved_spend".into(),
        type_arguments: vec![],
        arguments,
    })));
    Ok(ptb.finish())
}

pub(crate) fn build_cancel_ptb(
    package: ObjectID,
    config: ObjectArg,
    balance: ObjectArg,
    clock: ObjectArg,
    request: &CancelSpendRequest,
    signature: Vec<u8>,
) -> Result<ProgrammableTransaction> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    let config = ptb.input(CallArg::Object(config))?;
    let balance = ptb.input(CallArg::Object(balance))?;
    let clock = ptb.input(CallArg::Object(clock))?;
    let nonce = ptb.input(CallArg::Pure(bcs::to_bytes(&request.reservation_nonce)?))?;
    let timestamp = ptb.input(CallArg::Pure(bcs::to_bytes(&request.timestamp_ms)?))?;
    let signature = ptb.input(CallArg::Pure(bcs::to_bytes(&signature)?))?;
    ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
        package,
        module: "ai_credit".into(),
        function: "cancel_reserved_spend".into(),
        type_arguments: vec![],
        arguments: vec![config, balance, nonce, timestamp, signature, clock],
    })));
    Ok(ptb.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shared(id: &str, mutability: SharedObjectMutability) -> ObjectArg {
        ObjectArg::SharedObject {
            id: ObjectID::from_hex_literal(id).unwrap(),
            initial_shared_version: 1.into(),
            mutability,
        }
    }

    #[test]
    fn builders_target_only_registered_ai_credit_functions() {
        let reserve = ReserveSpendRequest {
            balance_id: "0x2".into(),
            memory_account_id: "0x3".into(),
            agent_object_id: "0x4".into(),
            reservation_nonce: 1,
            max_amount_mist: 10,
            provider_envelope_hash: vec![1; 32],
            request_hash: vec![2; 32],
            fx_quote_id: b"quote".to_vec(),
            myso_usd_e8: 450_000,
            markup_bps: 1500,
            timestamp_ms: 100,
            capture_deadline_ms: 200,
            hard_expiry_ms: 300,
        };
        let pt = build_reserve_ptb(
            ObjectID::from_hex_literal("0x50c1").unwrap(),
            shared("0x1", SharedObjectMutability::Immutable),
            shared("0x2", SharedObjectMutability::Mutable),
            shared("0x3", SharedObjectMutability::Immutable),
            shared("0x4", SharedObjectMutability::Immutable),
            shared("0x6", SharedObjectMutability::Immutable),
            &reserve,
            vec![0; 64],
        )
        .unwrap();
        assert_eq!(pt.commands.len(), 1);
        let Command::MoveCall(call) = &pt.commands[0] else {
            panic!("expected move call")
        };
        assert_eq!(call.module.as_str(), "ai_credit");
        assert_eq!(call.function.as_str(), "reserve_signed_spend");
        assert_eq!(call.arguments.len(), 16);
    }
}
