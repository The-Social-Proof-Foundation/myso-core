// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{vesting_events, vesting_wallets};

// Constants matching social_contracts::profile VestingWallet (profile.move)
pub const VESTING_EVENT_TYPE_VESTED: &str = "TokensVested";
pub const VESTING_EVENT_TYPE_CLAIMED: &str = "TokensClaimed";
pub const CURVE_FACTOR_LINEAR: i64 = 1000;
pub const CURVE_FACTOR_MIN: i64 = 100;
pub const CURVE_FACTOR_MAX: i64 = 10000;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = vesting_wallets)]
pub struct VestingWallet {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub start_time: i64,
    pub duration: i64,
    pub curve_factor: i64,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = vesting_wallets)]
pub struct NewVestingWallet {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub start_time: i64,
    pub duration: i64,
    pub curve_factor: i64,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = vesting_wallets)]
pub struct UpdateVestingWallet {
    pub claimed_amount: Option<i64>,
    pub remaining_balance: Option<i64>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = vesting_events)]
pub struct VestingEvent {
    pub id: i32,
    pub wallet_id: String,
    pub event_type: String,
    pub owner_address: String,
    pub amount: i64,
    pub remaining_balance: Option<i64>,
    pub start_time: Option<i64>,
    pub duration: Option<i64>,
    pub curve_factor: Option<i64>,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = vesting_events)]
pub struct NewVestingEvent {
    pub wallet_id: String,
    pub event_type: String,
    pub owner_address: String,
    pub amount: i64,
    pub remaining_balance: Option<i64>,
    pub start_time: Option<i64>,
    pub duration: Option<i64>,
    pub curve_factor: Option<i64>,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}
