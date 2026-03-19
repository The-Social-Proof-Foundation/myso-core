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
    SptHoldingRow, SptPoolRow, SptPriceHistory, SptTransaction, UserReservationHoldingRow,
};
use myso_indexer_alt_social_schema::schema::{spt_price_history, spt_transactions};

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SptSortBy {
    Price,
    MarketCap,
    PriceChange24h,
    Volume24h,
    CreatorEarnings,
    PlatformEarnings,
    EcosystemEarnings,
    TotalEarnings,
    CreatedAt,
}

fn order_by_clause(sort_by: SptSortBy, ascending: bool) -> String {
    let col = match sort_by {
        SptSortBy::Price => "price",
        SptSortBy::MarketCap => "market_cap",
        SptSortBy::PriceChange24h => "price_change_24h",
        SptSortBy::Volume24h => "volume_24h",
        SptSortBy::CreatorEarnings => "creator_earnings",
        SptSortBy::PlatformEarnings => "platform_earnings",
        SptSortBy::EcosystemEarnings => "ecosystem_earnings",
        SptSortBy::TotalEarnings => "total_earnings",
        SptSortBy::CreatedAt => "created_at",
    };
    let dir = if ascending {
        "ASC NULLS LAST"
    } else {
        "DESC NULLS FIRST"
    };
    format!("{} {}", col, dir)
}

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
            SELECT DISTINCT ON (owner_address) owner_address, username, display_name, profile_photo,
                   bio, selected_badge_id, social_proof_token_address, reservation_pool_address
            FROM profiles
            WHERE owner_address IN (SELECT owner FROM latest_pools)
            ORDER BY owner_address, updated_at DESC
        )
        SELECT h.holder_address, h.pool_id, h.balance, p.owner as profile_owner_address,
               pr.username as profile_username, pr.display_name as profile_display_name,
               pr.profile_photo as profile_photo, pr.bio as profile_bio,
               pr.selected_badge_id as profile_selected_badge_id,
               pr.social_proof_token_address as profile_social_proof_token_address,
               pr.reservation_pool_address as profile_reservation_pool_address
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

pub(crate) async fn get_spt_holdings_by_pool(
    conn: &mut Connection<'_>,
    pool_id: &str,
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
            WHERE pool_id = $1
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
            SELECT DISTINCT ON (owner_address) owner_address, username, display_name, profile_photo,
                   bio, selected_badge_id, social_proof_token_address, reservation_pool_address
            FROM profiles
            WHERE owner_address IN (SELECT owner FROM latest_pools)
            ORDER BY owner_address, updated_at DESC
        )
        SELECT h.holder_address, h.pool_id, h.balance, p.owner as profile_owner_address,
               pr.username as profile_username, pr.display_name as profile_display_name,
               pr.profile_photo as profile_photo, pr.bio as profile_bio,
               pr.selected_badge_id as profile_selected_badge_id,
               pr.social_proof_token_address as profile_social_proof_token_address,
               pr.reservation_pool_address as profile_reservation_pool_address
        FROM holdings h
        JOIN latest_pools p ON h.pool_id = p.pool_id
        LEFT JOIN latest_profiles pr ON p.owner = pr.owner_address
        ORDER BY h.balance DESC
        LIMIT $2 OFFSET $3
    "#;

    let results = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
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
        ),
        price_24h AS (
            SELECT price FROM spt_price_history
            WHERE pool_id = $1 AND time <= NOW() - INTERVAL '24 hours'
            ORDER BY time DESC
            LIMIT 1
        ),
        vol_24h AS (
            SELECT COALESCE(SUM(myso_amount), 0)::bigint as vol
            FROM spt_transactions
            WHERE pool_id = $1 AND time >= NOW() - INTERVAL '24 hours'
        ),
        rev AS (
            SELECT
                COALESCE(SUM(creator_fee), 0)::bigint as creator_earnings,
                COALESCE(SUM(platform_fee), 0)::bigint as platform_earnings,
                COALESCE(SUM(treasury_fee), 0)::bigint as ecosystem_earnings
            FROM spt_revenue
            WHERE pool_id = $1
        )
        SELECT p.pool_id, p.token_type, p.owner, p.associated_id, p.symbol, p.name,
               p.circulating_supply, p.base_price, p.quadratic_coefficient, p.created_at,
               p.time, p.transaction_id, COALESCE(lp.price, 0)::bigint as price,
               ph24.price as price_24h_ago,
               v.vol as volume_24h,
               r.creator_earnings,
               r.platform_earnings,
               r.ecosystem_earnings
        FROM latest_pool p
        LEFT JOIN latest_price lp ON true
        LEFT JOIN price_24h ph24 ON true
        LEFT JOIN vol_24h v ON true
        LEFT JOIN rev r ON true
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

