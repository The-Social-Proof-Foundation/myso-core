// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use prost_types::FieldMask;
use serde::{Deserialize, Serialize};
use myso_rpc::client::Client;
use myso_rpc::proto::myso::rpc::v2::{
    BatchGetObjectsRequest, GetObjectRequest, Object, get_object_result,
};

use myso_rpc::field::FieldMaskUtil;
use myso_rpc::proto::myso::rpc::v2::{
    GasPayment, ObjectReference, ProgrammableTransaction as ProtoProgrammableTransaction,
    SimulateTransactionRequest, Transaction, TransactionKind,
    simulate_transaction_request::TransactionChecks, transaction_kind,
};
use myso_types::base_types::{ObjectID, ObjectRef, SequenceNumber, MySoAddress};
use myso_types::transaction::{ProgrammableTransaction, TransactionData};

use crate::errors::Error;
use crate::types::ConstructionMetadata;
pub use pay_coin::PayCoin;
use pay_coin::pay_coin_pt;
pub use pay_myso::PayMySo;
use pay_myso::pay_myso_pt;
pub use stake::Stake;
use stake::stake_pt;
pub use withdraw_stake::WithdrawStake;
use withdraw_stake::withdraw_stake_pt;

mod pay_coin;
mod pay_myso;
mod stake;
mod withdraw_stake;

pub const MAX_GAS_COINS: usize = 255;
const MAX_COMMAND_ARGS: usize = 511;

pub struct TransactionObjectData {
    pub gas_coins: Vec<ObjectRef>,
    /// For PayMySo/Stake: extra gas coins to merge into gas
    /// For PayCoin: payment coins of the specified type
    /// For WithdrawStake: stake objects to withdraw
    pub objects: Vec<ObjectRef>,
    /// Party-owned (ConsensusAddress) version of objects
    pub party_objects: Vec<(ObjectID, SequenceNumber)>,
    /// Refers to the sum of the `Coin<MYSO>` balance of the coins participating in the transaction;
    /// either as gas or as objects.
    pub total_myso_balance: i128,
    pub budget: u64,
}

#[async_trait]
#[enum_dispatch]
pub trait TryConstructTransaction {
    async fn try_fetch_needed_objects(
        self,
        client: &mut Client,
        gas_price: Option<u64>,
        budget: Option<u64>,
    ) -> Result<TransactionObjectData, Error>;
}

#[enum_dispatch(TryConstructTransaction)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InternalOperation {
    PayMySo(PayMySo),
    PayCoin(PayCoin),
    Stake(Stake),
    WithdrawStake(WithdrawStake),
}

impl InternalOperation {
    pub fn sender(&self) -> MySoAddress {
        match self {
            InternalOperation::PayMySo(PayMySo { sender, .. })
            | InternalOperation::PayCoin(PayCoin { sender, .. })
            | InternalOperation::Stake(Stake { sender, .. })
            | InternalOperation::WithdrawStake(WithdrawStake { sender, .. }) => *sender,
        }
    }

    /// Combine with ConstructionMetadata to form the TransactionData
    pub fn try_into_data(self, metadata: ConstructionMetadata) -> Result<TransactionData, Error> {
        let pt = match self {
            Self::PayMySo(PayMySo {
                recipients,
                amounts,
                ..
            }) => {
                // For backwards compatibility: prefer objects (new format), fallback to extra_gas_coins (old format)
                let coins_to_merge = if !metadata.objects.is_empty() {
                    &metadata.objects
                } else {
                    &metadata.extra_gas_coins
                };
                pay_myso_pt(recipients, amounts, coins_to_merge, &metadata.party_objects)?
            }
            Self::PayCoin(PayCoin {
                recipients,
                amounts,
                ..
            }) => {
                let currency = &metadata
                    .currency
                    .ok_or(anyhow!("metadata.coin_type is needed to PayCoin"))?;
                pay_coin_pt(
                    recipients,
                    amounts,
                    &metadata.objects,
                    &metadata.party_objects,
                    currency,
                )?
            }
            InternalOperation::Stake(Stake {
                validator, amount, ..
            }) => {
                let (stake_all, amount) = match amount {
                    Some(amount) => (false, amount),
                    None => {
                        if (metadata.total_coin_value - metadata.budget as i128) < 0 {
                            return Err(anyhow!(
                                "ConstructionMetadata malformed. total_coin_value - budget < 0"
                            )
                            .into());
                        }
                        (true, metadata.total_coin_value as u64 - metadata.budget)
                    }
                };
                // For backwards compatibility: prefer objects (new format), fallback to extra_gas_coins (old format)
                let coins_to_merge = if !metadata.objects.is_empty() {
                    &metadata.objects
                } else {
                    &metadata.extra_gas_coins
                };
                stake_pt(
                    validator,
                    amount,
                    stake_all,
                    coins_to_merge,
                    &metadata.party_objects,
                )?
            }
            InternalOperation::WithdrawStake(WithdrawStake { stake_ids, .. }) => {
                let withdraw_all = stake_ids.is_empty();
                withdraw_stake_pt(metadata.objects, withdraw_all)?
            }
        };

        Ok(TransactionData::new_programmable(
            metadata.sender,
            metadata.gas_coins,
            pt,
            metadata.budget,
            metadata.gas_price,
        ))
    }
}

