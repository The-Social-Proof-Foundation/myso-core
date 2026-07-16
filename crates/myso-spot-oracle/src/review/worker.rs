// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use chrono::Utc;
use tracing::{info, warn};

use uuid::Uuid;

use crate::api::AppState;
use crate::claim::lifecycle::{default_context_for, LifecycleEvent};
use crate::review::canonicalize::{
    canonicalize_with_identity, market_key_hash_hex, semantic_claim_hash_hex, CanonicalClaim,
    CanonicalizeOptions,
};
use crate::review::claim_matcher::match_and_reconcile;
use crate::review::compiler::ResolverCompiler;
use crate::review::context_deadline::{apply_deadline_resolution, resolve_context_deadline};
use crate::review::deadline::DeadlinePolicy;
use crate::review::llm::{extract_claim_heuristic, LlmClient};
use crate::review::rules::ReviewDecision;
use crate::review::verify::{verify_and_build_verdict, VERDICT_FALSE, VERDICT_TRUE};
use crate::store::jobs::SpotJob;
use crate::store::SpotTrustedSourceRow;
use crate::types::{ClaimCategory, MarketStatus, TimeClass};

pub async fn process_review_job(state: Arc<AppState>, job: &SpotJob) -> anyhow::Result<()> {
    let market_id = job
        .market_id
        .ok_or_else(|| anyhow::anyhow!("ReviewPost missing market_id"))?;
    let post_id = job
        .payload
        .get("post_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("ReviewPost missing post_id"))?;
    let content = job
        .payload
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let post_type = job.payload.get("post_type").and_then(|v| v.as_str());

    let market = state
        .store
        .get_market(market_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("market not found"))?;
    if MarketStatus::from_str(&market.status) == Some(MarketStatus::PostCreated) {
        let mut ctx = default_context_for(&LifecycleEvent::ReviewEnqueued);
        ctx.job_id = Some(job.id);
        state
            .store
            .apply_market_transition(market_id, &LifecycleEvent::ReviewEnqueued, &ctx)
            .await?;
    }

    let (mut extracted, raw_response) = if let Some(key) = &state.args.openrouter_api_key {
        let llm = LlmClient::new(
            state.args.openrouter_api_url.clone(),
            key.clone(),
            state.args.llm_model.clone(),
        );
        match llm
            .extract_and_match(content, post_type, &state.event_registry)
            .await
        {
            Ok(r) => r,
            Err(err) => {
                warn!(post_id, error = %err, "LLM extraction failed, using heuristic");
                (
                    extract_claim_heuristic(content, &state.event_registry),
                    String::new(),
                )
            }
        }
    } else {
        (
            extract_claim_heuristic(content, &state.event_registry),
            String::new(),
        )
    };

    let claim_match = match_and_reconcile(content, &mut extracted, &state.event_registry);
    if let Some(ref ev) = claim_match.matched_event {
        state
            .metrics
            .event_match_total
            .with_label_values(&[&ev.category])
            .inc();
    }
    state
        .metrics
        .claim_match_total
        .with_label_values(&[&claim_match.domain, claim_match.match_tier])
        .inc();

    // CADR: infer deadline from context when extraction left it empty.
    if extracted.deadline.is_none() {
        if let Some(resolution) =
            resolve_context_deadline(content, extracted.claim_category, &state.event_registry)
        {
            state
                .metrics
                .deadline_inference_total
                .with_label_values(&[resolution.provenance.source.as_str()])
                .inc();
            info!(
                post_id,
                source = resolution.provenance.source.as_str(),
                event_id = ?resolution.provenance.event_id,
                deadline = %resolution.deadline,
                "CADR inferred deadline"
            );
            apply_deadline_resolution(&mut extracted, &resolution);
        }
    } else if extracted.resolver_hints.deadline_provenance.is_some() {
        state
            .metrics
            .deadline_inference_total
            .with_label_values(&["llm"])
            .inc();
    }

    let extraction_id = crate::store::reviews::insert_llm_extraction(
        state.store.pool(),
        post_id,
        if raw_response.is_empty() {
            None
        } else {
            Some(raw_response.as_str())
        },
        &extracted,
        &state.args.llm_model,
    )
    .await?;

    // Unsupported claims are not objectively checkable: no market, no verdict. Finalize the post
    // cleanly as `completed_no_actionable`.
    if extracted.time_class == TimeClass::Unsupported
        || extracted.claim_category == ClaimCategory::Unsupported
    {
        let reason = "unsupported_claim";
        let review_id = crate::store::reviews::insert_oracle_review(
            state.store.pool(),
            post_id,
            None,
            "rejected",
            Some(reason),
        )
        .await?;
        state
            .metrics
            .reviews_total
            .with_label_values(&["rejected", reason])
            .inc();
        let mut ctx = default_context_for(&LifecycleEvent::ReviewRejected);
        ctx.job_id = Some(job.id);
        ctx.status_reason = Some(reason.to_string());
        state
            .store
            .transition_with_metadata(
                market_id,
                LifecycleEvent::ReviewRejected,
                Some(review_id),
                None,
                None,
                ctx,
            )
            .await?;
        crate::store::jobs::enqueue_job(
            state.store.pool(),
            "SubmitChainTx",
            Some(market_id),
            None,
            80,
            Utc::now(),
            serde_json::json!({
                "tx_kind": "finalize_post",
                "post_id": post_id,
                "detected_claim_count": 1,
                "rejected_claim_count": 1,
                "truncated_claim_count": 0,
                "past_verified_count": 0,
            }),
        )
        .await?;
        info!(post_id, reason, "unsupported claim; finalized as no-actionable");
        return Ok(());
    }

    let canonical = canonicalize_with_identity(
        extraction_id,
        &extracted,
        Some(claim_match),
        &CanonicalizeOptions {
            price_market_spacing: chrono::Duration::seconds(
                state.args.price_market_spacing_secs as i64,
            ),
        },
    );
    let semantic_hex = semantic_claim_hash_hex(&canonical);
    let market_hex = market_key_hash_hex(&canonical);
    let dedup = resolve_dedup_target(&state, &canonical, &market_hex).await?;
    let market_exists = dedup.is_some();
    let canonical_id = crate::store::reviews::get_or_insert_canonical_claim(
        state.store.pool(),
        extraction_id,
        &canonical.normalized_fields,
        &semantic_hex,
        &market_hex,
    )
    .await?;

    let source_rows = state.store.list_enabled_sources().await.unwrap_or_default();

    // Past-claim track: verify against trusted sources and finalize with a verdict — never open a
    // market (even when no prior market ever existed, e.g. long-settled historical facts).
    if extracted.time_class == TimeClass::Past {
        return verify_and_finalize_past(
            &state,
            job,
            post_id,
            market_id,
            canonical_id,
            &canonical,
            &source_rows,
        )
        .await;
    }

    let deadline_policy = DeadlinePolicy::from_secs(
        state.args.min_deadline_lead_secs,
        state.args.max_deadline_horizon_secs,
        state.args.max_election_horizon_secs,
        state.args.max_sports_horizon_secs,
    );

    let provability =
        crate::review::rules::evaluate_provably(&canonical, market_exists, &state.sources);
    let decision = match provability {
        ReviewDecision::Accepted => {
            crate::review::rules::resolve_and_validate_deadline(&canonical, &deadline_policy)
        }
        other => other,
    };

    if let ReviewDecision::Rejected(reason) = &decision {
        state
            .metrics
            .deadline_rejection_total
            .with_label_values(&[reason.as_str()])
            .inc();
    }
    let (decision_str, reject_reason) = match &decision {
        ReviewDecision::Accepted => ("accepted", None),
        ReviewDecision::LinkedToExisting => ("linked", None),
        ReviewDecision::Rejected(reason) => ("rejected", Some(reason.as_str())),
    };
    let review_id = crate::store::reviews::insert_oracle_review(
        state.store.pool(),
        post_id,
        Some(canonical_id),
        decision_str,
        reject_reason,
    )
    .await?;

    state
        .metrics
        .reviews_total
        .with_label_values(&[decision_str, reject_reason.unwrap_or("none")])
        .inc();

    match decision {
        ReviewDecision::Accepted => {
            let deadline = canonical
                .normalized_fields
                .deadline
                .expect("review validated deadline");
            let compiled = ResolverCompiler::compile(&canonical, &state.sources, &source_rows)?;
            let def_id = crate::store::reviews::insert_resolver_definition(
                state.store.pool(),
                canonical_id,
                &compiled.resolver_definition,
                &compiled.compile_fingerprint,
            )
            .await?;
            let betting_options = serde_json::to_value(&compiled.betting_options)?;

            let claim_id = crate::store::claims::insert_spot_claim(
                state.store.pool(),
                &semantic_hex,
                canonical_id,
            )
            .await?;
            let spot_market_id = crate::store::claims::insert_spot_market(
                state.store.pool(),
                claim_id,
                &market_hex,
                canonical.normalized_fields.deadline,
                &betting_options,
                Some(def_id),
                canonical.normalized_fields.entity_ref.as_deref(),
                canonical.normalized_fields.competition_ref.as_deref(),
                canonical.normalized_fields.event_ref.as_deref(),
                canonical.normalized_fields.metric_ref.as_deref(),
                Some(&crate::review::outcome_identity::outcome_identity_hash_hex(
                    &canonical.outcome_identity,
                )),
                canonical
                    .outcome_market_key
                    .deadline_day
                    .as_deref(),
            )
            .await?;
            crate::store::claims::upsert_post_claim_link(
                state.store.pool(),
                post_id,
                claim_id,
                Some(spot_market_id),
                Some(market_id),
                Some(review_id),
                "primary",
            )
            .await?;

            let mut ctx = default_context_for(&LifecycleEvent::ReviewAccepted);
            ctx.job_id = Some(job.id);
            state
                .store
                .transition_with_metadata(
                    market_id,
                    LifecycleEvent::ReviewAccepted,
                    Some(review_id),
                    Some(def_id),
                    Some(&betting_options),
                    ctx,
                )
                .await?;

            let resolution_window_ms =
                (deadline - market.created_at).num_milliseconds().max(0);
            let max_buffer_ms = state.args.max_resolution_buffer_ms as i64;
            state
                .store
                .set_market_resolution_timing(
                    market_id,
                    resolution_window_ms,
                    max_buffer_ms,
                )
                .await?;

            crate::store::jobs::enqueue_job(
                state.store.pool(),
                "SubmitChainTx",
                Some(market_id),
                Some(def_id),
                100,
                Utc::now(),
                serde_json::json!({
                    "tx_kind": "create_claim_market",
                    "spot_claim_id": claim_id.to_string(),
                    "spot_market_id": spot_market_id.to_string(),
                    "semantic_claim_hash": semantic_hex,
                    "market_key_hash": market_hex,
                }),
            )
            .await?;
            crate::store::jobs::enqueue_job(
                state.store.pool(),
                "ResolveMarket",
                Some(market_id),
                Some(def_id),
                50,
                compiled.maturity_schedule.maturity_at,
                serde_json::json!({}),
            )
            .await?;

            let refund_at = deadline + chrono::Duration::milliseconds(max_buffer_ms);
            crate::store::jobs::enqueue_job(
                state.store.pool(),
                "SubmitChainTx",
                Some(market_id),
                Some(def_id),
                10,
                refund_at,
                serde_json::json!({"tx_kind": "refund_unresolved"}),
            )
            .await?;

            info!(post_id, market_id = %market_id, def_id = %def_id, "claim accepted and compiled");
            state.metrics.dedup_created_total.inc();
        }
        ReviewDecision::LinkedToExisting => {
            let existing_market = if let Some(row) = dedup {
                row
            } else {
                crate::store::claims::find_market_by_key_hash(state.store.pool(), &market_hex)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("linked market missing for key"))?
            };
            let claim_id = existing_market.claim_id;

            crate::store::claims::upsert_post_claim_link(
                state.store.pool(),
                post_id,
                claim_id,
                Some(existing_market.id),
                Some(market_id),
                Some(review_id),
                "linked",
            )
            .await?;

            let mut ctx = default_context_for(&LifecycleEvent::ReviewAccepted);
            ctx.job_id = Some(job.id);
            ctx.status_reason = Some("linked_to_existing_market".to_string());
            state
                .store
                .transition_with_metadata(
                    market_id,
                    LifecycleEvent::ReviewAccepted,
                    Some(review_id),
                    None,
                    None,
                    ctx,
                )
                .await?;

            crate::store::jobs::enqueue_job(
                state.store.pool(),
                "SubmitChainTx",
                Some(market_id),
                None,
                90,
                Utc::now(),
                serde_json::json!({
                    "tx_kind": "link_post",
                    "spot_claim_id": claim_id.to_string(),
                    "spot_market_id": existing_market.id.to_string(),
                }),
            )
            .await?;

            info!(
                post_id,
                claim_id = %claim_id,
                spot_market_id = %existing_market.id,
                "post linked to existing claim market"
            );
            state.metrics.dedup_linked_total.inc();
        }
        ReviewDecision::Rejected(reason) => {
            let mut ctx = default_context_for(&LifecycleEvent::ReviewRejected);
            ctx.job_id = Some(job.id);
            ctx.status_reason = Some(reason.as_str().to_string());
            state
                .store
                .transition_with_metadata(
                    market_id,
                    LifecycleEvent::ReviewRejected,
                    Some(review_id),
                    None,
                    None,
                    ctx,
                )
                .await?;
            info!(
                post_id,
                reason = reason.as_str(),
                "claim rejected"
            );
        }
    }
    Ok(())
}

