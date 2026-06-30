// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Int2, Nullable, Text};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    AUM_LEADERBOARD_MIN_ATTRIBUTION_COVERAGE_BPS, AgenticOrganizationRow,
    SPOT_ACCURACY_LEADERBOARD_MIN_RESOLVED,
};
use serde::{Deserialize, Serialize};

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;
use crate::org_stats::OrganizationStatsWindow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationLeaderboardSort {
    HighestNetCashFlow,
    FastestGrowing,
    HighestAccuracy,
    MostActive,
    HighestRevenue,
    LargestEstimatedAum,
}

impl OrganizationLeaderboardSort {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "highest_net_cash_flow" | "net_cash_flow" => Some(Self::HighestNetCashFlow),
            "fastest_growing" | "growth" => Some(Self::FastestGrowing),
            "highest_accuracy" | "accuracy" => Some(Self::HighestAccuracy),
            "most_active" | "activity" => Some(Self::MostActive),
            "highest_revenue" | "revenue" => Some(Self::HighestRevenue),
            "largest_estimated_aum" | "aum" => Some(Self::LargestEstimatedAum),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationLeaderboardEntry {
    pub organization: AgenticOrganizationRow,
    pub sort_value: i64,
    pub rank: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationLeaderboardResult {
    pub entries: Vec<OrganizationLeaderboardEntry>,
    pub total: i64,
}

#[derive(Debug, QueryableByName)]
struct LeaderboardCountRow {
    #[diesel(sql_type = BigInt)]
    count: i64,
}

#[derive(Debug, QueryableByName)]
struct LeaderboardOrgRow {
    #[diesel(sql_type = Text)]
    organization_id: String,
    #[diesel(sql_type = Text)]
    account_id: String,
    #[diesel(sql_type = Text)]
    principal_owner: String,
    #[diesel(sql_type = Text)]
    profile_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    description: Option<String>,
    #[diesel(sql_type = Int2)]
    org_type: i16,
    #[diesel(sql_type = Nullable<Text>)]
    root_agent_id: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    active: bool,
    #[diesel(sql_type = BigInt)]
    created_at_ms: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    deactivated_at_ms: Option<i64>,
    #[diesel(sql_type = Text)]
    event_id: String,
    #[diesel(sql_type = Text)]
    transaction_id: String,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = BigInt)]
    sort_value: i64,
}

fn eligibility_clause(sort: OrganizationLeaderboardSort, bind_index: i32) -> String {
    match sort {
        OrganizationLeaderboardSort::HighestAccuracy => {
            format!("AND s.spot_bets_resolved >= ${bind_index}")
        }
        OrganizationLeaderboardSort::LargestEstimatedAum => {
            format!("AND s.attribution_coverage_bps >= ${bind_index}")
        }
        _ => String::new(),
    }
}

fn eligibility_bind(sort: OrganizationLeaderboardSort) -> Option<i64> {
    match sort {
        OrganizationLeaderboardSort::HighestAccuracy => {
            Some(SPOT_ACCURACY_LEADERBOARD_MIN_RESOLVED)
        }
        OrganizationLeaderboardSort::LargestEstimatedAum => {
            Some(i64::from(AUM_LEADERBOARD_MIN_ATTRIBUTION_COVERAGE_BPS))
        }
        _ => None,
    }
}

fn all_time_order_column(sort: OrganizationLeaderboardSort) -> &'static str {
    match sort {
        OrganizationLeaderboardSort::HighestNetCashFlow => "s.net_cash_flow_myso",
        OrganizationLeaderboardSort::FastestGrowing => "COALESCE(d.growth_score, 0)",
        OrganizationLeaderboardSort::HighestAccuracy => "COALESCE(s.spot_accuracy_bps, 0)",
        OrganizationLeaderboardSort::MostActive => "s.total_actions_executed",
        OrganizationLeaderboardSort::HighestRevenue => "s.total_revenue_myso",
        OrganizationLeaderboardSort::LargestEstimatedAum => {
            "s.estimated_assets_under_management_myso"
        }
    }
}

fn windowed_order_expression(sort: OrganizationLeaderboardSort) -> &'static str {
    match sort {
        OrganizationLeaderboardSort::HighestNetCashFlow => "windowed.net_cash_flow_delta",
        OrganizationLeaderboardSort::FastestGrowing => "windowed.max_growth_score",
        OrganizationLeaderboardSort::HighestAccuracy => "COALESCE(windowed.spot_accuracy_bps, 0)",
        OrganizationLeaderboardSort::MostActive => "s.total_actions_executed",
        OrganizationLeaderboardSort::HighestRevenue => "windowed.revenue_delta",
        OrganizationLeaderboardSort::LargestEstimatedAum => "windowed.latest_aum",
    }
}

