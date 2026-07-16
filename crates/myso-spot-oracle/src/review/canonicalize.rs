// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::review::deadline::DEFAULT_PRICE_MARKET_SPACING;
use crate::review::outcome_identity::{
    build_outcome_identity, build_outcome_market_key, outcome_identity_hash,
    outcome_market_hash, OutcomeIdentityFields, OutcomeMarketKey,
};
use crate::store::reviews::ExtractedClaim;
use crate::types::{ClaimCategory, ComparisonOp, ResolverHints};

/// How a market resolves at maturity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeType {
    Binary,
    MultiChoice,
    Scalar,
}

/// Semantic identity of a claim (no deadline — permanent claim registry key).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticClaimFields {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comparison: Option<ComparisonOp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
    pub outcome_type: OutcomeType,
    pub suggested_sources: Vec<String>,
    #[serde(default)]
    pub claim_category: ClaimCategory,
    #[serde(default)]
    pub resolver_hints: ResolverHints,
}

/// Time-bounded market identity (semantic claim + deadline + betting options).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketKeyFields {
    pub semantic: SemanticClaimFields,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
    pub betting_options: Vec<String>,
}

/// Full normalized fields for resolver compilation (includes deadline + options).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalClaimFields {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comparison: Option<ComparisonOp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
    pub outcome_type: OutcomeType,
    pub suggested_sources: Vec<String>,
    pub suggested_options: Vec<String>,
    #[serde(default)]
    pub claim_category: ClaimCategory,
    #[serde(default)]
    pub resolver_hints: ResolverHints,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub competition_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_ref: Option<String>,
}

/// Canonical claim consumed by the Resolver Compiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalClaim {
    pub normalized_fields: CanonicalClaimFields,
    /// SHA-256 of stable outcome identity (graph refs — no resolver_hints).
    pub semantic_claim_hash: [u8; 32],
    /// SHA-256 of outcome identity + deadline day + betting_options.
    pub market_key_hash: [u8; 32],
    /// Legacy alias for market_key_hash.
    pub claim_hash: [u8; 32],
    pub source_extraction_id: Uuid,
    pub outcome_identity: OutcomeIdentityFields,
    pub outcome_market_key: OutcomeMarketKey,
}

const ASSET_ALIASES: &[(&str, &str)] = &[
    ("btc", "bitcoin"),
    ("eth", "ethereum"),
    ("sol", "solana"),
];

#[derive(Debug, Clone, Copy)]
pub struct CanonicalizeOptions {
    pub price_market_spacing: chrono::Duration,
}

impl Default for CanonicalizeOptions {
    fn default() -> Self {
        Self {
            price_market_spacing: DEFAULT_PRICE_MARKET_SPACING,
        }
    }
}

pub fn canonicalize(extraction_id: Uuid, extracted: &ExtractedClaim) -> CanonicalClaim {
    canonicalize_with_identity(extraction_id, extracted, None, &CanonicalizeOptions::default())
}

pub fn canonicalize_with_options(
    extraction_id: Uuid,
    extracted: &ExtractedClaim,
    opts: &CanonicalizeOptions,
) -> CanonicalClaim {
    canonicalize_with_identity(extraction_id, extracted, None, opts)
}

