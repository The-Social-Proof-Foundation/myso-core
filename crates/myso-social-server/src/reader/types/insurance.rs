// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Bool, SmallInt, Text, Timestamp, Timestamptz};
use diesel::QueryableByName;
use serde::Serialize;

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub enable_flag: bool,
    #[diesel(sql_type = BigInt)]
    pub min_coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_duration_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceVaultInfo {
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub underwriter: String,
    #[diesel(sql_type = BigInt)]
    pub capital_balance: i64,
    #[diesel(sql_type = BigInt)]
    pub reserved: i64,
    #[diesel(sql_type = BigInt)]
    pub base_rate_bps_per_day: i64,
    #[diesel(sql_type = BigInt)]
    pub utilization_multiplier_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_exposure_per_market: i64,
    #[diesel(sql_type = BigInt)]
    pub max_exposure_per_user: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = Timestamp)]
    pub created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceVaultRow {
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub underwriter: String,
    #[diesel(sql_type = BigInt)]
    pub capital_balance: i64,
    #[diesel(sql_type = BigInt)]
    pub reserved: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceVaultTransactionRow {
    #[diesel(sql_type = Text)]
    pub transaction_type: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub balance_after: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceVaultExposureRow {
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub total_exposure: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsurancePolicyInfo {
    #[diesel(sql_type = Text)]
    pub policy_id: String,
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = Text)]
    pub insured: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub covered_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub premium_paid: i64,
    #[diesel(sql_type = BigInt)]
    pub start_time_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub expiry_time_ms: i64,
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsurancePolicyRow {
    #[diesel(sql_type = Text)]
    pub policy_id: String,
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = Text)]
    pub insured: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub covered_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub premium_paid: i64,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
}
