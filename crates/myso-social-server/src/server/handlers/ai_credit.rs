// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::ai_credit::{AiCreditBalanceResponse, IngestUsageLineRequest};

use super::super::{AppState, PageParams};

pub async fn get_profile_ai_credit_balance(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<AiCreditBalanceResponse>, SocialError> {
    let balance = state
        .reader
        .get_ai_credit_balance_by_owner(&address)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("AI credit balance for '{}'", address)))?;
    let agent_budgets = state
        .reader
        .list_ai_credit_agent_budgets(&balance.balance_id)
        .await?;
    let credits = balance.balance_mist / 1_000_000_000;
    Ok(Json(AiCreditBalanceResponse {
        balance,
        credits,
        agent_budgets,
    }))
}

pub async fn ingest_usage_line_internal(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<IngestUsageLineRequest>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let expected = std::env::var("AI_CREDIT_USAGE_SYNC_SECRET").ok();
    if let Some(secret) = expected {
        let provided = headers
            .get("x-ai-credit-sync-secret")
            .and_then(|v| v.to_str().ok());
        if provided != Some(secret.as_str()) {
            return Err(SocialError::bad_request("invalid sync secret"));
        }
    }
    state.reader.ingest_ai_credit_usage_line(req).await?;
    Ok(Json(serde_json::json!({ "ok": true })))
}

#[derive(Debug, serde::Deserialize)]
pub struct UsageHistoryQuery {
    #[serde(flatten)]
    pub page: PageParams,
}

pub async fn list_ai_credit_usage_history(
    State(state): State<Arc<AppState>>,
    Path(balance_id): Path<String>,
    Query(query): Query<UsageHistoryQuery>,
) -> Result<Json<Vec<myso_indexer_alt_social_schema::models::AiCreditUsageLineRow>>, SocialError> {
    let lines = state
        .reader
        .list_ai_credit_usage_lines(&balance_id, query.page.limit())
        .await?;
    Ok(Json(lines))
}
