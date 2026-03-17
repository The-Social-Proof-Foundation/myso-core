// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{blocked_events, blocked_profiles};

pub const EVENT_TYPE_BLOCK: &str = "block";
pub const EVENT_TYPE_UNBLOCK: &str = "unblock";

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = blocked_events)]
pub struct BlockedEvent {
    pub id: i32,
    pub event_id: Option<String>,
    pub event_type: String,
    pub blocker_address: String,
    pub blocked_address: Option<String>,
    pub raw_event_data: Option<serde_json::Value>,
    pub processed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = blocked_events)]
pub struct NewBlockedEvent {
    pub event_id: Option<String>,
    pub event_type: String,
    pub blocker_address: String,
    pub blocked_address: Option<String>,
    pub raw_event_data: Option<serde_json::Value>,
    pub processed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl NewBlockedEvent {
    pub fn new_block_event(
        event_id: Option<String>,
        blocker_address: String,
        blocked_address: String,
        raw_event_data: Option<serde_json::Value>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            event_id,
            event_type: EVENT_TYPE_BLOCK.to_string(),
            blocker_address,
            blocked_address: Some(blocked_address),
            raw_event_data,
            processed_at: chrono::Utc::now().naive_utc(),
            created_at,
        }
    }

    pub fn new_unblock_event(
        event_id: Option<String>,
        blocker_address: String,
        blocked_address: String,
        raw_event_data: Option<serde_json::Value>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            event_id,
            event_type: EVENT_TYPE_UNBLOCK.to_string(),
            blocker_address,
            blocked_address: Some(blocked_address),
            raw_event_data,
            processed_at: chrono::Utc::now().naive_utc(),
            created_at,
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct BlockedProfile {
    pub id: i32,
    pub blocker_address: String,
    pub blocked_address: String,
    pub blocked_profile_id: Option<String>,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    pub first_blocked_at: NaiveDateTime,
    pub last_blocked_at: NaiveDateTime,
    pub total_block_count: i32,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct NewBlockedProfile {
    pub blocker_address: String,
    pub blocked_address: String,
    pub blocked_profile_id: Option<String>,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    pub first_blocked_at: NaiveDateTime,
    pub last_blocked_at: NaiveDateTime,
    pub total_block_count: i32,
}

impl NewBlockedProfile {
    pub fn new(
        blocker_address: String,
        blocked_address: String,
        blocked_profile_id: Option<String>,
        blocked_username: String,
        blocked_display_name: Option<String>,
        blocked_profile_photo: Option<String>,
        blocked_at: NaiveDateTime,
    ) -> Self {
        Self {
            blocker_address,
            blocked_address,
            blocked_profile_id,
            blocked_username,
            blocked_display_name,
            blocked_profile_photo,
            first_blocked_at: blocked_at,
            last_blocked_at: blocked_at,
            total_block_count: 1,
        }
    }
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct UpdateBlockedProfile {
    pub blocked_profile_id: Option<Option<String>>,
    pub blocked_username: Option<String>,
    pub blocked_display_name: Option<Option<String>>,
    pub blocked_profile_photo: Option<Option<String>>,
    pub last_blocked_at: Option<NaiveDateTime>,
    pub total_block_count: Option<i32>,
}
