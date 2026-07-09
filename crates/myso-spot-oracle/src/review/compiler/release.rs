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
    let owner = hints
        .owner
        .clone()
        .filter(|s| !s.is_empty())
        .or_else(|| {
            if f.object.contains('/') {
                Some(f.object.split('/').next().unwrap_or("").to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow::anyhow!("release claim missing owner"))?;
    let repo = hints
        .repo
        .clone()
        .filter(|s| !s.is_empty())
        .or_else(|| {
            if f.object.contains('/') {
                Some(f.object.split('/').nth(1).unwrap_or("").to_string())
            } else if !f.object.is_empty() {
                Some(f.object.clone())
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow::anyhow!("release claim missing repo"))?;
    let tag_predicate = hints
        .tag_predicate
        .clone()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| f.predicate.clone());

    let betting_options = default_betting_options(canonical);
    let maturity_schedule =
        super::maturity::compute_schedule(canonical, ClaimCategory::ReleasePublished);

    let preview = crate::resolver::ResolverDefinition {
        id: uuid::Uuid::new_v4(),
        resolver_kind: ResolverKind::ReleasePublished,
        spec: ResolverSpec::ReleasePublished {
            owner: owner.clone(),
            repo: repo.clone(),
            tag_predicate: tag_predicate.clone(),
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
        &["github_releases"],
    )?;

    let spec = ResolverSpec::ReleasePublished {
        owner,
        repo,
        tag_predicate,
        source_id: source_id.clone(),
    };

    Ok(build_definition(
        canonical,
        ResolverKind::ReleasePublished,
        spec,
        vec![source_id],
        betting_options,
        maturity_schedule,
    ))
}