pub fn canonicalize_with_identity(
    extraction_id: Uuid,
    extracted: &ExtractedClaim,
    graph_refs: Option<crate::review::claim_matcher::ClaimMatch>,
    opts: &CanonicalizeOptions,
) -> CanonicalClaim {
    let subject = normalize_asset(&extracted.subject);
    let object = normalize_repo_or_asset(&extracted.object);
    let threshold = extracted.threshold.as_ref().map(|t| normalize_number(t));
    let comparison = extracted.comparison.or_else(|| infer_comparison(&extracted.predicate));
    let mut suggested_sources = extracted.suggested_sources.clone();
    suggested_sources.sort();
    suggested_sources.dedup();
    let mut suggested_options = extracted.suggested_options.clone();
    suggested_options.sort();
    suggested_options.dedup();

    let mut resolver_hints = extracted.resolver_hints.clone();
    if let Some(owner) = resolver_hints.owner.as_ref() {
        resolver_hints.owner = Some(normalize_repo_slug(owner));
    }
    if let Some(repo) = resolver_hints.repo.as_ref() {
        resolver_hints.repo = Some(normalize_repo_slug(repo));
    }
    if let Some(url) = resolver_hints.feed_url.as_ref() {
        resolver_hints.feed_url = Some(normalize_url(url));
    }
    if let Some(url) = resolver_hints.url.as_ref() {
        resolver_hints.url = Some(normalize_url(url));
    }
    resolver_hints
        .preferred_sources
        .sort();
    resolver_hints.preferred_sources.dedup();

    let claim_category = derive_category(extracted, &subject, &comparison);

    let predicate = extracted.predicate.trim().to_lowercase();
    let metric = extracted.metric.as_ref().map(|s| s.trim().to_lowercase());

    let refs = graph_refs.as_ref();
    let entity_ref = refs.and_then(|r| r.entity_ref.clone()).or_else(|| {
        extracted
            .resolver_hints
            .matched_event_id
            .as_ref()
            .map(|_| slugify_entity(&subject))
    });
    let event_ref = refs
        .and_then(|r| r.event_ref.clone())
        .or_else(|| extracted.resolver_hints.matched_event_id.clone());
    let competition_ref = refs.and_then(|r| r.competition_ref.clone());
    let metric_ref = refs.and_then(|r| r.metric_ref.clone());

    let outcome_identity = build_outcome_identity(
        entity_ref.clone(),
        competition_ref.clone(),
        event_ref.clone(),
        metric_ref.clone(),
        predicate.clone(),
        object.clone(),
        metric.clone(),
        comparison,
        threshold.clone(),
        extracted.outcome_type,
        claim_category,
        suggested_sources.clone(),
    );
    let semantic_claim_hash = outcome_identity_hash(&outcome_identity);

    let fields = CanonicalClaimFields {
        subject,
        predicate,
        object,
        metric,
        comparison,
        threshold,
        deadline: extracted.deadline,
        outcome_type: extracted.outcome_type,
        suggested_sources,
        suggested_options: suggested_options.clone(),
        claim_category,
        resolver_hints,
        entity_ref,
        competition_ref,
        event_ref,
        metric_ref,
    };

    let outcome_market_key = build_outcome_market_key(
        outcome_identity.clone(),
        extracted.deadline,
        suggested_options,
        claim_category,
        opts.price_market_spacing,
    );
    let market_key_hash = outcome_market_hash(&outcome_market_key);

    CanonicalClaim {
        normalized_fields: fields,
        semantic_claim_hash,
        market_key_hash,
        claim_hash: market_key_hash,
        source_extraction_id: extraction_id,
        outcome_identity,
        outcome_market_key,
    }
}

