// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Double, Integer, Nullable, SmallInt, Text, Timestamptz};
use serde::Serialize;

use crate::json_serde::{json_string_i64, json_string_opt_i64};

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptPoolRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = Text)]
    pub associated_id: String,
    /// Nano-SPT on-chain units (`10^9` per display token).
    #[serde(serialize_with = "json_string_i64::serialize")]
    #[diesel(sql_type = BigInt)]
    pub circulating_supply: i64,
    #[diesel(sql_type = BigInt)]
    pub base_price: i64,
    #[diesel(sql_type = BigInt)]
    pub quadratic_coefficient: i64,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptTransactionRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_type: String,
    #[diesel(sql_type = Text)]
    pub sender: String,
    #[serde(serialize_with = "json_string_i64::serialize")]
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub myso_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub creator_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub treasury_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub price: i64,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptHoldingRow {
    #[diesel(sql_type = Text)]
    pub holder_address: String,
    #[serde(serialize_with = "json_string_i64::serialize")]
    #[diesel(sql_type = BigInt)]
    pub balance: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptPriceHistoryRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = BigInt)]
    pub price: i64,
    #[serde(serialize_with = "json_string_i64::serialize")]
    #[diesel(sql_type = BigInt)]
    pub circulating_supply: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptExchangeConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub post_threshold: i64,
    #[diesel(sql_type = BigInt)]
    pub profile_threshold: i64,
    #[diesel(sql_type = BigInt)]
    pub max_individual_reservation_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub total_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub trading_creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub trading_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub trading_treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub reservation_creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub reservation_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub reservation_treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_reservers_per_pool: i64,
    #[diesel(sql_type = BigInt)]
    pub base_price: i64,
    #[diesel(sql_type = BigInt)]
    pub quadratic_coefficient: i64,
    #[diesel(sql_type = BigInt)]
    pub max_hold_percent_bps: i64,
    #[diesel(sql_type = Bool)]
    pub trading_enabled: bool,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptReservationPoolRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub associated_id: String,
    #[diesel(sql_type = SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = BigInt)]
    pub total_reserved: i64,
    #[diesel(sql_type = BigInt)]
    pub required_threshold: i64,
    #[diesel(sql_type = Text)]
    pub status: String,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptReservationPoolWithDisplayRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub associated_id: String,
    #[diesel(sql_type = SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = BigInt)]
    pub total_reserved: i64,
    #[diesel(sql_type = BigInt)]
    pub required_threshold: i64,
    #[diesel(sql_type = Text)]
    pub status: String,
    #[diesel(sql_type = BigInt)]
    pub created_at_epoch: i64,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub icon: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub primary_label: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub secondary_label: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub volume_24h: i64,
    #[diesel(sql_type = BigInt)]
    pub volume_change_24h: i64,
    #[diesel(sql_type = Nullable<Double>)]
    pub volume_change_percent_24h: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SptUserHoldingItem {
    pub pool_id: String,
    /// nano-SPT balance (`10^9` per display token); string JSON for JS integer safety.
    #[serde(serialize_with = "json_string_i64::serialize")]
    pub amount: i64,
    pub acquired_at: i64,
    pub source: String,
    /// `1` = profile SPT, `2` = post SPT (same as on-chain / indexer).
    pub token_type: i16,
    /// Profile or post object id for this pool (subject of the token).
    pub associated_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptReservationVolumeBucketRow {
    /// `date_trunc` bucket start, Unix epoch seconds (UTC).
    #[diesel(sql_type = BigInt)]
    pub bucket_start: i64,
    /// Exclusive bucket end, Unix epoch seconds (UTC).
    #[diesel(sql_type = BigInt)]
    pub bucket_end: i64,
    /// Earliest reservation row time in bucket, Unix epoch seconds (UTC).
    #[diesel(sql_type = BigInt)]
    pub earliest_at: i64,
    /// Latest reservation row time in bucket, Unix epoch seconds (UTC).
    #[diesel(sql_type = BigInt)]
    pub latest_at: i64,
    #[diesel(sql_type = BigInt)]
    pub deposit_volume: i64,
    #[diesel(sql_type = BigInt)]
    pub withdrawal_volume: i64,
    #[diesel(sql_type = BigInt)]
    pub deposit_count: i64,
    #[diesel(sql_type = BigInt)]
    pub withdrawal_count: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptReservationRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub reserver_address: String,
    /// Nano-MYSO: 10^9 units per display MYSO. Serialised as a JSON string for JS precision safety.
    #[serde(serialize_with = "json_string_i64::serialize")]
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub reserved_at: i64,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[serde(serialize_with = "json_string_opt_i64::serialize")]
    #[diesel(sql_type = Nullable<BigInt>)]
    pub fee_amount: Option<i64>,
    #[serde(serialize_with = "json_string_opt_i64::serialize")]
    #[diesel(sql_type = Nullable<BigInt>)]
    pub creator_fee: Option<i64>,
    #[serde(serialize_with = "json_string_opt_i64::serialize")]
    #[diesel(sql_type = Nullable<BigInt>)]
    pub platform_fee: Option<i64>,
    #[serde(serialize_with = "json_string_opt_i64::serialize")]
    #[diesel(sql_type = Nullable<BigInt>)]
    pub treasury_fee: Option<i64>,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptRevenueRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_type: String,
    #[diesel(sql_type = Text)]
    pub trader: String,
    #[diesel(sql_type = Text)]
    pub creator_address: String,
    #[diesel(sql_type = Text)]
    pub platform_address: String,
    #[diesel(sql_type = Text)]
    pub treasury_address: String,
    #[diesel(sql_type = BigInt)]
    pub creator_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub treasury_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub total_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub token_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub myso_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub token_price: i64,
    #[diesel(sql_type = BigInt)]
    pub revenue_time: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}
