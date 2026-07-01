// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::openrouter_client::OpenRouterModelRate;
use crate::pricing::CATALOG_USD_PEG;

pub const MIST_PER_MYSO: u64 = 1_000_000_000;
pub const CAP_AI_SPEND: u64 = 16384;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogFile {
    pub catalog: CatalogMeta,
    #[serde(default)]
    pub models: Vec<ModelRates>,
    #[serde(default)]
    pub embeddings: Vec<EmbeddingRates>,
    #[serde(default)]
    pub tools: Vec<ToolRates>,
    #[serde(default)]
    pub defaults: CatalogDefaults,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogMeta {
    pub version: String,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub effective_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelRates {
    pub aliases: Vec<String>,
    pub display_name: String,
    pub input_mist_per_1m: u64,
    pub output_mist_per_1m: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingRates {
    pub aliases: Vec<String>,
    pub display_name: String,
    pub mist_per_1m: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolRates {
    pub id: String,
    pub flat_mist: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CatalogDefaults {
    pub unknown_model_input_mist_per_1m: u64,
    pub unknown_model_output_mist_per_1m: u64,
    pub min_charge_mist: u64,
}

#[derive(Debug, Clone)]
pub struct PricingCatalog {
    pub version: String,
    pub source: Option<String>,
    pub effective_date: Option<String>,
    pub models: HashMap<String, ModelRates>,
    pub embeddings: HashMap<String, EmbeddingRates>,
    pub tools: HashMap<String, ToolRates>,
    pub defaults: CatalogDefaults,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CatalogSyncReport {
    pub checked: usize,
    pub updated: usize,
    pub skipped_drift: usize,
    pub unchanged: usize,
}

pub fn usd_per_1m_to_mist(usd_per_1m: f64, catalog_usd_peg: f64) -> u64 {
    if catalog_usd_peg <= 0.0 {
        return 0;
    }
    (usd_per_1m * (MIST_PER_MYSO as f64 / catalog_usd_peg)).round() as u64
}

fn relative_drift_pct(old: u64, new: u64) -> f64 {
    if old == 0 {
        return if new == 0 { 0.0 } else { 100.0 };
    }
    ((new as f64 - old as f64).abs() / old as f64) * 100.0
}

impl PricingCatalog {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path).with_context(|| format!("read catalog {}", path.display()))?;
        let file: CatalogFile = toml::from_str(&text).context("parse pricing catalog")?;
        Ok(Self::from_file(file))
    }

    pub fn from_file(file: CatalogFile) -> Self {
        let mut models = HashMap::new();
        for model in file.models {
            for alias in &model.aliases {
                models.insert(alias.to_lowercase(), model.clone());
            }
        }
        let mut embeddings = HashMap::new();
        for embed in file.embeddings {
            for alias in &embed.aliases {
                embeddings.insert(alias.to_lowercase(), embed.clone());
            }
        }
        let mut tools = HashMap::new();
        for tool in file.tools {
            tools.insert(tool.id.clone(), tool);
        }
        Self {
            version: file.catalog.version,
            source: file.catalog.source,
            effective_date: file.catalog.effective_date,
            models,
            embeddings,
            tools,
            defaults: file.defaults,
        }
    }

    pub fn is_known_inference_model(&self, model_id: &str) -> bool {
        self.models.contains_key(&model_id.to_lowercase())
    }

    pub fn is_known_embedding_model(&self, model_id: &str) -> bool {
        self.embeddings.contains_key(&model_id.to_lowercase())
    }

    pub fn is_known_tool(&self, tool_id: &str) -> bool {
        self.tools.contains_key(tool_id)
    }

    /// Equal-weight average input/output MIST per 1M tokens across catalog models (UI reference only).
    pub fn reference_mist_per_1m(&self) -> ReferenceRates {
        let mut models: Vec<&ModelRates> = Vec::new();
        let mut seen = HashSet::new();
        for model in self.models.values() {
            if seen.insert(model.display_name.clone()) {
                models.push(model);
            }
        }
        if models.is_empty() {
            return ReferenceRates {
                input_mist_per_1m: self.defaults.unknown_model_input_mist_per_1m,
                output_mist_per_1m: self.defaults.unknown_model_output_mist_per_1m,
                model_count: 0,
            };
        }
        let n = models.len() as u128;
        let input_sum = models.iter().map(|m| m.input_mist_per_1m as u128).sum::<u128>();
        let output_sum = models.iter().map(|m| m.output_mist_per_1m as u128).sum::<u128>();
        ReferenceRates {
            input_mist_per_1m: (input_sum / n) as u64,
            output_mist_per_1m: (output_sum / n) as u64,
            model_count: models.len(),
        }
    }

    pub fn model_rates(&self, model_id: &str) -> ModelRates {
        let key = model_id.to_lowercase();
        if let Some(r) = self.models.get(&key) {
            return r.clone();
        }
        for (alias, rates) in &self.models {
            if key.contains(alias) || alias.contains(&key) {
                return rates.clone();
            }
        }
        ModelRates {
            aliases: vec![model_id.to_string()],
            display_name: "unknown".to_string(),
            input_mist_per_1m: self.defaults.unknown_model_input_mist_per_1m,
            output_mist_per_1m: self.defaults.unknown_model_output_mist_per_1m,
        }
    }

    pub fn embedding_mist_per_1m(&self, model_id: &str) -> u64 {
        let key = model_id.to_lowercase();
        self.embeddings
            .get(&key)
            .map(|e| e.mist_per_1m)
            .unwrap_or(20_000_000)
    }

    pub fn tool_flat_mist(&self, tool_id: &str) -> u64 {
        self.tools
            .get(tool_id)
            .map(|t| t.flat_mist)
            .unwrap_or(10_000_000)
    }

    fn unique_models(&self) -> Vec<ModelRates> {
        let mut models: Vec<ModelRates> = Vec::new();
        let mut seen = HashSet::new();
        for model in self.models.values() {
            if seen.insert(model.display_name.clone()) {
                models.push(model.clone());
            }
        }
        models.sort_by(|a, b| a.display_name.cmp(&b.display_name));
        models
    }

    fn unique_embeddings(&self) -> Vec<EmbeddingRates> {
        let mut embeddings: Vec<EmbeddingRates> = Vec::new();
        let mut seen = HashSet::new();
        for embed in self.embeddings.values() {
            if seen.insert(embed.display_name.clone()) {
                embeddings.push(embed.clone());
            }
        }
        embeddings.sort_by(|a, b| a.display_name.cmp(&b.display_name));
        embeddings
    }

    pub fn to_catalog_file(&self) -> CatalogFile {
        let mut tools: Vec<ToolRates> = self.tools.values().cloned().collect();
        tools.sort_by(|a, b| a.id.cmp(&b.id));
        CatalogFile {
            catalog: CatalogMeta {
                version: self.version.clone(),
                source: self.source.clone(),
                effective_date: self.effective_date.clone(),
            },
            models: self.unique_models(),
            embeddings: self.unique_embeddings(),
            tools,
            defaults: self.defaults.clone(),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let file = self.to_catalog_file();
        let text = toml::to_string_pretty(&file).context("serialize pricing catalog")?;
        let tmp_path = PathBuf::from(format!("{}.tmp", path.display()));
        std::fs::write(&tmp_path, text.as_bytes())
            .with_context(|| format!("write catalog tmp {}", tmp_path.display()))?;
        std::fs::rename(&tmp_path, path)
            .with_context(|| format!("rename catalog to {}", path.display()))?;
        Ok(())
    }

    pub fn apply_openrouter_rates(
        &mut self,
        remote: &HashMap<String, OpenRouterModelRate>,
        max_drift_pct: f64,
    ) -> CatalogSyncReport {
        let mut report = CatalogSyncReport::default();
        let mut updated_models: Vec<ModelRates> = self.unique_models();
        let mut any_changed = false;

        for model in &mut updated_models {
            let Some(remote_rate) = model
                .aliases
                .iter()
                .find_map(|alias| remote.get(&alias.to_lowercase()))
            else {
                continue;
            };

            report.checked += 1;
            let new_input = usd_per_1m_to_mist(remote_rate.input_usd_per_1m, CATALOG_USD_PEG);
            let new_output = usd_per_1m_to_mist(remote_rate.output_usd_per_1m, CATALOG_USD_PEG);

            if model.input_mist_per_1m == new_input && model.output_mist_per_1m == new_output {
                report.unchanged += 1;
                continue;
            }

            let input_drift = relative_drift_pct(model.input_mist_per_1m, new_input);
            let output_drift = relative_drift_pct(model.output_mist_per_1m, new_output);
            if input_drift > max_drift_pct || output_drift > max_drift_pct {
                tracing::warn!(
                    model = %model.display_name,
                    alias = %remote_rate.id,
                    input_drift_pct = input_drift,
                    output_drift_pct = output_drift,
                    max_drift_pct,
                    "skipping catalog rate update due to drift cap"
                );
                report.skipped_drift += 1;
                continue;
            }

            model.input_mist_per_1m = new_input;
            model.output_mist_per_1m = new_output;
            report.updated += 1;
            any_changed = true;
        }

        if any_changed {
            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
            self.version = today.clone();
            self.effective_date = Some(today);
            if self.source.is_none() {
                self.source = Some("openrouter+manual".to_string());
            }

            self.models.clear();
            for model in updated_models {
                for alias in &model.aliases {
                    self.models.insert(alias.to_lowercase(), model.clone());
                }
            }
        }

        report
    }

    pub fn to_response(&self) -> CatalogResponse {
        self.to_response_with_fx(crate::pricing::CATALOG_USD_PEG, 1.0, String::new(), None, true)
    }

    pub fn to_response_with_fx(
        &self,
        catalog_usd_peg: f64,
        myso_usd: f64,
        price_oracle_url: String,
        price_age_secs: Option<u64>,
        price_stale: bool,
    ) -> CatalogResponse {
        let mut models: Vec<ModelRates> = Vec::new();
        let mut seen_models = HashSet::new();
        for model in self.models.values() {
            if seen_models.insert(model.display_name.clone()) {
                models.push(model.clone());
            }
        }
        let mut embeddings: Vec<EmbeddingRates> = Vec::new();
        let mut seen_embed = HashSet::new();
        for embed in self.embeddings.values() {
            if seen_embed.insert(embed.display_name.clone()) {
                embeddings.push(embed.clone());
            }
        }
        let tools: Vec<ToolRates> = self.tools.values().cloned().collect();
        let reference = self.reference_mist_per_1m();
        CatalogResponse {
            version: self.version.clone(),
            source: self.source.clone(),
            effective_date: self.effective_date.clone(),
            models,
            embeddings,
            tools,
            min_charge_mist: self.defaults.min_charge_mist,
            reference_mist_per_1m_in: reference.input_mist_per_1m,
            reference_mist_per_1m_out: reference.output_mist_per_1m,
            reference_model_count: reference.model_count,
            mist_per_myso: MIST_PER_MYSO,
            catalog_usd_peg,
            myso_usd,
            price_oracle_url,
            price_age_secs,
            price_stale,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReferenceRates {
    pub input_mist_per_1m: u64,
    pub output_mist_per_1m: u64,
    pub model_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct CatalogResponse {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    pub models: Vec<ModelRates>,
    pub embeddings: Vec<EmbeddingRates>,
    pub tools: Vec<ToolRates>,
    pub min_charge_mist: u64,
    /// Equal-weight catalog average — display reference, not billing rate.
    pub reference_mist_per_1m_in: u64,
    pub reference_mist_per_1m_out: u64,
    pub reference_model_count: usize,
    pub mist_per_myso: u64,
    pub catalog_usd_peg: f64,
    pub myso_usd: f64,
    pub price_oracle_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_age_secs: Option<u64>,
    pub price_stale: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::openrouter_client::OpenRouterModelRate;

    fn sample_catalog() -> PricingCatalog {
        let file: CatalogFile = toml::from_str(
            r#"
[catalog]
version = "2026-01-01"
source = "openrouter+manual"
effective_date = "2026-01-01"

[[models]]
aliases = ["openai/gpt-4o-mini", "gpt-4o-mini"]
display_name = "GPT-4o mini"
input_mist_per_1m = 150_000_000
output_mist_per_1m = 600_000_000

[defaults]
unknown_model_input_mist_per_1m = 1_000_000_000
unknown_model_output_mist_per_1m = 1_000_000_000
min_charge_mist = 1_000_000
"#,
        )
        .unwrap();
        PricingCatalog::from_file(file)
    }

    #[test]
    fn apply_openrouter_rates_updates_matching_alias() {
        let mut catalog = sample_catalog();
        let mut remote = HashMap::new();
        remote.insert(
            "openai/gpt-4o-mini".to_string(),
            OpenRouterModelRate {
                id: "openai/gpt-4o-mini".to_string(),
                input_usd_per_1m: 0.2,
                output_usd_per_1m: 0.8,
            },
        );

        let report = catalog.apply_openrouter_rates(&remote, 50.0);
        assert_eq!(report.checked, 1);
        assert_eq!(report.updated, 1);
        assert_eq!(catalog.model_rates("gpt-4o-mini").input_mist_per_1m, 200_000_000);
        assert_eq!(catalog.model_rates("gpt-4o-mini").output_mist_per_1m, 800_000_000);
    }

    #[test]
    fn apply_openrouter_rates_skips_unknown_models() {
        let mut catalog = sample_catalog();
        let mut remote = HashMap::new();
        remote.insert(
            "anthropic/claude-unknown".to_string(),
            OpenRouterModelRate {
                id: "anthropic/claude-unknown".to_string(),
                input_usd_per_1m: 100.0,
                output_usd_per_1m: 100.0,
            },
        );

        let report = catalog.apply_openrouter_rates(&remote, 50.0);
        assert_eq!(report.checked, 0);
        assert_eq!(report.updated, 0);
    }

    #[test]
    fn apply_openrouter_rates_respects_drift_cap() {
        let mut catalog = sample_catalog();
        let mut remote = HashMap::new();
        remote.insert(
            "openai/gpt-4o-mini".to_string(),
            OpenRouterModelRate {
                id: "openai/gpt-4o-mini".to_string(),
                input_usd_per_1m: 10.0,
                output_usd_per_1m: 10.0,
            },
        );

        let report = catalog.apply_openrouter_rates(&remote, 5.0);
        assert_eq!(report.checked, 1);
        assert_eq!(report.skipped_drift, 1);
        assert_eq!(report.updated, 0);
        assert_eq!(catalog.model_rates("gpt-4o-mini").input_mist_per_1m, 150_000_000);
    }

    #[test]
    fn save_and_load_round_trip() {
        let catalog = sample_catalog();
        let path = std::env::temp_dir().join(format!(
            "myso_ai_credit_catalog_test_{}.toml",
            uuid::Uuid::new_v4()
        ));
        catalog.save(&path).unwrap();
        let loaded = PricingCatalog::load(&path).unwrap();
        assert_eq!(loaded.version, catalog.version);
        assert_eq!(
            loaded.model_rates("openai/gpt-4o-mini").input_mist_per_1m,
            catalog.model_rates("openai/gpt-4o-mini").input_mist_per_1m
        );
        let _ = std::fs::remove_file(path);
    }
}
