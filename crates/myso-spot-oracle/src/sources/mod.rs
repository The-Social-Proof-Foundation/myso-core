// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! `TrustedSource` — the deterministic settlement contract. Produces auditable
//! evidence (URL + content hash + optional raw body) that can settle a market at
//! maturity. Sources are fetched directly by SPoT at resolution time.

pub mod adapters;
pub mod direct_fetch;
pub mod http_fetch;
pub mod rate_limit;
pub mod source_config;

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::resolver::ResolverDefinition;
use crate::sources::source_config::{SourceDomain, SourceHealth, SourceMetadata};
use crate::store::SpotTrustedSourceRow;

pub use source_config::{load_sources_config, SourceConfig};

/// Auditable evidence produced by a `TrustedSource::resolve` call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceEvidence {
    pub adapter_id: String,
    pub source_url: String,
    /// SHA-256 hex of the fetched response body.
    pub content_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_response: Option<String>,
    pub fetched_at: DateTime<Utc>,
    /// Parsed snapshot the resolver engine evaluates against the `ResolverSpec`.
    pub payload: serde_json::Value,
}

/// Deterministic settlement contract — produce evidence that can settle a market.
#[async_trait]
pub trait TrustedSource: Send + Sync {
    fn id(&self) -> &str;
    fn domain(&self) -> SourceDomain;
    fn supports(&self, def: &ResolverDefinition) -> bool;
    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence>;
    async fn health(&self) -> SourceHealth;
    fn metadata(&self) -> SourceMetadata;
}

/// Holds `TrustedSource` impls present in the binary, keyed by adapter id.
#[derive(Clone, Default)]
pub struct ResolverRegistry {
    by_id: HashMap<String, Arc<dyn TrustedSource>>,
}

impl ResolverRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, src: Arc<dyn TrustedSource>) {
        self.by_id.insert(src.id().to_string(), src);
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn TrustedSource>> {
        self.by_id.get(id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<dyn TrustedSource>> {
        self.by_id.values().cloned().collect()
    }

    pub fn supports(&self, def: &ResolverDefinition) -> Vec<Arc<dyn TrustedSource>> {
        self.by_id
            .values()
            .filter(|s| s.supports(def))
            .cloned()
            .collect()
    }

    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}

/// Build the default registry from every `TrustedSource` impl compiled into the binary.
pub fn build_default_registry() -> ResolverRegistry {
    let mut reg = ResolverRegistry::new();
    for src in adapters::all_default_sources() {
        reg.register(src);
    }
    reg
}

fn trusted_ids_for_row(row: &SpotTrustedSourceRow) -> Vec<String> {
    match row.adapter_type.as_str() {
        "rss" => vec!["rss_event".to_string()],
        "github_releases" => vec!["github_releases".to_string()],
        "http_official" => {
            let base = row
                .config
                .get("api_base_url")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if base.contains("coingecko") {
                vec!["coingecko".to_string(), "http_official".to_string()]
            } else if base.contains("coinbase") {
                vec!["coinbase".to_string(), "http_official".to_string()]
            } else {
                vec!["http_official".to_string()]
            }
        }
        "coingecko" => vec!["coingecko".to_string()],
        "coinbase" => vec!["coinbase".to_string()],
        "chainlink" => vec!["chainlink".to_string()],
        other => vec![other.to_string()],
    }
}

/// Restrict the default registry to `TrustedSource` impls referenced by enabled rows.
pub fn build_registry_from_sources(
    default: &ResolverRegistry,
    rows: &[SpotTrustedSourceRow],
) -> ResolverRegistry {
    let mut reg = ResolverRegistry::new();
    for row in rows.iter().filter(|r| r.enabled) {
        for trusted_id in trusted_ids_for_row(row) {
            if let Some(src) = default.get(&trusted_id) {
                reg.register(src);
            }
        }
    }
    if !rows.iter().any(|r| r.enabled) {
        return reg;
    }
    for id in [
        "coingecko",
        "coinbase",
        "chainlink",
        "github_releases",
        "http_official",
        "rss_event",
        "wikipedia",
    ] {
        if let Some(src) = default.get(id) {
            if reg.get(id).is_none() {
                reg.register(src);
            }
        }
    }
    reg
}
