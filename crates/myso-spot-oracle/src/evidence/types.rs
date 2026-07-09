// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Canonical evidence types for auditable settlement. Off-chain storage carries
//! payload + provenance; on-chain tx carries URLs only (contract requirement).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use myso_discovery_service_core::api::FetchProvenance;

use crate::sources::SourceEvidence;

/// Provenance metadata for a single evidence fetch.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceProvenance {
    pub source_id: String,
    pub source_url: String,
    pub content_hash: String,
    pub fetched_at: DateTime<Utc>,
    #[serde(default)]
    pub cache_hit: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_job_id: Option<String>,
}

impl From<FetchProvenance> for EvidenceProvenance {
    fn from(p: FetchProvenance) -> Self {
        Self {
            source_id: p.source_id,
            source_url: p.source_url,
            content_hash: p.content_hash,
            fetched_at: p.fetched_at,
            cache_hit: p.cache_hit,
            discovery_job_id: None,
        }
    }
}

/// Optional cryptographic signature for future signed oracle feeds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceSignature {
    pub scheme: String,
    pub public_key_id: String,
    pub signature_hex: String,
}

/// One auditable evidence record influencing a resolution decision.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceRecord {
    pub adapter_id: String,
    pub provenance: EvidenceProvenance,
    pub payload: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_response: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<EvidenceSignature>,
}

impl EvidenceRecord {
    pub fn from_source_evidence(ev: &SourceEvidence) -> Self {
        Self {
            adapter_id: ev.adapter_id.clone(),
            provenance: EvidenceProvenance {
                source_id: ev.adapter_id.clone(),
                source_url: ev.source_url.clone(),
                content_hash: ev.content_hash.clone(),
                fetched_at: ev.fetched_at,
                cache_hit: false,
                discovery_job_id: None,
            },
            payload: ev.payload.clone(),
            raw_response: ev.raw_response.clone(),
            signature: None,
        }
    }
}

/// Deterministic bundle of evidence for one resolve attempt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceBundle {
    pub market_id: Uuid,
    pub resolver_job_id: Uuid,
    pub records: Vec<EvidenceRecord>,
    pub bundle_hash: String,
}

impl EvidenceBundle {
    pub fn build(
        market_id: Uuid,
        resolver_job_id: Uuid,
        source_evidence: &[SourceEvidence],
    ) -> Self {
        let mut records: Vec<EvidenceRecord> = source_evidence
            .iter()
            .map(EvidenceRecord::from_source_evidence)
            .collect();
        records.sort_by(|a, b| a.adapter_id.cmp(&b.adapter_id));
        let bundle_hash = compute_bundle_hash(&records);
        Self {
            market_id,
            resolver_job_id,
            records,
            bundle_hash,
        }
    }

    pub fn evidence_urls(&self) -> Vec<String> {
        self.records
            .iter()
            .map(|r| r.provenance.source_url.clone())
            .collect()
    }
}

pub fn compute_bundle_hash(records: &[EvidenceRecord]) -> String {
    let json = serde_json::to_vec(records).expect("evidence records serialize");
    let mut hasher = Sha256::new();
    hasher.update(&json);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn bundle_hash_is_stable_for_same_records() {
        let record = EvidenceRecord {
            adapter_id: "coingecko".to_string(),
            provenance: EvidenceProvenance {
                source_id: "coingecko".to_string(),
                source_url: "https://api.coingecko.com".to_string(),
                content_hash: "abc".to_string(),
                fetched_at: Utc::now(),
                cache_hit: false,
                discovery_job_id: None,
            },
            payload: serde_json::json!({"bitcoin": {"usd": 1.0}}),
            raw_response: None,
            signature: None,
        };
        let h1 = compute_bundle_hash(&[record.clone()]);
        let h2 = compute_bundle_hash(&[record]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn fetch_provenance_roundtrip() {
        let fp = FetchProvenance {
            source_id: "coingecko".to_string(),
            source_url: "https://api.coingecko.com".to_string(),
            content_hash: "deadbeef".to_string(),
            fetched_at: Utc::now(),
            cache_hit: true,
        };
        let ep: EvidenceProvenance = fp.clone().into();
        assert_eq!(ep.source_id, fp.source_id);
        assert_eq!(ep.content_hash, fp.content_hash);
        assert!(ep.cache_hit);
    }
}
