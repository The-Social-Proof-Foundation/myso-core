// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, SmallInt, Text, Timestamptz};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SpotRecordResponse {
    pub post_id: String,
    pub status: i16,
    pub outcome: Option<i16>,
    pub betting_options: Vec<String>,
    pub option_escrow: std::collections::HashMap<String, i64>,
    pub resolution_window_ms: Option<i64>,
    pub max_resolution_window_ms: Option<i64>,
    pub created_at_ms: i64,
    pub last_resolution_at_ms: Option<i64>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SpotBetRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub user_address: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub escrow_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub amm_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SpotTransferRow {
    #[diesel(sql_type = Text)]
    pub user_address: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SpotConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub enable_flag: bool,
    #[diesel(sql_type = BigInt)]
    pub confidence_threshold_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub resolution_window_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub max_resolution_window_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub payout_delay_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_split_bps_platform: i64,
    #[diesel(sql_type = Text)]
    pub oracle_address: String,
    #[diesel(sql_type = BigInt)]
    pub max_single_bet: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}
