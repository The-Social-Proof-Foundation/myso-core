// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

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
}

/// Canonical claim consumed by the Resolver Compiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalClaim {
    pub normalized_fields: CanonicalClaimFields,
    /// SHA-256 of semantic fields (permanent claim identity).
    pub semantic_claim_hash: [u8; 32],
    /// SHA-256 of semantic + deadline + betting_options (shared market key).
    pub market_key_hash: [u8; 32],
    /// Legacy alias for market_key_hash.
    pub claim_hash: [u8; 32],
    pub source_extraction_id: Uuid,
}

const ASSET_ALIASES: &[(&str, &str)] = &[
    ("btc", "bitcoin"),
    ("eth", "ethereum"),
    ("sol", "solana"),
];

pub fn canonicalize(extraction_id: Uuid, extracted: &ExtractedClaim) -> CanonicalClaim {
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

    let semantic = SemanticClaimFields {
        subject: subject.clone(),
        predicate: predicate.clone(),
        object: object.clone(),
        metric: metric.clone(),
        comparison,
        threshold: threshold.clone(),
        outcome_type: extracted.outcome_type,
        suggested_sources: suggested_sources.clone(),
        claim_category,
        resolver_hints: resolver_hints.clone(),
    };
    let semantic_claim_hash = hash_json(&semantic);

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
    };

    let market_key = MarketKeyFields {
        semantic,
        deadline: extracted.deadline,
        betting_options: suggested_options,
    };
    let market_key_hash = hash_json(&market_key);

    CanonicalClaim {
        normalized_fields: fields,
        semantic_claim_hash,
        market_key_hash,
        claim_hash: market_key_hash,
        source_extraction_id: extraction_id,
    }
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

fn hash_json<T: Serialize>(value: &T) -> [u8; 32] {
    let json = serde_json::to_vec(value).expect("canonical fields serialize");
    let mut hasher = Sha256::new();
    hasher.update(&json);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::reviews::ExtractedClaim;
    use crate::types::ComparisonOp;

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
            resolver_hints: ResolverHints::default(),
        };
        let claim = canonicalize(Uuid::new_v4(), &extracted);
        assert_eq!(claim.normalized_fields.subject, "bitcoin");
        assert_eq!(
            claim.normalized_fields.claim_category,
            ClaimCategory::PriceThreshold
        );
    }
}
