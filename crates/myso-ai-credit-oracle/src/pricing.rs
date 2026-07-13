// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::time::Instant;

use crate::catalog::{PricingCatalog, MIST_PER_MYSO};
use crate::myso_price_client::{validate_myso_usd, MIN_MYSO_USD};

pub const USAGE_INFERENCE: u8 = 1;
pub const USAGE_TOOL: u8 = 2;
pub const USAGE_EMBED: u8 = 3;
pub const CATALOG_USD_PEG: f64 = 1.0;
/// Used when the remote price oracle is unreachable and no price has been fetched yet.
pub const DEFAULT_MYSO_USD_FALLBACK: f64 = 0.0045;
/// On-chain default oracle markup (15%) when GraphQL/REST config is unavailable.
pub const DEFAULT_ORACLE_MARKUP_BPS: u64 = 1500;
pub const USD_MICROS_PER_USD: u128 = 1_000_000;
pub const USD_E8_PER_USD: u128 = 100_000_000;

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
    oracle_markup_bps: u64,
    myso_usd: f64,
    price_fetched_at: Option<Instant>,
    price_ever_fetched: bool,
    markup_ever_fetched: bool,
}

impl PricingEngine {
    pub fn new(catalog: PricingCatalog, ecosystem_margin_pct: f64) -> Self {
        Self {
            catalog,
            ecosystem_margin_pct: ecosystem_margin_pct.clamp(0.0, 1.0),
            oracle_markup_bps: pct_to_markup_bps(ecosystem_margin_pct),
            myso_usd: CATALOG_USD_PEG,
            price_fetched_at: None,
            price_ever_fetched: false,
            markup_ever_fetched: false,
        }
    }

    pub fn set_oracle_markup_bps(&mut self, bps: u64) {
        self.oracle_markup_bps = bps;
        self.ecosystem_margin_pct = bps as f64 / 10_000.0;
        self.markup_ever_fetched = true;
    }

    pub fn set_ecosystem_margin_pct(&mut self, pct: f64) {
        let pct = pct.clamp(0.0, 1.0);
        self.ecosystem_margin_pct = pct;
        self.oracle_markup_bps = pct_to_markup_bps(pct);
        self.markup_ever_fetched = true;
    }

    pub fn apply_fallback_markup(&mut self) {
        self.set_oracle_markup_bps(DEFAULT_ORACLE_MARKUP_BPS);
    }

    pub fn oracle_markup_bps(&self) -> u64 {
        self.oracle_markup_bps
    }

    pub fn markup_ever_fetched(&self) -> bool {
        self.markup_ever_fetched
    }

    pub fn set_myso_usd(&mut self, usd: f64, fetched_at: Instant) {
        if validate_myso_usd(usd).is_ok() {
            self.myso_usd = usd;
            self.price_fetched_at = Some(fetched_at);
            self.price_ever_fetched = true;
        }
    }

    pub fn apply_fallback_myso_usd(&mut self) {
        self.myso_usd = DEFAULT_MYSO_USD_FALLBACK;
        self.price_fetched_at = Some(Instant::now());
        self.price_ever_fetched = true;
    }

    pub fn myso_usd(&self) -> f64 {
        self.myso_usd
    }

    pub fn price_ever_fetched(&self) -> bool {
        self.price_ever_fetched
    }

    pub fn price_age_secs(&self) -> Option<u64> {
        self.price_fetched_at.map(|t| t.elapsed().as_secs())
    }

    pub fn is_price_stale(&self, max_stale_secs: u64) -> bool {
        match self.price_fetched_at {
            Some(t) => t.elapsed().as_secs() > max_stale_secs,
            None => true,
        }
    }

    pub fn ecosystem_margin_pct(&self) -> f64 {
        self.ecosystem_margin_pct
    }

    pub fn catalog_version(&self) -> &str {
        &self.catalog.version
    }

    pub fn replace_catalog(&mut self, catalog: PricingCatalog) {
        self.catalog = catalog;
    }

