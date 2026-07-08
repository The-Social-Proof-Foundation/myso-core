// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Reusable discovery primitives shared by `myso-discovery-service` (PoC crawl runtime)
//! and `myso-spot-oracle` (read-only source registry).
//!
//! Boundary: this crate contains only traits, registry, lifecycle, models, source-config
//! parsing, and a shared HTTP fetch client. It does **not** contain the scheduler, runtime,
//! embed client, store, api, admin, jobs, metrics, or identity — those live in the
//! `myso-discovery-service` binary crate.

pub mod lifecycle;
pub mod normalizer;
pub mod prioritizer;
pub mod sources;
