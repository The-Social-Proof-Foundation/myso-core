// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

// API-layer type: subset of Platform for list/detail responses.
// DB-table type: myso_indexer_alt_social_schema::models::Platform.

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

impl From<myso_indexer_alt_social_schema::models::Platform> for PlatformRow {
    fn from(p: myso_indexer_alt_social_schema::models::Platform) -> Self {
        Self {
            platform_id: p.platform_id,
            name: p.name,
            tagline: p.tagline,
            description: p.description,
            logo: p.logo,
            developer_address: p.developer_address,
            status: p.status,
            is_approved: p.is_approved,
            primary_category: p.primary_category,
            secondary_category: p.secondary_category,
            created_at: p.created_at,
            updated_at: p.updated_at,
            deleted_at: p.deleted_at,
            wants_dao_governance: p.wants_dao_governance,
            governance_registry_id: p.governance_registry_id,
            delegate_count: p.delegate_count,
            delegate_term_epochs: p.delegate_term_epochs,
            max_votes_per_user: p.max_votes_per_user,
            min_on_chain_age_days: p.min_on_chain_age_days,
            proposal_submission_cost: p.proposal_submission_cost,
            quadratic_base_cost: p.quadratic_base_cost,
            quorum_votes: p.quorum_votes,
            voting_period_epochs: p.voting_period_epochs,
        }
    }
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
    pub platform_id: String,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}
