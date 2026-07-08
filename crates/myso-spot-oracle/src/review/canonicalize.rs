// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::store::reviews::ExtractedClaim;
use crate::types::ComparisonOp;

/// How a market resolves at maturity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeType {
    Binary,
    MultiChoice,
    Scalar,
}

/// Stable, normalized claim fields used for duplicate detection and compilation.
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
}

/// Canonical claim consumed by the Resolver Compiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalClaim {
    pub normalized_fields: CanonicalClaimFields,
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
    let object = normalize_asset(&extracted.object);
    let threshold = extracted.threshold.as_ref().map(|t| normalize_number(t));
    let comparison = extracted.comparison.or_else(|| infer_comparison(&extracted.predicate));
    let mut suggested_sources = extracted.suggested_sources.clone();
    suggested_sources.sort();
    suggested_sources.dedup();
    let mut suggested_options = extracted.suggested_options.clone();
    suggested_options.sort();
    suggested_options.dedup();

    let fields = CanonicalClaimFields {
        subject,
        predicate: extracted.predicate.trim().to_lowercase(),
        object,
        metric: extracted.metric.as_ref().map(|s| s.trim().to_lowercase()),
        comparison,
        threshold,
        deadline: extracted.deadline,
        outcome_type: extracted.outcome_type,
        suggested_sources,
        suggested_options,
    };
    let claim_hash = hash_fields(&fields);
    CanonicalClaim {
        normalized_fields: fields,
        claim_hash,
        source_extraction_id: extraction_id,
    }
}

pub fn claim_hash_hex(claim: &CanonicalClaim) -> String {
    hex::encode(claim.claim_hash)
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

fn hash_fields(fields: &CanonicalClaimFields) -> [u8; 32] {
    let json = serde_json::to_vec(fields).expect("canonical fields serialize");
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
        };
        let claim = canonicalize(Uuid::new_v4(), &extracted);
        assert_eq!(claim.normalized_fields.subject, "bitcoin");
    }
}
