// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Per-sub-agent P&L.
//!
//! A sub-agent signs from its `derived_address`, so every SPT trade, reservation and payout it makes is
//! already indexed under that address. This module reuses the profile P&L and SPT portfolio queries with
//! `derived_address` as the wallet, then subtracts the agent's AI credit spend
//! (`ai_credit_usage_lines.amount_mist`, keyed by `agent_object_id`).
//!
//! All amounts are MYSO base units. Revoked and deactivated agents are kept in every roll-up and flagged
//! by [`SubAgentStatus`]: revoking an agent does not erase its financial history.
//!
//! # Net P&L
//! - Windowed (`windows`): wallet cash-flow from [`crate::pnl`] minus AI spend in the same window.
//!   Swap cash-flow counts MYSO paid/received, not mark-to-market.
//! - Lifetime (`lifetime`): SPT trading P&L (realized + unrealized, weighted-average cost) plus
//!   all-time non-swap inbound payouts minus all-time AI spend. Swap cash-flow is *not* added again
//!   here, since the position P&L already covers it.

use diesel::sql_types::{BigInt, Text};
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::SubAgentRow;
use myso_pg_db::Connection;
use serde::Serialize;

use crate::memory::{get_sub_agent_by_object_id, list_sub_agents};
use crate::metrics::DbReaderMetrics;
use crate::pnl::{get_profile_pnl_for_windows, ProfilePnLWindow, ProfilePnLWindowResult};
use crate::returns::{get_user_spt_portfolio, SptPortfolioMetrics};

/// Upper bound on agents loaded for one roll-up.
const MAX_AGENTS_PER_ROLLUP: i64 = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubAgentStatus {
    Active,
    Deactivated,
    Revoked,
}

impl SubAgentStatus {
    pub fn from_row(row: &SubAgentRow) -> Self {
        Self::from_flags(row.active, row.revoked_at_ms)
    }

