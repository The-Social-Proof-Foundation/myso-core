// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams, VestingWalletsQuery};

pub async fn list_vesting_wallets(
    State(state): State<Arc<AppState>>,
    Query(params): Query<VestingWalletsQuery>,
) -> Result<Json<Vec<crate::reader::VestingWalletRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params
        .offset
        .unwrap_or_else(|| (params.page.unwrap_or(1).max(1) - 1) * limit);
    let wallets = state
        .reader
        .list_vesting_wallets(
            params.active.unwrap_or(false),
            params.owner.as_deref(),
            limit,
            offset,
        )
        .await?;
    Ok(Json(wallets))
}

pub async fn list_vesting_wallets_active(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingWalletRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let wallets = state
        .reader
        .list_vesting_wallets(true, None, limit, offset)
        .await?;
    Ok(Json(wallets))
}

pub async fn get_vesting_wallet(
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<String>,
) -> Result<Json<crate::reader::VestingWalletRow>, SocialError> {
    let wallet = state
        .reader
        .get_vesting_wallet_by_id(&wallet_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Vesting wallet '{}'", wallet_id)))?;
    Ok(Json(wallet))
}

pub async fn get_vesting_wallet_events(
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let events = state
        .reader
        .get_vesting_wallet_events(&wallet_id, limit, offset)
        .await?;
    Ok(Json(events))
}

pub async fn get_vesting_claimable(
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let amount = state
        .reader
        .get_vesting_claimable(&wallet_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Vesting wallet '{}'", wallet_id)))?;
    Ok(Json(serde_json::json!({ "claimable": amount })))
}

pub async fn get_user_vesting_wallets(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingWalletRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let wallets = state
        .reader
        .get_user_vesting_wallets(&address, limit, offset)
        .await?;
    Ok(Json(wallets))
}

pub async fn list_vesting_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let events = state.reader.list_vesting_events(limit, offset).await?;
    Ok(Json(events))
}

pub async fn get_vesting_analytics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let analytics = state.reader.get_vesting_analytics().await?;
    Ok(Json(analytics))
}

pub async fn get_vesting_leaderboard(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingWalletRow>>, SocialError> {
    let limit = params.limit();
    let wallets = state.reader.get_vesting_leaderboard(limit).await?;
    Ok(Json(wallets))
}
