// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! `myso-discovery-service` binary crate. Runtime/scheduler/store/api/admin live here;
//! reusable primitives (`sources`, `lifecycle`, `normalizer`, `prioritizer`) are imported
//! from `myso-discovery-service-core` and re-exported so existing `crate::` paths resolve.

pub mod admin;
pub mod api;
pub mod config;
pub mod embed_client;
pub mod identity;
pub mod jobs;
pub mod metrics;
pub mod runtime;
pub mod scheduler;
pub mod store;

// Re-export reusable primitives from `-core` so `crate::sources`, `crate::lifecycle`,
// `crate::normalizer`, `crate::prioritizer` keep resolving inside this crate and for
// any in-flight downstream code that imports `myso_discovery_service::sources`.
pub use myso_discovery_service_core::{lifecycle, normalizer, prioritizer, sources};

// Backwards-compat alias: `SourceRegistry` is now `DiscoveryRegistry` in `-core`. Kept for
// one release so in-flight discovery code keeps compiling; new code should use `DiscoveryRegistry`.
#[deprecated(
    since = "0.1.0",
    note = "use `DiscoveryRegistry` (responsibility-matched name); `SourceRegistry` is a compat alias"
)]
pub type SourceRegistry = myso_discovery_service_core::sources::DiscoveryRegistry;
