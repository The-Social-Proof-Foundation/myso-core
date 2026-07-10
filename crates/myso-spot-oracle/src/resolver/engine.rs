// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deterministic resolution engine. Loads stored `ResolverDefinition` only — no NLU.

use std::sync::Arc;

use chrono::Utc;
use tracing::info;

use crate::api::AppState;
use crate::claim::lifecycle::{default_context_for, LifecycleEvent};
use crate::evidence::EvidenceBundle;
use crate::resolver::{ResolutionDraft, ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::SourceEvidence;
use crate::store::jobs::SpotJob;
use crate::types::{MarketStatus, OnChainSpotStatus};
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

    let status = MarketStatus::from_str(&market.status);
    match status {
        Some(MarketStatus::Resolved | MarketStatus::Refunded | MarketStatus::Rejected | MarketStatus::DaoRequired | MarketStatus::Failed) => {
            return Ok(());
        }
        Some(MarketStatus::Waiting | MarketStatus::Resolving) => {}
        _ => return Ok(()),
    }

    let now = Utc::now();
    if now < def.maturity_schedule.maturity_at {
        state
            .store
            .requeue_job(job.id, def.maturity_schedule.maturity_at, "before maturity")
            .await?;
        return Ok(());
    }

    if now > def.maturity_schedule.deadline {
        enqueue_refund(state.clone(), market_id, def_id).await?;
        return Ok(());
    }

    let mut ctx = default_context_for(&LifecycleEvent::ResolveAttemptStarted);
    ctx.job_id = Some(job.id);
    state
        .store
        .apply_market_transition(market_id, &LifecycleEvent::ResolveAttemptStarted, &ctx)
        .await?;

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
        state
            .store
            .requeue_job(job.id, now + chrono::Duration::minutes(5), "all adapters failed")
            .await?;
        anyhow::bail!("all adapters failed to resolve");
    }

    let bundle = EvidenceBundle::build(market_id, job.id, &evidence_list);
    state.store.insert_evidence_bundle(&bundle).await?;

    let draft = evaluate_definition(&def, &evidence_list)?;
    state
        .store
        .upsert_resolver_state(
            market_id,
            draft.outcome_label.as_deref(),
            Some(draft.confidence_bps as i32),
        )
        .await?;

    if draft.outcome_label.is_none() {
        let next_run = if now + chrono::Duration::minutes(15) < def.maturity_schedule.deadline {
            now + chrono::Duration::minutes(15)
        } else {
            def.maturity_schedule.deadline
        };
        state
            .store
            .requeue_job(job.id, next_run, "outcome not yet determined")
            .await?;
        return Ok(());
    }

    if draft.evidence_urls.is_empty() {
        anyhow::bail!("resolution draft missing evidence urls");
    }

    if now < def.maturity_schedule.deadline {
        state
            .store
            .requeue_job(
                job.id,
                def.maturity_schedule.deadline,
                "before resolution_at",
            )
            .await?;
        return Ok(());
    }

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
                "bundle_hash": bundle.bundle_hash,
            }),
        )
        .await?;
    info!(
        market_id = %market_id,
        outcome = ?draft.outcome_label,
        confidence_bps = draft.confidence_bps,
        "resolution draft ready for chain submit"
    );
    Ok(())
}

async fn enqueue_refund(state: Arc<AppState>, market_id: uuid::Uuid, def_id: uuid::Uuid) -> anyhow::Result<()> {
    state
        .store
        .enqueue_job(
            "SubmitChainTx",
            Some(market_id),
            Some(def_id),
            150,
            Utc::now(),
            serde_json::json!({"tx_kind": "refund_unresolved"}),
        )
        .await?;
    Ok(())
}

