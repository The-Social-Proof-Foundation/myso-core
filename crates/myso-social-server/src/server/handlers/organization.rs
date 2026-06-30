// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use myso_indexer_alt_social_reader::OrganizationStatsWindow;
use myso_indexer_alt_social_schema::models::AgenticOrganizationRow;
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::organization::{
    parse_leaderboard_sort, parse_org_type, parse_stats_window, AgenticOrganizationListResponse,
};

use super::super::{AppState, PageParams};

#[derive(Debug, Deserialize)]
pub struct OrganizationListQuery {
    pub org_type: Option<i16>,
    #[serde(default = "default_active_only")]
    pub active_only: bool,
    #[serde(flatten)]
    pub page: PageParams,
}

fn default_active_only() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct OrganizationLeaderboardQuery {
    pub sort: String,
    pub category: String,
    pub window: Option<String>,
    #[serde(flatten)]
    pub page: PageParams,
}

#[derive(Debug, Deserialize)]
pub struct OrganizationStatisticsQuery {
    pub window: Option<String>,
}

pub async fn list_organization_categories(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<myso_indexer_alt_social_reader::OrganizationCategoryInfo>> {
    Json(state.reader.organization_categories())
}

pub async fn get_agentic_organization(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
) -> Result<Json<AgenticOrganizationRow>, SocialError> {
    let org = state
        .reader
        .get_agentic_organization(&organization_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Organization '{}'", organization_id)))?;
    Ok(Json(org))
}

pub async fn list_profile_organizations(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<OrganizationListQuery>,
) -> Result<Json<AgenticOrganizationListResponse>, SocialError> {
    let response = state
        .reader
        .list_agentic_organizations_by_owner(
            &address,
            query.org_type,
            query.active_only,
            query.page.limit(),
            query.page.offset(),
        )
        .await?;
    Ok(Json(response))
}

pub async fn get_organization_statistics(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
    Query(query): Query<OrganizationStatisticsQuery>,
) -> Result<Json<myso_indexer_alt_social_reader::OrganizationStatistics>, SocialError> {
    let window = query
        .window
        .as_deref()
        .and_then(parse_stats_window)
        .unwrap_or(OrganizationStatsWindow::All);
    let stats = state
        .reader
        .get_organization_statistics(&organization_id, window)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Organization '{}'", organization_id)))?;
    Ok(Json(stats))
}

pub async fn get_organization_leaderboard(
    State(state): State<Arc<AppState>>,
    Query(query): Query<OrganizationLeaderboardQuery>,
) -> Result<Json<myso_indexer_alt_social_reader::OrganizationLeaderboardResult>, SocialError> {
    let sort = parse_leaderboard_sort(&query.sort).ok_or_else(|| {
        SocialError::bad_request(format!("Unknown leaderboard sort '{}'", query.sort))
    })?;
    let org_type = parse_org_type(&query.category).ok_or_else(|| {
        SocialError::bad_request(format!(
            "Unknown organization category '{}'",
            query.category
        ))
    })?;
    let window = query
        .window
        .as_deref()
        .and_then(parse_stats_window)
        .unwrap_or(OrganizationStatsWindow::All);

    let response = state
        .reader
        .get_organization_leaderboard(
            sort,
            org_type,
            window,
            query.page.limit(),
            query.page.offset(),
        )
        .await?;
    Ok(Json(response))
}
