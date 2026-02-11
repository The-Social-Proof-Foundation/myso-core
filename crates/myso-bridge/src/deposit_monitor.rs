// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deposit monitoring for custodial deposit addresses
//! Detects when users send tokens to deposit addresses and triggers auto-bridging

use crate::error::{BridgeError, BridgeResult};
use crate::storage::BridgeOrchestratorTables;
use alloy::primitives::{Address as EthAddress, B256, U256};
use alloy::providers::Provider;
use alloy::rpc::types::{Filter, Log};
use fastcrypto::hash::{HashFunction, Keccak256};
use futures::StreamExt;
use myso_rpc::field::{FieldMask, FieldMaskUtil};
use myso_rpc::proto::myso::rpc::v2::Checkpoint;
use myso_types::base_types::MySoAddress;
use std::str::FromStr;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

/// Event representing a deposit to an EVM deposit address
#[derive(Debug, Clone)]
pub struct EvmDepositEvent {
    pub tx_hash: B256,
    pub log_index: u64,
    pub block_number: u64,
    pub token_address: EthAddress,
    pub from_address: EthAddress,
    pub to_address: EthAddress, // Our deposit address
    pub amount: U256,
}

/// Event representing a deposit to a MySo deposit address
#[derive(Debug, Clone)]
pub struct MySoDepositEvent {
    pub tx_digest: myso_types::digests::TransactionDigest,
    pub sender: MySoAddress,
    pub recipient: MySoAddress, // Our deposit address
    pub coin_type: String,
    pub amount: u64,
    pub timestamp: u64,
    /// Index of this balance change within the transaction (disambiguates multiple deposits per tx)
    pub balance_change_index: u16,
}

/// Transfer(address,address,uint256) event signature
fn transfer_event_signature() -> B256 {
    let hash = Keccak256::digest(b"Transfer(address,address,uint256)");
    B256::from_slice(hash.digest.as_ref())
}

/// Monitors EVM chains for deposits to our generated deposit addresses.
/// Uses confirmation_blocks for reorg safety and persists cursor to RocksDB.
pub struct EvmDepositMonitor {
    provider: crate::utils::EthProvider,
    storage: Arc<BridgeOrchestratorTables>,
    chain_id: u64,
    supported_tokens: Vec<EthAddress>,
    poll_interval: Duration,
    confirmation_blocks: u64,
    deposit_tx: tokio::sync::mpsc::UnboundedSender<EvmDepositEvent>,
}

impl EvmDepositMonitor {
    pub fn new(
        provider: crate::utils::EthProvider,
        storage: Arc<BridgeOrchestratorTables>,
        chain_id: u64,
        supported_tokens: Vec<EthAddress>,
        poll_interval_secs: u64,
        confirmation_blocks: u64,
        deposit_tx: tokio::sync::mpsc::UnboundedSender<EvmDepositEvent>,
    ) -> Self {
        Self {
            provider,
            storage,
            chain_id,
            supported_tokens,
            poll_interval: Duration::from_secs(poll_interval_secs),
            confirmation_blocks,
            deposit_tx,
        }
    }