pub async fn get_organization_leaderboard(
    conn: &mut Connection<'_>,
    sort: OrganizationLeaderboardSort,
    org_type: i16,
    window: OrganizationStatsWindow,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<OrganizationLeaderboardResult> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let days = window.days_parameter();
    let count_eligibility = if window == OrganizationStatsWindow::All {
        eligibility_clause(sort, 2)
    } else {
        eligibility_clause(sort, 3)
    };
    let list_eligibility = eligibility_clause(
        sort,
        if window == OrganizationStatsWindow::All {
            4
        } else {
            5
        },
    );
    let order_col = if window == OrganizationStatsWindow::All {
        all_time_order_column(sort)
    } else {
        windowed_order_expression(sort)
    };

    let count_sql = if window == OrganizationStatsWindow::All {
        format!(
            r#"
            SELECT COUNT(*)::bigint AS count
            FROM sub_agent_organizations o
            JOIN sub_agent_organization_stats s ON s.organization_id = o.organization_id
            WHERE o.active = true AND o.org_type = $1
            {count_eligibility}
            "#
        )
    } else {
        format!(
            r#"
            SELECT COUNT(*)::bigint AS count
            FROM sub_agent_organizations o
            JOIN sub_agent_organization_stats s ON s.organization_id = o.organization_id
            LEFT JOIN LATERAL (
                SELECT
                    MAX(d.growth_score) AS max_growth_score,
                    MAX(d.spot_accuracy_bps) AS spot_accuracy_bps,
                    GREATEST(MAX(d.total_revenue_myso) - MIN(d.total_revenue_myso), 0) AS revenue_delta,
                    GREATEST(MAX(d.net_cash_flow_myso) - MIN(d.net_cash_flow_myso), 0) AS net_cash_flow_delta,
                    MAX(d.estimated_aum_myso) AS latest_aum,
                    MAX(d.attribution_coverage_bps) AS attribution_coverage_bps
                FROM sub_agent_organization_stats_daily d
                WHERE d.organization_id = o.organization_id
                  AND d.snapshot_date >= CURRENT_DATE - ($2::bigint * INTERVAL '1 day')
            ) windowed ON true
            WHERE o.active = true AND o.org_type = $1
            {count_eligibility}
            "#
        )
    };

    let total = match (
        window == OrganizationStatsWindow::All,
        eligibility_bind(sort),
    ) {
        (true, Some(bind)) => {
            diesel::sql_query(&count_sql)
                .bind::<Int2, _>(org_type)
                .bind::<BigInt, _>(bind)
                .get_result::<LeaderboardCountRow>(conn)
                .await?
                .count
        }
        (true, None) => {
            diesel::sql_query(&count_sql)
                .bind::<Int2, _>(org_type)
                .get_result::<LeaderboardCountRow>(conn)
                .await?
                .count
        }
        (false, Some(bind)) => {
            diesel::sql_query(&count_sql)
                .bind::<Int2, _>(org_type)
                .bind::<BigInt, _>(days)
                .bind::<BigInt, _>(bind)
                .get_result::<LeaderboardCountRow>(conn)
                .await?
                .count
        }
        (false, None) => {
            diesel::sql_query(&count_sql)
                .bind::<Int2, _>(org_type)
                .bind::<BigInt, _>(days)
                .get_result::<LeaderboardCountRow>(conn)
                .await?
                .count
        }
    };

    let list_sql = if window == OrganizationStatsWindow::All {
        format!(
            r#"
            SELECT
                o.organization_id,
                o.account_id,
                o.principal_owner,
                o.profile_id,
                o.name,
                o.description,
                o.org_type,
                o.root_agent_id,
                o.active,
                o.created_at_ms,
                o.deactivated_at_ms,
                o.event_id,
                o.transaction_id,
                o.time,
                ({order_col})::bigint AS sort_value
            FROM sub_agent_organizations o
            JOIN sub_agent_organization_stats s ON s.organization_id = o.organization_id
            LEFT JOIN LATERAL (
                SELECT growth_score
                FROM sub_agent_organization_stats_daily d
                WHERE d.organization_id = o.organization_id
                ORDER BY d.snapshot_date DESC
                LIMIT 1
            ) d ON true
            WHERE o.active = true AND o.org_type = $1
            {list_eligibility}
            ORDER BY sort_value DESC, o.created_at_ms ASC
            LIMIT $2 OFFSET $3
            "#
        )
    } else {
        format!(
            r#"
            SELECT
                o.organization_id,
                o.account_id,
                o.principal_owner,
                o.profile_id,
                o.name,
                o.description,
                o.org_type,
                o.root_agent_id,
                o.active,
                o.created_at_ms,
                o.deactivated_at_ms,
                o.event_id,
                o.transaction_id,
                o.time,
                ({order_col})::bigint AS sort_value
            FROM sub_agent_organizations o
            JOIN sub_agent_organization_stats s ON s.organization_id = o.organization_id
            LEFT JOIN LATERAL (
                SELECT
                    MAX(d.growth_score) AS max_growth_score,
                    MAX(d.spot_accuracy_bps) AS spot_accuracy_bps,
                    GREATEST(MAX(d.total_revenue_myso) - MIN(d.total_revenue_myso), 0) AS revenue_delta,
                    GREATEST(MAX(d.net_cash_flow_myso) - MIN(d.net_cash_flow_myso), 0) AS net_cash_flow_delta,
                    MAX(d.estimated_aum_myso) AS latest_aum,
                    MAX(d.attribution_coverage_bps) AS attribution_coverage_bps
                FROM sub_agent_organization_stats_daily d
                WHERE d.organization_id = o.organization_id
                  AND d.snapshot_date >= CURRENT_DATE - ($4::bigint * INTERVAL '1 day')
            ) windowed ON true
            WHERE o.active = true AND o.org_type = $1
            {list_eligibility}
            ORDER BY sort_value DESC, o.created_at_ms ASC
            LIMIT $2 OFFSET $3
            "#
        )
    };

    let rows: Vec<LeaderboardOrgRow> = match (
        window == OrganizationStatsWindow::All,
        eligibility_bind(sort),
    ) {
        (true, Some(bind)) => {
            diesel::sql_query(&list_sql)
                .bind::<Int2, _>(org_type)
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .bind::<BigInt, _>(bind)
                .load(conn)
                .await?
        }
        (true, None) => {
            diesel::sql_query(&list_sql)
                .bind::<Int2, _>(org_type)
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .load(conn)
                .await?
        }
        (false, Some(bind)) => {
            diesel::sql_query(&list_sql)
                .bind::<Int2, _>(org_type)
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .bind::<BigInt, _>(days)
                .bind::<BigInt, _>(bind)
                .load(conn)
                .await?
        }
        (false, None) => {
            diesel::sql_query(&list_sql)
                .bind::<Int2, _>(org_type)
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .bind::<BigInt, _>(days)
                .load(conn)
                .await?
        }
    };

    let entries = rows
        .into_iter()
        .enumerate()
        .map(|(idx, row)| OrganizationLeaderboardEntry {
            organization: AgenticOrganizationRow {
                organization_id: row.organization_id,
                account_id: row.account_id,
                principal_owner: row.principal_owner,
                profile_id: row.profile_id,
                name: row.name,
                description: row.description,
                org_type: row.org_type,
                root_agent_id: row.root_agent_id,
                active: row.active,
                created_at_ms: row.created_at_ms,
                deactivated_at_ms: row.deactivated_at_ms,
                event_id: row.event_id,
                transaction_id: row.transaction_id,
                time: row.time,
            },
            sort_value: row.sort_value,
            rank: offset + idx as i64 + 1,
        })
        .collect();

    metrics.requests_succeeded.inc();
    Ok(OrganizationLeaderboardResult { entries, total })
}

