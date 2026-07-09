// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::resolver::ResolverSpec;
use crate::review::compiler::{build_definition, default_betting_options, CompiledMarketSpec};
use crate::review::CanonicalClaim;
use crate::sources::ResolverRegistry;
use crate::store::DiscoverySourceRow;
use crate::types::{ClaimCategory, ComparisonOp, ResolverKind};

pub fn compile(
    canonical: &CanonicalClaim,
    registry: &ResolverRegistry,
    source_rows: &[DiscoverySourceRow],
) -> anyhow::Result<CompiledMarketSpec> {
    let f = &canonical.normalized_fields;
    let betting_options = default_betting_options(canonical);
    let maturity_schedule = super::maturity::compute_schedule(canonical, ClaimCategory::PriceThreshold);

    let comparator = f.comparison.unwrap_or(ComparisonOp::Gt);
    let threshold = f
        .threshold
        .clone()
        .ok_or_else(|| anyhow::anyhow!("price claim missing threshold"))?;
    let asset = f.subject.clone();
    let quote = if f.object.is_empty() {
        "usd".to_string()
    } else {
        f.object.clone()
    };

    let preview = crate::resolver::ResolverDefinition {
        id: uuid::Uuid::new_v4(),
        resolver_kind: ResolverKind::PriceThreshold,
        spec: ResolverSpec::PriceThreshold {
            asset: asset.clone(),
            quote: quote.clone(),
            comparator,
            threshold: threshold.clone(),
            source_id: String::new(),
            json_path: price_json_path(&asset, &quote, ""),
        },
        source_ids: vec![],
        betting_options: betting_options.clone(),
        maturity_schedule: maturity_schedule.clone(),
    };

    let source_id = super::source_select::select_source(
        registry,
        source_rows,
        &preview,
        &f.resolver_hints.preferred_sources,
        &["coingecko", "coinbase", "http_official", "chainlink"],
    )?;
    let json_path = price_json_path(&asset, &quote, &source_id);

    let spec = ResolverSpec::PriceThreshold {
        asset,
        quote,
        comparator,
        threshold,
        source_id: source_id.clone(),
        json_path,
    };

    Ok(build_definition(
        canonical,
        ResolverKind::PriceThreshold,
        spec,
        vec![source_id],
        betting_options,
        maturity_schedule,
    ))
}

pub fn price_json_path(asset: &str, quote: &str, source_id: &str) -> String {
    match source_id {
        "coinbase" => format!("{asset}-{quote}"),
        _ => format!("{asset}.{quote}"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::review::compiler::fixtures;
    use crate::review::compiler::test_registry;

    #[test]
    fn btc_price_compiles_deterministically() {
        let canonical = fixtures::btc_price_claim();
        let registry = test_registry();
        let a = compile(&canonical, &registry, &[]).unwrap();
        let b = compile(&canonical, &registry, &[]).unwrap();
        assert_eq!(a.compile_fingerprint, b.compile_fingerprint);
        assert_eq!(a.resolver_definition.resolver_kind, ResolverKind::PriceThreshold);
    }
}
