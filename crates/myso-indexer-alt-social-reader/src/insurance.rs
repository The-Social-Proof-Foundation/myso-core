// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text, Timestamptz};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{InsurancePolicyRow, InsuranceVaultRow};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct InsuranceConfigRow {
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
               premium_paid, start_time_ms, expiry_time_ms, vault_id, status,
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
               premium_paid, start_time_ms, expiry_time_ms, vault_id, status,
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
               premium_paid, start_time_ms, expiry_time_ms, vault_id, status,
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
               premium_paid, start_time_ms, expiry_time_ms, vault_id, status,
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
        SELECT updated_by, enable_flag, min_coverage_bps, max_coverage_bps, max_duration_ms,
               fee_bps, version, timestamp_ms, time, transaction_id
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
