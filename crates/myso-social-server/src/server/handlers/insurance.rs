// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, InsurancePolicyFilters, PageParams};

pub async fn get_insurance_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::InsuranceConfigInfo>, SocialError> {
    let config = state
        .reader
        .get_insurance_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("Insurance configuration".to_string()))?;
    Ok(Json(config))
}

pub async fn list_insurance_vaults(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::InsuranceVaultRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.list_insurance_vaults(limit, offset).await?;
    Ok(Json(data))
}

pub async fn get_insurance_vault(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
) -> Result<Json<crate::reader::InsuranceVaultInfo>, SocialError> {
    let vault = state
        .reader
        .get_insurance_vault(&vault_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Insurance vault '{}'", vault_id)))?;
    Ok(Json(vault))
}

pub async fn list_insurance_vault_transactions(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::InsuranceVaultTransactionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_insurance_vault_transactions(&vault_id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_insurance_vault_exposures(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
) -> Result<Json<Vec<crate::reader::InsuranceVaultExposureRow>>, SocialError> {
    let data = state
        .reader
        .get_insurance_vault_exposures(&vault_id)
        .await?;
    Ok(Json(data))
}

pub async fn list_insurance_policies(
    State(state): State<Arc<AppState>>,
    Query(filters): Query<InsurancePolicyFilters>,
) -> Result<Json<Vec<crate::reader::InsurancePolicyRow>>, SocialError> {
    let limit = filters.limit.unwrap_or(20).min(100);
    let page = filters.page.unwrap_or(1).max(1);
    let offset = filters.offset.unwrap_or_else(|| (page - 1) * limit);
    let data = state
        .reader
        .list_insurance_policies(
            filters.insured.as_deref(),
            filters.market_id.as_deref(),
            filters.vault_id.as_deref(),
            filters.status,
            limit,
            offset,
        )
        .await?;
    Ok(Json(data))
}

pub async fn get_insurance_policy(
    State(state): State<Arc<AppState>>,
    Path(policy_id): Path<String>,
) -> Result<Json<crate::reader::InsurancePolicyInfo>, SocialError> {
    let policy = state
        .reader
        .get_insurance_policy(&policy_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Insurance policy '{}'", policy_id)))?;
    Ok(Json(policy))
}

pub async fn list_insurance_market_policies(
    State(state): State<Arc<AppState>>,
    Path(market_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::InsurancePolicyRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_insurance_market_policies(&market_id, limit, offset)
        .await?;
    Ok(Json(data))
}
