// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Enterprise workforce reads + internal ingest: org memory permissions, org roles,
//! AI-credit spend approvals, unified audit log, and relayer-pushed memory usage stats.

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    AiCreditSpendApprovalRow, AuditLogRow, MemoryUsageStatsRow, NewAiCreditSpendApproval,
    NewAuditLog, OrgMemoryPermissionRow, OrgRoleAssignmentRow, OrgRoleRow,
    APPROVAL_STATUS_REQUESTED,
};
use myso_indexer_alt_social_schema::schema::{
    ai_credit_balances, ai_credit_spend_approvals, audit_log, memory_usage_stats,
    org_memory_permissions, org_role_assignments, org_roles,
};
use myso_pg_db::Db;
use serde::{Deserialize, Serialize};

use crate::error::SocialError;

// ============================================================
// Org memory permissions + roles (reads)
// ============================================================

pub(crate) async fn list_org_memory_permissions(
    db: &Db,
    organization_id: &str,
    member: Option<&str>,
    active_only: bool,
) -> Result<Vec<OrgMemoryPermissionRow>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = org_memory_permissions::table
        .filter(org_memory_permissions::organization_id.eq(organization_id))
        .into_boxed();
    if let Some(member) = member {
        query = query.filter(org_memory_permissions::member_address.eq(member));
    }
    if active_only {
        query = query.filter(org_memory_permissions::active.eq(true));
    }
    query
        .order(org_memory_permissions::permission_kind.asc())
        .select(OrgMemoryPermissionRow::as_select())
        .load(&mut conn)
        .await
        .map_err(Into::into)
}

pub(crate) async fn list_org_roles(
    db: &Db,
    organization_id: &str,
) -> Result<Vec<OrgRoleRow>, SocialError> {
    let mut conn = db.connect().await?;
    org_roles::table
        .filter(org_roles::organization_id.eq(organization_id))
        .filter(org_roles::active.eq(true))
        .order(org_roles::role_name.asc())
        .select(OrgRoleRow::as_select())
        .load(&mut conn)
        .await
        .map_err(Into::into)
}

pub(crate) async fn list_org_role_assignments(
    db: &Db,
    organization_id: &str,
    member: Option<&str>,
    active_only: bool,
) -> Result<Vec<OrgRoleAssignmentRow>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = org_role_assignments::table
        .filter(org_role_assignments::organization_id.eq(organization_id))
        .into_boxed();
    if let Some(member) = member {
        query = query.filter(org_role_assignments::member_address.eq(member));
    }
    if active_only {
        query = query.filter(org_role_assignments::active.eq(true));
    }
    query
        .order(org_role_assignments::assigned_at_ms.desc())
        .select(OrgRoleAssignmentRow::as_select())
        .load(&mut conn)
        .await
        .map_err(Into::into)
}

// ============================================================
// Spend approvals
// ============================================================

pub(crate) async fn list_spend_approvals_by_owner(
    db: &Db,
    owner: &str,
    status: Option<&str>,
    agent_object_id: Option<&str>,
) -> Result<Vec<AiCreditSpendApprovalRow>, SocialError> {
    let mut conn = db.connect().await?;
    let balance_ids: Vec<String> = ai_credit_balances::table
        .filter(ai_credit_balances::principal_owner.eq(owner))
        .select(ai_credit_balances::balance_id)
        .load(&mut conn)
        .await?;
    if balance_ids.is_empty() {
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
    query
        .order(ai_credit_spend_approvals::updated_at.desc())
        .select(AiCreditSpendApprovalRow::as_select())
        .load(&mut conn)
        .await
        .map_err(Into::into)
}

#[derive(Debug, Clone, Deserialize)]
pub struct IngestApprovalRequest {
    pub balance_id: String,
    pub agent_object_id: String,
    pub requested_amount_mist: Option<i64>,
    pub threshold_mist: Option<i64>,
    pub organization_id: Option<String>,
}

/// Idempotent `requested` upsert from the oracle. Never downgrades an `approved` row;
/// repeated preflights refresh the requested amount to the max estimate seen.
pub(crate) async fn ingest_requested_approval(
    db: &Db,
    req: IngestApprovalRequest,
) -> Result<(), SocialError> {
    let mut conn = db.connect().await?;
    let now = chrono::Utc::now();
    diesel::insert_into(ai_credit_spend_approvals::table)
        .values(NewAiCreditSpendApproval {
            balance_id: req.balance_id,
            agent_object_id: req.agent_object_id,
            status: APPROVAL_STATUS_REQUESTED.to_string(),
            requested_amount_mist: req.requested_amount_mist,
            threshold_mist: req.threshold_mist,
            approval_nonce: None,
            max_amount_mist: None,
            expires_at_ms: None,
            approved_by: None,
            approved_by_agent_id: None,
            organization_id: req.organization_id,
            consumed_amount_mist: None,
            requested_at: now,
            updated_at: now,
            event_id: None,
        })
        .on_conflict((
            ai_credit_spend_approvals::balance_id,
            ai_credit_spend_approvals::agent_object_id,
        ))
        .do_update()
        .set((
            ai_credit_spend_approvals::requested_amount_mist.eq(diesel::dsl::sql::<
                diesel::sql_types::Nullable<diesel::sql_types::BigInt>,
            >(
                "GREATEST(COALESCE(ai_credit_spend_approvals.requested_amount_mist, 0), COALESCE(EXCLUDED.requested_amount_mist, 0))",
            )),
            ai_credit_spend_approvals::threshold_mist
                .eq(diesel::upsert::excluded(ai_credit_spend_approvals::threshold_mist)),
            ai_credit_spend_approvals::organization_id
                .eq(diesel::upsert::excluded(ai_credit_spend_approvals::organization_id)),
            ai_credit_spend_approvals::updated_at.eq(now),
            // Consumed/revoked/expired rows return to `requested` for a fresh ask;
            // live `approved` rows are left untouched.
            ai_credit_spend_approvals::status.eq(diesel::dsl::sql::<diesel::sql_types::Text>(
                "CASE WHEN ai_credit_spend_approvals.status = 'approved' THEN 'approved' ELSE 'requested' END",
            )),
        ))
        .execute(&mut conn)
        .await?;
    Ok(())
}

// ============================================================
// Audit log
// ============================================================

#[derive(Debug, Clone, Deserialize)]
pub struct AuditLogFilter {
    pub action: Option<String>,
    pub actor: Option<String>,
    pub target_type: Option<String>,
    pub source: Option<String>,
}

pub(crate) async fn list_audit_logs_for_org(
    db: &Db,
    organization_id: &str,
    filter: &AuditLogFilter,
    limit: i64,
    offset: i64,
) -> Result<Vec<AuditLogRow>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = audit_log::table
        .filter(audit_log::organization_id.eq(organization_id))
        .into_boxed();
    query = apply_audit_filter(query, filter);
    query
        .order(audit_log::time.desc())
        .limit(limit)
        .offset(offset)
        .select(AuditLogRow::as_select())
        .load(&mut conn)
        .await
        .map_err(Into::into)
}

