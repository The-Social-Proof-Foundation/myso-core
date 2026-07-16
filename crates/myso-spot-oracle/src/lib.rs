// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! `myso-spot-oracle` — single-process Axum/Tokio service that gates off-chain
//! claim review and deterministic resolution against the existing
//! `social_proof_of_truth.move` contract.
//!
//! SPoT owns its trusted-source registry and fetches sources directly at
//! resolution time. It has no compile-time or runtime dependency on Discovery.

pub mod api;
pub mod blockchain;
pub mod claim;
pub mod config;
pub mod evidence;
pub mod events;
pub mod ingest;
pub mod jobs;
pub mod knowledge;
pub mod metrics;
pub mod resolver;
pub mod review;
pub mod runtime;
pub mod scheduler;
pub mod social_client;
pub mod sources;
pub mod store;
pub mod types;
