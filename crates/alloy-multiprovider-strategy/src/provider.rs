//! Multi-provider with quorum-based consensus.
//!
//! This module provides the main `MultiProvider` type that wraps multiple RPC providers
//! and implements the `Provider` trait with automatic quorum consensus.

use crate::config::MultiProviderConfig;
use crate::error::Result;
use crate::health::HealthStatus;
use crate::transport::QuorumTransport;

use alloy_network::Ethereum;
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_client::RpcClient;

pub type InnerProvider = RootProvider<Ethereum>;

/// A multi-provider that wraps multiple RPC providers with quorum-based consensus
/// and automatic health ranking.
///
/// This type implements the full `Provider` trait, so you can use all standard
/// provider methods (e.g., `get_block_number()`, `get_balance()`, `call()`, etc.)
/// and they will automatically use quorum consensus across multiple providers.
///
/// # Example
///
/// ```rust,no_run
/// use alloy_multiprovider_strategy::{MultiProvider, MultiProviderConfig};
/// use alloy_provider::Provider;  // Import the Provider trait
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = MultiProviderConfig::new(
///     vec![
///         "https://eth.llamarpc.com".to_string(),
///         "https://rpc.ankr.com/eth".to_string(),
///     ],
///     2, // Require 2 matching responses
/// );
///
/// let provider = MultiProvider::new(config).await?;
///
/// // Use standard Provider trait methods - they automatically use quorum!
/// let block_number = provider.get_block_number().await?;
/// println!("Block: {}", block_number);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct MultiProvider {
    inner: InnerProvider,
    transport: QuorumTransport,
}

impl MultiProvider {
    pub async fn new(config: MultiProviderConfig) -> Result<Self> {
        let start_health_check = config.start_health_check_on_init;
        let transport = QuorumTransport::new(config)?;

        transport.run_health_check().await;

        if start_health_check {
            transport.start_health_check_task();
        }

        let client = RpcClient::new(transport.clone(), false);
        let inner = RootProvider::new(client);

        Ok(Self { inner, transport })
    }

    pub async fn from_urls(urls: Vec<String>, quorum: usize) -> Result<Self> {
        let config = MultiProviderConfig::new(urls, quorum);
        Self::new(config).await
    }

    pub fn transport(&self) -> &QuorumTransport {
        &self.transport
    }

    pub async fn run_health_check(&self) {
        self.transport.run_health_check().await;
    }

    pub fn start_health_check_task(&self) {
        self.transport.start_health_check_task();
    }

    pub fn shutdown(&self) {
        self.transport.shutdown();
    }

    pub fn get_provider_health(&self) -> Vec<(String, HealthStatus)> {
        self.transport.get_provider_health()
    }

    pub fn healthy_provider_count(&self) -> usize {
        self.transport.healthy_provider_count()
    }

    pub fn provider_count(&self) -> usize {
        self.transport.provider_count()
    }

    pub fn quorum(&self) -> usize {
        self.transport.quorum()
    }
}

impl Provider for MultiProvider {
    fn root(&self) -> &RootProvider<Ethereum> {
        &self.inner
    }
}
