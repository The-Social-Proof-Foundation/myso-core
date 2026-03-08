// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_types::base_types::EpochId;
use myso_types::effects::TransactionEffectsAPI;
use myso_types::full_checkpoint_content::Checkpoint;
use myso_types::transaction::TransactionDataAPI;
use tracing::debug;

use crate::Row;
use crate::pipeline::Pipeline;
use crate::tables::MoveCallRow;

pub struct MoveCallProcessor;

impl Row for MoveCallRow {
    fn get_epoch(&self) -> EpochId {
        self.epoch
    }

    fn get_checkpoint(&self) -> u64 {
        self.checkpoint
    }
}

#[async_trait]
impl Processor for MoveCallProcessor {
    const NAME: &'static str = Pipeline::MoveCall.name();
    const FANOUT: usize = 16;
    type Value = MoveCallRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let epoch = checkpoint.summary.data().epoch;
        let checkpoint_seq = checkpoint.summary.data().sequence_number;
        let timestamp_ms = checkpoint.summary.data().timestamp_ms;

        let mut entries = Vec::new();
        let mut tx_with_move_calls = 0u64;
        let mut tx_without_move_calls = 0u64;

        for executed_tx in &checkpoint.transactions {
            let move_calls = executed_tx.transaction.move_calls();
            let transaction_digest = executed_tx.effects.transaction_digest().base58_encode();

            if move_calls.is_empty() {
                tx_without_move_calls += 1;
            } else {
                tx_with_move_calls += 1;
            }

            for (cmd_idx, package, module, function) in move_calls.iter() {
                let row = MoveCallRow {
                    transaction_digest: transaction_digest.clone(),
                    cmd_idx: *cmd_idx as u64,
                    checkpoint: checkpoint_seq,
                    epoch,
                    timestamp_ms,
                    package: package.to_string(),
                    module: module.to_string(),
                    function: function.to_string(),
                };
                entries.push(row);
            }
        }

        debug!(
            checkpoint = checkpoint_seq,
            move_calls_extracted = entries.len(),
            tx_with_move_calls,
            tx_without_move_calls,
            "MoveCall pipeline processed checkpoint"
        );

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use myso_indexer_alt_framework::pipeline::Processor;
    use myso_types::base_types::ObjectID;
    use myso_types::test_checkpoint_data_builder::TestCheckpointBuilder;

    use super::MoveCallProcessor;

    #[tokio::test]
    async fn test_move_call_processor_produces_rows_with_move_calls() {
        let checkpoint = Arc::new(
            TestCheckpointBuilder::new(1)
                .start_transaction(0)
                .add_move_call(ObjectID::ZERO, "test_module", "test_function")
                .finish_transaction()
                .build_checkpoint(),
        );

        let processor = MoveCallProcessor;
        let rows = processor.process(&checkpoint).await.unwrap();

        assert!(
            !rows.is_empty(),
            "MoveCallProcessor should produce rows when transaction has move calls"
        );
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].module, "test_module");
        assert_eq!(rows[0].function, "test_function");
    }

    #[tokio::test]
    async fn test_move_call_processor_empty_when_no_move_calls() {
        let checkpoint = Arc::new(
            TestCheckpointBuilder::new(1)
                .start_transaction(0)
                .finish_transaction()
                .build_checkpoint(),
        );

        let processor = MoveCallProcessor;
        let rows = processor.process(&checkpoint).await.unwrap();

        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn test_move_call_processor_multiple_move_calls() {
        let checkpoint = Arc::new(
            TestCheckpointBuilder::new(1)
                .start_transaction(0)
                .add_move_call(ObjectID::ZERO, "mod1", "func1")
                .add_move_call(ObjectID::ZERO, "mod2", "func2")
                .finish_transaction()
                .build_checkpoint(),
        );

        let processor = MoveCallProcessor;
        let rows = processor.process(&checkpoint).await.unwrap();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].module, "mod1");
        assert_eq!(rows[0].function, "func1");
        assert_eq!(rows[1].module, "mod2");
        assert_eq!(rows[1].function, "func2");
    }
}
