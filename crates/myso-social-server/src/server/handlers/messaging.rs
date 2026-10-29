// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::AppState;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

fn pagination(query: &PaginationQuery) -> (i64, i64) {
    let limit = query.limit.unwrap_or(50).clamp(1, 200);
    let offset = query.offset.unwrap_or(0).max(0);
    (limit, offset)
}

pub async fn get_messaging_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::MessagingConfigInfo>, SocialError> {
    state
        .reader
        .get_messaging_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("Messaging configuration"))
        .map(Json)
}

pub async fn get_paid_message_history(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<Vec<crate::reader::PaidMessageEscrowInfo>>, SocialError> {
    let (limit, offset) = pagination(&query);
    state
        .reader
        .get_paid_message_escrows(&address, limit, offset)
        .await
        .map(Json)
}

pub async fn get_message_history(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<Vec<crate::reader::MessageDigestInfo>>, SocialError> {
    let (limit, offset) = pagination(&query);
    state
        .reader
        .get_message_digests(&address, limit, offset)
        .await
        .map(Json)
}

pub async fn get_agent_groups(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<Vec<crate::reader::MessagingAgentGroupInfo>>, SocialError> {
    let (limit, offset) = pagination(&query);
    state
        .reader
        .get_messaging_agent_groups(&organization_id, limit, offset)
        .await
        .map(Json)
}

#[derive(Debug, Deserialize)]
pub struct AgentRoomsQuery {
    pub principal: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_agent_rooms(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AgentRoomsQuery>,
) -> Result<Json<Vec<crate::reader::AgentRoomInfo>>, SocialError> {
    // An unscoped listing would expose every principal's rooms; require a filter.
    let principal = query
        .principal
        .as_deref()
        .filter(|p| !p.is_empty())
        .ok_or_else(|| SocialError::bad_request("principal query parameter is required"))?;
    let (limit, offset) = pagination(&PaginationQuery {
        limit: query.limit,
        offset: query.offset,
    });
    state
        .reader
        .get_agent_rooms(Some(principal), None, limit, offset)
        .await
        .map(Json)
}

pub async fn list_organization_rooms(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<Vec<crate::reader::AgentRoomInfo>>, SocialError> {
    let (limit, offset) = pagination(&query);
    state
        .reader
        .get_agent_rooms(None, Some(&organization_id), limit, offset)
        .await
        .map(Json)
}

pub async fn list_group_digests(
    State(state): State<Arc<AppState>>,
    Path(group_id): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<Vec<crate::reader::MessageDigestInfo>>, SocialError> {
    let (limit, offset) = pagination(&query);
    state
        .reader
        .get_group_digests(&group_id, limit, offset)
        .await
        .map(Json)
}

pub async fn get_agent_room_by_group(
    State(state): State<Arc<AppState>>,
    Path(group_id): Path<String>,
) -> Result<Json<crate::reader::AgentRoomInfo>, SocialError> {
    state
        .reader
        .get_agent_room_by_group(&group_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Agent room for group '{group_id}'")))
        .map(Json)
}

pub async fn get_messaging_revenue_summary(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<crate::reader::MessagingRevenueSummaryInfo>, SocialError> {
    state
        .reader
        .get_messaging_revenue_summary(&address)
        .await
        .map(Json)
}
