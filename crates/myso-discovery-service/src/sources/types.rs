// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryDomain {
    Creative,
    Factual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub id: String,
    pub adapter_type: String,
    pub domain: DiscoveryDomain,
    pub trust_score: f64,
    pub enabled: bool,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub entries: Vec<CuratedEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuratedEntry {
    pub url: String,
    pub media_type: String,
    #[serde(default)]
    pub creator_x_handle: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub trust_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryAssetRecord {
    pub external_source_url: String,
    pub media_type: String,
    pub canonical_metadata: serde_json::Value,
    pub source_trust_score: f64,
    pub creator_confidence: f64,
    pub creator_x_handle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawDiscoveryRecord {
    pub external_source_url: String,
    pub media_type: String,
    pub title: Option<String>,
    pub creator_x_handle: Option<String>,
    pub trust_score: f64,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMetadata {
    pub id: String,
    pub description: String,
    pub domain: DiscoveryDomain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceHealth {
    pub healthy: bool,
    pub message: String,
}
