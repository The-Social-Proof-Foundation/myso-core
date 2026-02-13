// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::abi::EthBridgeConfig;
use crate::crypto::BridgeAuthorityKeyPair;
use crate::error::BridgeError;
use crate::eth_client::EthClient;
use crate::metered_eth_provider::new_metered_eth_multi_provider;
use crate::metrics::BridgeMetrics;
use crate::myso_client::MySoBridgeClient;
use crate::types::{BridgeAction, is_route_valid};
use crate::utils::get_eth_contract_addresses;
use alloy::primitives::Address as EthAddress;
use alloy::providers::Provider;
use anyhow::anyhow;
use futures::StreamExt;
use myso_config::Config;
use myso_keys::keypair_file::read_key;
use myso_types::base_types::ObjectRef;
use myso_types::base_types::{MySoAddress, ObjectID};
use myso_types::bridge::BridgeChainId;
use myso_types::crypto::KeypairTraits;
use myso_types::crypto::{MySoKeyPair, NetworkKeyPair, get_key_pair_from_rng};
use myso_types::digests::{get_mainnet_chain_identifier, get_testnet_chain_identifier};
use myso_types::event::EventID;
use myso_types::gas_coin::GasCoin;
use myso_types::object::Owner;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EthConfig {
    /// Rpc url for Eth fullnode, used for query stuff.
    /// @deprecated (use eth_rpc_urls instead)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eth_rpc_url: Option<String>,
    /// Multiple RPC URLs for Eth fullnodes.
    /// Quorum-based consensus is used across providers for redundancy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eth_rpc_urls: Option<Vec<String>>,
    /// Quorum size for multi-provider consensus. Must be <= number of URLs.
    #[serde(default = "default_quorum")]
    pub eth_rpc_quorum: usize,
    /// Health check interval in seconds for multi-provider.
    #[serde(default = "default_health_check_interval_secs")]
    pub eth_health_check_interval_secs: u64,
    /// The proxy address of MySoBridge
    pub eth_bridge_proxy_address: String,
    /// The expected BridgeChainId on Eth side.
    pub eth_bridge_chain_id: u8,
    /// The starting block for EthSyncer to monitor eth contracts.
    /// It is required when `run_client` is true. Usually this is
    /// the block number when the bridge contracts are deployed.
    /// When BridgeNode starts, it reads the contract watermark from storage.
    /// If the watermark is not found, it will start from this fallback block number.
    /// If the watermark is found, it will start from the watermark.
    /// this v.s.`eth_contracts_start_block_override`:
    pub eth_contracts_start_block_fallback: Option<u64>,
    /// The starting block for EthSyncer to monitor eth contracts. It overrides
    /// the watermark in storage. This is useful when we want to reprocess the events
    /// from a specific block number.
    /// Note: this field has to be reset after starting the BridgeNode, otherwise it will
    /// reprocess the events from this block number every time it starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth_contracts_start_block_override: Option<u64>,
}

fn default_quorum() -> usize {
    1
}

fn default_health_check_interval_secs() -> u64 {
    300 // 5 minutes
}

