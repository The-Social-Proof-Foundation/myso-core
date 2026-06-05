// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    platform_blocked_profiles, platform_events, platform_memberships, platform_moderators,
    platform_token_airdrops, platforms,
};

pub const PLATFORM_STATUS_DEVELOPMENT: i16 = 0;
pub const PLATFORM_STATUS_ALPHA: i16 = 1;
pub const PLATFORM_STATUS_BETA: i16 = 2;
pub const PLATFORM_STATUS_LIVE: i16 = 3;
pub const PLATFORM_STATUS_MAINTENANCE: i16 = 4;
pub const PLATFORM_STATUS_SUNSET: i16 = 5;
pub const PLATFORM_STATUS_SHUTDOWN: i16 = 6;

pub const ALLOWED_CATEGORIES: &[&str] = &[
    "Social Network",
    "Messaging",
    "Long Form Publishing",
    "Community Forum",
    "Video Streaming",
    "Live Streaming",
    "Audio Streaming",
    "Decentralized Exchange",
    "Prediction Market",
    "Insurance Market",
    "Agentic Market",
    "Yield and Staking",
    "Real World Asset",
    "Ticketing and Events",
    "IP Licensing and Royalties",
    "Digital Asset Vault",
    "Reputation",
    "Advertising",
    "Data Marketplace",
    "Oracle and Data Feeds",
    "Analytics",
    "File Storage",
    "Privacy",
    "Gaming",
    "Developer Tools",
    "Hardware",
    "Research",
];

pub fn validate_category(category: &str) -> Result<(), String> {
    if ALLOWED_CATEGORIES.contains(&category) {
        Ok(())
    } else {
        Err(format!(
            "Invalid category: '{}'. Must be one of: {}",
            category,
            ALLOWED_CATEGORIES.join(", ")
        ))
    }
}

pub fn validate_categories(primary: &str, secondary: Option<&str>) -> Result<(), String> {
    validate_category(primary)?;
    if let Some(sec) = secondary {
        validate_category(sec)?;
        if primary == sec {
            return Err(format!(
                "Primary and secondary categories must be different. Both provided: '{}'",
                primary
            ));
        }
    }
    Ok(())
}

pub fn platform_status_to_text(status: i16) -> &'static str {
    match status {
        PLATFORM_STATUS_DEVELOPMENT => "Development",
        PLATFORM_STATUS_ALPHA => "Alpha",
        PLATFORM_STATUS_BETA => "Beta",
        PLATFORM_STATUS_LIVE => "Live",
        PLATFORM_STATUS_MAINTENANCE => "Maintenance",
        PLATFORM_STATUS_SUNSET => "Sunset",
        PLATFORM_STATUS_SHUTDOWN => "Shutdown",
        _ => "Unknown",
    }
}

pub fn milliseconds_to_naive_datetime(ms: u64) -> NaiveDateTime {
    let min_timestamp_ms = 1577836800000u64;
    let max_timestamp_ms = 4102444800000u64;

    if ms == 0 || ms < min_timestamp_ms || ms > max_timestamp_ms {
        chrono::Utc::now().naive_utc()
    } else {
        chrono::DateTime::from_timestamp((ms / 1000) as i64, ((ms % 1000) * 1_000_000) as u32)
            .map(|dt| dt.naive_utc())
            .unwrap_or_else(|| chrono::Utc::now().naive_utc())
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platforms)]
pub struct Platform {
    pub id: i32,
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub media_previews: Option<serde_json::Value>,
    pub developer_address: String,
    pub terms_of_service: Option<String>,
    pub privacy_policy: Option<String>,
    pub platform_names: Option<serde_json::Value>,
    pub links: Option<serde_json::Value>,
    pub status: i16,
    pub release_date: Option<String>,
    pub shutdown_date: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_approved: bool,
    pub approval_changed_at: Option<NaiveDateTime>,
    pub approved_by: Option<String>,
    pub wants_dao_governance: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub delegate_count: Option<i64>,
    pub delegate_term_epochs: Option<i64>,
    pub max_votes_per_user: Option<i64>,
    pub proposal_submission_cost: Option<i64>,
    pub quadratic_base_cost: Option<i64>,
    pub quorum_votes: Option<i64>,
    pub voting_period_epochs: Option<i64>,
    pub version: Option<i64>,
    pub primary_category: String,
    pub secondary_category: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platforms)]
