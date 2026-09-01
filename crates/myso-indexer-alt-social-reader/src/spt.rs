// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel::sql_types::{Array, BigInt, Bool, Integer, Nullable, Text, Timestamptz};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    SptHoldingRow, SptPoolRow, SptPriceHistory, SptReservationHoldingRow, SptSwap, SptTransaction,
    SptTransfer,
};
use myso_indexer_alt_social_schema::schema::{
    spt_price_history, spt_swaps, spt_transactions, spt_transfers,
};

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;
use crate::social_graph::{
    ViewerSocialContext, batch_viewer_social_context, resolve_profile_address,
};

const SPT_HOLDING_VIEWER_NULLS: &str = ", NULL::boolean AS viewer_is_following, NULL::boolean AS viewer_follows_viewer, \
     NULL::boolean AS blocked_by_viewer, NULL::boolean AS blocked_by_subject";

const SPT_RESERVATION_VIEWER_NULLS: &str = SPT_HOLDING_VIEWER_NULLS;

/// SQL fragment: aggregate creator/platform/ecosystem earnings for a trading pool and its
/// linked reservation pool(s). Each fee event is stored under exactly one pool_id at index time.
pub(crate) const SPT_EARNINGS_BY_ASSOCIATED_ID_LATERAL: &str = r#"
    SELECT
        COALESCE(SUM(creator_fee), 0)::bigint AS creator_earnings,
        COALESCE(SUM(platform_fee), 0)::bigint AS platform_earnings,
        COALESCE(SUM(treasury_fee), 0)::bigint AS ecosystem_earnings
    FROM spt_revenue
    WHERE pool_id = p.pool_id
       OR pool_id IN (
           SELECT pool_id FROM spt_reservation_pools
           WHERE associated_id = p.associated_id
       )
"#;

/// SPT transactions for a pool with optional batched per-sender [`ViewerSocialContext`].
#[derive(Debug, Clone)]
pub struct SptTransactionsWithViewer {
    pub transactions: Vec<SptTransaction>,
    pub viewer_by_sender: Option<HashMap<String, ViewerSocialContext>>,
}

fn viewer_ref_strings(viewer_owner: &str, viewer_profile_id: &Option<String>) -> Vec<String> {
    let mut viewer_refs = vec![viewer_owner.to_string()];
    if let Some(pid) = viewer_profile_id {
        if pid != viewer_owner {
            viewer_refs.push(pid.clone());
        }
    }
    viewer_refs
}

async fn apply_viewer_context_to_holding_rows(
    conn: &mut Connection<'_>,
    rows: &mut [SptHoldingRow],
    viewer: &str,
) -> anyhow::Result<()> {
    let (v_pid, v_owner) = resolve_profile_address(conn, viewer).await?;
    let addrs: Vec<String> = rows.iter().map(|r| r.holder_address.clone()).collect();
    let ctx = batch_viewer_social_context(conn, &addrs, &v_pid, &v_owner).await?;
    for r in rows.iter_mut() {
        if let Some(c) = ctx.get(&r.holder_address) {
            r.viewer_is_following = Some(c.is_following);
            r.viewer_follows_viewer = Some(c.follows_viewer);
            r.blocked_by_viewer = Some(c.blocked_by_viewer);
            r.blocked_by_subject = Some(c.blocked_by_subject);
        }
    }
    Ok(())
}

async fn apply_viewer_context_to_reservation_rows(
    conn: &mut Connection<'_>,
    rows: &mut [SptReservationHoldingRow],
    viewer: &str,
) -> anyhow::Result<()> {
    let (v_pid, v_owner) = resolve_profile_address(conn, viewer).await?;
    let addrs: Vec<String> = rows.iter().map(|r| r.reserver_address.clone()).collect();
    let ctx = batch_viewer_social_context(conn, &addrs, &v_pid, &v_owner).await?;
    for r in rows.iter_mut() {
        if let Some(c) = ctx.get(&r.reserver_address) {
            r.viewer_is_following = Some(c.is_following);
            r.viewer_follows_viewer = Some(c.follows_viewer);
            r.blocked_by_viewer = Some(c.blocked_by_viewer);
            r.blocked_by_subject = Some(c.blocked_by_subject);
        }
    }
    Ok(())
}

#[derive(Debug, Clone, QueryableByName)]
pub struct SptExchangeConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub post_threshold: i64,
    #[diesel(sql_type = BigInt)]
    pub profile_threshold: i64,
    #[diesel(sql_type = BigInt)]
    pub max_individual_reservation_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub total_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub trading_creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub trading_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub trading_treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub reservation_creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub reservation_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub reservation_treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_reservers_per_pool: i64,
    #[diesel(sql_type = BigInt)]
    pub base_price: i64,
    #[diesel(sql_type = BigInt)]
    pub quadratic_coefficient: i64,
    #[diesel(sql_type = BigInt)]
    pub max_hold_percent_bps: i64,
    #[diesel(sql_type = Bool)]
    pub trading_enabled: bool,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_creator_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_treasury_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

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

/// Percent change from `prev` to `current`. `None` when `prev <= 0` (no baseline).
pub fn pct_change(current: i64, prev: i64) -> Option<f64> {
    pct_change_i128(current as i128, prev as i128)
}

