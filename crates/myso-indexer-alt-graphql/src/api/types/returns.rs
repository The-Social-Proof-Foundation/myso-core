// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Personal SPT investment metrics (weighted-average cost).
//! These are **not** token price performance. Window return is
//! `windowPl / capitalAtRisk`, never a delta of two lifetime ROI percentages.

use async_graphql::{Enum, SimpleObject};
use myso_indexer_alt_social_reader::{
    SptPortfolioMetrics as DbPortfolio, SptPositionMetrics as DbPosition,
    SptPositionTimeSeriesPoint as DbPoint, SptReturnWindow as DbWindow,
    TraderReturnLeaderboardEntry as DbLeader, TraderReturnSort as DbSort,
};

pub fn spt_return_metrics_enabled() -> bool {
    matches!(
        std::env::var("SPT_RETURN_METRICS_ENABLED").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum SptReturnWindow {
    Hours24,
    Days7,
    Days30,
    Days180,
    Days365,
    All,
}

impl From<SptReturnWindow> for DbWindow {
    fn from(w: SptReturnWindow) -> Self {
        match w {
            SptReturnWindow::Hours24 => Self::Hours24,
            SptReturnWindow::Days7 => Self::Days7,
            SptReturnWindow::Days30 => Self::Days30,
            SptReturnWindow::Days180 => Self::Days180,
            SptReturnWindow::Days365 => Self::Days365,
            SptReturnWindow::All => Self::All,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum TraderReturnSort {
    HighestLifetimeRoi,
    HighestLifetimeNetPl,
    HighestWindowNetPl,
    /// Window P/L divided by capital-at-risk. Not delta lifetime ROI.
    HighestWindowReturnPct,
    HighestRealizedPlInWindow,
    LargestPortfolioValue,
}

impl From<TraderReturnSort> for DbSort {
    fn from(s: TraderReturnSort) -> Self {
        match s {
            TraderReturnSort::HighestLifetimeRoi => Self::HighestLifetimeRoi,
            TraderReturnSort::HighestLifetimeNetPl => Self::HighestLifetimeNetPl,
            TraderReturnSort::HighestWindowNetPl => Self::HighestWindowNetPl,
            TraderReturnSort::HighestWindowReturnPct => Self::HighestWindowReturnPct,
            TraderReturnSort::HighestRealizedPlInWindow => Self::HighestRealizedPlInWindow,
            TraderReturnSort::LargestPortfolioValue => Self::LargestPortfolioValue,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub(crate) struct SptPositionMetrics {
    pub pool_id: String,
    pub token_balance: i64,
    /// Full-precision share of circulating supply (not basis points).
    pub supply_pct: Option<f64>,
    pub current_value_myso: i64,
    pub avg_entry_price: Option<i64>,
    pub avg_exit_price: Option<i64>,
    pub unrealized_pl_myso: i64,
    pub unrealized_roi_pct: Option<f64>,
    pub realized_pl_myso: i64,
    /// Realized ROI uses disposed cost basis, not total invested.
    pub realized_roi_pct: Option<f64>,
    pub disposed_cost_basis_myso: i64,
    pub total_invested_myso: i64,
    pub total_returned_myso: i64,
    pub lifetime_net_pl_myso: i64,
    pub lifetime_roi_pct: Option<f64>,
    pub cost_basis_unknown: bool,
    /// Window P/L (realized + mark-to-market). Default window is 24h.
    pub window_pl_myso: i64,
    /// Cost basis at window start plus net buys in window.
    pub window_capital_at_risk_myso: i64,
    /// `windowPl / capitalAtRisk` for 24h. Not token price % and not lifetime ROI.
    pub window_return_pct: Option<f64>,
}

impl From<DbPosition> for SptPositionMetrics {
    fn from(p: DbPosition) -> Self {
        Self {
            pool_id: p.pool_id,
            token_balance: p.token_balance,
            supply_pct: p.supply_pct,
            current_value_myso: p.current_value_myso,
            avg_entry_price: p.avg_entry_price,
            avg_exit_price: p.avg_exit_price,
            unrealized_pl_myso: p.unrealized_pl_myso,
            unrealized_roi_pct: p.unrealized_roi_pct,
            realized_pl_myso: p.realized_pl_myso,
            realized_roi_pct: p.realized_roi_pct,
            disposed_cost_basis_myso: p.disposed_cost_basis_myso,
            total_invested_myso: p.total_invested_myso,
            total_returned_myso: p.total_returned_myso,
            lifetime_net_pl_myso: p.lifetime_net_pl_myso,
            lifetime_roi_pct: p.lifetime_roi_pct,
            cost_basis_unknown: p.cost_basis_unknown,
            window_pl_myso: p.window_pl_myso,
            window_capital_at_risk_myso: p.window_capital_at_risk_myso,
            window_return_pct: p.window_return_pct,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub(crate) struct SptPortfolioMetrics {
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

impl From<DbPortfolio> for SptPortfolioMetrics {
    fn from(p: DbPortfolio) -> Self {
        Self {
            holder_address: p.holder_address,
            current_value_myso: p.current_value_myso,
            total_invested_myso: p.total_invested_myso,
            total_returned_myso: p.total_returned_myso,
            realized_pl_myso: p.realized_pl_myso,
            disposed_cost_basis_myso: p.disposed_cost_basis_myso,
            unrealized_pl_myso: p.unrealized_pl_myso,
            lifetime_net_pl_myso: p.lifetime_net_pl_myso,
            lifetime_roi_pct: p.lifetime_roi_pct,
            realized_roi_pct: p.realized_roi_pct,
            positions: p.positions.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub(crate) struct SptPositionTimeSeriesPoint {
    pub time: String,
    pub token_balance: i64,
    pub supply_pct: Option<f64>,
    pub mark_value_myso: i64,
    pub unrealized_roi_pct: Option<f64>,
}

impl From<DbPoint> for SptPositionTimeSeriesPoint {
    fn from(p: DbPoint) -> Self {
        Self {
            time: p.time.to_rfc3339(),
            token_balance: p.token_balance,
            supply_pct: p.supply_pct,
            mark_value_myso: p.mark_value_myso,
            unrealized_roi_pct: p.unrealized_roi_pct,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub(crate) struct TraderReturnLeaderboardEntry {
    pub rank: i64,
    pub wallet: String,
    pub sort_value: f64,
    /// Explicit metric name (e.g. windowReturnPct vs lifetimeRoiPct).
    pub sort_metric: String,
    pub metrics: SptPortfolioMetrics,
}

impl From<DbLeader> for TraderReturnLeaderboardEntry {
    fn from(e: DbLeader) -> Self {
        Self {
            rank: e.rank,
            wallet: e.wallet,
            sort_value: e.sort_value,
            sort_metric: e.sort_metric,
            metrics: e.metrics.into(),
        }
    }
}
