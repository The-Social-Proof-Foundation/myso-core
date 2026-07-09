// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Shared HTTP API types and client for the Discovery factual query surface.
//! Used by `myso-discovery-service` (server) and SPoT / other consumers.

mod http;

pub use http::HttpDiscoveryClient;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Provenance metadata attached to every normalized factual response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FetchProvenance {
    pub source_id: String,
    pub source_url: String,
    pub content_hash: String,
    pub fetched_at: DateTime<Utc>,
    pub cache_hit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NormalizedPrice {
    pub asset: String,
    pub quote: String,
    pub price: f64,
    pub observed_at: DateTime<Utc>,
    pub provenance: FetchProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NormalizedRelease {
    pub owner: String,
    pub repo: String,
    pub tag: String,
    pub published_at: Option<DateTime<Utc>>,
    pub provenance: FetchProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NormalizedEvent {
    pub title: String,
    pub url: String,
    pub published_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub provenance: FetchProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceSummary {
    pub id: String,
    pub adapter_type: String,
    pub domain: String,
    pub trust_score: f64,
    pub enabled: bool,
    pub content_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_healthy: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceHealthResponse {
    pub source_id: String,
    pub healthy: bool,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceQuery {
    pub asset: String,
    pub quote: String,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub refresh: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseQuery {
    pub owner: String,
    pub repo: String,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub refresh: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventsQuery {
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub feed: Option<String>,
    #[serde(default)]
    pub since: Option<DateTime<Utc>>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub refresh: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RefreshRequest {
    pub source_id: String,
    #[serde(default)]
    pub kind: Option<String>,
}

/// Typed operations for the Discovery factual HTTP API.
/// Implemented by [`HttpDiscoveryClient`]; consumed by SPoT `TrustedSource` adapters.
#[async_trait]
pub trait DiscoveryClient: Send + Sync {
    async fn list_sources(&self) -> anyhow::Result<Vec<SourceSummary>>;
    async fn source_health(&self, source_id: &str) -> anyhow::Result<SourceHealthResponse>;
    async fn all_sources_health(&self) -> anyhow::Result<Vec<SourceHealthResponse>>;
    async fn get_price(&self, query: &PriceQuery) -> anyhow::Result<NormalizedPrice>;
    async fn get_release(&self, query: &ReleaseQuery) -> anyhow::Result<NormalizedRelease>;
    async fn get_events(&self, query: &EventsQuery) -> anyhow::Result<Vec<NormalizedEvent>>;
    async fn refresh_source(&self, request: &RefreshRequest) -> anyhow::Result<serde_json::Value>;
}
