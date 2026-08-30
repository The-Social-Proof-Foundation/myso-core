// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Bool, Double, Nullable, SmallInt, Text};
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::*;

pub(crate) async fn get_spt_pool(
    db: &Db,
    pool_id: &str,
) -> Result<Option<SptPoolRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT pool_id, token_type, owner, associated_id, circulating_supply,
               base_price, quadratic_coefficient, created_at, time, transaction_id
        FROM spt_pools
        WHERE pool_id = $1
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .get_result::<SptPoolRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn list_spt_pools(
    db: &Db,
    limit: i64,
    offset: i64,
    owner: Option<&str>,
    token_type: Option<i16>,
) -> Result<Vec<SptPoolRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT pool_id, token_type, owner, associated_id, circulating_supply,
               base_price, quadratic_coefficient, created_at, time, transaction_id
        FROM (SELECT DISTINCT ON (pool_id) * FROM spt_pools ORDER BY pool_id, time DESC) p
        WHERE ($1::text IS NULL OR owner = $1)
          AND ($2::smallint IS NULL OR token_type = $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
    ";
    let results = diesel::sql_query(query)
        .bind::<Nullable<Text>, _>(owner)
        .bind::<Nullable<SmallInt>, _>(token_type)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptPoolRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spt_transactions(
    db: &Db,
    pool_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SptTransactionRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT pool_id, transaction_type, sender, amount, myso_amount, fee_amount,
               creator_fee, platform_fee, treasury_fee, price, created_at, time, transaction_id,
               counterparty_pool_id, is_swap_leg
        FROM spt_transactions
        WHERE pool_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptTransactionRow>(&mut conn)
        .await?;
    Ok(results)
}

/// Swaps where `pool_id` is either the source or destination pool, newest first.
pub(crate) async fn get_spt_swaps(
    db: &Db,
    pool_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SptSwapRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT transaction_id, trader, source_pool_id, dest_pool_id, sell_amount, dest_amount,
               sell_myso_gross, buy_myso_gross, sell_fee_amount, buy_fee_amount,
               sell_creator_fee, sell_platform_fee, sell_treasury_fee,
               buy_creator_fee, buy_platform_fee, buy_treasury_fee, leftover_myso,
               source_new_price, dest_new_price, created_at, time
        FROM spt_swaps
        WHERE source_pool_id = $1 OR dest_pool_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptSwapRow>(&mut conn)
        .await?;
    Ok(results)
}

/// P2P SPT transfers for a pool, newest first.
pub(crate) async fn get_spt_transfers(
    db: &Db,
    pool_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SptTransferRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT transaction_id, pool_id, from_address, to_address, amount, created_at, time
        FROM spt_transfers
        WHERE pool_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptTransferRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spt_holdings(
    db: &Db,
    pool_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SptHoldingRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT holder_address, SUM(amount)::bigint as balance
        FROM spt_holdings
        WHERE pool_id = $1
        GROUP BY holder_address
        HAVING SUM(amount) != 0
        ORDER BY balance DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptHoldingRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spt_price_history(
    db: &Db,
    pool_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SptPriceHistoryRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT pool_id, price, circulating_supply, time, transaction_id
        FROM spt_price_history
        WHERE pool_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptPriceHistoryRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spt_exchange_config(
    db: &Db,
) -> Result<Option<SptExchangeConfigRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT updated_by, post_threshold, profile_threshold, max_individual_reservation_bps,
               total_fee_bps, creator_fee_bps, platform_fee_bps, treasury_fee_bps,
               trading_creator_fee_bps, trading_platform_fee_bps, trading_treasury_fee_bps,
               reservation_creator_fee_bps, reservation_platform_fee_bps, reservation_treasury_fee_bps,
               max_reservers_per_pool, base_price, quadratic_coefficient, max_hold_percent_bps,
               trading_enabled, non_platform_platform_to_creator_bps,
               non_platform_platform_to_treasury_bps, updated_at, time, transaction_id
        FROM spt_config
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<SptExchangeConfigRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_spt_reservation_pool(
    db: &Db,
    pool_id: &str,
) -> Result<Option<SptReservationPoolRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT pool_id, associated_id, token_type, owner, total_reserved, required_threshold,
               status, created_at, time, transaction_id
        FROM spt_reservation_pools
        WHERE pool_id = $1 OR associated_id = $1
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .get_result::<SptReservationPoolRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

/// Lists reservation pools with display labels and per-pool **total** rolling 24h volume:
/// secondary-market trades (`spt_transactions`) plus reservation ledger activity (`spt_reservations`).
///
/// Windows use the database clock (`NOW()`), i.e. **UTC** in PostgreSQL. **Transactions:** rows whose
/// hypertable `time` (indexer ingestion instant) falls in the window, matching [`get_spt_market_sentiment`]. **Reservations:**
/// use hypertable `time` on `spt_reservations` for pools in this listing (pre-mint catalog only); [`get_spt_market_sentiment`]
/// aggregates **all** reservation rows in the window, including after a token has minted.
/// **Current** window: last 24h. **Prior window:** strictly after `NOW() - 48h` through `NOW() - 24h` (contiguous).
///
/// `volume_24h` and deltas are **MYSO base units**: sum of `myso_amount` on `spt_transactions` for
/// matching `pool_id`, plus sum of `ABS(amount)` on `spt_reservations` matched to the pool via
/// `pool_id` or `reservation_pool_{associated_id}` (same resolution as volume-history).
///
/// `volume_change_percent_24h` is **100.0** when the prior window had no volume but the current window
/// does (division-by-zero avoided); **null** only when both windows have zero combined volume.
///
pub(crate) async fn list_spt_reservation_pools(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<(Vec<SptReservationPoolWithDisplayRow>, i64), SocialError> {
    let mut conn = db.connect().await?;
    let query = r#"
        WITH latest_reservation_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_reservation_pools
            ORDER BY pool_id, time DESC
        ),
        latest_profiles AS (
            SELECT DISTINCT ON (profile_id) *
            FROM profiles
            WHERE profile_id IS NOT NULL
            ORDER BY profile_id, updated_at DESC
        ),
        latest_posts AS (
            SELECT DISTINCT ON (post_id) *
            FROM posts
            ORDER BY post_id, time DESC
        ),
        tx_vol_current AS (
            SELECT pool_id, COALESCE(SUM(myso_amount), 0)::bigint AS vol
            FROM spt_transactions
            WHERE time > NOW() - INTERVAL '24 hours'
              AND transaction_type IN ('BUY', 'SELL')
            GROUP BY pool_id
        ),
        tx_vol_previous AS (
            SELECT pool_id, COALESCE(SUM(myso_amount), 0)::bigint AS vol
            FROM spt_transactions
            WHERE time BETWEEN NOW() - INTERVAL '48 hours' AND NOW() - INTERVAL '24 hours'
              AND transaction_type IN ('BUY', 'SELL')
            GROUP BY pool_id
        ),
        res_vol_current AS (
            SELECT lp.pool_id, COALESCE(SUM(ABS(sr.amount)), 0)::bigint AS vol
            FROM spt_reservations sr
            INNER JOIN latest_reservation_pools lp ON (
                sr.pool_id = lp.pool_id
                OR sr.pool_id = ('reservation_pool_' || lp.associated_id)
            )
            WHERE sr.time > NOW() - INTERVAL '24 hours'
            GROUP BY lp.pool_id
        ),
        res_vol_previous AS (
            SELECT lp.pool_id, COALESCE(SUM(ABS(sr.amount)), 0)::bigint AS vol
            FROM spt_reservations sr
            INNER JOIN latest_reservation_pools lp ON (
                sr.pool_id = lp.pool_id
                OR sr.pool_id = ('reservation_pool_' || lp.associated_id)
            )
            WHERE sr.time > NOW() - INTERVAL '48 hours'
              AND sr.time <= NOW() - INTERVAL '24 hours'
            GROUP BY lp.pool_id
        )
        SELECT
            rp.id, rp.pool_id, rp.associated_id, rp.token_type, rp.owner,
            rp.total_reserved, rp.required_threshold, rp.status,
            rp.created_at as created_at_epoch, rp.time as created_at, rp.transaction_id,
            CASE
                WHEN rp.token_type = 1 THEN prof.profile_photo
                WHEN rp.token_type = 2 THEN
                    CASE
                        WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                            CASE
                                WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                ELSE NULL
                            END
                        ELSE NULL
                    END
                ELSE NULL
            END as icon,
            CASE
                WHEN rp.token_type = 1 THEN
                    CASE
                        WHEN prof.profile_id IS NOT NULL THEN COALESCE(prof.display_name, prof.username)
                        ELSE 'Anonymous wallet'
                    END
                WHEN rp.token_type = 2 THEN post.content
                ELSE NULL
            END as primary_label,
            CASE
                WHEN rp.token_type = 1 THEN prof.username
                ELSE NULL
            END as secondary_label,
            (COALESCE(cur.vol, 0) + COALESCE(res_cur.vol, 0)) as volume_24h,
            (COALESCE(cur.vol, 0) + COALESCE(res_cur.vol, 0))
                - (COALESCE(prev.vol, 0) + COALESCE(res_prev.vol, 0)) as volume_change_24h,
            (CASE
                  WHEN (COALESCE(prev.vol, 0) + COALESCE(res_prev.vol, 0)) = 0
                   AND (COALESCE(cur.vol, 0) + COALESCE(res_cur.vol, 0)) = 0 THEN NULL
                  WHEN (COALESCE(prev.vol, 0) + COALESCE(res_prev.vol, 0)) = 0
                   AND (COALESCE(cur.vol, 0) + COALESCE(res_cur.vol, 0)) > 0 THEN 100.0::double precision
                  ELSE (
                    (COALESCE(cur.vol, 0) + COALESCE(res_cur.vol, 0)
                     - COALESCE(prev.vol, 0) - COALESCE(res_prev.vol, 0))::double precision
                    * 100.0
                    / (COALESCE(prev.vol, 0) + COALESCE(res_prev.vol, 0))::double precision
                  )
             END) as volume_change_percent_24h
        FROM latest_reservation_pools rp
        LEFT JOIN tx_vol_current cur ON cur.pool_id = rp.pool_id
        LEFT JOIN tx_vol_previous prev ON prev.pool_id = rp.pool_id
        LEFT JOIN res_vol_current res_cur ON res_cur.pool_id = rp.pool_id
        LEFT JOIN res_vol_previous res_prev ON res_prev.pool_id = rp.pool_id
        LEFT JOIN latest_profiles prof ON
            rp.token_type = 1 AND
            (CASE
                WHEN rp.associated_id LIKE 'profile_%' THEN SUBSTRING(rp.associated_id FROM 9)
                ELSE rp.associated_id
            END) = prof.profile_id
        LEFT JOIN latest_posts post ON
            rp.token_type = 2 AND
            (CASE
                WHEN rp.associated_id LIKE 'post_%' THEN SUBSTRING(rp.associated_id FROM 6)
                ELSE rp.associated_id
            END) = post.post_id
        WHERE (rp.status = 'active' OR rp.status = 'threshold_met')
        AND NOT EXISTS (
            SELECT 1 FROM spt_pools tok
            WHERE (
                tok.associated_id = rp.associated_id
                OR (
                    LEFT(rp.associated_id, 8) = 'profile_'
                    AND LENGTH(rp.associated_id) > 8
                    AND tok.associated_id = SUBSTRING(rp.associated_id FROM 9)
                )
                OR (
                    LEFT(tok.associated_id, 8) = 'profile_'
                    AND LENGTH(tok.associated_id) > 8
                    AND rp.associated_id = SUBSTRING(tok.associated_id FROM 9)
                )
            )
            LIMIT 1
        )
        ORDER BY rp.total_reserved DESC
        LIMIT $1 OFFSET $2
        "#;
    let mut pools: Vec<SptReservationPoolWithDisplayRow> = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load(&mut conn)
        .await?;
    for pool in &mut pools {
        if let Some(pct) = pool.volume_change_percent_24h {
            if !pct.is_finite() {
                pool.volume_change_percent_24h = None;
            }
        }
    }
    let count_query = r#"
        WITH latest_reservation_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_reservation_pools
            ORDER BY pool_id, time DESC
        )
        SELECT COUNT(*) as count
        FROM latest_reservation_pools rp
        WHERE (rp.status = 'active' OR rp.status = 'threshold_met')
        AND NOT EXISTS (
            SELECT 1 FROM spt_pools tok
            WHERE (
                tok.associated_id = rp.associated_id
                OR (
                    LEFT(rp.associated_id, 8) = 'profile_'
                    AND LENGTH(rp.associated_id) > 8
                    AND tok.associated_id = SUBSTRING(rp.associated_id FROM 9)
                )
                OR (
                    LEFT(tok.associated_id, 8) = 'profile_'
                    AND LENGTH(tok.associated_id) > 8
                    AND rp.associated_id = SUBSTRING(tok.associated_id FROM 9)
                )
            )
            LIMIT 1
        )
        "#;
    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }
    let total: CountRow = diesel::sql_query(count_query).get_result(&mut conn).await?;
    Ok((pools, total.count))
}

pub(crate) async fn get_spt_analytics_top_performers(
    db: &Db,
) -> Result<Vec<serde_json::Value>, SocialError> {
    let mut conn = db.connect().await?;
    let query = r#"
        WITH current_prices AS (
            SELECT DISTINCT ON (pool_id) pool_id, price as current_price
            FROM spt_price_history
            ORDER BY pool_id, time DESC
        ),
        -- Baseline: true 24h-ago price, else first indexed price (since-open for young pools).
        previous_prices AS (
            SELECT p.pool_id,
                COALESCE(
                    (SELECT price FROM spt_price_history h
                     WHERE h.pool_id = p.pool_id AND h.time < NOW() - INTERVAL '24 hours'
                     ORDER BY h.time DESC LIMIT 1),
                    (SELECT price FROM spt_price_history h
                     WHERE h.pool_id = p.pool_id
                     ORDER BY h.time ASC LIMIT 1)
                ) AS previous_price
            FROM (SELECT DISTINCT pool_id FROM spt_pools) p
        ),
        current_volume AS (
            SELECT pool_id, COALESCE(SUM(myso_amount), 0) as vol
            FROM spt_transactions
            WHERE time > NOW() - INTERVAL '24 hours'
            GROUP BY pool_id
        ),
        previous_volume AS (
            SELECT pool_id, COALESCE(SUM(myso_amount), 0) as vol
            FROM spt_transactions
            WHERE time BETWEEN NOW() - INTERVAL '48 hours' AND NOW() - INTERVAL '24 hours'
            GROUP BY pool_id
        ),
        pool_info AS (
            SELECT DISTINCT ON (pool_id) pool_id
            FROM spt_pools
            ORDER BY pool_id, time DESC
        )
        SELECT
            p.pool_id,
            COALESCE(cp.current_price, 0) as current_price,
            COALESCE(pp.previous_price, 0) as previous_price,
            COALESCE(cv.vol, 0) as current_volume,
            COALESCE(pv.vol, 0) as previous_volume,
            CASE WHEN COALESCE(pp.previous_price, 0) = 0 THEN 0.0
                 ELSE (COALESCE(cp.current_price, 0) - COALESCE(pp.previous_price, 0)) * 100.0 / pp.previous_price
            END as price_change_percentage,
            CASE WHEN COALESCE(pv.vol, 0) = 0 THEN 0.0
                 ELSE (COALESCE(cv.vol, 0) - COALESCE(pv.vol, 0)) * 100.0 / pv.vol
            END as volume_change_percentage
        FROM pool_info p
        LEFT JOIN current_prices cp ON p.pool_id = cp.pool_id
        LEFT JOIN previous_prices pp ON p.pool_id = pp.pool_id
        LEFT JOIN current_volume cv ON p.pool_id = cv.pool_id
        LEFT JOIN previous_volume pv ON p.pool_id = pv.pool_id
        ORDER BY price_change_percentage DESC NULLS LAST
        LIMIT 50
        "#;
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        pool_id: String,
        #[diesel(sql_type = BigInt)]
        current_price: i64,
        #[diesel(sql_type = BigInt)]
        previous_price: i64,
        #[diesel(sql_type = BigInt)]
        current_volume: i64,
        #[diesel(sql_type = BigInt)]
        previous_volume: i64,
        #[diesel(sql_type = Double)]
        price_change_percentage: f64,
        #[diesel(sql_type = Double)]
        volume_change_percentage: f64,
    }
    let rows: Vec<Row> = diesel::sql_query(query).load(&mut conn).await?;
    Ok(rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "pool_id": r.pool_id,
                "current_price": r.current_price,
                "previous_price": r.previous_price,
                "current_volume": r.current_volume,
                "previous_volume": r.previous_volume,
                "price_change_percentage": r.price_change_percentage,
                "volume_change_percentage": r.volume_change_percentage
            })
        })
        .collect())
}

