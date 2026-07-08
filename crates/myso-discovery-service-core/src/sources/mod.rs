// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod adapters;
pub mod config_loader;
pub mod http_client;
pub mod registry;
pub mod types;

pub use registry::{build_default_registry, DiscoveryRegistry};
pub use types::*;

use async_trait::async_trait;

/// `DiscoverySource` discovers candidate content that *might* be useful (continuous crawl).
/// It never settles markets. SPoT's `TrustedSource::resolve()` is the separate contract
/// for deterministic settlement evidence and lives in `myso-spot-oracle`.
#[async_trait]
pub trait DiscoverySource: Send + Sync {
    fn id(&self) -> &str;
    fn domain(&self) -> DiscoveryDomain;
    fn supports(&self, config: &SourceConfig) -> bool;
    async fn discover(&self, config: &SourceConfig) -> anyhow::Result<Vec<RawDiscoveryRecord>>;
    async fn health(&self) -> SourceHealth;
    fn metadata(&self) -> SourceMetadata;
}
