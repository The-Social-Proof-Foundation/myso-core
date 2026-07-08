// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod adapters;
pub mod registry;
pub mod types;

pub use registry::{build_default_registry, SourceRegistry};
pub use types::*;

use async_trait::async_trait;

#[async_trait]
pub trait DiscoverySource: Send + Sync {
    fn id(&self) -> &str;
    fn domain(&self) -> DiscoveryDomain;
    fn supports(&self, config: &SourceConfig) -> bool;
    async fn discover(&self, config: &SourceConfig) -> anyhow::Result<Vec<RawDiscoveryRecord>>;
    async fn health(&self) -> SourceHealth;
    fn metadata(&self) -> SourceMetadata;
}