    pub fn inference_mist(&self, model_id: &str, tokens_in: u64, tokens_out: u64) -> u64 {
        self.inference_breakdown(model_id, tokens_in, tokens_out)
            .amount_mist
    }

    /// Convert OpenRouter's actual account charge into MIST using checked
    /// integer arithmetic.  The current FX quote and on-chain markup are the
    /// only inputs; model token rates are deliberately not involved.
    pub fn provider_cost_breakdown(&self, provider_cost_usd_micros: u64) -> PriceBreakdown {
        Self::provider_cost_breakdown_at_quote(
            provider_cost_usd_micros,
            self.myso_usd_e8(),
            self.oracle_markup_bps,
        )
    }

    /// Convert provider cost using the exact FX + markup quote that was locked
    /// into an on-chain reservation. Capture must never read a refreshed price.
    pub fn provider_cost_breakdown_at_quote(
        provider_cost_usd_micros: u64,
        myso_usd_e8: u64,
        markup_bps: u64,
    ) -> PriceBreakdown {
        assert!(myso_usd_e8 > 0, "MYSO/USD quote must be positive");
        let marked_usd_micros = ceil_mul_div_u128(
            provider_cost_usd_micros as u128,
            10_000u128 + markup_bps as u128,
            10_000,
        );
        let base_mist = usd_micros_to_mist_at_quote(provider_cost_usd_micros as u128, myso_usd_e8);
        let amount_mist = usd_micros_to_mist_at_quote(marked_usd_micros, myso_usd_e8);
        PriceBreakdown {
            base_mist,
            margin_mist: amount_mist.saturating_sub(base_mist),
            amount_mist,
        }
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
        let flat = self.scale_mist(self.catalog.tool_flat_mist(tool_id));
        self.split_margin(flat)
    }

    pub fn with_margin(&self, base_mist: u64) -> u64 {
        self.split_margin(base_mist).amount_mist
    }

    pub fn split_margin(&self, base_mist: u64) -> PriceBreakdown {
        let min = self.scale_mist(self.catalog.defaults.min_charge_mist);
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
        let catalog_mist = ((tokens as u128) * (mist_per_1m as u128) / 1_000_000) as u64;
        self.scale_mist(catalog_mist)
    }

    pub fn scale_mist(&self, catalog_mist: u64) -> u64 {
        if catalog_mist == 0 {
            return 0;
        }
        let rate = self.myso_usd.max(MIN_MYSO_USD);
        ((catalog_mist as f64 / rate).ceil()) as u64
    }

    pub fn myso_usd_e8(&self) -> u64 {
        let bounded = self.myso_usd.max(MIN_MYSO_USD);
        (bounded * USD_E8_PER_USD as f64).round().max(1.0) as u64
    }
}

fn usd_micros_to_mist_at_quote(usd_micros: u128, myso_usd_e8: u64) -> u64 {
    if usd_micros == 0 {
        return 0;
    }
    // USD micros * (1e8 USD-price scale) * (1e9 MIST/MYSO)
    // divided by (1e6 micros/USD) * price_e8.
    let numerator = usd_micros
        .checked_mul(USD_E8_PER_USD)
        .and_then(|v| v.checked_mul(MIST_PER_MYSO as u128))
        .expect("provider cost conversion overflow");
    let denominator = USD_MICROS_PER_USD
        .checked_mul(myso_usd_e8 as u128)
        .expect("provider cost denominator overflow");
    let amount = ceil_div_u128(numerator, denominator);
    u64::try_from(amount).expect("provider cost exceeds u64 MIST")
}

fn ceil_div_u128(value: u128, divisor: u128) -> u128 {
    debug_assert!(divisor > 0);
    value.saturating_add(divisor - 1) / divisor
}

fn ceil_mul_div_u128(value: u128, multiplier: u128, divisor: u128) -> u128 {
    ceil_div_u128(
        value
            .checked_mul(multiplier)
            .expect("fixed-point multiplication overflow"),
        divisor,
    )
}

