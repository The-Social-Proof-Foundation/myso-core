// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! YAML registration for event providers (parallel to trusted sources).

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventProviderConfig {
    pub id: String,
    pub provider_type: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
    #[serde(default)]
    pub config: serde_json::Value,
}

fn default_enabled() -> bool {
    true
}

fn default_poll_interval() -> u64 {
    3600
}

/// Load event provider registration from YAML.
pub fn load_event_providers_config(path: &Path) -> anyhow::Result<Vec<EventProviderConfig>> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("reading event providers config {}", path.display()))?;
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }
    let configs: Vec<EventProviderConfig> = serde_yaml::from_str(&raw)
        .with_context(|| format!("parsing event providers config {}", path.display()))?;
    for cfg in &configs {
        if cfg.id.trim().is_empty() {
            return Err(anyhow!("event provider config has empty id"));
        }
        if cfg.provider_type.trim().is_empty() {
            return Err(anyhow!(
                "event provider {} missing provider_type",
                cfg.id
            ));
        }
    }
    Ok(configs)
}
