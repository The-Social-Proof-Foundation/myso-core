// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{
    BigInt, Bool, Integer, Jsonb, Nullable, SmallInt, Text, Timestamp, Timestamptz,
};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{InsurancePolicyRow, InsuranceVaultRow};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct InsuranceConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub insurance_enabled: bool,
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
    pub updated_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = BigInt)]
    pub min_spot_total_liquidity: i64,
    #[diesel(sql_type = BigInt)]
    pub max_coverage_fraction_of_option_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_risk_multiplier_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub min_premium_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub spot_smoothing_per_option: i64,
    #[diesel(sql_type = BigInt)]
    pub implied_prob_floor_bps: i64,
    #[diesel(sql_type = Bool)]
    pub odds_floor_1x: bool,
    #[diesel(sql_type = BigInt)]
    pub odds_cap_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub liq_cap_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub liq_ref_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub exposure_cap_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub exposure_k_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub odds_base_bps: i64,
}

#[derive(Debug, Clone, QueryableByName)]
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

#[derive(Debug, Clone, QueryableByName)]
pub struct InsuranceVaultExposureRow {
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub total_exposure: i64,
}

pub(crate) async fn get_insurance_policy(
    conn: &mut Connection<'_>,
    policy_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<InsurancePolicyRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT policy_id, market_id, insured, option_id, covered_amount, coverage_bps,
               premium_paid, premium_raw, implied_probability_bps, risk_multiplier_bps,
               base_premium, market_total_amount, option_escrow_amount,
               start_time_ms, expiry_time_ms, vault_id, status,
               route_id, route_leg_index, backstop_sweep_amount,
               created_at, updated_at, transaction_id
        FROM insurance_policies
        WHERE policy_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(policy_id)
        .get_result::<InsurancePolicyRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_insurance_policies_by_insured(
    conn: &mut Connection<'_>,
    insured: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsurancePolicyRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT policy_id, market_id, insured, option_id, covered_amount, coverage_bps,
               premium_paid, premium_raw, implied_probability_bps, risk_multiplier_bps,
               base_premium, market_total_amount, option_escrow_amount,
               start_time_ms, expiry_time_ms, vault_id, status,
               route_id, route_leg_index, backstop_sweep_amount,
               created_at, updated_at, transaction_id
        FROM insurance_policies
        WHERE insured = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(insured)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsurancePolicyRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_insurance_vault(
    conn: &mut Connection<'_>,
    vault_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<InsuranceVaultRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT vault_id, underwriter, capital_balance, reserved, base_rate_bps_per_day,
               utilization_multiplier_bps, max_exposure_per_market, max_exposure_per_user,
               max_exposure_per_option, enabled, paused,
               version, created_at, updated_at, transaction_id
        FROM insurance_vaults
        WHERE vault_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .get_result::<InsuranceVaultRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_insurance_vaults(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsuranceVaultRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT vault_id, underwriter, capital_balance, reserved, base_rate_bps_per_day,
               utilization_multiplier_bps, max_exposure_per_market, max_exposure_per_user,
               max_exposure_per_option, enabled, paused,
               version, created_at, updated_at, transaction_id
        FROM insurance_vaults
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";

    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsuranceVaultRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_insurance_vault_transactions(
    conn: &mut Connection<'_>,
    vault_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsuranceVaultTransactionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT transaction_type, amount, balance_after, timestamp_ms
        FROM insurance_vault_transactions
        WHERE vault_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsuranceVaultTransactionRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_insurance_vault_exposures(
    conn: &mut Connection<'_>,
    vault_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsuranceVaultExposureRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT market_id, option_id, SUM(reserved_amount) as total_exposure
        FROM insurance_market_exposures
        WHERE vault_id = $1
        GROUP BY market_id, option_id
        ORDER BY total_exposure DESC
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .load::<InsuranceVaultExposureRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_insurance_policies(
    conn: &mut Connection<'_>,
    insured: Option<&str>,
    market_id: Option<&str>,
    vault_id: Option<&str>,
    status: Option<i16>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsurancePolicyRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT policy_id, market_id, insured, option_id, covered_amount, coverage_bps,
               premium_paid, premium_raw, implied_probability_bps, risk_multiplier_bps,
               base_premium, market_total_amount, option_escrow_amount,
               start_time_ms, expiry_time_ms, vault_id, status,
               route_id, route_leg_index, backstop_sweep_amount,
               created_at, updated_at, transaction_id
        FROM insurance_policies
        WHERE ($1::text IS NULL OR insured = $1)
          AND ($2::text IS NULL OR market_id = $2)
          AND ($3::text IS NULL OR vault_id = $3)
          AND ($4::smallint IS NULL OR status = $4)
        ORDER BY created_at DESC
        LIMIT $5 OFFSET $6
    ";

    let results = diesel::sql_query(query)
        .bind::<Nullable<Text>, _>(insured)
        .bind::<Nullable<Text>, _>(market_id)
        .bind::<Nullable<Text>, _>(vault_id)
        .bind::<Nullable<SmallInt>, _>(status)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsurancePolicyRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_insurance_market_policies(
    conn: &mut Connection<'_>,
    market_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsurancePolicyRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT policy_id, market_id, insured, option_id, covered_amount, coverage_bps,
               premium_paid, premium_raw, implied_probability_bps, risk_multiplier_bps,
               base_premium, market_total_amount, option_escrow_amount,
               start_time_ms, expiry_time_ms, vault_id, status,
               route_id, route_leg_index, backstop_sweep_amount,
               created_at, updated_at, transaction_id
        FROM insurance_policies
        WHERE market_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(market_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsurancePolicyRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_insurance_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<InsuranceConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, insurance_enabled, min_coverage_bps, max_coverage_bps, max_duration_ms,
               fee_bps, version, updated_at, time, transaction_id,
               min_spot_total_liquidity, max_coverage_fraction_of_option_bps,
               max_risk_multiplier_bps, min_premium_amount, spot_smoothing_per_option,
               implied_prob_floor_bps, odds_floor_1x, odds_cap_bps, liq_cap_bps, liq_ref_amount,
               exposure_cap_bps, exposure_k_bps, odds_base_bps
        FROM insurance_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<InsuranceConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

#[derive(Debug, Clone, QueryableByName)]
pub struct InsuranceRouterConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub paused: bool,
    #[diesel(sql_type = BigInt)]
    pub max_route_reserve_market: i64,
    #[diesel(sql_type = BigInt)]
    pub max_route_reserve_user: i64,
    #[diesel(sql_type = BigInt)]
    pub max_route_reserve_option: i64,
    #[diesel(sql_type = BigInt)]
    pub max_vault_concentration_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub min_vault_health_factor_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_route_legs: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

pub(crate) async fn get_insurance_router_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<InsuranceRouterConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, paused, max_route_reserve_market,
               max_route_reserve_user, max_route_reserve_option, max_vault_concentration_bps,
               min_vault_health_factor_bps, max_route_legs, version, updated_at, time,
               transaction_id
        FROM insurance_router_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<InsuranceRouterConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}
#[derive(Debug, Clone, QueryableByName)]
pub struct InsuranceModuleEventRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub event_type: String,
    #[diesel(sql_type = Jsonb)]
    pub event_data: serde_json::Value,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Row from `insurance_policy_events` (policy lifecycle timeline).
#[derive(Debug, Clone, QueryableByName)]
pub struct InsurancePolicyEventHistoryRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub policy_id: String,
    #[diesel(sql_type = Text)]
    pub event_type: String,
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
    pub reserve_locked: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub premium_raw: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub implied_probability_bps: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub risk_multiplier_bps: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub base_premium: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub market_total_amount: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub option_escrow_amount: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub refunded_amount: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub fee_paid: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub payout: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct InsuranceUserExposureAggRow {
    #[diesel(sql_type = Text)]
    pub insured: String,
    #[diesel(sql_type = BigInt)]
    pub total_reserved: i64,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct InsuranceCoverageRouteRow {
    #[diesel(sql_type = Text)]
    pub route_id: String,
    #[diesel(sql_type = Text)]
    pub insured: String,
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub duration_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub total_covered: i64,
    #[diesel(sql_type = BigInt)]
    pub total_premium: i64,
    #[diesel(sql_type = BigInt)]
    pub total_reserve: i64,
    #[diesel(sql_type = BigInt)]
    pub total_backstop_sweep: i64,
    #[diesel(sql_type = BigInt)]
    pub expiry_time_ms: i64,
    #[diesel(sql_type = Jsonb)]
    pub policy_ids: serde_json::Value,
    #[diesel(sql_type = Jsonb)]
    pub vault_ids: serde_json::Value,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamp)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct InsuranceRouteFillRow {
    #[diesel(sql_type = BigInt)]
    pub id: i64,
    #[diesel(sql_type = Text)]
    pub route_id: String,
    #[diesel(sql_type = SmallInt)]
    pub leg_index: i16,
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub policy_id: String,
    #[diesel(sql_type = BigInt)]
    pub covered_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub premium_paid: i64,
    #[diesel(sql_type = BigInt)]
    pub reserve_locked: i64,
    #[diesel(sql_type = BigInt)]
    pub backstop_sweep_amount: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Timestamp)]
    pub created_at: chrono::NaiveDateTime,
}

pub(crate) async fn list_insurance_module_events(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsuranceModuleEventRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, event_type, event_data, event_id, created_at
        FROM insurance_events
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";

    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsuranceModuleEventRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_insurance_policy_events_for_policy(
    conn: &mut Connection<'_>,
    policy_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsurancePolicyEventHistoryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, policy_id, event_type, market_id, insured, option_id,
               covered_amount, coverage_bps, premium_paid, reserve_locked,
               premium_raw, implied_probability_bps, risk_multiplier_bps,
               base_premium, market_total_amount, option_escrow_amount,
               refunded_amount, fee_paid, payout,
               timestamp_ms, time, transaction_id
        FROM insurance_policy_events
        WHERE policy_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(policy_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsurancePolicyEventHistoryRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_insurance_user_exposure_totals_for_vault(
    conn: &mut Connection<'_>,
    vault_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsuranceUserExposureAggRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT insured, SUM(reserved_amount)::bigint AS total_reserved
        FROM insurance_user_exposures
        WHERE vault_id = $1
        GROUP BY insured
        ORDER BY total_reserved DESC
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .load::<InsuranceUserExposureAggRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_insurance_coverage_route(
    conn: &mut Connection<'_>,
    route_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<InsuranceCoverageRouteRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT route_id, insured, market_id, option_id, coverage_bps, duration_ms,
               total_covered, total_premium, total_reserve, total_backstop_sweep,
               expiry_time_ms, policy_ids, vault_ids, transaction_id, created_at
        FROM insurance_coverage_routes
        WHERE route_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(route_id)
        .get_result::<InsuranceCoverageRouteRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_insurance_route_fills_for_route(
    conn: &mut Connection<'_>,
    route_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<InsuranceRouteFillRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, route_id, leg_index, vault_id, policy_id, covered_amount,
               premium_paid, reserve_locked, backstop_sweep_amount,
               event_id, transaction_id, timestamp_ms, created_at
        FROM insurance_route_fills
        WHERE route_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(route_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsuranceRouteFillRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}
