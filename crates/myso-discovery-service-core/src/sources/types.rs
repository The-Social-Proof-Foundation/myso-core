// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryDomain {
    Creative,
    Factual,
}

impl DiscoveryDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Creative => "creative",
            Self::Factual => "factual",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub id: String,
    pub adapter_type: String,
    pub domain: DiscoveryDomain,
    pub trust_score: f64,
    pub enabled: bool,
    /// Filesystem path for `manual_curated` entry lists (test/seed only).
    #[serde(default)]
    pub path: Option<String>,
    /// Hand-curated entries (test/seed only; never the primary data path for factual sources).
    #[serde(default)]
    pub entries: Vec<CuratedEntry>,
    /// Real fetch parameters — where to fetch, not hand-authored results.
    #[serde(default)]
    pub config: SourceFetchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceFetchConfig {
    /// RSS adapter: feed URLs to poll.
    #[serde(default)]
    pub feed_urls: Vec<String>,
    /// GitHub releases adapter: `owner`/`repo`.
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub repo: Option<String>,
    /// HTTP official adapter: `{api_base_url}{poll_path}`.
    #[serde(default)]
    pub api_base_url: Option<String>,
    #[serde(default)]
    pub poll_path: Option<String>,
    /// Optional bearer/api key read from this env var at fetch time.
    #[serde(default)]
    pub api_key_env: Option<String>,
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
    /// SHA-256 hex of the fetched response body (None for non-fetched sources like manual_curated).
    #[serde(default)]
    pub content_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawDiscoveryRecord {
    pub external_source_url: String,
    pub media_type: String,
    pub title: Option<String>,
    pub creator_x_handle: Option<String>,
    pub trust_score: f64,
    /// SHA-256 hex of the fetched response body, computed by the adapter.
    #[serde(default)]
    pub content_hash: Option<String>,
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