impl EthConfig {
    /// Backwards compatible function to get list of RPC URLs
    pub fn rpc_urls(&self) -> Vec<String> {
        if let Some(ref urls) = self.eth_rpc_urls {
            urls.clone()
        } else if let Some(ref url) = self.eth_rpc_url {
            vec![url.clone()]
        } else {
            vec![]
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MySoConfig {
    /// Rpc url for MySo fullnode, used for query stuff and submit transactions.
    pub myso_rpc_url: String,
    /// The expected BridgeChainId on MySo side.
    pub myso_bridge_chain_id: u8,
    /// Path of the file where bridge client key (any MySoKeyPair) is stored.
    /// If `run_client` is true, and this is None, then use `bridge_authority_key_path` as client key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_key_path: Option<PathBuf>,
    /// The gas object to use for paying for gas fees for the client. It needs to
    /// be owned by the address associated with bridge client key. If not set
    /// and `run_client` is true, it will query and use the gas object with highest
    /// amount for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_gas_object: Option<ObjectID>,
    /// Override the last processed EventID for bridge module `bridge`.
    /// When set, MySoSyncer will start from this cursor (exclusively) instead of the one in storage.
    /// If the cursor is not found in storage or override, the query will start from genesis.
    /// Key: myso module, Value: last processed EventID (tx_digest, event_seq).
    /// Note 1: This field should be rarely used. Only use it when you understand how to follow up.
    /// Note 2: the EventID needs to be valid, namely it must exist and matches the filter.
    /// Otherwise, it will miss one event because of fullnode Event query semantics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myso_bridge_module_last_processed_event_id_override: Option<EventID>,
    /// Override the next sequence number for MySoSyncer
    /// When set, MySoSyncer will start from this sequence number (exclusively) instead of the one in storage.
    /// If the sequence number is not found in storage or override, the query will first fallback to the sequence number corresponding to the last processed EventID from the bridge module `bridge` (which in turn can be overridden via `myso_bridge_module_last_processed_event_id_override`) if available, otherwise fallback to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myso_bridge_next_sequence_number_override: Option<u64>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeNodeConfig {
    /// The port that the server listens on.
    pub server_listen_port: u16,
    /// The port that for metrics server.
    pub metrics_port: u16,
    /// Path of the file where bridge authority key (Secp256k1) is stored.
    pub bridge_authority_key_path: PathBuf,
    /// Whether to run client. If true, `myso.bridge_client_key_path`
    /// and `db_path` needs to be provided.
    pub run_client: bool,
    /// Path of the client storage. Required when `run_client` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_path: Option<PathBuf>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
    /// MySo configuration
    pub myso: MySoConfig,
    /// Eth configuration
    pub eth: EthConfig,
    /// Network key used for metrics pushing
    #[serde(default = "default_ed25519_key_pair")]
    pub metrics_key_pair: NetworkKeyPair,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<MetricsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchdog_config: Option<WatchdogConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay: Option<RelayConfigFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposits: Option<DepositConfigFile>,
}

pub fn default_ed25519_key_pair() -> NetworkKeyPair {
    get_key_pair_from_rng(&mut rand::rngs::OsRng).1
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetricsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_interval_seconds: Option<u64>,
    pub push_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WatchdogConfig {
    /// Total supplies to watch on MySo. Mapping from coin name to coin type tag
    pub total_supplies: BTreeMap<String, String>,
}

/// Auto-relay configuration loaded from config file
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelayConfigFile {
    pub enabled: bool,
    #[serde(default = "default_max_retries")]
    pub max_retries: u8,
    #[serde(default = "default_retry_delay")]
    pub retry_delay_seconds: u64,
    #[serde(default)]
    pub myso: MySoRelayConfigFile,
    pub evm: Option<EvmRelayConfigFile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MySoRelayConfigFile {
    #[serde(default = "default_myso_gas_budget")]
    pub gas_budget: u64,
}

impl Default for MySoRelayConfigFile {
    fn default() -> Self {
        Self {
            gas_budget: default_myso_gas_budget(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EvmRelayConfigFile {
    pub enabled: bool,
    pub rpc_url: Option<String>,
    pub bridge_contract_address: Option<String>,
    #[serde(default = "default_max_gas_price_gwei")]
    pub max_gas_price_gwei: u64,
    #[serde(default = "default_gas_buffer_percent")]
    pub gas_estimate_buffer_percent: u8,
    #[serde(default = "default_confirmation_blocks")]
    pub confirmation_blocks: u64,
}

fn default_max_retries() -> u8 {
    3
}

fn default_retry_delay() -> u64 {
    30
}

fn default_myso_gas_budget() -> u64 {
    100_000_000 // 0.1 MYSO
}

fn default_max_gas_price_gwei() -> u64 {
    10
}

fn default_gas_buffer_percent() -> u8 {
    20
}

fn default_confirmation_blocks() -> u64 {
    2
}

/// Deposit system configuration loaded from config file
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DepositConfigFile {
    pub enabled: bool,
    #[serde(default = "default_deposit_poll_interval")]
    pub poll_interval_secs: u64,
    #[serde(default = "default_auto_fund_gas")]
    pub auto_fund_gas: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_tokens: Option<Vec<String>>,
    /// Number of confirmations to wait for EVM deposits (reorg safety)
    #[serde(default = "default_evm_confirmation_blocks")]
    pub evm_confirmation_blocks: u64,
}

fn default_deposit_poll_interval() -> u64 {
    45
}

fn default_auto_fund_gas() -> bool {
    true
}

fn default_evm_confirmation_blocks() -> u64 {
    12 // Base recommended
}

/// Runtime deposit system configuration
#[derive(Debug, Clone)]
pub struct DepositConfig {
    pub enabled: bool,
    pub poll_interval_secs: u64,
    pub auto_fund_gas: bool,
    pub supported_tokens: Vec<EthAddress>,
    pub evm_confirmation_blocks: u64,
}

impl Config for BridgeNodeConfig {}

impl BridgeNodeConfig {
    pub async fn validate(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(BridgeServerConfig, Option<BridgeClientConfig>)> {
        info!("Starting config validation");
        if !is_route_valid(
            BridgeChainId::try_from(self.myso.myso_bridge_chain_id)?,
            BridgeChainId::try_from(self.eth.eth_bridge_chain_id)?,
        ) {
            return Err(anyhow!(
                "Route between MySo chain id {} and Eth chain id {} is not valid",
                self.myso.myso_bridge_chain_id,
                self.eth.eth_bridge_chain_id,
            ));
        };

        let bridge_authority_key = match read_key(&self.bridge_authority_key_path, true)? {
            MySoKeyPair::Secp256k1(key) => key,
            _ => unreachable!("we required secp256k1 key in `read_key`"),
        };

        // we do this check here instead of `prepare_for_myso` below because
        // that is only called when `run_client` is true.
        let myso_client =
            Arc::new(MySoBridgeClient::new(&self.myso.myso_rpc_url, metrics.clone()).await?);
        let bridge_committee = myso_client
            .get_bridge_committee()
            .await
            .map_err(|e| anyhow!("Error getting bridge committee: {:?}", e))?;
        if !bridge_committee.is_active_member(&bridge_authority_key.public().into()) {
            return Err(anyhow!(
                "Bridge authority key is not part of bridge committee"
            ));
        }

        let (eth_client, eth_contracts) = self.prepare_for_eth(metrics.clone()).await?;
        let bridge_summary = myso_client
            .get_bridge_summary()
            .await
            .map_err(|e| anyhow!("Error getting bridge summary: {:?}", e))?;
        if bridge_summary.chain_id != self.myso.myso_bridge_chain_id {
            anyhow::bail!(
                "Bridge chain id mismatch: expected {}, but connected to {}",
                self.myso.myso_bridge_chain_id,
                bridge_summary.chain_id
            );
        }

        // Validate approved actions that must be governance actions
        for action in &self.approved_governance_actions {
            if !action.is_governance_action() {
                anyhow::bail!(format!(
                    "{:?}",
                    BridgeError::ActionIsNotGovernanceAction(Box::new(action.clone()))
                ));
            }
        }
        let approved_governance_actions = self.approved_governance_actions.clone();

        let bridge_server_config = BridgeServerConfig {
            key: bridge_authority_key,
            metrics_port: self.metrics_port,
            eth_bridge_proxy_address: eth_contracts[0], // the first contract is bridge proxy
            server_listen_port: self.server_listen_port,
            myso_client: myso_client.clone(),
            eth_client: eth_client.clone(),
            approved_governance_actions,
        };
        if !self.run_client {
            return Ok((bridge_server_config, None));
        }

        // If client is enabled, prepare client config
        let (bridge_client_key, client_myso_address, gas_object_ref) =
            self.prepare_for_myso(myso_client.clone(), metrics).await?;

        let db_path = self
            .db_path
            .clone()
            .ok_or(anyhow!("`db_path` is required when `run_client` is true"))?;

        let relay_config = self.relay.as_ref().map(|relay_cfg| {
            if self.deposits.as_ref().map(|d| d.enabled).unwrap_or(false) && !relay_cfg.enabled {
                tracing::warn!(
                    "Deposits are enabled but relay is disabled. \
                     MySo → EVM deposits will not be automatically bridged."
                );
            }
            let evm_config = relay_cfg.evm.as_ref().map(|evm_cfg| {
                let rpc_url = evm_cfg
                    .rpc_url
                    .clone()
                    .unwrap_or_else(|| self.eth.rpc_urls().first().cloned().unwrap_or_default());
                let bridge_address = evm_cfg
                    .bridge_contract_address
                    .clone()
                    .unwrap_or_else(|| self.eth.eth_bridge_proxy_address.clone());
                crate::relay::EvmRelayConfig {
                    enabled: evm_cfg.enabled,
                    rpc_url,
                    bridge_contract_address: bridge_address
                        .parse()
                        .expect("Invalid bridge contract address in config"),
                    max_gas_price_gwei: evm_cfg.max_gas_price_gwei,
                    gas_estimate_buffer_percent: evm_cfg.gas_estimate_buffer_percent,
                    confirmation_blocks: evm_cfg.confirmation_blocks,
                }
            });
            crate::relay::RelayConfig {
                enabled: relay_cfg.enabled,
                max_retries: relay_cfg.max_retries,
                retry_delay_seconds: relay_cfg.retry_delay_seconds,
                myso_gas_budget: relay_cfg.myso.gas_budget,
                evm: evm_config,
            }
        });

        let deposit_config = self.deposits.as_ref().map(|deposit_cfg| {
            let supported_tokens = match &deposit_cfg.supported_tokens {
                Some(tokens) => tokens
                    .iter()
                    .filter_map(|addr_str| addr_str.parse::<EthAddress>().ok())
                    .collect(),
                None => Vec::new(),
            };
            let poll_interval_secs = std::env::var("DEPOSIT_POLL_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(deposit_cfg.poll_interval_secs);
            DepositConfig {
                enabled: deposit_cfg.enabled,
                poll_interval_secs,
                auto_fund_gas: deposit_cfg.auto_fund_gas,
                supported_tokens,
                evm_confirmation_blocks: deposit_cfg.evm_confirmation_blocks,
            }
        });

        let bridge_client_config = BridgeClientConfig {
            myso_address: client_myso_address,
            key: bridge_client_key,
            gas_object_ref,
            metrics_port: self.metrics_port,
            myso_client: myso_client.clone(),
            eth_client: eth_client.clone(),
            db_path,
            eth_contracts,
            // in `prepare_for_eth` we check if this is None when `run_client` is true. Safe to unwrap here.
            eth_contracts_start_block_fallback: self
                .eth
                .eth_contracts_start_block_fallback
                .unwrap(),
            eth_contracts_start_block_override: self.eth.eth_contracts_start_block_override,
            myso_bridge_module_last_processed_event_id_override: self
                .myso
                .myso_bridge_module_last_processed_event_id_override,
            myso_bridge_next_sequence_number_override: self
                .myso
                .myso_bridge_next_sequence_number_override,
            myso_bridge_chain_id: self.myso.myso_bridge_chain_id,
            relay_config,
            deposit_config,
            eth_bridge_chain_id: self.eth.eth_bridge_chain_id,
        };

        info!("Config validation complete");
        Ok((bridge_server_config, Some(bridge_client_config)))
    }

    async fn prepare_for_eth(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(Arc<EthClient>, Vec<EthAddress>)> {
        info!("Creating Ethereum client provider");
        let bridge_proxy_address = EthAddress::from_str(&self.eth.eth_bridge_proxy_address)?;
        let rpc_urls = self.eth.rpc_urls();
        anyhow::ensure!(
            !rpc_urls.is_empty(),
            "At least one Ethereum RPC URL must be provided"
        );

        let provider = new_metered_eth_multi_provider(
            rpc_urls.clone(),
            self.eth.eth_rpc_quorum,
            self.eth.eth_health_check_interval_secs,
            metrics.clone(),
        )
        .await?;

        let chain_id = provider.get_chain_id().await?;
        let (
            committee_address,
            limiter_address,
            vault_address,
            config_address,
            _weth_address,
            _usdt_address,
            _wbtc_address,
            _lbtc_address,
        ) = get_eth_contract_addresses(bridge_proxy_address, provider.clone()).await?;
        let config = EthBridgeConfig::new(config_address, provider.clone());

        if self.run_client && self.eth.eth_contracts_start_block_fallback.is_none() {
            return Err(anyhow!(
                "eth_contracts_start_block_fallback is required when run_client is true"
            ));
        }

        // If bridge chain id is Eth Mainent or Sepolia, we expect to see chain
        // identifier to match accordingly.
        let bridge_chain_id: u8 = config.chainID().call().await?;
        if self.eth.eth_bridge_chain_id != bridge_chain_id {
            return Err(anyhow!(
                "Bridge chain id mismatch: expected {}, but connected to {}",
                self.eth.eth_bridge_chain_id,
                bridge_chain_id
            ));
        }
        if bridge_chain_id == BridgeChainId::EthMainnet as u8 && chain_id != 1 {
            anyhow::bail!("Expected Eth chain id 1, but connected to {}", chain_id);
        }
        if bridge_chain_id == BridgeChainId::EthSepolia as u8 && chain_id != 11155111 {
            anyhow::bail!(
                "Expected Eth chain id 11155111, but connected to {}",
                chain_id
            );
        }
        info!(
            "Connected to Eth chain: {}, Bridge chain id: {}",
            chain_id, bridge_chain_id,
        );

        // Filter out zero addresses (can happen due to storage layout mismatch during upgrades)
        let all_addresses = vec![
            bridge_proxy_address,
            committee_address,
            config_address,
            limiter_address,
            vault_address,
        ];
        let valid_addresses: Vec<_> = all_addresses
            .into_iter()
            .filter(|addr| !addr.is_zero())
            .collect();

        if valid_addresses.len() < 5 {
            tracing::warn!(
                "Some contract addresses are zero - likely storage layout mismatch. \
                Event watching will be limited. Valid addresses: {:?}",
                valid_addresses
            );
        }

        let eth_client = Arc::new(
            EthClient::from_provider(provider, HashSet::from_iter(valid_addresses.clone())).await?,
        );
        info!("Ethereum client setup complete");
        Ok((eth_client, valid_addresses))
    }

    async fn prepare_for_myso(
        &self,
        myso_client: Arc<MySoBridgeClient>,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(MySoKeyPair, MySoAddress, ObjectRef)> {
        let bridge_client_key = match &self.myso.bridge_client_key_path {
            None => read_key(&self.bridge_authority_key_path, true),
            Some(path) => read_key(path, false),
        }?;

        // If bridge chain id is MySo Mainent or Testnet, we expect to see chain
        // identifier to match accordingly.
        let myso_identifier = myso_client
            .get_chain_identifier()
            .await
            .map_err(|e| anyhow!("Error getting chain identifier from MySo: {:?}", e))?;
        if self.myso.myso_bridge_chain_id == BridgeChainId::MySoMainnet as u8
            && myso_identifier != get_mainnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected myso chain identifier {}, but connected to {}",
                self.myso.myso_bridge_chain_id,
                myso_identifier
            );
        }
        if self.myso.myso_bridge_chain_id == BridgeChainId::MySoTestnet as u8
            && myso_identifier != get_testnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected myso chain identifier {}, but connected to {}",
                self.myso.myso_bridge_chain_id,
                myso_identifier
            );
        }
        info!(
            "Connected to MySo chain: {}, Bridge chain id: {}",
            myso_identifier, self.myso.myso_bridge_chain_id,
        );

        let client_myso_address = MySoAddress::from(&bridge_client_key.public());

        let gas_object_id = match self.myso.bridge_client_gas_object {
            Some(id) => id,
            None => {
                info!("No gas object configured, finding gas object with highest balance");
                let myso_client = myso_rpc_api::Client::new(&self.myso.myso_rpc_url)?;
                // Minimum balance for gas object is 10 MYSO
                pick_highest_balance_coin(myso_client, client_myso_address, 10_000_000_000).await?
            }
        };
        let (gas_coin, gas_object_ref, owner) = myso_client
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await;
        if owner != Owner::AddressOwner(client_myso_address) {
            return Err(anyhow!(
                "Gas object {:?} is not owned by bridge client key's associated myso address {:?}, but {:?}",
                gas_object_id,
                client_myso_address,
                owner
            ));
        }
        let balance = gas_coin.value();
        info!("Gas object balance: {}", balance);
        metrics.gas_coin_balance.set(balance as i64);

        info!("MySo client setup complete");
        Ok((bridge_client_key, client_myso_address, gas_object_ref))
    }
}

pub struct BridgeServerConfig {
    pub key: BridgeAuthorityKeyPair,
    pub server_listen_port: u16,
    pub eth_bridge_proxy_address: EthAddress,
    pub metrics_port: u16,
    pub myso_client: Arc<MySoBridgeClient>,
    pub eth_client: Arc<EthClient>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
}

pub struct BridgeClientConfig {
    pub myso_address: MySoAddress,
    pub key: MySoKeyPair,
    pub gas_object_ref: ObjectRef,
    pub metrics_port: u16,
    pub myso_client: Arc<MySoBridgeClient>,
    pub eth_client: Arc<EthClient>,
    pub db_path: PathBuf,
    pub eth_contracts: Vec<EthAddress>,
    // See `BridgeNodeConfig` for the explanation of following two fields.
    pub eth_contracts_start_block_fallback: u64,
    pub eth_contracts_start_block_override: Option<u64>,
    pub myso_bridge_module_last_processed_event_id_override: Option<EventID>,
    pub myso_bridge_next_sequence_number_override: Option<u64>,
    pub myso_bridge_chain_id: u8,
    pub relay_config: Option<crate::relay::RelayConfig>,
    pub deposit_config: Option<DepositConfig>,
    pub eth_bridge_chain_id: u8,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeCommitteeConfig {
    pub bridge_authority_port_and_key_path: Vec<(u64, PathBuf)>,
}

impl Config for BridgeCommitteeConfig {}

pub async fn pick_highest_balance_coin(
    client: myso_rpc_api::Client,
    address: MySoAddress,
    minimal_amount: u64,
) -> anyhow::Result<ObjectID> {
    info!("Looking for a mysotable gas coin for address {:?}", address);

    // Only look at MYSO coins specifically
    let mut stream = client
        .list_owned_objects(address, Some(GasCoin::type_()))
        .boxed();

    let mut coins_checked = 0;

    while let Some(Ok(object)) = stream.next().await {
        let Ok(coin) = GasCoin::try_from(&object) else {
            continue;
        };
        info!(
            "Checking coin: {:?}, balance: {}",
            object.id(),
            coin.value()
        );
        coins_checked += 1;

        // Take the first coin with a sufficient balance
        if coin.value() >= minimal_amount {
            info!(
                "Found mysotable gas coin with {} mist (object ID: {:?})",
                coin.value(),
                object.id(),
            );
            return Ok(object.id());
        }

        // Only check a small number of coins before giving up
        if coins_checked >= 1000 {
            break;
        }
    }

    Err(anyhow!(
        "No mysotable gas coin with >= {} mist found for address {:?} after checking {} coins",
        minimal_amount,
        address,
        coins_checked
    ))
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EthContractAddresses {
    pub myso_bridge: EthAddress,
    pub bridge_committee: EthAddress,
    pub bridge_config: EthAddress,
    pub bridge_limiter: EthAddress,
    pub bridge_vault: EthAddress,
}