fn pct_to_markup_bps(pct: f64) -> u64 {
    ((pct.clamp(0.0, 1.0) * 10_000.0).round()) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CatalogFile;
    use std::time::Instant;

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
min_charge_mist = 1_000_000
"#,
        )
        .unwrap();
        PricingCatalog::from_file(file)
    }

    #[test]
    fn inference_with_margin_at_usd_peg() {
        let mut engine = PricingEngine::new(test_catalog(), 0.15);
        engine.set_myso_usd(1.0, Instant::now());
        let breakdown = engine.inference_breakdown("gpt-4o-mini", 2000, 500);
        assert_eq!(breakdown.base_mist, 600_000);
        assert_eq!(breakdown.amount_mist, 1_000_000);
        assert_eq!(breakdown.margin_mist, 400_000);
    }

    #[test]
    fn split_margin_at_fifteen_percent_bps() {
        let mut engine = PricingEngine::new(test_catalog(), 0.0);
        engine.set_oracle_markup_bps(1500);
        let breakdown = engine.split_margin(1_000_000);
        assert_eq!(breakdown.base_mist, 1_000_000);
        assert_eq!(breakdown.amount_mist, 1_150_000);
        assert_eq!(breakdown.margin_mist, 150_000);
        assert!((engine.ecosystem_margin_pct() - 0.15).abs() < f64::EPSILON);
    }

    #[test]
    fn apply_fallback_markup_uses_default_bps() {
        let mut engine = PricingEngine::new(test_catalog(), 0.0);
        engine.apply_fallback_markup();
        assert_eq!(engine.oracle_markup_bps(), DEFAULT_ORACLE_MARKUP_BPS);
        assert!(engine.markup_ever_fetched());
    }

    #[test]
    fn inference_scales_with_cheaper_myso() {
        let mut engine = PricingEngine::new(test_catalog(), 0.0);
        engine.set_myso_usd(0.004, Instant::now());
        let breakdown = engine.inference_breakdown("gpt-4o-mini", 1_000_000, 0);
        assert_eq!(breakdown.base_mist, 37_500_000_000);
    }

    #[test]
    fn price_stale_when_never_fetched() {
        let engine = PricingEngine::new(test_catalog(), 0.15);
        assert!(!engine.price_ever_fetched());
        assert!(engine.is_price_stale(300));
    }

    #[test]
    fn fallback_sets_default_myso_usd() {
        let mut engine = PricingEngine::new(test_catalog(), 0.15);
        engine.apply_fallback_myso_usd();
        assert!(engine.price_ever_fetched());
        assert!((engine.myso_usd() - DEFAULT_MYSO_USD_FALLBACK).abs() < f64::EPSILON);
    }

    #[test]
    fn actual_provider_cost_uses_fixed_point_fx_and_markup() {
        let mut engine = PricingEngine::new(test_catalog(), 0.0);
        engine.set_oracle_markup_bps(1500);
        engine.set_myso_usd(0.0045, Instant::now());
        // $0.0012 provider cost + 15% = $0.00138. At $0.0045/MYSO this
        // captures 0.306666667 MYSO, rounded up to integer MIST.
        let breakdown = engine.provider_cost_breakdown(1_200);
        assert_eq!(breakdown.base_mist, 266_666_667);
        assert_eq!(breakdown.amount_mist, 306_666_667);
        assert_eq!(breakdown.margin_mist, 40_000_000);
        assert_eq!(engine.myso_usd_e8(), 450_000);
    }

    #[test]
    fn capture_uses_locked_quote_not_refreshed_engine_price() {
        let locked = PricingEngine::provider_cost_breakdown_at_quote(1_200, 450_000, 1500);
        assert_eq!(locked.amount_mist, 306_666_667);

        let mut engine = PricingEngine::new(test_catalog(), 0.0);
        engine.set_oracle_markup_bps(1500);
        engine.set_myso_usd(0.003, Instant::now());
        assert_ne!(
            engine.provider_cost_breakdown(1_200).amount_mist,
            locked.amount_mist
        );
    }
}
