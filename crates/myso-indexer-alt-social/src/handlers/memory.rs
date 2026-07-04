// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    builtin_org_role_mask, expand_org_permission_mask, AuditAction, NewAgentMemoryVault,
    NewAgenticOrganization, NewAuditLog, NewMemoryAccount, NewMemoryConfig, NewOrgInvitation,
    NewOrgMemoryPermission, NewOrgRole, NewOrgRoleAssignment, NewOrganizationEvent, NewSubAgent,
    NewSubAgentEvent, AUDIT_ACTOR_HUMAN, AUDIT_SOURCE_CHAIN, BUILTIN_ORG_ROLES,
    EVENT_TYPE_ORG_CATEGORY_UPDATED, EVENT_TYPE_ORG_CREATED, EVENT_TYPE_ORG_DEACTIVATED,
    EVENT_TYPE_ORG_UPDATED, ORG_INVITATION_STATUS_ACCEPTED, ORG_INVITATION_STATUS_DECLINED,
    ORG_INVITATION_STATUS_PENDING,
};

pub(crate) fn json_to_i64(v: &serde_json::Value) -> i64 {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
        .unwrap_or(0)
}

pub(crate) fn json_opt_i64(v: &serde_json::Value) -> Option<i64> {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
}

pub(crate) fn json_str(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| v.as_str()).map(String::from)
}

fn json_opt_string(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| {
        if v.is_null() {
            None
        } else {
            v.as_str().map(String::from)
        }
    })
}

fn json_opt_addr(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| {
        if v.is_null() {
            None
        } else {
            v.as_str().map(String::from)
        }
    })
}

fn json_u8(data: &serde_json::Value, key: &str) -> i16 {
    data.get(key)
        .and_then(|v| v.as_u64())
        .and_then(|n| i16::try_from(n).ok())
        .unwrap_or(0)
}

fn json_u64(data: &serde_json::Value, key: &str) -> i64 {
    data.get(key).map(json_to_i64).unwrap_or(0)
}

fn json_bool(data: &serde_json::Value, key: &str) -> bool {
    data.get(key).and_then(|v| v.as_bool()).unwrap_or(true)
}

/// Chain-derived audit row written in the same commit as the domain update.
#[allow(clippy::too_many_arguments)]
pub(crate) fn chain_audit_row(
    action: AuditAction,
    actor_address: String,
    target_type: &str,
    target_id: String,
    organization_id: Option<String>,
    account_id: Option<String>,
    prev_state: Option<serde_json::Value>,
    new_state: Option<serde_json::Value>,
    event_id: &str,
    transaction_id: &str,
    now: chrono::DateTime<Utc>,
) -> NewAuditLog {
    NewAuditLog {
        time: now,
        source: AUDIT_SOURCE_CHAIN.to_string(),
        actor_address,
        // Best-effort: the on-chain signer address is authoritative; class refinement is a
        // read-side concern (agents can be resolved via sub_agents.derived_address).
        actor_type: AUDIT_ACTOR_HUMAN.to_string(),
        action: action.as_str().to_string(),
        target_type: target_type.to_string(),
        target_id,
        organization_id,
        account_id,
        prev_state,
        new_state,
        tx_digest: Some(transaction_id.to_string()),
        event_id: Some(event_id.to_string()),
        idempotency_key: None,
        metadata: None,
    }
}

