// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PlatformRow {
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub developer_address: String,
    pub status: i16,
    pub is_approved: bool,
    pub primary_category: String,
    pub secondary_category: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wants_dao_governance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governance_registry_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate_term_epochs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_votes_per_user: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_on_chain_age_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_submission_cost: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadratic_base_cost: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum_votes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voting_period_epochs: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PlatformModeratorRow {
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PlatformBlockedProfileRow {
    pub wallet_address: String,
    pub blocked_by: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PlatformApprovalRow {
    pub is_approved: bool,
    pub approval_changed_at: Option<chrono::NaiveDateTime>,
    pub approved_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PlatformMemberRow {
    pub wallet_address: String,
    pub joined_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PlatformEventRow {
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
