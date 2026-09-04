// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::Text;
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{
    PlatformRevenueBreakdownRow, PlatformRevenueSummaryRow,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

pub(crate) async fn get_platform_revenue_summary(
    conn: &mut Connection<'_>,
    platform_address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PlatformRevenueSummaryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT platform_address, total_revenue, total_subscription_revenue, total_mydata_revenue,
               total_spt_revenue, total_messaging_revenue, total_username_marketplace_revenue,
               total_transactions, total_creators,
               total_payers, avg_transaction_amount, active_months, last_active_month
        FROM platform_revenue_summary
        WHERE platform_address = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(platform_address)
        .get_result::<PlatformRevenueSummaryRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_platform_revenue_breakdown(
    conn: &mut Connection<'_>,
    platform_address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PlatformRevenueBreakdownRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT platform_address, revenue_source, currency, total_amount, transaction_count
        FROM platform_revenue_by_source_currency
        WHERE platform_address = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(platform_address)
        .load::<PlatformRevenueBreakdownRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(result)
}