    pub async fn run(self) -> BridgeResult<()> {
        info!(
            chain_id = self.chain_id,
            confirmation_blocks = self.confirmation_blocks,
            "Starting EVM deposit monitor"
        );

        let mut interval = tokio::time::interval(self.poll_interval);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            interval.tick().await;

            if let Err(e) = self.check_for_deposits().await {
                error!(?e, "Error checking for EVM deposits");
            }
        }
    }

    async fn check_for_deposits(&self) -> BridgeResult<()> {
        let current_block = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| BridgeError::Generic(format!("Failed to get block number: {:?}", e)))?;

        // Reorg safety: only process blocks that are confirmation_blocks old
        let safe_block = current_block.saturating_sub(self.confirmation_blocks);

        let start_block = self
            .storage
            .get_evm_deposit_monitor_cursor(self.chain_id)?
            .map(|last| last + 1)
            .unwrap_or_else(|| safe_block.saturating_sub(1000).max(0));

        if start_block > safe_block {
            return Ok(());
        }

        let end_block = safe_block;

        let deposit_addresses = self.storage.get_all_evm_deposit_addresses();

        if deposit_addresses.is_empty() {
            self.storage
                .update_evm_deposit_monitor_cursor(self.chain_id, end_block)?;
            return Ok(());
        }

        let deposit_addresses_set: HashSet<EthAddress> =
            deposit_addresses.iter().copied().collect();

        let transfer_sig = transfer_event_signature();

        for token_addr in &self.supported_tokens {
            let filter = Filter::new()
                .address(*token_addr)
                .from_block(start_block)
                .to_block(end_block)
                .event_signature(transfer_sig);

            let logs = self.provider.get_logs(&filter).await.map_err(|e| {
                BridgeError::Generic(format!("Failed to get logs: {:?}", e))
            })?;

            let relevant_logs: Vec<_> = logs
                .into_iter()
                .filter(|log| {
                    let topics = log.topics();
                    if topics.len() < 3 {
                        return false;
                    }
                    let to_address = EthAddress::from_slice(&topics[2].as_slice()[12..]);
                    deposit_addresses_set.contains(&to_address)
                })
                .collect();

            if !relevant_logs.is_empty() {
                info!(
                    token = ?token_addr,
                    log_count = relevant_logs.len(),
                    "Found {} deposit events",
                    relevant_logs.len()
                );

                for log in relevant_logs {
                    if let Err(e) = self.process_deposit_log(log).await {
                        error!(?e, "Failed to process deposit log");
                    }
                }
            }
        }

        self.storage
            .update_evm_deposit_monitor_cursor(self.chain_id, end_block)?;
        Ok(())
    }

    async fn process_deposit_log(&self, log: Log) -> BridgeResult<()> {
        let topics = log.topics();
        if topics.len() < 3 {
            return Err(BridgeError::Generic("Invalid Transfer event format".to_string()));
        }

        let from_address = EthAddress::from_slice(&topics[1].as_slice()[12..]);
        let to_address = EthAddress::from_slice(&topics[2].as_slice()[12..]);

        let amount = U256::from_be_slice(log.data().data.as_ref());

        let tx_hash = log
            .transaction_hash
            .ok_or_else(|| BridgeError::Generic("Log missing transaction hash".to_string()))?;

        let log_index = log
            .log_index
            .ok_or_else(|| BridgeError::Generic("Log missing log index".to_string()))?;

        let block_number = log
            .block_number
            .ok_or_else(|| BridgeError::Generic("Log missing block number".to_string()))?;

        let event = EvmDepositEvent {
            tx_hash,
            log_index,
            block_number,
            token_address: log.address(),
            from_address,
            to_address,
            amount,
        };

        info!(
            tx_hash = ?event.tx_hash,
            to = ?event.to_address,
            amount = ?event.amount,
            "Detected EVM deposit, sending for processing"
        );

        self.deposit_tx.send(event).map_err(|e| {
            BridgeError::Generic(format!("Failed to send deposit event: {:?}", e))
        })?;

        Ok(())
    }
}

/// Monitors MySo chain for deposits using gRPC SubscribeCheckpoints with balance_changes.
/// Push-based, no polling.
pub struct MySoDepositMonitor {
    grpc_client: myso_rpc::Client,
    storage: Arc<BridgeOrchestratorTables>,
    myso_chain_id: u8,
    deposit_tx: tokio::sync::mpsc::UnboundedSender<MySoDepositEvent>,
}

impl MySoDepositMonitor {
    pub fn new(
        grpc_client: myso_rpc::Client,
        storage: Arc<BridgeOrchestratorTables>,
        myso_chain_id: u8,
        deposit_tx: tokio::sync::mpsc::UnboundedSender<MySoDepositEvent>,
    ) -> Self {
        Self {
            grpc_client,
            storage,
            myso_chain_id,
            deposit_tx,
        }
    }

