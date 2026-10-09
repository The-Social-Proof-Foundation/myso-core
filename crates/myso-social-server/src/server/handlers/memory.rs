// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use myso_indexer_alt_social_schema::models::{MemoryAccountRow, SubAgentRow};
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::memory::SubAgentListResponse;

use super::super::{page_params, AppState, PageParams};

#[derive(Debug, Deserialize)]
pub struct SubAgentQuery {
    #[serde(default = "default_active_only")]
    pub active_only: bool,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

impl SubAgentQuery {
    fn page_params(&self) -> PageParams {
        page_params(self.limit, self.offset, self.page)
    }
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
            query.page_params().limit(),
            query.page_params().offset(),
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
            query.page_params().limit(),
            query.page_params().offset(),
        )
        .await?;
    Ok(Json(children))
}

pub async fn get_memory_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::MemoryConfigInfo>, SocialError> {
    state
        .reader
        .get_memory_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("Memory configuration"))
        .map(Json)
}

#[derive(Debug, Deserialize)]
pub struct SubAgentPnlQuery {
    /// Revoked and deactivated agents are included by default so their history is not erased.
    #[serde(default)]
    pub active_only: bool,
    /// Comma-separated windows: `days_7`, `days_30`, `days_180`, `days_365`, `all`.
    pub windows: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

/// P&L (MYSO base units, net of AI credit spend) for every sub-agent of a profile, plus a roll-up.
pub async fn list_profile_sub_agents_pnl(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<SubAgentPnlQuery>,
) -> Result<Json<myso_indexer_alt_social_reader::SubAgentPnlSummary>, SocialError> {
    let windows = super::profiles::parse_profile_pnl_windows(query.windows.as_deref())?;
    let page = page_params(query.limit, query.offset, query.page);
    let summary = state
        .reader
        .list_sub_agent_pnl(
            &address,
            query.active_only,
            &windows,
            page.limit().min(50),
            page.offset(),
        )
        .await?;
    Ok(Json(summary))
}

pub async fn get_sub_agent_pnl(
    State(state): State<Arc<AppState>>,
    Path(agent_object_id): Path<String>,
    Query(query): Query<SubAgentPnlQuery>,
) -> Result<Json<myso_indexer_alt_social_reader::SubAgentPnl>, SocialError> {
    let windows = super::profiles::parse_profile_pnl_windows(query.windows.as_deref())?;
    let pnl = state
        .reader
        .get_sub_agent_pnl(&agent_object_id, &windows)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Sub-agent object '{}'", agent_object_id)))?;
    Ok(Json(pnl))
}
