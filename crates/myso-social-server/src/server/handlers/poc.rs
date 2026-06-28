// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams};

pub async fn list_poc_badges(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocBadgeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let badges = state.reader.list_poc_badges(limit, offset).await?;
    Ok(Json(badges))
}

pub async fn get_poc_badge_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PocBadgeRow>, SocialError> {
    let badge = state
        .reader
        .get_poc_badge_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("PoC badge '{}'", id)))?;
    Ok(Json(badge))
}

pub async fn list_poc_revenue_redirections(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocRevenueRedirectionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let redirections = state
        .reader
        .list_poc_revenue_redirections(limit, offset)
        .await?;
    Ok(Json(redirections))
}

pub async fn list_poc_analysis_results(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocAnalysisResultRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let results = state
        .reader
        .list_poc_analysis_results(limit, offset)
        .await?;
    Ok(Json(results))
}

pub async fn list_poc_disputes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocDisputeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let disputes = state.reader.list_poc_disputes(limit, offset).await?;
    Ok(Json(disputes))
}

pub async fn get_poc_dispute_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PocDisputeRow>, SocialError> {
    let dispute = state
        .reader
        .get_poc_dispute_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("PoC dispute '{}'", id)))?;
    Ok(Json(dispute))
}

pub async fn get_poc_dispute_votes(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocDisputeVoteRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let votes = state
        .reader
        .get_poc_dispute_votes(&id, limit, offset)
        .await?;
    Ok(Json(votes))
}

pub async fn get_poc_analytics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let analytics = state.reader.get_poc_analytics().await?;
    Ok(Json(analytics))
}

pub async fn get_poc_configuration(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::PocConfigRow>, SocialError> {
    let config = state
        .reader
        .get_poc_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("PoC configuration".to_string()))?;
    Ok(Json(config))
}

pub async fn get_poc_beneficiary_vault_by_vault_id(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
) -> Result<Json<crate::reader::PocBeneficiaryVaultRow>, SocialError> {
    let row = state
        .reader
        .get_poc_beneficiary_vault_by_vault_id(&vault_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("PoC beneficiary vault '{vault_id}'")))?;
    Ok(Json(row))
}

pub async fn get_poc_beneficiary_vault_by_beneficiary(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<crate::reader::PocBeneficiaryVaultRow>, SocialError> {
    let row = state
        .reader
        .get_poc_beneficiary_vault_by_beneficiary_address(&address)
        .await?
        .ok_or_else(|| {
            SocialError::not_found(format!("PoC beneficiary vault for beneficiary '{address}'"))
        })?;
    Ok(Json(row))
}

pub async fn list_poc_beneficiary_vault_coin_balances(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
) -> Result<Json<Vec<crate::reader::PocVaultCoinBalanceRow>>, SocialError> {
    let rows = state
        .reader
        .list_poc_beneficiary_vault_coin_balances(&vault_id)
        .await?;
    Ok(Json(rows))
}

pub async fn list_poc_vault_deposits(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocVaultDepositRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let rows = state
        .reader
        .list_poc_vault_deposits_for_vault(&vault_id, limit, offset)
        .await?;
    Ok(Json(rows))
}

pub async fn list_poc_vault_claims(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocVaultClaimRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let rows = state
        .reader
        .list_poc_vault_claims_for_vault(&vault_id, limit, offset)
        .await?;
    Ok(Json(rows))
}

#[derive(Debug, Deserialize)]
pub struct PocUsernameBeneficiaryQuery {
    pub status: Option<i16>,
    #[serde(flatten)]
    pub page: PageParams,
}

pub async fn list_poc_username_beneficiaries(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PocUsernameBeneficiaryQuery>,
) -> Result<Json<Vec<myso_indexer_alt_social_schema::models::PocUsernameBeneficiaryRow>>, SocialError>
{
    let limit = params.page.limit();
    let offset = params.page.offset();
    let rows = state
        .reader
        .list_poc_username_beneficiaries(params.status, limit, offset)
        .await?;
    Ok(Json(rows))
}

pub async fn get_poc_username_beneficiary_by_username(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> Result<Json<myso_indexer_alt_social_schema::models::PocUsernameBeneficiaryRow>, SocialError> {
    let row = state
        .reader
        .get_poc_username_beneficiary_by_username(&username)
        .await?
        .ok_or_else(|| {
            SocialError::not_found(format!("PoC username beneficiary for username '{username}'"))
        })?;
    Ok(Json(row))
}

pub async fn get_poc_username_beneficiary_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<myso_indexer_alt_social_schema::models::PocUsernameBeneficiaryRow>, SocialError> {
    let row = state
        .reader
        .get_poc_username_beneficiary_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("PoC username beneficiary '{id}'")))?;
    Ok(Json(row))
}