/// Percent change for values that may exceed `i64` (e.g. `price * circulating_supply`).
pub fn pct_change_i128(current: i128, prev: i128) -> Option<f64> {
    if prev > 0 {
        Some(((current - prev) as f64 / prev as f64) * 100.0)
    } else {
        None
    }
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

    let query = format!(
        r#"
        WITH holdings AS (
            SELECT pool_id, holder_address, SUM(amount)::bigint as balance
            FROM spt_holdings
            WHERE holder_address = $1
            GROUP BY pool_id, holder_address
            HAVING SUM(amount) != 0
        ),
        latest_pools AS (
            SELECT DISTINCT ON (p.pool_id) p.pool_id, p.owner, p.token_type, p.associated_id
            FROM spt_pools p
            WHERE p.pool_id IN (SELECT pool_id FROM holdings)
            ORDER BY p.pool_id, time DESC
        ),
        latest_profiles AS (
            SELECT DISTINCT ON (owner_address) owner_address, username, display_name, profile_photo,
                   bio, selected_badge_id, social_proof_token_address, reservation_pool_address
            FROM profiles
            WHERE LOWER(TRIM(owner_address)) IN (SELECT LOWER(TRIM(owner)) FROM latest_pools)
            ORDER BY owner_address, updated_at DESC
        )
        SELECT h.holder_address, h.pool_id, h.balance, p.token_type, p.associated_id, p.owner as profile_owner_address,
               pr.username as profile_username, pr.display_name as profile_display_name,
               pr.profile_photo as profile_photo, pr.bio as profile_bio,
               pr.selected_badge_id as profile_selected_badge_id,
               pr.social_proof_token_address as profile_social_proof_token_address,
               pr.reservation_pool_address as profile_reservation_pool_address
               {nulls}
        FROM holdings h
        JOIN latest_pools p ON h.pool_id = p.pool_id
        LEFT JOIN latest_profiles pr ON LOWER(TRIM(p.owner)) = LOWER(TRIM(pr.owner_address))
        ORDER BY h.balance DESC
        LIMIT $2 OFFSET $3
    "#,
        nulls = SPT_HOLDING_VIEWER_NULLS,
    );

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
    viewer: Option<&str>,
    prioritize_followed: bool,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptHoldingRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let base = r#"
        WITH holdings AS (
            SELECT pool_id, holder_address, SUM(amount)::bigint as balance
            FROM spt_holdings
            WHERE pool_id = $1
            GROUP BY pool_id, holder_address
            HAVING SUM(amount) != 0
        ),
        latest_pools AS (
            SELECT DISTINCT ON (p.pool_id) p.pool_id, p.owner, p.token_type, p.associated_id
            FROM spt_pools p
            WHERE p.pool_id IN (SELECT pool_id FROM holdings)
            ORDER BY p.pool_id, time DESC
        ),
        latest_profiles AS (
            SELECT DISTINCT ON (owner_address) owner_address, username, display_name, profile_photo,
                   bio, selected_badge_id, social_proof_token_address, reservation_pool_address
            FROM profiles
            WHERE LOWER(TRIM(owner_address)) IN (SELECT LOWER(TRIM(owner)) FROM latest_pools)
            ORDER BY owner_address, updated_at DESC
        )
        SELECT h.holder_address, h.pool_id, h.balance, p.token_type, p.associated_id, p.owner as profile_owner_address,
               pr.username as profile_username, pr.display_name as profile_display_name,
               pr.profile_photo as profile_photo, pr.bio as profile_bio,
               pr.selected_badge_id as profile_selected_badge_id,
               pr.social_proof_token_address as profile_social_proof_token_address,
               pr.reservation_pool_address as profile_reservation_pool_address
    "#;

    let mut results: Vec<SptHoldingRow> = if prioritize_followed && let Some(v) = viewer {
        let (v_pid, v_owner) = resolve_profile_address(conn, v).await?;
        let refs = viewer_ref_strings(&v_owner, &v_pid);
        let query = format!(
            r#"{} {nulls}
        FROM holdings h
        JOIN latest_pools p ON h.pool_id = p.pool_id
        LEFT JOIN latest_profiles pr ON LOWER(TRIM(p.owner)) = LOWER(TRIM(pr.owner_address))
        ORDER BY (
            EXISTS (
                SELECT 1 FROM social_graph_relationships sgr
                WHERE sgr.follower_address = ANY($4::TEXT[])
                AND sgr.following_address = h.holder_address
            )
        )::int DESC, h.balance DESC
        LIMIT $2 OFFSET $3
    "#,
            base,
            nulls = SPT_HOLDING_VIEWER_NULLS,
        );
        diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .bind::<Array<Text>, _>(&refs)
            .load(conn)
            .await?
    } else {
        let query = format!(
            r#"{} {nulls}
        FROM holdings h
        JOIN latest_pools p ON h.pool_id = p.pool_id
        LEFT JOIN latest_profiles pr ON LOWER(TRIM(p.owner)) = LOWER(TRIM(pr.owner_address))
        ORDER BY h.balance DESC
        LIMIT $2 OFFSET $3
    "#,
            base,
            nulls = SPT_HOLDING_VIEWER_NULLS,
        );
        diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(conn)
            .await?
    };

    if let Some(v) = viewer {
        apply_viewer_context_to_holding_rows(conn, &mut results, v).await?;
    }

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
            SELECT pool_id, token_type, owner, associated_id, circulating_supply,
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
            SELECT price, circulating_supply FROM spt_price_history
            WHERE pool_id = $1 AND time <= NOW() - INTERVAL '24 hours'
            ORDER BY time DESC
            LIMIT 1
        ),
        -- Since-open fallback when no sample exists at/before now-24h (young pools).
        first_price AS (
            SELECT price, circulating_supply FROM spt_price_history
            WHERE pool_id = $1
            ORDER BY time ASC
            LIMIT 1
        ),
        vol_24h AS (
            SELECT COALESCE(SUM(myso_amount), 0)::bigint as vol
            FROM spt_transactions
            WHERE pool_id = $1 AND time >= NOW() - INTERVAL '24 hours'
        ),
        rev AS (
            SELECT
                COALESCE(SUM(sr.creator_fee), 0)::bigint as creator_earnings,
                COALESCE(SUM(sr.platform_fee), 0)::bigint as platform_earnings,
                COALESCE(SUM(sr.treasury_fee), 0)::bigint as ecosystem_earnings
            FROM spt_revenue sr
            WHERE sr.pool_id = $1
               OR sr.pool_id IN (
                   SELECT pool_id FROM spt_reservation_pools rp
                   WHERE rp.associated_id = (SELECT associated_id FROM latest_pool LIMIT 1)
               )
        )
        SELECT p.pool_id, p.token_type, p.owner, p.associated_id,
               p.circulating_supply, p.base_price, p.quadratic_coefficient, p.created_at,
               p.time, p.transaction_id, COALESCE(lp.price, 0)::bigint as price,
               COALESCE(ph24.price, fp.price) as price_24h_ago,
               COALESCE(ph24.circulating_supply, fp.circulating_supply) as circulating_supply_24h_ago,
               v.vol as volume_24h,
               r.creator_earnings,
               r.platform_earnings,
               r.ecosystem_earnings
        FROM latest_pool p
        LEFT JOIN latest_price lp ON true
        LEFT JOIN price_24h ph24 ON true
        LEFT JOIN first_price fp ON true
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

