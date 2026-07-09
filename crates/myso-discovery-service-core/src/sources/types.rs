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

/// Corpus modality: binary media (PoC embed) vs text/structured (SPoT settlement).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ContentKind {
    Media,
    #[default]
    Text,
}

impl ContentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Media => "media",
            Self::Text => "text",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "media" => Some(Self::Media),
            "text" => Some(Self::Text),
            _ => None,
        }
    }
}

/// Map curated / adapter media_type strings to PoC embed codes (`image`/`audio`/`video`)
/// and a content kind. Non-media MIME stays as-is with `ContentKind::Text`.
pub fn normalize_media_type(raw: &str) -> (String, ContentKind) {
    let lower = raw.trim().to_ascii_lowercase();
    let base = lower.split(';').next().unwrap_or(&lower).trim();
    if matches!(base, "image" | "1") || base.starts_with("image/") {
        return ("image".to_string(), ContentKind::Media);
    }
    if matches!(base, "video" | "2") || base.starts_with("video/") {
        return ("video".to_string(), ContentKind::Media);
    }
    if matches!(base, "audio" | "3") || base.starts_with("audio/") {
        return ("audio".to_string(), ContentKind::Media);
    }
    (raw.trim().to_string(), ContentKind::Text)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub id: String,
    pub adapter_type: String,
    pub domain: DiscoveryDomain,
    pub trust_score: f64,
    pub enabled: bool,
    /// Optional override; when unset, adapters choose (factual→text, curated media→media).
    #[serde(default)]
    pub content_kind: Option<ContentKind>,
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
    pub content_kind: ContentKind,
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
    #[serde(default)]
    pub content_kind: ContentKind,
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
