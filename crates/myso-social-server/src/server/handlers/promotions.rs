// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams};

pub async fn list_promotions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotedPostRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let promotions = state.reader.list_promotions(limit, offset).await?;
    Ok(Json(promotions))
}

pub async fn get_promotion_views(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotionViewRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let views = state.reader.get_promotion_views(&id, limit, offset).await?;
    Ok(Json(views))
}

pub async fn get_promotion_stats(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PromotionStatsRow>, SocialError> {
    let stats = state
        .reader
        .get_promotion_stats(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Promotion '{}'", id)))?;
    Ok(Json(stats))
}

pub async fn get_promotion_time_series(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotionTimeSeriesRow>>, SocialError> {
    let limit = params.limit();
    let data = state.reader.get_promotion_time_series(&id, limit).await?;
    Ok(Json(data))
}

pub async fn get_promotion_hourly(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotionHourlyRow>>, SocialError> {
    let limit = params.limit();
    let data = state.reader.get_promotion_hourly(&id, limit).await?;
    Ok(Json(data))
}

pub async fn get_top_performing_promotions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotedPostRow>>, SocialError> {
    let limit = params.limit();
    let promotions = state.reader.get_top_performing_promotions(limit).await?;
    Ok(Json(promotions))
}

pub async fn get_spending_trends(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotionTimeSeriesRow>>, SocialError> {
    let limit = params.limit();
    let data = state.reader.get_spending_trends(limit).await?;
    Ok(Json(data))
}
