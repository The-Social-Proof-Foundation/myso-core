// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Enterprise workforce reads: org memory permissions, roles, spend approvals,
//! audit log, and per-agent spend breakdown for org dashboards.

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    AiCreditAgentBudgetRow, AiCreditSpendApprovalRow, AuditLogRow, OrgMemoryPermissionRow,
    OrgRoleAssignmentRow, OrgRoleRow,
};
use myso_indexer_alt_social_schema::schema::{
    ai_credit_agent_budgets, ai_credit_balances, ai_credit_spend_approvals, audit_log,
    org_memory_permissions, org_role_assignments, org_roles,
};
use myso_pg_db::Connection;
use serde::{Deserialize, Serialize};

use crate::metrics::DbReaderMetrics;
use crate::org_stats::OrganizationStatsWindow;

/// Per-agent spend row for org dashboards (`usage_lines` ⋈ budgets ⋈ `sub_agents` ⋈ memory stats).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpendBreakdownEntry {
    pub agent_object_id: String,
    pub label: String,
    pub derived_address: String,
    pub spent_mist: i64,
    pub usage_events: i64,
    pub budget_mist: Option<i64>,
    pub require_approval_above_mist: Option<i64>,
    pub budget_enabled: bool,
    pub memory_entries: i64,
    pub memory_bytes: i64,
    pub org_shared_memory_entries: i64,
}

#[derive(Debug, QueryableByName)]
struct AgentSpendBreakdownRow {
    #[diesel(sql_type = diesel::sql_types::Text)]
    agent_object_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    label: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    derived_address: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    spent_mist: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    usage_events: i64,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
    budget_mist: Option<i64>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
    require_approval_above_mist: Option<i64>,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    budget_enabled: bool,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    memory_entries: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    memory_bytes: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    org_shared_memory_entries: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AuditLogFilter {
    pub action: Option<String>,
    pub actor: Option<String>,
    pub target_type: Option<String>,
    pub source: Option<String>,
}

pub async fn list_agent_spend_breakdown(
    conn: &mut Connection<'_>,
    organization_id: &str,
    window: OrganizationStatsWindow,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<AgentSpendBreakdownEntry>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let days = window.days_parameter();
    let limit = limit.clamp(1, 100);

    let query = r#"
        SELECT
            sa.agent_object_id,
            sa.label,
            sa.derived_address,
            COALESCE(SUM(u.amount_mist), 0)::bigint AS spent_mist,
            COUNT(u.id)::bigint AS usage_events,
            ab.budget_mist,
            ab.require_approval_above_mist,
            COALESCE(ab.enabled, false) AS budget_enabled,
            COALESCE(mus.entries, 0)::bigint AS memory_entries,
            COALESCE(mus.bytes, 0)::bigint AS memory_bytes,
            COALESCE(mus.org_shared_entries, 0)::bigint AS org_shared_memory_entries
        FROM sub_agents sa
        LEFT JOIN ai_credit_usage_lines u
            ON u.agent_object_id = sa.agent_object_id
           AND u.organization_id = sa.organization_id
           AND ($2::bigint < 0 OR u.created_at >= NOW() - ($2::bigint * INTERVAL '1 day'))
        LEFT JOIN ai_credit_agent_budgets ab
            ON ab.agent_object_id = sa.agent_object_id
        LEFT JOIN memory_usage_stats mus
            ON mus.agent_object_id = sa.agent_object_id
        WHERE sa.organization_id = $1
        GROUP BY
            sa.agent_object_id,
            sa.label,
            sa.derived_address,
            ab.budget_mist,
            ab.require_approval_above_mist,
            ab.enabled,
            mus.entries,
            mus.bytes,
            mus.org_shared_entries
        ORDER BY spent_mist DESC, sa.label ASC
        LIMIT $3
    "#;

    let rows: Vec<AgentSpendBreakdownRow> = diesel::sql_query(query)
        .bind::<diesel::sql_types::Text, _>(organization_id)
        .bind::<diesel::sql_types::BigInt, _>(days)
        .bind::<diesel::sql_types::BigInt, _>(limit)
        .load(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|row| AgentSpendBreakdownEntry {
            agent_object_id: row.agent_object_id,
            label: row.label,
            derived_address: row.derived_address,
            spent_mist: row.spent_mist,
            usage_events: row.usage_events,
            budget_mist: row.budget_mist,
            require_approval_above_mist: row.require_approval_above_mist,
            budget_enabled: row.budget_enabled,
            memory_entries: row.memory_entries,
            memory_bytes: row.memory_bytes,
            org_shared_memory_entries: row.org_shared_memory_entries,
        })
        .collect())
}

