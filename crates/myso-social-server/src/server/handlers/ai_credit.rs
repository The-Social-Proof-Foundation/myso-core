// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Extension, Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::ai_credit::{AiCreditBalanceResponse, IngestUsageLineRequest};

use super::super::auth::WalletAuthContext;
use super::super::{page_params, AppState, PageParams};

fn same_address(left: &str, right: &str) -> bool {
    left.trim_start_matches("0x")
        .eq_ignore_ascii_case(right.trim_start_matches("0x"))
}

async fn require_balance_owner(
    state: &AppState,
    balance_id: &str,
    auth: Option<&WalletAuthContext>,
) -> Result<(), SocialError> {
    if let Some(auth) = auth {
        let balance = state
            .reader
            .get_ai_credit_balance_by_owner(&auth.sender_address)
            .await?;
        if !balance.is_some_and(|balance| balance.balance_id.eq_ignore_ascii_case(balance_id)) {
            return Err(SocialError::forbidden(
                "AI credit data is outside the authenticated wallet scope",
            ));
        }
    }
    Ok(())
}

pub async fn get_profile_ai_credit_balance(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    auth: Option<Extension<WalletAuthContext>>,
) -> Result<Json<AiCreditBalanceResponse>, SocialError> {
    if let Some(auth) = auth.as_ref() {
        if !same_address(&auth.sender_address, &address) {
            return Err(SocialError::forbidden(
                "AI credit data is outside the authenticated wallet scope",
            ));
        }
    }
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
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

impl ReservationHistoryQuery {
    fn page_params(&self) -> PageParams {
        page_params(self.limit, self.offset, self.page)
    }
}

pub async fn list_ai_spend_reservations(
    State(state): State<Arc<AppState>>,
    Path(balance_id): Path<String>,
    Query(query): Query<ReservationHistoryQuery>,
    auth: Option<Extension<WalletAuthContext>>,
) -> Result<Json<Vec<myso_indexer_alt_social_schema::models::AiSpendReservationRow>>, SocialError> {
    require_balance_owner(&state, &balance_id, auth.as_ref().map(|auth| &auth.0)).await?;
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
        .list_ai_spend_reservations(
            &balance_id,
            query.status.as_deref(),
            query.page_params().limit(),
            query.page_params().offset(),
        )
        .await?;
    Ok(Json(rows))
}

pub async fn get_ai_spend_reservation(
    State(state): State<Arc<AppState>>,
    Path((balance_id, reservation_nonce)): Path<(String, i64)>,
    auth: Option<Extension<WalletAuthContext>>,
) -> Result<Json<myso_indexer_alt_social_schema::models::AiSpendReservationRow>, SocialError> {
    require_balance_owner(&state, &balance_id, auth.as_ref().map(|auth| &auth.0)).await?;
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
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

impl UsageHistoryQuery {
    fn page_params(&self) -> PageParams {
        page_params(self.limit, self.offset, self.page)
    }
}

pub async fn list_ai_credit_usage_history(
    State(state): State<Arc<AppState>>,
    Path(balance_id): Path<String>,
    Query(query): Query<UsageHistoryQuery>,
    auth: Option<Extension<WalletAuthContext>>,
) -> Result<Json<Vec<myso_indexer_alt_social_schema::models::AiCreditUsageLineRow>>, SocialError> {
    require_balance_owner(&state, &balance_id, auth.as_ref().map(|auth| &auth.0)).await?;
    let lines = state
        .reader
        .list_ai_credit_usage_lines(
            &balance_id,
            query.page_params().limit(),
            query.page_params().offset(),
        )
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
