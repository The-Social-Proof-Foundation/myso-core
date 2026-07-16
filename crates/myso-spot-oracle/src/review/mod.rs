// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Claim review pipeline: LLM extraction (NLU only), canonicalization,
//! deterministic admissibility rules, and the Resolver Compiler. Implemented
//! across the review-pipeline phase; types live here now so the resolver traits
//! can reference them.

pub mod canonicalize;
pub mod claim_matcher;
pub mod compiler;
pub mod context_deadline;
pub mod deadline;
pub mod ingest;
pub mod llm;
pub mod outcome_identity;
pub mod rules;
pub mod verify;
pub mod worker;

pub use canonicalize::{CanonicalClaim, CanonicalClaimFields, OutcomeType};
