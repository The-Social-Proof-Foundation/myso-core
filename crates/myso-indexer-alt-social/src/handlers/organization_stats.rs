// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Tier 1 synchronous organization statistics helpers.
//!
//! Simple integer counters and counterparty upserts run in the same DB transaction as the
//! source handler commit. Complex metrics (accuracy, AUM, engagement composite, growth score)
//! are recomputed by [`super::organization_stats_rollup`].

use anyhow::Result;
use chrono::Utc;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Int2, Int4, Text};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_social_schema::models::NewOrganizationStats;

#[derive(Debug, Clone, Copy)]
pub enum OrgStatColumn {
    TotalPosts,
    TotalComments,
    TotalReactions,
    TotalReposts,
    TotalAgents,
    ActiveAgents,
    TotalActionsExecuted,
}

impl OrgStatColumn {
    fn sql_name(self) -> &'static str {
        match self {
            Self::TotalPosts => "total_posts",
            Self::TotalComments => "total_comments",
            Self::TotalReactions => "total_reactions",
            Self::TotalReposts => "total_reposts",
            Self::TotalAgents => "total_agents",
            Self::ActiveAgents => "active_agents",
            Self::TotalActionsExecuted => "total_actions_executed",
        }
    }
}

pub async fn init_org_stats(
    conn: &mut Connection<'_>,
    organization_id: &str,
    activity_at_ms: i64,
) -> Result<()> {
    let now = Utc::now();
    let row = NewOrganizationStats {
        organization_id: organization_id.to_string(),
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
        last_activity_at_ms: Some(activity_at_ms),
        stats_rollup_at: None,
        updated_at: now,
        ai_credit_spent_mist: 0,
        ai_credit_usage_events: 0,
        memory_entries: 0,
        memory_bytes: 0,
        org_shared_memory_entries: 0,
    };
    diesel::insert_into(
        myso_indexer_alt_social_schema::schema::sub_agent_organization_stats::table,
    )
    .values(&row)
    .on_conflict(
        myso_indexer_alt_social_schema::schema::sub_agent_organization_stats::organization_id,
    )
    .do_nothing()
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn increment_org_stat(
    conn: &mut Connection<'_>,
    organization_id: &str,
    column: OrgStatColumn,
    delta: i64,
    activity_at_ms: i64,
) -> Result<()> {
    if delta == 0 {
        return Ok(());
    }
    let sql = format!(
        "UPDATE sub_agent_organization_stats SET {col} = {col} + $2, \
         last_activity_at_ms = GREATEST(COALESCE(last_activity_at_ms, 0), $3), \
         updated_at = NOW() \
         WHERE organization_id = $1",
        col = column.sql_name()
    );
    sql_query(&sql)
        .bind::<Text, _>(organization_id)
        .bind::<BigInt, _>(delta)
        .bind::<BigInt, _>(activity_at_ms)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn bump_org_max_tree_depth(
    conn: &mut Connection<'_>,
    organization_id: &str,
    depth: i16,
    activity_at_ms: i64,
) -> Result<()> {
    sql_query(
        "UPDATE sub_agent_organization_stats SET max_tree_depth = GREATEST(max_tree_depth, $2), \
         last_activity_at_ms = GREATEST(COALESCE(last_activity_at_ms, 0), $3), \
         updated_at = NOW() \
         WHERE organization_id = $1",
    )
    .bind::<Text, _>(organization_id)
    .bind::<Int2, _>(depth)
    .bind::<BigInt, _>(activity_at_ms)
    .execute(conn)
    .await?;
    Ok(())
}

/// Upserts a counterparty row. Returns `true` when this is a new distinct counterparty.
pub async fn record_counterparty(
    conn: &mut Connection<'_>,
    organization_id: &str,
    counterparty_address: &str,
    at_ms: i64,
) -> Result<bool> {
    let updated = sql_query(
        "UPDATE sub_agent_organization_counterparties SET \
         last_interaction_at_ms = GREATEST(last_interaction_at_ms, $3), \
         interaction_count = interaction_count + 1 \
         WHERE organization_id = $1 AND counterparty_address = $2",
    )
    .bind::<Text, _>(organization_id)
    .bind::<Text, _>(counterparty_address)
    .bind::<BigInt, _>(at_ms)
    .execute(conn)
    .await?;

    if updated > 0 {
        return Ok(false);
    }

    let inserted = sql_query(
        "INSERT INTO sub_agent_organization_counterparties \
         (organization_id, counterparty_address, first_interaction_at_ms, last_interaction_at_ms, interaction_count) \
         VALUES ($1, $2, $3, $3, 1) \
         ON CONFLICT (organization_id, counterparty_address) DO NOTHING",
    )
    .bind::<Text, _>(organization_id)
    .bind::<Text, _>(counterparty_address)
    .bind::<BigInt, _>(at_ms)
    .execute(conn)
    .await?;

    if inserted > 0 {
        sql_query(
            "UPDATE sub_agent_organization_stats SET total_counterparties = total_counterparties + 1, \
             last_activity_at_ms = GREATEST(COALESCE(last_activity_at_ms, 0), $2), \
             updated_at = NOW() \
             WHERE organization_id = $1",
        )
        .bind::<Text, _>(organization_id)
        .bind::<BigInt, _>(at_ms)
        .execute(conn)
        .await?;
        return Ok(true);
    }

    // Concurrent insert won the race; treat as existing counterparty.
    sql_query(
        "UPDATE sub_agent_organization_counterparties SET \
         last_interaction_at_ms = GREATEST(last_interaction_at_ms, $3), \
         interaction_count = interaction_count + 1 \
         WHERE organization_id = $1 AND counterparty_address = $2",
    )
    .bind::<Text, _>(organization_id)
    .bind::<Text, _>(counterparty_address)
    .bind::<BigInt, _>(at_ms)
    .execute(conn)
    .await?;
    Ok(false)
}

pub async fn apply_sub_agent_registration_stats(
    conn: &mut Connection<'_>,
    organization_id: &str,
    active: bool,
    depth: i16,
    activity_at_ms: i64,
) -> Result<()> {
    increment_org_stat(
        conn,
        organization_id,
        OrgStatColumn::TotalAgents,
        1,
        activity_at_ms,
    )
    .await?;
    if active {
        increment_org_stat(
            conn,
            organization_id,
            OrgStatColumn::ActiveAgents,
            1,
            activity_at_ms,
        )
        .await?;
    }
    bump_org_max_tree_depth(conn, organization_id, depth, activity_at_ms).await?;
    increment_org_stat(
        conn,
        organization_id,
        OrgStatColumn::TotalActionsExecuted,
        1,
        activity_at_ms,
    )
    .await?;
    Ok(())
}

pub async fn apply_sub_agent_active_delta(
    conn: &mut Connection<'_>,
    organization_id: &str,
    delta: i32,
    activity_at_ms: i64,
) -> Result<()> {
    if delta == 0 {
        return Ok(());
    }
    sql_query(
        "UPDATE sub_agent_organization_stats SET active_agents = GREATEST(active_agents + $2, 0), \
         last_activity_at_ms = GREATEST(COALESCE(last_activity_at_ms, 0), $3), \
         updated_at = NOW() \
         WHERE organization_id = $1",
    )
    .bind::<Text, _>(organization_id)
    .bind::<Int4, _>(delta)
    .bind::<BigInt, _>(activity_at_ms)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn set_org_root_agent(
    conn: &mut Connection<'_>,
    organization_id: &str,
    root_agent_id: &str,
) -> Result<()> {
    diesel::update(
        myso_indexer_alt_social_schema::schema::sub_agent_organizations::table.filter(
            myso_indexer_alt_social_schema::schema::sub_agent_organizations::organization_id
                .eq(organization_id),
        ),
    )
    .set(
        myso_indexer_alt_social_schema::schema::sub_agent_organizations::root_agent_id
            .eq(Some(root_agent_id.to_string())),
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn resolve_organization_id_for_sub_agent(
    conn: &mut Connection<'_>,
    sub_agent_id: &str,
) -> Result<Option<String>> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use myso_indexer_alt_social_schema::schema::sub_agents;

    let org_id = sub_agents::table
        .filter(sub_agents::agent_object_id.eq(sub_agent_id))
        .select(sub_agents::organization_id)
        .first::<Option<String>>(conn)
        .await
        .ok();
    Ok(org_id.flatten())
}

pub async fn stamp_and_count_social_action(
    conn: &mut Connection<'_>,
    organization_id: Option<&str>,
    column: OrgStatColumn,
    activity_at_ms: i64,
    counterparty: Option<&str>,
) -> Result<()> {
    let Some(organization_id) = organization_id else {
        return Ok(());
    };
    increment_org_stat(conn, organization_id, column, 1, activity_at_ms).await?;
    increment_org_stat(
        conn,
        organization_id,
        OrgStatColumn::TotalActionsExecuted,
        1,
        activity_at_ms,
    )
    .await?;
    if let Some(counterparty) = counterparty {
        record_counterparty(conn, organization_id, counterparty, activity_at_ms).await?;
    }
    Ok(())
}

pub async fn resolve_organization_id_for_derived_address(
    conn: &mut Connection<'_>,
    derived_address: &str,
) -> Result<Option<String>> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use myso_indexer_alt_social_schema::schema::sub_agents;

    let org_id = sub_agents::table
        .filter(sub_agents::derived_address.eq(derived_address))
        .filter(sub_agents::active.eq(true))
        .select(sub_agents::organization_id)
        .first::<Option<String>>(conn)
        .await
        .optional()?;
    Ok(org_id.flatten())
}

pub async fn resolve_organization_id_for_post(
    conn: &mut Connection<'_>,
    post_id: &str,
) -> Result<Option<String>> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use myso_indexer_alt_social_schema::schema::posts;

    let org_id = posts::table
        .filter(posts::post_id.eq(post_id))
        .select(posts::organization_id)
        .first::<Option<String>>(conn)
        .await
        .optional()?;
    Ok(org_id.flatten())
}

async fn bump_org_bigint_column(
    conn: &mut Connection<'_>,
    organization_id: &str,
    column: &str,
    delta: i64,
    activity_at_ms: i64,
) -> Result<()> {
    if delta == 0 {
        return Ok(());
    }
    let sql = format!(
        "UPDATE sub_agent_organization_stats SET {column} = {column} + $2, \
         last_activity_at_ms = GREATEST(COALESCE(last_activity_at_ms, 0), $3), \
         updated_at = NOW() \
         WHERE organization_id = $1",
    );
    sql_query(&sql)
        .bind::<Text, _>(organization_id)
        .bind::<BigInt, _>(delta)
        .bind::<BigInt, _>(activity_at_ms)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn apply_org_revenue(
    conn: &mut Connection<'_>,
    organization_id: Option<&str>,
    amount: i64,
    counterparty: Option<&str>,
    activity_at_ms: i64,
) -> Result<()> {
    let Some(organization_id) = organization_id else {
        return Ok(());
    };
    bump_org_bigint_column(
        conn,
        organization_id,
        "total_revenue_myso",
        amount,
        activity_at_ms,
    )
    .await?;
    bump_org_bigint_column(
        conn,
        organization_id,
        "total_transactions",
        1,
        activity_at_ms,
    )
    .await?;
    increment_org_stat(
        conn,
        organization_id,
        OrgStatColumn::TotalActionsExecuted,
        1,
        activity_at_ms,
    )
    .await?;
    if let Some(counterparty) = counterparty {
        record_counterparty(conn, organization_id, counterparty, activity_at_ms).await?;
    }
    Ok(())
}

pub async fn apply_org_outbound_spend(
    conn: &mut Connection<'_>,
    organization_id: Option<&str>,
    amount: i64,
    counterparty: Option<&str>,
    activity_at_ms: i64,
) -> Result<()> {
    let Some(organization_id) = organization_id else {
        return Ok(());
    };
    bump_org_bigint_column(
        conn,
        organization_id,
        "total_outbound_spend_myso",
        amount,
        activity_at_ms,
    )
    .await?;
    bump_org_bigint_column(
        conn,
        organization_id,
        "total_transactions",
        1,
        activity_at_ms,
    )
    .await?;
    increment_org_stat(
        conn,
        organization_id,
        OrgStatColumn::TotalActionsExecuted,
        1,
        activity_at_ms,
    )
    .await?;
    if let Some(counterparty) = counterparty {
        record_counterparty(conn, organization_id, counterparty, activity_at_ms).await?;
    }
    Ok(())
}

/// Tier 1 AI-credit spend attribution: bumps `ai_credit_spent_mist` and
/// `ai_credit_usage_events` on the org stats row when the settling agent belongs to an org.
pub async fn apply_org_ai_credit_spend(
    conn: &mut Connection<'_>,
    organization_id: Option<&str>,
    amount_mist: i64,
    activity_at_ms: i64,
) -> Result<()> {
    let Some(organization_id) = organization_id else {
        return Ok(());
    };
    bump_org_bigint_column(
        conn,
        organization_id,
        "ai_credit_spent_mist",
        amount_mist,
        activity_at_ms,
    )
    .await?;
    bump_org_bigint_column(
        conn,
        organization_id,
        "ai_credit_usage_events",
        1,
        activity_at_ms,
    )
    .await?;
    Ok(())
}

pub async fn apply_spot_bet_stats(
    conn: &mut Connection<'_>,
    bettor_org: Option<&str>,
    post_org: Option<&str>,
    escrow_amount: i64,
    bettor_address: &str,
    activity_at_ms: i64,
) -> Result<()> {
    if let Some(org_id) = bettor_org {
        bump_org_bigint_column(conn, org_id, "spot_bets_placed", 1, activity_at_ms).await?;
        bump_org_bigint_column(conn, org_id, "total_spot_participation", 1, activity_at_ms).await?;
        apply_org_outbound_spend(conn, Some(org_id), escrow_amount, None, activity_at_ms).await?;
    }
    if let Some(org_id) = post_org {
        if bettor_org != Some(org_id) {
            bump_org_bigint_column(conn, org_id, "total_spot_participation", 1, activity_at_ms)
                .await?;
        }
        record_counterparty(conn, org_id, bettor_address, activity_at_ms).await?;
    }
    Ok(())
}