/// RPC auto-selects gas coins if empty, uses reference gas price if None, and estimates budget if None.
/// Returns the resolved budget and gas coins used by the transaction.
async fn simulate_transaction(
    client: &mut Client,
    pt: ProgrammableTransaction,
    sender: MySoAddress,
    gas_coins: Vec<ObjectRef>,
    gas_price: Option<u64>,
    budget: Option<u64>,
) -> Result<(u64, Vec<Object>), Error> {
    let ptb_proto: ProtoProgrammableTransaction = pt.into();
    let mut transaction = Transaction::default()
        .with_kind(
            TransactionKind::default()
                .with_programmable_transaction(ptb_proto)
                .with_kind(transaction_kind::Kind::ProgrammableTransaction),
        )
        .with_sender(sender.to_string());

    let mut gas_payment = GasPayment::default();
    gas_payment.objects = gas_coins
        .into_iter()
        .map(|gas_ref| {
            let mut obj_ref = ObjectReference::default();
            obj_ref.object_id = Some(gas_ref.0.to_string());
            obj_ref.version = Some(gas_ref.1.value());
            obj_ref.digest = Some(gas_ref.2.to_string());
            obj_ref
        })
        .collect();
    gas_payment.budget = budget;
    gas_payment.price = gas_price;
    gas_payment.owner = Some(sender.to_string());
    transaction.gas_payment = Some(gas_payment);

    let request = SimulateTransactionRequest::default()
        .with_transaction(transaction)
        .with_read_mask(FieldMask::from_paths([
            "transaction.effects.status",
            "transaction.transaction.gas_payment",
        ]))
        .with_checks(TransactionChecks::Enabled)
        .with_do_gas_selection(true);

    let response = client
        .execution_client()
        .simulate_transaction(request)
        .await?
        .into_inner();

    let executed_tx = response.transaction();
    let effects = executed_tx.effects();
    if !effects.status().success() {
        return Err(Error::TransactionDryRunError(Box::new(
            effects.status().error().clone(),
        )));
    }

    let resolved_tx = executed_tx.transaction();
    let gas_payment = resolved_tx.gas_payment();

    let mut batch_request =
        BatchGetObjectsRequest::default().with_read_mask(FieldMask::from_paths([
            "object_id",
            "version",
            "digest",
            "balance",
        ]));

    for obj_ref in gas_payment.objects() {
        let get_request = GetObjectRequest::default()
            .with_object_id(obj_ref.object_id().to_string())
            .with_version(obj_ref.version());
        batch_request.requests.push(get_request);
    }

    let batch_response = client
        .ledger_client()
        .batch_get_objects(batch_request)
        .await?
        .into_inner();

    let mut gas_coins = Vec::new();
    for result in batch_response.objects {
        match result.result {
            Some(get_object_result::Result::Object(obj)) => {
                gas_coins.push(obj);
            }
            Some(get_object_result::Result::Error(err)) => {
                return Err(Error::DataError(format!(
                    "Failed to fetch gas coin object: {:?}",
                    err
                )));
            }
            None => {
                return Err(Error::DataError(
                    "Failed to fetch gas coin object: no result returned".to_string(),
                ));
            }
            Some(_) => {
                return Err(Error::DataError(
                    "Failed to fetch gas coin object: unexpected result type".to_string(),
                ));
            }
        }
    }

    Ok((gas_payment.budget(), gas_coins))
}
