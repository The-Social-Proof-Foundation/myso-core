// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewAgentMemoryVault, NewAgenticOrganization, NewMemoryAccount, NewOrganizationEvent,
    NewSubAgent, NewSubAgentEvent, EVENT_TYPE_ORG_CATEGORY_UPDATED, EVENT_TYPE_ORG_CREATED,
    EVENT_TYPE_ORG_DEACTIVATED, EVENT_TYPE_ORG_UPDATED,
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
            Some(vec![
                SocialEventRow::AgenticOrganizationUpsert(org),
                SocialEventRow::OrganizationStatsInit {
                    organization_id: organization_id.clone(),
                    activity_at_ms: created_at_ms,
                },
                SocialEventRow::OrganizationEvent(NewOrganizationEvent {
                    event_type: EVENT_TYPE_ORG_CREATED.to_string(),
                    organization_id: Some(organization_id),
                    account_id: Some(account_id),
                    principal_owner: Some(principal_owner),
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
                    transaction_id,
                    time: now,
                }),
            ])
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
                    organization_id: Some(organization_id),
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
                    transaction_id,
                    time: now,
                }),
            ])
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
