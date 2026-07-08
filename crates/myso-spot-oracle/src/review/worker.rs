// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use chrono::Utc;
use tracing::{info, warn};

use crate::api::AppState;
use crate::review::canonicalize::{canonicalize, claim_hash_hex};
use crate::review::compiler::ResolverCompiler;
use crate::review::llm::{extract_claim_heuristic, LlmClient};
use crate::review::rules::ReviewDecision;
use crate::store::jobs::SpotJob;

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
    let hash_hex = claim_hash_hex(&canonical);
    let duplicate =
        crate::store::reviews::claim_hash_exists(state.store.pool(), &hash_hex).await?;
    let canonical_id = crate::store::reviews::insert_canonical_claim(
        state.store.pool(),
        extraction_id,
        &canonical.normalized_fields,
        &hash_hex,
    )
    .await?;

    let decision = crate::review::rules::evaluate(&canonical, duplicate, &state.sources);
    let (decision_str, reject_reason) = match &decision {
        ReviewDecision::Accepted => ("accepted", None),
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
            let compiled = ResolverCompiler::compile(&canonical, &state.sources)?;
            let def_id = crate::store::reviews::insert_resolver_definition(
                state.store.pool(),
                canonical_id,
                &compiled.resolver_definition,
            )
            .await?;
            let betting_options = serde_json::to_value(&compiled.betting_options)?;
            crate::store::markets::update_market_status(
                state.store.pool(),
                market_id,
                "pending_create",
                Some(review_id),
                Some(def_id),
                Some(&betting_options),
            )
            .await?;
            crate::store::jobs::enqueue_job(
                state.store.pool(),
                "SubmitChainTx",
                Some(market_id),
                Some(def_id),
                100,
                Utc::now(),
                serde_json::json!({"tx_kind": "create_market"}),
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
            info!(post_id, market_id = %market_id, def_id = %def_id, "claim accepted and compiled");
        }
        ReviewDecision::Rejected(reason) => {
            crate::store::markets::update_market_status(
                state.store.pool(),
                market_id,
                "rejected",
                Some(review_id),
                None,
                None,
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
