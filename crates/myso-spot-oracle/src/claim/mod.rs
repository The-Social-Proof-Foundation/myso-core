// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Claim lifecycle state machine — off-chain status transitions aligned with
//! on-chain `SpotRecord` states in `social_proof_of_truth.move`.

pub mod lifecycle;

pub use lifecycle::{apply_transition, LifecycleEvent, TransitionContext};
