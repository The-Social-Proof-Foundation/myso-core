// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Resolver framework: immutable `ResolverDefinition`, executor kinds, and the
//! `Resolver` trait. Compile (review time) and resolve (resolution time) are split
//! so the scheduler + engine never interpret natural language — they only load a
//! stored definition and execute it.

pub mod engine;

pub use crate::types::ResolverKind;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::review::CanonicalClaim;
use crate::sources::{ResolverRegistry, TrustedSource};
use crate::types::ComparisonOp;

/// When the scheduler may first attempt resolution, and the hard deadline after
/// which unresolved markets are refunded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaturitySchedule {
    pub maturity_at: DateTime<Utc>,
    pub deadline: DateTime<Utc>,
}

/// Immutable, fully-specified execution spec. No free text — every field the
/// executor needs is concrete (URLs, JSON paths, thresholds, option labels).
/// Persisted once by the Resolver Compiler and loaded by the resolver engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverDefinition {
    pub id: Uuid,
    pub resolver_kind: ResolverKind,
    pub spec: ResolverSpec,
    /// Selected `TrustedSource` adapter ids (cross-referenced with the registry).
    pub source_ids: Vec<String>,
    /// Market betting-option labels, in on-chain order. The resolver emits an
    /// outcome label; the engine maps it back to `outcome_option_id`.
    pub betting_options: Vec<String>,
    pub maturity_schedule: MaturitySchedule,
}

/// Deterministic execution parameters per resolver kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolverSpec {
    PriceThreshold {
        asset: String,
        quote: String,
        comparator: ComparisonOp,
        threshold: String,
        source_id: String,
        json_path: String,
    },
    EventOccurrence {
        feed_url: String,
        match_predicate: String,
        source_id: String,
    },
    ReleasePublished {
        owner: String,
        repo: String,
        tag_predicate: String,
        source_id: String,
    },
    CustomHttp {
        url: String,
        json_path: String,
        comparator: ComparisonOp,
        expected: String,
        source_id: String,
    },
}

/// Resolver-engine output before `outcome_option_id` mapping. `outcome_label`
/// matches one of `ResolverDefinition.betting_options`; `None` => unresolved.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionDraft {
    pub outcome_label: Option<String>,
    pub confidence_bps: u16,
    pub reasoning: String,
    pub evidence_urls: Vec<String>,
}

/// A resolver kind: compiles a canonical claim into a `ResolverDefinition` (review
/// time) and executes a stored definition against the registry (resolution time).
/// `compile` is called only by `review/compiler.rs`; `resolve` only by
/// `resolver/engine.rs`. The scheduler never calls either — it dispatches
/// `spot_jobs` referencing a stored definition id.
#[async_trait]
pub trait Resolver: Send + Sync {
    fn kind(&self) -> ResolverKind;
    /// Called only by the Resolver Compiler at review time — not at resolution time.
    fn compile(
        canonical: &CanonicalClaim,
        sources: &[&dyn TrustedSource],
    ) -> anyhow::Result<ResolverDefinition>;
    /// Called by the Resolver Engine at resolution time — reads the stored
    /// definition only, never the claim text.
    async fn resolve(
        &self,
        def: &ResolverDefinition,
        registry: &ResolverRegistry,
    ) -> anyhow::Result<ResolutionDraft>;
}
