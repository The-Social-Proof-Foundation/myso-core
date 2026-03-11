// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{
    AppState, RevenueLeaderboardQuery, RevenueQuery, SubscriptionAnalyticsQuery,
    TreasuryHistoryQuery,
};

pub async fn get_revenue_dashboard(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let data = state.reader.get_revenue_dashboard().await?;
    Ok(Json(data))
}

pub async fn get_revenue_leaderboard(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RevenueLeaderboardQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let min_revenue = params.min_revenue.unwrap_or(0);
    let leaderboard = state
        .reader
        .get_revenue_leaderboard(limit, min_revenue, params.revenue_source.as_deref())
        .await?;
    Ok(Json(serde_json::json!({
        "leaderboard": leaderboard,
        "min_revenue": min_revenue,
        "limit": limit
    })))
}

pub async fn get_revenue_chart_data(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RevenueQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let period = params.period.as_deref().unwrap_or("day");
    let points = params.points.unwrap_or(30);
    let end_date = params
        .end_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());
    let start_date = params
        .start_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| {
            end_date
                - match period {
                    "hour" => chrono::Duration::hours(points),
                    "day" => chrono::Duration::days(points),
                    "week" => chrono::Duration::weeks(points),
                    "month" => chrono::Duration::days(points * 30),
                    _ => chrono::Duration::days(points),
                }
        });
    let chart_data = state
        .reader
        .get_revenue_chart_data(
            params.creator_address.as_deref(),
            period,
            start_date,
            end_date,
            points,
        )
        .await?;
    Ok(Json(serde_json::json!({
        "chart_data": chart_data,
        "period": period,
        "start_date": start_date,
        "end_date": end_date
    })))
}

pub async fn get_unified_revenue(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RevenueQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let start_date = params
        .start_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc());
    let end_date = params
        .end_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc());
    let (records, total_count, total_amount) = state
        .reader
        .get_unified_revenue(
            params.creator_address.as_deref(),
            params.platform_address.as_deref(),
            params.revenue_source.as_deref(),
            params.revenue_type.as_deref(),
            params.content_id.as_deref(),
            params.content_type.as_deref(),
            start_date,
            end_date,
            limit,
            offset,
        )
        .await?;
    let total_pages = if limit > 0 {
        ((total_count + limit - 1) / limit).max(1)
    } else {
        1
    };
    let format_myso = |v: i64| format!("{:.9}", v as f64 / 1_000_000_000.0);
    Ok(Json(serde_json::json!({
        "revenue_records": records,
        "total_count": total_count,
        "total_amount": total_amount,
        "total_amount_formatted": format_myso(total_amount),
        "pagination": {
            "total": total_count,
            "limit": limit,
            "offset": offset,
            "page": (offset / limit) + 1,
            "total_pages": total_pages
        }
    })))
}

pub async fn get_creator_revenue_stats(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let stats = state
        .reader
        .get_creator_revenue_stats(&address)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Creator '{}'", address)))?;
    Ok(Json(stats))
}

pub async fn get_platform_revenue_stats(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let stats = state.reader.get_platform_revenue_stats(&address).await?;
    Ok(Json(stats))
}

pub async fn get_treasury_current(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let treasury = state
        .reader
        .get_current_treasury()
        .await?
        .ok_or_else(|| SocialError::not_found("Treasury".to_string()))?;
    Ok(Json(treasury))
}

pub async fn get_treasury_history(
    State(state): State<Arc<AppState>>,
    Query(params): Query<TreasuryHistoryQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit.unwrap_or(100).min(100);
    let history = state.reader.get_treasury_history(limit).await?;
    Ok(Json(serde_json::json!({
        "history": history,
        "count": history.len()
    })))
}

pub async fn get_subscription_analytics(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SubscriptionAnalyticsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let end_date = params
        .end_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());
    let start_date = params
        .start_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| end_date - chrono::Duration::days(30));
    let analytics = state
        .reader
        .get_subscription_analytics(
            params.service_id.as_deref(),
            params.profile_owner.as_deref(),
            start_date,
            end_date,
        )
        .await?;
    Ok(Json(serde_json::json!({
        "analytics": analytics,
        "period_start": start_date,
        "period_end": end_date
    })))
}

pub async fn get_service_performance(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SubscriptionAnalyticsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let services = state
        .reader
        .get_service_performance(params.profile_owner.as_deref())
        .await?;
    Ok(Json(serde_json::json!({
        "total_count": services.len() as i64,
        "services": services
    })))
}
