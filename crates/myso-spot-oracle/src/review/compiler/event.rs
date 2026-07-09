// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::resolver::ResolverSpec;
use crate::review::compiler::{build_definition, default_betting_options, CompiledMarketSpec};
use crate::review::CanonicalClaim;
use crate::sources::ResolverRegistry;
use crate::store::DiscoverySourceRow;
use crate::types::{ClaimCategory, ResolverKind};

pub fn compile(
    canonical: &CanonicalClaim,
    registry: &ResolverRegistry,
    source_rows: &[DiscoverySourceRow],
) -> anyhow::Result<CompiledMarketSpec> {
    let f = &canonical.normalized_fields;
    let hints = &f.resolver_hints;

    let feed_url = hints
        .feed_url
        .clone()
        .filter(|s| !s.is_empty())
        .or_else(|| feed_url_from_sources(source_rows))
        .ok_or_else(|| anyhow::anyhow!("event claim missing feed_url"))?;

    let match_predicate = hints
        .match_predicate
        .clone()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| f.predicate.clone());

    let betting_options = default_betting_options(canonical);
    let maturity_schedule =
        super::maturity::compute_schedule(canonical, ClaimCategory::EventOccurrence);

    let preview = crate::resolver::ResolverDefinition {
        id: uuid::Uuid::new_v4(),
        resolver_kind: ResolverKind::EventOccurrence,
        spec: ResolverSpec::EventOccurrence {
            feed_url: feed_url.clone(),
            match_predicate: match_predicate.clone(),
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
        &["rss_event", "http_official"],
    )?;

    let spec = ResolverSpec::EventOccurrence {
        feed_url,
        match_predicate,
        source_id: source_id.clone(),
    };

    Ok(build_definition(
        canonical,
        ResolverKind::EventOccurrence,
        spec,
        vec![source_id],
        betting_options,
        maturity_schedule,
    ))
}

fn feed_url_from_sources(source_rows: &[DiscoverySourceRow]) -> Option<String> {
    for row in source_rows {
        if row.adapter_type == "rss" {
            if let Some(urls) = row.config.get("feed_urls").and_then(|v| v.as_array()) {
                if let Some(first) = urls.first().and_then(|u| u.as_str()) {
                    return Some(first.to_string());
                }
            }
        }
    }
    None
}
