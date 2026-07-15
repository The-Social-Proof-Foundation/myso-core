// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Primitives shared by the review (claim interpretation) and resolver
//! (deterministic execution) layers. Kept in one neutral module so neither
//! layer depends on the other purely to reuse an enum.

use serde::{Deserialize, Serialize};

/// Numeric/string comparison operator used both in extracted claims and in the
/// immutable `ResolverSpec`. Canonicalized to one of these forms before compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOp {
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,
}

/// Executor kind emitted by the Resolver Compiler. The scheduler + resolver engine
/// branch on this enum only — never on adapter identity or claim text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverKind {
    PriceThreshold,
    EventOccurrence,
    ReleasePublished,
    CustomHttp,
}

/// LLM-assigned claim category; maps 1:1 to [`ResolverKind`] for supported claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ClaimCategory {
    #[default]
    Unsupported,
    PriceThreshold,
    ReleasePublished,
    EventOccurrence,
    CustomHttp,
}

/// When a claim resolves relative to now. `future` claims open live betting markets; `past`
/// claims are verified against trusted sources (never opening a new market); `unsupported`
/// claims are objectively non-checkable.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum TimeClass {
    #[default]
    Future,
    Past,
    Unsupported,
}

impl ClaimCategory {
    pub fn resolver_kind(self) -> Option<ResolverKind> {
        match self {
            Self::PriceThreshold => Some(ResolverKind::PriceThreshold),
            Self::ReleasePublished => Some(ResolverKind::ReleasePublished),
            Self::EventOccurrence => Some(ResolverKind::EventOccurrence),
            Self::CustomHttp => Some(ResolverKind::CustomHttp),
            Self::Unsupported => None,
        }
    }
}

/// Kind-specific hints from LLM extraction; flattened in JSON on `ExtractedClaim`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResolverHints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_predicate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_predicate: Option<String>,
    #[serde(default)]
    pub match_fields: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comparison: Option<ComparisonOp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
    #[serde(default)]
    pub preferred_sources: Vec<String>,
}

/// Off-chain market lifecycle status (stored in `markets.status`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketStatus {
    PostCreated,
    PendingReview,
    Rejected,
    PendingCreate,
    Waiting,
    Resolving,
    DaoRequired,
    Resolved,
    Refunded,
    Failed,
}

impl MarketStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PostCreated => "post_created",
            Self::PendingReview => "pending_review",
            Self::Rejected => "rejected",
            Self::PendingCreate => "pending_create",
            Self::Waiting => "waiting",
            Self::Resolving => "resolving",
            Self::DaoRequired => "dao_required",
            Self::Resolved => "resolved",
            Self::Refunded => "refunded",
            Self::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "post_created" => Some(Self::PostCreated),
            "pending_review" => Some(Self::PendingReview),
            "rejected" => Some(Self::Rejected),
            "pending_create" => Some(Self::PendingCreate),
            "waiting" | "active" => Some(Self::Waiting),
            "resolving" => Some(Self::Resolving),
            "dao_required" => Some(Self::DaoRequired),
            "resolved" => Some(Self::Resolved),
            "refunded" => Some(Self::Refunded),
            "failed" => Some(Self::Failed),
            _ => None,
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Rejected | Self::Resolved | Self::Refunded | Self::Failed
        )
    }
}

/// On-chain `SpotRecord.status` values from `social_proof_of_truth.move`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnChainSpotStatus {
    Open = 1,
    DaoRequired = 2,
    Resolved = 3,
    Refundable = 4,
}

impl OnChainSpotStatus {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Open),
            2 => Some(Self::DaoRequired),
            3 => Some(Self::Resolved),
            4 => Some(Self::Refundable),
            _ => None,
        }
    }
}
