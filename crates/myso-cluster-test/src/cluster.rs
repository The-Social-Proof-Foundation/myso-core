// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::config::{ClusterTestOpt, Env};
use async_trait::async_trait;
use myso_config::Config;
use myso_config::{MYSO_KEYSTORE_FILENAME, MYSO_NETWORK_CONFIG, PersistedConfig};
use myso_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use myso_sdk::myso_client_config::{MySoClientConfig, MySoEnv};
use myso_sdk::wallet_context::WalletContext;
use myso_swarm::memory::Swarm;
use myso_swarm_config::genesis_config::GenesisConfig;
use myso_swarm_config::network_config::NetworkConfig;
use myso_types::base_types::MySoAddress;
use myso_types::crypto::KeypairTraits;
use myso_types::crypto::MySoKeyPair;
use myso_types::crypto::{AccountKeyPair, get_key_pair};
use std::path::Path;
use tempfile::tempdir;
use test_cluster::{TestCluster, TestClusterBuilder};
use tracing::info;

const DEVNET_FAUCET_ADDR: &str = "https://faucet.devnet.mysocial.network:443";
const STAGING_FAUCET_ADDR: &str = "https://faucet.staging.mysocial.network:443";
const CONTINUOUS_FAUCET_ADDR: &str = "https://faucet.ci.mysocial.network:443";
const CONTINUOUS_NOMAD_FAUCET_ADDR: &str = "https://faucet.nomad.ci.mysocial.network:443";
const TESTNET_FAUCET_ADDR: &str = "https://faucet.testnet.mysocial.network:443";
const DEVNET_FULLNODE_ADDR: &str = "https://rpc.devnet.mysocial.network:443";
const STAGING_FULLNODE_ADDR: &str = "https://fullnode.staging.mysocial.network:443";
const CONTINUOUS_FULLNODE_ADDR: &str = "https://fullnode.ci.mysocial.network:443";
const CONTINUOUS_NOMAD_FULLNODE_ADDR: &str = "https://fullnode.nomad.ci.mysocial.network:443";
const TESTNET_FULLNODE_ADDR: &str = "https://fullnode.testnet.mysocial.network:443";

pub struct ClusterFactory;

impl ClusterFactory {
    pub async fn start(
        options: &ClusterTestOpt,
    ) -> Result<Box<dyn Cluster + Sync + Send>, anyhow::Error> {
        Ok(match &options.env {
            Env::NewLocal => Box::new(LocalNewCluster::start(options).await?),
            _ => Box::new(RemoteRunningCluster::start(options).await?),
        })
    }
}

/// Cluster Abstraction
#[async_trait]
pub trait Cluster {
    async fn start(options: &ClusterTestOpt) -> Result<Self, anyhow::Error>
    where
        Self: Sized;

    fn fullnode_url(&self) -> &str;
    fn user_key(&self) -> AccountKeyPair;

    /// Returns faucet url in a remote cluster.
    fn remote_faucet_url(&self) -> Option<&str>;

    /// Returns faucet key in a local cluster.
    fn local_faucet_key(&self) -> Option<&AccountKeyPair>;

    /// Place to put config for the wallet, and any locally running services.
    fn config_directory(&self) -> &Path;
}

/// Represents an up and running cluster deployed remotely.
pub struct RemoteRunningCluster {
    fullnode_url: String,
    faucet_url: String,
    config_directory: tempfile::TempDir,
}

#[async_trait]
impl Cluster for RemoteRunningCluster {
    async fn start(options: &ClusterTestOpt) -> Result<Self, anyhow::Error> {
        let (fullnode_url, faucet_url) = match options.env {
            Env::Devnet => (
                String::from(DEVNET_FULLNODE_ADDR),
                String::from(DEVNET_FAUCET_ADDR),
            ),
            Env::Staging => (
                String::from(STAGING_FULLNODE_ADDR),
                String::from(STAGING_FAUCET_ADDR),
            ),
            Env::Ci => (
                String::from(CONTINUOUS_FULLNODE_ADDR),
                String::from(CONTINUOUS_FAUCET_ADDR),
            ),
            Env::CiNomad => (
                String::from(CONTINUOUS_NOMAD_FULLNODE_ADDR),
                String::from(CONTINUOUS_NOMAD_FAUCET_ADDR),
            ),
            Env::Testnet => (
                String::from(TESTNET_FULLNODE_ADDR),
                String::from(TESTNET_FAUCET_ADDR),
            ),
            Env::CustomRemote => (
                options
                    .fullnode_address
                    .clone()
                    .expect("Expect 'fullnode_address' for Env::Custom"),
                options
                    .faucet_address
                    .clone()
                    .expect("Expect 'faucet_address' for Env::Custom"),
            ),
            Env::NewLocal => unreachable!("NewLocal shouldn't use RemoteRunningCluster"),
        };

        // TODO: test connectivity before proceeding?

        Ok(Self {
            fullnode_url,
            faucet_url,
            config_directory: tempfile::tempdir()?,
        })
    }

    fn fullnode_url(&self) -> &str {
        &self.fullnode_url
    }

    fn user_key(&self) -> AccountKeyPair {
        get_key_pair().1
    }

    fn remote_faucet_url(&self) -> Option<&str> {
        Some(&self.faucet_url)
    }

    fn local_faucet_key(&self) -> Option<&AccountKeyPair> {
        None
    }

