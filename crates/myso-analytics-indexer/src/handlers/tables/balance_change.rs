// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_types::balance_change::address_balance_changes_from_accumulator_events;
use myso_types::base_types::EpochId;
use myso_types::coin::Coin;
use myso_types::effects::TransactionEffectsAPI;
use myso_types::full_checkpoint_content::Checkpoint;
use myso_types::full_checkpoint_content::ExecutedTransaction;
use myso_types::gas_coin::GAS;
use myso_types::object::Owner;

use crate::Row;
use crate::pipeline::Pipeline;
use crate::tables::BalanceChangeRow;

impl Row for BalanceChangeRow {
    fn get_epoch(&self) -> EpochId {
        self.epoch
    }

    fn get_checkpoint(&self) -> u64 {
        self.checkpoint
    }
}

fn balance_changes(
    transaction: &ExecutedTransaction,
    checkpoint: &Checkpoint,
) -> Result<Vec<(String, String, i128)>> {
    if transaction.effects.status().is_err() {
        let net_gas_usage = transaction.effects.gas_cost_summary().net_gas_usage();
        return Ok(
            (net_gas_usage > 0)
                .then(|| {
                    let owner = transaction.effects.gas_object().1;
                    owner
                        .get_owner_address()
                        .ok()
                        .map(|addr| {
                            (
                                addr.to_string(),
                                GAS::type_tag().to_canonical_string(true),
                                -(net_gas_usage as i128),
                            )
                        })
                })
                .into_iter()
                .flatten()
                .collect(),
        );
    }

    let mut changes: BTreeMap<(Owner, move_core_types::language_storage::TypeTag), i128> =
        BTreeMap::new();

    for (addr, type_, balance) in
        address_balance_changes_from_accumulator_events(&transaction.effects)
    {
        *changes
            .entry((Owner::AddressOwner(addr), type_))
            .or_insert(0i128) += balance;
    }

    for object in transaction.input_objects(&checkpoint.object_set) {
        if let Some((type_, balance)) = Coin::extract_balance_if_coin(object)? {
            *changes
                .entry((object.owner().clone(), type_))
                .or_insert(0i128) -= balance as i128;
        }
    }

    for object in transaction.output_objects(&checkpoint.object_set) {
        if let Some((type_, balance)) = Coin::extract_balance_if_coin(object)? {
            *changes
                .entry((object.owner().clone(), type_))
                .or_insert(0i128) += balance as i128;
        }
    }

    Ok(changes
        .into_iter()
        .filter_map(|((owner, coin_type), amount)| {
            if amount == 0 {
                return None;
            }
            let owner_addr = owner.get_owner_address().ok()?;
            Some((
                owner_addr.to_string(),
                coin_type.to_canonical_string(true),
                amount,
            ))
        })
        .collect())
}

pub struct BalanceChangeProcessor;

#[async_trait]
impl Processor for BalanceChangeProcessor {
    const NAME: &'static str = Pipeline::BalanceChange.name();
    const FANOUT: usize = 16;
    type Value = BalanceChangeRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let epoch = checkpoint.summary.data().epoch;
        let checkpoint_seq = checkpoint.summary.data().sequence_number;
        let timestamp_ms = checkpoint.summary.data().timestamp_ms;

        let mut entries = Vec::new();

        for executed_tx in &checkpoint.transactions {
            let digest = executed_tx.effects.transaction_digest();

            let changes = balance_changes(executed_tx, checkpoint).with_context(|| {
                format!("Calculating balance changes for transaction {}", digest)
            })?;

            for (owner, coin_type, amount) in changes {
                let amount_i64 = amount.clamp(i64::MIN as i128, i64::MAX as i128) as i64;
                entries.push(BalanceChangeRow {
                    checkpoint: checkpoint_seq,
                    transaction_digest: digest.to_string(),
                    epoch,
                    timestamp_ms,
                    owner,
                    coin_type,
                    amount: amount_i64,
                });
            }
        }

        Ok(entries)
    }
}
