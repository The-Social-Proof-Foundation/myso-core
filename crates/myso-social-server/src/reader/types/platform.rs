// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use diesel::sql_types::{BigInt, Text};
use diesel::QueryableByName;

// API-layer type: subset of Platform for list/detail responses.
// DB-table type: myso_indexer_alt_social_schema::models::Platform.

#[derive(Debug, Serialize)]
pub struct PlatformRow {
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub media_previews: Option<serde_json::Value>,
    pub developer_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderators_group_id: Option<String>,
    pub status: i16,
    pub is_approved: bool,
    pub primary_category: String,
    pub secondary_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
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
            cover_photo: p.cover_photo,
            media_previews: p.media_previews,
            developer_address: p.developer_address,
            moderators_group_id: p.moderators_group_id,
            status: p.status,
            is_approved: p.is_approved,
            primary_category: p.primary_category,
            secondary_category: p.secondary_category,
            redirect_uri: p.redirect_uri,
            created_at: p.created_at,
            updated_at: p.updated_at,
            deleted_at: p.deleted_at,
            wants_dao_governance: p.wants_dao_governance,
            governance_registry_id: p.governance_registry_id,
            delegate_count: p.delegate_count,
            delegate_term_epochs: p.delegate_term_epochs,
            max_votes_per_user: p.max_votes_per_user,
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
    pub permissions: Vec<String>,
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
pub struct PlatformUserAccessRow {
    pub is_member: bool,
    pub is_blocked: bool,
    pub is_moderator: bool,
    pub moderator_permissions: Vec<String>,
    pub can_block_users: bool,
    pub can_moderate_content: bool,
    pub can_manage_badges: bool,
    pub can_withdraw_from_platform_treasury: bool,
    pub can_manage_promotions: bool,
}

impl PlatformUserAccessRow {
    pub(crate) fn from_db(
        is_member: bool,
        is_blocked: bool,
        is_moderator: bool,
        moderator_permissions: Vec<String>,
        developer_address: &str,
        user_address: &str,
    ) -> Self {
        let permissions = moderator_permissions;
        let is_developer = developer_address == user_address;
        let can_block_users = permissions.iter().any(|p| {
            p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_BLOCK_ADMIN
        });
        let can_moderate_content = permissions.iter().any(|p| {
            p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_CONTENT_MODERATOR
        });
        let can_manage_badges = permissions.iter().any(|p| {
            p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_BADGE_ADMIN
        });
        let can_withdraw_from_platform_treasury = is_developer
            || permissions.iter().any(|p| {
                p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_TREASURY_ADMIN
            });
        let can_manage_promotions = permissions.iter().any(|p| {
            p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_PROMOTION_ADMIN
        });
        Self {
            is_member,
            is_blocked,
            is_moderator,
            moderator_permissions: permissions,
            can_block_users,
            can_moderate_content,
            can_manage_badges,
            can_withdraw_from_platform_treasury,
            can_manage_promotions,
        }
    }
}

#[derive(diesel::QueryableByName)]
pub struct PlatformUserAccessDbRow {
    #[diesel(sql_type = diesel::sql_types::Bool)]
    is_member: bool,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    is_blocked: bool,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    is_moderator: bool,
    #[diesel(sql_type = diesel::sql_types::Array<diesel::sql_types::Text>)]
    moderator_permissions: Vec<String>,
}

impl From<PlatformUserAccessDbRow> for PlatformUserAccessRow {
    fn from(row: PlatformUserAccessDbRow) -> Self {
        Self::from_db(
            row.is_member,
            row.is_blocked,
            row.is_moderator,
            row.moderator_permissions,
            "",
            "",
        )
    }
}

#[derive(Debug, Serialize)]
pub struct PlatformTreasuryInfo {
    pub platform_id: String,
    pub balance_mist: i64,
    pub last_funded_at: Option<i64>,
    pub last_withdrawn_at: Option<i64>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PlatformTreasuryWithdrawalRow {
    pub id: i32,
    pub platform_id: String,
    pub recipient: String,
    pub amount: i64,
    pub reason_code: i16,
    pub executed_by: String,
    pub timestamp: i64,
    pub created_at: chrono::NaiveDateTime,
    pub event_id: Option<String>,
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

#[derive(Debug, Serialize, QueryableByName)]
#[serde(rename_all = "camelCase")]
pub struct PlatformConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub max_reasoning_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_cover_photo_url_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_media_previews: i64,
    #[diesel(sql_type = BigInt)]
    pub max_badge_name_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_badge_description_length: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
}