pub(crate) async fn get_spt_portfolio_performance(
    db: &Db,
    address: &str,
) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;
    let query = r#"
        WITH latest_holdings AS (
            SELECT DISTINCT ON (pool_id) pool_id, amount
            FROM spt_holdings
            WHERE holder_address = $1
            ORDER BY pool_id, time DESC
        ),
        initial_tx AS (
            SELECT DISTINCT ON (pool_id) pool_id, price
            FROM spt_transactions
            WHERE sender = $1 AND transaction_type = 'BUY'
            ORDER BY pool_id, time ASC
        ),
        current_prices AS (
            SELECT DISTINCT ON (pool_id) pool_id, price
            FROM spt_price_history
            ORDER BY pool_id, time DESC
        ),
        pool_info AS (
            SELECT DISTINCT ON (pool_id) pool_id
            FROM spt_pools
            ORDER BY pool_id, time DESC
        )
        SELECT 
            h.pool_id, h.amount,
            h.amount * cp.price as current_value,
            COALESCE(it.price * h.amount, 0) as initial_value,
            CASE WHEN COALESCE(it.price * h.amount, 0) = 0 THEN 0.0
                 ELSE ((h.amount * cp.price) - (it.price * h.amount)) * 100.0 / (it.price * h.amount)
            END as roi_percentage
        FROM latest_holdings h
        JOIN pool_info p ON h.pool_id = p.pool_id
        JOIN current_prices cp ON h.pool_id = cp.pool_id
        LEFT JOIN initial_tx it ON h.pool_id = it.pool_id
        WHERE h.amount > 0
        "#;
    #[derive(QueryableByName)]
    struct HoldingRow {
        #[diesel(sql_type = Text)]
        pool_id: String,
        #[diesel(sql_type = BigInt)]
        amount: i64,
        #[diesel(sql_type = BigInt)]
        current_value: i64,
        #[diesel(sql_type = BigInt)]
        initial_value: i64,
        #[diesel(sql_type = Double)]
        roi_percentage: f64,
    }
    let holdings: Vec<HoldingRow> = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .load(&mut conn)
        .await?;
    let current_value: i64 = holdings.iter().map(|h| h.current_value).sum();
    let initial_value: i64 = holdings.iter().map(|h| h.initial_value).sum();
    let roi = if initial_value > 0 {
        (current_value - initial_value) as f64 * 100.0 / initial_value as f64
    } else {
        0.0
    };
    let holdings_json: Vec<serde_json::Value> = holdings
        .into_iter()
        .map(|h| {
            serde_json::json!({
                "pool_id": h.pool_id,
                "amount": h.amount,
                "current_value": h.current_value,
                "initial_value": h.initial_value,
                "roi_percentage": h.roi_percentage
            })
        })
        .collect();
    Ok(serde_json::json!({
        "address": address,
        "current_value": current_value,
        "initial_investment": initial_value,
        "roi_percentage": roi,
        "holdings": holdings_json,
        "value_history": []
    }))
}

