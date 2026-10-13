// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::PromotedPostSummaryRow;
use crate::reader::PromotionAnalyticsQuery;
use crate::reader::PromotionBudgetHistoryResponse;
use crate::reader::PromotionDetailRow;
use crate::reader::PromotionListQuery;
use crate::reader::PromotionPlatformBreakdownResponse;
use crate::reader::PromotionRollupRow;
use crate::reader::PromotionStatusHistoryResponse;

use super::super::{AppState, PageParams};

/// Campaign list with aggregated views, spend, and remaining budget.
///
/// Filters: `owner`, `profile_id`, `platform_id`, `post_id`, `status` (`active`,
/// `inactive`, `all`). Paged with `limit`/`offset`/`page`. The total for the same
/// filtered set is `campaign_count` on the summary endpoint.
pub async fn list_promotions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PromotionListQuery>,
) -> Result<Json<Vec<PromotedPostSummaryRow>>, SocialError> {
    let promotions = state.reader.list_campaigns(&params).await?;
    Ok(Json(promotions))
}

/// Advertiser-level rollup over the same filters as [`list_promotions`].
///
/// `campaign_count` is the pagination total for that filtered campaign list.
pub async fn get_promotion_rollup(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PromotionListQuery>,
) -> Result<Json<PromotionRollupRow>, SocialError> {
    let rollup = state.reader.get_promotion_rollup(&params).await?;
    Ok(Json(rollup))
}

/// Individual campaign detail, including budget reconciliation.
pub async fn get_promotion(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<PromotionDetailRow>, SocialError> {
    let promotion = state
        .reader
        .get_promotion_detail(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Promotion '{}'", id)))?;
    Ok(Json(promotion))
}

/// Budget ledger for a campaign (`view_payment`, `withdrawal`) with the reconciliation check.
pub async fn get_promotion_budget_events(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PromotionAnalyticsQuery>,
) -> Result<Json<PromotionBudgetHistoryResponse>, SocialError> {
    let history = state
        .reader
        .get_promotion_budget_history(&id, &params)
        .await?;
    Ok(Json(history))
}

/// Activation/deactivation history for a campaign.
pub async fn get_promotion_status_events(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PromotionAnalyticsQuery>,
) -> Result<Json<PromotionStatusHistoryResponse>, SocialError> {
    let history = state
        .reader
        .get_promotion_status_history(&id, &params)
        .await?;
    Ok(Json(history))
}

/// Confirmed views for a campaign split by platform, over a resolved window.
pub async fn get_promotion_platform_breakdown(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PromotionAnalyticsQuery>,
) -> Result<Json<PromotionPlatformBreakdownResponse>, SocialError> {
    let breakdown = state
        .reader
        .get_promotion_platform_breakdown(&id, &params)
        .await?;
    Ok(Json(breakdown))
}

/// Individual confirmed views, newest first. `from`/`to` (epoch ms) and `platform_id` narrow the set.
pub async fn get_promotion_views(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PromotionAnalyticsQuery>,
) -> Result<Json<Vec<crate::reader::PromotionViewRow>>, SocialError> {
    let views = state.reader.get_promotion_views(&id, &params).await?;
    Ok(Json(views))
}

/// Aggregated delivery stats for a campaign. 404s when the campaign is not indexed.
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

/// Daily delivery series. `bucket` (`7d`/`30d`/`90d`/`180d`/`1y`) or explicit `from`/`to` (epoch ms).
pub async fn get_promotion_time_series(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PromotionAnalyticsQuery>,
) -> Result<Json<Vec<crate::reader::PromotionTimeSeriesRow>>, SocialError> {
    let data = state.reader.get_promotion_time_series(&id, &params).await?;
    Ok(Json(data))
}

/// Hour-of-day delivery profile over the resolved window.
pub async fn get_promotion_hourly(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PromotionAnalyticsQuery>,
) -> Result<Json<Vec<crate::reader::PromotionHourlyRow>>, SocialError> {
    let data = state.reader.get_promotion_hourly(&id, &params).await?;
    Ok(Json(data))
}

/// Campaigns ranked by confirmed views, carrying the same spend aggregates as the list.
pub async fn get_top_performing_promotions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<PromotedPostSummaryRow>>, SocialError> {
    let limit = params.limit();
    let promotions = state.reader.get_top_performing_promotions(limit).await?;
    Ok(Json(promotions))
}

/// Daily spending across campaigns, optionally narrowed to one advertiser or platform.
pub async fn get_spending_trends(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PromotionAnalyticsQuery>,
) -> Result<Json<Vec<crate::reader::PromotionTimeSeriesRow>>, SocialError> {
    let data = state.reader.get_spending_trends(&params).await?;
    Ok(Json(data))
}
