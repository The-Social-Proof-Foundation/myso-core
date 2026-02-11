// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Gas management for custodial deposit addresses
//! Ensures deposit addresses have sufficient gas to execute bridge transactions

use crate::error::{BridgeError, BridgeResult};
use crate::myso_client::MySoBridgeClient;
use crate::utils::EthProvider;
use alloy::network::TransactionBuilder;
use alloy::network::EthereumWallet;
use alloy::primitives::{Address as EthAddress, TxHash, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy_signer::Signer;
use myso_types::base_types::MySoAddress;
use myso_types::crypto::MySoKeyPair;
use std::sync::Arc;
use tracing::{error, info, warn};

// Gas thresholds for MySocial (in MIST)
// Will be used when MySocial gas funding is fully implemented
#[allow(dead_code)]
const MIN_MYS_GAS_BALANCE: u64 = 10_000_000; // 0.01 MYS
#[allow(dead_code)]
const MYS_GAS_FUND_AMOUNT: u64 = 20_000_000; // 0.02 MYS

/// Manages gas funding for deposit addresses
pub struct DepositGasManager {
    /// Relayer's main EVM wallet (for funding deposit addresses)
    relayer_eth_signer: Option<PrivateKeySigner>,
    /// Relayer's main MySocial keypair (for funding deposit addresses)
    /// Will be used when MySocial gas funding is implemented
    #[allow(dead_code)]
    relayer_mys_keypair: MySoKeyPair,
    /// MySocial client (for querying balances and funding)
    /// Will be used when MySocial gas funding is implemented
    #[allow(dead_code)]
    myso_client: Arc<MySoBridgeClient>,
    /// EVM provider (read-only; signer provider built on-the-fly for sends)
    eth_provider: Option<EthProvider>,
    /// Chain ID for EVM
    eth_chain_id: Option<u64>,
}

impl DepositGasManager {
    pub fn new(
        relayer_mys_keypair: MySoKeyPair,
        myso_client: Arc<MySoBridgeClient>,
        relayer_eth_signer: Option<PrivateKeySigner>,
        eth_provider: Option<EthProvider>,
        eth_chain_id: Option<u64>,
    ) -> Self {
        Self {
            relayer_eth_signer,
            relayer_mys_keypair,
            myso_client,
            eth_provider,
            eth_chain_id,
        }
    }

    /// Ensure an EVM deposit address has sufficient gas using actual gas estimates
    ///
    /// This is the production method that uses real gas estimates from the transactions.
    /// It calculates the required balance as: (approval_gas + bridge_gas) × gas_price × 1.2
    pub async fn ensure_evm_deposit_has_gas_with_estimates(
        &self,
        deposit_address: EthAddress,
        approval_gas_limit: u64,
        bridge_gas_limit: u64,
        gas_price: U256,
    ) -> BridgeResult<()> {
        let provider = self.eth_provider.as_ref().ok_or_else(|| {
            BridgeError::Generic("EVM provider not configured for gas management".to_string())
        })?;

        let _relayer_signer = self.relayer_eth_signer.as_ref().ok_or_else(|| {
            BridgeError::Generic("Relayer EVM wallet not configured".to_string())
        })?;

        let balance = provider
            .get_balance(deposit_address)
            .await
            .map_err(|e| {
                BridgeError::Generic(format!(
                    "Failed to check balance for {:?}: {:?}",
                    deposit_address, e
                ))
            })?;

        let total_gas_needed = approval_gas_limit + bridge_gas_limit;
        let required_balance = gas_price
            .checked_mul(U256::from(total_gas_needed))
            .ok_or_else(|| BridgeError::Generic("Gas price calculation overflow".to_string()))?;

        info!(
            ?deposit_address,
            balance_wei = ?balance,
            balance_eth = balance.to::<u128>() as f64 / 1e18,
            approval_gas_limit,
            bridge_gas_limit,
            total_gas_needed,
            gas_price_wei = ?gas_price,
            gas_price_gwei = gas_price.to::<u64>() as f64 / 1e9,
            required_balance_wei = ?required_balance,
            required_balance_eth = required_balance.to::<u128>() as f64 / 1e18,
            "Checking EVM deposit address balance with actual gas estimates"
        );

        if balance < required_balance {
            let funding_amount = required_balance
                .checked_sub(balance)
                .ok_or_else(|| BridgeError::Generic("Funding calculation underflow".to_string()))?;

            let funding_amount_with_buffer = funding_amount
                .checked_mul(U256::from(120))
                .and_then(|v| v.checked_div(U256::from(100)))
                .ok_or_else(|| BridgeError::Generic("Funding buffer calculation overflow".to_string()))?;

            info!(
                ?deposit_address,
                current_balance_eth = balance.to::<u128>() as f64 / 1e18,
                funding_amount_eth = funding_amount_with_buffer.to::<u128>() as f64 / 1e18,
                "Funding EVM deposit address with gas (using actual estimates)"
            );

            self.fund_evm_address(deposit_address, funding_amount_with_buffer)
                .await?;

            info!(?deposit_address, "Successfully funded EVM deposit address");
        } else {
            info!(
                ?deposit_address,
                balance_eth = balance.to::<u128>() as f64 / 1e18,
                required_balance_eth = required_balance.to::<u128>() as f64 / 1e18,
                "EVM deposit address has sufficient gas"
            );
        }

        Ok(())
    }

    /// Send ETH from relayer's main wallet to a deposit address
    async fn fund_evm_address(
        &self,
        to_address: EthAddress,
        amount: U256,
    ) -> BridgeResult<TxHash> {
        let provider = self.eth_provider.as_ref().ok_or_else(|| {
            BridgeError::Generic("EVM provider not configured".to_string())
        })?;

        let signer = self
            .relayer_eth_signer
            .as_ref()
            .ok_or_else(|| BridgeError::Generic("Relayer wallet not configured".to_string()))?
            .clone()
            .with_chain_id(Some(
                self.eth_chain_id
                    .ok_or_else(|| BridgeError::Generic("Chain ID not configured".to_string()))?,
            ));

        let relayer_address = signer.address();

        let relayer_balance = provider
            .get_balance(relayer_address)
            .await
            .map_err(|e| {
                BridgeError::Generic(format!("Failed to check relayer balance: {:?}", e))
            })?;

        if relayer_balance < amount {
            return Err(BridgeError::Generic(format!(
                "Relayer has insufficient balance. Has: {}, Needs: {}",
                relayer_balance, amount
            )));
        }

        let wallet = EthereumWallet::from(signer);
        let signer_provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect_provider(provider.clone());

        let tx = TransactionRequest::default()
            .with_to(to_address)
            .with_value(amount);

        let pending_tx = signer_provider.send_transaction(tx).await.map_err(|e| {
            BridgeError::Generic(format!("Failed to send funding transaction: {:?}", e))
        })?;

        let tx_hash = *pending_tx.tx_hash();

        let receipt = pending_tx.get_receipt().await.map_err(|e| {
            BridgeError::Generic(format!("Failed to confirm funding transaction: {:?}", e))
        })?;

        if !receipt.status() {
            return Err(BridgeError::Generic(
                "Funding transaction reverted".to_string(),
            ));
        }

        info!(
            ?tx_hash,
            ?to_address,
            amount_eth = amount.to::<u128>() as f64 / 1e18,
            "EVM funding transaction confirmed"
        );

        Ok(tx_hash)
    }

    /// Ensure a MySocial deposit address has sufficient gas
    ///
    /// TODO: Complete implementation for production
    /// Primary use case is EVM→MySocial deposits, which don't need MySocial gas funding.
    pub async fn ensure_myso_deposit_has_gas(
        &self,
        deposit_address: MySoAddress,
    ) -> BridgeResult<()> {
        info!(
            ?deposit_address,
            min_balance_mist = MIN_MYS_GAS_BALANCE,
            "Checking MySocial deposit address gas balance"
        );

        warn!(
            ?deposit_address,
            "MySocial gas funding requires manual setup - automatic funding not yet implemented"
        );

        Ok(())
    }

    /// Check if relayer's main EVM wallet has sufficient balance
    pub async fn check_relayer_evm_balance(&self) -> BridgeResult<U256> {
        let provider = self.eth_provider.as_ref().ok_or_else(|| {
            BridgeError::Generic("EVM provider not configured".to_string())
        })?;

        let signer = self.relayer_eth_signer.as_ref().ok_or_else(|| {
            BridgeError::Generic("Relayer wallet not configured".to_string())
        })?;

        let balance = provider
            .get_balance(signer.address())
            .await
            .map_err(|e| BridgeError::Generic(format!("Failed to get relayer balance: {:?}", e)))?;

        let balance_eth = balance.to::<u128>() as f64 / 1e18;

        const WARN_THRESHOLD: f64 = 1.0; // 1 ETH
        const CRITICAL_THRESHOLD: f64 = 0.1; // 0.1 ETH

        if balance_eth < CRITICAL_THRESHOLD {
            error!(
                address = ?signer.address(),
                balance_eth,
                "CRITICAL: Relayer EVM wallet balance critically low!"
            );
        } else if balance_eth < WARN_THRESHOLD {
            warn!(
                address = ?signer.address(),
                balance_eth,
                "WARNING: Relayer EVM wallet balance getting low"
            );
        }

        Ok(balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_constants() {
        assert_eq!(MIN_MYS_GAS_BALANCE, 10_000_000u64);
        assert_eq!(MYS_GAS_FUND_AMOUNT, 20_000_000u64);
        assert!(MYS_GAS_FUND_AMOUNT > MIN_MYS_GAS_BALANCE);
    }
}
