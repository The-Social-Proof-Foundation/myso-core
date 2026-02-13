// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! The MySoSyncer module is responsible for synchronizing Events emitted
//! on MySo blockchain from concerned modules of bridge package 0x9.
//!
//! There are two modes of operation:
//! - Event-based (legacy): Uses JSON-RPC to query events by module
//! - gRPC-based (new): Iterates over bridge records using LinkedTable iteration
//!
//! As of now, only the event-based mode is being used.

use crate::{
    error::BridgeResult,
    events::{EmittedMySoToEthTokenBridgeV1, EmittedMySoToEthTokenBridgeV2, MySoBridgeEvent},
    metrics::BridgeMetrics,
    myso_client::{MySoClient, MySoClientInner},
    retry_with_max_elapsed_time,
    types::BridgeAction,
};
use myso_types::{Identifier, event::EventID};
use mysten_metrics::spawn_logged_monitored_task;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::Notify,
    task::JoinHandle,
    time::{self, Duration},
};

const MYSO_EVENTS_CHANNEL_SIZE: usize = 1000;

/// Map from contract address to their start cursor (exclusive)
pub type MySoTargetModules = HashMap<Identifier, Option<EventID>>;

pub type GrpcSyncedEvents = (u64, Vec<MySoBridgeEvent>);

pub struct MySoSyncer<C> {
    myso_client: Arc<MySoClient<C>>,
    // The last transaction that the syncer has fully processed.
    // Syncer will resume post this transaction (i.e. exclusive), when it starts.
    #[allow(unused)]
    cursors: MySoTargetModules,
    metrics: Arc<BridgeMetrics>,
}

