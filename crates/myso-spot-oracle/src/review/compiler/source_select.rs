// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::resolver::{ResolverDefinition, ResolverSpec};
use crate::sources::ResolverRegistry;
use crate::store::SpotTrustedSourceRow;

/// Deterministically pick a `TrustedSource` adapter for a preview definition.
pub fn select_source(
    registry: &ResolverRegistry,
    source_rows: &[SpotTrustedSourceRow],
    preview: &ResolverDefinition,
    preferred: &[String],
    fallback_order: &[&str],
) -> anyhow::Result<String> {
    for id in preferred {
        if adapter_supports(registry, preview, id) {
            return Ok(id.clone());
        }
    }

    for id in fallback_order {
        if adapter_supports(registry, preview, id) {
            return Ok((*id).to_string());
        }
    }

    let mut candidates: Vec<String> = registry
        .all()
        .into_iter()
        .filter_map(|s| {
            let id = s.id().to_string();
            if adapter_supports(registry, preview, &id) {
                Some(id)
            } else {
                None
            }
        })
        .collect();

    if candidates.is_empty() {
        anyhow::bail!(
            "no trusted source supports resolver kind {:?}",
            preview.resolver_kind
        );
    }

    candidates.sort_by(|a, b| {
        let score_a = trust_score_for(source_rows, a);
        let score_b = trust_score_for(source_rows, b);
        score_b
            .partial_cmp(&score_a)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.cmp(b))
    });

    Ok(candidates[0].clone())
}

fn adapter_supports(
    registry: &ResolverRegistry,
    preview: &ResolverDefinition,
    adapter_id: &str,
) -> bool {
    if registry.get(adapter_id).is_none() {
        return false;
    }
    let mut def = preview.clone();
    set_spec_source_id(&mut def, adapter_id);
    !registry.supports(&def).is_empty()
}

fn set_spec_source_id(def: &mut ResolverDefinition, source_id: &str) {
    match &mut def.spec {
        ResolverSpec::PriceThreshold { source_id: sid, .. }
        | ResolverSpec::EventOccurrence { source_id: sid, .. }
        | ResolverSpec::ReleasePublished { source_id: sid, .. }
        | ResolverSpec::CustomHttp { source_id: sid, .. } => {
            *sid = source_id.to_string();
        }
    }
    def.source_ids = vec![source_id.to_string()];
}

fn trust_score_for(source_rows: &[SpotTrustedSourceRow], adapter_id: &str) -> f64 {
    source_rows
        .iter()
        .find(|r| adapter_id_matches_row(adapter_id, r))
        .map(|r| r.trust_score)
        .unwrap_or(0.0)
}

fn adapter_id_matches_row(adapter_id: &str, row: &SpotTrustedSourceRow) -> bool {
    if row.adapter_type == adapter_id {
        return true;
    }
    match adapter_id {
        "rss_event" => row.adapter_type == "rss",
        "coingecko" | "coinbase" | "chainlink" => row.adapter_type == adapter_id,
        "github_releases" => row.adapter_type == "github_releases",
        "http_official" => row.adapter_type == "http_official",
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    use crate::resolver::{MaturitySchedule, ResolverKind, ResolverSpec};
    use crate::review::compiler::test_registry;
    use crate::types::ComparisonOp;

    fn price_preview() -> ResolverDefinition {
        ResolverDefinition {
            id: Uuid::new_v4(),
            resolver_kind: ResolverKind::PriceThreshold,
            spec: ResolverSpec::PriceThreshold {
                asset: "bitcoin".to_string(),
                quote: "usd".to_string(),
                comparator: ComparisonOp::Gt,
                threshold: "1".to_string(),
                source_id: String::new(),
                json_path: "bitcoin.usd".to_string(),
            },
            source_ids: vec![],
            betting_options: vec!["Yes".into(), "No".into()],
            maturity_schedule: MaturitySchedule {
                maturity_at: Utc::now(),
                deadline: Utc::now(),
            },
        }
    }

    #[test]
    fn source_pick_is_deterministic() {
        let registry = test_registry();
        let preview = price_preview();
        let a = select_source(&registry, &[], &preview, &[], &["coingecko", "coinbase"]).unwrap();
        let b = select_source(&registry, &[], &preview, &[], &["coingecko", "coinbase"]).unwrap();
        assert_eq!(a, b);
        assert_eq!(a, "coingecko");
    }
}
