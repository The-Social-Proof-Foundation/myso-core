// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams};

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
