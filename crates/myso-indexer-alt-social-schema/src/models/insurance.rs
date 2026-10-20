// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, SmallInt, Text, Timestamp};
use serde::{Deserialize, Serialize};

use crate::schema::{
    insurance_config, insurance_coverage_routes, insurance_events, insurance_market_exposures,
    insurance_policies, insurance_policy_events, insurance_route_fills, insurance_router_config,
    insurance_user_exposures, insurance_vault_transactions, insurance_vaults,
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
pub const DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY: i64 = 1;
pub const DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS: i64 = 10_000;
pub const DEFAULT_MAX_RISK_MULTIPLIER_BPS: i64 = 500_000;
pub const DEFAULT_MIN_PREMIUM_AMOUNT: i64 = 1;
pub const DEFAULT_SPOT_SMOOTHING_PER_OPTION: i64 = 0;
pub const DEFAULT_IMPLIED_PROB_FLOOR_BPS: i64 = 10;
pub const DEFAULT_ODDS_CAP_BPS: i64 = 500_000;
pub const DEFAULT_LIQ_CAP_BPS: i64 = 500_000;
pub const DEFAULT_LIQ_REF_AMOUNT: i64 = 1_000_000_000_000;
pub const DEFAULT_EXPOSURE_CAP_BPS: i64 = 30_000;
pub const DEFAULT_EXPOSURE_K_BPS: i64 = 5000;

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
    pub premium_raw: i64,
    #[diesel(sql_type = BigInt)]
    pub implied_probability_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub risk_multiplier_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub base_premium: i64,
    #[diesel(sql_type = BigInt)]
    pub market_total_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub option_escrow_amount: i64,
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
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    pub route_id: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<SmallInt>)]
    pub route_leg_index: Option<i16>,
    #[diesel(sql_type = BigInt)]
    pub backstop_sweep_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub contract_version: i64,
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
    #[diesel(sql_type = BigInt)]
    pub max_exposure_per_option: i64,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub enabled: bool,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub paused: bool,
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
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub insurance_enabled: bool,
    pub min_spot_total_liquidity: i64,
    pub max_coverage_fraction_of_option_bps: i64,
    pub max_risk_multiplier_bps: i64,
    pub min_premium_amount: i64,
    pub spot_smoothing_per_option: i64,
    pub implied_prob_floor_bps: i64,
    pub odds_floor_1x: bool,
    pub odds_cap_bps: i64,
    pub liq_cap_bps: i64,
    pub liq_ref_amount: i64,
    pub exposure_cap_bps: i64,
    pub exposure_k_bps: i64,
    pub odds_base_bps: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_config)]
