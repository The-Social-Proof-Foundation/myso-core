// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use chrono::Utc;
use tracing::{info, warn};

use crate::api::AppState;
use crate::claim::lifecycle::{default_context_for, LifecycleEvent};
use crate::review::canonicalize::{canonicalize, market_key_hash_hex, semantic_claim_hash_hex};
use crate::review::compiler::ResolverCompiler;
use crate::review::llm::{extract_claim_heuristic, LlmClient};
use crate::review::rules::ReviewDecision;
use crate::store::jobs::SpotJob;
use crate::types::MarketStatus;

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

    let (extracted, raw_response) = if let Some(key) = &state.args.openrouter_api_key {
        let llm = LlmClient::new(
            state.args.openrouter_api_url.clone(),
            key.clone(),
            state.args.llm_model.clone(),
        );
        match llm.extract_claim(content, post_type).await {
            Ok(r) => r,
            Err(err) => {
                warn!(post_id, error = %err, "LLM extraction failed, using heuristic");
                (extract_claim_heuristic(content), String::new())
            }
        }
    } else {
        (extract_claim_heuristic(content), String::new())
    };

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

    let canonical = canonicalize(extraction_id, &extracted);
    let semantic_hex = semantic_claim_hash_hex(&canonical);
    let market_hex = market_key_hash_hex(&canonical);
    let market_exists =
        crate::store::claims::market_key_hash_exists(state.store.pool(), &market_hex).await?;
    let canonical_id = crate::store::reviews::get_or_insert_canonical_claim(
        state.store.pool(),
        extraction_id,
        &canonical.normalized_fields,
        &semantic_hex,
        &market_hex,
    )
    .await?;

    let source_rows = state.store.list_enabled_sources().await.unwrap_or_default();
    let decision = crate::review::rules::evaluate(&canonical, market_exists, &state.sources);
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

            let refund_at = market.created_at
                + chrono::Duration::milliseconds(market.max_resolution_window_ms);
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
        }
        ReviewDecision::LinkedToExisting => {
            let existing_market =
                crate::store::claims::find_market_by_key_hash(state.store.pool(), &market_hex)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("linked market missing for key"))?;
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
