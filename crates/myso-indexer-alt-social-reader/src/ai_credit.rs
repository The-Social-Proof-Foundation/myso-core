// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel::sql_types::{BigInt, Nullable, Text, Timestamptz};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::AiCreditBalanceRow;
use myso_indexer_alt_social_schema::schema::ai_credit_balances;
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

pub(crate) async fn get_ai_credit_balance_by_owner(
    conn: &mut Connection<'_>,
    owner: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<AiCreditBalanceRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = ai_credit_balances::table
        .filter(ai_credit_balances::principal_owner.eq(owner))
        .select(AiCreditBalanceRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub(crate) async fn list_ai_credit_usage_lines(
    conn: &mut Connection<'_>,
    balance_id: &str,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::AiCreditUsageLineRow>> {
    use myso_indexer_alt_social_schema::models::AiCreditUsageLineRow;
    use myso_indexer_alt_social_schema::schema::ai_credit_usage_lines;

    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = ai_credit_usage_lines::table
        .filter(ai_credit_usage_lines::balance_id.eq(balance_id))
        .order(ai_credit_usage_lines::created_at.desc())
        .limit(limit)
        .select(AiCreditUsageLineRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

#[derive(Debug, Clone, QueryableByName)]
pub struct AiCreditConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Text)]
    pub oracle_pubkey_hex: String,
    #[diesel(sql_type = Text)]
    pub treasury_address: String,
    #[diesel(sql_type = BigInt)]
    pub min_deposit_mist: i64,
    #[diesel(sql_type = BigInt)]
    pub max_single_settlement_mist: i64,
    #[diesel(sql_type = BigInt)]
    pub receipt_ttl_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub oracle_markup_bps: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub catalog_version: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Latest AI credit configuration.
pub(crate) async fn get_ai_credit_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<AiCreditConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, oracle_pubkey_hex, treasury_address, min_deposit_mist,
               max_single_settlement_mist, receipt_ttl_ms, oracle_markup_bps, catalog_version,
               version, updated_at, time, transaction_id
        FROM ai_credit_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<AiCreditConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}
