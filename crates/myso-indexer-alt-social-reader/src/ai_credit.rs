// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
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
