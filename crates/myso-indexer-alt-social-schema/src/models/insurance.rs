// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, SmallInt, Text, Timestamp};
use serde::{Deserialize, Serialize};

use crate::schema::{
    insurance_config, insurance_events, insurance_market_exposures, insurance_policies,
    insurance_policy_events, insurance_user_exposures, insurance_vault_transactions,
    insurance_vaults,
};

pub const STATUS_ACTIVE: i16 = 1;
pub const STATUS_CANCELLED: i16 = 2;
pub const STATUS_CLAIMED: i16 = 3;
pub const STATUS_EXPIRED: i16 = 4;
pub const BPS_DENOM: i64 = 10_000;
pub const DAY_MS: i64 = 86_400_000;
pub const DEFAULT_MIN_COVERAGE_BPS: i64 = 1000;
pub const DEFAULT_MAX_COVERAGE_BPS: i64 = 9000;
pub const DEFAULT_MAX_DURATION_MS: i64 = 2_592_000_000;
pub const DEFAULT_FEE_BPS: i64 = 50;

/// Query result for an insurance policy (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Query result for an insurance vault (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct InsuranceVaultRow {
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
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = insurance_config)]
pub struct InsuranceConfig {
    pub id: i32,
    pub updated_by: String,
    pub min_coverage_bps: i64,
    pub max_coverage_bps: i64,
    pub max_duration_ms: i64,
    pub fee_bps: i64,
    pub version: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub enable_flag: bool,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_config)]
pub struct NewInsuranceConfig {
    pub updated_by: String,
    pub enable_flag: bool,
    pub min_coverage_bps: i64,
    pub max_coverage_bps: i64,
    pub max_duration_ms: i64,
    pub fee_bps: i64,
    pub version: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = insurance_vaults)]
pub struct InsuranceVault {
    pub vault_id: String,
    pub underwriter: String,
    pub capital_balance: i64,
    pub reserved: i64,
    pub base_rate_bps_per_day: i64,
    pub utilization_multiplier_bps: i64,
    pub max_exposure_per_market: i64,
    pub max_exposure_per_user: i64,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_vaults)]
pub struct NewInsuranceVault {
    pub vault_id: String,
    pub underwriter: String,
    pub capital_balance: i64,
    pub reserved: i64,
    pub base_rate_bps_per_day: i64,
    pub utilization_multiplier_bps: i64,
    pub max_exposure_per_market: i64,
    pub max_exposure_per_user: i64,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = insurance_vaults)]
pub struct UpdateInsuranceVault {
    pub capital_balance: Option<i64>,
    pub reserved: Option<i64>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = insurance_policies)]
pub struct InsurancePolicy {
    pub policy_id: String,
    pub market_id: String,
    pub insured: String,
    pub option_id: i16,
    pub covered_amount: i64,
    pub coverage_bps: i64,
    pub premium_paid: i64,
    pub start_time_ms: i64,
    pub expiry_time_ms: i64,
    pub vault_id: String,
    pub status: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_policies)]
pub struct NewInsurancePolicy {
    pub policy_id: String,
    pub market_id: String,
    pub insured: String,
    pub option_id: i16,
    pub covered_amount: i64,
    pub coverage_bps: i64,
    pub premium_paid: i64,
    pub start_time_ms: i64,
    pub expiry_time_ms: i64,
    pub vault_id: String,
    pub status: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = insurance_policies)]
pub struct UpdateInsurancePolicy {
    pub status: Option<i16>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_events)]
pub struct NewInsuranceEventLog {
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_vault_transactions)]
pub struct NewInsuranceVaultTransaction {
    pub vault_id: String,
    pub transaction_type: String,
    pub amount: i64,
    pub balance_after: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_policy_events)]
pub struct NewInsurancePolicyEvent {
    pub policy_id: String,
    pub event_type: String,
    pub market_id: String,
    pub insured: String,
    pub option_id: i16,
    pub covered_amount: i64,
    pub coverage_bps: i64,
    pub premium_paid: i64,
    pub reserve_locked: i64,
    pub refunded_amount: Option<i64>,
    pub fee_paid: Option<i64>,
    pub payout: Option<i64>,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_market_exposures)]
pub struct NewInsuranceMarketExposure {
    pub vault_id: String,
    pub market_id: String,
    pub option_id: i16,
    pub reserved_amount: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_user_exposures)]
pub struct NewInsuranceUserExposure {
    pub vault_id: String,
    pub insured: String,
    pub reserved_amount: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}
