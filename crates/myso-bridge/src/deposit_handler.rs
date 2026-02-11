// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deposit event handler - processes detected deposits and triggers bridging

use crate::deposit_bridge::DepositBridgeHandler;
use crate::deposit_monitor::{EvmDepositEvent, MySoDepositEvent};
use std::sync::Arc;
use tracing::{error, info};

/// Runs the EVM deposit processing loop.
/// Receives deposit events from EvmDepositMonitor and triggers bridge execution.
pub async fn run_evm_deposit_processor(
    mut evm_deposit_rx: tokio::sync::mpsc::UnboundedReceiver<EvmDepositEvent>,
    bridge_handler: Arc<DepositBridgeHandler>,
) {
    info!("Starting EVM deposit processor");

    while let Some(deposit_event) = evm_deposit_rx.recv().await {
        info!(
            tx_hash = ?deposit_event.tx_hash,
            to_address = ?deposit_event.to_address,
            amount = ?deposit_event.amount,
            "Processing EVM deposit"
        );

        match bridge_handler.handle_evm_deposit(deposit_event.clone()).await {
            Ok(bridge_tx_hash) => {
                if bridge_tx_hash == alloy::primitives::TxHash::ZERO {
                    // Already processed, skipped
                    continue;
                }
                info!(
                    deposit_tx = ?deposit_event.tx_hash,
                    bridge_tx = ?bridge_tx_hash,
                    "Successfully bridged EVM deposit"
                );
            }
            Err(e) => {
                error!(
                    deposit_tx = ?deposit_event.tx_hash,
                    error = ?e,
                    "Failed to bridge EVM deposit"
                );
            }
        }
    }

    info!("EVM deposit processor shutting down");
}

/// Runs the MySocial deposit processing loop.
/// Receives deposit events from MySoDepositMonitor and triggers bridge execution (MySo→EVM).
pub async fn run_myso_deposit_processor(
    mut myso_deposit_rx: tokio::sync::mpsc::UnboundedReceiver<MySoDepositEvent>,
    bridge_handler: Arc<DepositBridgeHandler>,
) {
    info!("Starting MySocial deposit processor");

    while let Some(deposit_event) = myso_deposit_rx.recv().await {
        info!(
            tx_digest = ?deposit_event.tx_digest,
            recipient = ?deposit_event.recipient,
            amount = deposit_event.amount,
            coin_type = deposit_event.coin_type,
            "Processing MySocial deposit"
        );

        match bridge_handler.handle_myso_deposit(deposit_event.clone()).await {
            Ok(bridge_tx_digest) => {
                if bridge_tx_digest == myso_types::digests::TransactionDigest::ZERO {
                    continue;
                }
                info!(
                    deposit_tx = ?deposit_event.tx_digest,
                    bridge_tx = ?bridge_tx_digest,
                    "Successfully bridged MySocial deposit to EVM"
                );
            }
            Err(e) => {
                error!(
                    deposit_tx = ?deposit_event.tx_digest,
                    error = ?e,
                    "Failed to bridge MySocial deposit"
                );
            }
        }
    }

    info!("MySocial deposit processor shutting down");
}
