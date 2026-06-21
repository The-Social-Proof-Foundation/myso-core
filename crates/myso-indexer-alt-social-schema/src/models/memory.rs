// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{agent_memory_vaults, memory_accounts, sub_agent_events, sub_agents};

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
#[diesel(table_name = agent_memory_vaults)]
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
#[diesel(table_name = agent_memory_vaults)]
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