    fn config_directory(&self) -> &Path {
        self.config_directory.path()
    }
}

/// Represents a local Cluster which starts per cluster test run.
pub struct LocalNewCluster {
    test_cluster: TestCluster,
    fullnode_url: String,
    faucet_key: AccountKeyPair,
    config_directory: tempfile::TempDir,
    #[allow(unused)]
    data_ingestion_path: tempfile::TempDir,
}

impl LocalNewCluster {
    #[allow(unused)]
    pub fn swarm(&self) -> &Swarm {
        &self.test_cluster.swarm
    }
}

#[async_trait]
impl Cluster for LocalNewCluster {
    async fn start(options: &ClusterTestOpt) -> Result<Self, anyhow::Error> {
        let data_ingestion_path = tempdir()?;

        let mut cluster_builder = TestClusterBuilder::new()
            .enable_fullnode_events()
            .with_data_ingestion_dir(data_ingestion_path.path().to_path_buf());

        // Check if we already have a config directory that is passed
        if let Some(config_dir) = options.config_dir.clone() {
            assert!(options.epoch_duration_ms.is_none());
            // Load the config of the MySo authority.
            let network_config_path = config_dir.join(MYSO_NETWORK_CONFIG);
            let network_config: NetworkConfig = PersistedConfig::read(&network_config_path)
                .map_err(|err| {
                    err.context(format!(
                        "Cannot open MySo network config file at {:?}",
                        network_config_path
                    ))
                })?;

            cluster_builder = cluster_builder.set_network_config(network_config);
            cluster_builder = cluster_builder.with_config_dir(config_dir);
        } else {
            // Let the faucet account hold 1000 gas objects on genesis
            let genesis_config = GenesisConfig::custom_genesis(1, 100);
            // Custom genesis should be build here where we add the extra accounts
            cluster_builder = cluster_builder.set_genesis_config(genesis_config);

            if let Some(epoch_duration_ms) = options.epoch_duration_ms {
                cluster_builder = cluster_builder.with_epoch_duration_ms(epoch_duration_ms);
            }
        }

        let mut test_cluster = cluster_builder.build().await;

        // Use the wealthy account for faucet
        let faucet_key = test_cluster.swarm.config_mut().account_keys.swap_remove(0);
        let faucet_address = MySoAddress::from(faucet_key.public());
        info!(?faucet_address, "faucet_address");

        // This cluster has fullnode handle, safe to unwrap
        let fullnode_url = test_cluster.fullnode_handle.rpc_url.clone();

        // Let nodes connect to one another
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        Ok(Self {
            test_cluster,
            fullnode_url,
            faucet_key,
            config_directory: tempfile::tempdir()?,
            data_ingestion_path,
        })
    }

    fn fullnode_url(&self) -> &str {
        &self.fullnode_url
    }

    fn user_key(&self) -> AccountKeyPair {
        get_key_pair().1
    }

    fn remote_faucet_url(&self) -> Option<&str> {
        None
    }

    fn local_faucet_key(&self) -> Option<&AccountKeyPair> {
        Some(&self.faucet_key)
    }

    fn config_directory(&self) -> &Path {
        self.config_directory.path()
    }
}

// Make linter happy
#[async_trait]
impl Cluster for Box<dyn Cluster + Send + Sync> {
    async fn start(_options: &ClusterTestOpt) -> Result<Self, anyhow::Error> {
        unreachable!(
            "If we already have a boxed Cluster trait object we wouldn't have to call this function"
        );
    }
    fn fullnode_url(&self) -> &str {
        (**self).fullnode_url()
    }

    fn user_key(&self) -> AccountKeyPair {
        (**self).user_key()
    }

    fn remote_faucet_url(&self) -> Option<&str> {
        (**self).remote_faucet_url()
    }

    fn local_faucet_key(&self) -> Option<&AccountKeyPair> {
        (**self).local_faucet_key()
    }

    fn config_directory(&self) -> &Path {
        (**self).config_directory()
    }
}

pub async fn new_wallet_context_from_cluster(
    cluster: &(dyn Cluster + Sync + Send),
    key_pair: AccountKeyPair,
) -> WalletContext {
    let config_dir = cluster.config_directory();
    let wallet_config_path = config_dir.join("client.yaml");
    let fullnode_url = cluster.fullnode_url();
    info!("Use RPC: {}", &fullnode_url);
    let keystore_path = config_dir.join(MYSO_KEYSTORE_FILENAME);
    let mut keystore = Keystore::from(FileBasedKeystore::load_or_create(&keystore_path).unwrap());
    let address: MySoAddress = key_pair.public().into();
    keystore
        .import(None, MySoKeyPair::Ed25519(key_pair))
        .await
        .unwrap();
    MySoClientConfig {
        keystore,
        external_keys: None,
        envs: vec![MySoEnv {
            alias: "localnet".to_string(),
            rpc: fullnode_url.into(),
            ws: None,
            basic_auth: None,
            chain_id: None,
        }],
        active_address: Some(address),
        active_env: Some("localnet".to_string()),
    }
    .persisted(&wallet_config_path)
    .save()
    .unwrap();

    info!(
        "Initialize wallet from config path: {:?}",
        wallet_config_path
    );

    WalletContext::new(&wallet_config_path).unwrap_or_else(|e| {
        panic!(
            "Failed to init wallet context from path {:?}, error: {e}",
            wallet_config_path
        )
    })
}
