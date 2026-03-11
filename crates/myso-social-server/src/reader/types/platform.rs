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