#[derive(QueryableByName)]
struct SptTransactionSqlRow {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    pool_id: String,
    #[diesel(sql_type = Text)]
    transaction_type: String,
    #[diesel(sql_type = Text)]
    sender: String,
    #[diesel(sql_type = BigInt)]
    amount: i64,
    #[diesel(sql_type = BigInt)]
    myso_amount: i64,
    #[diesel(sql_type = BigInt)]
    fee_amount: i64,
    #[diesel(sql_type = BigInt)]
    creator_fee: i64,
    #[diesel(sql_type = BigInt)]
    platform_fee: i64,
    #[diesel(sql_type = BigInt)]
    treasury_fee: i64,
    #[diesel(sql_type = BigInt)]
    price: i64,
    #[diesel(sql_type = BigInt)]
    created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    transaction_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    counterparty_pool_id: Option<String>,
    #[diesel(sql_type = Bool)]
    is_swap_leg: bool,
}

fn spt_transaction_from_sql(r: SptTransactionSqlRow) -> SptTransaction {
    SptTransaction {
        id: r.id,
        pool_id: r.pool_id,
        transaction_type: r.transaction_type,
        sender: r.sender,
        amount: r.amount,
        myso_amount: r.myso_amount,
        fee_amount: r.fee_amount,
        creator_fee: r.creator_fee,
        platform_fee: r.platform_fee,
        treasury_fee: r.treasury_fee,
        price: r.price,
        created_at: r.created_at,
        time: r.time,
        transaction_id: r.transaction_id,
        counterparty_pool_id: r.counterparty_pool_id,
        is_swap_leg: r.is_swap_leg,
    }
}

pub(crate) async fn get_spt_transactions(
    conn: &mut Connection<'_>,
    pool_id: &str,
    limit: i64,
    offset: i64,
    viewer: Option<&str>,
    prioritize_followed: bool,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<SptTransactionsWithViewer> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let transactions: Vec<SptTransaction> = if prioritize_followed && let Some(v) = viewer {
        let (v_pid, v_owner) = resolve_profile_address(conn, v).await?;
        let refs = viewer_ref_strings(&v_owner, &v_pid);
        let q = r#"
            SELECT id, pool_id, transaction_type, sender, amount, myso_amount, fee_amount,
                   creator_fee, platform_fee, treasury_fee, price, created_at, time, transaction_id,
                   counterparty_pool_id, is_swap_leg
            FROM spt_transactions
            WHERE pool_id = $1
            ORDER BY (
                EXISTS (
                    SELECT 1 FROM social_graph_relationships sgr
                    WHERE sgr.follower_address = ANY($4::TEXT[])
                    AND sgr.following_address = spt_transactions.sender
                )
            )::int DESC, time DESC
            LIMIT $2 OFFSET $3
        "#;
        let rows: Vec<SptTransactionSqlRow> = diesel::sql_query(q)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .bind::<Array<Text>, _>(&refs)
            .load(conn)
            .await?;
        rows.into_iter().map(spt_transaction_from_sql).collect()
    } else {
        spt_transactions::table
            .filter(spt_transactions::pool_id.eq(pool_id))
            .order(spt_transactions::time.desc())
            .limit(limit)
            .offset(offset)
            .select(SptTransaction::as_select())
            .load::<SptTransaction>(conn)
            .await?
    };

    let viewer_by_sender = if let Some(v) = viewer {
        let senders: Vec<String> = transactions.iter().map(|t| t.sender.clone()).collect();
        let (v_pid, v_owner) = resolve_profile_address(conn, v).await?;
        Some(batch_viewer_social_context(conn, &senders, &v_pid, &v_owner).await?)
    } else {
        None
    };

    metrics.requests_succeeded.inc();
    Ok(SptTransactionsWithViewer {
        transactions,
        viewer_by_sender,
    })
}