fn evaluate_definition(
    def: &ResolverDefinition,
    evidence: &[SourceEvidence],
) -> anyhow::Result<ResolutionDraft> {
    match def.resolver_kind {
        ResolverKind::PriceThreshold => evaluate_price_threshold(def, evidence),
        ResolverKind::ReleasePublished => evaluate_release(def, evidence),
        ResolverKind::EventOccurrence => evaluate_event(def, evidence),
        ResolverKind::CustomHttp => evaluate_custom_http(def, evidence),
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

    let threshold_f: f64 = threshold.parse()?;
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

    let mut urls = Vec::new();
    let mut outcomes: Vec<String> = Vec::new();
    let mut prices: Vec<f64> = Vec::new();
    for ev in evidence {
        urls.push(ev.source_url.clone());
        let price = json_path_value(&ev.payload, &path)
            .or_else(|| json_path_value(&ev.payload, &format!("{asset}.{quote}")))
            .or_else(|| {
                ev.payload
                    .get("data")
                    .and_then(|d| d.get("amount"))
                    .and_then(|a| a.as_str())
                    .and_then(|v| v.parse::<f64>().ok())
            });
        let Some(price) = price else {
            continue;
        };
        prices.push(price);
        let condition = match comparator {
            ComparisonOp::Gt => price > threshold_f,
            ComparisonOp::Gte => price >= threshold_f,
            ComparisonOp::Lt => price < threshold_f,
            ComparisonOp::Lte => price <= threshold_f,
            ComparisonOp::Eq => (price - threshold_f).abs() < f64::EPSILON,
            ComparisonOp::Neq => (price - threshold_f).abs() >= f64::EPSILON,
        };
        outcomes.push(if condition {
            yes_label.clone()
        } else {
            no_label.clone()
        });
    }

    if outcomes.is_empty() {
        anyhow::bail!("could not parse price from evidence");
    }

    let (outcome, confidence_bps, reasoning) = quorum_outcome(
        &outcomes,
        &format!(
            "price {:?} vs threshold {threshold_f} ({comparator:?}) for {asset}/{quote}",
            prices
        ),
    );

    Ok(ResolutionDraft {
        outcome_label: Some(outcome),
        confidence_bps,
        reasoning,
        evidence_urls: urls,
    })
}

/// Majority quorum: unanimous → high confidence; conflict → low confidence (DAO_REQUIRED).
fn quorum_outcome(outcomes: &[String], base_reasoning: &str) -> (String, u16, String) {
    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for o in outcomes {
        *counts.entry(o.as_str()).or_default() += 1;
    }
    let (winner, count) = counts
        .into_iter()
        .max_by_key(|(_, c)| *c)
        .map(|(k, c)| (k.to_string(), c))
        .unwrap_or_else(|| (outcomes[0].clone(), 1));
    if count == outcomes.len() {
        (
            winner,
            9500,
            format!("{base_reasoning}; quorum={count}/{}", outcomes.len()),
        )
    } else {
        (
            winner,
            0,
            format!(
                "{base_reasoning}; CONFLICT quorum={count}/{} — escalate DAO_REQUIRED",
                outcomes.len()
            ),
        )
    }
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

fn evaluate_event(
    def: &ResolverDefinition,
    evidence: &[SourceEvidence],
) -> anyhow::Result<ResolutionDraft> {
    let ResolverSpec::EventOccurrence { match_predicate, .. } = &def.spec else {
        anyhow::bail!("expected EventOccurrence spec");
    };
    let urls: Vec<String> = evidence.iter().map(|e| e.source_url.clone()).collect();
    let matched = evidence.iter().any(|ev| {
        // Direct RSS fetch returns an array of event objects.
        if let Some(arr) = ev.payload.as_array() {
            return arr.iter().any(|item| {
                let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("");
                let summary = item.get("summary").and_then(|v| v.as_str()).unwrap_or("");
                format!("{title} {summary}")
                    .to_lowercase()
                    .contains(&match_predicate.to_lowercase())
            });
        }
        let title = ev.payload.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let summary = ev
            .payload
            .get("summary")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let blob = format!("{title} {summary}").to_lowercase();
        blob.contains(&match_predicate.to_lowercase())
    });
    if !matched {
        return Ok(ResolutionDraft {
            outcome_label: None,
            confidence_bps: 0,
            reasoning: format!("event not yet matched for predicate '{match_predicate}'"),
            evidence_urls: urls,
        });
    }
    let yes = def.betting_options.first().cloned().unwrap_or_else(|| "Yes".to_string());
    let _no = def.betting_options.get(1).cloned().unwrap_or_else(|| "No".to_string());
    Ok(ResolutionDraft {
        outcome_label: Some(yes),
        confidence_bps: 8500,
        reasoning: format!("event matched predicate '{match_predicate}'"),
        evidence_urls: urls,
    })
}

fn evaluate_custom_http(
    def: &ResolverDefinition,
    evidence: &[SourceEvidence],
) -> anyhow::Result<ResolutionDraft> {
    let ResolverSpec::CustomHttp {
        json_path,
        comparator,
        expected,
        ..
    } = &def.spec
    else {
        anyhow::bail!("expected CustomHttp spec");
    };
    let urls: Vec<String> = evidence.iter().map(|e| e.source_url.clone()).collect();
    let actual = evidence
        .first()
        .and_then(|e| json_path_value_str(&e.payload, json_path));
    let Some(actual) = actual else {
        return Ok(ResolutionDraft {
            outcome_label: None,
            confidence_bps: 0,
            reasoning: "custom http value not yet available".to_string(),
            evidence_urls: urls,
        });
    };
    let condition = match comparator {
        ComparisonOp::Eq => actual == *expected,
        ComparisonOp::Neq => actual != *expected,
        ComparisonOp::Gt => actual.parse::<f64>().ok().zip(expected.parse::<f64>().ok()).map(|(a, e)| a > e).unwrap_or(false),
        ComparisonOp::Gte => actual.parse::<f64>().ok().zip(expected.parse::<f64>().ok()).map(|(a, e)| a >= e).unwrap_or(false),
        ComparisonOp::Lt => actual.parse::<f64>().ok().zip(expected.parse::<f64>().ok()).map(|(a, e)| a < e).unwrap_or(false),
        ComparisonOp::Lte => actual.parse::<f64>().ok().zip(expected.parse::<f64>().ok()).map(|(a, e)| a <= e).unwrap_or(false),
    };
    let yes = def.betting_options.first().cloned().unwrap_or_else(|| "Yes".to_string());
    let no = def.betting_options.get(1).cloned().unwrap_or_else(|| "No".to_string());
    Ok(ResolutionDraft {
        outcome_label: Some(if condition { yes } else { no }),
        confidence_bps: 8800,
        reasoning: format!("custom http {actual} vs expected {expected} ({comparator:?})"),
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

fn json_path_value_str(payload: &serde_json::Value, path: &str) -> Option<String> {
    let mut current = payload;
    for part in path.split('.') {
        current = current.get(part)?;
    }
    current
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| current.as_f64().map(|f| f.to_string()))
}

pub fn is_high_confidence(confidence_bps: u64, threshold_bps: u64) -> bool {
    confidence_bps >= threshold_bps
}

pub fn on_chain_status_for_resolve(high_confidence: bool) -> i16 {
    if high_confidence {
        OnChainSpotStatus::Resolved as i16
    } else {
        OnChainSpotStatus::DaoRequired as i16
    }
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
        assert!(draft.confidence_bps >= 9000);
    }

    #[test]
    fn price_threshold_conflict_low_confidence() {
        let def = ResolverDefinition {
            id: Uuid::new_v4(),
            resolver_kind: ResolverKind::PriceThreshold,
            spec: ResolverSpec::PriceThreshold {
                asset: "bitcoin".to_string(),
                quote: "usd".to_string(),
                comparator: ComparisonOp::Gt,
                threshold: "40000".to_string(),
                source_id: "coingecko".to_string(),
                json_path: "bitcoin.usd".to_string(),
            },
            source_ids: vec!["coingecko".to_string(), "coinbase".to_string()],
            betting_options: vec!["Yes".to_string(), "No".to_string()],
            maturity_schedule: MaturitySchedule {
                maturity_at: Utc::now(),
                deadline: Utc::now(),
            },
        };
        let evidence = vec![
            SourceEvidence {
                adapter_id: "coingecko".to_string(),
                source_url: "https://api.coingecko.com".to_string(),
                content_hash: "a".to_string(),
                raw_response: None,
                fetched_at: Utc::now(),
                payload: serde_json::json!({ "bitcoin": { "usd": 50000.0 } }),
            },
            SourceEvidence {
                adapter_id: "coinbase".to_string(),
                source_url: "https://api.coinbase.com".to_string(),
                content_hash: "b".to_string(),
                raw_response: None,
                fetched_at: Utc::now(),
                payload: serde_json::json!({ "bitcoin": { "usd": 10000.0 } }),
            },
        ];
        let draft = evaluate_definition(&def, &evidence).unwrap();
        assert!(draft.outcome_label.is_some());
        assert_eq!(draft.confidence_bps, 0);
        assert!(draft.reasoning.contains("CONFLICT"));
        assert!(!is_high_confidence(draft.confidence_bps as u64, 7000));
    }

    #[test]
    fn confidence_threshold_mapping() {
        assert!(is_high_confidence(9500, 7000));
        assert!(!is_high_confidence(6500, 7000));
        assert_eq!(on_chain_status_for_resolve(true), 3);
        assert_eq!(on_chain_status_for_resolve(false), 2);
    }
}
