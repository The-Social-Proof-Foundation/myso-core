// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::resolver::{
    MaturitySchedule, ResolverDefinition, ResolverKind, ResolverSpec,
};
use crate::review::rules::is_price_claim;
use crate::review::CanonicalClaim;
use crate::sources::ResolverRegistry;
use crate::types::ComparisonOp;

#[derive(Debug, Clone)]
pub struct CompiledMarketSpec {
    pub resolver_definition: ResolverDefinition,
    pub betting_options: Vec<String>,
    pub source_ids: Vec<String>,
    pub maturity_schedule: MaturitySchedule,
}

pub struct ResolverCompiler;

impl ResolverCompiler {
    pub fn compile(
        canonical: &CanonicalClaim,
        registry: &ResolverRegistry,
    ) -> anyhow::Result<CompiledMarketSpec> {
        let f = &canonical.normalized_fields;
        let betting_options = if f.suggested_options.is_empty() {
            vec!["Yes".to_string(), "No".to_string()]
        } else {
            f.suggested_options.clone()
        };

        let now = Utc::now();
        let deadline = f.deadline.unwrap_or_else(|| now + Duration::hours(24));
        let maturity_at = now + Duration::minutes(1);
        let maturity_schedule = MaturitySchedule {
            maturity_at,
            deadline,
        };

        if is_price_claim(canonical) {
            let comparator = f.comparison.unwrap_or(ComparisonOp::Gt);
            let threshold = f
                .threshold
                .clone()
                .ok_or_else(|| anyhow::anyhow!("price claim missing threshold"))?;
            let source_id = pick_price_source(registry, &f.suggested_sources);
            let asset = f.subject.clone();
            let quote = if f.object.is_empty() {
                "usd".to_string()
            } else {
                f.object.clone()
            };
            let def = ResolverDefinition {
                id: Uuid::new_v4(),
                resolver_kind: ResolverKind::PriceThreshold,
                spec: ResolverSpec::PriceThreshold {
                    asset,
                    quote,
                    comparator,
                    threshold,
                    source_id: source_id.clone(),
                    json_path: "bitcoin.usd".to_string(),
                },
                source_ids: vec![source_id.clone()],
                betting_options: betting_options.clone(),
                maturity_schedule: maturity_schedule.clone(),
            };
            return Ok(CompiledMarketSpec {
                resolver_definition: def,
                betting_options,
                source_ids: vec![source_id],
                maturity_schedule,
            });
        }

        if f.predicate.contains("release") || f.suggested_sources.iter().any(|s| s.contains("github")) {
            let source_id = if registry.get("github_releases").is_some() {
                "github_releases".to_string()
            } else {
                anyhow::bail!("no github_releases adapter registered");
            };
            let def = ResolverDefinition {
                id: Uuid::new_v4(),
                resolver_kind: ResolverKind::ReleasePublished,
                spec: ResolverSpec::ReleasePublished {
                    owner: "rust-lang".to_string(),
                    repo: "rust".to_string(),
                    tag_predicate: f.object.clone(),
                    source_id: source_id.clone(),
                },
                source_ids: vec![source_id.clone()],
                betting_options: betting_options.clone(),
                maturity_schedule: maturity_schedule.clone(),
            };
            return Ok(CompiledMarketSpec {
                resolver_definition: def,
                betting_options,
                source_ids: vec![source_id],
                maturity_schedule,
            });
        }

        let source_id = registry
            .all()
            .first()
            .map(|s| s.id().to_string())
            .ok_or_else(|| anyhow::anyhow!("no trusted source for event claim"))?;
        let def = ResolverDefinition {
            id: Uuid::new_v4(),
            resolver_kind: ResolverKind::EventOccurrence,
            spec: ResolverSpec::EventOccurrence {
                feed_url: String::new(),
                match_predicate: f.predicate.clone(),
                source_id: source_id.clone(),
            },
            source_ids: vec![source_id.clone()],
            betting_options: betting_options.clone(),
            maturity_schedule: maturity_schedule.clone(),
        };
        Ok(CompiledMarketSpec {
            resolver_definition: def,
            betting_options,
            source_ids: vec![source_id],
            maturity_schedule,
        })
    }
}

fn pick_price_source(registry: &ResolverRegistry, suggested: &[String]) -> String {
    for id in suggested {
        if registry.get(id).is_some() {
            return id.clone();
        }
    }
    for candidate in ["coingecko", "coinbase", "http_official", "chainlink"] {
        if registry.get(candidate).is_some() {
            return candidate.to_string();
        }
    }
    "coingecko".to_string()
}
