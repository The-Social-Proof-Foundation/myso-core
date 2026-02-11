// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Auto-bridge execution for custodial deposits
//! Handles deposits to our addresses and automatically calls bridge contracts
//! Supports both EVM→MySo and MySo→EVM flows.

use crate::abi::{EthBridgeConfig, EthERC20, EthMySoBridge};
use crate::deposit_addresses::DepositAddressManager;
use crate::deposit_gas_manager::DepositGasManager;
use crate::deposit_monitor::{EvmDepositEvent, MySoDepositEvent};
use crate::error::{BridgeError, BridgeResult};
use crate::myso_client::MySoBridgeClient;
use crate::storage::{BridgeOrchestratorTables, DepositAddressKey, DepositTxKey};
use crate::utils::EthProvider;
use alloy::network::EthereumWallet;
use alloy::network::TransactionBuilder;
use alloy::primitives::{Address as EthAddress, TxHash, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use futures::StreamExt;
use move_core_types::ident_str;
use move_core_types::language_storage::StructTag;
use myso_types::base_types::ObjectRef;
use myso_types::bridge::BRIDGE_MODULE_NAME;
use myso_types::coin::Coin;
use myso_types::digests::TransactionDigest;
use myso_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use myso_types::transaction::{ObjectArg, Transaction, TransactionData};
use myso_types::{parse_myso_type_tag, BRIDGE_PACKAGE_ID, TypeTag};
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use alloy_signer::Signer;
use tracing::{info, warn};

/// Max token ID to check when looking up token address (plan fix: extend from 20)
const MAX_TOKEN_ID_LOOKUP: u8 = 100;

/// Handles automatic bridging for EVM deposits (EVM → MySocial) and MySo deposits (MySo → EVM)
pub struct DepositBridgeHandler {
    storage: Arc<BridgeOrchestratorTables>,
    address_manager: Arc<DepositAddressManager>,
    gas_manager: Arc<DepositGasManager>,
    eth_provider: EthProvider,
    eth_bridge_address: EthAddress,
    eth_bridge_config_address: EthAddress,
    eth_chain_id: u64,
    /// Bridge chain ID (u8) for DepositTxKey - avoids truncation of full EVM chain ID
    eth_bridge_chain_id: u8,
    token_address_to_id: Arc<RwLock<HashMap<EthAddress, u8>>>,
    /// MySo client for MySo→EVM bridging (send_token)
    myso_client: Arc<MySoBridgeClient>,
    /// MySo bridge chain ID for DepositTxKey (MySo chain)
    myso_bridge_chain_id: u8,
}

impl DepositBridgeHandler {
    pub fn new(
        storage: Arc<BridgeOrchestratorTables>,
        address_manager: Arc<DepositAddressManager>,
        gas_manager: Arc<DepositGasManager>,
        eth_provider: EthProvider,
        eth_bridge_address: EthAddress,
        eth_bridge_config_address: EthAddress,
        eth_chain_id: u64,
        eth_bridge_chain_id: u8,
        myso_client: Arc<MySoBridgeClient>,
        myso_bridge_chain_id: u8,
    ) -> Self {
        Self {
            storage,
            address_manager,
            gas_manager,
            eth_provider,
            eth_bridge_address,
            eth_bridge_config_address,
            eth_chain_id,
            eth_bridge_chain_id,
            token_address_to_id: Arc::new(RwLock::new(HashMap::new())),
            myso_client,
            myso_bridge_chain_id,
        }
    }

    /// Handle an EVM deposit event (tokens sent to EVM deposit address)
    pub async fn handle_evm_deposit(&self, event: EvmDepositEvent) -> BridgeResult<TxHash> {
        let deposit_key = DepositTxKey::from_evm(
            event.tx_hash,
            event.log_index as u16,
            self.eth_bridge_chain_id,
        );

        if self.storage.is_deposit_processed(&deposit_key)? {
            info!(
                tx_hash = ?event.tx_hash,
                log_index = event.log_index,
                "Deposit already processed, skipping"
            );
            return Ok(TxHash::ZERO);
        }

        let deposit_address_key = DepositAddressKey::from_evm(event.to_address);
        let recipient_info = self
            .storage
            .get_recipient_for_deposit(&deposit_address_key)?
            .ok_or_else(|| {
                BridgeError::Generic(format!(
                    "No recipient found for deposit address {:?}",
                    event.to_address
                ))
            })?;

        info!(
            deposit_address = ?event.to_address,
            destination_len = recipient_info.destination_address.len(),
            amount = ?event.amount,
            "Processing EVM deposit"
        );

        let deposit_signer = self
            .address_manager
            .get_evm_signer_for_index(recipient_info.hd_index)?
            .with_chain_id(Some(self.eth_chain_id));

        let token_id = self.get_token_id_for_address(event.token_address).await?;

        let token_contract = EthERC20::new(event.token_address, self.eth_provider.clone());
        let balance_at_block = token_contract
            .balanceOf(event.to_address)
            .block(event.block_number.into())
            .call()
            .await
            .map_err(|e| {
                BridgeError::Generic(format!(
                    "Failed to query balance at block {}: {:?}",
                    event.block_number, e
                ))
            })?;

        let amount_to_bridge = if balance_at_block < event.amount {
            warn!(
                deposit_address = ?event.to_address,
                block_number = event.block_number,
                event_amount = ?event.amount,
                actual_balance_at_block = ?balance_at_block,
                "Fee-on-transfer token: bridging actual received amount"
            );
            balance_at_block
        } else {
            event.amount
        };

        info!(
            deposit_address = ?event.to_address,
            amount_to_bridge = ?amount_to_bridge,
            "Determined amount to bridge"
        );

        if recipient_info.destination_address.len() != 32 {
            return Err(BridgeError::Generic(
                "Destination address must be 32 bytes for MySocial".to_string(),
            ));
        }

        let destination_bytes: Vec<u8> = recipient_info.destination_address.clone();
        let destination_chain_id = recipient_info.destination_chain;

        let gas_price_raw = self
            .eth_provider
            .get_gas_price()
            .await
            .map_err(|e| BridgeError::Generic(format!("Failed to get gas price: {:?}", e)))?;
        let gas_price = U256::from(gas_price_raw);

        let token_contract_read = EthERC20::new(event.token_address, self.eth_provider.clone());
        let approve_call = token_contract_read.approve(self.eth_bridge_address, amount_to_bridge);
        let approval_gas_estimate = approve_call
            .estimate_gas()
            .await
            .map_err(|e| {
                BridgeError::Generic(format!("Failed to estimate approval gas: {:?}", e))
            })?;
        let approval_gas_limit = (approval_gas_estimate * 120 / 100).min(u64::MAX.into()) as u64;

        const CONSERVATIVE_BRIDGE_GAS_LIMIT: u64 = 250_000;

        self.gas_manager
            .ensure_evm_deposit_has_gas_with_estimates(
                event.to_address,
                approval_gas_limit,
                CONSERVATIVE_BRIDGE_GAS_LIMIT,
                gas_price,
            )
            .await?;

        let wallet = EthereumWallet::from(deposit_signer.clone());
        let signer_provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect_provider(self.eth_provider.clone());

        let token_contract_signer = EthERC20::new(event.token_address, signer_provider.clone());
        let approve_req: TransactionRequest = token_contract_signer
            .approve(self.eth_bridge_address, amount_to_bridge)
            .into_transaction_request()
            .with_chain_id(self.eth_chain_id)
            .with_gas_limit(approval_gas_limit)
            .with_gas_price(gas_price.to::<u128>());

        let pending_approval = signer_provider
            .send_transaction(approve_req)
            .await
            .map_err(|e| {
                BridgeError::Generic(format!("Failed to send approval: {:?}", e))
            })?;

        let approval_tx_hash = *pending_approval.tx_hash();
        info!(?approval_tx_hash, "Approval transaction sent");

        let approval_receipt = pending_approval
            .get_receipt()
            .await
            .map_err(|e| {
                BridgeError::Generic(format!("Failed to get approval receipt: {:?}", e))
            })?;

        if !approval_receipt.status() {
            return Err(BridgeError::Generic(
                "Token approval transaction reverted".to_string(),
            ));
        }

        let bridge_contract = EthMySoBridge::new(
            self.eth_bridge_address,
            signer_provider.clone(),
        );

        let bridge_call = bridge_contract.bridgeERC20(
            token_id,
            amount_to_bridge,
            destination_bytes.clone().into(),
            destination_chain_id,
        );

        let bridge_gas_estimate = bridge_call
            .estimate_gas()
            .await
            .map_err(|e| {
                BridgeError::Generic(format!("Failed to estimate bridge gas: {:?}", e))
            })?;
        let bridge_gas_limit = ((bridge_gas_estimate * 120 / 100) as u128)
            .min(u64::MAX as u128) as u64;

        self.gas_manager
            .ensure_evm_deposit_has_gas_with_estimates(
                event.to_address,
                0,
                bridge_gas_limit,
                gas_price,
            )
            .await?;

        let bridge_req: TransactionRequest = bridge_call
            .into_transaction_request()
            .with_chain_id(self.eth_chain_id)
            .with_gas_limit(bridge_gas_limit)
            .with_gas_price(gas_price.to::<u128>());

        let pending_bridge = signer_provider
            .send_transaction(bridge_req)
            .await
            .map_err(|e| {
                BridgeError::Generic(format!("Failed to send bridge transaction: {:?}", e))
            })?;

        let tx_hash = *pending_bridge.tx_hash();
        info!(?tx_hash, "Bridge transaction sent");

        let receipt = pending_bridge
            .get_receipt()
            .await
            .map_err(|e| {
                BridgeError::Generic(format!("Failed to get bridge receipt: {:?}", e))
            })?;

        if !receipt.status() {
            return Err(BridgeError::Generic(
                "Bridge transaction reverted".to_string(),
            ));
        }

        self.storage.mark_deposit_processed(
            deposit_key,
            format!("{:?}", tx_hash),
            amount_to_bridge.to_string(),
        )?;

        info!(?tx_hash, "EVM deposit bridged successfully");

        Ok(tx_hash)
    }

    /// Handle a MySo deposit event (tokens sent to MySo deposit address → bridge to EVM)
    pub async fn handle_myso_deposit(
        &self,
        event: MySoDepositEvent,
    ) -> BridgeResult<TransactionDigest> {
        let deposit_key = DepositTxKey::from_myso(
            event.tx_digest,
            self.myso_bridge_chain_id,
            event.balance_change_index,
        );

        if self.storage.is_deposit_processed(&deposit_key)? {
            info!(
                tx_digest = ?event.tx_digest,
                "MySo deposit already processed, skipping"
            );
            return Ok(TransactionDigest::ZERO);
        }

        let deposit_address_key = DepositAddressKey::from_myso(event.recipient);
        let recipient_info = self
            .storage
            .get_recipient_for_deposit(&deposit_address_key)?
            .ok_or_else(|| {
                BridgeError::Generic(format!(
                    "No recipient found for MySo deposit address {:?}",
                    event.recipient
                ))
            })?;

        if recipient_info.destination_address.len() != 20 {
            return Err(BridgeError::Generic(
                "Destination address must be 20 bytes for EVM".to_string(),
            ));
        }

        let target_chain = recipient_info.destination_chain;
        let target_address: [u8; 20] = recipient_info.destination_address[..20]
            .try_into()
            .map_err(|_| BridgeError::Generic("Invalid destination address length".to_string()))?;
        let hd_index = recipient_info.hd_index;

        let coin_type_tag = parse_myso_type_tag(&event.coin_type).map_err(|e| {
            BridgeError::Generic(format!("Invalid coin type '{}': {:?}", event.coin_type, e))
        })?;

        let (inner_type_tag, coin_object_type) = match &coin_type_tag {
            TypeTag::Struct(s) if Coin::is_coin(s) => {
                let inner = Coin::is_coin_with_coin_type(s).ok_or_else(|| {
                    BridgeError::Generic("Coin type has no inner type".to_string())
                })?;
                (
                    TypeTag::Struct(Box::new(inner.clone())),
                    StructTag::from(s.as_ref().clone()),
                )
            }
            _ => (
                coin_type_tag.clone(),
                Coin::type_(coin_type_tag.clone()),
            ),
        };

        let coin_obj_ref = self
            .pick_coin_with_balance(
                event.recipient,
                coin_object_type,
                event.amount,
            )
            .await?;

        self.gas_manager
            .ensure_myso_deposit_has_gas(event.recipient)
            .await?;

        let keypair = self.address_manager.get_myso_keypair_for_index(hd_index)?;
        let sender = myso_types::base_types::MySoAddress::from(&keypair.public());

        let bridge_object_arg = self
            .myso_client
            .get_mutable_bridge_object_arg_must_succeed()
            .await;

        let rgp = self
            .myso_client
            .get_reference_gas_price_until_success()
            .await;

        const GAS_BUDGET: u64 = 1_000_000_000;
        let gas_obj_ref = self
            .pick_gas_coin_for_address(sender, GAS_BUDGET)
            .await?;

        let mut builder = ProgrammableTransactionBuilder::new();
        let arg_target_chain = builder.pure(target_chain).map_err(|e| {
            BridgeError::Generic(format!("Failed to build target_chain arg: {:?}", e))
        })?;
        let arg_target_address = builder.pure(target_address.as_slice()).map_err(|e| {
            BridgeError::Generic(format!("Failed to build target_address arg: {:?}", e))
        })?;
        let arg_token = builder
            .obj(ObjectArg::ImmOrOwnedObject(coin_obj_ref))
            .map_err(|e| BridgeError::Generic(format!("Failed to build token arg: {:?}", e)))?;
        let arg_bridge = builder
            .obj(bridge_object_arg)
            .map_err(|e| BridgeError::Generic(format!("Failed to build bridge arg: {:?}", e)))?;

        builder.programmable_move_call(
            BRIDGE_PACKAGE_ID,
            BRIDGE_MODULE_NAME.to_owned(),
            ident_str!("send_token").to_owned(),
            vec![inner_type_tag],
            vec![arg_bridge, arg_target_chain, arg_target_address, arg_token],
        );

        let pt = builder.finish();
        let tx_data =
            TransactionData::new_programmable(sender, vec![gas_obj_ref], pt, 500_000_000, rgp);
        let sig = myso_types::crypto::Signature::new_secure(
            &IntentMessage::new(Intent::myso_transaction(), tx_data.clone()),
            &keypair,
        );
        let signed_tx = Transaction::from_data(tx_data, vec![sig]);
        let tx_digest = *signed_tx.digest();

        info!(?tx_digest, "Sending MySo→EVM bridge transaction");

        let resp = self
            .myso_client
            .execute_transaction_block_with_effects(signed_tx)
            .await?;

        match &resp.status {
            myso_json_rpc_types::MySoExecutionStatus::Success => {
                self.storage.mark_deposit_processed(
                    deposit_key,
                    tx_digest.to_string(),
                    event.amount.to_string(),
                )?;
                info!(?tx_digest, "MySo→EVM deposit bridged successfully");
                Ok(tx_digest)
            }
            myso_json_rpc_types::MySoExecutionStatus::Failure { error } => {
                Err(BridgeError::Generic(format!(
                    "Bridge transaction failed: {}",
                    error
                )))
            }
        }
    }

    /// Find a gas coin for the address with balance >= amount
    async fn pick_gas_coin_for_address(
        &self,
        owner: myso_types::base_types::MySoAddress,
        minimal_amount: u64,
    ) -> BridgeResult<ObjectRef> {
        let gas_coin_type = myso_types::gas_coin::GasCoin::type_();
        let mut stream = self
            .myso_client
            .grpc_client()
            .list_owned_objects(owner, Some(gas_coin_type))
            .boxed();

        let mut coins_checked = 0u32;
        while let Some(Ok(object)) = stream.next().await {
            coins_checked += 1;
            if let Ok(Some((_, balance))) = Coin::extract_balance_if_coin(&object) {
                if balance >= minimal_amount {
                    return Ok(object.compute_object_reference());
                }
            }
            if coins_checked >= 100 {
                break;
            }
        }

        Err(BridgeError::Generic(format!(
            "No gas coin with balance >= {} found for address {:?} (checked {} coins). \
            Ensure ensure_myso_deposit_has_gas is implemented or fund the deposit address manually.",
            minimal_amount, owner, coins_checked
        )))
    }

    /// Find a coin owned by the address with balance >= amount
    async fn pick_coin_with_balance(
        &self,
        owner: myso_types::base_types::MySoAddress,
        coin_object_type: StructTag,
        amount: u64,
    ) -> BridgeResult<ObjectRef> {
        let mut stream = self
            .myso_client
            .grpc_client()
            .list_owned_objects(owner, Some(coin_object_type))
            .boxed();

        let mut coins_checked = 0u32;
        while let Some(Ok(object)) = stream.next().await {
            coins_checked += 1;
            if let Ok(Some((_, balance))) = Coin::extract_balance_if_coin(&object) {
                if balance >= amount {
                    return Ok(object.compute_object_reference());
                }
            }
            if coins_checked >= 1000 {
                break;
            }
        }

        Err(BridgeError::Generic(format!(
            "No coin with balance >= {} found for address {:?} (checked {} coins)",
            amount, owner, coins_checked
        )))
    }

    async fn get_token_id_for_address(&self, token_address: EthAddress) -> BridgeResult<u8> {
        {
            let cache = self.token_address_to_id.read().await;
            if let Some(&token_id) = cache.get(&token_address) {
                return Ok(token_id);
            }
        }

        let config = EthBridgeConfig::new(
            self.eth_bridge_config_address,
            self.eth_provider.clone(),
        );

        for token_id in 0u8..=MAX_TOKEN_ID_LOOKUP {
            match config.tokenAddressOf(token_id).call().await {
                Ok(addr) if addr == token_address => {
                    let mut cache = self.token_address_to_id.write().await;
                    cache.insert(token_address, token_id);
                    return Ok(token_id);
                }
                Ok(_) => continue,
                Err(e) => {
                    warn!(token_id, ?e, "Error querying token address");
                    continue;
                }
            }
        }

        Err(BridgeError::Generic(format!(
            "Token address {:?} not found in bridge configuration",
            token_address
        )))
    }
}
