// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Enterprise workforce endpoints: org memory permissions, roles, spend approvals,
//! audit logs, and internal ingest for off-chain producers (oracle + memory relayer).

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use myso_indexer_alt_social_reader::OrganizationStatsWindow;
use myso_indexer_alt_social_schema::models::{
    AiCreditSpendApprovalRow, AuditLogRow, OrgMemoryPermissionRow, OrgRoleAssignmentRow,
    OrgRoleRow,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::enterprise::{
    AuditLogFilter, IngestApprovalRequest, IngestAuditLogsRequest, IngestMemoryUsageStatsRequest,
};

use super::super::{AppState, PageParams};

fn check_sync_secret(
    headers: &HeaderMap,
    header_name: &str,
    env_var: &str,
) -> Result<(), SocialError> {
    if let Ok(secret) = std::env::var(env_var) {
        let provided = headers.get(header_name).and_then(|v| v.to_str().ok());
        if provided != Some(secret.as_str()) {
            return Err(SocialError::bad_request("invalid sync secret"));
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct MemberQuery {
    pub member: Option<String>,
    #[serde(default = "default_active_only")]
    pub active_only: bool,
}

fn default_active_only() -> bool {
    true
}

pub async fn list_org_memory_permissions(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
    Query(query): Query<MemberQuery>,
) -> Result<Json<Vec<OrgMemoryPermissionRow>>, SocialError> {
    let rows = state
        .reader
        .list_org_memory_permissions(
            &organization_id,
            query.member.as_deref(),
            query.active_only,
        )
        .await?;
    Ok(Json(rows))
}

pub async fn list_org_roles(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
) -> Result<Json<Vec<OrgRoleRow>>, SocialError> {
    let rows = state.reader.list_org_roles(&organization_id).await?;
    Ok(Json(rows))
}

pub async fn list_org_role_assignments(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
    Query(query): Query<MemberQuery>,
) -> Result<Json<Vec<OrgRoleAssignmentRow>>, SocialError> {
    let rows = state
        .reader
        .list_org_role_assignments(
            &organization_id,
            query.member.as_deref(),
            query.active_only,
        )
        .await?;
    Ok(Json(rows))
}

#[derive(Debug, Deserialize)]
pub struct ApprovalsQuery {
    pub status: Option<String>,
    pub agent: Option<String>,
}

pub async fn list_profile_spend_approvals(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<ApprovalsQuery>,
) -> Result<Json<Vec<AiCreditSpendApprovalRow>>, SocialError> {
    let rows = state
        .reader
        .list_spend_approvals_by_owner(&address, query.status.as_deref(), query.agent.as_deref())
        .await?;
    Ok(Json(rows))
}

#[derive(Debug, Deserialize)]
pub struct SpendBreakdownQuery {
    pub window: Option<String>,
    pub limit: Option<i64>,
}

pub async fn list_org_spend_breakdown(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
    Query(query): Query<SpendBreakdownQuery>,
) -> Result<Json<Vec<myso_indexer_alt_social_reader::AgentSpendBreakdownEntry>>, SocialError> {
    let window = query
        .window
        .as_deref()
        .and_then(OrganizationStatsWindow::parse)
        .unwrap_or(OrganizationStatsWindow::All);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let rows = state
        .reader
        .list_agent_spend_breakdown(&organization_id, window, limit)
        .await?;
    Ok(Json(rows))
}

pub async fn list_org_spend_approvals(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
    Query(query): Query<ApprovalsQuery>,
) -> Result<Json<Vec<AiCreditSpendApprovalRow>>, SocialError> {
    let rows = state
        .reader
        .list_spend_approvals_by_org(
            &organization_id,
            query.status.as_deref(),
            query.agent.as_deref(),
            100,
        )
        .await?;
    Ok(Json(rows))
}

#[derive(Debug, Deserialize)]
pub struct AuditLogQuery {
    #[serde(flatten)]
    pub filter: AuditLogFilter,
    #[serde(flatten)]
    pub page: PageParams,
}

pub async fn list_org_audit_logs(
    State(state): State<Arc<AppState>>,
    Path(organization_id): Path<String>,
    Query(query): Query<AuditLogQuery>,
) -> Result<Json<Vec<AuditLogRow>>, SocialError> {
    let rows = state
        .reader
        .list_audit_logs_for_org(
            &organization_id,
            &query.filter,
            query.page.limit(),
            query.page.offset(),
        )
        .await?;
    Ok(Json(rows))
}

pub async fn list_profile_audit_logs(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<AuditLogQuery>,
) -> Result<Json<Vec<AuditLogRow>>, SocialError> {
    let rows = state
        .reader
        .list_audit_logs_for_actor(
            &address,
            &query.filter,
            query.page.limit(),
            query.page.offset(),
        )
        .await?;
    Ok(Json(rows))
}

// ==== Internal ingest (shared-secret gated) ====

/// Oracle upserts a `requested` approval row when preflight rejects an over-threshold spend.
pub async fn ingest_approval_internal(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<IngestApprovalRequest>,
) -> Result<Json<serde_json::Value>, SocialError> {
    check_sync_secret(&headers, "x-ai-credit-sync-secret", "AI_CREDIT_USAGE_SYNC_SECRET")?;
    state.reader.ingest_requested_approval(req).await?;
    Ok(Json(serde_json::json!({ "ok": true })))
}

/// Memory relayer pushes per-agent usage aggregates.
pub async fn ingest_memory_usage_stats_internal(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<IngestMemoryUsageStatsRequest>,
) -> Result<Json<serde_json::Value>, SocialError> {
    check_sync_secret(
        &headers,
        "x-memory-usage-sync-secret",
        "MEMORY_USAGE_SYNC_SECRET",
    )?;
    let upserted = state.reader.ingest_memory_usage_stats(req).await?;
    Ok(Json(serde_json::json!({ "ok": true, "upserted": upserted })))
}

/// Off-chain services push audit entries (idempotent per `idempotency_key`).
pub async fn ingest_audit_logs_internal(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<IngestAuditLogsRequest>,
) -> Result<Json<serde_json::Value>, SocialError> {
    check_sync_secret(&headers, "x-audit-sync-secret", "AUDIT_SYNC_SECRET")?;
    let inserted = state.reader.ingest_audit_logs(req).await?;
    Ok(Json(serde_json::json!({ "ok": true, "inserted": inserted })))
}
