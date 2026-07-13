// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams};

fn check_spot_oracle_sync_secret(headers: &HeaderMap) -> Result<(), SocialError> {
    if let Ok(secret) = std::env::var("SPOT_ORACLE_SYNC_SECRET") {
        let provided = headers
            .get("x-spot-oracle-sync-secret")
            .and_then(|v| v.to_str().ok());
        if provided != Some(secret.as_str()) {
            return Err(SocialError::bad_request("invalid spot oracle sync secret"));
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct PendingSpotPostsQuery {
    pub limit: Option<i64>,
    /// Cursor = last `created_at` (ms) from the previous page.
    pub cursor: Option<i64>,
}

/// Secret-gated endpoint consumed by the SPoT oracle PostPoller.
/// Returns posts with `enable_spot = true` and `spot_id IS NULL`.
pub async fn list_pending_spot_posts(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PendingSpotPostsQuery>,
    headers: HeaderMap,
) -> Result<Json<Vec<crate::reader::PendingSpotPostRow>>, SocialError> {
    check_spot_oracle_sync_secret(&headers)?;
    let limit = params.limit.unwrap_or(50).clamp(1, 500);
    let data = state
        .reader
        .list_pending_spot_posts(limit, params.cursor)
        .await?;
    Ok(Json(data))
}

pub async fn get_spot_record(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> Result<Json<crate::reader::SpotRecordResponse>, SocialError> {
    let record = state
        .reader
        .get_spot_record(&post_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("SPoT record '{}'", post_id)))?;
    Ok(Json(record))
}

pub async fn list_spot_bets(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SpotBetRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.list_spot_bets(&post_id, limit, offset).await?;
    Ok(Json(data))
}

pub async fn list_spot_payouts(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SpotTransferRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_spot_payouts(&post_id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn list_spot_refunds(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SpotTransferRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_spot_refunds(&post_id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn list_contested_spot_records(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SpotRecordResponse>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_contested_spot_records(limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_spot_configuration(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::SpotConfigInfo>, SocialError> {
    let config = state
        .reader
        .get_spot_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("SPoT configuration".to_string()))?;
    Ok(Json(config))
}

/// Mandatory SPoT betting route: post → claim → open market.
pub async fn get_spot_route(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> Result<Json<crate::reader::SpotRouteResponse>, SocialError> {
    let route = state
        .reader
        .get_spot_route(&post_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("post '{}'", post_id)))?;
    Ok(Json(route))
}

pub async fn list_spot_pending_creator_payouts(
    State(state): State<Arc<AppState>>,
    Path(creator): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SpotPendingCreatorPayoutRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_spot_pending_creator_payouts(&creator, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_spot_creator_stats(
    State(state): State<Arc<AppState>>,
    Path(creator): Path<String>,
) -> Result<Json<crate::reader::SpotCreatorStatsResponse>, SocialError> {
    let stats = state.reader.get_spot_creator_stats(&creator).await?;
    Ok(Json(stats))
}

pub async fn list_expired_spot_creator_payouts(
    State(state): State<Arc<AppState>>,
    Path(market_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SpotPendingCreatorPayoutRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_expired_spot_creator_payouts(&market_id, limit, offset)
        .await?;
    Ok(Json(data))
}
