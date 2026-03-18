// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{InsurancePolicyRow, InsuranceVaultRow};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

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