/// Swaps where `pool_id` is either the source or destination pool, newest first.
pub(crate) async fn get_spt_swaps_for_pool(
    conn: &mut Connection<'_>,
    pool_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptSwap>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let results = spt_swaps::table
        .filter(
            spt_swaps::source_pool_id
                .eq(pool_id)
                .or(spt_swaps::dest_pool_id.eq(pool_id)),
        )
        .order(spt_swaps::time.desc())
        .limit(limit)
        .offset(offset)
        .select(SptSwap::as_select())
        .load::<SptSwap>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

/// Swaps executed by a given trader, newest first.
pub(crate) async fn get_spt_swaps_for_trader(
    conn: &mut Connection<'_>,
    trader: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptSwap>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let results = spt_swaps::table
        .filter(spt_swaps::trader.eq(trader))
        .order(spt_swaps::time.desc())
        .limit(limit)
        .offset(offset)
        .select(SptSwap::as_select())
        .load::<SptSwap>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

/// P2P SPT transfers for a pool, newest first.
pub(crate) async fn get_spt_transfers_for_pool(
    conn: &mut Connection<'_>,
    pool_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptTransfer>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let results = spt_transfers::table
        .filter(spt_transfers::pool_id.eq(pool_id))
        .order(spt_transfers::time.desc())
        .limit(limit)
        .offset(offset)
        .select(SptTransfer::as_select())
        .load::<SptTransfer>(conn)
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
            SELECT DISTINCT ON (pool_id) pool_id, token_type, owner, associated_id,
                   circulating_supply, base_price, quadratic_coefficient, created_at, time, transaction_id
            FROM spt_pools
            WHERE 1=1 {token_filter}
            ORDER BY pool_id, time DESC
        ),
        -- `market_cap` uses numeric so `price * circulating_supply` cannot overflow bigint
        -- when sorting (nano-SPT supplies are large).
        pool_metrics AS (
            SELECT
                p.pool_id,
                p.token_type,
                p.owner,
                p.associated_id,
                p.circulating_supply,
                p.base_price,
                p.quadratic_coefficient,
                p.created_at,
                p.time,
                p.transaction_id,
                COALESCE(ph.price, 0)::bigint as price,
                COALESCE(ph24.price, fp.price) as price_24h_ago,
                COALESCE(ph24.circulating_supply, fp.circulating_supply) as circulating_supply_24h_ago,
                COALESCE(v.vol, 0)::bigint as volume_24h,
                COALESCE(r.creator_earnings, 0)::bigint as creator_earnings,
                COALESCE(r.platform_earnings, 0)::bigint as platform_earnings,
                COALESCE(r.ecosystem_earnings, 0)::bigint as ecosystem_earnings,
                (COALESCE(ph.price, 0)::numeric * p.circulating_supply::numeric) as market_cap,
                CASE WHEN COALESCE(ph24.price, fp.price) IS NOT NULL
                          AND COALESCE(ph24.price, fp.price) > 0
                    THEN ((COALESCE(ph.price, 0) - COALESCE(ph24.price, fp.price))::float
                          / COALESCE(ph24.price, fp.price) * 100)
                    ELSE NULL
                END as price_change_24h,
                (COALESCE(r.creator_earnings, 0) + COALESCE(r.platform_earnings, 0) + COALESCE(r.ecosystem_earnings, 0))::bigint as total_earnings
            FROM latest_pools p
            LEFT JOIN LATERAL (
                SELECT price FROM spt_price_history
                WHERE pool_id = p.pool_id ORDER BY time DESC LIMIT 1
            ) ph ON true
            LEFT JOIN LATERAL (
                SELECT price, circulating_supply FROM spt_price_history
                WHERE pool_id = p.pool_id AND time <= NOW() - INTERVAL '24 hours'
                ORDER BY time DESC LIMIT 1
            ) ph24 ON true
            LEFT JOIN LATERAL (
                SELECT price, circulating_supply FROM spt_price_history
                WHERE pool_id = p.pool_id ORDER BY time ASC LIMIT 1
            ) fp ON true
            LEFT JOIN LATERAL (
                SELECT COALESCE(SUM(myso_amount), 0)::bigint as vol
                FROM spt_transactions
                WHERE pool_id = p.pool_id AND time >= NOW() - INTERVAL '24 hours'
            ) v ON true
            LEFT JOIN LATERAL (
                {earnings_sql}
            ) r ON true
        )
        SELECT pool_id, token_type, owner, associated_id, circulating_supply,
               base_price, quadratic_coefficient, created_at, time, transaction_id, price,
               price_24h_ago, circulating_supply_24h_ago, volume_24h, creator_earnings, platform_earnings, ecosystem_earnings
        FROM pool_metrics
        ORDER BY {order_clause}
        LIMIT $1 OFFSET $2
        "#,
        token_filter = token_filter,
        order_clause = order_clause,
        earnings_sql = SPT_EARNINGS_BY_ASSOCIATED_ID_LATERAL,
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

/// Latest reservation pool object id for a profile/post `associated_id` (`profile_...` / `post_...`).
pub(crate) async fn get_reservation_pool_id_for_associated_id(
    conn: &mut Connection<'_>,
    associated_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<String>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    #[derive(QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct Row {
        #[diesel(sql_type = Text)]
        pool_id: String,
    }

    let result = diesel::sql_query(
        "SELECT pool_id FROM spt_reservation_pools WHERE associated_id = $1 ORDER BY time DESC LIMIT 1",
    )
    .bind::<Text, _>(associated_id)
    .get_result::<Row>(conn)
    .await
    .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result.map(|r| r.pool_id))
}

