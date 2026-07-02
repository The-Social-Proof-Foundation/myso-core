// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Tier 2 organization statistics rollup — recomputes complex metrics from source tables.

use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{NaiveDate, Utc};
use diesel::sql_query;
use diesel_async::RunQueryDsl;
use myso_futures::service::Service;
use myso_indexer_alt_framework::postgres::Connection;
use myso_pg_db::Db;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{info, warn};

const DEFAULT_ROLLUP_INTERVAL: Duration = Duration::from_secs(300);

/// Recompute Tier 2 organization metrics for all orgs with stats rows.
pub async fn run_organization_stats_rollup(conn: &mut Connection<'_>) -> Result<usize> {
    let now = Utc::now();
    let snapshot_date = now.date_naive();

    let tier2_updated = sql_query(
        r#"
        WITH spot AS (
            SELECT
                sb.organization_id,
                COUNT(*) FILTER (WHERE sr.outcome IS NOT NULL) AS resolved,
                COUNT(*) FILTER (
                    WHERE sr.outcome IS NOT NULL
                      AND sb.option_id = sr.outcome
                ) AS correct
            FROM spot_bets sb
            LEFT JOIN spot_resolutions sr ON sr.post_id = sb.post_id
            WHERE sb.organization_id IS NOT NULL
            GROUP BY sb.organization_id
        ),
        originality AS (
            SELECT
                p.organization_id,
                COUNT(*) AS posts_analyzed,
                AVG(
                    GREATEST(
                        0,
                        10000 - LEAST(COALESCE(par.highest_similarity_score, 0), 10000)
                    )
                )::INT AS avg_bps
            FROM posts p
            INNER JOIN poc_analysis_results par ON par.post_id = p.post_id
            WHERE p.organization_id IS NOT NULL
            GROUP BY p.organization_id
        ),
        aum AS (
            SELECT
                org_id AS organization_id,
                COALESCE(SUM(balance), 0) AS estimated_aum,
                CASE
                    WHEN COUNT(*) = 0 THEN 0
                    ELSE (COUNT(*) FILTER (WHERE has_attr) * 10000) / COUNT(*)
                END AS coverage_bps
            FROM (
                SELECT sb.organization_id AS org_id,
                       COALESCE(sb.escrow_amount, 0) AS balance,
                       sb.organization_id IS NOT NULL AS has_attr
                FROM spot_bets sb
                WHERE sb.organization_id IS NOT NULL
                UNION ALL
                SELECT sr.organization_id AS org_id,
                       COALESCE(sr.amount, 0) AS balance,
                       sr.organization_id IS NOT NULL AS has_attr
                FROM spt_reservations sr
                WHERE sr.organization_id IS NOT NULL
            ) financial
            GROUP BY org_id
        ),
        engagement AS (
            SELECT
                os.organization_id,
                os.total_reactions
                    + (3 * os.total_comments)
                    + (2 * os.total_reposts) AS weighted_engagement
            FROM sub_agent_organization_stats os
        ),
        ai_credit AS (
            SELECT
                l.organization_id,
                COALESCE(SUM(l.amount_mist) FILTER (WHERE l.settled), 0) AS spent_mist,
                COUNT(*) AS usage_events
            FROM ai_credit_usage_lines l
            WHERE l.organization_id IS NOT NULL
            GROUP BY l.organization_id
        ),
        memory_usage AS (
            SELECT
                m.organization_id,
                COALESCE(SUM(m.entries), 0) AS entries,
                COALESCE(SUM(m.bytes), 0) AS bytes,
                COALESCE(SUM(m.org_shared_entries), 0) AS org_shared_entries
            FROM memory_usage_stats m
            WHERE m.organization_id IS NOT NULL
            GROUP BY m.organization_id
        )
        UPDATE sub_agent_organization_stats os
        SET
            net_cash_flow_myso = os.total_revenue_myso - os.total_outbound_spend_myso,
            spot_bets_resolved = COALESCE(sp.resolved, 0),
            spot_bets_correct = COALESCE(sp.correct, 0),
            spot_accuracy_bps = CASE
                WHEN COALESCE(sp.resolved, 0) >= 5 AND COALESCE(sp.resolved, 0) > 0
                THEN (COALESCE(sp.correct, 0) * 10000) / sp.resolved
                ELSE NULL
            END,
            originality_posts_analyzed = COALESCE(o.posts_analyzed, 0),
            originality_score_average_bps = o.avg_bps,
            estimated_assets_under_management_myso = COALESCE(a.estimated_aum, 0),
            attribution_coverage_bps = COALESCE(a.coverage_bps, 0),
            total_engagement = COALESCE(e.weighted_engagement, 0),
            ai_credit_spent_mist = COALESCE(ac.spent_mist, os.ai_credit_spent_mist),
            ai_credit_usage_events = COALESCE(ac.usage_events, os.ai_credit_usage_events),
            memory_entries = COALESCE(mu.entries, 0),
            memory_bytes = COALESCE(mu.bytes, 0),
            org_shared_memory_entries = COALESCE(mu.org_shared_entries, 0),
            stats_rollup_at = $1,
            updated_at = $1
        FROM engagement e
        LEFT JOIN spot sp ON sp.organization_id = os.organization_id
        LEFT JOIN originality o ON o.organization_id = os.organization_id
        LEFT JOIN aum a ON a.organization_id = os.organization_id
        LEFT JOIN ai_credit ac ON ac.organization_id = os.organization_id
        LEFT JOIN memory_usage mu ON mu.organization_id = os.organization_id
        WHERE os.organization_id = e.organization_id
        "#,
    )
    .bind::<diesel::sql_types::Timestamptz, _>(now)
    .execute(conn)
    .await
    .context("sub_agent_organization_stats tier-2 update")?;

    // Expire stale spend approvals: approved allowances past their on-chain expiry, and
    // requested rows the owner never acted on within seven days.
    let approvals_expired = sql_query(
        r#"
        UPDATE ai_credit_spend_approvals
        SET status = 'expired', updated_at = $1
        WHERE (status = 'approved' AND expires_at_ms IS NOT NULL AND expires_at_ms < $2)
           OR (status = 'requested' AND requested_at < $1 - INTERVAL '7 days')
        "#,
    )
    .bind::<diesel::sql_types::Timestamptz, _>(now)
    .bind::<diesel::sql_types::BigInt, _>(now.timestamp_millis())
    .execute(conn)
    .await
    .context("ai_credit_spend_approvals expiry sweep")?;

    let daily_updated = sql_query(
        r#"
        INSERT INTO sub_agent_organization_stats_daily (
            organization_id,
            org_type,
            snapshot_date,
            total_revenue_myso,
            net_cash_flow_myso,
            total_outbound_spend_myso,
            total_counterparties,
            active_agents,
            total_engagement,
            estimated_aum_myso,
            total_actions_executed,
            growth_score,
            spot_accuracy_bps,
            attribution_coverage_bps,
            ai_credit_spent_mist,
            memory_bytes,
            time
        )
        SELECT
            os.organization_id,
            ao.org_type,
            $2::date,
            os.total_revenue_myso,
            os.net_cash_flow_myso,
            os.total_outbound_spend_myso,
            os.total_counterparties,
            os.active_agents,
            os.total_engagement,
            os.estimated_assets_under_management_myso,
            os.total_actions_executed,
            (
                5 * GREATEST(
                    os.total_revenue_myso - COALESCE(prev.total_revenue_myso, 0),
                    0
                )
                + 3 * GREATEST(
                    os.total_counterparties - COALESCE(prev.total_counterparties, 0),
                    0
                )
                + 3 * CASE
                    WHEN os.active_agents > 0 THEN os.active_agents * 1000
                    ELSE 0
                  END
                + os.total_engagement
            ) AS growth_score,
            os.spot_accuracy_bps,
            os.attribution_coverage_bps,
            os.ai_credit_spent_mist,
            os.memory_bytes,
            $1
        FROM sub_agent_organization_stats os
        INNER JOIN sub_agent_organizations ao ON ao.organization_id = os.organization_id
        LEFT JOIN LATERAL (
            SELECT d.total_revenue_myso, d.total_counterparties
            FROM sub_agent_organization_stats_daily d
            WHERE d.organization_id = os.organization_id
              AND d.snapshot_date <= ($2::date - INTERVAL '30 days')::date
            ORDER BY d.snapshot_date DESC
            LIMIT 1
        ) prev ON TRUE
        ON CONFLICT (organization_id, snapshot_date) DO UPDATE SET
            total_revenue_myso = EXCLUDED.total_revenue_myso,
            net_cash_flow_myso = EXCLUDED.net_cash_flow_myso,
            total_outbound_spend_myso = EXCLUDED.total_outbound_spend_myso,
            total_counterparties = EXCLUDED.total_counterparties,
            active_agents = EXCLUDED.active_agents,
            total_engagement = EXCLUDED.total_engagement,
            estimated_aum_myso = EXCLUDED.estimated_aum_myso,
            total_actions_executed = EXCLUDED.total_actions_executed,
            growth_score = EXCLUDED.growth_score,
            spot_accuracy_bps = EXCLUDED.spot_accuracy_bps,
            attribution_coverage_bps = EXCLUDED.attribution_coverage_bps,
            ai_credit_spent_mist = EXCLUDED.ai_credit_spent_mist,
            memory_bytes = EXCLUDED.memory_bytes,
            time = EXCLUDED.time
        "#,
    )
    .bind::<diesel::sql_types::Timestamptz, _>(now)
    .bind::<diesel::sql_types::Date, _>(snapshot_date)
    .execute(conn)
    .await
    .context("sub_agent_organization_stats_daily snapshot")?;

    info!(
        tier2_rows = tier2_updated,
        daily_rows = daily_updated,
        approvals_expired,
        "organization stats rollup completed"
    );
    Ok(tier2_updated + daily_updated + approvals_expired)
}

/// Spawn a background service that runs the rollup on a fixed interval.
pub fn spawn_rollup_service(store: Db, rollup_interval: Duration) -> Service {
    Service::new().spawn_aborting(async move {
        let mut tick = interval(rollup_interval);
        tick.set_missed_tick_behavior(MissedTickBehavior::Delay);
        loop {
            tick.tick().await;
            match store.connect().await {
                Ok(mut conn) => {
                    if let Err(e) = run_organization_stats_rollup(&mut conn).await {
                        warn!(error = %e, "organization stats rollup failed");
                    }
                }
                Err(e) => {
                    warn!(error = %e, "organization stats rollup: db connect failed");
                }
            }
        }
    })
}

pub fn spawn_default_rollup_service(store: Db) -> Service {
    spawn_rollup_service(store, DEFAULT_ROLLUP_INTERVAL)
}

#[allow(dead_code)]
pub fn today_snapshot_date() -> NaiveDate {
    Utc::now().date_naive()
}
