// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    SptHoldingRow, SptPoolRow, SptPriceHistory, SptTransaction,
};
use myso_indexer_alt_social_schema::schema::{spt_price_history, spt_transactions};

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;


pub(crate) async fn get_spt_holdings_by_holder(
    conn: &mut Connection<'_>,
    holder_address: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptHoldingRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = r#"
        WITH holdings AS (
            SELECT pool_id, holder_address, SUM(amount)::bigint as balance
            FROM spt_holdings
            WHERE holder_address = $1
            GROUP BY pool_id, holder_address
            HAVING SUM(amount) != 0
        ),
        latest_pools AS (
            SELECT DISTINCT ON (p.pool_id) p.pool_id, p.owner
            FROM spt_pools p
            WHERE p.pool_id IN (SELECT pool_id FROM holdings)
            ORDER BY p.pool_id, time DESC
        ),
        latest_profiles AS (
            SELECT DISTINCT ON (owner_address) owner_address, username, display_name, profile_photo
            FROM profiles
            WHERE owner_address IN (SELECT owner FROM latest_pools)
            ORDER BY owner_address, updated_at DESC
        )
        SELECT h.holder_address, h.pool_id, h.balance, p.owner as profile_owner_address,
               pr.username as profile_username, pr.display_name as profile_display_name,
               pr.profile_photo as profile_photo
        FROM holdings h
        JOIN latest_pools p ON h.pool_id = p.pool_id
        LEFT JOIN latest_profiles pr ON p.owner = pr.owner_address
        ORDER BY h.balance DESC
        LIMIT $2 OFFSET $3
    "#;

    let results = diesel::sql_query(query)
        .bind::<Text, _>(holder_address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptHoldingRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_spt_pool(
    conn: &mut Connection<'_>,
    pool_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SptPoolRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = r#"
        WITH latest_pool AS (
            SELECT pool_id, token_type, owner, associated_id, symbol, name, circulating_supply,
                   base_price, quadratic_coefficient, created_at, time, transaction_id
            FROM spt_pools
            WHERE pool_id = $1
            ORDER BY time DESC
            LIMIT 1
        ),
        latest_price AS (
            SELECT price FROM spt_price_history
            WHERE pool_id = $1
            ORDER BY time DESC
            LIMIT 1
        )
        SELECT p.pool_id, p.token_type, p.owner, p.associated_id, p.symbol, p.name,
               p.circulating_supply, p.base_price, p.quadratic_coefficient, p.created_at,
               p.time, p.transaction_id, COALESCE(lp.price, 0)::bigint as price
        FROM latest_pool p
        LEFT JOIN latest_price lp ON true
    "#;

    let result = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .get_result::<SptPoolRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_spt_transactions(
    conn: &mut Connection<'_>,
    pool_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptTransaction>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let results = spt_transactions::table
        .filter(spt_transactions::pool_id.eq(pool_id))
        .order(spt_transactions::time.desc())
        .limit(limit)
        .offset(offset)
        .select(SptTransaction::as_select())
        .load::<SptTransaction>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_spt_price_history(
    conn: &mut Connection<'_>,
    pool_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptPriceHistory>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let results = spt_price_history::table
        .filter(spt_price_history::pool_id.eq(pool_id))
        .order(spt_price_history::time.desc())
        .limit(limit)
        .offset(offset)
        .select(SptPriceHistory::as_select())
        .load::<SptPriceHistory>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_spt_pool_id_for_profile(
    conn: &mut Connection<'_>,
    profile_address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<String>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let associated_id = format!("profile_{}", profile_address);

    #[derive(QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct PoolIdRow {
        #[diesel(sql_type = Text)]
        pool_id: String,
    }

    let result = diesel::sql_query(
        "SELECT pool_id FROM spt_pools WHERE associated_id = $1 ORDER BY time DESC LIMIT 1",
    )
    .bind::<Text, _>(&associated_id)
    .get_result::<PoolIdRow>(conn)
    .await
    .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result.map(|r| r.pool_id))
}