pub(crate) async fn get_spt_reservation_holdings_for_reserver(
    conn: &mut Connection<'_>,
    reserver_address: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptReservationHoldingRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = format!(
        r#"
        SELECT urh.reserver_address, urh.pool_id, urh.associated_id, urh.token_type, urh.owner,
               urh.amount, urh.reserved_at, urh.total_reserved, urh.required_threshold,
               urh.threshold_met, urh.pool_status,
               p.username as profile_username, p.display_name as profile_display_name,
               p.profile_photo as profile_photo, p.social_proof_token_address as profile_social_proof_token_address,
               p.reservation_pool_address as profile_reservation_pool_address
               {nulls}
        FROM spt_reservation_holdings urh
        LEFT JOIN LATERAL (
            SELECT username, display_name, profile_photo, social_proof_token_address, reservation_pool_address
            FROM profiles
            WHERE LOWER(TRIM(owner_address)) = LOWER(TRIM(urh.owner))
            ORDER BY updated_at DESC
            LIMIT 1
        ) p ON true
        WHERE LOWER(TRIM(urh.reserver_address)) = LOWER(TRIM($1::text))
        ORDER BY urh.reserved_at DESC
        LIMIT $2 OFFSET $3
    "#,
        nulls = SPT_RESERVATION_VIEWER_NULLS,
    );

    let results = diesel::sql_query(query)
        .bind::<Text, _>(reserver_address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptReservationHoldingRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

/// Current reservation holders for a pool (from `spt_reservation_holdings`), ordered by amount DESC.
pub(crate) async fn get_reservation_holdings_for_pool(
    conn: &mut Connection<'_>,
    pool_id: &str,
    limit: i64,
    offset: i64,
    viewer: Option<&str>,
    prioritize_followed: bool,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptReservationHoldingRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let select_body = r#"
        SELECT urh.reserver_address, urh.pool_id, urh.associated_id, urh.token_type, urh.owner,
               urh.amount, urh.reserved_at, urh.total_reserved, urh.required_threshold,
               urh.threshold_met, urh.pool_status,
               p.username as profile_username, p.display_name as profile_display_name,
               p.profile_photo as profile_photo, p.social_proof_token_address as profile_social_proof_token_address,
               p.reservation_pool_address as profile_reservation_pool_address
    "#;

    let mut results: Vec<SptReservationHoldingRow> = if prioritize_followed && let Some(v) = viewer
    {
        let (v_pid, v_owner) = resolve_profile_address(conn, v).await?;
        let refs = viewer_ref_strings(&v_owner, &v_pid);
        let query = format!(
            r#"{} {nulls}
        FROM spt_reservation_holdings urh
        LEFT JOIN LATERAL (
            SELECT username, display_name, profile_photo, social_proof_token_address, reservation_pool_address
            FROM profiles
            WHERE LOWER(TRIM(owner_address)) = LOWER(TRIM(urh.owner))
            ORDER BY updated_at DESC
            LIMIT 1
        ) p ON true
        WHERE urh.pool_id = $1
        ORDER BY (
            EXISTS (
                SELECT 1 FROM social_graph_relationships sgr
                WHERE sgr.follower_address = ANY($4::TEXT[])
                AND sgr.following_address = urh.reserver_address
            )
        )::int DESC, urh.amount DESC, urh.reserved_at DESC
        LIMIT $2 OFFSET $3
    "#,
            select_body,
            nulls = SPT_RESERVATION_VIEWER_NULLS,
        );
        diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .bind::<Array<Text>, _>(&refs)
            .load(conn)
            .await?
    } else {
        let query = format!(
            r#"{} {nulls}
        FROM spt_reservation_holdings urh
        LEFT JOIN LATERAL (
            SELECT username, display_name, profile_photo, social_proof_token_address, reservation_pool_address
            FROM profiles
            WHERE LOWER(TRIM(owner_address)) = LOWER(TRIM(urh.owner))
            ORDER BY updated_at DESC
            LIMIT 1
        ) p ON true
        WHERE urh.pool_id = $1
        ORDER BY urh.amount DESC, urh.reserved_at DESC
        LIMIT $2 OFFSET $3
    "#,
            select_body,
            nulls = SPT_RESERVATION_VIEWER_NULLS,
        );
        diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(conn)
            .await?
    };

    if let Some(v) = viewer {
        apply_viewer_context_to_reservation_rows(conn, &mut results, v).await?;
    }

    metrics.requests_succeeded.inc();
    Ok(results)
}

/// Reservers with net zero reserved MYSO for this pool (sum of deposit/withdraw deltas is 0)
/// who have at least one indexed deposit, so they were holders and fully exited.
pub(crate) async fn get_former_reservation_holdings_for_pool(
    conn: &mut Connection<'_>,
    pool_id: &str,
    limit: i64,
    offset: i64,
    viewer: Option<&str>,
    prioritize_followed: bool,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptReservationHoldingRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let select_body = r#"
        SELECT l.reserver_address, l.pool_id, sp.associated_id, sp.token_type, sp.owner,
               l.amount, l.reserved_at, sp.total_reserved, sp.required_threshold,
               (sp.total_reserved >= sp.required_threshold) AS threshold_met,
               sp.status AS pool_status,
               po.username as profile_username, po.display_name as profile_display_name,
               po.profile_photo as profile_photo, po.social_proof_token_address as profile_social_proof_token_address,
               po.reservation_pool_address as profile_reservation_pool_address
    "#;

    let from_where = r#"
        FROM (
            SELECT
                reserver_address,
                pool_id,
                SUM(amount) AS amount,
                MAX(reserved_at) AS reserved_at
            FROM spt_reservations
            WHERE pool_id = $1
            GROUP BY reserver_address, pool_id
            HAVING SUM(amount) = 0
               AND SUM(CASE WHEN amount > 0 THEN 1 ELSE 0 END) > 0
        ) l
        INNER JOIN LATERAL (
            SELECT associated_id, token_type, owner, total_reserved, required_threshold, status
            FROM spt_reservation_pools sp2
            WHERE sp2.pool_id = l.pool_id
            ORDER BY sp2.time DESC
            LIMIT 1
        ) sp ON true
        LEFT JOIN LATERAL (
            SELECT username, display_name, profile_photo, social_proof_token_address, reservation_pool_address
            FROM profiles
            WHERE LOWER(TRIM(owner_address)) = LOWER(TRIM(sp.owner))
            ORDER BY updated_at DESC
            LIMIT 1
        ) po ON true
    "#;

    let mut results: Vec<SptReservationHoldingRow> = if prioritize_followed && let Some(v) = viewer
    {
        let (v_pid, v_owner) = resolve_profile_address(conn, v).await?;
        let refs = viewer_ref_strings(&v_owner, &v_pid);
        let query = format!(
            r#"{} {nulls}
        {}
        ORDER BY (
            EXISTS (
                SELECT 1 FROM social_graph_relationships sgr
                WHERE sgr.follower_address = ANY($4::TEXT[])
                AND sgr.following_address = l.reserver_address
            )
        )::int DESC, l.reserved_at DESC
        LIMIT $2 OFFSET $3
    "#,
            select_body,
            from_where,
            nulls = SPT_RESERVATION_VIEWER_NULLS,
        );
        diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .bind::<Array<Text>, _>(&refs)
            .load(conn)
            .await?
    } else {
        let query = format!(
            r#"{} {nulls}
        {}
        ORDER BY l.reserved_at DESC
        LIMIT $2 OFFSET $3
    "#,
            select_body,
            from_where,
            nulls = SPT_RESERVATION_VIEWER_NULLS,
        );
        diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(conn)
            .await?
    };

    if let Some(v) = viewer {
        apply_viewer_context_to_reservation_rows(conn, &mut results, v).await?;
    }

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_spt_exchange_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SptExchangeConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, post_threshold, profile_threshold, max_individual_reservation_bps,
               total_fee_bps, creator_fee_bps, platform_fee_bps, treasury_fee_bps,
               trading_creator_fee_bps, trading_platform_fee_bps, trading_treasury_fee_bps,
               reservation_creator_fee_bps, reservation_platform_fee_bps, reservation_treasury_fee_bps,
               max_reservers_per_pool, base_price, quadratic_coefficient, max_hold_percent_bps,
               trading_enabled, non_platform_platform_to_creator_bps,
               non_platform_platform_to_treasury_bps, updated_at, transaction_id, version, time
        FROM spt_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<SptExchangeConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

/// Calendar period for aggregating [`SptReservationVolumeBucket`] rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SptReservationVolumeInterval {
    Event,
    FiveMin,
    Hour,
    Day,
}

/// One UTC bucket of reservation activity for a pool (`deposit_volume` / `withdrawal_volume` in MYSO base units).
#[derive(Debug, Clone, QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SptReservationVolumeBucket {
    /// Bucket start, Unix epoch seconds (UTC). Event rows use the trade timestamp.
    #[diesel(sql_type = BigInt)]
    pub bucket_start: i64,
    /// Exclusive bucket end, Unix epoch seconds (UTC). Event rows repeat `bucket_start`.
    #[diesel(sql_type = BigInt)]
    pub bucket_end: i64,
    /// Earliest `spt_reservations.time` in this bucket, Unix epoch seconds (UTC).
    #[diesel(sql_type = BigInt)]
    pub earliest_at: i64,
    /// Latest `spt_reservations.time` in this bucket, Unix epoch seconds (UTC).
    #[diesel(sql_type = BigInt)]
    pub latest_at: i64,
    #[diesel(sql_type = BigInt)]
    pub deposit_volume: i64,
    #[diesel(sql_type = BigInt)]
    pub withdrawal_volume: i64,
    #[diesel(sql_type = BigInt)]
    pub deposit_count: i64,
    #[diesel(sql_type = BigInt)]
    pub withdrawal_count: i64,
}

const RESERVATION_VOLUME_HISTORY_MAX_LIMIT: i64 = 500;
const EVENT_DOWNSAMPLE_THRESHOLD: i64 = 288;

const RESERVATION_POOL_MATCH_SQL: &str = r#"
        (
            r.pool_id = $1
            OR r.pool_id = (
                SELECT 'reservation_pool_' || associated_id
                FROM spt_reservation_pools
                WHERE pool_id = $1 OR associated_id = $1
                ORDER BY time DESC
                LIMIT 1
            )
        )
"#;

fn reservation_volume_bucket_width(interval: SptReservationVolumeInterval) -> &'static str {
    match interval {
        SptReservationVolumeInterval::Event => "interval '0 seconds'",
        SptReservationVolumeInterval::FiveMin => "interval '5 minutes'",
        SptReservationVolumeInterval::Hour => "interval '1 hour'",
        SptReservationVolumeInterval::Day => "interval '1 day'",
    }
}

