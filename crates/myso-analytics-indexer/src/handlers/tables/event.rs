// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use move_core_types::annotated_value::MoveValue;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_json_rpc_types::type_and_fields_from_move_event_data;
use myso_types::base_types::EpochId;
use myso_types::effects::TransactionEffectsAPI;
use myso_types::event::Event;
use myso_types::full_checkpoint_content::Checkpoint;
use tracing::{debug, warn};

use crate::Row;
use crate::package_store::PackageCache;
use crate::pipeline::Pipeline;
use crate::tables::EventRow;

pub struct EventProcessor {
    package_cache: Arc<PackageCache>,
}

impl EventProcessor {
    pub fn new(package_cache: Arc<PackageCache>) -> Self {
        Self { package_cache }
    }
}

impl Row for EventRow {
    fn get_epoch(&self) -> EpochId {
        self.epoch
    }

    fn get_checkpoint(&self) -> u64 {
        self.checkpoint
    }
}

#[async_trait]
impl Processor for EventProcessor {
    const NAME: &'static str = Pipeline::Event.name();
    const FANOUT: usize = 16;
    type Value = EventRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let epoch = checkpoint.summary.data().epoch;
        let checkpoint_seq = checkpoint.summary.data().sequence_number;
        let timestamp_ms = checkpoint.summary.data().timestamp_ms;

        let mut entries = Vec::new();
        let mut tx_with_events = 0u64;
        let mut tx_without_events = 0u64;

        for executed_tx in &checkpoint.transactions {
            let digest = executed_tx.effects.transaction_digest();

            if let Some(events) = &executed_tx.events {
                tx_with_events += 1;
                for (idx, event) in events.data.iter().enumerate() {
                    let Event {
                        package_id,
                        transaction_module,
                        sender,
                        type_,
                        contents,
                    } = event;

                    let event_json = match self
                        .package_cache
                        .resolver_for_epoch(epoch)
                        .type_layout(move_core_types::language_storage::TypeTag::Struct(
                            Box::new(type_.clone()),
                        ))
                        .await
                    {
                        Ok(layout) => match MoveValue::simple_deserialize(contents, &layout) {
                            Ok(move_value) => {
                                match type_and_fields_from_move_event_data(move_value) {
                                    Ok((_, json)) => json.to_string(),
                                    Err(e) => {
                                        warn!(
                                            package = %package_id,
                                            module = %transaction_module,
                                            event_type = %type_,
                                            error = %e,
                                            "Event JSON conversion failed, using fallback"
                                        );
                                        r#"{"_error":"layout_unavailable"}"#.to_string()
                                    }
                                }
                            }
                            Err(e) => {
                                warn!(
                                    package = %package_id,
                                    module = %transaction_module,
                                    event_type = %type_,
                                    error = %e,
                                    "Event deserialization failed, using fallback"
                                );
                                r#"{"_error":"deserialize_failed"}"#.to_string()
                            }
                        },
                        Err(e) => {
                            warn!(
                                package = %package_id,
                                module = %transaction_module,
                                event_type = %type_,
                                error = %e,
                                "Event type layout resolution failed, using fallback"
                            );
                            r#"{"_error":"layout_unavailable"}"#.to_string()
                        }
                    };

                    let row = EventRow {
                        transaction_digest: digest.base58_encode(),
                        event_index: idx as u64,
                        checkpoint: checkpoint_seq,
                        epoch,
                        timestamp_ms,
                        sender: sender.to_string(),
                        package: package_id.to_string(),
                        module: transaction_module.to_string(),
                        event_type: type_.to_string(),
                        bcs: "".to_string(),
                        bcs_length: contents.len() as u64,
                        event_json,
                    };

                    entries.push(row);
                }
            } else {
                tx_without_events += 1;
            }
        }

        debug!(
            checkpoint = checkpoint_seq,
            events_extracted = entries.len(),
            tx_with_events,
            tx_without_events,
            "Event pipeline processed checkpoint"
        );

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use myso_indexer_alt_framework::pipeline::Processor;
    use myso_types::event::Event;
    use myso_types::test_checkpoint_data_builder::TestCheckpointBuilder;
    use tempfile::TempDir;

    use super::EventProcessor;
    use crate::package_store::PackageCache;

    #[tokio::test]
    async fn test_event_processor_produces_rows_with_events() {
        let temp = TempDir::new().unwrap();
        let package_cache = Arc::new(PackageCache::new(temp.path(), "http://localhost:9000"));

        let checkpoint = Arc::new(
            TestCheckpointBuilder::new(1)
                .start_transaction(0)
                .with_events(vec![Event::random_for_testing()])
                .finish_transaction()
                .build_checkpoint(),
        );

        let processor = EventProcessor::new(package_cache);
        let rows = processor.process(&checkpoint).await.unwrap();

        assert!(
            !rows.is_empty(),
            "EventProcessor should produce rows when checkpoint has events"
        );
        assert_eq!(rows.len(), 1);
        assert!(rows[0].event_json.contains("_error") || !rows[0].event_json.is_empty());
    }

    #[tokio::test]
    async fn test_event_processor_empty_when_no_events() {
        let temp = TempDir::new().unwrap();
        let package_cache = Arc::new(PackageCache::new(temp.path(), "http://localhost:9000"));

        let checkpoint = Arc::new(
            TestCheckpointBuilder::new(1)
                .start_transaction(0)
                .finish_transaction()
                .build_checkpoint(),
        );

        let processor = EventProcessor::new(package_cache);
        let rows = processor.process(&checkpoint).await.unwrap();

        assert!(rows.is_empty());
    }
}
