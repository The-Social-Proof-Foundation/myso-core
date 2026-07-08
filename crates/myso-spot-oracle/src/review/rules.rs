// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::review::CanonicalClaim;
use crate::sources::ResolverRegistry;
use crate::types::ComparisonOp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewDecision {
    Accepted,
    Rejected(RejectReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {
    MissingDeadline,
    MissingThreshold,
    MissingComparison,
    DuplicateClaim,
    UnsupportedCategory,
    NoTrustedSource,
    InvalidOptions,
    AmbiguousClaim,
}

impl RejectReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingDeadline => "missing_deadline",
            Self::MissingThreshold => "missing_threshold",
            Self::MissingComparison => "missing_comparison",
            Self::DuplicateClaim => "duplicate_claim",
            Self::UnsupportedCategory => "unsupported_category",
            Self::NoTrustedSource => "no_trusted_source",
            Self::InvalidOptions => "invalid_options",
            Self::AmbiguousClaim => "ambiguous_claim",
        }
    }
}

pub fn evaluate(
    canonical: &CanonicalClaim,
    duplicate: bool,
    registry: &ResolverRegistry,
) -> ReviewDecision {
    let f = &canonical.normalized_fields;
    if f.subject.is_empty() || f.predicate.is_empty() {
        return ReviewDecision::Rejected(RejectReason::AmbiguousClaim);
    }
    if duplicate {
        return ReviewDecision::Rejected(RejectReason::DuplicateClaim);
    }
    let options_len = f.suggested_options.len();
    if !(2..=10).contains(&options_len) {
        return ReviewDecision::Rejected(RejectReason::InvalidOptions);
    }
    let unique: std::collections::HashSet<_> = f.suggested_options.iter().collect();
    if unique.len() != options_len {
        return ReviewDecision::Rejected(RejectReason::InvalidOptions);
    }

    let is_price = f.metric.as_deref() == Some("price")
        || f.predicate.contains("price")
        || f.subject == "bitcoin"
        || f.subject == "ethereum";

    if is_price {
        if f.threshold.is_none() {
            return ReviewDecision::Rejected(RejectReason::MissingThreshold);
        }
        if f.comparison.is_none() {
            return ReviewDecision::Rejected(RejectReason::MissingComparison);
        }
        if registry.get("coingecko").is_none()
            && registry.get("coinbase").is_none()
            && registry.get("http_official").is_none()
        {
            return ReviewDecision::Rejected(RejectReason::NoTrustedSource);
        }
        return ReviewDecision::Accepted;
    }

    if f.deadline.is_none() {
        return ReviewDecision::Rejected(RejectReason::MissingDeadline);
    }

    if registry.is_empty() {
        return ReviewDecision::Rejected(RejectReason::NoTrustedSource);
    }

    ReviewDecision::Accepted
}

pub fn is_price_claim(canonical: &CanonicalClaim) -> bool {
    let f = &canonical.normalized_fields;
    f.metric.as_deref() == Some("price")
        || f.predicate.contains("price")
        || matches!(f.comparison, Some(ComparisonOp::Gt | ComparisonOp::Lt | ComparisonOp::Gte | ComparisonOp::Lte))
}