async fn resolve_dedup_target(
    state: &Arc<AppState>,
    canonical: &CanonicalClaim,
    market_hex: &str,
) -> anyhow::Result<Option<crate::store::claims::SpotMarketRow>> {
    if let Some(row) =
        crate::store::claims::find_market_by_key_hash(state.store.pool(), market_hex).await?
    {
        return Ok(Some(row));
    }

    let identity_hex = crate::review::outcome_identity::outcome_identity_hash_hex(
        &canonical.outcome_identity,
    );
    if let Some(row) = crate::store::claims::find_market_by_outcome_identity_hash(
        state.store.pool(),
        &identity_hex,
    )
    .await?
    {
        return Ok(Some(row));
    }

    let f = &canonical.normalized_fields;
    if let (Some(event_ref), Some(entity_ref), Some(deadline)) =
        (f.event_ref.as_ref(), f.entity_ref.as_ref(), f.deadline)
    {
        let deadline_day = crate::review::outcome_identity::deadline_day_bucket(deadline);
        if let Some(row) = crate::store::claims::find_market_by_graph_refs(
            state.store.pool(),
            event_ref,
            entity_ref,
            &deadline_day,
        )
        .await?
        {
            return Ok(Some(row));
        }
    }

    if crate::blockchain::chain_configured(&state.args) {
        if let Ok(hash_bytes) = hex::decode(market_hex.trim_start_matches("0x")) {
            if let Some(on_chain) =
                crate::blockchain::chain_lookup::lookup_market_by_key_hash(&state.args, &hash_bytes)
                    .await?
            {
                if let Some(row) =
                    crate::store::claims::find_market_by_key_hash(state.store.pool(), market_hex)
                        .await?
                {
                    return Ok(Some(row));
                }
                if let Some(row) = crate::store::claims::find_market_by_object_id(
                    state.store.pool(),
                    &on_chain.market_object_id,
                )
                .await?
                {
                    return Ok(Some(row));
                }
            }
        }
    }

    Ok(None)
}