/// Returns the 14 allowed organization categories for discovery APIs.
pub fn organization_categories() -> Vec<OrganizationCategoryInfo> {
    vec![
        category(0, "company", "Company"),
        category(1, "startup", "Startup"),
        category(2, "investment_fund", "Investment Fund"),
        category(3, "nonprofit", "Nonprofit"),
        category(4, "research", "Research"),
        category(5, "government", "Government"),
        category(6, "media", "Media"),
        category(7, "stewardship", "Stewardship"),
        category(8, "brand", "Brand"),
        category(9, "community", "Community"),
        category(10, "sports", "Sports"),
        category(11, "education", "Education"),
        category(12, "healthcare", "Healthcare"),
        category(13, "other", "Other"),
    ]
}

fn category(
    value: i16,
    slug: &'static str,
    display_name: &'static str,
) -> OrganizationCategoryInfo {
    OrganizationCategoryInfo {
        value,
        slug: slug.to_string(),
        display_name: display_name.to_string(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationCategoryInfo {
    pub value: i16,
    pub slug: String,
    pub display_name: String,
}

pub fn org_type_from_slug(slug: &str) -> Option<i16> {
    organization_categories()
        .into_iter()
        .find(|c| c.slug.eq_ignore_ascii_case(slug))
        .map(|c| c.value)
}