pub(crate) fn handle_memory_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next().unwrap_or(event_id).to_string();
    let now = Utc::now();
    match event_name {
        "MemoryAccountCreated" => {
            let account_id = json_str(data, "account_id")?;
            let principal_owner = json_str(data, "owner")?;
            let profile_id = json_str(data, "profile_id")?;
            Some(vec![
                SocialEventRow::MemoryAccount(NewMemoryAccount {
                    account_id: account_id.clone(),
                    principal_owner,
                    profile_id: profile_id.clone(),
                    active: true,
                    created_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                    time: now,
                }),
                SocialEventRow::ProfileMemoryAccountLink {
                    profile_id,
                    memory_account_id: account_id,
                },
            ])
        }
        "SubAgentRegistered" | "SubAgentUpdated" => {
            let is_register = event_name == "SubAgentRegistered";
            let mut rows = Vec::new();
            if is_register {
                if let Some(organization_id) = json_str(data, "organization_id") {
                    rows.push(SocialEventRow::OrganizationAgentRegistered {
                        organization_id,
                        active: json_bool(data, "active"),
                        depth: json_u8(data, "depth").max(1),
                        parent_object_id: json_opt_addr(data, "parent_object_id"),
                        agent_object_id: json_str(data, "agent_object_id").unwrap_or_default(),
                        activity_at_ms: data
                            .get("created_at")
                            .and_then(json_opt_i64)
                            .filter(|v| *v > 0)
                            .unwrap_or_else(|| now.timestamp_millis()),
                    });
                }
            }
            if rows.is_empty() {
                None
            } else {
                Some(rows)
            }
        }
        "SubAgentDeactivated" => {
            let agent_object_id = json_str(data, "agent_object_id")?;
            Some(vec![SocialEventRow::OrganizationAgentActiveDelta {
                agent_object_id,
                active_delta: -1,
                activity_at_ms: now.timestamp_millis(),
            }])
        }
        "SubAgentRevoked" => {
            let agent_object_id = json_str(data, "agent_object_id")?;
            Some(vec![SocialEventRow::OrganizationAgentActiveDelta {
                agent_object_id,
                active_delta: -1,
                activity_at_ms: now.timestamp_millis(),
            }])
        }
        "SubAgentsClearedOnTransfer" => None,
        "MemoryAccountDeactivated" => {
            let account_id = json_str(data, "account_id")?;
            Some(vec![SocialEventRow::MemoryAccountActiveUpdate {
                account_id,
                active: false,
            }])
        }
        "MemoryAccountReactivated" => {
            let account_id = json_str(data, "account_id")?;
            Some(vec![SocialEventRow::MemoryAccountActiveUpdate {
                account_id,
                active: true,
            }])
        }
        "MemoryAccountMigrated" => {
            let account_id = json_str(data, "account_id")?;
            let from = json_u64(data, "from");
            let to = json_u64(data, "to");
            Some(vec![SocialEventRow::SubAgentEvent(NewSubAgentEvent {
                event_type: event_name.to_string(),
                account_id: Some(account_id),
                principal_owner: None,
                profile_id: None,
                agent_object_id: None,
                derived_address: None,
                label: None,
                identity_class: None,
                role_tags: None,
                capabilities: None,
                delegatable_caps: None,
                register_scope: None,
                approval_required_caps: None,
                max_action_spend: None,
                platform_scope: None,
                parent_object_id: None,
                depth: None,
                registered_by: None,
                expires_at_ms: None,
                active: None,
                created_at_ms: None,
                revoked_count: None,
                previous_owner: None,
                new_owner: None,
                migration_from_version: Some(from),
                migration_to_version: Some(to),
                registry_id: None,
                organization_id: None,
                event_id: event_id.to_string(),
                transaction_id,
                time: now,
            })])
        }
        "MemoryRegistryMigrated" => {
            let registry_id = json_str(data, "registry_id")?;
            let from = json_u64(data, "from");
            let to = json_u64(data, "to");
            Some(vec![SocialEventRow::SubAgentEvent(NewSubAgentEvent {
                event_type: event_name.to_string(),
                account_id: None,
                principal_owner: None,
                profile_id: None,
                agent_object_id: None,
                derived_address: None,
                label: None,
                identity_class: None,
                role_tags: None,
                capabilities: None,
                delegatable_caps: None,
                register_scope: None,
                approval_required_caps: None,
                max_action_spend: None,
                platform_scope: None,
                parent_object_id: None,
                depth: None,
                registered_by: None,
                expires_at_ms: None,
                active: None,
                created_at_ms: None,
                revoked_count: None,
                previous_owner: None,
                new_owner: None,
                migration_from_version: Some(from),
                migration_to_version: Some(to),
                registry_id: Some(registry_id),
                organization_id: None,
                event_id: event_id.to_string(),
                transaction_id,
                time: now,
            })])
        }
        "AgentMemoryVaultCreated" => {
            let vault_id = json_str(data, "vault_id")?;
            let agent_object_id = json_str(data, "agent_object_id")?;
            let memory_account_id = json_str(data, "memory_account_id")?;
            Some(vec![SocialEventRow::AgentMemoryVault(
                NewAgentMemoryVault {
                    vault_id,
                    agent_object_id,
                    memory_account_id,
                    created_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                },
            )])
        }
        "AgenticOrganizationCreated" => {
            let organization_id = json_str(data, "organization_id")?;
            let account_id = json_str(data, "account_id")?;
            let principal_owner = json_str(data, "principal_owner")?;
            let profile_id = json_str(data, "profile_id")?;
            let name = json_opt_string(data, "name");
            let description = json_opt_string(data, "description");
            let org_type = json_u8(data, "org_type");
            let created_at_ms = data
                .get("created_at")
                .and_then(json_opt_i64)
                .filter(|v| *v > 0)
                .unwrap_or_else(|| now.timestamp_millis());
            let org = NewAgenticOrganization {
                organization_id: organization_id.clone(),
                account_id: account_id.clone(),
                principal_owner: principal_owner.clone(),
                profile_id: profile_id.clone(),
                name: name.clone(),
                description: description.clone(),
                org_type,
                root_agent_id: None,
                active: true,
                created_at_ms,
                deactivated_at_ms: None,
                event_id: event_id.to_string(),
                transaction_id: transaction_id.clone(),
                time: now,
            };
            let mut rows = vec![
                SocialEventRow::AgenticOrganizationUpsert(org),
                SocialEventRow::OrganizationStatsInit {
                    organization_id: organization_id.clone(),
                    activity_at_ms: created_at_ms,
                },
                SocialEventRow::OrganizationEvent(NewOrganizationEvent {
                    event_type: EVENT_TYPE_ORG_CREATED.to_string(),
                    organization_id: Some(organization_id.clone()),
                    account_id: Some(account_id.clone()),
                    principal_owner: Some(principal_owner.clone()),
                    profile_id: Some(profile_id),
                    name,
                    description,
                    org_type: Some(org_type),
                    previous_org_type: None,
                    root_agent_id: None,
                    agent_object_id: None,
                    active: Some(true),
                    created_at_ms: Some(created_at_ms),
                    deactivated_at_ms: None,
                    updated_at_ms: None,
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                    time: now,
                }),
            ];
            // Seed the built-in role definitions so dashboards can list them per org.
            for role_name in BUILTIN_ORG_ROLES {
                if let Some(mask) = builtin_org_role_mask(role_name) {
                    rows.push(SocialEventRow::OrgRoleUpsert(NewOrgRole {
                        organization_id: organization_id.clone(),
                        role_name: role_name.to_string(),
                        mask,
                        is_builtin: true,
                        defined_by: principal_owner.clone(),
                        active: true,
                        updated_at_ms: created_at_ms,
                        event_id: event_id.to_string(),
                        transaction_id: transaction_id.clone(),
                        time: now,
                    }));
                }
            }
            rows.push(SocialEventRow::AuditLog(chain_audit_row(
                AuditAction::OrgCreate,
                principal_owner,
                "organization",
                organization_id.clone(),
                Some(organization_id),
                Some(account_id),
                None,
                Some(serde_json::json!({ "org_type": org_type })),
                event_id,
                &transaction_id,
                now,
            )));
            Some(rows)
        }
        "AgenticOrganizationUpdated" => {
            let organization_id = json_str(data, "organization_id")?;
            let name = json_opt_string(data, "name");
            let description = json_opt_string(data, "description");
            Some(vec![
                SocialEventRow::AgenticOrganizationMetadataUpdate {
                    organization_id: organization_id.clone(),
                    name: name.clone(),
                    description: description.clone(),
                },
                SocialEventRow::OrganizationEvent(NewOrganizationEvent {
                    event_type: EVENT_TYPE_ORG_UPDATED.to_string(),
                    organization_id: Some(organization_id),
                    account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    name,
                    description,
                    org_type: None,
                    previous_org_type: None,
                    root_agent_id: None,
                    agent_object_id: None,
                    active: None,
                    created_at_ms: None,
                    deactivated_at_ms: None,
                    updated_at_ms: None,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AgenticOrganizationCategoryUpdated" => {
            let organization_id = json_str(data, "organization_id")?;
            let org_type = json_u8(data, "org_type");
            let previous_org_type = json_u8(data, "previous_org_type");
            let updated_at_ms = data
                .get("updated_at")
                .and_then(json_opt_i64)
                .filter(|v| *v > 0)
                .unwrap_or_else(|| now.timestamp_millis());
            Some(vec![
                SocialEventRow::AgenticOrganizationCategoryUpdate {
                    organization_id: organization_id.clone(),
                    org_type,
                    previous_org_type,
                    updated_at_ms,
                },
                SocialEventRow::OrganizationEvent(NewOrganizationEvent {
                    event_type: EVENT_TYPE_ORG_CATEGORY_UPDATED.to_string(),
                    organization_id: Some(organization_id),
                    account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    name: None,
                    description: None,
                    org_type: Some(org_type),
                    previous_org_type: Some(previous_org_type),
                    root_agent_id: None,
                    agent_object_id: None,
                    active: None,
                    created_at_ms: None,
                    deactivated_at_ms: None,
                    updated_at_ms: Some(updated_at_ms),
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AgenticOrganizationDeactivated" => {
            let organization_id = json_str(data, "organization_id")?;
            let deactivated_at_ms = data
                .get("deactivated_at")
                .and_then(json_opt_i64)
                .filter(|v| *v > 0)
                .unwrap_or_else(|| now.timestamp_millis());
            Some(vec![
                SocialEventRow::AgenticOrganizationDeactivate {
                    organization_id: organization_id.clone(),
                    deactivated_at_ms,
                },
                SocialEventRow::OrganizationEvent(NewOrganizationEvent {
                    event_type: EVENT_TYPE_ORG_DEACTIVATED.to_string(),
                    organization_id: Some(organization_id.clone()),
                    account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    name: None,
                    description: None,
                    org_type: None,
                    previous_org_type: None,
                    root_agent_id: None,
                    agent_object_id: None,
                    active: Some(false),
                    created_at_ms: None,
                    deactivated_at_ms: Some(deactivated_at_ms),
                    updated_at_ms: None,
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                    time: now,
                }),
                SocialEventRow::AuditLog(chain_audit_row(
                    AuditAction::OrgDeactivate,
                    // Event does not carry the signer; the tx digest identifies it on-chain.
                    "unknown".to_string(),
                    "organization",
                    organization_id.clone(),
                    Some(organization_id),
                    None,
                    Some(serde_json::json!({ "active": true })),
                    Some(serde_json::json!({ "active": false })),
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "OrgMemoryGroupCreated" => {
            let group_id = json_str(data, "group_id")?;
            let organization_id = json_str(data, "organization_id")?;
            let account_id = json_str(data, "account_id")?;
            let principal_owner = json_str(data, "principal_owner")?;
            Some(vec![
                SocialEventRow::AgenticOrganizationMemoryGroupSet {
                    organization_id: organization_id.clone(),
                    group_id: group_id.clone(),
                },
                SocialEventRow::AuditLog(chain_audit_row(
                    AuditAction::OrgMemoryGroupCreate,
                    principal_owner,
                    "org_memory_group",
                    group_id.clone(),
                    Some(organization_id),
                    Some(account_id),
                    None,
                    Some(serde_json::json!({ "group_id": group_id })),
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "OrgMemoryPermissionGranted" | "OrgMemoryPermissionRevoked" => {
            let granted = event_name == "OrgMemoryPermissionGranted";
            let organization_id = json_str(data, "organization_id")?;
            let account_id = json_str(data, "account_id")?;
            let group_id = json_str(data, "group_id")?;
            let member = json_str(data, "member")?;
            let mask = json_u64(data, "permissions_mask");
            let actor = if granted {
                json_str(data, "granted_by")?
            } else {
                json_str(data, "revoked_by")?
            };
            let timestamp_ms = json_u64(data, "timestamp_ms");
            let mut rows = Vec::new();
            for bit in expand_org_permission_mask(mask) {
                rows.push(SocialEventRow::OrgMemoryPermissionUpsert(
                    NewOrgMemoryPermission {
                        organization_id: organization_id.clone(),
                        member_address: member.clone(),
                        permission_kind: bit,
                        active: granted,
                        granted_by: actor.clone(),
                        group_id: Some(group_id.clone()),
                        event_id: event_id.to_string(),
                        transaction_id: transaction_id.clone(),
                        time: now,
                    },
                ));
            }
            rows.push(SocialEventRow::AuditLog(chain_audit_row(
                if granted {
                    AuditAction::OrgMemoryGrant
                } else {
                    AuditAction::OrgMemoryRevoke
                },
                actor,
                "org_member",
                member,
                Some(organization_id),
                Some(account_id),
                None,
                Some(serde_json::json!({
                    "permissions_mask": mask,
                    "active": granted,
                    "timestamp_ms": timestamp_ms,
                })),
                event_id,
                &transaction_id,
                now,
            )));
            Some(rows)
        }
        "OrgRoleDefined" => {
            let organization_id = json_str(data, "organization_id")?;
            let account_id = json_str(data, "account_id")?;
            let role_name = json_str(data, "role_name")?;
            let mask = json_u64(data, "mask");
            let previous_mask = data.get("previous_mask").and_then(json_opt_i64);
            let defined_by = json_str(data, "defined_by")?;
            let timestamp_ms = json_u64(data, "timestamp_ms");
            Some(vec![
                SocialEventRow::OrgRoleUpsert(NewOrgRole {
                    organization_id: organization_id.clone(),
                    role_name: role_name.clone(),
                    mask,
                    is_builtin: false,
                    defined_by: defined_by.clone(),
                    active: true,
                    updated_at_ms: timestamp_ms,
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                    time: now,
                }),
                SocialEventRow::AuditLog(chain_audit_row(
                    AuditAction::OrgRoleDefine,
                    defined_by,
                    "org_role",
                    role_name,
                    Some(organization_id),
                    Some(account_id),
                    previous_mask.map(|m| serde_json::json!({ "mask": m })),
                    Some(serde_json::json!({ "mask": mask })),
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "OrgRoleAssigned" => {
            let organization_id = json_str(data, "organization_id")?;
            let account_id = json_str(data, "account_id")?;
            let group_id = json_str(data, "group_id")?;
            let member = json_str(data, "member")?;
            let role_name = json_str(data, "role_name")?;
            let mask = json_u64(data, "mask");
            let granted_mask = json_u64(data, "granted_mask");
            let assigned_by = json_str(data, "assigned_by")?;
            let timestamp_ms = json_u64(data, "timestamp_ms");
            let mut rows = vec![SocialEventRow::OrgRoleAssignmentUpsert(
                NewOrgRoleAssignment {
                    organization_id: organization_id.clone(),
                    member_address: member.clone(),
                    role_name: role_name.clone(),
                    role_mask: mask,
                    assigned_mask: granted_mask,
                    active: true,
                    assigned_by: assigned_by.clone(),
                    assigned_at_ms: timestamp_ms,
                    revoked_at_ms: None,
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                    time: now,
                },
            )];
            // The role's constituent permissions become active member permissions.
            for bit in expand_org_permission_mask(granted_mask) {
                rows.push(SocialEventRow::OrgMemoryPermissionUpsert(
                    NewOrgMemoryPermission {
                        organization_id: organization_id.clone(),
                        member_address: member.clone(),
                        permission_kind: bit,
                        active: true,
                        granted_by: assigned_by.clone(),
                        group_id: Some(group_id.clone()),
                        event_id: event_id.to_string(),
                        transaction_id: transaction_id.clone(),
                        time: now,
                    },
                ));
            }
            rows.push(SocialEventRow::AuditLog(chain_audit_row(
                AuditAction::OrgRoleAssign,
                assigned_by,
                "org_member",
                member,
                Some(organization_id),
                Some(account_id),
                None,
                Some(serde_json::json!({
                    "role_name": role_name,
                    "mask": mask,
                    "granted_mask": granted_mask,
                })),
                event_id,
                &transaction_id,
                now,
            )));
            Some(rows)
        }
        "OrgRoleRevoked" => {
            let organization_id = json_str(data, "organization_id")?;
            let account_id = json_str(data, "account_id")?;
            let group_id = json_str(data, "group_id")?;
            let member = json_str(data, "member")?;
            let role_name = json_str(data, "role_name")?;
            let revoked_mask = json_u64(data, "revoked_mask");
            let revoked_by = json_str(data, "revoked_by")?;
            let timestamp_ms = json_u64(data, "timestamp_ms");
            let mut rows = vec![SocialEventRow::OrgRoleAssignmentRevoke {
                organization_id: organization_id.clone(),
                member_address: member.clone(),
                role_name: role_name.clone(),
                revoked_at_ms: timestamp_ms,
                event_id: event_id.to_string(),
                transaction_id: transaction_id.clone(),
            }];
            for bit in expand_org_permission_mask(revoked_mask) {
                rows.push(SocialEventRow::OrgMemoryPermissionUpsert(
                    NewOrgMemoryPermission {
                        organization_id: organization_id.clone(),
                        member_address: member.clone(),
                        permission_kind: bit,
                        active: false,
                        granted_by: revoked_by.clone(),
                        group_id: Some(group_id.clone()),
                        event_id: event_id.to_string(),
                        transaction_id: transaction_id.clone(),
                        time: now,
                    },
                ));
            }
            rows.push(SocialEventRow::AuditLog(chain_audit_row(
                AuditAction::OrgRoleRevoke,
                revoked_by,
                "org_member",
                member,
                Some(organization_id),
                Some(account_id),
                Some(serde_json::json!({
                    "role_name": role_name,
                    "granted_mask": revoked_mask,
                })),
                None,
                event_id,
                &transaction_id,
                now,
            )));
            Some(rows)
        }
        "OrgInvitationCreated" => {
            let organization_id = json_str(data, "organization_id")?;
            let account_id = json_str(data, "account_id")?;
            let invitee = json_str(data, "invitee")?;
            let role_name = json_opt_string(data, "role_name");
            let permissions_mask = json_u64(data, "permissions_mask");
            let invited_by = json_str(data, "invited_by")?;
            let timestamp_ms = json_u64(data, "timestamp_ms");
            let expires_at_ms = data.get("expires_at_ms").and_then(json_opt_i64);
            Some(vec![
                SocialEventRow::OrgInvitationUpsert(NewOrgInvitation {
                    organization_id: organization_id.clone(),
                    invitee_address: invitee.clone(),
                    role_name: role_name.clone(),
                    permissions_mask,
                    status: ORG_INVITATION_STATUS_PENDING.to_string(),
                    invited_by: invited_by.clone(),
                    created_at_ms: timestamp_ms,
                    expires_at_ms,
                    responded_at_ms: None,
                    responded_by: None,
                    granted_mask: None,
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                    time: now,
                }),
                SocialEventRow::AuditLog(chain_audit_row(
                    AuditAction::OrgInvitationCreate,
                    invited_by,
                    "org_invitation",
                    invitee,
                    Some(organization_id),
                    Some(account_id),
                    None,
                    Some(serde_json::json!({
                        "role_name": role_name,
                        "permissions_mask": permissions_mask,
                        "expires_at_ms": expires_at_ms,
                    })),
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "OrgInvitationAccepted" => {
            let organization_id = json_str(data, "organization_id")?;
            let account_id = json_str(data, "account_id")?;
            let group_id = json_str(data, "group_id")?;
            let invitee = json_str(data, "invitee")?;
            let role_name = json_opt_string(data, "role_name");
            let _permissions_mask = json_u64(data, "permissions_mask");
            let granted_mask = json_u64(data, "granted_mask");
            let accepted_by = json_str(data, "accepted_by")?;
            let timestamp_ms = json_u64(data, "timestamp_ms");
            let mut rows = vec![SocialEventRow::OrgInvitationRespond {
                organization_id: organization_id.clone(),
                invitee_address: invitee.clone(),
                status: ORG_INVITATION_STATUS_ACCEPTED.to_string(),
                responded_at_ms: timestamp_ms,
                responded_by: accepted_by.clone(),
                granted_mask: Some(granted_mask),
                event_id: event_id.to_string(),
                transaction_id: transaction_id.clone(),
            }];
            for bit in expand_org_permission_mask(granted_mask) {
                rows.push(SocialEventRow::OrgMemoryPermissionUpsert(
                    NewOrgMemoryPermission {
                        organization_id: organization_id.clone(),
                        member_address: invitee.clone(),
                        permission_kind: bit,
                        active: true,
                        granted_by: accepted_by.clone(),
                        group_id: Some(group_id.clone()),
                        event_id: event_id.to_string(),
                        transaction_id: transaction_id.clone(),
                        time: now,
                    },
                ));
            }
            rows.push(SocialEventRow::AuditLog(chain_audit_row(
                AuditAction::OrgInvitationAccept,
                accepted_by,
                "org_invitation",
                invitee,
                Some(organization_id),
                Some(account_id),
                Some(serde_json::json!({ "status": ORG_INVITATION_STATUS_PENDING })),
                Some(serde_json::json!({
                    "status": ORG_INVITATION_STATUS_ACCEPTED,
                    "role_name": role_name,
                    "granted_mask": granted_mask,
                })),
                event_id,
                &transaction_id,
                now,
            )));
            Some(rows)
        }
        "OrgInvitationDeclined" => {
            let organization_id = json_str(data, "organization_id")?;
            let account_id = json_str(data, "account_id")?;
            let invitee = json_str(data, "invitee")?;
            let declined_by = json_str(data, "declined_by")?;
            let timestamp_ms = json_u64(data, "timestamp_ms");
            Some(vec![
                SocialEventRow::OrgInvitationRespond {
                    organization_id: organization_id.clone(),
                    invitee_address: invitee.clone(),
                    status: ORG_INVITATION_STATUS_DECLINED.to_string(),
                    responded_at_ms: timestamp_ms,
                    responded_by: declined_by.clone(),
                    granted_mask: None,
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AuditLog(chain_audit_row(
                    AuditAction::OrgInvitationDecline,
                    declined_by,
                    "org_invitation",
                    invitee,
                    Some(organization_id),
                    Some(account_id),
                    Some(serde_json::json!({ "status": ORG_INVITATION_STATUS_PENDING })),
                    Some(serde_json::json!({ "status": ORG_INVITATION_STATUS_DECLINED })),
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "MemoryConfigUpdatedEvent" => {
            let updated_by = json_str(data, "updated_by")?;
            let max_organizations_per_user =
                json_to_i64(data.get("max_organizations_per_user")?) as i16;
            let org_category_update_cooldown_ms =
                json_to_i64(data.get("org_category_update_cooldown_ms")?);
            let max_agent_depth = json_to_i64(data.get("max_agent_depth")?) as i16;
            let max_label_length = json_to_i64(data.get("max_label_length")?);
            let max_org_name_length = json_to_i64(data.get("max_org_name_length")?);
            let max_org_description_length = json_to_i64(data.get("max_org_description_length")?);
            let updated_at = json_to_i64(data.get("timestamp")?);
            Some(vec![SocialEventRow::MemoryConfig(NewMemoryConfig {
                updated_by,
                max_organizations_per_user,
                org_category_update_cooldown_ms,
                max_agent_depth,
                max_label_length,
                max_org_name_length,
                max_org_description_length,
                version: 1,
                updated_at,
                time: now,
                transaction_id,
            })])
        }
        _ => None,
    }
}

pub(crate) fn sub_agent_from_event(
    data: &serde_json::Value,
    event_id: &str,
    transaction_id: &str,
    now: chrono::DateTime<Utc>,
) -> Option<NewSubAgent> {
    let created_at_ms = data
        .get("created_at")
        .and_then(json_opt_i64)
        .filter(|v| *v > 0)
        .unwrap_or_else(|| now.timestamp_millis());
    Some(NewSubAgent {
        agent_object_id: json_str(data, "agent_object_id")?,
        derived_address: json_str(data, "derived_address")?,
        account_id: json_str(data, "account_id")?,
        organization_id: json_opt_addr(data, "organization_id"),
        label: json_str(data, "label").unwrap_or_default(),
        identity_class: json_u8(data, "identity_class"),
        role_tags: json_u64(data, "role_tags"),
        capabilities: json_u64(data, "capabilities"),
        delegatable_caps: json_u64(data, "delegatable_caps"),
        register_scope: json_u8(data, "register_scope"),
        approval_required_caps: json_u64(data, "approval_required_caps"),
        max_action_spend: data.get("max_action_spend").and_then(json_opt_i64),
        platform_scope: json_opt_addr(data, "platform_scope"),
        parent_object_id: json_opt_addr(data, "parent_object_id"),
        depth: json_u8(data, "depth").max(1),
        registered_by: json_str(data, "registered_by")?,
        expires_at_ms: data.get("expires_at").and_then(json_opt_i64),
        active: json_bool(data, "active"),
        created_at_ms,
        deactivated_at_ms: None,
        revoked_at_ms: None,
        updated_at_ms: now.timestamp_millis(),
        event_id: event_id.to_string(),
        transaction_id: transaction_id.to_string(),
        time: now,
    })
}

pub(crate) fn sub_agent_audit_event(
    event_type: &str,
    data: &serde_json::Value,
    event_id: &str,
    transaction_id: &str,
    now: chrono::DateTime<Utc>,
) -> NewSubAgentEvent {
    NewSubAgentEvent {
        event_type: event_type.to_string(),
        account_id: json_str(data, "account_id"),
        principal_owner: json_str(data, "principal_owner").or_else(|| json_str(data, "owner")),
        profile_id: json_str(data, "profile_id"),
        organization_id: json_str(data, "organization_id"),
        agent_object_id: json_str(data, "agent_object_id"),
        derived_address: json_str(data, "derived_address"),
        label: json_str(data, "label"),
        identity_class: data
            .get("identity_class")
            .map(|_| json_u8(data, "identity_class")),
        role_tags: data.get("role_tags").map(|_| json_u64(data, "role_tags")),
        capabilities: data
            .get("capabilities")
            .map(|_| json_u64(data, "capabilities")),
        delegatable_caps: data
            .get("delegatable_caps")
            .map(|_| json_u64(data, "delegatable_caps")),
        register_scope: data
            .get("register_scope")
            .map(|_| json_u8(data, "register_scope")),
        approval_required_caps: data
            .get("approval_required_caps")
            .map(|_| json_u64(data, "approval_required_caps")),
        max_action_spend: data.get("max_action_spend").and_then(json_opt_i64),
        platform_scope: json_opt_addr(data, "platform_scope"),
        parent_object_id: json_opt_addr(data, "parent_object_id"),
        depth: data.get("depth").map(|_| json_u8(data, "depth")),
        registered_by: json_str(data, "registered_by"),
        expires_at_ms: data.get("expires_at").and_then(json_opt_i64),
        active: data.get("active").and_then(|v| v.as_bool()),
        created_at_ms: data.get("created_at").and_then(json_opt_i64),
        revoked_count: data
            .get("revoked_count")
            .map(|_| json_u64(data, "revoked_count")),
        previous_owner: json_str(data, "previous_owner"),
        new_owner: json_str(data, "new_owner"),
        migration_from_version: None,
        migration_to_version: None,
        registry_id: None,
        event_id: event_id.to_string(),
        transaction_id: transaction_id.to_string(),
        time: now,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::SocialEventRow;

    #[test]
    fn memory_account_created_emits_registry_and_profile_link_rows() {
        let data = serde_json::json!({
            "account_id": "0xae23224508d2e7c700a9bb9e93e99b6d9a8f7fdec96a80979000f0bb11c47cbd",
            "owner": "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
            "profile_id": "0x452d2f34638572f21211200557e427b41c44e4700c5889f23fb485e3b308a0a0",
        });
        let rows = handle_memory_event("MemoryAccountCreated", &data, "tx-digest:0")
            .expect("MemoryAccountCreated should produce rows");
        assert_eq!(rows.len(), 2);
        let (memory_row, link_row) = match (&rows[0], &rows[1]) {
            (SocialEventRow::MemoryAccount(_), SocialEventRow::ProfileMemoryAccountLink { .. }) => {
                (&rows[0], &rows[1])
            }
            (SocialEventRow::ProfileMemoryAccountLink { .. }, SocialEventRow::MemoryAccount(_)) => {
                (&rows[1], &rows[0])
            }
            _ => panic!("expected MemoryAccount and ProfileMemoryAccountLink rows"),
        };
        let SocialEventRow::MemoryAccount(account) = memory_row else {
            unreachable!()
        };
        assert_eq!(
            account.account_id,
            "0xae23224508d2e7c700a9bb9e93e99b6d9a8f7fdec96a80979000f0bb11c47cbd"
        );
        let SocialEventRow::ProfileMemoryAccountLink {
            profile_id,
            memory_account_id,
        } = link_row
        else {
            unreachable!()
        };
        assert_eq!(
            profile_id,
            "0x452d2f34638572f21211200557e427b41c44e4700c5889f23fb485e3b308a0a0"
        );
        assert_eq!(
            memory_account_id,
            "0xae23224508d2e7c700a9bb9e93e99b6d9a8f7fdec96a80979000f0bb11c47cbd"
        );
    }

    #[test]
    fn sub_agent_registered_without_org_skips_registry_rows_in_memory_pipeline() {
        let data = serde_json::json!({
            "agent_object_id": "0xagent",
            "derived_address": "0xderived",
            "account_id": "0xaccount",
            "principal_owner": "0xowner",
            "profile_id": "0xprofile",
            "label": "bot",
            "registered_by": "0xowner",
            "active": true,
        });
        let rows = handle_memory_event("SubAgentRegistered", &data, "tx-digest:1");
        assert!(
            rows.is_none(),
            "memory pipeline should not index sub_agents rows"
        );
    }

    #[test]
    fn sub_agent_registered_with_org_emits_org_stats_row_only() {
        let data = serde_json::json!({
            "agent_object_id": "0xagent",
            "derived_address": "0xderived",
            "account_id": "0xaccount",
            "organization_id": "0xorg",
            "label": "bot",
            "registered_by": "0xowner",
            "active": true,
            "depth": 1,
        });
        let rows = handle_memory_event("SubAgentRegistered", &data, "tx-digest:1")
            .expect("org agent registration should produce org stats row");
        assert_eq!(rows.len(), 1);
        let SocialEventRow::OrganizationAgentRegistered {
            organization_id, ..
        } = &rows[0]
        else {
            panic!("expected OrganizationAgentRegistered row");
        };
        assert_eq!(organization_id, "0xorg");
    }

    #[test]
    fn memory_account_deactivated_emits_active_update_row() {
        let data = serde_json::json!({
            "account_id": "0xaccount",
            "owner": "0xowner",
        });
        let rows = handle_memory_event("MemoryAccountDeactivated", &data, "tx-digest:2")
            .expect("MemoryAccountDeactivated should produce rows");
        assert_eq!(rows.len(), 1);
        let SocialEventRow::MemoryAccountActiveUpdate { account_id, active } = &rows[0] else {
            panic!("expected MemoryAccountActiveUpdate row");
        };
        assert_eq!(account_id, "0xaccount");
        assert!(!active);
    }

    #[test]
    fn memory_account_reactivated_emits_active_update_row() {
        let data = serde_json::json!({
            "account_id": "0xaccount",
            "owner": "0xowner",
        });
        let rows = handle_memory_event("MemoryAccountReactivated", &data, "tx-digest:3")
            .expect("MemoryAccountReactivated should produce rows");
        assert_eq!(rows.len(), 1);
        let SocialEventRow::MemoryAccountActiveUpdate { account_id, active } = &rows[0] else {
            panic!("expected MemoryAccountActiveUpdate row");
        };
        assert_eq!(account_id, "0xaccount");
        assert!(active);
    }

    #[test]
    fn memory_account_migrated_emits_audit_row() {
        let data = serde_json::json!({
            "account_id": "0xaccount",
            "from": 1,
            "to": 2,
        });
        let rows = handle_memory_event("MemoryAccountMigrated", &data, "tx-digest:4")
            .expect("MemoryAccountMigrated should produce rows");
        assert_eq!(rows.len(), 1);
        let SocialEventRow::SubAgentEvent(e) = &rows[0] else {
            panic!("expected SubAgentEvent row");
        };
        assert_eq!(e.event_type, "MemoryAccountMigrated");
        assert_eq!(e.account_id.as_deref(), Some("0xaccount"));
        assert_eq!(e.migration_from_version, Some(1));
        assert_eq!(e.migration_to_version, Some(2));
    }

    #[test]
    fn agent_memory_vault_created_emits_vault_row() {
        let data = serde_json::json!({
            "vault_id": "0xvault",
            "agent_object_id": "0xagent",
            "memory_account_id": "0xaccount",
        });
        let rows = handle_memory_event("AgentMemoryVaultCreated", &data, "tx-digest:5")
            .expect("AgentMemoryVaultCreated should produce rows");
        assert_eq!(rows.len(), 1);
        let SocialEventRow::AgentMemoryVault(v) = &rows[0] else {
            panic!("expected AgentMemoryVault row");
        };
        assert_eq!(v.vault_id, "0xvault");
        assert_eq!(v.agent_object_id, "0xagent");
        assert_eq!(v.memory_account_id, "0xaccount");
    }
}
