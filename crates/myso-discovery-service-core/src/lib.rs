// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Reusable discovery primitives shared by `myso-discovery-service` (PoC crawl runtime)
//! and `myso-spot-oracle` (read-only source registry).
//!
//! Boundary: this crate contains traits, registry, lifecycle, models, source-config parsing,
//! the factual `/v1/*` API types + HTTP client, and a shared external fetch client. It does
//! **not** contain the scheduler, runtime, embed client, store, admin, jobs, metrics, or
//! identity — those live in the `myso-discovery-service` binary crate.

pub mod api;
pub mod lifecycle;
pub mod normalizer;
pub mod prioritizer;
pub mod sources;
