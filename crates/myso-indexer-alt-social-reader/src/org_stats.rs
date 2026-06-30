// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel::sql_types::{BigInt, Int4, Nullable};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    AgenticOrganizationRow, OrganizationStatsRow, SPOT_ACCURACY_DISPLAY_MIN_RESOLVED,
};
use serde::{Deserialize, Serialize};

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

/// Rolling window for organization statistics. Cutoffs use database `NOW()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationStatsWindow {
    Days7,
    Days30,
    Days180,
    Days365,
    All,
}

impl OrganizationStatsWindow {
    pub fn days_parameter(self) -> i64 {
        match self {
            OrganizationStatsWindow::Days7 => 7,
            OrganizationStatsWindow::Days30 => 30,
            OrganizationStatsWindow::Days180 => 180,
            OrganizationStatsWindow::Days365 => 365,
            OrganizationStatsWindow::All => -1,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "7d" | "days7" => Some(Self::Days7),
            "30d" | "days30" => Some(Self::Days30),
            "180d" | "days180" => Some(Self::Days180),
            "365d" | "days365" | "1y" => Some(Self::Days365),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

/// V1 organization statistics catalog. Financial fields use net cash flow — not PNL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStatistics {
    pub organization_id: String,
    pub window: OrganizationStatsWindow,

    pub total_revenue_myso: i64,
    pub total_outbound_spend_myso: i64,
    pub net_cash_flow_myso: i64,

    pub total_actions_executed: i64,
    pub total_agents: i32,
    pub active_agents: i32,
    pub total_counterparties: i64,
    pub total_posts: i64,
    pub total_engagement: i64,
    pub total_spot_participation: i64,
    pub total_transactions: i64,

    /// Normalized 0.0–1.0; null in API when insufficient sample.
    pub spot_accuracy: Option<f64>,
    pub spot_bets_resolved: i64,
    /// True when `spot_bets_resolved < 5` (display threshold).
    pub insufficient_sample: bool,

    /// Normalized 0.0–1.0 originality score (inverted similarity).
    pub originality_score_average: Option<f64>,
    pub originality_posts_analyzed: i64,

    pub estimated_assets_under_management_myso: i64,
    /// Fraction of financial events carrying organization attribution (0.0–1.0).
    pub attribution_coverage: f64,

    pub organization_age_ms: i64,
    pub stats_rollup_at: Option<DateTime<Utc>>,
}

fn bps_to_ratio(bps: Option<i32>) -> Option<f64> {
    bps.map(|v| (v as f64) / 10_000.0)
}

fn build_statistics_from_rows(
    org: &AgenticOrganizationRow,
    stats: &OrganizationStatsRow,
    window: OrganizationStatsWindow,
) -> OrganizationStatistics {
    let now_ms = Utc::now().timestamp_millis();
    let organization_age_ms = now_ms.saturating_sub(org.created_at_ms);
    let insufficient_sample = stats.spot_bets_resolved < SPOT_ACCURACY_DISPLAY_MIN_RESOLVED;

    OrganizationStatistics {
        organization_id: org.organization_id.clone(),
        window,
        total_revenue_myso: stats.total_revenue_myso,
        total_outbound_spend_myso: stats.total_outbound_spend_myso,
        net_cash_flow_myso: stats.net_cash_flow_myso,
        total_actions_executed: stats.total_actions_executed,
        total_agents: stats.total_agents,
        active_agents: stats.active_agents,
        total_counterparties: stats.total_counterparties,
        total_posts: stats.total_posts,
        total_engagement: stats.total_engagement,
        total_spot_participation: stats.total_spot_participation,
        total_transactions: stats.total_transactions,
        spot_accuracy: if insufficient_sample {
            None
        } else {
            bps_to_ratio(stats.spot_accuracy_bps)
        },
        spot_bets_resolved: stats.spot_bets_resolved,
        insufficient_sample,
        originality_score_average: if stats.originality_posts_analyzed > 0 {
            bps_to_ratio(stats.originality_score_average_bps)
        } else {
            None
        },
        originality_posts_analyzed: stats.originality_posts_analyzed,
        estimated_assets_under_management_myso: stats.estimated_assets_under_management_myso,
        attribution_coverage: (stats.attribution_coverage_bps as f64) / 10_000.0,
        organization_age_ms,
        stats_rollup_at: stats.stats_rollup_at,
    }
}

#[derive(Debug, QueryableByName)]
struct WindowedStatsRow {
    #[diesel(sql_type = BigInt)]
    total_revenue_myso: i64,
    #[diesel(sql_type = BigInt)]
    total_outbound_spend_myso: i64,
    #[diesel(sql_type = BigInt)]
    net_cash_flow_myso: i64,
    #[diesel(sql_type = BigInt)]
    total_counterparties: i64,
    #[diesel(sql_type = Int4)]
    active_agents: i32,
    #[diesel(sql_type = BigInt)]
    total_engagement: i64,
    #[diesel(sql_type = BigInt)]
    estimated_aum_myso: i64,
    #[diesel(sql_type = Nullable<Int4>)]
    spot_accuracy_bps: Option<i32>,
    #[diesel(sql_type = BigInt)]
    spot_bets_resolved: i64,
    #[diesel(sql_type = Int4)]
    attribution_coverage_bps: i32,
}

async fn load_windowed_financial_overlay(
    conn: &mut Connection<'_>,
    organization_id: &str,
    days: i64,
) -> anyhow::Result<Option<WindowedStatsRow>> {
    let query = r#"
        WITH windowed AS (
            SELECT *
            FROM sub_agent_organization_stats_daily
            WHERE organization_id = $1
              AND snapshot_date >= CURRENT_DATE - ($2::bigint * INTERVAL '1 day')
        ),
        latest AS (
            SELECT * FROM windowed
            ORDER BY snapshot_date DESC
            LIMIT 1
        ),
        earliest AS (
            SELECT * FROM windowed
            ORDER BY snapshot_date ASC
            LIMIT 1
        )
        SELECT
            GREATEST(l.total_revenue_myso - COALESCE(e.total_revenue_myso, 0), 0)::bigint AS total_revenue_myso,
            GREATEST(l.total_outbound_spend_myso - COALESCE(e.total_outbound_spend_myso, 0), 0)::bigint AS total_outbound_spend_myso,
            GREATEST(l.net_cash_flow_myso - COALESCE(e.net_cash_flow_myso, 0), 0)::bigint AS net_cash_flow_myso,
            GREATEST(l.total_counterparties - COALESCE(e.total_counterparties, 0), 0)::bigint AS total_counterparties,
            l.active_agents::int AS active_agents,
            GREATEST(l.total_engagement - COALESCE(e.total_engagement, 0), 0)::bigint AS total_engagement,
            l.estimated_aum_myso::bigint AS estimated_aum_myso,
            l.spot_accuracy_bps,
            COALESCE(s.spot_bets_resolved, 0)::bigint AS spot_bets_resolved,
            l.attribution_coverage_bps::int AS attribution_coverage_bps
        FROM latest l
        LEFT JOIN earliest e ON true
        LEFT JOIN sub_agent_organization_stats s ON s.organization_id = $1
    "#;

    diesel::sql_query(query)
        .bind::<diesel::sql_types::Text, _>(organization_id)
        .bind::<diesel::sql_types::BigInt, _>(days)
        .get_result::<WindowedStatsRow>(conn)
        .await
        .optional()
        .map_err(Into::into)
}

pub async fn get_organization_statistics(
    conn: &mut Connection<'_>,
    org: &AgenticOrganizationRow,
    window: OrganizationStatsWindow,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<OrganizationStatistics> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let stats = myso_indexer_alt_social_schema::schema::sub_agent_organization_stats::table
        .filter(
            myso_indexer_alt_social_schema::schema::sub_agent_organization_stats::organization_id
                .eq(&org.organization_id),
        )
        .select(OrganizationStatsRow::as_select())
        .first(conn)
        .await
        .optional()?;

    let stats = stats.unwrap_or_else(|| OrganizationStatsRow {
        organization_id: org.organization_id.clone(),
        total_agents: 0,
        active_agents: 0,
        max_tree_depth: 0,
        total_posts: 0,
        total_comments: 0,
        total_reactions: 0,
        total_reposts: 0,
        total_engagement: 0,
        total_revenue_myso: 0,
        total_outbound_spend_myso: 0,
        net_cash_flow_myso: 0,
        estimated_assets_under_management_myso: 0,
        attribution_coverage_bps: 0,
        total_spot_participation: 0,
        spot_bets_placed: 0,
        spot_bets_resolved: 0,
        spot_bets_correct: 0,
        spot_accuracy_bps: None,
        originality_posts_analyzed: 0,
        originality_score_average_bps: None,
        total_counterparties: 0,
        total_actions_executed: 0,
        total_transactions: 0,
        last_activity_at_ms: None,
        stats_rollup_at: None,
        updated_at: Utc::now(),
    });

    let mut result = build_statistics_from_rows(org, &stats, window);

    if window != OrganizationStatsWindow::All {
        if let Some(days) = (window.days_parameter() >= 0).then_some(window.days_parameter()) {
            if let Some(overlay) =
                load_windowed_financial_overlay(conn, &org.organization_id, days).await?
            {
                result.total_revenue_myso = overlay.total_revenue_myso;
                result.total_outbound_spend_myso = overlay.total_outbound_spend_myso;
                result.net_cash_flow_myso = overlay.net_cash_flow_myso;
                result.total_counterparties = overlay.total_counterparties;
                result.active_agents = overlay.active_agents;
                result.total_engagement = overlay.total_engagement;
                result.estimated_assets_under_management_myso = overlay.estimated_aum_myso;
                result.attribution_coverage = (overlay.attribution_coverage_bps as f64) / 10_000.0;
                result.spot_bets_resolved = overlay.spot_bets_resolved;
                result.insufficient_sample =
                    overlay.spot_bets_resolved < SPOT_ACCURACY_DISPLAY_MIN_RESOLVED;
                result.spot_accuracy = if result.insufficient_sample {
                    None
                } else {
                    bps_to_ratio(overlay.spot_accuracy_bps)
                };
            }
        }
    }

    metrics.requests_succeeded.inc();
    Ok(result)
}
