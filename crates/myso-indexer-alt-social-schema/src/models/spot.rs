// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Int4, Jsonb, Nullable, SmallInt, Text};
use serde::{Deserialize, Serialize};

use crate::schema::{
    spot_bet_withdrawals, spot_bets, spot_config, spot_events, spot_payouts, spot_records,
    spot_refunds, spot_resolutions,
};

pub const STATUS_OPEN: i16 = 1;
pub const STATUS_DAO_REQUIRED: i16 = 2;
pub const STATUS_RESOLVED: i16 = 3;
pub const STATUS_REFUNDABLE: i16 = 4;
pub const OUTCOME_DRAW: i16 = 255;
pub const OUTCOME_UNAPPLICABLE: i16 = 254;
pub const DEFAULT_CONFIDENCE_THRESHOLD_BPS: i32 = 7000;
pub const DEFAULT_FEE_BPS: i32 = 100;
pub const DEFAULT_FEE_SPLIT_PLATFORM_BPS: i32 = 5000;
pub const DEFAULT_MAX_BETS_PER_RECORD: i32 = 10000;
pub const MAX_BETTING_OPTIONS: i16 = 10;
pub const MIN_BETTING_OPTIONS: i16 = 2;

/// Query result for a spot bet (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct SpotBetRow {
    #[diesel(sql_type = Int4)]
    pub id: i32,
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
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub option_label: Option<String>,
}

/// Query result for a spot record (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct SpotRecordRow {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub outcome: Option<i16>,
    #[diesel(sql_type = Jsonb)]
    pub betting_options: serde_json::Value,
    #[diesel(sql_type = Jsonb)]
    pub option_escrow: serde_json::Value,
    #[diesel(sql_type = BigInt)]
    pub created_at_ms: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub resolution_window_ms: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub max_resolution_window_ms: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub last_resolution_at_ms: Option<i64>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub record_object_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub active_proposal_id: Option<String>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub oracle_proposed_outcome: Option<i16>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub proposed_outcome: Option<i16>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub dao_escalated_at_ms: Option<i64>,
}

/// Query result for a spot payout (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct SpotPayoutRow {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub user_address: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Query result for a spot refund (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct SpotRefundRow {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub user_address: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Query result for a spot resolution (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct SpotResolutionRow {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = SmallInt)]
    pub outcome: i16,
    #[diesel(sql_type = BigInt)]
    pub total_escrow: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_taken: i64,
    #[diesel(sql_type = BigInt)]
    pub resolved_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Text)]
    pub reasoning: String,
    #[diesel(sql_type = Jsonb)]
    pub evidence_urls: serde_json::Value,
}

/// Query result for a spot bet withdrawal (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct SpotBetWithdrawalRow {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub user_address: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_taken: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_records)]
pub struct NewSpotRecord {
    pub post_id: String,
    pub status: i16,
    pub outcome: Option<i16>,
    pub amm_split_bps_used: i32,
    pub betting_options: Option<serde_json::Value>,
    pub option_escrow: Option<serde_json::Value>,
    pub resolution_window_ms: Option<i64>,
    pub max_resolution_window_ms: Option<i64>,
    pub created_at_ms: i64,
    pub last_resolution_at_ms: Option<i64>,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
    pub record_object_id: Option<String>,
    pub active_proposal_id: Option<String>,
    pub oracle_proposed_outcome: Option<i16>,
    pub proposed_outcome: Option<i16>,
    pub dao_escalated_at_ms: Option<i64>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_bets)]
pub struct NewSpotBet {
    pub post_id: String,
    pub user_address: String,
    pub option_id: i16,
    pub escrow_amount: i64,
    pub amm_amount: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_payouts)]
pub struct NewSpotPayout {
    pub post_id: String,
    pub user_address: String,
    pub amount: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_refunds)]
pub struct NewSpotRefund {
    pub post_id: String,
    pub user_address: String,
    pub amount: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_resolutions)]
pub struct NewSpotResolution {
    pub post_id: String,
    pub outcome: i16,
    pub total_escrow: i64,
    pub fee_taken: i64,
    pub resolved_at_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub reasoning: String,
    pub evidence_urls: serde_json::Value,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_events)]
pub struct NewSpotEventLog {
    pub event_type: String,
    pub post_id: String,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_config)]
pub struct NewSpotConfig {
    pub updated_by: String,
    pub enable_flag: bool,
    pub confidence_threshold_bps: i64,
    pub resolution_window_ms: i64,
    pub max_resolution_window_ms: i64,
    pub payout_delay_ms: i64,
    pub fee_bps: i64,
    pub fee_split_bps_platform: i64,
    pub oracle_address: String,
    pub max_single_bet: i64,
    pub version: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub spot_governance_registry_id: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_bet_withdrawals)]
pub struct NewSpotBetWithdrawal {
    pub post_id: String,
    pub user_address: String,
    pub option_id: i16,
    pub amount: i64,
    pub fee_taken: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}