pub struct NewInsuranceConfig {
    pub updated_by: String,
    pub insurance_enabled: bool,
    pub min_coverage_bps: i64,
    pub max_coverage_bps: i64,
    pub max_duration_ms: i64,
    pub fee_bps: i64,
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub min_spot_total_liquidity: i64,
    pub max_coverage_fraction_of_option_bps: i64,
    pub max_risk_multiplier_bps: i64,
    pub min_premium_amount: i64,
    pub spot_smoothing_per_option: i64,
    pub implied_prob_floor_bps: i64,
    pub odds_floor_1x: bool,
    pub odds_cap_bps: i64,
    pub liq_cap_bps: i64,
    pub liq_ref_amount: i64,
    pub exposure_cap_bps: i64,
    pub exposure_k_bps: i64,
    pub odds_base_bps: i64,
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
    pub max_exposure_per_option: i64,
    pub enabled: bool,
    pub paused: bool,
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
    pub max_exposure_per_option: i64,
    pub enabled: bool,
    pub paused: bool,
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

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = insurance_vaults)]
pub struct UpdateInsuranceVaultStatus {
    pub max_exposure_per_option: i64,
    pub enabled: bool,
    pub paused: bool,
    pub max_exposure_per_market: i64,
    pub max_exposure_per_user: i64,
    pub base_rate_bps_per_day: i64,
    pub utilization_multiplier_bps: i64,
    pub updated_at: NaiveDateTime,
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
    pub premium_raw: i64,
    pub implied_probability_bps: i64,
    pub risk_multiplier_bps: i64,
    pub base_premium: i64,
    pub market_total_amount: i64,
    pub option_escrow_amount: i64,
    pub start_time_ms: i64,
    pub expiry_time_ms: i64,
    pub vault_id: String,
    pub status: i16,
    pub route_id: Option<String>,
    pub route_leg_index: Option<i16>,
    pub backstop_sweep_amount: i64,
    pub contract_version: i64,
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
    pub premium_raw: i64,
    pub implied_probability_bps: i64,
    pub risk_multiplier_bps: i64,
    pub base_premium: i64,
    pub market_total_amount: i64,
    pub option_escrow_amount: i64,
    pub start_time_ms: i64,
    pub expiry_time_ms: i64,
    pub vault_id: String,
    pub status: i16,
    pub route_id: Option<String>,
    pub route_leg_index: Option<i16>,
    pub backstop_sweep_amount: i64,
    pub contract_version: i64,
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
    pub premium_raw: Option<i64>,
    pub implied_probability_bps: Option<i64>,
    pub risk_multiplier_bps: Option<i64>,
    pub base_premium: Option<i64>,
    pub market_total_amount: Option<i64>,
    pub option_escrow_amount: Option<i64>,
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

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_coverage_routes)]
pub struct NewInsuranceCoverageRoute {
    pub route_id: String,
    pub insured: String,
    pub market_id: String,
    pub option_id: i16,
    pub coverage_bps: i64,
    pub duration_ms: i64,
    pub total_covered: i64,
    pub total_premium: i64,
    pub total_reserve: i64,
    pub total_backstop_sweep: i64,
    pub expiry_time_ms: i64,
    #[diesel(sql_type = diesel::sql_types::Jsonb)]
    pub policy_ids: serde_json::Value,
    #[diesel(sql_type = diesel::sql_types::Jsonb)]
    pub vault_ids: serde_json::Value,
    pub contract_version: i64,
    pub transaction_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_route_fills)]
pub struct NewInsuranceRouteFill {
    pub route_id: String,
    pub leg_index: i16,
    pub vault_id: String,
    pub policy_id: String,
    pub covered_amount: i64,
    pub premium_paid: i64,
    pub reserve_locked: i64,
    pub backstop_sweep_amount: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub timestamp_ms: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_router_config)]
pub struct NewInsuranceRouterConfig {
    pub updated_by: String,
    pub paused: bool,
    pub max_route_reserve_market: i64,
    pub max_route_reserve_user: i64,
    pub max_route_reserve_option: i64,
    pub max_vault_concentration_bps: i64,
    pub min_vault_health_factor_bps: i64,
    pub max_route_legs: i64,
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewInsuranceRouterConfig {
    pub fn from_event(
        updated_by: String,
        paused: bool,
        max_route_reserve_market: u64,
        max_route_reserve_user: u64,
        max_route_reserve_option: u64,
        max_vault_concentration_bps: u64,
        min_vault_health_factor_bps: u64,
        max_route_legs: u64,
        version: u64,
        updated_at: u64,
        transaction_id: String,
    ) -> Self {
        let time = chrono::DateTime::<chrono::Utc>::from_timestamp((updated_at / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now);
        Self {
            updated_by,
            paused,
            max_route_reserve_market: max_route_reserve_market as i64,
            max_route_reserve_user: max_route_reserve_user as i64,
            max_route_reserve_option: max_route_reserve_option as i64,
            max_vault_concentration_bps: max_vault_concentration_bps as i64,
            min_vault_health_factor_bps: min_vault_health_factor_bps as i64,
            max_route_legs: max_route_legs as i64,
            version: version as i64,
            updated_at: updated_at as i64,
            time,
            transaction_id,
        }
    }
}
