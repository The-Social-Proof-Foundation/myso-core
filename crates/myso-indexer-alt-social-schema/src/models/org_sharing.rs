// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Org memory share permissions and org roles (from `social_contracts::memory`
//! `OrgMemoryPermission*` / `OrgRole*` events).

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{org_memory_permissions, org_role_assignments, org_roles};

/// Fixed org permission bits (mirror of memory.move `ORG_PERM_*`).
pub const ORG_PERM_MEMORY_READ: i64 = 1;
pub const ORG_PERM_MEMORY_WRITE: i64 = 2;
pub const ORG_PERM_AGENT_MANAGER: i64 = 4;
pub const ORG_PERM_BUDGET_MANAGER: i64 = 8;
pub const ORG_PERM_SPEND_APPROVER: i64 = 16;
pub const ORG_PERM_DASHBOARD_VIEWER: i64 = 32;
pub const ORG_PERM_AUDITOR: i64 = 64;
pub const ORG_PERM_ALL: i64 = 127;

/// Built-in role names (mirror of memory.move built-in role masks).
pub const BUILTIN_ORG_ROLES: [&str; 6] = [
    "owner",
    "admin",
    "agent_manager",
    "finance_approver",
    "memory_administrator",
    "auditor",
];

pub fn is_builtin_org_role(name: &str) -> bool {
    BUILTIN_ORG_ROLES.contains(&name)
}

/// Built-in role masks (mirror of memory.move `ROLE_MASK_*`).
pub fn builtin_org_role_mask(name: &str) -> Option<i64> {
    match name {
        "owner" => Some(127),
        "admin" => Some(111),
        "agent_manager" => Some(36),
        "finance_approver" => Some(24),
        "memory_administrator" => Some(3),
        "auditor" => Some(96),
        _ => None,
    }
}

/// Expand a permission mask into its individual bits.
pub fn expand_org_permission_mask(mask: i64) -> Vec<i64> {
    let mut bits = Vec::new();
    let mut bit = 1i64;
    while bit <= ORG_PERM_ALL {
        if mask & bit != 0 {
            bits.push(bit);
        }
        bit <<= 1;
    }
    bits
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = org_memory_permissions)]
pub struct NewOrgMemoryPermission {
    pub organization_id: String,
    pub member_address: String,
    pub permission_kind: i64,
    pub active: bool,
    pub granted_by: String,
    pub group_id: Option<String>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = org_memory_permissions)]
pub struct OrgMemoryPermissionRow {
    pub organization_id: String,
    pub member_address: String,
    pub permission_kind: i64,
    pub active: bool,
    pub granted_by: String,
    pub group_id: Option<String>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = org_roles)]
pub struct NewOrgRole {
    pub organization_id: String,
    pub role_name: String,
    pub mask: i64,
    pub is_builtin: bool,
    pub defined_by: String,
    pub active: bool,
    pub updated_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = org_roles)]
pub struct OrgRoleRow {
    pub organization_id: String,
    pub role_name: String,
    pub mask: i64,
    pub is_builtin: bool,
    pub defined_by: String,
    pub active: bool,
    pub updated_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = org_role_assignments)]
pub struct NewOrgRoleAssignment {
    pub organization_id: String,
    pub member_address: String,
    pub role_name: String,
    pub role_mask: i64,
    pub assigned_mask: i64,
    pub active: bool,
    pub assigned_by: String,
    pub assigned_at_ms: i64,
    pub revoked_at_ms: Option<i64>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = org_role_assignments)]
pub struct OrgRoleAssignmentRow {
    pub organization_id: String,
    pub member_address: String,
    pub role_name: String,
    pub role_mask: i64,
    pub assigned_mask: i64,
    pub active: bool,
    pub assigned_by: String,
    pub assigned_at_ms: i64,
    pub revoked_at_ms: Option<i64>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_mask_yields_individual_bits() {
        assert_eq!(expand_org_permission_mask(0), Vec::<i64>::new());
        assert_eq!(expand_org_permission_mask(3), vec![1, 2]);
        assert_eq!(
            expand_org_permission_mask(ORG_PERM_ALL),
            vec![1, 2, 4, 8, 16, 32, 64]
        );
        // finance_approver mask = budget manager + spend approver
        assert_eq!(expand_org_permission_mask(24), vec![8, 16]);
    }

    #[test]
    fn builtin_role_names_match_move() {
        assert!(is_builtin_org_role("owner"));
        assert!(is_builtin_org_role("finance_approver"));
        assert!(!is_builtin_org_role("observer"));
    }
}