pub struct NewPlatform {
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub media_previews: Option<serde_json::Value>,
    pub developer_address: String,
    pub terms_of_service: Option<String>,
    pub privacy_policy: Option<String>,
    pub platform_names: Option<serde_json::Value>,
    pub links: Option<serde_json::Value>,
    pub status: i16,
    pub release_date: Option<String>,
    pub shutdown_date: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_approved: bool,
    pub approval_changed_at: Option<NaiveDateTime>,
    pub approved_by: Option<String>,
    pub wants_dao_governance: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub delegate_count: Option<i64>,
    pub delegate_term_epochs: Option<i64>,
    pub max_votes_per_user: Option<i64>,
    pub proposal_submission_cost: Option<i64>,
    pub quadratic_base_cost: Option<i64>,
    pub quorum_votes: Option<i64>,
    pub voting_period_epochs: Option<i64>,
    pub version: Option<i64>,
    pub primary_category: String,
    pub secondary_category: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = platforms)]
pub struct UpdatePlatform {
    pub name: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub media_previews: Option<serde_json::Value>,
    pub terms_of_service: Option<String>,
    pub privacy_policy: Option<String>,
    pub platform_names: Option<serde_json::Value>,
    pub links: Option<serde_json::Value>,
    pub status: Option<i16>,
    pub release_date: Option<String>,
    pub shutdown_date: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_approved: Option<bool>,
    pub approval_changed_at: Option<NaiveDateTime>,
    pub approved_by: Option<String>,
    pub wants_dao_governance: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub delegate_count: Option<i64>,
    pub delegate_term_epochs: Option<i64>,
    pub max_votes_per_user: Option<i64>,
    pub proposal_submission_cost: Option<i64>,
    pub quadratic_base_cost: Option<i64>,
    pub quorum_votes: Option<i64>,
    pub voting_period_epochs: Option<i64>,
    pub version: Option<i64>,
    pub primary_category: Option<String>,
    pub secondary_category: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platform_moderators)]
pub struct PlatformModerator {
    pub id: i32,
    pub platform_id: String,
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_moderators)]
pub struct NewPlatformModerator {
    pub platform_id: String,
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platform_blocked_profiles)]
pub struct PlatformBlockedProfile {
    pub id: i32,
    pub platform_id: String,
    pub wallet_address: String,
    pub blocked_by: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_blocked_profiles)]
pub struct NewPlatformBlockedProfile {
    pub platform_id: String,
    pub wallet_address: String,
    pub blocked_by: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platform_events)]
pub struct PlatformEvent {
    pub id: i32,
    pub event_type: String,
    pub platform_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub reasoning: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_events)]
pub struct NewPlatformEvent {
    pub event_type: String,
    pub platform_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub reasoning: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platform_memberships)]
pub struct PlatformMembership {
    pub id: i32,
    pub platform_id: String,
    pub wallet_address: String,
    pub joined_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_memberships)]
pub struct NewPlatformMembership {
    pub platform_id: String,
    pub wallet_address: String,
    pub joined_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platform_token_airdrops)]
pub struct PlatformTokenAirdrop {
    pub id: i32,
    pub platform_id: String,
    pub recipient: String,
    pub amount: i64,
    pub reason_code: i16,
    pub executed_by: String,
    pub timestamp: i64,
    pub created_at: NaiveDateTime,
    pub event_id: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_token_airdrops)]
pub struct NewPlatformTokenAirdrop {
    pub platform_id: String,
    pub recipient: String,
    pub amount: i64,
    pub reason_code: i16,
    pub executed_by: String,
    pub timestamp: i64,
    pub created_at: NaiveDateTime,
    pub event_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformMemberRow {
    pub wallet_address: String,
    pub joined_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformModeratorRow {
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: NaiveDateTime,
}

/// Row returned when listing platforms a profile has joined (membership + platform columns + counts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePlatformMembershipRow {
    pub membership_id: i32,
    pub joined_at: NaiveDateTime,
    pub platform_db_id: i32,
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub media_previews: Option<serde_json::Value>,
    pub developer_address: String,
    pub terms_of_service: Option<String>,
    pub privacy_policy: Option<String>,
    pub platform_names: Option<serde_json::Value>,
    pub links: Option<serde_json::Value>,
    pub status: i16,
    pub release_date: Option<String>,
    pub shutdown_date: Option<String>,
    pub platform_created_at: NaiveDateTime,
    pub platform_updated_at: NaiveDateTime,
    pub is_approved: bool,
    pub approval_changed_at: Option<NaiveDateTime>,
    pub approved_by: Option<String>,
    pub wants_dao_governance: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub delegate_count: Option<i64>,
    pub delegate_term_epochs: Option<i64>,
    pub max_votes_per_user: Option<i64>,
    pub proposal_submission_cost: Option<i64>,
    pub quadratic_base_cost: Option<i64>,
    pub quorum_votes: Option<i64>,
    pub voting_period_epochs: Option<i64>,
    pub version: Option<i64>,
    pub primary_category: String,
    pub secondary_category: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub moderator_count: i64,
    pub blocked_profiles_count: i64,
}
