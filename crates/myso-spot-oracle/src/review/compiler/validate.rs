// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::review::compiler::CompiledMarketSpec;
use crate::resolver::{ResolverDefinition, ResolverSpec};
use crate::sources::ResolverRegistry;

pub fn validate_compiled(
    spec: &CompiledMarketSpec,
    registry: &ResolverRegistry,
) -> anyhow::Result<()> {
    validate_definition(&spec.resolver_definition, registry)?;
    if spec.betting_options.len() < 2 || spec.betting_options.len() > 10 {
        anyhow::bail!("betting options must be 2-10");
    }
    if spec.maturity_schedule.maturity_at >= spec.maturity_schedule.deadline {
        anyhow::bail!("maturity_at must be before deadline");
    }
    if spec.compile_fingerprint.is_empty() {
        anyhow::bail!("missing compile fingerprint");
    }
    Ok(())
}

pub fn validate_definition(
    def: &ResolverDefinition,
    registry: &ResolverRegistry,
) -> anyhow::Result<()> {
    if def.source_ids.is_empty() {
        anyhow::bail!("resolver definition missing source_ids");
    }
    for id in &def.source_ids {
        if registry.get(id).is_none() {
            anyhow::bail!("source {id} not registered");
        }
    }
    if def.betting_options.len() < 2 || def.betting_options.len() > 10 {
        anyhow::bail!("betting options must be 2-10");
    }
    validate_spec(&def.spec)?;
    if registry.supports(def).is_empty() {
        anyhow::bail!("no adapter supports compiled definition");
    }
    Ok(())
}

fn validate_spec(spec: &ResolverSpec) -> anyhow::Result<()> {
    match spec {
        ResolverSpec::PriceThreshold {
            asset,
            quote,
            threshold,
            source_id,
            json_path,
            ..
        } => {
            require_non_empty(asset, "asset")?;
            require_non_empty(quote, "quote")?;
            require_non_empty(threshold, "threshold")?;
            require_non_empty(source_id, "source_id")?;
            require_non_empty(json_path, "json_path")?;
        }
        ResolverSpec::ReleasePublished {
            owner,
            repo,
            source_id,
            ..
        } => {
            require_non_empty(owner, "owner")?;
            require_non_empty(repo, "repo")?;
            require_non_empty(source_id, "source_id")?;
        }
        ResolverSpec::EventOccurrence {
            feed_url,
            match_predicate,
            source_id,
        } => {
            require_non_empty(feed_url, "feed_url")?;
            require_non_empty(match_predicate, "match_predicate")?;
            require_non_empty(source_id, "source_id")?;
        }
        ResolverSpec::CustomHttp {
            url,
            json_path,
            expected,
            source_id,
            ..
        } => {
            require_non_empty(url, "url")?;
            require_non_empty(json_path, "json_path")?;
            require_non_empty(expected, "expected")?;
            require_non_empty(source_id, "source_id")?;
        }
    }
    Ok(())
}

fn require_non_empty(value: &str, field: &str) -> anyhow::Result<()> {
    if value.trim().is_empty() {
        anyhow::bail!("resolver spec field {field} must not be empty");
    }
    Ok(())
}
