// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use myso_indexer_alt_social_schema::models::{MemoryAccountRow, SubAgentRow};
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::memory::SubAgentListResponse;

use super::super::{AppState, PageParams};

#[derive(Debug, Deserialize)]
pub struct SubAgentQuery {
    #[serde(default = "default_active_only")]
    pub active_only: bool,
    #[serde(flatten)]
    pub page: PageParams,
}

fn default_active_only() -> bool {
    true
}

pub async fn list_profile_sub_agents(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<SubAgentQuery>,
) -> Result<Json<SubAgentListResponse>, SocialError> {
    let result = state
        .reader
        .list_sub_agents(
            &address,
            query.active_only,
            query.page.limit(),
            query.page.offset(),
        )
        .await?;
    Ok(Json(result))
}

pub async fn get_profile_memory_account(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<MemoryAccountRow>, SocialError> {
    let account = state
        .reader
        .get_memory_account_by_owner(&address)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Memory account for '{}'", address)))?;
    Ok(Json(account))
}

pub async fn get_sub_agent(
    State(state): State<Arc<AppState>>,
    Path(derived_address): Path<String>,
) -> Result<Json<SubAgentRow>, SocialError> {
    let agent = state
        .reader
        .get_sub_agent(&derived_address)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Sub-agent '{}'", derived_address)))?;
    Ok(Json(agent))
}

pub async fn get_sub_agent_by_object_id(
    State(state): State<Arc<AppState>>,
    Path(agent_object_id): Path<String>,
) -> Result<Json<SubAgentRow>, SocialError> {
    let agent = state
        .reader
        .get_sub_agent_by_object_id(&agent_object_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Sub-agent object '{}'", agent_object_id)))?;
    Ok(Json(agent))
}

pub async fn list_sub_agent_children(
    State(state): State<Arc<AppState>>,
    Path(agent_object_id): Path<String>,
    Query(query): Query<SubAgentQuery>,
) -> Result<Json<Vec<SubAgentRow>>, SocialError> {
    let children = state
        .reader
        .list_sub_agent_children(
            &agent_object_id,
            query.active_only,
            query.page.limit(),
            query.page.offset(),
        )
        .await?;
    Ok(Json(children))
}
