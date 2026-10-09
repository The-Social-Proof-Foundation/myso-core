// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! GraphQL types for per-sub-agent P&L ([`super::memory::SubAgent::pnl`],
//! [`super::profile::Profile::sub_agents_pnl`]).
//!
//! Amounts are MYSO base units. Revoked and deactivated agents stay in every roll-up and are
//! flagged by [`SubAgentStatus`].

use async_graphql::{Enum, Object, SimpleObject};
use myso_indexer_alt_social_reader::{
    SubAgentLifetimePnl as DbLifetime, SubAgentPnl as DbAgentPnl, SubAgentPnlSummary as DbSummary,
    SubAgentPnlTotals as DbTotals, SubAgentStatus as DbStatus, SubAgentWindowPnl as DbWindowPnl,
};

use super::memory::SubAgent;
use super::pnl::{ProfilePnLWindow, ProfilePnLWindowStats};
use super::returns::SptPortfolioMetrics;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum SubAgentStatus {
    Active,
    Deactivated,
    Revoked,
}

impl From<DbStatus> for SubAgentStatus {
    fn from(s: DbStatus) -> Self {
        match s {
            DbStatus::Active => Self::Active,
            DbStatus::Deactivated => Self::Deactivated,
            DbStatus::Revoked => Self::Revoked,
        }
    }
}

/// One window of an agent's cash-flow P&L, net of its AI credit spend.
#[derive(SimpleObject, Clone)]
pub(crate) struct SubAgentWindowPnl {
    pub window: ProfilePnLWindow,
    pub cash_flow: ProfilePnLWindowStats,
    /// AI credit spend recorded for the agent in the window.
    pub ai_spend_myso: i64,
    /// `cashFlow.netCashFlowMyso - aiSpendMyso`.
    pub net_cash_flow_after_ai_myso: i64,
}

impl From<DbWindowPnl> for SubAgentWindowPnl {
    fn from(w: DbWindowPnl) -> Self {
        Self {
            window: ProfilePnLWindowStats::from(w.cash_flow.clone()).window,
            cash_flow: w.cash_flow.into(),
            ai_spend_myso: w.ai_spend_myso,
            net_cash_flow_after_ai_myso: w.net_cash_flow_after_ai_myso,
        }
    }
}

/// Lifetime P&L: SPT trading P&L (realized + unrealized) + non-swap inbound payouts - AI spend.
#[derive(SimpleObject, Clone)]
pub(crate) struct SubAgentLifetimePnl {
    pub realized_pl_myso: i64,
    pub unrealized_pl_myso: i64,
    pub trading_pl_myso: i64,
    pub gross_inbound_myso: i64,
    pub ai_spend_myso: i64,
    pub net_pnl_myso: i64,
}

impl From<DbLifetime> for SubAgentLifetimePnl {
    fn from(l: DbLifetime) -> Self {
        Self {
            realized_pl_myso: l.realized_pl_myso,
            unrealized_pl_myso: l.unrealized_pl_myso,
            trading_pl_myso: l.trading_pl_myso,
            gross_inbound_myso: l.gross_inbound_myso,
            ai_spend_myso: l.ai_spend_myso,
            net_pnl_myso: l.net_pnl_myso,
        }
    }
}

#[derive(Clone)]
pub(crate) struct SubAgentPnl {
    inner: DbAgentPnl,
}

impl SubAgentPnl {
    pub(crate) fn from_row(inner: DbAgentPnl) -> Self {
        Self { inner }
    }
}

#[Object]
impl SubAgentPnl {
    async fn agent(&self) -> SubAgent {
        SubAgent::from_row(self.inner.agent.clone())
    }

    async fn status(&self) -> SubAgentStatus {
        self.inner.status.into()
    }

    async fn lifetime(&self) -> SubAgentLifetimePnl {
        self.inner.lifetime.clone().into()
    }

    /// SPT positions held at the agent's derived address (weighted-average cost).
    async fn spt_portfolio(&self) -> SptPortfolioMetrics {
        self.inner.portfolio.clone().into()
    }

    /// Per-window cash-flow P&L net of AI spend; empty when not requested.
    async fn windows(&self) -> Vec<SubAgentWindowPnl> {
        self.inner.windows.iter().cloned().map(Into::into).collect()
    }
}

/// Roll-up across every sub-agent of a principal, including revoked and deactivated ones.
#[derive(SimpleObject, Clone)]
pub(crate) struct SubAgentPnlTotals {
    pub agent_count: i64,
    pub active_count: i64,
    pub deactivated_count: i64,
    pub revoked_count: i64,
    pub realized_pl_myso: i64,
    pub unrealized_pl_myso: i64,
    pub trading_pl_myso: i64,
    pub gross_inbound_myso: i64,
    pub ai_spend_myso: i64,
    pub net_pnl_myso: i64,
}

impl From<DbTotals> for SubAgentPnlTotals {
    fn from(t: DbTotals) -> Self {
        Self {
            agent_count: t.agent_count,
            active_count: t.active_count,
            deactivated_count: t.deactivated_count,
            revoked_count: t.revoked_count,
            realized_pl_myso: t.realized_pl_myso,
            unrealized_pl_myso: t.unrealized_pl_myso,
            trading_pl_myso: t.trading_pl_myso,
            gross_inbound_myso: t.gross_inbound_myso,
            ai_spend_myso: t.ai_spend_myso,
            net_pnl_myso: t.net_pnl_myso,
        }
    }
}

#[derive(Clone)]
pub(crate) struct SubAgentPnlSummary {
    inner: DbSummary,
}

impl SubAgentPnlSummary {
    pub(crate) fn from_row(inner: DbSummary) -> Self {
        Self { inner }
    }
}

#[Object]
impl SubAgentPnlSummary {
    /// One page of agents with windowed stats.
    async fn agents(&self) -> Vec<SubAgentPnl> {
        self.inner
            .agents
            .iter()
            .cloned()
            .map(SubAgentPnl::from_row)
            .collect()
    }

    /// Roll-up over all agents, not just this page.
    async fn totals(&self) -> SubAgentPnlTotals {
        self.inner.totals.clone().into()
    }

    async fn total_count(&self) -> i64 {
        self.inner.total_count
    }
}

/// Shared default for the `windows` argument: 7d, 30d and all-time.
pub(crate) fn resolve_windows(
    windows: Option<Vec<ProfilePnLWindow>>,
) -> Vec<myso_indexer_alt_social_reader::ProfilePnLWindow> {
    windows
        .unwrap_or_else(|| {
            vec![
                ProfilePnLWindow::Days7,
                ProfilePnLWindow::Days30,
                ProfilePnLWindow::All,
            ]
        })
        .into_iter()
        .map(Into::into)
        .collect()
}
