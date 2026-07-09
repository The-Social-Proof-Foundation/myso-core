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
    let hints = &f.resolver_hints;

    let url = hints
        .url
        .clone()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("custom_http claim missing url"))?;
    let json_path = hints
        .json_path
        .clone()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("custom_http claim missing json_path"))?;
    let comparator = hints.comparison.or(f.comparison).unwrap_or(ComparisonOp::Eq);
    let expected = hints
        .expected
        .clone()
        .or(f.threshold.clone())
        .ok_or_else(|| anyhow::anyhow!("custom_http claim missing expected value"))?;

    let betting_options = default_betting_options(canonical);
    let maturity_schedule =
        super::maturity::compute_schedule(canonical, ClaimCategory::CustomHttp);

    let preview = crate::resolver::ResolverDefinition {
        id: uuid::Uuid::new_v4(),
        resolver_kind: ResolverKind::CustomHttp,
        spec: ResolverSpec::CustomHttp {
            url: url.clone(),
            json_path: json_path.clone(),
            comparator,
            expected: expected.clone(),
            source_id: String::new(),
        },
        source_ids: vec![],
        betting_options: betting_options.clone(),
        maturity_schedule: maturity_schedule.clone(),
    };

    let source_id = super::source_select::select_source(
        registry,
        source_rows,
        &preview,
        &hints.preferred_sources,
        &["http_official"],
    )?;

    let spec = ResolverSpec::CustomHttp {
        url,
        json_path,
        comparator,
        expected,
        source_id: source_id.clone(),
    };

    Ok(build_definition(
        canonical,
        ResolverKind::CustomHttp,
        spec,
        vec![source_id],
        betting_options,
        maturity_schedule,
    ))
}
