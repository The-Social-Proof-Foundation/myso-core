// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;
use uuid::Uuid;

use crate::review::canonicalize::{CanonicalClaimFields, OutcomeType};
use crate::review::outcome_identity::{build_outcome_identity, build_outcome_market_key};
use crate::types::{ClaimCategory, ComparisonOp, ResolverHints};

fn test_claim(fields: CanonicalClaimFields) -> crate::review::CanonicalClaim {
    let outcome_identity = build_outcome_identity(
        fields.entity_ref.clone(),
        fields.competition_ref.clone(),
        fields.event_ref.clone(),
        fields.metric_ref.clone(),
        fields.predicate.clone(),
        fields.object.clone(),
        fields.metric.clone(),
        fields.comparison,
        fields.threshold.clone(),
        fields.outcome_type,
        fields.claim_category,
        fields.suggested_sources.clone(),
    );
    let outcome_market_key = build_outcome_market_key(
        outcome_identity.clone(),
        fields.deadline,
        fields.suggested_options.clone(),
        fields.claim_category,
        chrono::Duration::hours(24),
    );
    crate::review::CanonicalClaim {
        normalized_fields: fields,
        claim_hash: [0u8; 32],
        semantic_claim_hash: [0u8; 32],
        market_key_hash: [0u8; 32],
        source_extraction_id: Uuid::new_v4(),
        outcome_identity,
        outcome_market_key,
    }
}

pub fn btc_price_claim() -> crate::review::CanonicalClaim {
    let fields = CanonicalClaimFields {
        subject: "bitcoin".to_string(),
        predicate: "price".to_string(),
        object: "usd".to_string(),
        metric: Some("price".to_string()),
        comparison: Some(ComparisonOp::Gt),
        threshold: Some("1".to_string()),
        deadline: Some(Utc::now() + chrono::Duration::hours(24)),
        outcome_type: OutcomeType::Binary,
        suggested_sources: vec!["coingecko".to_string()],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
        claim_category: ClaimCategory::PriceThreshold,
        resolver_hints: ResolverHints {
            preferred_sources: vec!["coingecko".to_string()],
            ..Default::default()
        },
        entity_ref: None,
        competition_ref: None,
        event_ref: None,
        metric_ref: Some("price_usd".to_string()),
    };
    test_claim(fields)
}

pub fn github_release_claim() -> crate::review::CanonicalClaim {
    let fields = CanonicalClaimFields {
        subject: "rust".to_string(),
        predicate: "release".to_string(),
        object: "rust-lang/rust".to_string(),
        metric: None,
        comparison: None,
        threshold: None,
        deadline: Some(Utc::now() + chrono::Duration::days(7)),
        outcome_type: OutcomeType::Binary,
        suggested_sources: vec!["github_releases".to_string()],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
        claim_category: ClaimCategory::ReleasePublished,
        resolver_hints: ResolverHints {
            owner: Some("rust-lang".to_string()),
            repo: Some("rust".to_string()),
            tag_predicate: Some("1.80".to_string()),
            preferred_sources: vec!["github_releases".to_string()],
            ..Default::default()
        },
        entity_ref: None,
        competition_ref: None,
        event_ref: None,
        metric_ref: None,
    };
    test_claim(fields)
}

pub fn rss_event_claim() -> crate::review::CanonicalClaim {
    let fields = CanonicalClaimFields {
        subject: "fed".to_string(),
        predicate: "rate cut".to_string(),
        object: "".to_string(),
        metric: None,
        comparison: None,
        threshold: None,
        deadline: Some(Utc::now() + chrono::Duration::days(30)),
        outcome_type: OutcomeType::Binary,
        suggested_sources: vec!["rss_event".to_string()],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
        claim_category: ClaimCategory::EventOccurrence,
        resolver_hints: ResolverHints {
            feed_url: Some("https://www.federalreserve.gov/feeds/press_all.xml".to_string()),
            match_predicate: Some("rate cut".to_string()),
            preferred_sources: vec!["rss_event".to_string()],
            ..Default::default()
        },
        entity_ref: None,
        competition_ref: None,
        event_ref: None,
        metric_ref: None,
    };
    test_claim(fields)
}

pub fn custom_http_claim() -> crate::review::CanonicalClaim {
    let fields = CanonicalClaimFields {
        subject: "api".to_string(),
        predicate: "status".to_string(),
        object: "".to_string(),
        metric: None,
        comparison: Some(ComparisonOp::Eq),
        threshold: Some("ok".to_string()),
        deadline: Some(Utc::now() + chrono::Duration::days(1)),
        outcome_type: OutcomeType::Binary,
        suggested_sources: vec!["http_official".to_string()],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
        claim_category: ClaimCategory::CustomHttp,
        resolver_hints: ResolverHints {
            url: Some("https://httpbin.org/json".to_string()),
            json_path: Some("slideshow.author".to_string()),
            expected: Some("Yours Truly".to_string()),
            comparison: Some(ComparisonOp::Eq),
            preferred_sources: vec!["http_official".to_string()],
            ..Default::default()
        },
        entity_ref: None,
        competition_ref: None,
        event_ref: None,
        metric_ref: None,
    };
    test_claim(fields)
}

pub fn unsupported_claim() -> crate::review::CanonicalClaim {
    let fields = CanonicalClaimFields {
        subject: "".to_string(),
        predicate: "".to_string(),
        object: "".to_string(),
        metric: None,
        comparison: None,
        threshold: None,
        deadline: None,
        outcome_type: OutcomeType::Binary,
        suggested_sources: vec![],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
        claim_category: ClaimCategory::Unsupported,
        resolver_hints: ResolverHints::default(),
        entity_ref: None,
        competition_ref: None,
        event_ref: None,
        metric_ref: None,
    };
    test_claim(fields)
}
