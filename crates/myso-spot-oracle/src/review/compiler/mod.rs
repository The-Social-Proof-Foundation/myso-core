// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

mod custom_http;
pub(crate) mod event;
mod maturity;
mod price;
mod release;
pub(crate) mod source_select;
mod validate;

use uuid::Uuid;

use crate::resolver::{MaturitySchedule, ResolverDefinition};
use crate::review::CanonicalClaim;

#[derive(Debug, Clone)]
pub struct CompiledMarketSpec {
    pub resolver_definition: ResolverDefinition,
    pub betting_options: Vec<String>,
    pub source_ids: Vec<String>,
    pub maturity_schedule: MaturitySchedule,
    pub compile_fingerprint: String,
}

pub struct ResolverCompiler;

impl ResolverCompiler {
    pub fn compile(
        canonical: &CanonicalClaim,
        registry: &crate::sources::ResolverRegistry,
        source_rows: &[crate::store::SpotTrustedSourceRow],
    ) -> anyhow::Result<CompiledMarketSpec> {
        let category = canonical.normalized_fields.claim_category;
        let spec = match category {
            crate::types::ClaimCategory::PriceThreshold => {
                price::compile(canonical, registry, source_rows)?
            }
            crate::types::ClaimCategory::ReleasePublished => {
                release::compile(canonical, registry, source_rows)?
            }
            crate::types::ClaimCategory::EventOccurrence => {
                event::compile(canonical, registry, source_rows)?
            }
            crate::types::ClaimCategory::CustomHttp => {
                custom_http::compile(canonical, registry, source_rows)?
            }
            crate::types::ClaimCategory::Unsupported => {
                anyhow::bail!("cannot compile unsupported claim category")
            }
        };
        validate::validate_compiled(&spec, registry)?;
        Ok(spec)
    }
}

pub fn compute_compile_fingerprint(
    canonical: &CanonicalClaim,
    def: &ResolverDefinition,
) -> String {
    use sha2::{Digest, Sha256};
    let canonical_json = serde_json::to_vec(&canonical.normalized_fields).expect("serialize");
    let spec_json = serde_json::to_value(&def.spec).expect("serialize");
    let mut source_ids = def.source_ids.clone();
    source_ids.sort();
    let payload = serde_json::json!({
        "canonical": canonical_json,
        "spec": spec_json,
        "source_ids": source_ids,
        "resolver_kind": def.resolver_kind,
    });
    let bytes = serde_json::to_vec(&payload).expect("serialize");
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    hex::encode(hasher.finalize())
}

pub(crate) fn default_betting_options(canonical: &CanonicalClaim) -> Vec<String> {
    let f = &canonical.normalized_fields;
    if f.suggested_options.is_empty() {
        vec!["Yes".to_string(), "No".to_string()]
    } else {
        f.suggested_options.clone()
    }
}

pub(crate) fn build_definition(
    canonical: &CanonicalClaim,
    kind: crate::types::ResolverKind,
    spec: crate::resolver::ResolverSpec,
    source_ids: Vec<String>,
    betting_options: Vec<String>,
    maturity_schedule: MaturitySchedule,
) -> CompiledMarketSpec {
    let def = ResolverDefinition {
        id: Uuid::new_v4(),
        resolver_kind: kind,
        spec,
        source_ids: source_ids.clone(),
        betting_options: betting_options.clone(),
        maturity_schedule: maturity_schedule.clone(),
    };
    let compile_fingerprint = compute_compile_fingerprint(canonical, &def);
    CompiledMarketSpec {
        resolver_definition: def,
        betting_options,
        source_ids,
        maturity_schedule,
        compile_fingerprint,
    }
}

#[cfg(test)]
pub(crate) mod fixtures;
#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) fn test_registry() -> crate::sources::ResolverRegistry {
    crate::sources::build_default_registry()
}
