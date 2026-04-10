// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, MyDataQuery};

pub async fn list_mydata(
    State(state): State<Arc<AppState>>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataBasic>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_mydata(
            limit,
            offset,
            params.creator.as_deref(),
            params.media_type.as_deref(),
            params.platform_id.as_deref(),
            params.sort_by.as_deref(),
        )
        .await?;
    Ok(Json(data))
}

pub async fn get_mydata_configuration(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::MyDataConfigInfo>, SocialError> {
    let config = state
        .reader
        .get_mydata_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("MyData configuration".to_string()))?;
    Ok(Json(config))
}

pub async fn get_popular_mydata(
    State(state): State<Arc<AppState>>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataBasic>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state.reader.get_popular_mydata(limit, offset).await?;
    Ok(Json(data))
}

pub async fn get_mydata_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::MyDataBasic>, SocialError> {
    let data = state
        .reader
        .get_mydata_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("MyData '{}'", id)))?;
    Ok(Json(data))
}

pub async fn get_mydata_purchases(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::PurchaseInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .get_mydata_purchases(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_mydata_subscriptions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::SubscriptionInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .get_mydata_subscriptions(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_mydata_revenue(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::RevenueInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(30).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state.reader.get_mydata_revenue(&id, limit, offset).await?;
    Ok(Json(data))
}

pub async fn get_mydata_access_logs(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::AccessLogInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(200);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .get_mydata_access_logs(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_creator_mydata(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataBasic>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state.reader.get_creator_mydata(&id, limit, offset).await?;
    Ok(Json(data))
}

pub async fn get_mydata_stats(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::MyDataStatsResponse>, SocialError> {
    let stats = state
        .reader
        .get_mydata_stats(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("MyData '{}'", id)))?;
    Ok(Json(stats))
}

pub async fn get_mydata_revenue_timeline(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<crate::reader::DailyRevenue>>, SocialError> {
    let data = state.reader.get_mydata_revenue_timeline(&id).await?;
    Ok(Json(data))
}

pub async fn get_mydata_access_analytics(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<crate::reader::AccessAnalytics>>, SocialError> {
    let data = state.reader.get_mydata_access_analytics(&id).await?;
    Ok(Json(data))
}

pub async fn list_mydata_query_broad_pools(
    State(state): State<Arc<AppState>>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataQueryBroadPoolInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_mydata_query_broad_pools(limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn list_mydata_query_sub_pools_for_broad_pool(
    State(state): State<Arc<AppState>>,
    Path(pool_id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataQuerySubPoolInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_mydata_query_sub_pools_for_broad_pool(&pool_id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn list_mydata_query_sub_pools_for_mydata_listing(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataQuerySubPoolInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_mydata_query_sub_pools_for_listing(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn list_mydata_query_listings_for_sub_pool(
    State(state): State<Arc<AppState>>,
    Path(sub_pool_id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataQueryListingSubPoolInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_mydata_query_listings_for_sub_pool(&sub_pool_id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_mydata_query_snapshot_anchor(
    State(state): State<Arc<AppState>>,
    Path(snapshot_id): Path<String>,
) -> Result<Json<crate::reader::MyDataQuerySnapshotAnchorInfo>, SocialError> {
    let data = state
        .reader
        .get_mydata_query_snapshot_anchor(&snapshot_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("snapshot anchor '{}'", snapshot_id)))?;
    Ok(Json(data))
}

pub async fn get_mydata_query_distribution_round(
    State(state): State<Arc<AppState>>,
    Path(snapshot_id): Path<String>,
) -> Result<Json<crate::reader::MyDataQueryDistributionRoundInfo>, SocialError> {
    let data = state
        .reader
        .get_mydata_query_distribution_round(&snapshot_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("distribution round '{}'", snapshot_id)))?;
    Ok(Json(data))
}

pub async fn list_mydata_query_distribution_rounds(
    State(state): State<Arc<AppState>>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataQueryDistributionRoundInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_mydata_query_distribution_rounds(limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_mydata_query_merkle_root(
    State(state): State<Arc<AppState>>,
    Path(snapshot_id): Path<String>,
) -> Result<Json<crate::reader::MyDataQueryMerkleRootInfo>, SocialError> {
    let data = state
        .reader
        .get_mydata_query_merkle_root(&snapshot_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("merkle root '{}'", snapshot_id)))?;
    Ok(Json(data))
}

pub async fn list_mydata_query_claims_for_snapshot(
    State(state): State<Arc<AppState>>,
    Path(snapshot_id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataQueryClaimInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(200);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_mydata_query_claims_for_snapshot(&snapshot_id, limit, offset)
        .await?;
    Ok(Json(data))
}