pub(crate) async fn get_spt_creator_revenue_streams(
    db: &Db,
    address: &str,
    from_ts: chrono::DateTime<chrono::Utc>,
    to_ts: chrono::DateTime<chrono::Utc>,
) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;
    let query = r#"
        WITH token_pools AS (
            SELECT DISTINCT ON (pool_id) pool_id
            FROM spt_pools
            WHERE owner = $1
            ORDER BY pool_id, time DESC
        ),
        buy_rev AS (
            SELECT pool_id, SUM(creator_fee) as buy_revenue, COUNT(*) as buy_count
            FROM spt_transactions
            WHERE transaction_type = 'BUY' AND pool_id IN (SELECT pool_id FROM token_pools)
              AND time >= $2 AND time <= $3
            GROUP BY pool_id
        ),
        sell_rev AS (
            SELECT pool_id, SUM(creator_fee) as sell_revenue, COUNT(*) as sell_count
            FROM spt_transactions
            WHERE transaction_type = 'SELL' AND pool_id IN (SELECT pool_id FROM token_pools)
              AND time >= $2 AND time <= $3
            GROUP BY pool_id
        )
        SELECT 
            tp.pool_id,
            COALESCE(bt.buy_revenue, 0) as buy_revenue,
            COALESCE(st.sell_revenue, 0) as sell_revenue,
            COALESCE(bt.buy_revenue, 0) + COALESCE(st.sell_revenue, 0) as total_revenue,
            COALESCE(bt.buy_count, 0) + COALESCE(st.sell_count, 0) as transactions_count
        FROM token_pools tp
        LEFT JOIN buy_rev bt ON tp.pool_id = bt.pool_id
        LEFT JOIN sell_rev st ON tp.pool_id = st.pool_id
        ORDER BY total_revenue DESC
        "#;
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        pool_id: String,
        #[diesel(sql_type = BigInt)]
        buy_revenue: i64,
        #[diesel(sql_type = BigInt)]
        sell_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = BigInt)]
        transactions_count: i64,
    }
    let rows: Vec<Row> = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<diesel::sql_types::Timestamptz, _>(from_ts)
        .bind::<diesel::sql_types::Timestamptz, _>(to_ts)
        .load(&mut conn)
        .await?;
    let total_revenue: i64 = rows.iter().map(|r| r.total_revenue).sum();
    let token_pools: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "pool_id": r.pool_id,
                "total_revenue": r.total_revenue,
                "buy_revenue": r.buy_revenue,
                "sell_revenue": r.sell_revenue,
                "transactions_count": r.transactions_count
            })
        })
        .collect();
    Ok(serde_json::json!({
        "address": address,
        "total_revenue": total_revenue,
        "token_pools": token_pools,
        "revenue_by_period": []
    }))
}

