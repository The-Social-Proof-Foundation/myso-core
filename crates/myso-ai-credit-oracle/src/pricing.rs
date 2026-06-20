// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::catalog::{MIST_PER_MYSO, PricingCatalog};

pub const USAGE_INFERENCE: u8 = 1;
pub const USAGE_TOOL: u8 = 2;
pub const USAGE_EMBED: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriceBreakdown {
    pub base_mist: u64,
    pub margin_mist: u64,
    pub amount_mist: u64,
}

#[derive(Clone)]
pub struct PricingEngine {
    catalog: PricingCatalog,
    ecosystem_margin_pct: f64,
}

impl PricingEngine {
    pub fn new(catalog: PricingCatalog, ecosystem_margin_pct: f64) -> Self {
        Self {
            catalog,
            ecosystem_margin_pct: ecosystem_margin_pct.clamp(0.10, 0.15),
        }
    }

    pub fn ecosystem_margin_pct(&self) -> f64 {
        self.ecosystem_margin_pct
    }

    pub fn catalog_version(&self) -> &str {
        &self.catalog.version
    }

    pub fn inference_mist(&self, model_id: &str, tokens_in: u64, tokens_out: u64) -> u64 {
        self.inference_breakdown(model_id, tokens_in, tokens_out).amount_mist
    }

    pub fn inference_breakdown(
        &self,
        model_id: &str,
        tokens_in: u64,
        tokens_out: u64,
    ) -> PriceBreakdown {
        let rates = self.catalog.model_rates(model_id);
        let base = self.token_mist(tokens_in, rates.input_mist_per_1m)
            + self.token_mist(tokens_out, rates.output_mist_per_1m);
        self.split_margin(base)
    }

    pub fn embedding_mist(&self, model_id: &str, tokens: u64) -> u64 {
        self.embedding_breakdown(model_id, tokens).amount_mist
    }

    pub fn embedding_breakdown(&self, model_id: &str, tokens: u64) -> PriceBreakdown {
        let rate = self.catalog.embedding_mist_per_1m(model_id);
        self.split_margin(self.token_mist(tokens, rate))
    }

    pub fn tool_mist(&self, tool_id: &str) -> u64 {
        self.tool_breakdown(tool_id).amount_mist
    }

    pub fn tool_breakdown(&self, tool_id: &str) -> PriceBreakdown {
        self.split_margin(self.catalog.tool_flat_mist(tool_id))
    }

    pub fn with_margin(&self, base_mist: u64) -> u64 {
        self.split_margin(base_mist).amount_mist
    }

    pub fn split_margin(&self, base_mist: u64) -> PriceBreakdown {
        let min = self.catalog.defaults.min_charge_mist;
        let with_margin = ((base_mist as f64) * (1.0 + self.ecosystem_margin_pct)).ceil() as u64;
        let amount_mist = with_margin.max(min);
        let margin_mist = amount_mist.saturating_sub(base_mist);
        PriceBreakdown {
            base_mist,
            margin_mist,
            amount_mist,
        }
    }

    fn token_mist(&self, tokens: u64, mist_per_1m: u64) -> u64 {
        ((tokens as u128) * (mist_per_1m as u128) / 1_000_000) as u64
    }

    pub fn credits_from_mist(&self, mist: u64) -> f64 {
        mist as f64 / MIST_PER_MYSO as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CatalogFile;

    fn test_catalog() -> PricingCatalog {
        let file: CatalogFile = toml::from_str(
            r#"
[catalog]
version = "test"

[[models]]
aliases = ["gpt-4o-mini"]
display_name = "mini"
input_mist_per_1m = 150_000_000
output_mist_per_1m = 600_000_000

[defaults]
unknown_model_input_mist_per_1m = 1_000_000_000
unknown_model_output_mist_per_1m = 1_000_000_000
min_charge_mist = 0
"#,
        )
        .unwrap();
        PricingCatalog::from_file(file)
    }

    #[test]
    fn inference_with_margin() {
        let engine = PricingEngine::new(test_catalog(), 0.125);
        let breakdown = engine.inference_breakdown("gpt-4o-mini", 2000, 500);
        assert_eq!(breakdown.base_mist, 600_000);
        assert_eq!(breakdown.amount_mist, 675_000);
        assert_eq!(breakdown.margin_mist, 75_000);
    }

    #[test]
    fn margin_clamps_to_ten_percent_minimum() {
        let engine = PricingEngine::new(test_catalog(), 0.0);
        let breakdown = engine.inference_breakdown("gpt-4o-mini", 1, 0);
        assert_eq!(breakdown.amount_mist, 165);
    }
}
