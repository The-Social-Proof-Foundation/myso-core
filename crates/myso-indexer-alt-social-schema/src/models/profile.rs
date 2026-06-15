// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{profile_badges, profile_events, profile_offers, profile_sale_fees, profiles};

pub const PROFILE_SALE_FEE_BPS: i32 = 500;
pub const CURVE_PRECISION: i64 = 1000;
pub const MAX_BADGE_NAME_LENGTH: usize = 100;
pub const MAX_BADGE_DESCRIPTION_LENGTH: usize = 500;
pub const MAX_BADGE_MEDIA_URL_LENGTH: usize = 2048;
pub const MAX_BADGE_ICON_URL_LENGTH: usize = 2048;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
pub struct Profile {
    pub id: i32,
    pub owner_address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub website: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub cover_photo: Option<String>,
    pub profile_id: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub post_count: i32,
    pub min_offer_amount: Option<i64>,
    pub birthdate: Option<String>,
    pub current_location: Option<String>,
    pub raised_location: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub political_view: Option<String>,
    pub religion: Option<String>,
    pub education: Option<String>,
    pub primary_language: Option<String>,
    pub relationship_status: Option<String>,
    pub x_username: Option<String>,
    pub social_proof_token_address: Option<String>,
    pub reservation_pool_address: Option<String>,
    pub selected_badge_id: Option<String>,
    pub selected_ecosystem_badge_id: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub owner_address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub website: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub cover_photo: Option<String>,
    pub profile_id: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub post_count: i32,
    pub min_offer_amount: Option<i64>,
    pub birthdate: Option<String>,
    pub current_location: Option<String>,
    pub raised_location: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub political_view: Option<String>,
    pub religion: Option<String>,
    pub education: Option<String>,
    pub primary_language: Option<String>,
    pub relationship_status: Option<String>,
    pub x_username: Option<String>,
    pub social_proof_token_address: Option<String>,
    pub reservation_pool_address: Option<String>,
    pub selected_badge_id: Option<String>,
    pub selected_ecosystem_badge_id: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_badges)]
pub struct NewProfileBadge {
    pub profile_id: String,
    pub badge_id: String,
    pub badge_name: String,
    pub badge_description: Option<String>,
    pub badge_media_url: Option<String>,
    pub badge_icon_url: Option<String>,
    pub platform_id: String,
    pub assigned_by: String,
    pub assigned_at: i64,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub revoked_by: Option<String>,
    pub badge_type: i16,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_events)]
pub struct NewProfileEvent {
    pub event_type: String,
    pub profile_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = profiles)]
pub struct ProfileUpdateSet {
    pub updated_at: NaiveDateTime,
    pub display_name: Option<Option<String>>,
    pub bio: Option<Option<String>>,
    pub profile_photo: Option<Option<String>>,
    pub cover_photo: Option<Option<String>>,
    pub birthdate: Option<Option<String>>,
    pub current_location: Option<Option<String>>,
    pub raised_location: Option<Option<String>>,
    pub phone: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub gender: Option<Option<String>>,
    pub political_view: Option<Option<String>>,
    pub religion: Option<Option<String>>,
    pub education: Option<Option<String>>,
    pub primary_language: Option<Option<String>>,
    pub relationship_status: Option<Option<String>>,
    pub x_username: Option<Option<String>>,
    pub min_offer_amount: Option<Option<i64>>,
    pub username: Option<String>,
    pub selected_badge_id: Option<Option<String>>,
    pub selected_ecosystem_badge_id: Option<Option<String>>,
    pub reservation_pool_address: Option<Option<String>>,
    pub social_proof_token_address: Option<Option<String>>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_offers)]
pub struct NewProfileOffer {
    pub profile_id: String,
    pub offeror_address: String,
    pub amount: i64,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub resolved_at: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_offers)]
pub struct ProfileOffer {
    pub id: i32,
    pub profile_id: String,
    pub offeror_address: String,
    pub amount: i64,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub resolved_at: Option<i64>,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_sale_fees)]
pub struct NewProfileSaleFee {
    pub profile_id: String,
    pub offeror_address: String,
    pub previous_owner_address: String,
    pub sale_amount: i64,
    pub fee_amount: i64,
    pub fee_recipient_address: String,
    pub timestamp: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_sale_fees)]
pub struct ProfileSaleFee {
    pub id: i32,
    pub profile_id: String,
    pub offeror_address: String,
    pub previous_owner_address: String,
    pub sale_amount: i64,
    pub fee_amount: i64,
    pub fee_recipient_address: String,
    pub timestamp: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}