pub async fn list_org_memory_permissions(
    conn: &mut Connection<'_>,
    organization_id: &str,
    member: Option<&str>,
    active_only: bool,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<OrgMemoryPermissionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let mut query = org_memory_permissions::table
        .filter(org_memory_permissions::organization_id.eq(organization_id))
        .into_boxed();
    if let Some(member) = member {
        query = query.filter(org_memory_permissions::member_address.eq(member));
    }
    if active_only {
        query = query.filter(org_memory_permissions::active.eq(true));
    }
    let rows = query
        .order(org_memory_permissions::permission_kind.asc())
        .select(OrgMemoryPermissionRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub async fn list_org_roles(
    conn: &mut Connection<'_>,
    organization_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<OrgRoleRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = org_roles::table
        .filter(org_roles::organization_id.eq(organization_id))
        .filter(org_roles::active.eq(true))
        .order(org_roles::role_name.asc())
        .select(OrgRoleRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub async fn list_org_role_assignments(
    conn: &mut Connection<'_>,
    organization_id: &str,
    member: Option<&str>,
    active_only: bool,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<OrgRoleAssignmentRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let mut query = org_role_assignments::table
        .filter(org_role_assignments::organization_id.eq(organization_id))
        .into_boxed();
    if let Some(member) = member {
        query = query.filter(org_role_assignments::member_address.eq(member));
    }
    if active_only {
        query = query.filter(org_role_assignments::active.eq(true));
    }
    let rows = query
        .order(org_role_assignments::assigned_at_ms.desc())
        .select(OrgRoleAssignmentRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub async fn list_spend_approvals_by_owner(
    conn: &mut Connection<'_>,
    owner: &str,
    status: Option<&str>,
    agent_object_id: Option<&str>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<AiCreditSpendApprovalRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let balance_ids: Vec<String> = ai_credit_balances::table
        .filter(ai_credit_balances::principal_owner.eq(owner))
        .select(ai_credit_balances::balance_id)
        .load(conn)
        .await?;
    if balance_ids.is_empty() {
        metrics.requests_succeeded.inc();
        return Ok(Vec::new());
    }
    let mut query = ai_credit_spend_approvals::table
        .filter(ai_credit_spend_approvals::balance_id.eq_any(balance_ids))
        .into_boxed();
    if let Some(status) = status {
        query = query.filter(ai_credit_spend_approvals::status.eq(status.to_string()));
    }
    if let Some(agent) = agent_object_id {
        query = query.filter(ai_credit_spend_approvals::agent_object_id.eq(agent.to_string()));
    }
    let rows = query
        .order(ai_credit_spend_approvals::updated_at.desc())
        .select(AiCreditSpendApprovalRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub async fn list_spend_approvals_by_org(
    conn: &mut Connection<'_>,
    organization_id: &str,
    status: Option<&str>,
    agent_object_id: Option<&str>,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<AiCreditSpendApprovalRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let limit = limit.clamp(1, 100);
    let mut query = ai_credit_spend_approvals::table
        .filter(ai_credit_spend_approvals::organization_id.eq(organization_id))
        .into_boxed();
    if let Some(status) = status {
        query = query.filter(ai_credit_spend_approvals::status.eq(status.to_string()));
    }
    if let Some(agent) = agent_object_id {
        query = query.filter(ai_credit_spend_approvals::agent_object_id.eq(agent.to_string()));
    }
    let rows = query
        .order(ai_credit_spend_approvals::updated_at.desc())
        .limit(limit)
        .select(AiCreditSpendApprovalRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub async fn list_pending_spend_approvals_for_balance(
    conn: &mut Connection<'_>,
    balance_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<AiCreditSpendApprovalRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = ai_credit_spend_approvals::table
        .filter(ai_credit_spend_approvals::balance_id.eq(balance_id))
        .filter(
            ai_credit_spend_approvals::status.eq_any(["requested", "approved"]),
        )
        .order(ai_credit_spend_approvals::updated_at.desc())
        .select(AiCreditSpendApprovalRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub async fn get_agent_budget(
    conn: &mut Connection<'_>,
    agent_object_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<AiCreditAgentBudgetRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = ai_credit_agent_budgets::table
        .filter(ai_credit_agent_budgets::agent_object_id.eq(agent_object_id))
        .select(AiCreditAgentBudgetRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub async fn list_agent_budgets_for_balance(
    conn: &mut Connection<'_>,
    balance_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<AiCreditAgentBudgetRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = ai_credit_agent_budgets::table
        .filter(ai_credit_agent_budgets::balance_id.eq(balance_id))
        .order(ai_credit_agent_budgets::agent_object_id.asc())
        .select(AiCreditAgentBudgetRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

type BoxedAuditQuery<'a> = audit_log::BoxedQuery<'a, diesel::pg::Pg>;

fn apply_audit_filter<'a>(
    mut query: BoxedAuditQuery<'a>,
    filter: &AuditLogFilter,
) -> BoxedAuditQuery<'a> {
    if let Some(action) = &filter.action {
        query = query.filter(audit_log::action.eq(action.clone()));
    }
    if let Some(actor) = &filter.actor {
        query = query.filter(audit_log::actor_address.eq(actor.clone()));
    }
    if let Some(target_type) = &filter.target_type {
        query = query.filter(audit_log::target_type.eq(target_type.clone()));
    }
    if let Some(source) = &filter.source {
        query = query.filter(audit_log::source.eq(source.clone()));
    }
    query
}

pub async fn list_audit_logs_for_org(
    conn: &mut Connection<'_>,
    organization_id: &str,
    filter: &AuditLogFilter,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<AuditLogRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let limit = limit.clamp(1, 100);
    let offset = offset.max(0);
    let mut query = audit_log::table
        .filter(audit_log::organization_id.eq(organization_id))
        .into_boxed();
    query = apply_audit_filter(query, filter);
    let rows = query
        .order(audit_log::time.desc())
        .limit(limit)
        .offset(offset)
        .select(AuditLogRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub async fn list_audit_logs_for_actor(
    conn: &mut Connection<'_>,
    actor: &str,
    filter: &AuditLogFilter,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<AuditLogRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let limit = limit.clamp(1, 100);
    let offset = offset.max(0);
    let mut query = audit_log::table
        .filter(audit_log::actor_address.eq(actor))
        .into_boxed();
    query = apply_audit_filter(query, filter);
    let rows = query
        .order(audit_log::time.desc())
        .limit(limit)
        .offset(offset)
        .select(AuditLogRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}
