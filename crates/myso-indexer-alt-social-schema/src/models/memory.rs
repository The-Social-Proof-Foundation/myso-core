// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    memory_accounts, memory_config, memory_usage_stats, sub_agent_events, sub_agent_memory_vaults,
    sub_agents,
};

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = memory_accounts)]
pub struct NewMemoryAccount {
    pub account_id: String,
    pub principal_owner: String,
    pub profile_id: String,
    pub active: bool,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = sub_agents)]
pub struct NewSubAgent {
    pub agent_object_id: String,
    pub derived_address: String,
    pub account_id: String,
    pub label: String,
    pub identity_class: i16,
    pub role_tags: i64,
    pub capabilities: i64,
    pub delegatable_caps: i64,
    pub register_scope: i16,
    pub approval_required_caps: i64,
    pub max_action_spend: Option<i64>,
    pub platform_scope: Option<String>,
    pub parent_object_id: Option<String>,
    pub depth: i16,
    pub registered_by: String,
    pub expires_at_ms: Option<i64>,
    pub active: bool,
    pub created_at_ms: i64,
    pub deactivated_at_ms: Option<i64>,
    pub revoked_at_ms: Option<i64>,
    pub updated_at_ms: i64,
    pub organization_id: Option<String>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_events)]
pub struct NewSubAgentEvent {
    pub event_type: String,
    pub account_id: Option<String>,
    pub principal_owner: Option<String>,
    pub profile_id: Option<String>,
    pub agent_object_id: Option<String>,
    pub derived_address: Option<String>,
    pub label: Option<String>,
    pub identity_class: Option<i16>,
    pub role_tags: Option<i64>,
    pub capabilities: Option<i64>,
    pub delegatable_caps: Option<i64>,
    pub register_scope: Option<i16>,
    pub approval_required_caps: Option<i64>,
    pub max_action_spend: Option<i64>,
    pub platform_scope: Option<String>,
    pub parent_object_id: Option<String>,
    pub depth: Option<i16>,
    pub registered_by: Option<String>,
    pub expires_at_ms: Option<i64>,
    pub active: Option<bool>,
    pub created_at_ms: Option<i64>,
    pub revoked_count: Option<i64>,
    pub previous_owner: Option<String>,
    pub new_owner: Option<String>,
    pub migration_from_version: Option<i64>,
    pub migration_to_version: Option<i64>,
    pub registry_id: Option<String>,
    pub organization_id: Option<String>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_memory_vaults)]
pub struct NewAgentMemoryVault {
    pub vault_id: String,
    pub agent_object_id: String,
    pub memory_account_id: String,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_memory_vaults)]
pub struct AgentMemoryVaultRow {
    pub vault_id: String,
    pub agent_object_id: String,
    pub memory_account_id: String,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = memory_accounts)]
pub struct MemoryAccountRow {
    pub account_id: String,
    pub principal_owner: String,
    pub profile_id: String,
    pub active: bool,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

/// Per-agent memory usage pushed by the memory relayer (internal ingest).
#[derive(Debug, Clone, Insertable, AsChangeset, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = memory_usage_stats)]
pub struct MemoryUsageStatsRow {
    pub agent_object_id: String,
    pub organization_id: Option<String>,
    pub account_id: Option<String>,
    pub entries: i64,
    pub bytes: i64,
    pub org_shared_entries: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = sub_agents)]
pub struct SubAgentRow {
    pub agent_object_id: String,
    pub derived_address: String,
    pub account_id: String,
    pub label: String,
    pub identity_class: i16,
    pub role_tags: i64,
    pub capabilities: i64,
    pub delegatable_caps: i64,
    pub register_scope: i16,
    pub approval_required_caps: i64,
    pub max_action_spend: Option<i64>,
    pub platform_scope: Option<String>,
    pub parent_object_id: Option<String>,
    pub depth: i16,
    pub registered_by: String,
    pub expires_at_ms: Option<i64>,
    pub active: bool,
    pub created_at_ms: i64,
    pub deactivated_at_ms: Option<i64>,
    pub revoked_at_ms: Option<i64>,
    pub updated_at_ms: i64,
    pub organization_id: Option<String>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = memory_config)]
pub struct NewMemoryConfig {
    pub updated_by: String,
    pub max_organizations_per_user: i16,
    pub org_category_update_cooldown_ms: i64,
    pub max_agent_depth: i16,
    pub max_label_length: i64,
    pub max_org_name_length: i64,
    pub max_org_description_length: i64,
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewMemoryConfig {
    pub fn from_event(
        updated_by: String,
        max_organizations_per_user: u8,
        org_category_update_cooldown_ms: u64,
        max_agent_depth: u8,
        max_label_length: u64,
        max_org_name_length: u64,
        max_org_description_length: u64,
        version: u64,
        updated_at: u64,
        transaction_id: String,
    ) -> Self {
        let time = chrono::DateTime::<chrono::Utc>::from_timestamp((updated_at / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now);
        Self {
            updated_by,
            max_organizations_per_user: max_organizations_per_user as i16,
            org_category_update_cooldown_ms: org_category_update_cooldown_ms as i64,
            max_agent_depth: max_agent_depth as i16,
            max_label_length: max_label_length as i64,
            max_org_name_length: max_org_name_length as i64,
            max_org_description_length: max_org_description_length as i64,
            version: version as i64,
            updated_at: updated_at as i64,
            time,
            transaction_id,
        }
    }
}
