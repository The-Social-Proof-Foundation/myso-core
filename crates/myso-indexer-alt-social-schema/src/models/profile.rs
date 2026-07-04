// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    profile_badges, profile_config, profile_events, profile_offers, profile_sale_fees, profiles,
};

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
    pub memory_account_id: Option<String>,
    pub ai_credit_balance_id: Option<String>,
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
    pub memory_account_id: Option<String>,
    pub ai_credit_balance_id: Option<String>,
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

#[derive(Debug, Clone, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_config)]
pub struct NewProfileConfig {
    pub updated_by: String,
    pub max_vesting_pieces: i64,
    pub curve_factor_min: i64,
    pub curve_factor_max: i64,
    pub curve_precision: i64,
    pub min_claim_threshold_divisor: i64,
    pub min_username_length: i64,
    pub max_username_length: i64,
    pub profile_sale_fee_bps: i64,
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

pub fn default_profile_config() -> NewProfileConfig {
    NewProfileConfig {
        updated_by: String::new(),
        max_vesting_pieces: 10,
        curve_factor_min: 100,
        curve_factor_max: 10_000,
        curve_precision: 1000,
        min_claim_threshold_divisor: 1000,
        min_username_length: 2,
        max_username_length: 50,
        profile_sale_fee_bps: PROFILE_SALE_FEE_BPS as i64,
        version: 0,
        updated_at: 0,
        time: chrono::Utc::now(),
        transaction_id: String::new(),
    }
}

pub fn merge_profile_config(prev: &NewProfileConfig, incoming: &NewProfileConfig) -> NewProfileConfig {
    let version = if incoming.version > 0 {
        incoming.version
    } else {
        prev.version + 1
    };
    NewProfileConfig {
        updated_by: if incoming.updated_by.is_empty() {
            prev.updated_by.clone()
        } else {
            incoming.updated_by.clone()
        },
        max_vesting_pieces: if incoming.max_vesting_pieces > 0 {
            incoming.max_vesting_pieces
        } else {
            prev.max_vesting_pieces
        },
        curve_factor_min: if incoming.curve_factor_min > 0 {
            incoming.curve_factor_min
        } else {
            prev.curve_factor_min
        },
        curve_factor_max: if incoming.curve_factor_max > 0 {
            incoming.curve_factor_max
        } else {
            prev.curve_factor_max
        },
        curve_precision: if incoming.curve_precision > 0 {
            incoming.curve_precision
        } else {
            prev.curve_precision
        },
        min_claim_threshold_divisor: if incoming.min_claim_threshold_divisor > 0 {
            incoming.min_claim_threshold_divisor
        } else {
            prev.min_claim_threshold_divisor
        },
        min_username_length: if incoming.min_username_length > 0 {
            incoming.min_username_length
        } else {
            prev.min_username_length
        },
        max_username_length: if incoming.max_username_length > 0 {
            incoming.max_username_length
        } else {
            prev.max_username_length
        },
        profile_sale_fee_bps: if incoming.profile_sale_fee_bps > 0 {
            incoming.profile_sale_fee_bps
        } else {
            prev.profile_sale_fee_bps
        },
        version,
        updated_at: incoming.updated_at,
        time: incoming.time,
        transaction_id: incoming.transaction_id.clone(),
    }
}

impl NewProfileConfig {
    pub fn from_event(
        updated_by: String,
        max_vesting_pieces: u64,
        curve_factor_min: u64,
        curve_factor_max: u64,
        curve_precision: u64,
        min_claim_threshold_divisor: u64,
        min_username_length: u64,
        max_username_length: u64,
        profile_sale_fee_bps: u64,
        version: u64,
        updated_at: u64,
        transaction_id: String,
    ) -> Self {
        let time = chrono::DateTime::<chrono::Utc>::from_timestamp((updated_at / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now);
        Self {
            updated_by,
            max_vesting_pieces: max_vesting_pieces as i64,
            curve_factor_min: curve_factor_min as i64,
            curve_factor_max: curve_factor_max as i64,
            curve_precision: curve_precision as i64,
            min_claim_threshold_divisor: min_claim_threshold_divisor as i64,
            min_username_length: min_username_length as i64,
            max_username_length: max_username_length as i64,
            profile_sale_fee_bps: profile_sale_fee_bps as i64,
            version: version as i64,
            updated_at: updated_at as i64,
            time,
            transaction_id,
        }
    }
}

#[cfg(test)]
mod merge_profile_config_tests {
    use super::{default_profile_config, merge_profile_config, NewProfileConfig, PROFILE_SALE_FEE_BPS};

    fn sample_config(
        max_vesting_pieces: i64,
        profile_sale_fee_bps: i64,
        version: i64,
    ) -> NewProfileConfig {
        let mut cfg = default_profile_config();
        cfg.max_vesting_pieces = max_vesting_pieces;
        cfg.profile_sale_fee_bps = profile_sale_fee_bps;
        cfg.version = version;
        cfg.updated_by = "0xabc".to_string();
        cfg
    }

    #[test]
    fn treasury_fee_update_preserves_vesting_fields() {
        let prev = sample_config(42, PROFILE_SALE_FEE_BPS as i64, 1);
        let incoming = NewProfileConfig {
            updated_by: "0xdef".to_string(),
            max_vesting_pieces: 0,
            curve_factor_min: 0,
            curve_factor_max: 0,
            curve_precision: 0,
            min_claim_threshold_divisor: 0,
            min_username_length: 0,
            max_username_length: 0,
            profile_sale_fee_bps: 750,
            version: 0,
            updated_at: 100,
            time: prev.time,
            transaction_id: "tx1".to_string(),
        };
        let merged = merge_profile_config(&prev, &incoming);
        assert_eq!(merged.max_vesting_pieces, 42);
        assert_eq!(merged.profile_sale_fee_bps, 750);
        assert_eq!(merged.updated_by, "0xdef");
    }

    #[test]
    fn profile_config_update_preserves_fee() {
        let prev = sample_config(42, 600, 2);
        let incoming = NewProfileConfig {
            updated_by: "0xdef".to_string(),
            max_vesting_pieces: 99,
            curve_factor_min: 0,
            curve_factor_max: 0,
            curve_precision: 0,
            min_claim_threshold_divisor: 0,
            min_username_length: 0,
            max_username_length: 0,
            profile_sale_fee_bps: 0,
            version: 0,
            updated_at: 200,
            time: prev.time,
            transaction_id: "tx2".to_string(),
        };
        let merged = merge_profile_config(&prev, &incoming);
        assert_eq!(merged.max_vesting_pieces, 99);
        assert_eq!(merged.profile_sale_fee_bps, 600);
    }
}
