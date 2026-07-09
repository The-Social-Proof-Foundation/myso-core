// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! `myso-spot-oracle` — single-process Axum/Tokio service that gates off-chain
//! claim review and deterministic resolution against the existing
//! `social_proof_of_truth.move` contract.
//!
//! Dependency boundary: depends on `myso-discovery-service-core` (traits,
//! `DiscoveryRegistry`, lifecycle, models, source-config YAML loader) and the
//! `myso-discovery-service-schema` + `myso-spot-oracle-schema` migrations only.
//! It never imports the `myso-discovery-service` runtime/scheduler/store.

pub mod api;
pub mod blockchain;
pub mod claim;
pub mod config;
pub mod evidence;
pub mod ingest;
pub mod jobs;
pub mod metrics;
pub mod resolver;
pub mod review;
pub mod rss;
pub mod runtime;
pub mod scheduler;
pub mod social_client;
pub mod sources;
pub mod store;
pub mod types;