pub(crate) async fn list_spt_pools(
    conn: &mut Connection<'_>,
    token_type: Option<i16>,
    sort_by: SptSortBy,
    ascending: bool,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptPoolRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let token_filter = token_type
        .map(|t| format!("AND token_type = {}", t))
        .unwrap_or_default();
    let order_clause = order_by_clause(sort_by, ascending);

    let query = format!(
        r#"
        WITH latest_pools AS (
            SELECT DISTINCT ON (pool_id) pool_id, token_type, owner, associated_id, symbol, name,
                   circulating_supply, base_price, quadratic_coefficient, created_at, time, transaction_id
            FROM spt_pools
            WHERE 1=1 {}
            ORDER BY pool_id, time DESC
        ),
        pool_metrics AS (
            SELECT
                p.pool_id,
                p.token_type,
                p.owner,
                p.associated_id,
                p.symbol,
                p.name,
                p.circulating_supply,
                p.base_price,
                p.quadratic_coefficient,
                p.created_at,
                p.time,
                p.transaction_id,
                COALESCE(ph.price, 0)::bigint as price,
                ph24.price as price_24h_ago,
                COALESCE(v.vol, 0)::bigint as volume_24h,
                COALESCE(r.creator_earnings, 0)::bigint as creator_earnings,
                COALESCE(r.platform_earnings, 0)::bigint as platform_earnings,
                COALESCE(r.ecosystem_earnings, 0)::bigint as ecosystem_earnings,
                (COALESCE(ph.price, 0) * p.circulating_supply)::bigint as market_cap,
                CASE WHEN ph24.price IS NOT NULL AND ph24.price > 0
                    THEN ((COALESCE(ph.price, 0) - ph24.price)::float / ph24.price * 100)
                    ELSE NULL
                END as price_change_24h,
                (COALESCE(r.creator_earnings, 0) + COALESCE(r.platform_earnings, 0) + COALESCE(r.ecosystem_earnings, 0))::bigint as total_earnings
            FROM latest_pools p
            LEFT JOIN LATERAL (
                SELECT price FROM spt_price_history
                WHERE pool_id = p.pool_id ORDER BY time DESC LIMIT 1
            ) ph ON true
            LEFT JOIN LATERAL (
                SELECT price FROM spt_price_history
                WHERE pool_id = p.pool_id AND time <= NOW() - INTERVAL '24 hours'
                ORDER BY time DESC LIMIT 1
            ) ph24 ON true
            LEFT JOIN LATERAL (
                SELECT COALESCE(SUM(myso_amount), 0)::bigint as vol
                FROM spt_transactions
                WHERE pool_id = p.pool_id AND time >= NOW() - INTERVAL '24 hours'
            ) v ON true
            LEFT JOIN LATERAL (
                SELECT
                    COALESCE(SUM(creator_fee), 0)::bigint as creator_earnings,
                    COALESCE(SUM(platform_fee), 0)::bigint as platform_earnings,
                    COALESCE(SUM(treasury_fee), 0)::bigint as ecosystem_earnings
                FROM spt_revenue WHERE pool_id = p.pool_id
            ) r ON true
        )
        SELECT pool_id, token_type, owner, associated_id, symbol, name, circulating_supply,
               base_price, quadratic_coefficient, created_at, time, transaction_id, price,
               price_24h_ago, volume_24h, creator_earnings, platform_earnings, ecosystem_earnings
        FROM pool_metrics
        ORDER BY {}
        LIMIT $1 OFFSET $2
        "#,
        token_filter,
        order_clause
    );

    let results = diesel::sql_query(&query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptPoolRow>(conn)
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

pub(crate) async fn get_user_reservation_holdings(
    conn: &mut Connection<'_>,
    reserver_address: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<UserReservationHoldingRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = r#"
        SELECT urh.reserver_address, urh.pool_id, urh.associated_id, urh.token_type, urh.owner,
               urh.amount, urh.reserved_at, urh.total_reserved, urh.required_threshold,
               urh.threshold_met, urh.pool_status,
               p.username as profile_username, p.display_name as profile_display_name,
               p.profile_photo as profile_photo, p.social_proof_token_address as profile_social_proof_token_address,
               p.reservation_pool_address as profile_reservation_pool_address
        FROM user_reservation_holdings urh
        LEFT JOIN LATERAL (
            SELECT username, display_name, profile_photo, social_proof_token_address, reservation_pool_address
            FROM profiles
            WHERE owner_address = urh.owner
            ORDER BY updated_at DESC
            LIMIT 1
        ) p ON true
        WHERE urh.reserver_address = $1
        ORDER BY urh.reserved_at DESC
        LIMIT $2 OFFSET $3
    "#;

    let results = diesel::sql_query(query)
        .bind::<Text, _>(reserver_address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<UserReservationHoldingRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}
