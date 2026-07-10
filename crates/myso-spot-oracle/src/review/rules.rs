// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::review::deadline::{DeadlinePolicy, DeadlineValidation};
use crate::review::CanonicalClaim;
use crate::resolver::{ResolverDefinition, ResolverSpec};
use crate::sources::ResolverRegistry;
use crate::types::{ClaimCategory, ComparisonOp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewDecision {
    Accepted,
    /// Post links to an existing claim/market (shared liquidity).
    LinkedToExisting,
    Rejected(RejectReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {
    MissingDeadline,
    DeadlineInPast,
    DeadlineTooFar,
    MissingThreshold,
    MissingComparison,
    DuplicateClaim,
    UnsupportedCategory,
    NoTrustedSource,
    InvalidOptions,
    AmbiguousClaim,
    MissingFeedUrl,
    MissingRepo,
    MissingUrl,
    MissingJsonPath,
}

impl RejectReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingDeadline => "missing_deadline",
            Self::DeadlineInPast => "deadline_in_past",
            Self::DeadlineTooFar => "deadline_too_far",
            Self::MissingThreshold => "missing_threshold",
            Self::MissingComparison => "missing_comparison",
            Self::DuplicateClaim => "duplicate_claim",
            Self::UnsupportedCategory => "unsupported_category",
            Self::NoTrustedSource => "no_trusted_source",
            Self::InvalidOptions => "invalid_options",
            Self::AmbiguousClaim => "ambiguous_claim",
            Self::MissingFeedUrl => "missing_feed_url",
            Self::MissingRepo => "missing_repo",
            Self::MissingUrl => "missing_url",
            Self::MissingJsonPath => "missing_json_path",
        }
    }
}

pub fn evaluate(
    canonical: &CanonicalClaim,
    market_exists: bool,
    registry: &ResolverRegistry,
    deadline_policy: &DeadlinePolicy,
) -> ReviewDecision {
    let f = &canonical.normalized_fields;
    if f.claim_category == ClaimCategory::Unsupported {
        return ReviewDecision::Rejected(RejectReason::UnsupportedCategory);
    }
    if f.subject.is_empty() || f.predicate.is_empty() {
        return ReviewDecision::Rejected(RejectReason::AmbiguousClaim);
    }
    if market_exists {
        return ReviewDecision::LinkedToExisting;
    }
    let options_len = f.suggested_options.len();
    if options_len < 2 || options_len > 10 {
        return ReviewDecision::Rejected(RejectReason::InvalidOptions);
    }
    let unique: std::collections::HashSet<_> = f.suggested_options.iter().collect();
    if unique.len() != options_len {
        return ReviewDecision::Rejected(RejectReason::InvalidOptions);
    }

    if f.deadline.is_none() {
        return ReviewDecision::Rejected(RejectReason::MissingDeadline);
    }
    match deadline_policy.validate(f.deadline.unwrap()) {
        DeadlineValidation::Ok => {}
        DeadlineValidation::InPast => {
            return ReviewDecision::Rejected(RejectReason::DeadlineInPast);
        }
        DeadlineValidation::TooFar => {
            return ReviewDecision::Rejected(RejectReason::DeadlineTooFar);
        }
    }

    match f.claim_category {
        ClaimCategory::PriceThreshold => {
            if f.threshold.is_none() {
                return ReviewDecision::Rejected(RejectReason::MissingThreshold);
            }
            if f.comparison.is_none() {
                return ReviewDecision::Rejected(RejectReason::MissingComparison);
            }
        }
        ClaimCategory::ReleasePublished => {
            let has_repo = f.resolver_hints.owner.is_some() && f.resolver_hints.repo.is_some()
                || f.object.contains('/');
            if !has_repo {
                return ReviewDecision::Rejected(RejectReason::MissingRepo);
            }
        }
        ClaimCategory::EventOccurrence => {
            if f.resolver_hints.feed_url.as_ref().is_none_or(|s| s.is_empty())
                && !f.suggested_sources.iter().any(|s| s.contains("rss"))
            {
                return ReviewDecision::Rejected(RejectReason::MissingFeedUrl);
            }
        }
        ClaimCategory::CustomHttp => {
            if f.resolver_hints.url.as_ref().is_none_or(|s| s.is_empty()) {
                return ReviewDecision::Rejected(RejectReason::MissingUrl);
            }
            if f.resolver_hints.json_path.as_ref().is_none_or(|s| s.is_empty()) {
                return ReviewDecision::Rejected(RejectReason::MissingJsonPath);
            }
        }
        ClaimCategory::Unsupported => {}
    }

    if registry.is_empty() {
        return ReviewDecision::Rejected(RejectReason::NoTrustedSource);
    }

    if let Some(preview) = preview_definition(canonical) {
        let preferred = &f.resolver_hints.preferred_sources;
        let fallback: &[&str] = match f.claim_category {
            ClaimCategory::PriceThreshold => &["coingecko", "coinbase", "http_official", "chainlink"],
            ClaimCategory::ReleasePublished => &["github_releases"],
            ClaimCategory::EventOccurrence => &["rss_event", "http_official"],
            ClaimCategory::CustomHttp => &["http_official"],
            ClaimCategory::Unsupported => &[],
        };
        if crate::review::compiler::source_select::select_source(
            registry,
            &[],
            &preview,
            preferred,
            fallback,
        )
        .is_err()
        {
            return ReviewDecision::Rejected(RejectReason::NoTrustedSource);
        }
    } else {
        return ReviewDecision::Rejected(RejectReason::UnsupportedCategory);
    }

    ReviewDecision::Accepted
}