fn slugify_entity(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '.' {
                '_'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn derive_category(
    extracted: &ExtractedClaim,
    subject: &str,
    comparison: &Option<ComparisonOp>,
) -> ClaimCategory {
    if extracted.claim_category != ClaimCategory::Unsupported {
        return extracted.claim_category;
    }
    if extracted.metric.as_deref() == Some("price")
        || extracted.predicate.to_lowercase().contains("price")
        || subject == "bitcoin"
        || subject == "ethereum"
        || comparison.is_some()
    {
        return ClaimCategory::PriceThreshold;
    }
    if extracted.predicate.to_lowercase().contains("release")
        || extracted.suggested_sources.iter().any(|s| s.contains("github"))
        || extracted.resolver_hints.owner.is_some()
    {
        return ClaimCategory::ReleasePublished;
    }
    if extracted.resolver_hints.feed_url.is_some()
        || extracted.suggested_sources.iter().any(|s| s.contains("rss"))
    {
        return ClaimCategory::EventOccurrence;
    }
    if extracted.resolver_hints.url.is_some() {
        return ClaimCategory::CustomHttp;
    }
    ClaimCategory::Unsupported
}

pub fn claim_hash_hex(claim: &CanonicalClaim) -> String {
    hex::encode(claim.market_key_hash)
}

pub fn semantic_claim_hash_hex(claim: &CanonicalClaim) -> String {
    hex::encode(claim.semantic_claim_hash)
}

pub fn market_key_hash_hex(claim: &CanonicalClaim) -> String {
    hex::encode(claim.market_key_hash)
}

fn normalize_asset(s: &str) -> String {
    let lower = s.trim().to_lowercase();
    for (alias, canonical) in ASSET_ALIASES {
        if lower == *alias || lower.contains(alias) {
            return (*canonical).to_string();
        }
    }
    lower
}

fn normalize_repo_or_asset(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.contains('/') {
        let parts: Vec<&str> = trimmed.split('/').collect();
        if parts.len() >= 2 {
            return format!(
                "{}/{}",
                normalize_repo_slug(parts[0]),
                normalize_repo_slug(parts[1])
            );
        }
    }
    normalize_asset(trimmed)
}

fn normalize_repo_slug(s: &str) -> String {
    s.trim().to_lowercase()
}

fn normalize_url(s: &str) -> String {
    s.trim().to_string()
}

fn normalize_number(s: &str) -> String {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    if let Ok(v) = cleaned.parse::<f64>() {
        if v.fract() == 0.0 {
            return format!("{}", v as i64);
        }
        return format!("{v}");
    }
    cleaned
}

fn infer_comparison(predicate: &str) -> Option<ComparisonOp> {
    let p = predicate.to_lowercase();
    if p.contains("above") || p.contains("exceed") || p.contains("greater") {
        Some(ComparisonOp::Gt)
    } else if p.contains("below") || p.contains("under") || p.contains("less") {
        Some(ComparisonOp::Lt)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::store::reviews::ExtractedClaim;
    use crate::types::{ComparisonOp, ResolverHints};

    #[test]
    fn btc_alias_normalizes_to_bitcoin() {
        let extracted = ExtractedClaim {
            subject: "BTC".to_string(),
            predicate: "exceed".to_string(),
            object: "usd".to_string(),
            metric: Some("price".to_string()),
            comparison: Some(ComparisonOp::Gt),
            threshold: Some("100000".to_string()),
            deadline: None,
            outcome_type: OutcomeType::Binary,
            suggested_sources: vec![],
            suggested_options: vec!["Yes".to_string(), "No".to_string()],
            claim_category: ClaimCategory::PriceThreshold,
            time_class: crate::types::TimeClass::Future,
            resolver_hints: ResolverHints::default(),
        };
        let claim = canonicalize(Uuid::new_v4(), &extracted);
        assert_eq!(claim.normalized_fields.subject, "bitcoin");
        assert_eq!(
            claim.normalized_fields.claim_category,
            ClaimCategory::PriceThreshold
        );
    }

    #[test]
    fn price_claims_in_same_spacing_bucket_share_market_key() {
        let base = Utc::now() + chrono::Duration::minutes(10);
        let mk = |deadline: chrono::DateTime<Utc>| {
            let extracted = ExtractedClaim {
                subject: "bitcoin".to_string(),
                predicate: "price".to_string(),
                object: "usd".to_string(),
                metric: Some("price".to_string()),
                comparison: Some(ComparisonOp::Gt),
                threshold: Some("100".to_string()),
                deadline: Some(deadline),
                outcome_type: OutcomeType::Binary,
                suggested_sources: vec!["coingecko".to_string()],
                suggested_options: vec!["Yes".to_string(), "No".to_string()],
                claim_category: ClaimCategory::PriceThreshold,
                time_class: crate::types::TimeClass::Future,
                resolver_hints: ResolverHints::default(),
            };
            canonicalize(Uuid::new_v4(), &extracted).market_key_hash
        };
        let a = mk(base + chrono::Duration::minutes(3));
        let b = mk(base + chrono::Duration::minutes(8));
        assert_eq!(a, b);
    }
}
