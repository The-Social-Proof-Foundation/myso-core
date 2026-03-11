// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Nullable, SmallInt, Text};
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::{
    InsuranceConfigInfo, InsurancePolicyInfo, InsurancePolicyRow, InsuranceVaultExposureRow,
    InsuranceVaultInfo, InsuranceVaultRow, InsuranceVaultTransactionRow,
};

pub(crate) async fn get_insurance_configuration(
    db: &Db,
) -> Result<Option<InsuranceConfigInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT updated_by, enable_flag, min_coverage_bps, max_coverage_bps, max_duration_ms,
               fee_bps, version, timestamp_ms, time, transaction_id
        FROM insurance_config
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<InsuranceConfigInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn list_insurance_vaults(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<InsuranceVaultRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT vault_id, underwriter, capital_balance, reserved
        FROM insurance_vaults
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsuranceVaultRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_insurance_vault(
    db: &Db,
    vault_id: &str,
) -> Result<Option<InsuranceVaultInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT vault_id, underwriter, capital_balance, reserved, base_rate_bps_per_day,
               utilization_multiplier_bps, max_exposure_per_market, max_exposure_per_user,
               version, created_at, updated_at
        FROM insurance_vaults
        WHERE vault_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .get_result::<InsuranceVaultInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn list_insurance_vault_transactions(
    db: &Db,
    vault_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<InsuranceVaultTransactionRow>, SocialError> {
    let mut conn = db.connect().await?;
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
        .load::<InsuranceVaultTransactionRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_insurance_vault_exposures(
    db: &Db,
    vault_id: &str,
) -> Result<Vec<InsuranceVaultExposureRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT market_id, option_id, SUM(reserved_amount) as total_exposure
        FROM insurance_market_exposures
        WHERE vault_id = $1
        GROUP BY market_id, option_id
        ORDER BY total_exposure DESC
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .load::<InsuranceVaultExposureRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_insurance_policies(
    db: &Db,
    insured: Option<&str>,
    market_id: Option<&str>,
    vault_id: Option<&str>,
    status: Option<i16>,
    limit: i64,
    offset: i64,
) -> Result<Vec<InsurancePolicyRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT policy_id, market_id, insured, option_id, covered_amount, premium_paid, status
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
        .load::<InsurancePolicyRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_insurance_policy(
    db: &Db,
    policy_id: &str,
) -> Result<Option<InsurancePolicyInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT policy_id, market_id, insured, option_id, covered_amount, coverage_bps,
               premium_paid, start_time_ms, expiry_time_ms, vault_id, status
        FROM insurance_policies
        WHERE policy_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(policy_id)
        .get_result::<InsurancePolicyInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn list_insurance_market_policies(
    db: &Db,
    market_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<InsurancePolicyRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT policy_id, market_id, insured, option_id, covered_amount, premium_paid, status
        FROM insurance_policies
        WHERE market_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(market_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<InsurancePolicyRow>(&mut conn)
        .await?;
    Ok(results)
}
