// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Organization role checks for enterprise REST reads.
//!
//! ## Route → permission mapping (FX4)
//!
//! | Route | Required permission (Move type) | DB bit |
//! |-------|----------------------------------|--------|
//! | `GET /organizations/:id/audit-logs` | `OrgAuditor` | `ORG_PERM_AUDITOR` (64) |
//! | `GET /organizations/:id/spend-breakdown` | `OrgDashboardViewer` | `ORG_PERM_DASHBOARD_VIEWER` (32) |
//! | `GET /organizations/:id/approvals` | `OrgDashboardViewer` | 32 |
//! | `GET /organizations/:id/memory-permissions` | `OrgDashboardViewer` | 32 |
//! | `GET /organizations/:id/roles` | `OrgDashboardViewer` | 32 |
//! | `GET /organizations/:id/role-assignments` | `OrgDashboardViewer` | 32 |
//! | `GET /organizations/:id/invitations` | `OrgDashboardViewer` | 32 |
//!
//! Access is granted when the wallet holds the required bit via active
//! `org_role_assignments.assigned_mask` or direct `org_memory_permissions` rows.

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use myso_indexer_alt_social_schema::models::{
    OrgMemoryPermissionRow, OrgRoleAssignmentRow, ORG_PERM_AUDITOR, ORG_PERM_DASHBOARD_VIEWER,
};
use serde::Serialize;
use std::sync::Arc;

use super::wallet::WalletAuthContext;
use crate::reader::Reader;
use crate::server::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrgAccessRequirement {
    DashboardViewer,
    Auditor,
}

impl OrgAccessRequirement {
    pub fn permission_bit(self) -> i64 {
        match self {
            OrgAccessRequirement::DashboardViewer => ORG_PERM_DASHBOARD_VIEWER,
            OrgAccessRequirement::Auditor => ORG_PERM_AUDITOR,
        }
    }
}

#[derive(Serialize)]
struct ForbiddenResponse {
    error: String,
    code: &'static str,
}

/// Pure permission check used by middleware and unit tests.
pub fn member_has_org_permission(
    memory_permissions: &[OrgMemoryPermissionRow],
    role_assignments: &[OrgRoleAssignmentRow],
    required_bit: i64,
) -> bool {
    memory_permissions
        .iter()
        .any(|row| row.active && row.permission_kind & required_bit == required_bit)
        || role_assignments
            .iter()
            .any(|row| row.active && row.assigned_mask & required_bit == required_bit)
}

pub async fn caller_has_org_permission(
    reader: &Reader,
    wallet_address: &str,
    organization_id: &str,
    required: OrgAccessRequirement,
) -> Result<bool, crate::error::SocialError> {
    let required_bit = required.permission_bit();
    let memory_permissions = reader
        .list_org_memory_permissions(organization_id, Some(wallet_address), true)
        .await?;
    let role_assignments = reader
        .list_org_role_assignments(organization_id, Some(wallet_address), true)
        .await?;
    Ok(member_has_org_permission(
        &memory_permissions,
        &role_assignments,
        required_bit,
    ))
}

fn extract_organization_id(path: &str) -> Option<String> {
    let mut segments = path.trim_start_matches('/').split('/');
    match (segments.next(), segments.next()) {
        (Some("organizations"), Some(id)) if !id.is_empty() => Some(id.to_string()),
        _ => None,
    }
}

async fn org_access_middleware_inner(
    state: Arc<AppState>,
    required: OrgAccessRequirement,
    request: Request<Body>,
    next: Next,
) -> Response {
    let wallet = match request.extensions().get::<WalletAuthContext>() {
        Some(ctx) => ctx.sender_address.clone(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ForbiddenResponse {
                    error: "Wallet authentication required".to_string(),
                    code: "MISSING_WALLET_AUTH",
                }),
            )
                .into_response();
        }
    };

    let organization_id = match extract_organization_id(request.uri().path()) {
        Some(id) => id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ForbiddenResponse {
                    error: "Could not resolve organization id from path".to_string(),
                    code: "INVALID_ORG_PATH",
                }),
            )
                .into_response();
        }
    };

    match caller_has_org_permission(&state.reader, &wallet, &organization_id, required).await {
        Ok(true) => next.run(request).await,
        Ok(false) => (
            StatusCode::FORBIDDEN,
            Json(ForbiddenResponse {
                error: format!(
                    "Wallet {wallet} lacks {:?} access to organization {organization_id}",
                    required
                ),
                code: "ORG_ACCESS_DENIED",
            }),
        )
            .into_response(),
        Err(err) => err.into_response(),
    }
}

pub async fn org_dashboard_access_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    org_access_middleware_inner(state, OrgAccessRequirement::DashboardViewer, request, next).await
}

pub async fn org_auditor_access_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    org_access_middleware_inner(state, OrgAccessRequirement::Auditor, request, next).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn memory_perm(bit: i64, active: bool) -> OrgMemoryPermissionRow {
        OrgMemoryPermissionRow {
            organization_id: "org1".to_string(),
            member_address: "0xabc".to_string(),
            permission_kind: bit,
            active,
            granted_by: "0x1".to_string(),
            group_id: None,
            event_id: "e1".to_string(),
            transaction_id: "tx1".to_string(),
            time: Utc::now(),
        }
    }

    fn role_assignment(mask: i64, active: bool) -> OrgRoleAssignmentRow {
        OrgRoleAssignmentRow {
            organization_id: "org1".to_string(),
            member_address: "0xabc".to_string(),
            role_name: "auditor".to_string(),
            role_mask: mask,
            assigned_mask: mask,
            active,
            assigned_by: "0x1".to_string(),
            assigned_at_ms: 0,
            revoked_at_ms: None,
            event_id: "e1".to_string(),
            transaction_id: "tx1".to_string(),
            time: Utc::now(),
        }
    }

    #[test]
    fn dashboard_viewer_via_direct_permission() {
        assert!(member_has_org_permission(
            &[memory_perm(ORG_PERM_DASHBOARD_VIEWER, true)],
            &[],
            ORG_PERM_DASHBOARD_VIEWER,
        ));
    }

    #[test]
    fn auditor_via_role_assignment_mask() {
        assert!(member_has_org_permission(
            &[],
            &[role_assignment(
                ORG_PERM_AUDITOR | ORG_PERM_DASHBOARD_VIEWER,
                true
            )],
            ORG_PERM_AUDITOR,
        ));
    }

    #[test]
    fn dashboard_viewer_not_granted_by_auditor_only_direct_bit() {
        assert!(!member_has_org_permission(
            &[memory_perm(ORG_PERM_AUDITOR, true)],
            &[],
            ORG_PERM_DASHBOARD_VIEWER,
        ));
    }

    #[test]
    fn inactive_rows_do_not_grant_access() {
        assert!(!member_has_org_permission(
            &[memory_perm(ORG_PERM_DASHBOARD_VIEWER, false)],
            &[role_assignment(ORG_PERM_DASHBOARD_VIEWER, false)],
            ORG_PERM_DASHBOARD_VIEWER,
        ));
    }

    #[test]
    fn finance_approver_mask_does_not_include_dashboard_viewer() {
        assert!(!member_has_org_permission(
            &[],
            &[role_assignment(24, true)],
            ORG_PERM_DASHBOARD_VIEWER,
        ));
    }

    #[test]
    fn extract_org_id_from_enterprise_paths() {
        assert_eq!(
            extract_organization_id("/organizations/0xdead/audit-logs"),
            Some("0xdead".to_string())
        );
        assert_eq!(extract_organization_id("/profiles/0x1"), None);
    }
}
