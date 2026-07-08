// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Context};
use std::path::Path;

use crate::sources::types::SourceConfig;

/// Load source registration from a YAML file. The YAML declares **where to fetch**
/// (feed URLs, GitHub repo slugs, API bases) plus enable flags + trust scores — it
/// is registration + parameters, not hand-authored discovery results.
///
/// Fails fast with a clear error if the file is missing or empty, instead of
/// silently injecting a no-op `manual_curated` stub.
pub fn load_sources_config(path: &Path) -> anyhow::Result<Vec<SourceConfig>> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("reading sources config {}", path.display()))?;
    if raw.trim().is_empty() {
        return Err(anyhow!(
            "sources config {} is empty — refusing to start with no real sources registered",
            path.display()
        ));
    }
    let configs: Vec<SourceConfig> = serde_yaml::from_str(&raw)
        .with_context(|| format!("parsing sources config {}", path.display()))?;
    if configs.is_empty() {
        return Err(anyhow!(
            "sources config {} declares zero sources — refusing to start with no real sources",
            path.display()
        ));
    }
    Ok(configs)
}
