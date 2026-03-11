// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use myso_indexer_alt_social_schema::models::Profile;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams, ProfileQuery};

pub async fn latest_profiles(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ProfileQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    let total_count = state.reader.get_profile_count().await?;
    let total_pages = ((total_count as f64) / (limit as f64)).ceil() as i64;

    let profiles = state.reader.get_profiles(limit, offset).await?;

    Ok(Json(serde_json::json!({
        "profiles": profiles,
        "pagination": {
            "total": total_count,
            "limit": limit,
            "offset": offset,
            "page": page,
            "total_pages": total_pages
        }
    })))
}

pub async fn get_profile_by_address(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<Profile>, SocialError> {
    let profile = state
        .reader
        .get_profile_by_address(&address)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Profile for address '{}'", address)))?;
    Ok(Json(profile))
}

pub async fn get_profile_by_username(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> Result<Json<Profile>, SocialError> {
    let profile = state
        .reader
        .get_profile_by_username(&username)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Profile for username '{}'", username)))?;
    Ok(Json(profile))
}

pub async fn get_profile_posts(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PostBasicRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let posts = state
        .reader
        .get_profile_posts(&address, limit, offset)
        .await?;
    Ok(Json(posts))
}

pub async fn get_profile_events(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let events = state
        .reader
        .get_profile_events(&address, limit, offset)
        .await?;
    Ok(Json(events))
}

pub async fn get_profile_platform_memberships(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformMembershipRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let memberships = state
        .reader
        .get_profile_platform_memberships(&address, limit, offset)
        .await?;
    Ok(Json(memberships))
}

pub async fn get_profile_platform_events(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let (events, total) = state
        .reader
        .get_profile_platform_events(&address, limit, offset)
        .await?;
    Ok(Json(serde_json::json!({
        "events": events,
        "total": total
    })))
}

pub async fn get_profile_blocking_history(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::BlockedEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let history = state
        .reader
        .get_blocking_history(&address, limit, offset)
        .await?;
    Ok(Json(history))
}

pub async fn get_profile_badges(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileBadgeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let badges = state
        .reader
        .get_profile_badges(&address, limit, offset)
        .await?;
    Ok(Json(badges))
}

pub async fn get_profile_following(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<crate::reader::FollowsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let (profiles, pagination) = state.reader.get_following(&address, &query).await?;
    Ok(Json(serde_json::json!({
        "profiles": profiles,
        "pagination": pagination
    })))
}

pub async fn get_profile_followers(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<crate::reader::FollowsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let (profiles, pagination) = state.reader.get_followers(&address, &query).await?;
    Ok(Json(serde_json::json!({
        "profiles": profiles,
        "pagination": pagination
    })))
}

pub async fn get_profile_social_stats(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<crate::reader::FollowStatsRow>, SocialError> {
    let stats = state.reader.get_social_stats(&address).await?;
    Ok(Json(stats))
}

pub async fn get_profile_blocked(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::BlockedProfileRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let blocked = state
        .reader
        .get_blocked_profiles(&address, limit, offset)
        .await?;
    Ok(Json(blocked))
}

pub async fn get_profile_blocked_platforms(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::BlockedPlatformRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let platforms = state
        .reader
        .get_blocked_platforms(&address, limit, offset)
        .await?;
    Ok(Json(platforms))
}
