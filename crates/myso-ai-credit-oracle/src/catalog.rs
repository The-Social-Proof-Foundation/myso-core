// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub const MIST_PER_MYSO: u64 = 1_000_000_000;
pub const CAP_AI_SPEND: u64 = 16384;

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize, Default)]
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
