// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::sources::DiscoveryAssetRecord;

#[derive(Debug, Clone)]
pub struct PriorityWeights {
    pub popularity: f64,
    pub trending: f64,
    pub recency: f64,
    pub reupload_likelihood: f64,
    pub creator_importance: f64,
    pub work_cluster_size: f64,
    pub blockchain_relevance: f64,
    pub dispute_relevance: f64,
    pub source_trust: f64,
    pub prior_hit_count: f64,
}

impl Default for PriorityWeights {
    fn default() -> Self {
        Self {
            popularity: 0.05,
            trending: 0.05,
            recency: 0.10,
            reupload_likelihood: 0.10,
            creator_importance: 0.15,
            work_cluster_size: 0.10,
            blockchain_relevance: 0.20,
            dispute_relevance: 0.05,
            source_trust: 0.15,
            prior_hit_count: 0.05,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PrioritySignals {
    pub popularity: f64,
    pub trending: f64,
    pub recency: f64,
    pub reupload_likelihood: f64,
    pub creator_importance: f64,
    pub work_cluster_size: f64,
    pub blockchain_relevance: f64,
    pub dispute_relevance: f64,
    pub source_trust: f64,
    pub prior_hit_count: f64,
}

pub fn score_priority(signals: &PrioritySignals, weights: &PriorityWeights) -> i64 {
    let raw = signals.popularity * weights.popularity
        + signals.trending * weights.trending
        + signals.recency * weights.recency
        + signals.reupload_likelihood * weights.reupload_likelihood
        + signals.creator_importance * weights.creator_importance
        + signals.work_cluster_size * weights.work_cluster_size
        + signals.blockchain_relevance * weights.blockchain_relevance
        + signals.dispute_relevance * weights.dispute_relevance
        + signals.source_trust * weights.source_trust
        + signals.prior_hit_count * weights.prior_hit_count;
    (raw * 1_000_000.0) as i64
}

pub fn signals_for_asset(record: &DiscoveryAssetRecord, has_chain_ref: bool) -> PrioritySignals {
    PrioritySignals {
        source_trust: record.source_trust_score,
        creator_importance: record.creator_confidence,
        blockchain_relevance: if has_chain_ref { 1.0 } else { 0.0 },
        recency: 1.0,
        ..Default::default()
    }
}