fn reservation_volume_time_bucket(interval: SptReservationVolumeInterval) -> &'static str {
    match interval {
        SptReservationVolumeInterval::Event => "r.time",
        SptReservationVolumeInterval::FiveMin => "time_bucket('5 minutes', r.time)",
        SptReservationVolumeInterval::Hour => "time_bucket('1 hour', r.time)",
        SptReservationVolumeInterval::Day => "time_bucket('1 day', r.time)",
    }
}

fn reservation_volume_event_sql() -> String {
    format!(
        r#"
        SELECT
            (EXTRACT(EPOCH FROM r.time))::bigint AS bucket_start,
            (EXTRACT(EPOCH FROM r.time))::bigint AS bucket_end,
            (EXTRACT(EPOCH FROM r.time))::bigint AS earliest_at,
            (EXTRACT(EPOCH FROM r.time))::bigint AS latest_at,
            COALESCE(CASE WHEN r.amount > 0 THEN r.amount ELSE 0 END, 0)::bigint AS deposit_volume,
            COALESCE(CASE WHEN r.amount < 0 THEN -r.amount ELSE 0 END, 0)::bigint AS withdrawal_volume,
            (CASE WHEN r.amount > 0 THEN 1 ELSE 0 END)::bigint AS deposit_count,
            (CASE WHEN r.amount < 0 THEN 1 ELSE 0 END)::bigint AS withdrawal_count
        FROM spt_reservations r
        WHERE {pool_match}
        AND ($2::timestamptz IS NULL OR r.time >= $2)
        AND ($3::timestamptz IS NULL OR r.time <= $3)
        ORDER BY r.time DESC
        LIMIT $4
        "#,
        pool_match = RESERVATION_POOL_MATCH_SQL,
    )
}

