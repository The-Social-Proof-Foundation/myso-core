// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct SubscriptionConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub billing_period_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub max_renewal_months: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub ecosystem_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_creator_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_treasury_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Latest subscription configuration.
pub(crate) async fn get_subscription_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SubscriptionConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, billing_period_ms, max_renewal_months,
               platform_fee_bps, ecosystem_fee_bps,
               non_platform_platform_to_creator_bps, non_platform_platform_to_treasury_bps,
               version, updated_at, time, transaction_id
        FROM subscription_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<SubscriptionConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}
