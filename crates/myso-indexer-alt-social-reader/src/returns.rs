// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Personal SPT investment performance (WAC). Not token price % change.
//! Window rankings use window P/L / capital-at-risk, never delta lifetime ROI.

use diesel::sql_types::{BigInt, Bool, Double, Text, Timestamptz};
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

/// Default min capital (MYSO nano) to appear on leaderboards (~100 MYSO).
pub const MIN_LEADERBOARD_CAPITAL_MYSO: i64 = 100_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SptReturnWindow {
    Hours24,
    Days7,
    Days30,
    Days180,
    Days365,
    All,
}

impl SptReturnWindow {
    pub fn days_parameter(self) -> i64 {
        match self {
            Self::Hours24 => 0,
            Self::Days7 => 7,
            Self::Days30 => 30,
            Self::Days180 => 180,
            Self::Days365 => 365,
            Self::All => -1,
        }
    }

    pub fn hours_parameter(self) -> i64 {
        match self {
            Self::Hours24 => 24,
            Self::Days7 => 7 * 24,
            Self::Days30 => 30 * 24,
            Self::Days180 => 180 * 24,
            Self::Days365 => 365 * 24,
            Self::All => -1,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "24h" | "hours_24" | "hours24" => Some(Self::Hours24),
            "7d" | "days_7" | "days7" => Some(Self::Days7),
            "30d" | "days_30" | "days30" => Some(Self::Days30),
            "180d" | "days_180" | "days180" => Some(Self::Days180),
            "365d" | "days_365" | "1y" => Some(Self::Days365),
            "all" | "all_time" => Some(Self::All),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraderReturnSort {
    HighestLifetimeRoi,
    HighestLifetimeNetPl,
    HighestWindowNetPl,
    HighestWindowReturnPct,
    HighestRealizedPlInWindow,
    LargestPortfolioValue,
}

impl TraderReturnSort {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "highest_lifetime_roi" | "lifetime_roi" => Some(Self::HighestLifetimeRoi),
            "highest_lifetime_net_pl" | "lifetime_net_pl" => Some(Self::HighestLifetimeNetPl),
            "highest_window_net_pl" | "window_net_pl" => Some(Self::HighestWindowNetPl),
            "highest_window_return_pct" | "window_return_pct" => Some(Self::HighestWindowReturnPct),
            "highest_realized_pl_in_window" | "realized_pl_window" => {
                Some(Self::HighestRealizedPlInWindow)
            }
            "largest_portfolio_value" | "portfolio_value" => Some(Self::LargestPortfolioValue),
            _ => None,
        }
    }

    pub fn metric_label(self) -> &'static str {
        match self {
            Self::HighestLifetimeRoi => "lifetimeRoiPct",
            Self::HighestLifetimeNetPl => "lifetimeNetPlMyso",
            Self::HighestWindowNetPl => "windowPlMyso",
            Self::HighestWindowReturnPct => "windowReturnPct",
            Self::HighestRealizedPlInWindow => "windowRealizedPlMyso",
            Self::LargestPortfolioValue => "currentValueMyso",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SptPositionMetrics {
    pub pool_id: String,
    pub token_balance: i64,
    pub supply_pct: Option<f64>,
    pub current_value_myso: i64,
    pub avg_entry_price: Option<i64>,
    pub avg_exit_price: Option<i64>,
    pub unrealized_pl_myso: i64,
    pub unrealized_roi_pct: Option<f64>,
    pub realized_pl_myso: i64,
    pub realized_roi_pct: Option<f64>,
    pub disposed_cost_basis_myso: i64,
    pub total_invested_myso: i64,
    pub total_returned_myso: i64,
    pub lifetime_net_pl_myso: i64,
    pub lifetime_roi_pct: Option<f64>,
    pub cost_basis_unknown: bool,
    /// Window P/L (realized + mark-to-market). Default window is 24h.
    pub window_pl_myso: i64,
    /// Cost basis at window start plus net buys in window. Not lifetime invested.
    pub window_capital_at_risk_myso: i64,
    /// `window_pl_myso / window_capital_at_risk_myso`. Not delta lifetime ROI.
    pub window_return_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SptPortfolioMetrics {
    pub holder_address: String,
    pub current_value_myso: i64,
    pub total_invested_myso: i64,
    pub total_returned_myso: i64,
    pub realized_pl_myso: i64,
    pub disposed_cost_basis_myso: i64,
    pub unrealized_pl_myso: i64,
    pub lifetime_net_pl_myso: i64,
    pub lifetime_roi_pct: Option<f64>,
    pub realized_roi_pct: Option<f64>,
    pub positions: Vec<SptPositionMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SptPositionTimeSeriesPoint {
    pub time: chrono::DateTime<chrono::Utc>,
    pub token_balance: i64,
    pub supply_pct: Option<f64>,
    pub mark_value_myso: i64,
    pub unrealized_roi_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraderReturnLeaderboardEntry {
    pub rank: i64,
    pub wallet: String,
    pub sort_value: f64,
    pub sort_metric: String,
    pub metrics: SptPortfolioMetrics,
}

#[derive(QueryableByName)]
struct PositionRow {
    #[diesel(sql_type = Text)]
    pool_id: String,
    #[diesel(sql_type = BigInt)]
    token_balance: i64,
    #[diesel(sql_type = BigInt)]
    cost_basis_myso: i64,
    #[diesel(sql_type = BigInt)]
    total_invested_myso: i64,
    #[diesel(sql_type = BigInt)]
    total_returned_myso: i64,
    #[diesel(sql_type = BigInt)]
    realized_myso: i64,
    #[diesel(sql_type = BigInt)]
    disposed_cost_basis_myso: i64,
    #[diesel(sql_type = BigInt)]
    sold_token_qty: i64,
    #[diesel(sql_type = BigInt)]
    exit_proceeds_myso: i64,
    #[diesel(sql_type = Bool)]
    cost_basis_unknown: bool,
    #[diesel(sql_type = BigInt)]
    latest_price: i64,
    #[diesel(sql_type = BigInt)]
    circulating_supply: i64,
}

fn metrics_from_row(r: &PositionRow) -> SptPositionMetrics {
    let mark = if r.latest_price > 0 {
        (r.token_balance as i128 * r.latest_price as i128) as i64
    } else {
        0
    };
    let unrealized = mark.saturating_sub(r.cost_basis_myso);
    let supply_pct = if r.circulating_supply > 0 {
        Some(r.token_balance as f64 / r.circulating_supply as f64)
    } else {
        None
    };
    let avg_entry = if r.token_balance > 0 {
        Some(r.cost_basis_myso / r.token_balance)
    } else {
        None
    };
    let avg_exit = if r.sold_token_qty > 0 {
        Some(r.exit_proceeds_myso / r.sold_token_qty)
    } else {
        None
    };
    let realized_roi = if r.disposed_cost_basis_myso > 0 {
        Some((r.realized_myso as f64 / r.disposed_cost_basis_myso as f64) * 100.0)
    } else {
        None
    };
    let unrealized_roi = if r.cost_basis_myso > 0 {
        Some((unrealized as f64 / r.cost_basis_myso as f64) * 100.0)
    } else {
        None
    };
    let lifetime_net = r.realized_myso.saturating_add(unrealized);
    let lifetime_roi = if r.total_invested_myso > 0 {
        Some((lifetime_net as f64 / r.total_invested_myso as f64) * 100.0)
    } else {
        None
    };
    SptPositionMetrics {
        pool_id: r.pool_id.clone(),
        token_balance: r.token_balance,
        supply_pct,
        current_value_myso: mark,
        avg_entry_price: avg_entry,
        avg_exit_price: avg_exit,
        unrealized_pl_myso: unrealized,
        unrealized_roi_pct: unrealized_roi,
        realized_pl_myso: r.realized_myso,
        realized_roi_pct: realized_roi,
        disposed_cost_basis_myso: r.disposed_cost_basis_myso,
        total_invested_myso: r.total_invested_myso,
        total_returned_myso: r.total_returned_myso,
        lifetime_net_pl_myso: lifetime_net,
        lifetime_roi_pct: lifetime_roi,
        cost_basis_unknown: r.cost_basis_unknown,
        window_pl_myso: 0,
        window_capital_at_risk_myso: 0,
        window_return_pct: None,
    }
}

/// Snapshot-diff window math used by position reads and trader boards.
/// Capital-at-risk = start cost + net buys in window (not lifetime invested).
pub fn position_window_from_snaps(
    now_balance: i64,
    now_cost: i64,
    now_realized: i64,
    now_price: i64,
    start_balance: i64,
    start_cost: i64,
    start_realized: i64,
    start_price: i64,
) -> (i64, i64, Option<f64>) {
    let window_realized = now_realized.saturating_sub(start_realized);
    let now_mark = (now_balance as i128).saturating_mul(now_price as i128);
    let start_mark = (start_balance as i128).saturating_mul(start_price as i128);
    let cost_delta = now_cost as i128 - start_cost as i128;
    let window_mtm = (now_mark - start_mark - cost_delta) as i64;
    let window_pl = window_realized.saturating_add(window_mtm);
    let added = now_cost.saturating_sub(start_cost).max(0);
    let capital = start_cost.saturating_add(added);
    let pct = if capital > 0 {
        Some((window_pl as f64 / capital as f64) * 100.0)
    } else {
        None
    };
    (window_pl, capital, pct)
}

fn portfolio_from_positions(
    holder: &str,
    positions: Vec<SptPositionMetrics>,
) -> SptPortfolioMetrics {
    let current_value_myso: i64 = positions.iter().map(|p| p.current_value_myso).sum();
    let total_invested_myso: i64 = positions.iter().map(|p| p.total_invested_myso).sum();
    let total_returned_myso: i64 = positions.iter().map(|p| p.total_returned_myso).sum();
    let realized_pl_myso: i64 = positions.iter().map(|p| p.realized_pl_myso).sum();
    let disposed_cost_basis_myso: i64 = positions.iter().map(|p| p.disposed_cost_basis_myso).sum();
    let unrealized_pl_myso: i64 = positions.iter().map(|p| p.unrealized_pl_myso).sum();
    let lifetime_net_pl_myso = realized_pl_myso.saturating_add(unrealized_pl_myso);
    let lifetime_roi_pct = if total_invested_myso > 0 {
        Some((lifetime_net_pl_myso as f64 / total_invested_myso as f64) * 100.0)
    } else {
        None
    };
    let realized_roi_pct = if disposed_cost_basis_myso > 0 {
        Some((realized_pl_myso as f64 / disposed_cost_basis_myso as f64) * 100.0)
    } else {
        None
    };
    SptPortfolioMetrics {
        holder_address: holder.to_string(),
        current_value_myso,
        total_invested_myso,
        total_returned_myso,
        realized_pl_myso,
        disposed_cost_basis_myso,
        unrealized_pl_myso,
        lifetime_net_pl_myso,
        lifetime_roi_pct,
        realized_roi_pct,
        positions,
    }
}

const POSITION_SQL: &str = r#"
    SELECT
        s.pool_id, s.token_balance, s.cost_basis_myso,
        s.total_invested_myso, s.total_returned_myso, s.realized_myso,
        s.disposed_cost_basis_myso, s.sold_token_qty, s.exit_proceeds_myso,
        s.cost_basis_unknown,
        COALESCE(m.price, 0)::bigint AS latest_price,
        COALESCE(m.circulating_supply, 0)::bigint AS circulating_supply
    FROM user_spt_position_state s
    LEFT JOIN spt_pool_mark m ON m.pool_id = s.pool_id
"#;

pub async fn get_user_spt_positions(
    conn: &mut Connection<'_>,
    holder: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptPositionMetrics>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let sql = format!(
        "{POSITION_SQL} WHERE s.holder_address = $1 ORDER BY s.updated_at DESC LIMIT $2 OFFSET $3"
    );
    let rows: Vec<PositionRow> = diesel::sql_query(sql)
        .bind::<Text, _>(holder)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load(conn)
        .await?;
    let mut positions: Vec<SptPositionMetrics> = rows.iter().map(metrics_from_row).collect();
    attach_window_metrics(conn, holder, &mut positions, SptReturnWindow::Hours24).await?;
    Ok(positions)
}

#[derive(QueryableByName)]
struct WindowSnapRow {
    #[diesel(sql_type = Text)]
    pool_id: String,
    #[diesel(sql_type = BigInt)]
    now_balance: i64,
    #[diesel(sql_type = BigInt)]
    now_cost: i64,
    #[diesel(sql_type = BigInt)]
    now_realized: i64,
    #[diesel(sql_type = BigInt)]
    now_price: i64,
    #[diesel(sql_type = BigInt)]
    start_balance: i64,
    #[diesel(sql_type = BigInt)]
    start_cost: i64,
    #[diesel(sql_type = BigInt)]
    start_realized: i64,
    #[diesel(sql_type = BigInt)]
    start_price: i64,
}

async fn attach_window_metrics(
    conn: &mut Connection<'_>,
    holder: &str,
    positions: &mut [SptPositionMetrics],
    window: SptReturnWindow,
) -> anyhow::Result<()> {
    if positions.is_empty() {
        return Ok(());
    }
    let hours = window.hours_parameter();
    let rows: Vec<WindowSnapRow> = diesel::sql_query(
        r#"
        WITH snap_now AS (
            SELECT DISTINCT ON (pool_id)
                pool_id, token_balance, cost_basis_myso, realized_myso
            FROM user_spt_position_snapshots
            WHERE holder_address = $1
            ORDER BY pool_id, time DESC
        ),
        snap_start AS (
            SELECT DISTINCT ON (pool_id)
                pool_id, token_balance, cost_basis_myso, realized_myso
            FROM user_spt_position_snapshots
            WHERE holder_address = $1
              AND ($2::bigint < 0 OR time <= NOW() - ($2::bigint * INTERVAL '1 hour'))
            ORDER BY pool_id, time DESC
        ),
        prices AS (
            SELECT pool_id, price FROM spt_pool_mark
        ),
        start_prices AS (
            SELECT DISTINCT ON (pool_id) pool_id, price
            FROM spt_price_history
            WHERE $2::bigint < 0 OR time <= NOW() - ($2::bigint * INTERVAL '1 hour')
            ORDER BY pool_id, time DESC
        )
        SELECT
            n.pool_id,
            n.token_balance AS now_balance,
            n.cost_basis_myso AS now_cost,
            n.realized_myso AS now_realized,
            COALESCE(pn.price, 0)::bigint AS now_price,
            COALESCE(st.token_balance, 0)::bigint AS start_balance,
            COALESCE(st.cost_basis_myso, 0)::bigint AS start_cost,
            COALESCE(st.realized_myso, 0)::bigint AS start_realized,
            COALESCE(ps.price, 0)::bigint AS start_price
        FROM snap_now n
        LEFT JOIN snap_start st ON st.pool_id = n.pool_id
        LEFT JOIN prices pn ON pn.pool_id = n.pool_id
        LEFT JOIN start_prices ps ON ps.pool_id = n.pool_id
        "#,
    )
    .bind::<Text, _>(holder)
    .bind::<BigInt, _>(hours)
    .load(conn)
    .await?;

    let mut by_pool = std::collections::HashMap::with_capacity(rows.len());
    for row in rows {
        let (pl, capital, pct) = position_window_from_snaps(
            row.now_balance,
            row.now_cost,
            row.now_realized,
            row.now_price,
            row.start_balance,
            row.start_cost,
            row.start_realized,
            row.start_price,
        );
        by_pool.insert(row.pool_id, (pl, capital, pct));
    }
    for position in positions.iter_mut() {
        if let Some((pl, capital, pct)) = by_pool.get(&position.pool_id) {
            position.window_pl_myso = *pl;
            position.window_capital_at_risk_myso = *capital;
            position.window_return_pct = *pct;
        }
    }
    Ok(())
}

pub async fn get_user_spt_portfolio(
    conn: &mut Connection<'_>,
    holder: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<SptPortfolioMetrics> {
    let positions = get_user_spt_positions(conn, holder, 500, 0, metrics).await?;
    Ok(portfolio_from_positions(holder, positions))
}

pub async fn get_user_spt_position_timeseries(
    conn: &mut Connection<'_>,
    holder: &str,
    pool_id: &str,
    window: SptReturnWindow,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SptPositionTimeSeriesPoint>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let hours = window.hours_parameter();
    #[derive(QueryableByName)]
    struct SnapRow {
        #[diesel(sql_type = Timestamptz)]
        time: chrono::DateTime<chrono::Utc>,
        #[diesel(sql_type = BigInt)]
        token_balance: i64,
        #[diesel(sql_type = BigInt)]
        circulating_supply: i64,
        #[diesel(sql_type = BigInt)]
        cost_basis_myso: i64,
        #[diesel(sql_type = BigInt)]
        price: i64,
    }
    let rows: Vec<SnapRow> = diesel::sql_query(
        r#"
        SELECT
            s.time, s.token_balance, s.circulating_supply, s.cost_basis_myso,
            COALESCE((
                SELECT ph.price FROM spt_price_history ph
                WHERE ph.pool_id = s.pool_id AND ph.time <= s.time
                ORDER BY ph.time DESC LIMIT 1
            ), 0)::bigint AS price
        FROM user_spt_position_snapshots s
        WHERE s.holder_address = $1 AND s.pool_id = $2
          AND ($3::bigint < 0 OR s.time >= NOW() - ($3::bigint * INTERVAL '1 hour'))
        ORDER BY s.time ASC
        "#,
    )
    .bind::<Text, _>(holder)
    .bind::<Text, _>(pool_id)
    .bind::<BigInt, _>(hours)
    .load(conn)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| {
            let mark = if r.price > 0 {
                (r.token_balance as i128 * r.price as i128) as i64
            } else {
                0
            };
            let supply_pct = if r.circulating_supply > 0 {
                Some(r.token_balance as f64 / r.circulating_supply as f64)
            } else {
                None
            };
            let unrealized_roi_pct = if r.cost_basis_myso > 0 {
                Some(((mark - r.cost_basis_myso) as f64 / r.cost_basis_myso as f64) * 100.0)
            } else {
                None
            };
            SptPositionTimeSeriesPoint {
                time: r.time,
                token_balance: r.token_balance,
                supply_pct,
                mark_value_myso: mark,
                unrealized_roi_pct,
            }
        })
        .collect())
}

#[derive(QueryableByName)]
struct LeaderRow {
    #[diesel(sql_type = Text)]
    holder_address: String,
    #[diesel(sql_type = Double)]
    sort_value: f64,
}

pub async fn get_trader_return_leaderboard(
    conn: &mut Connection<'_>,
    window: SptReturnWindow,
    sort: TraderReturnSort,
    limit: i64,
    offset: i64,
    min_capital: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<TraderReturnLeaderboardEntry>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let hours = window.hours_parameter();
    let sql = match sort {
        TraderReturnSort::HighestLifetimeRoi => {
            r#"
            WITH priced AS (
                SELECT s.holder_address,
                       SUM(s.total_invested_myso)::bigint AS invested,
                       SUM(s.realized_myso + (s.token_balance * COALESCE(m.price, 0)
                           - s.cost_basis_myso))::bigint AS net
                FROM user_spt_position_state s
                LEFT JOIN spt_pool_mark m ON m.pool_id = s.pool_id
                GROUP BY s.holder_address
                HAVING SUM(s.total_invested_myso) >= $3
            )
            SELECT holder_address, CASE WHEN invested > 0 THEN (net::float8 / invested::float8) * 100.0 ELSE 0 END AS sort_value
            FROM priced
            ORDER BY sort_value DESC NULLS LAST
            LIMIT $1 OFFSET $2
            "#
        }
        TraderReturnSort::HighestLifetimeNetPl | TraderReturnSort::LargestPortfolioValue => {
            r#"
            SELECT s.holder_address,
                   SUM(CASE WHEN $4 = 1
                       THEN s.token_balance * COALESCE(m.price, 0)
                       ELSE s.realized_myso + (s.token_balance * COALESCE(m.price, 0)
                           - s.cost_basis_myso)
                   END)::float8 AS sort_value
            FROM user_spt_position_state s
            LEFT JOIN spt_pool_mark m ON m.pool_id = s.pool_id
            GROUP BY s.holder_address
            HAVING SUM(s.total_invested_myso) >= $3
            ORDER BY sort_value DESC NULLS LAST
            LIMIT $1 OFFSET $2
            "#
        }
        TraderReturnSort::HighestWindowNetPl
        | TraderReturnSort::HighestWindowReturnPct
        | TraderReturnSort::HighestRealizedPlInWindow => {
            r#"
            WITH snap_now AS (
                SELECT DISTINCT ON (holder_address, pool_id)
                    holder_address, pool_id, token_balance, cost_basis_myso, realized_myso
                FROM user_spt_position_snapshots
                ORDER BY holder_address, pool_id, time DESC
            ),
            snap_start AS (
                SELECT DISTINCT ON (holder_address, pool_id)
                    holder_address, pool_id, token_balance, cost_basis_myso, realized_myso
                FROM user_spt_position_snapshots
                WHERE $5::bigint < 0 OR time <= NOW() - ($5::bigint * INTERVAL '1 hour')
                ORDER BY holder_address, pool_id, time DESC
            ),
            prices AS (
                SELECT pool_id, price FROM spt_pool_mark
            ),
            start_prices AS (
                SELECT DISTINCT ON (pool_id) pool_id, price
                FROM spt_price_history
                WHERE $5::bigint < 0 OR time <= NOW() - ($5::bigint * INTERVAL '1 hour')
                ORDER BY pool_id, time DESC
            ),
            agg AS (
                SELECT
                    n.holder_address,
                    SUM(n.realized_myso - COALESCE(st.realized_myso, 0))::bigint AS window_realized,
                    SUM(
                        (n.token_balance * COALESCE(pn.price, 0))
                        - (COALESCE(st.token_balance, 0) * COALESCE(ps.price, 0))
                        - (n.cost_basis_myso - COALESCE(st.cost_basis_myso, 0))
                    )::bigint AS window_mtm,
                    SUM(COALESCE(st.cost_basis_myso, 0) + GREATEST(n.cost_basis_myso - COALESCE(st.cost_basis_myso, 0), 0))::bigint AS capital_at_risk
                FROM snap_now n
                LEFT JOIN snap_start st ON st.holder_address = n.holder_address AND st.pool_id = n.pool_id
                LEFT JOIN prices pn ON pn.pool_id = n.pool_id
                LEFT JOIN start_prices ps ON ps.pool_id = n.pool_id
                GROUP BY n.holder_address
            )
            SELECT holder_address,
                   CASE
                     WHEN $4 = 2 THEN window_realized::float8
                     WHEN $4 = 3 THEN (window_realized + window_mtm)::float8
                     ELSE CASE WHEN capital_at_risk > 0
                          THEN ((window_realized + window_mtm)::float8 / capital_at_risk::float8) * 100.0
                          ELSE 0 END
                   END AS sort_value
            FROM agg
            WHERE capital_at_risk >= $3
            ORDER BY sort_value DESC NULLS LAST
            LIMIT $1 OFFSET $2
            "#
        }
    };

    let mode: i64 = match sort {
        TraderReturnSort::LargestPortfolioValue => 1,
        TraderReturnSort::HighestRealizedPlInWindow => 2,
        TraderReturnSort::HighestWindowNetPl => 3,
        _ => 0,
    };

    let rows: Vec<LeaderRow> = diesel::sql_query(sql)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .bind::<BigInt, _>(min_capital)
        .bind::<BigInt, _>(mode)
        .bind::<BigInt, _>(hours)
        .load(conn)
        .await?;

    let mut out = Vec::with_capacity(rows.len());
    for (i, row) in rows.into_iter().enumerate() {
        let metrics_row = get_user_spt_portfolio(conn, &row.holder_address, metrics).await?;
        out.push(TraderReturnLeaderboardEntry {
            rank: offset + i as i64 + 1,
            wallet: row.holder_address,
            sort_value: row.sort_value,
            sort_metric: sort.metric_label().to_string(),
            metrics: metrics_row,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod window_math_tests {
    use super::TraderReturnSort;

    #[test]
    fn window_sort_is_not_delta_lifetime_roi() {
        assert_eq!(
            TraderReturnSort::HighestWindowReturnPct.metric_label(),
            "windowReturnPct"
        );
        assert_ne!(
            TraderReturnSort::HighestWindowReturnPct.metric_label(),
            "lifetimeRoiPct"
        );
    }

    #[test]
    fn position_window_is_pl_over_capital_not_delta_roi() {
        // Open 100 cost, mark 150 (lifetime 50%). Then add 900, mark +10 on the new capital.
        // Window: start mark 150 / cost 100; now mark 1060 / cost 1000; realized unchanged.
        // window_mtm = 1060 - 150 - 900 = 10; capital = 100 + 900 = 1000; pct = 1%.
        let (pl, capital, pct) =
            super::position_window_from_snaps(1060, 1000, 0, 1, 150, 100, 0, 1);
        assert_eq!(pl, 10);
        assert_eq!(capital, 1000);
        let pct = pct.expect("capital");
        assert!((pct - 1.0).abs() < 0.01);
        let delta_lifetime = 6.0 - 50.0;
        assert!((pct - delta_lifetime).abs() > 40.0);
    }
}
