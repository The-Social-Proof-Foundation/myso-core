// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Stable outcome identity keyed on graph/registry refs — not volatile LLM strings or resolver hints.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::review::canonicalize::OutcomeType;
use crate::review::deadline::bucket_deadline_for_market_key;
use crate::types::{ClaimCategory, ComparisonOp};

/// Graph-ready identity fields used for semantic and market hashing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutcomeIdentityFields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub competition_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_ref: Option<String>,
    pub predicate: String,
    pub object: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comparison: Option<ComparisonOp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
    pub outcome_type: OutcomeType,
    pub claim_category: ClaimCategory,
    #[serde(default)]
    pub suggested_sources: Vec<String>,
}

/// Time-bounded market identity (outcome identity + deadline day + betting options).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutcomeMarketKey {
    pub identity: OutcomeIdentityFields,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline_day: Option<String>,
    pub betting_options: Vec<String>,
}

pub fn deadline_day_bucket(deadline: DateTime<Utc>) -> String {
    deadline.date_naive().format("%Y-%m-%d").to_string()
}

pub fn build_outcome_identity(
    entity_ref: Option<String>,
    competition_ref: Option<String>,
    event_ref: Option<String>,
    metric_ref: Option<String>,
    predicate: String,
    object: String,
    metric: Option<String>,
    comparison: Option<ComparisonOp>,
    threshold: Option<String>,
    outcome_type: OutcomeType,
    claim_category: ClaimCategory,
    suggested_sources: Vec<String>,
) -> OutcomeIdentityFields {
    let mut sources = suggested_sources;
    sources.sort();
    sources.dedup();
    OutcomeIdentityFields {
        entity_ref,
        competition_ref,
        event_ref,
        metric_ref,
        predicate,
        object,
        metric,
        comparison,
        threshold,
        outcome_type,
        claim_category,
        suggested_sources: sources,
    }
}

pub fn build_outcome_market_key(
    identity: OutcomeIdentityFields,
    deadline: Option<DateTime<Utc>>,
    betting_options: Vec<String>,
    claim_category: ClaimCategory,
    price_market_spacing: chrono::Duration,
) -> OutcomeMarketKey {
    let market_deadline = match claim_category {
        ClaimCategory::PriceThreshold => deadline
            .map(|d| bucket_deadline_for_market_key(d, price_market_spacing)),
        _ => deadline,
    };
    let deadline_day = market_deadline.map(deadline_day_bucket);
    let mut options = betting_options;
    options.sort();
    options.dedup();
    OutcomeMarketKey {
        identity,
        deadline_day,
        betting_options: options,
    }
}

pub fn outcome_identity_hash(identity: &OutcomeIdentityFields) -> [u8; 32] {
    hash_json(identity)
}

pub fn outcome_market_hash(market_key: &OutcomeMarketKey) -> [u8; 32] {
    hash_json(market_key)
}

pub fn outcome_identity_hash_hex(identity: &OutcomeIdentityFields) -> String {
    hex::encode(outcome_identity_hash(identity))
}

pub fn outcome_market_hash_hex(market_key: &OutcomeMarketKey) -> String {
    hex::encode(outcome_market_hash(market_key))
}

fn hash_json<T: Serialize>(value: &T) -> [u8; 32] {
    let json = serde_json::to_vec(value).expect("outcome identity serializes");
    let mut hasher = Sha256::new();
    hasher.update(&json);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::review::canonicalize::OutcomeType;
    use crate::review::deadline::DEFAULT_PRICE_MARKET_SPACING;
    use crate::types::ClaimCategory;

    #[test]
    fn same_entity_event_deadline_share_market_hash() {
        let identity = build_outcome_identity(
            Some("jd_vance".to_string()),
            None,
            Some("us_presidential_election_2028".to_string()),
            None,
            "win".to_string(),
            "election".to_string(),
            None,
            None,
            None,
            OutcomeType::Binary,
            ClaimCategory::EventOccurrence,
            vec!["wikipedia".to_string()],
        );
        let deadline = DateTime::parse_from_rfc3339("2028-11-07T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let key_a = build_outcome_market_key(
            identity.clone(),
            Some(deadline),
            vec!["Yes".to_string(), "No".to_string()],
            ClaimCategory::EventOccurrence,
            DEFAULT_PRICE_MARKET_SPACING,
        );
        let key_b = build_outcome_market_key(
            identity,
            Some(deadline + chrono::Duration::hours(3)),
            vec!["Yes".to_string(), "No".to_string()],
            ClaimCategory::EventOccurrence,
            DEFAULT_PRICE_MARKET_SPACING,
        );
        assert_eq!(outcome_market_hash(&key_a), outcome_market_hash(&key_b));
    }

    #[test]
    fn vance_and_rubio_differ_by_entity_ref() {
        let vance = build_outcome_identity(
            Some("jd_vance".to_string()),
            None,
            Some("us_presidential_election_2028".to_string()),
            None,
            "win".to_string(),
            "election".to_string(),
            None,
            None,
            None,
            OutcomeType::Binary,
            ClaimCategory::EventOccurrence,
            vec![],
        );
        let rubio = build_outcome_identity(
            Some("marco_rubio".to_string()),
            None,
            Some("us_presidential_election_2028".to_string()),
            None,
            "win".to_string(),
            "election".to_string(),
            None,
            None,
            None,
            OutcomeType::Binary,
            ClaimCategory::EventOccurrence,
            vec![],
        );
        assert_ne!(outcome_identity_hash(&vance), outcome_identity_hash(&rubio));
    }
}
