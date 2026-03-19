// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams, PlatformsQuery};

pub async fn list_platforms(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PlatformsQuery>,
) -> Result<Json<Vec<crate::reader::PlatformRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = params.offset.unwrap_or_else(|| (page - 1) * limit);
    let approved_only = params.approved.unwrap_or(false);
    let governance = params.governance;
    let platforms = state
        .reader
        .list_platforms(approved_only, governance, limit, offset)
        .await?;
    Ok(Json(platforms))
}

pub async fn list_platforms_approved(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let platforms = state.reader.list_platforms(true, None, limit, offset).await?;
    Ok(Json(platforms))
}

pub async fn get_platform_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PlatformRow>, SocialError> {
    let platform = state
        .reader
        .get_platform_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Platform '{}'", id)))?;
    Ok(Json(platform))
}

pub async fn get_platform_moderators(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformModeratorRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let moderators = state
        .reader
        .get_platform_moderators(&id, limit, offset)
        .await?;
    Ok(Json(moderators))
}

pub async fn get_platform_approval(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PlatformApprovalRow>, SocialError> {
    let approval = state
        .reader
        .get_platform_approval(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Platform '{}'", id)))?;
    Ok(Json(approval))
}

pub async fn get_platform_blocked(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformBlockedProfileRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let blocked = state
        .reader
        .get_platform_blocked_profiles(&id, limit, offset)
        .await?;
    Ok(Json(blocked))
}

pub async fn get_platform_members(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformMemberRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let members = state
        .reader
        .get_platform_members(&id, limit, offset)
        .await?;
    Ok(Json(members))
}

pub async fn check_platform_membership(
    State(state): State<Arc<AppState>>,
    Path((id, profile_address)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let is_member = state
        .reader
        .check_platform_membership(&id, &profile_address)
        .await?;
    Ok(Json(serde_json::json!({ "is_member": is_member })))
}

pub async fn get_platform_events(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let (events, total) = state.reader.get_platform_events(&id, limit, offset).await?;
    Ok(Json(serde_json::json!({
        "events": events,
        "total": total
    })))
}