/// Global SPT sentiment and rolling volumes. **Trades** use `spt_transactions.time` (ingestion
/// instant) for 24h / 48h windows, matching [`get_spt_analytics_top_performers`] and
/// [`get_spt_liquidity_profile`]. **Reservations** use `spt_reservations.time` for **all** ledger
/// rows in each window (deposits/withdrawals still count after the token has minted and left the
/// pre-mint pool catalog).
pub(crate) async fn get_spt_market_sentiment(db: &Db) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;
    let query = r#"
        WITH trade_current AS (
            SELECT
                COALESCE(SUM(CASE WHEN transaction_type = 'BUY' THEN myso_abs ELSE 0 END), 0)::bigint AS trade_buy,
                COALESCE(SUM(CASE WHEN transaction_type = 'SELL' THEN myso_abs ELSE 0 END), 0)::bigint AS trade_sell,
                COALESCE(COUNT(*), 0)::bigint AS transaction_count
            FROM (
                SELECT
                    transaction_type,
                    MAX(ABS(myso_amount))::bigint AS myso_abs
                FROM spt_transactions
                WHERE time > NOW() - INTERVAL '24 hours'
                  AND transaction_type IN ('BUY', 'SELL')
                GROUP BY transaction_id, pool_id, transaction_type, sender, amount
            ) buy_sell_groups
        ),
        trade_previous AS (
            SELECT COALESCE(SUM(myso_abs), 0)::bigint AS total_gross
            FROM (
                SELECT MAX(ABS(myso_amount))::bigint AS myso_abs
                FROM spt_transactions
                WHERE time BETWEEN NOW() - INTERVAL '48 hours' AND NOW() - INTERVAL '24 hours'
                  AND transaction_type IN ('BUY', 'SELL')
                GROUP BY transaction_id, pool_id, transaction_type, sender, amount
            ) prev_groups
        ),
        reservation_volume AS (
            SELECT
                COALESCE(SUM(ABS(amount)), 0)::bigint as reservation_gross,
                COALESCE(SUM(amount) FILTER (WHERE amount > 0), 0)::bigint as res_deposit,
                COALESCE(SUM(-amount) FILTER (WHERE amount < 0), 0)::bigint as res_withdraw_mag,
                COALESCE(COUNT(*), 0)::bigint as reservation_count,
                COALESCE(COUNT(DISTINCT reserver_address), 0)::bigint as unique_reservers
            FROM spt_reservations sr
            WHERE sr.time > NOW() - INTERVAL '24 hours'
        ),
        previous_reservation_volume AS (
            SELECT COALESCE(SUM(ABS(sr.amount)), 0)::bigint as total_gross
            FROM spt_reservations sr
            WHERE sr.time > NOW() - INTERVAL '48 hours'
              AND sr.time <= NOW() - INTERVAL '24 hours'
        ),
        unique_buyers AS (
            SELECT COALESCE(COUNT(*), 0)::bigint AS cnt
            FROM (
                SELECT sender AS addr FROM spt_transactions
                WHERE time > NOW() - INTERVAL '24 hours'
                  AND transaction_type = 'BUY'
                UNION
                SELECT sr.reserver_address AS addr FROM spt_reservations sr
                WHERE sr.time > NOW() - INTERVAL '24 hours'
                  AND sr.amount > 0
            ) u
        ),
        unique_sellers AS (
            SELECT COALESCE(COUNT(*), 0)::bigint AS cnt
            FROM (
                SELECT sender AS addr FROM spt_transactions
                WHERE time > NOW() - INTERVAL '24 hours'
                  AND transaction_type = 'SELL'
                UNION
                SELECT sr.reserver_address AS addr FROM spt_reservations sr
                WHERE sr.time > NOW() - INTERVAL '24 hours'
                  AND sr.amount < 0
            ) u
        )
        SELECT
            t.trade_buy::bigint AS buy_volume,
            t.trade_sell::bigint AS sell_volume,
            t.transaction_count,
            ub.cnt AS unique_buyers,
            us.cnt AS unique_sellers,
            COALESCE(r.reservation_gross, 0)::bigint as reservation_volume,
            COALESCE(r.reservation_count, 0)::bigint as reservation_count,
            COALESCE(r.unique_reservers, 0)::bigint as unique_reservers,
            (CASE
                WHEN (COALESCE(p.total_gross, 0) + COALESCE(pr.total_gross, 0)) = 0
                 AND (t.trade_buy + t.trade_sell + COALESCE(r.reservation_gross, 0)) = 0 THEN 0.0
                WHEN (COALESCE(p.total_gross, 0) + COALESCE(pr.total_gross, 0)) = 0
                 AND (t.trade_buy + t.trade_sell + COALESCE(r.reservation_gross, 0)) > 0 THEN 100.0
                ELSE (
                    (t.trade_buy + t.trade_sell + COALESCE(r.reservation_gross, 0)
                     - COALESCE(p.total_gross, 0) - COALESCE(pr.total_gross, 0))::double precision
                    * 100.0
                    / (COALESCE(p.total_gross, 0) + COALESCE(pr.total_gross, 0))::double precision
                )
            END) as volume_change_percentage,
            (CASE
                WHEN (t.trade_buy + t.trade_sell + COALESCE(r.res_deposit, 0) + COALESCE(r.res_withdraw_mag, 0)) = 0 THEN 0.0
                ELSE (
                    (t.trade_buy + COALESCE(r.res_deposit, 0) - t.trade_sell - COALESCE(r.res_withdraw_mag, 0))::double precision
                    / (t.trade_buy + t.trade_sell + COALESCE(r.res_deposit, 0) + COALESCE(r.res_withdraw_mag, 0))::double precision
                )
            END) as sentiment_score
        FROM trade_current t
        CROSS JOIN trade_previous p
        CROSS JOIN reservation_volume r
        CROSS JOIN previous_reservation_volume pr
        CROSS JOIN unique_buyers ub
        CROSS JOIN unique_sellers us
        "#;
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = BigInt)]
        buy_volume: i64,
        #[diesel(sql_type = BigInt)]
        sell_volume: i64,
        #[diesel(sql_type = BigInt)]
        transaction_count: i64,
        #[diesel(sql_type = BigInt)]
        unique_buyers: i64,
        #[diesel(sql_type = BigInt)]
        unique_sellers: i64,
        #[diesel(sql_type = BigInt)]
        reservation_volume: i64,
        #[diesel(sql_type = BigInt)]
        reservation_count: i64,
        #[diesel(sql_type = BigInt)]
        unique_reservers: i64,
        #[diesel(sql_type = Double)]
        volume_change_percentage: f64,
        #[diesel(sql_type = Double)]
        sentiment_score: f64,
    }
    let row: Row = diesel::sql_query(query).get_result(&mut conn).await?;
    let total_transaction_count = row.transaction_count + row.reservation_count;
    Ok(serde_json::json!({
        "overall_sentiment": row.sentiment_score,
        "buy_volume_24h": row.buy_volume,
        "sell_volume_24h": row.sell_volume,
        "transaction_count_24h": total_transaction_count,
        "unique_buyers_24h": row.unique_buyers,
        "unique_sellers_24h": row.unique_sellers,
        "reservation_volume_24h": row.reservation_volume,
        "reservation_count_24h": row.reservation_count,
        "unique_reservers_24h": row.unique_reservers,
        "volume_change_percentage": row.volume_change_percentage,
        "price_momentum": []
    }))
}