    /// Revocation wins over deactivation: a revoked agent is also marked inactive.
    fn from_flags(active: bool, revoked_at_ms: Option<i64>) -> Self {
        if revoked_at_ms.is_some() {
            Self::Revoked
        } else if !active {
            Self::Deactivated
        } else {
            Self::Active
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SubAgentWindowPnl {
    pub window: ProfilePnLWindow,
    pub cash_flow: ProfilePnLWindowResult,
    /// AI credit spend recorded for this agent in the window.
    pub ai_spend_myso: i64,
    /// `cash_flow.net_cash_flow_myso - ai_spend_myso`.
    pub net_cash_flow_after_ai_myso: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubAgentLifetimePnl {
    pub realized_pl_myso: i64,
    pub unrealized_pl_myso: i64,
    /// SPT trading P&L: realized + unrealized.
    pub trading_pl_myso: i64,
    /// All-time non-swap inbound payouts (creator fees, tips, subscriptions, ...).
    pub gross_inbound_myso: i64,
    pub ai_spend_myso: i64,
    /// `trading_pl_myso + gross_inbound_myso - ai_spend_myso`.
    pub net_pnl_myso: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubAgentPnl {
    pub agent: SubAgentRow,
    pub status: SubAgentStatus,
    pub lifetime: SubAgentLifetimePnl,
    pub portfolio: SptPortfolioMetrics,
    /// Empty for agents outside the requested page.
    pub windows: Vec<SubAgentWindowPnl>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SubAgentPnlTotals {
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

impl SubAgentPnlTotals {
    fn add(&mut self, status: SubAgentStatus, lifetime: &SubAgentLifetimePnl) {
        self.agent_count += 1;
        match status {
            SubAgentStatus::Active => self.active_count += 1,
            SubAgentStatus::Deactivated => self.deactivated_count += 1,
            SubAgentStatus::Revoked => self.revoked_count += 1,
        }
        self.realized_pl_myso = self
            .realized_pl_myso
            .saturating_add(lifetime.realized_pl_myso);
        self.unrealized_pl_myso = self
            .unrealized_pl_myso
            .saturating_add(lifetime.unrealized_pl_myso);
        self.trading_pl_myso = self
            .trading_pl_myso
            .saturating_add(lifetime.trading_pl_myso);
        self.gross_inbound_myso = self
            .gross_inbound_myso
            .saturating_add(lifetime.gross_inbound_myso);
        self.ai_spend_myso = self.ai_spend_myso.saturating_add(lifetime.ai_spend_myso);
        self.net_pnl_myso = self.net_pnl_myso.saturating_add(lifetime.net_pnl_myso);
    }
}

fn net_pnl(trading_pl_myso: i64, gross_inbound_myso: i64, ai_spend_myso: i64) -> i64 {
    trading_pl_myso
        .saturating_add(gross_inbound_myso)
        .saturating_sub(ai_spend_myso)
}

#[derive(Debug, Clone, Serialize)]
pub struct SubAgentPnlSummary {
    /// One page of agents (with windowed stats).
    pub agents: Vec<SubAgentPnl>,
    /// Roll-up across every agent of the principal, not just the page.
    pub totals: SubAgentPnlTotals,
    pub total_count: i64,
}

#[derive(QueryableByName)]
struct SpendRow {
    #[diesel(sql_type = BigInt)]
    total: i64,
}

async fn ai_spend_myso(
    conn: &mut Connection<'_>,
    agent_object_id: &str,
    window: ProfilePnLWindow,
) -> anyhow::Result<i64> {
    let row: SpendRow = diesel::sql_query(
        "SELECT COALESCE(SUM(amount_mist), 0)::bigint AS total FROM ai_credit_usage_lines \
         WHERE agent_object_id = $1 \
           AND ($2::bigint < 0 OR created_at >= (NOW() - ($2::bigint * INTERVAL '1 day')))",
    )
    .bind::<Text, _>(agent_object_id)
    .bind::<BigInt, _>(window.days_parameter())
    .get_result(conn)
    .await?;
    Ok(row.total)
}

async fn lifetime_for_agent(
    conn: &mut Connection<'_>,
    agent: &SubAgentRow,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<(SubAgentLifetimePnl, SptPortfolioMetrics)> {
    let portfolio = get_user_spt_portfolio(conn, &agent.derived_address, metrics).await?;
    let all_time =
        get_profile_pnl_for_windows(conn, &agent.derived_address, &[ProfilePnLWindow::All]).await?;
    let gross_inbound_myso = all_time
        .into_iter()
        .next()
        .map(|w| w.gross_inbound_myso)
        .unwrap_or_default();
    let ai_spend = ai_spend_myso(conn, &agent.agent_object_id, ProfilePnLWindow::All).await?;
    let trading_pl_myso = portfolio.lifetime_net_pl_myso;
    let lifetime = SubAgentLifetimePnl {
        realized_pl_myso: portfolio.realized_pl_myso,
        unrealized_pl_myso: portfolio.unrealized_pl_myso,
        trading_pl_myso,
        gross_inbound_myso,
        ai_spend_myso: ai_spend,
        net_pnl_myso: net_pnl(trading_pl_myso, gross_inbound_myso, ai_spend),
    };
    Ok((lifetime, portfolio))
}

async fn windows_for_agent(
    conn: &mut Connection<'_>,
    agent: &SubAgentRow,
    windows: &[ProfilePnLWindow],
) -> anyhow::Result<Vec<SubAgentWindowPnl>> {
    let cash_flows = get_profile_pnl_for_windows(conn, &agent.derived_address, windows).await?;
    let mut out = Vec::with_capacity(cash_flows.len());
    for cash_flow in cash_flows {
        let ai_spend = ai_spend_myso(conn, &agent.agent_object_id, cash_flow.window).await?;
        out.push(SubAgentWindowPnl {
            window: cash_flow.window,
            net_cash_flow_after_ai_myso: cash_flow.net_cash_flow_myso.saturating_sub(ai_spend),
            ai_spend_myso: ai_spend,
            cash_flow,
        });
    }
    Ok(out)
}

/// P&L for one sub-agent, looked up by `agent_object_id`. Revoked and deactivated agents resolve too.
pub async fn get_sub_agent_pnl(
    conn: &mut Connection<'_>,
    agent_object_id: &str,
    windows: &[ProfilePnLWindow],
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SubAgentPnl>> {
    let Some(agent) = get_sub_agent_by_object_id(conn, agent_object_id, metrics).await? else {
        return Ok(None);
    };
    let (lifetime, portfolio) = lifetime_for_agent(conn, &agent, metrics).await?;
    let windows = windows_for_agent(conn, &agent, windows).await?;
    Ok(Some(SubAgentPnl {
        status: SubAgentStatus::from_row(&agent),
        agent,
        lifetime,
        portfolio,
        windows,
    }))
}

/// P&L for every sub-agent of `principal_owner`, paged, plus a roll-up over all of them.
pub async fn list_sub_agent_pnl(
    conn: &mut Connection<'_>,
    principal_owner: &str,
    active_only: bool,
    windows: &[ProfilePnLWindow],
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<SubAgentPnlSummary> {
    let all = list_sub_agents(
        conn,
        principal_owner,
        active_only,
        MAX_AGENTS_PER_ROLLUP,
        0,
        metrics,
    )
    .await?;

    let mut totals = SubAgentPnlTotals::default();
    let mut rows = Vec::with_capacity(all.sub_agents.len());
    for agent in all.sub_agents {
        let (lifetime, portfolio) = lifetime_for_agent(conn, &agent, metrics).await?;
        let status = SubAgentStatus::from_row(&agent);
        totals.add(status, &lifetime);
        rows.push(SubAgentPnl {
            agent,
            status,
            lifetime,
            portfolio,
            windows: Vec::new(),
        });
    }

    let start = (offset.max(0) as usize).min(rows.len());
    let end = start.saturating_add(limit.max(0) as usize).min(rows.len());
    let mut agents: Vec<SubAgentPnl> = rows.drain(start..end).collect();
    for entry in &mut agents {
        entry.windows = windows_for_agent(conn, &entry.agent, windows).await?;
    }

    Ok(SubAgentPnlSummary {
        agents,
        totals,
        total_count: all.total_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lifetime(trading: i64, inbound: i64, spend: i64) -> SubAgentLifetimePnl {
        SubAgentLifetimePnl {
            realized_pl_myso: trading / 2,
            unrealized_pl_myso: trading - trading / 2,
            trading_pl_myso: trading,
            gross_inbound_myso: inbound,
            ai_spend_myso: spend,
            net_pnl_myso: net_pnl(trading, inbound, spend),
        }
    }

    #[test]
    fn revoked_wins_over_deactivated() {
        assert_eq!(
            SubAgentStatus::from_flags(false, Some(5)),
            SubAgentStatus::Revoked
        );
        assert_eq!(
            SubAgentStatus::from_flags(false, None),
            SubAgentStatus::Deactivated
        );
        assert_eq!(
            SubAgentStatus::from_flags(true, None),
            SubAgentStatus::Active
        );
    }

    #[test]
    fn net_pnl_subtracts_ai_spend() {
        assert_eq!(net_pnl(1_000, 200, 150), 1_050);
        assert_eq!(net_pnl(-500, 0, 100), -600);
    }

    #[test]
    fn totals_keep_revoked_and_deactivated_agents() {
        let mut totals = SubAgentPnlTotals::default();
        totals.add(SubAgentStatus::Active, &lifetime(1_000, 100, 50));
        totals.add(SubAgentStatus::Revoked, &lifetime(-400, 0, 25));
        totals.add(SubAgentStatus::Deactivated, &lifetime(200, 10, 5));

        assert_eq!(totals.agent_count, 3);
        assert_eq!(totals.active_count, 1);
        assert_eq!(totals.revoked_count, 1);
        assert_eq!(totals.deactivated_count, 1);
        assert_eq!(totals.trading_pl_myso, 800);
        assert_eq!(totals.gross_inbound_myso, 110);
        assert_eq!(totals.ai_spend_myso, 80);
        assert_eq!(totals.net_pnl_myso, 830);
    }

    #[test]
    fn totals_saturate_instead_of_overflowing() {
        let mut totals = SubAgentPnlTotals::default();
        totals.add(SubAgentStatus::Active, &lifetime(i64::MAX, 0, 0));
        totals.add(SubAgentStatus::Active, &lifetime(i64::MAX, 0, 0));
        assert_eq!(totals.trading_pl_myso, i64::MAX);
    }
}