pub(crate) async fn list_audit_logs_for_actor(
    db: &Db,
    actor: &str,
    filter: &AuditLogFilter,
    limit: i64,
    offset: i64,
) -> Result<Vec<AuditLogRow>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = audit_log::table
        .filter(audit_log::actor_address.eq(actor))
        .into_boxed();
    query = apply_audit_filter(query, filter);
    query
        .order(audit_log::time.desc())
        .limit(limit)
        .offset(offset)
        .select(AuditLogRow::as_select())
        .load(&mut conn)
        .await
        .map_err(Into::into)
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

#[derive(Debug, Clone, Deserialize)]
pub struct IngestAuditLogEntry {
    pub source: String,
    pub actor_address: String,
    pub actor_type: String,
    pub action: String,
    pub target_type: String,
    pub target_id: String,
    pub organization_id: Option<String>,
    pub account_id: Option<String>,
    pub prev_state: Option<serde_json::Value>,
    pub new_state: Option<serde_json::Value>,
    pub tx_digest: Option<String>,
    pub idempotency_key: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IngestAuditLogsRequest {
    pub entries: Vec<IngestAuditLogEntry>,
}

/// Batch audit ingest from off-chain services (oracle, relayers, scheduler).
/// Idempotent per entry via the partial unique index on `idempotency_key`.
pub(crate) async fn ingest_audit_logs(
    db: &Db,
    req: IngestAuditLogsRequest,
) -> Result<usize, SocialError> {
    let mut conn = db.connect().await?;
    let now = chrono::Utc::now();
    let mut inserted = 0usize;
    for entry in req.entries {
        let row = NewAuditLog {
            time: now,
            source: entry.source,
            actor_address: entry.actor_address,
            actor_type: entry.actor_type,
            action: entry.action,
            target_type: entry.target_type,
            target_id: entry.target_id,
            organization_id: entry.organization_id,
            account_id: entry.account_id,
            prev_state: entry.prev_state,
            new_state: entry.new_state,
            tx_digest: entry.tx_digest,
            event_id: None,
            idempotency_key: entry.idempotency_key,
            metadata: entry.metadata,
        };
        inserted += diesel::insert_into(audit_log::table)
            .values(&row)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .await?;
    }
    Ok(inserted)
}

// ============================================================
// Memory usage stats (relayer push)
// ============================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryUsageStatEntry {
    pub agent_object_id: String,
    pub organization_id: Option<String>,
    pub account_id: Option<String>,
    pub entries: i64,
    pub bytes: i64,
    pub org_shared_entries: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IngestMemoryUsageStatsRequest {
    pub stats: Vec<MemoryUsageStatEntry>,
}

pub(crate) async fn ingest_memory_usage_stats(
    db: &Db,
    req: IngestMemoryUsageStatsRequest,
) -> Result<usize, SocialError> {
    let mut conn = db.connect().await?;
    let now = chrono::Utc::now();
    let mut upserted = 0usize;
    for entry in req.stats {
        let row = MemoryUsageStatsRow {
            agent_object_id: entry.agent_object_id,
            organization_id: entry.organization_id,
            account_id: entry.account_id,
            entries: entry.entries,
            bytes: entry.bytes,
            org_shared_entries: entry.org_shared_entries,
            updated_at: now,
        };
        upserted += diesel::insert_into(memory_usage_stats::table)
            .values(&row)
            .on_conflict(memory_usage_stats::agent_object_id)
            .do_update()
            .set((
                memory_usage_stats::organization_id.eq(row.organization_id.clone()),
                memory_usage_stats::account_id.eq(row.account_id.clone()),
                memory_usage_stats::entries.eq(row.entries),
                memory_usage_stats::bytes.eq(row.bytes),
                memory_usage_stats::org_shared_entries.eq(row.org_shared_entries),
                memory_usage_stats::updated_at.eq(now),
            ))
            .execute(&mut conn)
            .await?;
    }
    Ok(upserted)
}
