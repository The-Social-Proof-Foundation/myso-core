// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use tracing::info;
use myso_types::effects::TransactionEffectsAPI;
use myso_types::execution_status::ExecutionStatus;
use myso_types::transaction::{TransactionDataAPI, TransactionKind};
use serde::Serialize;
use std::sync::Arc;

use myso_indexer_alt_framework::{
    FieldCount,
    pipeline::{
        Processor,
        concurrent::{BatchStatus, Handler},
    },
    store::Store,
    types::full_checkpoint_content::Checkpoint,
};

use crate::store::ClickHouseStore;

fn tx_kind_str(kind: &TransactionKind) -> &'static str {
    match kind {
        TransactionKind::ProgrammableTransaction(_) => "ProgrammableTransaction",
        TransactionKind::ChangeEpoch(_) => "ChangeEpoch",
        TransactionKind::Genesis(_) => "Genesis",
        TransactionKind::ConsensusCommitPrologue(_) => "ConsensusCommitPrologue",
        TransactionKind::AuthenticatorStateUpdate(_) => "AuthenticatorStateUpdate",
        TransactionKind::EndOfEpochTransaction(_) => "EndOfEpochTransaction",
        TransactionKind::RandomnessStateUpdate(_) => "RandomnessStateUpdate",
        TransactionKind::ConsensusCommitPrologueV2(_) => "ConsensusCommitPrologueV2",
        TransactionKind::ConsensusCommitPrologueV3(_) => "ConsensusCommitPrologueV3",
        TransactionKind::ConsensusCommitPrologueV4(_) => "ConsensusCommitPrologueV4",
        TransactionKind::ProgrammableSystemTransaction(_) => "ProgrammableSystemTransaction",
    }
}

/// Structure representing a transaction record in ClickHouse for the transactions table.
#[derive(Serialize, Clone, Debug, FieldCount)]
pub struct StoredTransaction {
    pub checkpoint_sequence_number: u64,
    pub transaction_digest: String,
    pub sender: String,
    pub timestamp_ms: i64,
    pub tx_kind: String,
    pub gas_computation_cost: u64,
    pub gas_storage_cost: u64,
    pub gas_storage_rebate: u64,
    pub status: u8,
    pub epoch: u64,
    pub gas_price: u64,
    pub gas_budget: u64,
    pub gas_owner: String,
    pub is_sponsored: u8,
    pub created_objects: u32,
    pub mutated_objects: u32,
    pub execution_error: Option<String>,
}

/// Handler that processes checkpoint data and writes to the transactions table
#[derive(Clone, Default)]
pub struct TxDigests;

#[async_trait::async_trait]
impl Processor for TxDigests {
    const NAME: &'static str = "tx_digests";
    type Value = StoredTransaction;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let Checkpoint {
            transactions,
            summary,
            ..
        } = checkpoint.as_ref();

        let checkpoint_sequence_number = *summary.sequence_number();
        let timestamp_ms = summary.timestamp_ms as i64;
        let epoch = summary.epoch;

        Ok(transactions
            .iter()
            .map(|tx| {
                let gas = tx.effects.gas_cost_summary();
                let (status, execution_error) = match tx.effects.status() {
                    ExecutionStatus::Success => (0u8, None),
                    ExecutionStatus::Failure { error, .. } => (1u8, Some(error.to_string())),
                };
                StoredTransaction {
                    checkpoint_sequence_number,
                    transaction_digest: tx.transaction.digest().to_string(),
                    sender: tx.transaction.sender().to_string(),
                    timestamp_ms,
                    tx_kind: tx_kind_str(tx.transaction.kind()).to_string(),
                    gas_computation_cost: gas.computation_cost,
                    gas_storage_cost: gas.storage_cost,
                    gas_storage_rebate: gas.storage_rebate,
                    status,
                    epoch,
                    gas_price: tx.transaction.gas_price(),
                    gas_budget: tx.transaction.gas_budget(),
                    gas_owner: tx.transaction.gas_owner().to_string(),
                    is_sponsored: tx.transaction.is_sponsored_tx() as u8,
                    created_objects: tx.effects.created().len() as u32,
                    mutated_objects: tx.effects.mutated().len() as u32,
                    execution_error,
                }
            })
            .collect())
    }
}

#[async_trait::async_trait]
impl Handler for TxDigests {
    type Store = ClickHouseStore;
    type Batch = Vec<Self::Value>;

    /// Smaller batches for faster inserts; avoids ClickHouse merge pressure.
    const MIN_EAGER_ROWS: usize = 100;
    const MAX_PENDING_ROWS: usize = 2_000;

    fn batch(
        &self,
        batch: &mut Self::Batch,
        values: &mut std::vec::IntoIter<Self::Value>,
    ) -> BatchStatus {
        batch.extend(values);
        if batch.len() >= Self::MIN_EAGER_ROWS {
            BatchStatus::Ready
        } else {
            BatchStatus::Pending
        }
    }

    async fn commit<'a>(
        &self,
        values: &Self::Batch,
        conn: &mut <Self::Store as Store>::Connection<'a>,
    ) -> Result<usize> {
        let row_count = values.len();
        if row_count == 0 {
            return Ok(0);
        }

        info!(row_count, "Committing transactions to ClickHouse");

        let block = crate::block_conv::stored_transactions_to_block(values)?;
        conn.bridge.insert("transactions", block).await?;

        Ok(row_count)
    }
}
