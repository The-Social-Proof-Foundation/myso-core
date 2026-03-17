// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, VestingEventsQuery, VestingPageParams, VestingWalletsQuery};

pub async fn list_vesting_wallets(
    State(state): State<Arc<AppState>>,
    Query(params): Query<VestingWalletsQuery>,
) -> Result<Json<crate::reader::VestingWalletsResponse>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let page = params.page();
    let response = state
        .reader
        .list_vesting_wallets(
            params.active.unwrap_or(false),
            params.owner_address.as_deref(),
            limit,
            offset,
            page,
        )
        .await?;
    Ok(Json(response))
}

pub async fn list_vesting_wallets_active(
    State(state): State<Arc<AppState>>,
    Query(params): Query<VestingPageParams>,
) -> Result<Json<crate::reader::VestingWalletsResponse>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let page = params.page();
    let response = state
        .reader
        .list_vesting_wallets(true, None, limit, offset, page)
        .await?;
    Ok(Json(response))
}

pub async fn get_vesting_wallet(
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<String>,
) -> Result<Json<crate::reader::VestingWalletWithStatus>, SocialError> {
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
    Query(params): Query<VestingPageParams>,
) -> Result<Json<crate::reader::VestingEventsResponse>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let page = params.page();
    let response = state
        .reader
        .get_vesting_wallet_events(&wallet_id, limit, offset, page)
        .await?;
    Ok(Json(response))
}

pub async fn get_vesting_claimable(
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<String>,
) -> Result<Json<crate::reader::ClaimableResponse>, SocialError> {
    let response = state
        .reader
        .get_vesting_claimable(&wallet_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Vesting wallet '{}'", wallet_id)))?;
    Ok(Json(response))
}

pub async fn get_user_vesting_wallets(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<VestingPageParams>,
) -> Result<Json<crate::reader::VestingWalletsResponse>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let page = params.page();
    let response = state
        .reader
        .get_user_vesting_wallets(&address, limit, offset, page)
        .await?;
    Ok(Json(response))
}

pub async fn list_vesting_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<VestingEventsQuery>,
) -> Result<Json<crate::reader::VestingEventsResponse>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let page = params.page();
    let response = state
        .reader
        .list_vesting_events(limit, offset, page, params.owner_address.as_deref())
        .await?;
    Ok(Json(response))
}

pub async fn get_vesting_analytics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::VestingAnalyticsResponse>, SocialError> {
    let analytics = state.reader.get_vesting_analytics().await?;
    Ok(Json(analytics))
}

pub async fn get_vesting_leaderboard(
    State(state): State<Arc<AppState>>,
    Query(params): Query<VestingPageParams>,
) -> Result<Json<crate::reader::VestingLeaderboardResponse>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let page = params.page();
    let response = state
        .reader
        .get_vesting_leaderboard(limit, offset, page)
        .await?;
    Ok(Json(response))
}