fn reservation_volume_bucket_sql(interval: SptReservationVolumeInterval) -> String {
    let bucket = reservation_volume_time_bucket(interval);
    let bucket_width = reservation_volume_bucket_width(interval);
    format!(
        r#"
        SELECT
            (EXTRACT(EPOCH FROM {bucket}))::bigint AS bucket_start,
            (EXTRACT(EPOCH FROM {bucket} + {bucket_width}))::bigint AS bucket_end,
            (EXTRACT(EPOCH FROM MIN(r.time)))::bigint AS earliest_at,
            (EXTRACT(EPOCH FROM MAX(r.time)))::bigint AS latest_at,
            COALESCE(SUM(CASE WHEN r.amount > 0 THEN r.amount ELSE 0 END), 0)::bigint AS deposit_volume,
            COALESCE(SUM(CASE WHEN r.amount < 0 THEN -r.amount ELSE 0 END), 0)::bigint AS withdrawal_volume,
            COUNT(*) FILTER (WHERE r.amount > 0)::bigint AS deposit_count,
            COUNT(*) FILTER (WHERE r.amount < 0)::bigint AS withdrawal_count
        FROM spt_reservations r
        WHERE {pool_match}
        AND ($2::timestamptz IS NULL OR r.time >= $2)
        AND ($3::timestamptz IS NULL OR r.time <= $3)
        GROUP BY {bucket}
        ORDER BY {bucket} DESC
        LIMIT $4
        "#,
        bucket = bucket,
        bucket_width = bucket_width,
        pool_match = RESERVATION_POOL_MATCH_SQL,
    )
}

fn reservation_volume_5m_cagg_sql() -> &'static str {
    r#"
        SELECT
            (EXTRACT(EPOCH FROM v.bucket))::bigint AS bucket_start,
            (EXTRACT(EPOCH FROM v.bucket + interval '5 minutes'))::bigint AS bucket_end,
            (EXTRACT(EPOCH FROM v.earliest_at))::bigint AS earliest_at,
            (EXTRACT(EPOCH FROM v.latest_at))::bigint AS latest_at,
            v.deposit_volume::bigint AS deposit_volume,
            v.withdrawal_volume::bigint AS withdrawal_volume,
            v.deposit_count::bigint AS deposit_count,
            v.withdrawal_count::bigint AS withdrawal_count
        FROM spt_reservation_volume_5m v
        WHERE (
            v.pool_id = $1
            OR v.pool_id = (
                SELECT 'reservation_pool_' || associated_id
                FROM spt_reservation_pools
                WHERE pool_id = $1 OR associated_id = $1
                ORDER BY time DESC
                LIMIT 1
            )
        )
        AND ($2::timestamptz IS NULL OR v.bucket >= $2)
        AND ($3::timestamptz IS NULL OR v.bucket <= $3)
        ORDER BY v.bucket DESC
        LIMIT $4
    "#
}

#[derive(QueryableByName)]
struct ReservationVolumeCountRow {
    #[diesel(sql_type = BigInt)]
    count: i64,
}

#[derive(QueryableByName)]
struct ReservationVolumeExistsRow {
    #[diesel(sql_type = Bool)]
    exists: bool,
}

async fn count_reservation_events(
    conn: &mut Connection<'_>,
    pool_id_param: &str,
    from: Option<chrono::DateTime<chrono::Utc>>,
    to: Option<chrono::DateTime<chrono::Utc>>,
) -> anyhow::Result<i64> {
    let query = format!(
        r#"
        SELECT COUNT(*)::bigint AS count
        FROM spt_reservations r
        WHERE {pool_match}
        AND ($2::timestamptz IS NULL OR r.time >= $2)
        AND ($3::timestamptz IS NULL OR r.time <= $3)
        "#,
        pool_match = RESERVATION_POOL_MATCH_SQL,
    );
    let row = diesel::sql_query(&query)
        .bind::<Text, _>(pool_id_param)
        .bind::<Nullable<Timestamptz>, _>(from)
        .bind::<Nullable<Timestamptz>, _>(to)
        .get_result::<ReservationVolumeCountRow>(conn)
        .await?;
    Ok(row.count)
}

async fn reservation_volume_5m_cagg_exists(conn: &mut Connection<'_>) -> bool {
    let result = diesel::sql_query(
        "SELECT EXISTS (
            SELECT 1
            FROM timescaledb_information.continuous_aggregates
            WHERE view_name = 'spt_reservation_volume_5m'
        ) AS exists",
    )
    .get_result::<ReservationVolumeExistsRow>(conn)
    .await;
    matches!(result, Ok(row) if row.exists)
}

