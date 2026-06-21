// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::AgenticOrganizationRow;
use myso_indexer_alt_social_schema::schema::sub_agent_organizations;
use serde::Serialize;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, Serialize)]
pub struct AgenticOrganizationListResult {
    pub organizations: Vec<AgenticOrganizationRow>,
    pub total_count: i64,
}

pub async fn get_agentic_organization(
    conn: &mut Connection<'_>,
    organization_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<AgenticOrganizationRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = sub_agent_organizations::table
        .filter(sub_agent_organizations::organization_id.eq(organization_id))
        .select(AgenticOrganizationRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub async fn list_agentic_organizations_by_owner(
    conn: &mut Connection<'_>,
    principal_owner: &str,
    org_type: Option<i16>,
    active_only: bool,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<AgenticOrganizationListResult> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let mut count_query = sub_agent_organizations::table
        .filter(sub_agent_organizations::principal_owner.eq(principal_owner))
        .into_boxed();
    let mut list_query = sub_agent_organizations::table
        .filter(sub_agent_organizations::principal_owner.eq(principal_owner))
        .into_boxed();

    if active_only {
        count_query = count_query.filter(sub_agent_organizations::active.eq(true));
        list_query = list_query.filter(sub_agent_organizations::active.eq(true));
    }
    if let Some(org_type) = org_type {
        count_query = count_query.filter(sub_agent_organizations::org_type.eq(org_type));
        list_query = list_query.filter(sub_agent_organizations::org_type.eq(org_type));
    }

    let total_count = count_query.count().get_result::<i64>(conn).await?;
    let organizations = list_query
        .order(sub_agent_organizations::created_at_ms.desc())
        .limit(limit)
        .offset(offset)
        .select(AgenticOrganizationRow::as_select())
        .load(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(AgenticOrganizationListResult {
        organizations,
        total_count,
    })
}