pub(crate) async fn get_spt_liquidity_profile(
    db: &Db,
    pool_id: &str,
) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;
    let pool_query = "
        SELECT EXISTS(
            SELECT 1 FROM spt_pools WHERE pool_id = $1
        ) AS pool_exists
    ";
    #[derive(QueryableByName)]
    struct PoolExistsRow {
        #[diesel(sql_type = Bool)]
        pool_exists: bool,
    }
    let pool_info: PoolExistsRow = diesel::sql_query(pool_query)
        .bind::<Text, _>(pool_id)
        .get_result(&mut conn)
        .await?;

    if pool_info.pool_exists {
        let metrics_query = "
            SELECT 
                COALESCE(SUM(myso_amount), 0) as total_volume,
                COALESCE(COUNT(*), 0) as transaction_count,
                COALESCE(MAX(myso_amount), 0) as largest_transaction,
                COALESCE(COUNT(DISTINCT sender), 0) as unique_traders_count,
                COALESCE(SUM(CASE WHEN transaction_type = 'BUY' THEN myso_amount ELSE 0 END), 0) as buy_volume,
                COALESCE(SUM(CASE WHEN transaction_type = 'SELL' THEN myso_amount ELSE 0 END), 0) as sell_volume
            FROM spt_transactions
            WHERE pool_id = $1 AND time > NOW() - INTERVAL '24 hours'
        ";
        #[derive(QueryableByName)]
        struct MetricsRow {
            #[diesel(sql_type = BigInt)]
            total_volume: i64,
            #[diesel(sql_type = BigInt)]
            transaction_count: i64,
            #[diesel(sql_type = BigInt)]
            largest_transaction: i64,
            #[diesel(sql_type = BigInt)]
            unique_traders_count: i64,
            #[diesel(sql_type = BigInt)]
            buy_volume: i64,
            #[diesel(sql_type = BigInt)]
            sell_volume: i64,
        }
        let metrics: MetricsRow = diesel::sql_query(metrics_query)
            .bind::<Text, _>(pool_id)
            .get_result(&mut conn)
            .await?;
        let avg_tx = if metrics.transaction_count > 0 {
            metrics.total_volume / metrics.transaction_count
        } else {
            0
        };
        let buy_sell_ratio = if metrics.sell_volume > 0 {
            metrics.buy_volume as f64 / metrics.sell_volume as f64
        } else {
            0.0
        };
        return Ok(serde_json::json!({
            "pool_id": pool_id,
            "total_volume_24h": metrics.total_volume,
            "transaction_count_24h": metrics.transaction_count,
            "average_transaction_size": avg_tx,
            "largest_transaction": metrics.largest_transaction,
            "unique_traders_count": metrics.unique_traders_count,
            "buy_volume_24h": metrics.buy_volume,
            "sell_volume_24h": metrics.sell_volume,
            "buy_sell_ratio": buy_sell_ratio,
            "reservation_metrics": {}
        }));
    }

    let reservation_pool_query = "
        SELECT total_reserved, required_threshold, status, associated_id, token_type
        FROM spt_reservation_pools
        WHERE pool_id = $1
        ORDER BY time DESC
        LIMIT 1
    ";
    #[derive(QueryableByName)]
    struct ReservationPoolRow {
        #[diesel(sql_type = BigInt)]
        total_reserved: i64,
        #[diesel(sql_type = BigInt)]
        required_threshold: i64,
        #[diesel(sql_type = Text)]
        status: String,
        #[diesel(sql_type = Text)]
        associated_id: String,
        #[diesel(sql_type = SmallInt)]
        token_type: i16,
    }
    let reservation_pool_info: Option<ReservationPoolRow> =
        diesel::sql_query(reservation_pool_query)
            .bind::<Text, _>(pool_id)
            .get_result(&mut conn)
            .await
            .optional()?;

    if let Some(rp) = reservation_pool_info {
        let reservation_metrics_query = "
            SELECT 
                COALESCE(SUM(amount), 0) as total_volume,
                COALESCE(COUNT(*), 0) as transaction_count,
                COALESCE(MAX(amount), 0) as largest_transaction,
                COALESCE(COUNT(DISTINCT reserver_address), 0) as unique_traders_count
            FROM spt_reservations
            WHERE pool_id = $1
              AND time > NOW() - INTERVAL '24 hours'
        ";
        #[derive(QueryableByName)]
        struct ReservationMetricsRow {
            #[diesel(sql_type = BigInt)]
            total_volume: i64,
            #[diesel(sql_type = BigInt)]
            transaction_count: i64,
            #[diesel(sql_type = BigInt)]
            largest_transaction: i64,
            #[diesel(sql_type = BigInt)]
            unique_traders_count: i64,
        }
        let metrics: ReservationMetricsRow = diesel::sql_query(reservation_metrics_query)
            .bind::<Text, _>(pool_id)
            .get_result(&mut conn)
            .await?;
        let avg_tx = if metrics.transaction_count > 0 {
            metrics.total_volume / metrics.transaction_count
        } else {
            0
        };
        return Ok(serde_json::json!({
            "pool_id": pool_id,
            "total_volume_24h": metrics.total_volume,
            "transaction_count_24h": metrics.transaction_count,
            "average_transaction_size": avg_tx,
            "largest_transaction": metrics.largest_transaction,
            "unique_traders_count": metrics.unique_traders_count,
            "buy_volume_24h": metrics.total_volume,
            "sell_volume_24h": 0,
            "buy_sell_ratio": 0.0,
            "reservation_metrics": {
                "total_reserved": rp.total_reserved,
                "required_threshold": rp.required_threshold,
                "status": rp.status,
                "associated_id": rp.associated_id,
                "token_type": rp.token_type
            }
        }));
    }

    Ok(serde_json::json!({
        "pool_id": pool_id,
        "total_volume_24h": 0,
        "transaction_count_24h": 0,
        "average_transaction_size": 0,
        "largest_transaction": 0,
        "unique_traders_count": 0,
        "buy_volume_24h": 0,
        "sell_volume_24h": 0,
        "buy_sell_ratio": 0.0,
        "reservation_metrics": {}
    }))
}