pub(crate) async fn get_spt_reservation_volume_history(
    conn: &mut Connection<'_>,
    pool_id_param: &str,
    interval: SptReservationVolumeInterval,
    limit: i64,
    from: Option<chrono::DateTime<chrono::Utc>>,
    to: Option<chrono::DateTime<chrono::Utc>>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptReservationVolumeBucket>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let limit = limit.clamp(1, RESERVATION_VOLUME_HISTORY_MAX_LIMIT);
    let mut interval = interval;
    if interval == SptReservationVolumeInterval::Event {
        let count = count_reservation_events(conn, pool_id_param, from, to).await?;
        if count > EVENT_DOWNSAMPLE_THRESHOLD.min(limit) {
            interval = SptReservationVolumeInterval::FiveMin;
        }
    }

    let use_five_min_cagg = interval == SptReservationVolumeInterval::FiveMin
        && reservation_volume_5m_cagg_exists(conn).await;
    let query = match interval {
        SptReservationVolumeInterval::Event => reservation_volume_event_sql(),
        SptReservationVolumeInterval::FiveMin if use_five_min_cagg => {
            reservation_volume_5m_cagg_sql().to_string()
        }
        SptReservationVolumeInterval::FiveMin
        | SptReservationVolumeInterval::Hour
        | SptReservationVolumeInterval::Day => reservation_volume_bucket_sql(interval),
    };

    let results = diesel::sql_query(&query)
        .bind::<Text, _>(pool_id_param)
        .bind::<Nullable<Timestamptz>, _>(from)
        .bind::<Nullable<Timestamptz>, _>(to)
        .bind::<BigInt, _>(limit)
        .load::<SptReservationVolumeBucket>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

#[cfg(test)]
mod pct_change_tests {
    use super::{pct_change, pct_change_i128};

    const SPT_SCALE: i64 = 1_000_000_000;

    #[test]
    fn ten_to_fourteen_display_tokens_is_forty_percent() {
        let prev = 10 * SPT_SCALE;
        let current = 14 * SPT_SCALE;
        assert_eq!(pct_change(current, prev), Some(40.0));
    }

    #[test]
    fn prev_non_positive_is_none() {
        assert_eq!(pct_change(14, 0), None);
        assert_eq!(pct_change(14, -1), None);
        assert_eq!(pct_change_i128(100, 0), None);
    }

    #[test]
    fn flat_price_market_cap_follows_supply() {
        let price = 1_000_000_000_i64;
        let prev_supply = 10 * SPT_SCALE;
        let current_supply = 14 * SPT_SCALE;
        let pct = pct_change_i128(
            (price as i128) * (current_supply as i128),
            (price as i128) * (prev_supply as i128),
        );
        assert_eq!(pct, Some(40.0));
    }

    #[test]
    fn default_curve_price_10_to_14_is_near_zero() {
        // p(s) = base + coeff * (s/SCALE)^2 / BPS_DENOM with defaults.
        let p10 = 1_000_001_000;
        let p14 = 1_000_001_960;
        let pct = pct_change(p14, p10).expect("positive baseline");
        assert!(pct > 0.0 && pct < 0.001, "got {pct}");
    }
}

#[cfg(test)]
mod earnings_sql_tests {
    use super::{
        SPT_EARNINGS_BY_ASSOCIATED_ID_LATERAL, SptReservationVolumeInterval,
        reservation_volume_5m_cagg_sql, reservation_volume_bucket_sql,
        reservation_volume_event_sql,
    };

    #[test]
    fn earnings_lateral_sums_trading_and_reservation_pool_revenue() {
        assert!(SPT_EARNINGS_BY_ASSOCIATED_ID_LATERAL.contains("pool_id = p.pool_id"));
        assert!(SPT_EARNINGS_BY_ASSOCIATED_ID_LATERAL.contains("spt_reservation_pools"));
        assert!(SPT_EARNINGS_BY_ASSOCIATED_ID_LATERAL.contains("associated_id = p.associated_id"));
        assert!(SPT_EARNINGS_BY_ASSOCIATED_ID_LATERAL.contains("creator_earnings"));
        assert!(SPT_EARNINGS_BY_ASSOCIATED_ID_LATERAL.contains("platform_earnings"));
        assert!(SPT_EARNINGS_BY_ASSOCIATED_ID_LATERAL.contains("ecosystem_earnings"));
    }

    #[test]
    fn event_volume_sql_is_ungrouped() {
        let sql = reservation_volume_event_sql();
        assert!(sql.contains("ORDER BY r.time DESC"));
        assert!(!sql.contains("GROUP BY"));
        assert!(!sql.contains("date_trunc"));
    }

    #[test]
    fn bucket_volume_sql_uses_time_bucket() {
        let five = reservation_volume_bucket_sql(SptReservationVolumeInterval::FiveMin);
        assert!(five.contains("time_bucket('5 minutes', r.time)"));
        assert!(!five.contains("date_trunc"));
        let hour = reservation_volume_bucket_sql(SptReservationVolumeInterval::Hour);
        assert!(hour.contains("time_bucket('1 hour', r.time)"));
        let day = reservation_volume_bucket_sql(SptReservationVolumeInterval::Day);
        assert!(day.contains("time_bucket('1 day', r.time)"));
    }

    #[test]
    fn five_min_cagg_sql_reads_reservation_volume_view() {
        let sql = reservation_volume_5m_cagg_sql();
        assert!(sql.contains("FROM spt_reservation_volume_5m v"));
        assert!(sql.contains("interval '5 minutes'"));
    }
}
