// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deterministic resolution engine. Loads stored `ResolverDefinition` only — no NLU.

use std::sync::Arc;

use chrono::Utc;
use tracing::info;

use crate::api::AppState;
use crate::resolver::{ResolutionDraft, ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::SourceEvidence;
use crate::store::jobs::SpotJob;
use crate::types::ComparisonOp;

pub async fn resolve_market(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let market_id = job
        .market_id
        .ok_or_else(|| anyhow::anyhow!("ResolveMarket missing market_id"))?;
    let def_id = job
        .resolver_definition_id
        .ok_or_else(|| anyhow::anyhow!("ResolveMarket missing resolver_definition_id"))?;
    let def = state
        .store
        .get_resolver_definition(def_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("resolver definition not found"))?;

    let market = state
        .store
        .get_market(market_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("market not found"))?;
    if market.status != "active" && market.status != "resolving" {
        return Ok(());
    }

    let adapters = state.sources.supports(&def);
    if adapters.is_empty() {
        anyhow::bail!("no adapter supports resolver definition");
    }

    let mut evidence_list: Vec<SourceEvidence> = Vec::new();
    for adapter in &adapters {
        match adapter.resolve(&def).await {
            Ok(ev) => evidence_list.push(ev),
            Err(err) => tracing::warn!(adapter = adapter.id(), error = %err, "resolve failed"),
        }
    }
    if evidence_list.is_empty() {
        anyhow::bail!("all adapters failed to resolve");
    }

    let store_raw = state.args.store_raw_evidence;
    for ev in &evidence_list {
        state
            .store
            .insert_evidence(
                market_id,
                Some(job.id),
                &ev.adapter_id,
                &ev.source_url,
                &ev.content_hash,
                if store_raw {
                    ev.raw_response.as_deref()
                } else {
                    None
                },
            )
            .await?;
    }

    let draft = evaluate_definition(&def, &evidence_list)?;
    state
        .store
        .upsert_resolver_state(
            market_id,
            draft.outcome_label.as_deref(),
            Some(draft.confidence_bps as i32),
        )
        .await?;

    if draft.outcome_label.is_some() {
        state
            .store
            .update_market_status(market_id, "resolving", None, None, None)
            .await?;
        state
            .store
            .enqueue_job(
                "SubmitChainTx",
                Some(market_id),
                Some(def_id),
                200,
                Utc::now(),
                serde_json::json!({
                    "tx_kind": "oracle_resolve",
                    "outcome_label": draft.outcome_label,
                    "confidence_bps": draft.confidence_bps,
                    "reasoning": draft.reasoning,
                    "evidence_urls": draft.evidence_urls,
                }),
            )
            .await?;
        info!(
            market_id = %market_id,
            outcome = ?draft.outcome_label,
            confidence_bps = draft.confidence_bps,
            "resolution draft ready for chain submit"
        );
    }
    Ok(())
}

fn evaluate_definition(
    def: &ResolverDefinition,
    evidence: &[SourceEvidence],
) -> anyhow::Result<ResolutionDraft> {
    match def.resolver_kind {
        ResolverKind::PriceThreshold => evaluate_price_threshold(def, evidence),
        ResolverKind::ReleasePublished => evaluate_release(def, evidence),
        ResolverKind::EventOccurrence => Ok(ResolutionDraft {
            outcome_label: None,
            confidence_bps: 0,
            reasoning: "event not yet matched".to_string(),
            evidence_urls: evidence.iter().map(|e| e.source_url.clone()).collect(),
        }),
        ResolverKind::CustomHttp => Ok(ResolutionDraft {
            outcome_label: None,
            confidence_bps: 0,
            reasoning: "custom http unresolved".to_string(),
            evidence_urls: evidence.iter().map(|e| e.source_url.clone()).collect(),
        }),
    }
}

fn evaluate_price_threshold(
    def: &ResolverDefinition,
    evidence: &[SourceEvidence],
) -> anyhow::Result<ResolutionDraft> {
    let ResolverSpec::PriceThreshold {
        asset,
        quote,
        comparator,
        threshold,
        json_path,
        ..
    } = &def.spec
    else {
        anyhow::bail!("expected PriceThreshold spec");
    };

    let path = if json_path.is_empty() {
        format!("{asset}.{quote}")
    } else {
        json_path.clone()
    };

    let mut price: Option<f64> = None;
    let mut urls = Vec::new();
    for ev in evidence {
        urls.push(ev.source_url.clone());
        if let Some(v) = json_path_value(&ev.payload, &path) {
            price = Some(v);
            break;
        }
        if let Some(v) = json_path_value(&ev.payload, &format!("{asset}.{quote}")) {
            price = Some(v);
            break;
        }
        if let Some(v) = ev.payload.get("data").and_then(|d| d.get("amount")).and_then(|a| a.as_str()) {
            if let Ok(p) = v.parse::<f64>() {
                price = Some(p);
                break;
            }
        }
    }
    let price = price.ok_or_else(|| anyhow::anyhow!("could not parse price from evidence"))?;
    let threshold_f: f64 = threshold.parse()?;
    let condition = match comparator {
        ComparisonOp::Gt => price > threshold_f,
        ComparisonOp::Gte => price >= threshold_f,
        ComparisonOp::Lt => price < threshold_f,
        ComparisonOp::Lte => price <= threshold_f,
        ComparisonOp::Eq => (price - threshold_f).abs() < f64::EPSILON,
        ComparisonOp::Neq => (price - threshold_f).abs() >= f64::EPSILON,
    };

    let yes_label = def
        .betting_options
        .first()
        .cloned()
        .unwrap_or_else(|| "Yes".to_string());
    let no_label = def
        .betting_options
        .get(1)
        .cloned()
        .unwrap_or_else(|| "No".to_string());
    let outcome = if condition { yes_label } else { no_label };

    Ok(ResolutionDraft {
        outcome_label: Some(outcome),
        confidence_bps: 9500,
        reasoning: format!(
            "price {price} vs threshold {threshold_f} ({comparator:?}) for {asset}/{quote}"
        ),
        evidence_urls: urls,
    })
}

fn evaluate_release(
    def: &ResolverDefinition,
    evidence: &[SourceEvidence],
) -> anyhow::Result<ResolutionDraft> {
    let ResolverSpec::ReleasePublished { tag_predicate, .. } = &def.spec else {
        anyhow::bail!("expected ReleasePublished spec");
    };
    let urls: Vec<String> = evidence.iter().map(|e| e.source_url.clone()).collect();
    let tag = evidence
        .first()
        .and_then(|e| e.payload.get("tag_name"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let matched = !tag.is_empty() && (tag_predicate.is_empty() || tag.contains(tag_predicate));
    let yes = def.betting_options.first().cloned().unwrap_or_else(|| "Yes".to_string());
    let no = def.betting_options.get(1).cloned().unwrap_or_else(|| "No".to_string());
    Ok(ResolutionDraft {
        outcome_label: Some(if matched { yes } else { no }),
        confidence_bps: if matched { 9000 } else { 8000 },
        reasoning: format!("release tag '{tag}' vs predicate '{tag_predicate}'"),
        evidence_urls: urls,
    })
}

fn json_path_value(payload: &serde_json::Value, path: &str) -> Option<f64> {
    let mut current = payload;
    for part in path.split('.') {
        current = current.get(part)?;
    }
    current.as_f64().or_else(|| current.as_str().and_then(|s| s.parse().ok()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    use crate::resolver::{MaturitySchedule, ResolverDefinition, ResolverKind, ResolverSpec};

    #[test]
    fn price_threshold_yes_when_above_threshold() {
        let def = ResolverDefinition {
            id: Uuid::new_v4(),
            resolver_kind: ResolverKind::PriceThreshold,
            spec: ResolverSpec::PriceThreshold {
                asset: "bitcoin".to_string(),
                quote: "usd".to_string(),
                comparator: ComparisonOp::Gt,
                threshold: "1".to_string(),
                source_id: "coingecko".to_string(),
                json_path: "bitcoin.usd".to_string(),
            },
            source_ids: vec!["coingecko".to_string()],
            betting_options: vec!["Yes".to_string(), "No".to_string()],
            maturity_schedule: MaturitySchedule {
                maturity_at: Utc::now(),
                deadline: Utc::now(),
            },
        };
        let evidence = vec![SourceEvidence {
            adapter_id: "coingecko".to_string(),
            source_url: "https://api.coingecko.com".to_string(),
            content_hash: "abc".to_string(),
            raw_response: None,
            fetched_at: Utc::now(),
            payload: serde_json::json!({ "bitcoin": { "usd": 50000.0 } }),
        }];
        let draft = evaluate_definition(&def, &evidence).unwrap();
        assert_eq!(draft.outcome_label.as_deref(), Some("Yes"));
    }
}
