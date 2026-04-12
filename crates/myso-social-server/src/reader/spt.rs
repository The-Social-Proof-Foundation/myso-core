// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Double, Nullable, SmallInt, Text};
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
        SELECT pool_id, token_type, owner, associated_id, symbol, name, circulating_supply,
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
        SELECT pool_id, token_type, owner, associated_id, symbol, name, circulating_supply,
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
               creator_fee, platform_fee, treasury_fee, price, created_at, time, transaction_id
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
               trading_enabled, updated_at, time, transaction_id
        FROM spt_exchange_config
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
/// use hypertable `time` on `spt_reservations`.
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
            GROUP BY pool_id
        ),
        tx_vol_previous AS (
            SELECT pool_id, COALESCE(SUM(myso_amount), 0)::bigint AS vol
            FROM spt_transactions
            WHERE time BETWEEN NOW() - INTERVAL '48 hours' AND NOW() - INTERVAL '24 hours'
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
        previous_prices AS (
            SELECT DISTINCT ON (pool_id) pool_id, price as previous_price
            FROM spt_price_history
            WHERE time < NOW() - INTERVAL '24 hours'
            ORDER BY pool_id, time DESC
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
            SELECT DISTINCT ON (pool_id) pool_id, name, symbol
            FROM spt_pools
            ORDER BY pool_id, time DESC
        )
        SELECT
            p.pool_id, p.name, p.symbol,
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
        #[diesel(sql_type = Text)]
        name: String,
        #[diesel(sql_type = Text)]
        symbol: String,
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
                "name": r.name,
                "symbol": r.symbol,
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
            SELECT DISTINCT ON (pool_id) pool_id, name, symbol
            FROM spt_pools
            ORDER BY pool_id, time DESC
        )
        SELECT 
            h.pool_id, p.name, p.symbol, h.amount,
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
        #[diesel(sql_type = Text)]
        name: String,
        #[diesel(sql_type = Text)]
        symbol: String,
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
                "name": h.name,
                "symbol": h.symbol,
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
            SELECT DISTINCT ON (pool_id) pool_id, name, symbol
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
            tp.pool_id, tp.name, tp.symbol,
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
        #[diesel(sql_type = Text)]
        name: String,
        #[diesel(sql_type = Text)]
        symbol: String,
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
                "name": r.name,
                "symbol": r.symbol,
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
/// [`get_spt_liquidity_profile`]. **Reservations** use `spt_reservations.time`.
pub(crate) async fn get_spt_market_sentiment(db: &Db) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;
    let query = r#"
        WITH latest_reservation_states AS (
            SELECT DISTINCT ON (pool_id) pool_id, associated_id
            FROM spt_reservation_pools
            ORDER BY pool_id, time DESC
        ),
        active_reservation_pools AS (
            SELECT lp.pool_id, lp.associated_id
            FROM latest_reservation_states lp
            WHERE NOT EXISTS (
                SELECT 1 FROM spt_pools tok
                WHERE (
                    tok.associated_id = lp.associated_id
                    OR (
                        LEFT(lp.associated_id, 8) = 'profile_'
                        AND LENGTH(lp.associated_id) > 8
                        AND tok.associated_id = SUBSTRING(lp.associated_id FROM 9)
                    )
                    OR (
                        LEFT(tok.associated_id, 8) = 'profile_'
                        AND LENGTH(tok.associated_id) > 8
                        AND lp.associated_id = SUBSTRING(tok.associated_id FROM 9)
                    )
                )
                LIMIT 1
            )
        ),
        trade_current AS (
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
                GROUP BY transaction_id, pool_id, transaction_type, sender, amount
            ) buy_sell_groups
        ),
        trade_previous AS (
            SELECT COALESCE(SUM(myso_abs), 0)::bigint AS total_gross
            FROM (
                SELECT MAX(ABS(myso_amount))::bigint AS myso_abs
                FROM spt_transactions
                WHERE time BETWEEN NOW() - INTERVAL '48 hours' AND NOW() - INTERVAL '24 hours'
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
            AND EXISTS (
                SELECT 1 FROM active_reservation_pools arp
                WHERE sr.pool_id = arp.pool_id
                   OR sr.pool_id = ('reservation_pool_' || arp.associated_id)
            )
        ),
        previous_reservation_volume AS (
            SELECT COALESCE(SUM(ABS(sr.amount)), 0)::bigint as total_gross
            FROM spt_reservations sr
            WHERE sr.time > NOW() - INTERVAL '48 hours'
              AND sr.time <= NOW() - INTERVAL '24 hours'
            AND EXISTS (
                SELECT 1 FROM active_reservation_pools arp
                WHERE sr.pool_id = arp.pool_id
                   OR sr.pool_id = ('reservation_pool_' || arp.associated_id)
            )
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
                AND EXISTS (
                    SELECT 1 FROM active_reservation_pools arp
                    WHERE sr.pool_id = arp.pool_id
                       OR sr.pool_id = ('reservation_pool_' || arp.associated_id)
                )
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
                AND EXISTS (
                    SELECT 1 FROM active_reservation_pools arp
                    WHERE sr.pool_id = arp.pool_id
                       OR sr.pool_id = ('reservation_pool_' || arp.associated_id)
                )
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
        SELECT name, symbol
        FROM spt_pools
        WHERE pool_id = $1
        ORDER BY time DESC
        LIMIT 1
    ";
    #[derive(QueryableByName)]
    struct PoolRow {
        #[diesel(sql_type = Text)]
        name: String,
        #[diesel(sql_type = Text)]
        symbol: String,
    }
    let pool_info: Option<PoolRow> = diesel::sql_query(pool_query)
        .bind::<Text, _>(pool_id)
        .get_result(&mut conn)
        .await
        .optional()?;

    if let Some(p) = pool_info {
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
            "name": p.name,
            "symbol": p.symbol,
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
            "name": "Reservation Pool",
            "symbol": "?",
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
        "name": "Unknown",
        "symbol": "?",
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
    let query = format!(
        r#"
        SELECT
            date_trunc('{}', r.time) AS bucket_start,
            COALESCE(SUM(CASE WHEN r.amount > 0 THEN r.amount ELSE 0 END), 0)::bigint AS deposit_volume,
            COALESCE(SUM(CASE WHEN r.amount < 0 THEN -r.amount ELSE 0 END), 0)::bigint AS withdrawal_volume,
            COUNT(*) FILTER (WHERE r.amount > 0)::bigint AS deposit_count,
            COUNT(*) FILTER (WHERE r.amount < 0)::bigint AS withdrawal_count
        FROM spt_reservations r
        WHERE (
            r.pool_id = $1
            OR r.pool_id = (
                SELECT 'reservation_pool_' || associated_id
                FROM spt_reservation_pools
                WHERE pool_id = $1 OR associated_id = $1
                ORDER BY time DESC
                LIMIT 1
            )
        )
        AND ($2::timestamptz IS NULL OR r.time >= $2)
        AND ($3::timestamptz IS NULL OR r.time <= $3)
        GROUP BY 1
        ORDER BY 1 DESC
        LIMIT $4
        "#,
        trunc
    );
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
        SELECT pool_id, token_type, owner, associated_id, symbol, name,
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
        SELECT pool_id, token_type, owner, associated_id, symbol, name,
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
}

pub(crate) async fn get_spt_user_holdings(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<(String, i64, i64)>, SocialError> {
    let mut conn = db.connect().await?;
    let query = r#"
        SELECT pool_id, SUM(amount)::bigint as amount, MAX(acquired_at)::bigint as acquired_at
        FROM spt_holdings
        WHERE holder_address = $1
        GROUP BY pool_id
        HAVING SUM(amount) != 0
        ORDER BY acquired_at DESC NULLS LAST
        LIMIT $2 OFFSET $3
    "#;
    let rows: Vec<UserHoldingRow> = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load(&mut conn)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| (r.pool_id, r.amount, r.acquired_at))
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
}

pub(crate) async fn get_spt_user_reservations(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<(String, i64, i64)>, SocialError> {
    let mut conn = db.connect().await?;
    let query = r#"
        SELECT pool_id, amount, reserved_at
        FROM spt_reservation_holdings
        WHERE LOWER(TRIM(reserver_address)) = LOWER(TRIM($1::text))
        ORDER BY reserved_at DESC NULLS LAST
        LIMIT $2 OFFSET $3
    "#;
    let rows: Vec<UserReservationRow> = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load(&mut conn)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| (r.pool_id, r.amount, r.reserved_at))
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
        .map(
            |(pool_id, amount, acquired_at)| crate::reader::SptUserHoldingItem {
                pool_id,
                amount,
                acquired_at,
                source: "holding".to_string(),
            },
        )
        .chain(
            reservations
                .into_iter()
                .map(
                    |(pool_id, amount, reserved_at)| crate::reader::SptUserHoldingItem {
                        pool_id,
                        amount,
                        acquired_at: reserved_at,
                        source: "reservation".to_string(),
                    },
                ),
        )
        .collect();

    items.sort_by(|a, b| b.acquired_at.cmp(&a.acquired_at));
    let skip = offset as usize;
    let take = limit as usize;
    let result: Vec<_> = items.into_iter().skip(skip).take(take).collect();
    Ok(result)
}
