// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams};

pub async fn check_social_graph_following(
    State(state): State<Arc<AppState>>,
    Path((follower, following)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let (is_following, following_back) =
        state.reader.check_following(&follower, &following).await?;
    Ok(Json(serde_json::json!({
        "is_following": is_following,
        "following_back": following_back
    })))
}

pub async fn get_social_graph_chart_data(
    State(state): State<Arc<AppState>>,
    Query(query): Query<crate::reader::SocialGraphChartQuery>,
) -> Result<Json<crate::reader::SocialGraphChartData>, SocialError> {
    let data = state.reader.get_social_graph_chart_data(&query).await?;
    Ok(Json(data))
}

pub async fn check_profile_blocked(
    State(state): State<Arc<AppState>>,
    Path((blocker, blocked)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let is_blocked = state
        .reader
        .check_profile_blocked(&blocker, &blocked)
        .await?;
    Ok(Json(serde_json::json!({ "is_blocked": is_blocked })))
}

pub async fn check_platform_blocked(
    State(state): State<Arc<AppState>>,
    Path((profile, platform)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let is_blocked = state
        .reader
        .check_platform_blocked(&profile, &platform)
        .await?;
    Ok(Json(serde_json::json!({ "is_blocked": is_blocked })))
}

pub async fn list_badges(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileBadgeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let badges = state.reader.list_badges(limit, offset).await?;
    Ok(Json(badges))
}

pub async fn get_badge_by_id(
    State(state): State<Arc<AppState>>,
    Path(badge_id): Path<String>,
) -> Result<Json<crate::reader::ProfileBadgeRow>, SocialError> {
    let badge = state
        .reader
        .get_badge_by_id(&badge_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Badge '{}'", badge_id)))?;
    Ok(Json(badge))
}
