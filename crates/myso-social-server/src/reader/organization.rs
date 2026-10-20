// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use myso_indexer_alt_social_reader::{
    get_agentic_organization_for_db, get_organization_leaderboard_for_db,
    get_organization_statistics_for_db, list_agentic_organizations_by_owner_for_db,
    org_type_from_slug, organization_categories, OrganizationCategoryInfo,
    OrganizationLeaderboardResult, OrganizationLeaderboardSort, OrganizationStatistics,
    OrganizationStatsWindow,
};
use myso_indexer_alt_social_schema::models::AgenticOrganizationRow;
use myso_pg_db::Db;
use serde::Serialize;

use crate::error::SocialError;

#[derive(Debug, Clone, Serialize)]
pub struct AgenticOrganizationListResponse {
    pub organizations: Vec<AgenticOrganizationRow>,
    pub total_count: i64,
}

pub(crate) async fn get_agentic_organization(
    db: &Db,
    organization_id: &str,
) -> Result<Option<AgenticOrganizationRow>, SocialError> {
    get_agentic_organization_for_db(db, organization_id)
        .await
        .map_err(Into::into)
}

pub(crate) async fn list_agentic_organizations_by_owner(
    db: &Db,
    principal_owner: &str,
    org_type: Option<i16>,
    active_only: bool,
    limit: i64,
    offset: i64,
) -> Result<AgenticOrganizationListResponse, SocialError> {
    let result = list_agentic_organizations_by_owner_for_db(
        db,
        principal_owner,
        org_type,
        active_only,
        limit,
        offset,
    )
    .await?;
    Ok(AgenticOrganizationListResponse {
        organizations: result.organizations,
        total_count: result.total_count,
    })
}

pub(crate) async fn get_organization_statistics(
    db: &Db,
    organization_id: &str,
    window: OrganizationStatsWindow,
) -> Result<Option<OrganizationStatistics>, SocialError> {
    get_organization_statistics_for_db(db, organization_id, window)
        .await
        .map_err(Into::into)
}

pub(crate) async fn get_organization_leaderboard(
    db: &Db,
    sort: OrganizationLeaderboardSort,
    org_type: i16,
    window: OrganizationStatsWindow,
    limit: i64,
    offset: i64,
) -> Result<OrganizationLeaderboardResult, SocialError> {
    get_organization_leaderboard_for_db(db, sort, org_type, window, limit, offset)
        .await
        .map_err(Into::into)
}

pub fn list_organization_categories() -> Vec<OrganizationCategoryInfo> {
    organization_categories()
}

pub fn parse_org_type(category: &str) -> Option<i16> {
    org_type_from_slug(category)
}

pub fn parse_leaderboard_sort(sort: &str) -> Option<OrganizationLeaderboardSort> {
    OrganizationLeaderboardSort::parse(sort)
}

pub fn parse_stats_window(window: &str) -> Option<OrganizationStatsWindow> {
    OrganizationStatsWindow::parse(window)
}
