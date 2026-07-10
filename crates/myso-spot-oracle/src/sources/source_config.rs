// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! SPoT-owned trusted-source YAML registration (copied from discovery-core types).

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceDomain {
    Creative,
    Factual,
}

impl SourceDomain {
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
    pub domain: SourceDomain,
    pub trust_score: f64,
    pub enabled: bool,
    #[serde(default)]
    pub config: SourceFetchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceFetchConfig {
    #[serde(default)]
    pub feed_urls: Vec<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub repo: Option<String>,
    #[serde(default)]
    pub api_base_url: Option<String>,
    #[serde(default)]
    pub poll_path: Option<String>,
    #[serde(default)]
    pub api_key_env: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMetadata {
    pub id: String,
    pub description: String,
    pub domain: SourceDomain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceHealth {
    pub healthy: bool,
    pub message: String,
}

/// Load trusted-source registration from YAML. Declares where to fetch + trust scores.
pub fn load_sources_config(path: &Path) -> anyhow::Result<Vec<SourceConfig>> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("reading sources config {}", path.display()))?;
    if raw.trim().is_empty() {
        return Err(anyhow!(
            "sources config {} is empty — refusing to start with no trusted sources",
            path.display()
        ));
    }
    let configs: Vec<SourceConfig> = serde_yaml::from_str(&raw)
        .with_context(|| format!("parsing sources config {}", path.display()))?;
    if configs.is_empty() {
        return Err(anyhow!(
            "sources config {} declares zero sources — refusing to start",
            path.display()
        ));
    }
    Ok(configs)
}
