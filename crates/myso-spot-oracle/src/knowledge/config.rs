// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, serde::Deserialize)]
struct KnowledgeDomainsFile {
    domains: HashMap<String, KnowledgeDomainEntry>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct KnowledgeDomainEntry {
    preferred_sources: Vec<String>,
}

pub fn load_knowledge_domains(
    path: impl AsRef<Path>,
) -> anyhow::Result<HashMap<String, Vec<String>>> {
    let raw = std::fs::read_to_string(path)?;
    let parsed: KnowledgeDomainsFile = serde_yaml::from_str(&raw)?;
    Ok(parsed
        .domains
        .into_iter()
        .map(|(k, v)| (k, v.preferred_sources))
        .collect())
}

pub fn preferred_sources_for_domain(
    domains: &HashMap<String, Vec<String>>,
    domain: &str,
) -> Vec<String> {
    domains.get(domain).cloned().unwrap_or_default()
}