impl<C> MySoSyncer<C>
where
    C: MySoClientInner + 'static,
{
    pub fn new(
        myso_client: Arc<MySoClient<C>>,
        cursors: MySoTargetModules,
        metrics: Arc<BridgeMetrics>,
    ) -> Self {
        Self {
            myso_client,
            cursors,
            metrics,
        }
    }

    pub async fn run_grpc(
        self,
        source_chain_id: u8,
        next_sequence_number: u64,
        query_interval: Duration,
        batch_size: u64,
    ) -> BridgeResult<(
        Vec<JoinHandle<()>>,
        mysten_metrics::metered_channel::Receiver<GrpcSyncedEvents>,
    )> {
        let (events_tx, events_rx) = mysten_metrics::metered_channel::channel(
            MYSO_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["myso_grpc_events_queue"]),
        );

        let task_handle = spawn_logged_monitored_task!(Self::run_grpc_listening_task(
            source_chain_id,
            next_sequence_number,
            events_tx,
            self.myso_client.clone(),
            query_interval,
            batch_size,
            self.metrics.clone(),
        ));

        Ok((vec![task_handle], events_rx))
    }

    async fn run_grpc_listening_task(
        source_chain_id: u8,
        mut next_sequence_cursor: u64,
        events_sender: mysten_metrics::metered_channel::Sender<GrpcSyncedEvents>,
        myso_client: Arc<MySoClient<C>>,
        query_interval: Duration,
        batch_size: u64,
        metrics: Arc<BridgeMetrics>,
    ) {
        tracing::info!(
            source_chain_id,
            next_sequence_cursor,
            "Starting myso grpc records listening task"
        );
        let mut interval = time::interval(query_interval);
        interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

        // Create a task to update metrics
        let notify = Arc::new(Notify::new());
        let notify_clone = notify.clone();
        let myso_client_clone = myso_client.clone();
        let chain_label = source_chain_id.to_string();
        let last_synced_myso_checkpoints_metric = metrics
            .last_synced_myso_checkpoints
            .with_label_values(&[&chain_label]);
        spawn_logged_monitored_task!(async move {
            loop {
                notify_clone.notified().await;
                let Ok(Ok(latest_checkpoint_sequence_number)) = retry_with_max_elapsed_time!(
                    myso_client_clone.get_latest_checkpoint_sequence_number(),
                    Duration::from_secs(120)
                ) else {
                    tracing::error!(
                        "Failed to query latest checkpoint sequence number from myso client after retry"
                    );
                    continue;
                };
                last_synced_myso_checkpoints_metric.set(latest_checkpoint_sequence_number as i64);
            }
        });

        loop {
            interval.tick().await;
            let Ok(Ok(on_chain_next_sequence_index)) = retry_with_max_elapsed_time!(
                myso_client.get_token_transfer_next_seq_number(source_chain_id),
                Duration::from_secs(120)
            ) else {
                tracing::error!(
                    source_chain_id,
                    "Failed to get next seq num from myso client after retry"
                );
                continue;
            };

            // start querying from the next_sequence_cursor till on_chain_next_sequence_index in batches
            let start_index = next_sequence_cursor;
            if start_index >= on_chain_next_sequence_index {
                notify.notify_one();
                continue;
            }

            let end_index = std::cmp::min(
                start_index + batch_size - 1,
                on_chain_next_sequence_index - 1,
            );

            let Ok(Ok(records)) = retry_with_max_elapsed_time!(
                myso_client.get_bridge_records_in_range(source_chain_id, start_index, end_index),
                Duration::from_secs(120)
            ) else {
                tracing::error!(
                    source_chain_id,
                    start_index,
                    end_index,
                    "Failed to get records from myso client after retry"
                );
                continue;
            };

            let len = records.len();
            if len != 0 {
                let mut events = Vec::with_capacity(len);
                let mut batch_last_sequence_index = start_index;

                for (seq_index, record) in records {
                    let event = match Self::bridge_record_to_event(&record, source_chain_id) {
                        Ok(event) => event,
                        Err(e) => {
                            tracing::error!(
                                source_chain_id,
                                seq_index,
                                "Failed to convert record to event: {:?}",
                                e
                            );
                            continue;
                        }
                    };

                    events.push(event);
                    batch_last_sequence_index = seq_index;
                }

                if !events.is_empty() {
                    events_sender
                        .send((batch_last_sequence_index + 1, events))
                        .await
                        .expect("Bridge events channel receiver is closed");

                    next_sequence_cursor = batch_last_sequence_index + 1;
                    tracing::info!(
                        source_chain_id,
                        last_processed_seq = batch_last_sequence_index,
                        next_sequence_cursor,
                        "Processed {len} bridge records"
                    );
                }
            }

            if end_index >= on_chain_next_sequence_index - 1 {
                // we have processed all records up to the latest checkpoint
                // so we can update the latest checkpoint metric
                notify.notify_one();
            }
        }
    }

    fn bridge_record_to_event(
        record: &myso_types::bridge::MoveTypeBridgeRecord,
        source_chain_id: u8,
    ) -> Result<MySoBridgeEvent, crate::error::BridgeError> {
        let action = BridgeAction::try_from_bridge_record(record)?;

        match action {
            BridgeAction::MySoToEthTokenTransfer(transfer) => Ok(
                MySoBridgeEvent::MySoToEthTokenBridgeV1(EmittedMySoToEthTokenBridgeV1 {
                    nonce: transfer.nonce,
                    myso_chain_id: transfer.myso_chain_id,
                    eth_chain_id: transfer.eth_chain_id,
                    myso_address: transfer.myso_address,
                    eth_address: transfer.eth_address,
                    token_id: transfer.token_id,
                    amount_myso_adjusted: transfer.amount_adjusted,
                }),
            ),
            BridgeAction::MySoToEthTokenTransferV2(transfer) => Ok(
                MySoBridgeEvent::MySoToEthTokenBridgeV2(EmittedMySoToEthTokenBridgeV2 {
                    nonce: transfer.nonce,
                    myso_chain_id: transfer.myso_chain_id,
                    eth_chain_id: transfer.eth_chain_id,
                    myso_address: transfer.myso_address,
                    eth_address: transfer.eth_address,
                    token_id: transfer.token_id,
                    amount_myso_adjusted: transfer.amount_adjusted,
                    timestamp_ms: transfer.timestamp_ms,
                }),
            ),
            _ => Err(crate::error::BridgeError::Generic(format!(
                "Unexpected action type for source_chain_id {}: {:?}",
                source_chain_id, action
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{myso_client::MySoClient, myso_mock_client::MySoMockClient};
    use myso_types::bridge::{BridgeChainId, MoveTypeBridgeMessage, MoveTypeBridgeRecord};
    use prometheus::Registry;
    use tokio::time::timeout;

    async fn assert_no_more_events<T: std::fmt::Debug>(
        interval: Duration,
        events_rx: &mut mysten_metrics::metered_channel::Receiver<T>,
    ) {
        match timeout(interval * 2, events_rx.recv()).await {
            Err(_e) => (),
            other => panic!("Should have timed out, but got: {:?}", other),
        };
    }

    /// Creates a test bridge record with valid BCS-encoded payload
    fn create_test_bridge_record(
        seq_num: u64,
        source_chain: BridgeChainId,
        target_chain: BridgeChainId,
        amount: u64,
    ) -> MoveTypeBridgeRecord {
        // Create the payload struct matching MySoToEthOnChainBcsPayload
        #[derive(serde::Serialize)]
        struct TestPayload {
            myso_address: Vec<u8>,
            target_chain: u8,
            eth_address: Vec<u8>,
            token_type: u8,
            amount: [u8; 8],
        }

        let payload = TestPayload {
            myso_address: vec![0u8; 32], // 32-byte MySoAddress
            target_chain: target_chain as u8,
            eth_address: vec![0u8; 20], // 20-byte EthAddress
            token_type: 1,              // MYSO token
            amount: amount.to_be_bytes(),
        };

        let payload_bytes = bcs::to_bytes(&payload).unwrap();

        MoveTypeBridgeRecord {
            message: MoveTypeBridgeMessage {
                message_type: 0, // TokenTransfer
                message_version: 1,
                seq_num,
                source_chain: source_chain as u8,
                payload: payload_bytes,
            },
            verified_signatures: None,
            claimed: false,
        }
    }

    #[tokio::test]
    async fn test_myso_syncer_grpc_basic() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);
        let metrics = Arc::new(BridgeMetrics::new(&registry));
        let mock = MySoMockClient::default();
        let client = Arc::new(MySoClient::new_for_testing(mock.clone()));

        let source_chain_id = BridgeChainId::MySoCustom as u8;
        let target_modules = HashMap::new(); // Not used for gRPC mode

        let interval = Duration::from_millis(200);
        let batch_size = 10;
        let next_sequence_number = 0;

        // Initially, no records on chain
        mock.set_next_seq_num(source_chain_id, 0);

        let (_handles, mut events_rx) =
            MySoSyncer::new(client.clone(), target_modules.clone(), metrics.clone())
                .run_grpc(source_chain_id, next_sequence_number, interval, batch_size)
                .await
                .unwrap();

        // Initially there are no records
        assert_no_more_events(interval, &mut events_rx).await;

        mock.set_latest_checkpoint_sequence_number(1000);

        // Add some bridge records
        let record_0 =
            create_test_bridge_record(0, BridgeChainId::MySoCustom, BridgeChainId::EthCustom, 1000);
        let record_1 =
            create_test_bridge_record(1, BridgeChainId::MySoCustom, BridgeChainId::EthCustom, 2000);

        mock.add_bridge_record(source_chain_id, 0, record_0);
        mock.add_bridge_record(source_chain_id, 1, record_1);
        mock.set_next_seq_num(source_chain_id, 2); // 2 records available (0 and 1)

        let (next_cursor, received_events) = events_rx.recv().await.unwrap();
        assert_eq!(received_events.len(), 2);
        assert_eq!(next_cursor, 2); // Next sequence number to process

        match &received_events[0] {
            MySoBridgeEvent::MySoToEthTokenBridgeV1(event) => {
                assert_eq!(event.nonce, 0);
                assert_eq!(event.myso_chain_id, BridgeChainId::MySoCustom);
                assert_eq!(event.eth_chain_id, BridgeChainId::EthCustom);
                assert_eq!(event.amount_myso_adjusted, 1000);
            }
            _ => panic!("Expected MySoToEthTokenBridgeV1 event"),
        }
        match &received_events[1] {
            MySoBridgeEvent::MySoToEthTokenBridgeV1(event) => {
                assert_eq!(event.nonce, 1);
                assert_eq!(event.amount_myso_adjusted, 2000);
            }
            _ => panic!("Expected MySoToEthTokenBridgeV1 event"),
        }

        // No more events should be received
        assert_no_more_events(interval, &mut events_rx).await;
        assert_eq!(
            metrics
                .last_synced_myso_checkpoints
                .get_metric_with_label_values(&[&source_chain_id.to_string()])
                .unwrap()
                .get(),
            1000
        );

        Ok(())
    }
}
