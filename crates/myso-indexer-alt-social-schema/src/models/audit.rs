// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Unified enterprise audit log: chain-derived rows are written in the same commit as
//! their domain-table updates; off-chain services push through the social-server internal
//! ingest endpoint. One shared action taxonomy so services cannot drift.

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::audit_log;

/// Audit row sources.
pub const AUDIT_SOURCE_CHAIN: &str = "chain";
pub const AUDIT_SOURCE_ORACLE: &str = "oracle";
pub const AUDIT_SOURCE_MEMORY_RELAYER: &str = "memory_relayer";
pub const AUDIT_SOURCE_WORKFLOW_RELAYER: &str = "workflow_relayer";
pub const AUDIT_SOURCE_SCHEDULER: &str = "scheduler";

/// Actor types.
pub const AUDIT_ACTOR_HUMAN: &str = "human";
pub const AUDIT_ACTOR_AGENT: &str = "agent";
pub const AUDIT_ACTOR_SERVICE: &str = "service";

/// Canonical audit action taxonomy. Serialized into `audit_log.action`; every producer
/// (indexer handlers and off-chain services) uses these strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    OrgMemoryGroupCreate,
    OrgMemoryGrant,
    OrgMemoryRevoke,
    OrgRoleDefine,
    OrgRoleAssign,
    OrgRoleRevoke,
    AgentBudgetChange,
    AgentBudgetDisable,
    SpendApprovalRequest,
    SpendApprovalApprove,
    SpendApprovalConsume,
    SpendApprovalRevoke,
    SpendApprovalExpire,
    SpendRejectedUnbilled,
    SettlementVoidResign,
    OrgCreate,
    OrgUpdate,
    OrgDeactivate,
    MemoryOrgWrite,
    MemoryOrgRecall,
    MemoryScopeDegraded,
    MemoryRestore,
    WorkflowItemCreate,
    WorkflowItemAction,
    WorkflowItemExpire,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditAction::OrgMemoryGroupCreate => "org_memory_group_create",
            AuditAction::OrgMemoryGrant => "org_memory_grant",
            AuditAction::OrgMemoryRevoke => "org_memory_revoke",
            AuditAction::OrgRoleDefine => "org_role_define",
            AuditAction::OrgRoleAssign => "org_role_assign",
            AuditAction::OrgRoleRevoke => "org_role_revoke",
            AuditAction::AgentBudgetChange => "agent_budget_change",
            AuditAction::AgentBudgetDisable => "agent_budget_disable",
            AuditAction::SpendApprovalRequest => "spend_approval_request",
            AuditAction::SpendApprovalApprove => "spend_approval_approve",
            AuditAction::SpendApprovalConsume => "spend_approval_consume",
            AuditAction::SpendApprovalRevoke => "spend_approval_revoke",
            AuditAction::SpendApprovalExpire => "spend_approval_expire",
            AuditAction::SpendRejectedUnbilled => "spend_rejected_unbilled",
            AuditAction::SettlementVoidResign => "settlement_void_resign",
            AuditAction::OrgCreate => "org_create",
            AuditAction::OrgUpdate => "org_update",
            AuditAction::OrgDeactivate => "org_deactivate",
            AuditAction::MemoryOrgWrite => "memory_org_write",
            AuditAction::MemoryOrgRecall => "memory_org_recall",
            AuditAction::MemoryScopeDegraded => "memory_scope_degraded",
            AuditAction::MemoryRestore => "memory_restore",
            AuditAction::WorkflowItemCreate => "workflow_item_create",
            AuditAction::WorkflowItemAction => "workflow_item_action",
            AuditAction::WorkflowItemExpire => "workflow_item_expire",
        }
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = audit_log)]
pub struct NewAuditLog {
    pub time: chrono::DateTime<chrono::Utc>,
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
    pub event_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = audit_log)]
pub struct AuditLogRow {
    pub id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
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
    pub event_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