pub(crate) async fn list_spt_reservations(
    db: &Db,
    pool_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SptReservationRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT pool_id, reserver_address, amount, reserved_at,
               (EXTRACT(EPOCH FROM time) * 1000)::bigint AS created_at,
               fee_amount, creator_fee, platform_fee, treasury_fee, time, transaction_id
        FROM spt_reservations
        WHERE pool_id = $1
           OR pool_id = (
               SELECT 'reservation_pool_' || associated_id
               FROM spt_reservation_pools
               WHERE pool_id = $1 OR associated_id = $1
               ORDER BY time DESC
               LIMIT 1
           )
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptReservationRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spt_reservation_volume_history(
    db: &Db,
    pool_id_param: &str,
    trunc: &str,
    limit: i64,
    from: Option<chrono::DateTime<chrono::Utc>>,
    to: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Vec<crate::reader::SptReservationVolumeBucketRow>, SocialError> {
    let mut conn = db.connect().await?;
    let limit = limit.clamp(1, 500);
    let pool_match = r#"
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
    let mut trunc = trunc;
    if trunc == "event" {
        #[derive(QueryableByName)]
        struct CountRow {
            #[diesel(sql_type = BigInt)]
            count: i64,
        }
        let count_sql = format!(
            r#"
            SELECT COUNT(*)::bigint AS count
            FROM spt_reservations r
            WHERE {pool_match}
            AND ($2::timestamptz IS NULL OR r.time >= $2)
            AND ($3::timestamptz IS NULL OR r.time <= $3)
            "#,
            pool_match = pool_match,
        );
        let count = diesel::sql_query(&count_sql)
            .bind::<Text, _>(pool_id_param)
            .bind::<Nullable<Timestamptz>, _>(from)
            .bind::<Nullable<Timestamptz>, _>(to)
            .get_result::<CountRow>(&mut conn)
            .await?
            .count;
        if count > 288.min(limit) {
            trunc = "five_min";
        }
    }

    let query = if trunc == "event" {
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
            pool_match = pool_match,
        )
    } else if trunc == "five_min" {
        #[derive(QueryableByName)]
        struct ExistsRow {
            #[diesel(sql_type = Bool)]
            exists: bool,
        }
        let cagg_exists = diesel::sql_query(
            "SELECT EXISTS (
                SELECT 1
                FROM timescaledb_information.continuous_aggregates
                WHERE view_name = 'spt_reservation_volume_5m'
            ) AS exists",
        )
        .get_result::<ExistsRow>(&mut conn)
        .await
        .map(|row| row.exists)
        .unwrap_or(false);
        if cagg_exists {
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
            .to_string()
        } else {
            format!(
                r#"
                SELECT
                    (EXTRACT(EPOCH FROM time_bucket('5 minutes', r.time)))::bigint AS bucket_start,
                    (EXTRACT(EPOCH FROM time_bucket('5 minutes', r.time) + interval '5 minutes'))::bigint AS bucket_end,
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
                GROUP BY time_bucket('5 minutes', r.time)
                ORDER BY time_bucket('5 minutes', r.time) DESC
                LIMIT $4
                "#,
                pool_match = pool_match,
            )
        }
    } else {
        let bucket = match trunc {
            "day" => "time_bucket('1 day', r.time)",
            _ => "time_bucket('1 hour', r.time)",
        };
        let bucket_width = match trunc {
            "day" => "interval '1 day'",
            _ => "interval '1 hour'",
        };
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
            pool_match = pool_match,
        )
    };
    use diesel::sql_types::{Nullable, Timestamptz};
    let results = diesel::sql_query(&query)
        .bind::<diesel::sql_types::Text, _>(pool_id_param)
        .bind::<Nullable<Timestamptz>, _>(from)
        .bind::<Nullable<Timestamptz>, _>(to)
        .bind::<BigInt, _>(limit)
        .load::<crate::reader::SptReservationVolumeBucketRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spt_revenue(
    db: &Db,
    pool_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SptRevenueRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT pool_id, transaction_type, trader, creator_address, platform_address,
               treasury_address, creator_fee, platform_fee, treasury_fee, total_fee,
               token_amount, myso_amount, token_price, revenue_time, time, transaction_id
        FROM spt_revenue
        WHERE pool_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(pool_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SptRevenueRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spt_pool_by_associated_id(
    db: &Db,
    associated_id: &str,
) -> Result<Option<SptPoolRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT pool_id, token_type, owner, associated_id,
               circulating_supply, base_price, quadratic_coefficient, created_at, time, transaction_id
        FROM (SELECT DISTINCT ON (pool_id) * FROM spt_pools WHERE associated_id = $1 ORDER BY pool_id, time DESC) p
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(associated_id)
        .get_result::<SptPoolRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_spt_popular(db: &Db, limit: i64) -> Result<Vec<SptPoolRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT pool_id, token_type, owner, associated_id,
               circulating_supply, base_price, quadratic_coefficient, created_at, time, transaction_id
        FROM (SELECT DISTINCT ON (pool_id) * FROM spt_pools ORDER BY pool_id, time DESC) p
        ORDER BY circulating_supply DESC
        LIMIT $1
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .load::<SptPoolRow>(&mut conn)
        .await?;
    Ok(results)
}

#[derive(QueryableByName)]
struct UserHoldingRow {
    #[diesel(sql_type = Text)]
    pool_id: String,
    #[diesel(sql_type = BigInt)]
    amount: i64,
    #[diesel(sql_type = BigInt)]
    acquired_at: i64,
    #[diesel(sql_type = SmallInt)]
    token_type: i16,
    #[diesel(sql_type = Text)]
    associated_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    profile_address: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    username: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    display_name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    profile_photo: Option<String>,
}

/// Normalize `profile_…` / bare associated ids for profile-row joins.
fn profile_assoc_sql(associated_col: &str) -> String {
    format!(
        r#"CASE
            WHEN {associated_col} LIKE 'profile_%' THEN SUBSTRING({associated_col} FROM 9)
            ELSE {associated_col}
        END"#
    )
}

/// Filled + remaining mint progress from pool totals (same formula as social_graph).
fn reservation_progress(total_reserved: i64, required_threshold: i64) -> (f64, f64) {
    let filled = if required_threshold > 0 {
        (total_reserved as f64 / required_threshold as f64 * 100.0)
            .min(100.0)
            .max(0.0)
    } else {
        0.0
    };
    let remaining = (100.0 - filled).max(0.0);
    (filled, remaining)
}

pub(crate) async fn get_spt_user_holdings(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<crate::reader::SptUserHoldingItem>, SocialError> {
    let mut conn = db.connect().await?;
    let assoc = profile_assoc_sql("p.associated_id");
    let query = format!(
        r#"
        SELECT h.pool_id, h.amount, h.acquired_at,
               COALESCE(p.token_type, 0::smallint) AS token_type,
               COALESCE(p.associated_id, ''::text) AS associated_id,
               COALESCE(pr.owner_address, p.owner) AS profile_address,
               pr.username AS username,
               pr.display_name AS display_name,
               pr.profile_photo AS profile_photo
        FROM (
            SELECT pool_id, SUM(amount)::bigint AS amount, MAX(acquired_at)::bigint AS acquired_at
            FROM spt_holdings
            WHERE holder_address = $1
            GROUP BY pool_id
            HAVING SUM(amount) != 0
        ) h
        LEFT JOIN LATERAL (
            SELECT token_type, associated_id, owner
            FROM spt_pools
            WHERE pool_id = h.pool_id
            ORDER BY time DESC
            LIMIT 1
        ) p ON true
        LEFT JOIN LATERAL (
            SELECT owner_address, username, display_name, profile_photo
            FROM profiles
            WHERE LOWER(TRIM(owner_address)) = LOWER(TRIM(p.owner))
               OR (
                    p.token_type = 1
                    AND profile_id IS NOT NULL
                    AND profile_id = ({assoc})
               )
            ORDER BY updated_at DESC
            LIMIT 1
        ) pr ON true
        ORDER BY h.acquired_at DESC NULLS LAST
        LIMIT $2 OFFSET $3
    "#,
        assoc = assoc
    );
    let rows: Vec<UserHoldingRow> = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load(&mut conn)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| crate::reader::SptUserHoldingItem {
            pool_id: r.pool_id,
            amount: r.amount,
            acquired_at: r.acquired_at,
            source: "holding".to_string(),
            token_type: r.token_type,
            associated_id: r.associated_id,
            profile_address: r.profile_address,
            username: r.username,
            display_name: r.display_name,
            profile_photo: r.profile_photo,
            total_reserved: None,
            required_threshold: None,
            reservation_percentage: None,
            remaining_percentage: None,
            threshold_met: None,
            pool_status: None,
        })
        .collect())
}

