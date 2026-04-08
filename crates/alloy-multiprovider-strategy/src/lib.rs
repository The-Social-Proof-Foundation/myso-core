//! # alloy-multiprovider-strategy
//!
//! A multi-provider wrapper for [alloy](https://alloy.rs/) with quorum-based consensus
//! and automatic health ranking.
//!
//! ## Features
//!
//! - **Multiple RPC Providers**: Connect to multiple Ethereum RPC endpoints simultaneously
//! - **Quorum Consensus**: Require a configurable number of providers to agree on responses
//! - **Automatic Health Ranking**: Providers are ranked by health (block height and latency)
//! - **Periodic Health Checks**: Background task monitors provider health at configurable intervals
//! - **Graceful Fallback**: If top providers fail, automatically try the next ranked providers
//! - **Full Provider Trait**: Implements the complete `alloy::Provider` trait - no need for special methods!
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use alloy_multiprovider_strategy::{MultiProvider, MultiProviderConfig};
//! use alloy_provider::Provider;  // Import the Provider trait!
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create configuration
//!     let config = MultiProviderConfig::new(
//!         vec![
//!             "https://eth.llamarpc.com".to_string(),
//!             "https://rpc.ankr.com/eth".to_string(),
//!             "https://ethereum.publicnode.com".to_string(),
//!         ],
//!         2, // Require 2 matching responses for quorum
//!     )
//!     .with_health_check_interval(Duration::from_secs(300))
//!     .with_max_block_delta(5)
//!     .with_max_healthy_latency(Duration::from_secs(5));
//!
//!     // Create the multi-provider
//!     let provider = MultiProvider::new(config).await?;
//!
//!     // Use standard Provider trait methods - they automatically use quorum!
//!     let block_number = provider.get_block_number().await?;
//!     println!("Current block: {}", block_number);
//!
//!     // All Provider trait methods work with quorum consensus
//!     let chain_id = provider.get_chain_id().await?;
//!     println!("Chain ID: {}", chain_id);
//!
//!     // Check provider health
//!     for (url, status) in provider.get_provider_health() {
//!         println!("{}: {:?}", url, status);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod health;
pub mod provider;
pub mod transport;

// Re-export main types at crate root
pub use config::MultiProviderConfig;
pub use error::{MultiProviderError, Result};
pub use health::{HealthStatus, ProviderHealth};
pub use provider::{InnerProvider, MultiProvider};
pub use transport::QuorumTransport;

// Re-export commonly used alloy types for convenience
pub use alloy_network::Ethereum;
pub use alloy_primitives::{Address, Bytes, TxHash, B256, U256};
pub use alloy_provider::{Provider, RootProvider};
