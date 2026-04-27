// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::SptUserHoldingItem;

use super::super::{
    AppState, PageParams, SptPoolsQuery, SptReservationVolumeQuery, SptUserHoldingsQuery,
    TimeRangeParams,
};

pub async fn list_spt_pools(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SptPoolsQuery>,
) -> Result<Json<Vec<crate::reader::SptPoolRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_spt_pools(limit, offset, params.owner.as_deref(), params.token_type)
        .await?;
    Ok(Json(data))
}

pub async fn get_spt_pool_by_associated_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::SptPoolRow>, SocialError> {
    let pool = state
        .reader
        .get_spt_pool_by_associated_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("SPT pool for associated ID '{}'", id)))?;
    Ok(Json(pool))
}

pub async fn get_spt_popular(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptPoolRow>>, SocialError> {
    let limit = params.limit();
    let pools = state.reader.get_spt_popular(limit).await?;
    Ok(Json(pools))
}

pub async fn get_spt_user_holdings(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<SptUserHoldingsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    if params.include_reservations == Some(true) {
        let items = state
            .reader
            .get_spt_user_holdings_with_reservations(&address, limit, offset)
            .await?;
        Ok(Json(
            serde_json::to_value(items).map_err(|e| SocialError::internal(e.to_string()))?,
        ))
    } else {
        let items = state
            .reader
            .get_spt_user_holdings(&address, limit, offset)
            .await?;
        Ok(Json(
            serde_json::to_value(items).map_err(|e| SocialError::internal(e.to_string()))?,
        ))
    }
}

pub async fn get_spt_user_reservations(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<SptUserHoldingItem>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let reservations = state
        .reader
        .get_spt_user_reservations(&address, limit, offset)
        .await?;
    Ok(Json(reservations))
}

pub async fn get_spt_pool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::SptPoolRow>, SocialError> {
    let pool = state
        .reader
        .get_spt_pool(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("SPT pool '{}'", id)))?;
    Ok(Json(pool))
}

pub async fn get_spt_pool_transactions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptTransactionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_spt_transactions(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_spt_pool_holdings(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptHoldingRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.get_spt_holdings(&id, limit, offset).await?;
    Ok(Json(data))
}

pub async fn get_spt_pool_price_history(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptPriceHistoryRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_spt_price_history(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_spt_pool_revenue(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptRevenueRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.get_spt_revenue(&id, limit, offset).await?;
    Ok(Json(data))
}

pub async fn get_spt_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::SptExchangeConfigRow>, SocialError> {
    let config = state
        .reader
        .get_spt_exchange_config()
        .await?
        .ok_or_else(|| SocialError::not_found("SPT exchange configuration".to_string()))?;
    Ok(Json(config))
}

pub async fn get_spt_analytics_top_performers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, SocialError> {
    let data = state.reader.get_spt_analytics_top_performers().await?;
    Ok(Json(data))
}

pub async fn get_spt_portfolio_performance(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let data = state.reader.get_spt_portfolio_performance(&address).await?;
    Ok(Json(data))
}

pub async fn get_spt_creator_revenue_streams(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(time_range): Query<TimeRangeParams>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let from_ts = time_range
        .from
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
        .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(30));
    let to_ts = time_range
        .to
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
        .unwrap_or_else(chrono::Utc::now);
    let data = state
        .reader
        .get_spt_creator_revenue_streams(&address, from_ts, to_ts)
        .await?;
    Ok(Json(data))
}

pub async fn get_spt_market_sentiment(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let data = state.reader.get_spt_market_sentiment().await?;
    Ok(Json(data))
}

pub async fn get_spt_liquidity_profile(
    State(state): State<Arc<AppState>>,
    Path(pool_id): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let data = state.reader.get_spt_liquidity_profile(&pool_id).await?;
    Ok(Json(data))
}

pub async fn list_spt_reservation_pools(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let (pools, total) = state
        .reader
        .list_spt_reservation_pools(limit, offset)
        .await?;
    let total_pages = if limit > 0 {
        ((total + limit - 1) / limit).max(1)
    } else {
        1
    };
    Ok(Json(serde_json::json!({
        "data": pools,
        "pagination": {
            "page": (offset / limit) + 1,
            "limit": limit,
            "total": total,
            "total_pages": total_pages
        }
    })))
}

pub async fn get_spt_reservation_pool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::SptReservationPoolRow>, SocialError> {
    let pool = state
        .reader
        .get_spt_reservation_pool(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("SPT reservation pool '{}'", id)))?;
    Ok(Json(pool))
}

pub async fn list_spt_reservation_pool_reservations(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptReservationRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_spt_reservations(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_spt_reservation_pool_volume_history(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<SptReservationVolumeQuery>,
) -> Result<Json<Vec<crate::reader::SptReservationVolumeBucketRow>>, SocialError> {
    let trunc = match params.interval.as_deref() {
        Some(s) if s.eq_ignore_ascii_case("day") => "day",
        Some(s) if s.eq_ignore_ascii_case("hour") => "hour",
        None => "hour",
        Some(other) => {
            return Err(SocialError::bad_request(format!(
                "invalid interval '{other}', expected hour or day"
            )));
        }
    };
    let limit = params.limit.unwrap_or(168).min(500);
    let from = params
        .from
        .and_then(chrono::DateTime::from_timestamp_millis);
    let to = params.to.and_then(chrono::DateTime::from_timestamp_millis);
    let data = state
        .reader
        .get_spt_reservation_volume_history(&id, trunc, limit, from, to)
        .await?;
    Ok(Json(data))
}