/// All-zero on-chain address encoding "no related market" in the finalize past-verdict vectors.
const ZERO_ADDR: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

/// Verify a past claim against trusted sources and enqueue an on-chain finalize carrying the
/// verdict + evidence hash (+ related historical market when one exists). No market is opened.
async fn verify_and_finalize_past(
    state: &Arc<AppState>,
    job: &SpotJob,
    post_id: &str,
    market_id: Uuid,
    canonical_id: Uuid,
    canonical: &CanonicalClaim,
    source_rows: &[SpotTrustedSourceRow],
) -> anyhow::Result<()> {
    let verdict = verify_and_build_verdict(state, canonical, source_rows).await;
    let decision_str = match verdict.verdict {
        VERDICT_TRUE => "past_true",
        VERDICT_FALSE => "past_false",
        _ => "past_unverifiable",
    };

    let review_id = crate::store::reviews::insert_oracle_review(
        state.store.pool(),
        post_id,
        Some(canonical_id),
        decision_str,
        None,
    )
    .await?;
    state
        .metrics
        .reviews_total
        .with_label_values(&[decision_str, "none"])
        .inc();

    let mut ctx = default_context_for(&LifecycleEvent::ReviewRejected);
    ctx.job_id = Some(job.id);
    ctx.status_reason = Some(decision_str.to_string());
    state
        .store
        .transition_with_metadata(
            market_id,
            LifecycleEvent::ReviewRejected,
            Some(review_id),
            None,
            None,
            ctx,
        )
        .await?;

    let related = verdict
        .related_market_object_id
        .clone()
        .unwrap_or_else(|| ZERO_ADDR.to_string());
    let evidence_hex = format!("0x{}", hex::encode(&verdict.evidence_hash));
    crate::store::jobs::enqueue_job(
        state.store.pool(),
        "SubmitChainTx",
        Some(market_id),
        None,
        80,
        Utc::now(),
        serde_json::json!({
            "tx_kind": "finalize_post",
            "post_id": post_id,
            "detected_claim_count": 1,
            "rejected_claim_count": 0,
            "truncated_claim_count": 0,
            "past_verified_count": 1,
            "past_claim_indexes": [0],
            "past_verdicts": [verdict.verdict],
            "past_related_market_ids": [related],
            "past_evidence_hashes": [evidence_hex],
            "veracity_manifest_hash": evidence_hex,
            "evidence_urls": verdict.evidence_urls,
            "summary": verdict.summary,
        }),
    )
    .await?;
    info!(
        post_id,
        verdict = decision_str,
        related = ?verdict.related_market_object_id,
        "past claim verified and finalized"
    );
    Ok(())
}

pub async fn run_review_job_loop(state: Arc<AppState>, job: SpotJob) -> anyhow::Result<()> {
    let job_id = job.id;
    match process_review_job(state.clone(), &job).await {
        Ok(()) => {
            crate::store::jobs::complete_job(state.store.pool(), job_id, "completed", None).await?;
        }
        Err(err) => {
            let backoff = chrono::Duration::seconds(30 * 2_i64.pow(job.attempts.min(5) as u32));
            crate::store::jobs::requeue_job(
                state.store.pool(),
                job_id,
                Utc::now() + backoff,
                &err.to_string(),
            )
            .await?;
            return Err(err);
        }
    }
    Ok(())
}
