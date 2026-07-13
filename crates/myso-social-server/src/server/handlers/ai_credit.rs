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
    let active_reservations = state
        .reader
        .list_live_ai_spend_reservations(&balance.balance_id)
        .await?;
    let available_mist = balance.balance_mist.saturating_sub(balance.reserved_mist);
    Ok(Json(AiCreditBalanceResponse {
        balance,
        billing_unit: "MIST",
        available_mist,
        agent_budgets,
        active_reservations,
    }))
}

#[derive(Debug, serde::Deserialize)]
pub struct ReservationHistoryQuery {
    #[serde(flatten)]
    pub page: PageParams,
    pub status: Option<String>,
}

pub async fn list_ai_spend_reservations(
    State(state): State<Arc<AppState>>,
    Path(balance_id): Path<String>,
    Query(query): Query<ReservationHistoryQuery>,
) -> Result<Json<Vec<myso_indexer_alt_social_schema::models::AiSpendReservationRow>>, SocialError> {
    const STATUSES: [&str; 4] = ["reserved", "captured", "cancelled", "expired"];
    if let Some(status) = query.status.as_deref() {
        if !STATUSES.contains(&status) {
            return Err(SocialError::bad_request(format!(
                "invalid reservation status '{status}'"
            )));
        }
    }
    let rows = state
        .reader
        .list_ai_spend_reservations(&balance_id, query.status.as_deref(), query.page.limit())
        .await?;
    Ok(Json(rows))
}

pub async fn get_ai_spend_reservation(
    State(state): State<Arc<AppState>>,
    Path((balance_id, reservation_nonce)): Path<(String, i64)>,
) -> Result<Json<myso_indexer_alt_social_schema::models::AiSpendReservationRow>, SocialError> {
    state
        .reader
        .get_ai_spend_reservation(&balance_id, reservation_nonce)
        .await?
        .ok_or_else(|| {
            SocialError::not_found(format!(
                "AI spend reservation '{balance_id}:{reservation_nonce}'"
            ))
        })
        .map(Json)
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

pub async fn get_ai_credit_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<myso_indexer_alt_social_schema::models::AiCreditConfigRow>, SocialError> {
    state
        .reader
        .get_ai_credit_config()
        .await?
        .ok_or_else(|| SocialError::not_found("AI credit config"))
        .map(Json)
}