    pub async fn run(self) -> BridgeResult<()> {
        info!(
            myso_chain_id = self.myso_chain_id,
            "Starting MySo deposit monitor (gRPC SubscribeCheckpoints + balance_changes)"
        );

        let mut client = self.grpc_client;
        let storage = self.storage.clone();
        let myso_chain_id = self.myso_chain_id;
        let deposit_tx = self.deposit_tx.clone();

        let read_mask = FieldMask::from_paths([
            Checkpoint::path_builder().sequence_number(),
            Checkpoint::path_builder().transactions().digest(),
            Checkpoint::path_builder().transactions().balance_changes().finish(),
            "transactions.transaction.sender".to_string(),
            "transactions.timestamp".to_string(),
        ]);

        loop {
            let mut subscription = match client
                .subscription_client()
                .subscribe_checkpoints(
                    myso_rpc::proto::myso::rpc::v2::SubscribeCheckpointsRequest::default()
                        .with_read_mask(read_mask.clone()),
                )
                .await
            {
                Ok(s) => s.into_inner(),
                Err(e) => {
                    tracing::warn!("MySo deposit monitor: failed to subscribe: {e}");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            while let Some(item) = subscription.next().await {
                let response = match item {
                    Ok(r) => r,
                    Err(e) => {
                        tracing::warn!("MySo deposit monitor: stream error: {e}");
                        break;
                    }
                };

                let checkpoint = match response.checkpoint {
                    Some(c) => c,
                    None => continue,
                };

                let deposit_addresses = storage.get_all_myso_deposit_addresses();
                if deposit_addresses.is_empty() {
                    continue;
                }

                let deposit_set: HashSet<MySoAddress> =
                    deposit_addresses.iter().copied().collect();

                for txn in checkpoint.transactions() {
                    let tx_digest = txn.digest();

                    let balance_changes = txn.balance_changes();
                    for (balance_change_index, change) in balance_changes.iter().enumerate() {
                        let recipient = match change
                            .address()
                            .parse::<MySoAddress>()
                        {
                            Ok(addr) => addr,
                            Err(_) => continue,
                        };
                        if !deposit_set.contains(&recipient) {
                            continue;
                        }

                        let amount_str = if change.amount().is_empty() { "0" } else { change.amount() };
                        let amount_i128: i128 = match amount_str.parse() {
                            Ok(a) => a,
                            Err(_) => continue,
                        };
                        if amount_i128 <= 0 {
                            continue;
                        }

                        let amount_u64: u64 = match amount_i128.try_into() {
                            Ok(a) => a,
                            Err(_) => continue,
                        };

                        let sender = match txn.transaction_opt() {
                            Some(t) => t.sender(),
                            None => continue,
                        };

                        let tx_digest_parsed = match std::str::FromStr::from_str(tx_digest) {
                            Ok(d) => d,
                            Err(_) => continue,
                        };

                        let balance_change_index_u16 = balance_change_index
                            .try_into()
                            .unwrap_or(u16::MAX);
                        let deposit_key = crate::storage::DepositTxKey::from_myso(
                            tx_digest_parsed,
                            myso_chain_id,
                            balance_change_index_u16,
                        );
                        if storage.is_deposit_processed(&deposit_key)? {
                            continue;
                        }

                        let coin_type = change.coin_type().to_string();

                        let event = MySoDepositEvent {
                            tx_digest: tx_digest_parsed,
                            sender: MySoAddress::from_str(sender).map_err(|e| {
                                BridgeError::Generic(format!("Invalid sender: {:?}", e))
                            })?,
                            recipient,
                            coin_type,
                            amount: amount_u64,
                            timestamp: txn
                                .timestamp_opt()
                                .map(|t| {
                                    let s = t.seconds as u64;
                                    let ns = t.nanos as u64;
                                    s * 1000 + ns / 1_000_000
                                })
                                .unwrap_or(0),
                            balance_change_index: balance_change_index_u16,
                        };

                        info!(
                            tx_digest = ?event.tx_digest,
                            recipient = ?event.recipient,
                            amount = event.amount,
                            coin_type = event.coin_type,
                            balance_change_index = event.balance_change_index,
                            "Detected MySo deposit, sending for processing"
                        );

                        deposit_tx.send(event).map_err(|e| {
                            BridgeError::Generic(format!("Failed to send: {:?}", e))
                        })?;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_deposit_event_creation() {
        let event = EvmDepositEvent {
            tx_hash: B256::ZERO,
            log_index: 0,
            block_number: 1000,
            token_address: EthAddress::ZERO,
            from_address: EthAddress::ZERO,
            to_address: EthAddress::ZERO,
            amount: U256::from(1000),
        };

        assert_eq!(event.log_index, 0);
        assert_eq!(event.block_number, 1000);
    }
}