#[derive(QueryableByName)]
struct UserReservationRow {
    #[diesel(sql_type = Text)]
    pool_id: String,
    #[diesel(sql_type = BigInt)]
    amount: i64,
    #[diesel(sql_type = BigInt)]
    reserved_at: i64,
    #[diesel(sql_type = SmallInt)]
    token_type: i16,
    #[diesel(sql_type = Text)]
    associated_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    profile_address: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    username: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    display_name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    profile_photo: Option<String>,
    #[diesel(sql_type = BigInt)]
    total_reserved: i64,
    #[diesel(sql_type = BigInt)]
    required_threshold: i64,
    #[diesel(sql_type = Bool)]
    threshold_met: bool,
    #[diesel(sql_type = Text)]
    pool_status: String,
}

pub(crate) async fn get_spt_user_reservations(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<crate::reader::SptUserHoldingItem>, SocialError> {
    let mut conn = db.connect().await?;
    let assoc = profile_assoc_sql("urh.associated_id");
    let query = format!(
        r#"
        SELECT urh.pool_id, urh.amount, urh.reserved_at, urh.token_type, urh.associated_id,
               COALESCE(pr.owner_address, urh.owner) AS profile_address,
               pr.username AS username,
               pr.display_name AS display_name,
               pr.profile_photo AS profile_photo,
               urh.total_reserved,
               urh.required_threshold,
               urh.threshold_met,
               urh.pool_status
        FROM spt_reservation_holdings urh
        LEFT JOIN LATERAL (
            SELECT owner_address, username, display_name, profile_photo
            FROM profiles
            WHERE LOWER(TRIM(owner_address)) = LOWER(TRIM(urh.owner))
               OR (
                    urh.token_type = 1
                    AND profile_id IS NOT NULL
                    AND profile_id = ({assoc})
               )
            ORDER BY updated_at DESC
            LIMIT 1
        ) pr ON true
        WHERE LOWER(TRIM(urh.reserver_address)) = LOWER(TRIM($1::text))
        ORDER BY urh.reserved_at DESC NULLS LAST
        LIMIT $2 OFFSET $3
    "#,
        assoc = assoc
    );
    let rows: Vec<UserReservationRow> = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load(&mut conn)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| {
            let (filled, remaining) =
                reservation_progress(r.total_reserved, r.required_threshold);
            crate::reader::SptUserHoldingItem {
                pool_id: r.pool_id,
                amount: r.amount,
                acquired_at: r.reserved_at,
                source: "reservation".to_string(),
                token_type: r.token_type,
                associated_id: r.associated_id,
                profile_address: r.profile_address,
                username: r.username,
                display_name: r.display_name,
                profile_photo: r.profile_photo,
                total_reserved: Some(r.total_reserved),
                required_threshold: Some(r.required_threshold),
                reservation_percentage: Some(filled),
                remaining_percentage: Some(remaining),
                threshold_met: Some(r.threshold_met),
                pool_status: Some(r.pool_status),
            }
        })
        .collect())
}

pub(crate) async fn get_spt_user_holdings_with_reservations(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<crate::reader::SptUserHoldingItem>, SocialError> {
    let holdings = get_spt_user_holdings(db, address, 500, 0).await?;
    let reservations = get_spt_user_reservations(db, address, 500, 0).await?;

    let mut items: Vec<crate::reader::SptUserHoldingItem> = holdings
        .into_iter()
        .chain(reservations.into_iter())
        .collect();

    items.sort_by(|a, b| b.acquired_at.cmp(&a.acquired_at));
    let skip = offset as usize;
    let take = limit as usize;
    let result: Vec<_> = items.into_iter().skip(skip).take(take).collect();
    Ok(result)
}