fn preview_definition(canonical: &CanonicalClaim) -> Option<ResolverDefinition> {
    let f = &canonical.normalized_fields;
    let kind = f.claim_category.resolver_kind()?;
    let spec = match f.claim_category {
        ClaimCategory::PriceThreshold => ResolverSpec::PriceThreshold {
            asset: f.subject.clone(),
            quote: if f.object.is_empty() {
                "usd".to_string()
            } else {
                f.object.clone()
            },
            comparator: f.comparison.unwrap_or(ComparisonOp::Gt),
            threshold: f.threshold.clone().unwrap_or_default(),
            source_id: "preview".to_string(),
            json_path: "preview".to_string(),
        },
        ClaimCategory::ReleasePublished => ResolverSpec::ReleasePublished {
            owner: f.resolver_hints.owner.clone().unwrap_or_default(),
            repo: f.resolver_hints.repo.clone().unwrap_or_default(),
            tag_predicate: f.predicate.clone(),
            source_id: "preview".to_string(),
        },
        ClaimCategory::EventOccurrence => ResolverSpec::EventOccurrence {
            feed_url: f
                .resolver_hints
                .feed_url
                .clone()
                .unwrap_or_else(|| "https://example.com/feed".to_string()),
            match_predicate: f.predicate.clone(),
            source_id: "preview".to_string(),
        },
        ClaimCategory::CustomHttp => ResolverSpec::CustomHttp {
            url: f.resolver_hints.url.clone().unwrap_or_default(),
            json_path: f.resolver_hints.json_path.clone().unwrap_or_default(),
            comparator: f.comparison.unwrap_or(ComparisonOp::Eq),
            expected: f
                .resolver_hints
                .expected
                .clone()
                .or(f.threshold.clone())
                .unwrap_or_default(),
            source_id: "preview".to_string(),
        },
        ClaimCategory::Unsupported => return None,
    };
    Some(ResolverDefinition {
        id: uuid::Uuid::new_v4(),
        resolver_kind: kind,
        spec,
        source_ids: vec![],
        betting_options: f.suggested_options.clone(),
        maturity_schedule: crate::resolver::MaturitySchedule {
            maturity_at: chrono::Utc::now(),
            deadline: chrono::Utc::now(),
        },
    })
}

pub fn is_price_claim(canonical: &CanonicalClaim) -> bool {
    canonical.normalized_fields.claim_category == ClaimCategory::PriceThreshold
        || {
            let f = &canonical.normalized_fields;
            f.metric.as_deref() == Some("price")
                || f.predicate.contains("price")
                || matches!(
                    f.comparison,
                    Some(ComparisonOp::Gt | ComparisonOp::Lt | ComparisonOp::Gte | ComparisonOp::Lte)
                )
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::review::canonicalize::{canonicalize, OutcomeType};
    use crate::review::compiler::fixtures;
    use crate::review::compiler::test_registry;
    use crate::store::reviews::ExtractedClaim;
    use crate::types::ResolverHints;
    use uuid::Uuid;

    #[test]
    fn unsupported_claim_rejected() {
        let canonical = fixtures::unsupported_claim();
        let registry = test_registry();
        let decision = evaluate(&canonical, false, &registry, &DeadlinePolicy::default());
        assert_eq!(
            decision,
            ReviewDecision::Rejected(RejectReason::UnsupportedCategory)
        );
    }

    #[test]
    fn btc_price_accepted() {
        let canonical = fixtures::btc_price_claim();
        let registry = test_registry();
        let decision = evaluate(&canonical, false, &registry, &DeadlinePolicy::default());
        assert_eq!(decision, ReviewDecision::Accepted);
    }

    #[test]
    fn price_without_deadline_gets_ongoing_spacing() {
        let event_registry = crate::events::EventRegistry::new();
        let mut extracted = ExtractedClaim {
            subject: "bitcoin".to_string(),
            predicate: "price".to_string(),
            object: "usd".to_string(),
            metric: Some("price".to_string()),
            comparison: Some(ComparisonOp::Gt),
            threshold: Some("1".to_string()),
            deadline: crate::review::deadline::resolve_claim_deadline(
                "BTC above $1",
                ClaimCategory::PriceThreshold,
                &event_registry,
            ),
            outcome_type: OutcomeType::Binary,
            suggested_sources: vec!["coingecko".to_string()],
            suggested_options: vec!["Yes".to_string(), "No".to_string()],
            claim_category: ClaimCategory::PriceThreshold,
            resolver_hints: ResolverHints::default(),
        };
        let canonical = canonicalize(Uuid::new_v4(), &extracted);
        assert!(canonical.normalized_fields.deadline.is_some());
        let registry = test_registry();
        let decision = evaluate(&canonical, false, &registry, &DeadlinePolicy::default());
        assert_eq!(decision, ReviewDecision::Accepted);
        extracted.deadline = None;
        let canonical = canonicalize(Uuid::new_v4(), &extracted);
        let decision = evaluate(&canonical, false, &registry, &DeadlinePolicy::default());
        assert_eq!(
            decision,
            ReviewDecision::Rejected(RejectReason::MissingDeadline)
        );
    }
}
