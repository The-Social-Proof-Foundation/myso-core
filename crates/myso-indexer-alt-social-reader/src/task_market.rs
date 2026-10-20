// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Task market reads for GraphQL: the latest `TaskConfigUpdated` and an agent's reputation.
//! Scores are derived from the indexed sums with the market's current prior, never read from
//! the cached column, so a prior change takes effect immediately.

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::Selectable;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::{
    credibility, reputation_aggregates, sub_agents, task_config_history,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;
use crate::reputation::{Aggregate, Score, ScoringParams, overall_score, parse_sum, role_score};

const ROLE_SELLER: i16 = 1;
const ROLE_BUYER: i16 = 2;

/// The latest `TaskConfigUpdated`, every field.
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = task_config_history)]
pub struct TaskConfigRow {
    pub updated_by: String,
    pub platform_fee_bps: i64,
    pub ecosystem_fee_bps: i64,
    pub ecosystem_fee_recipient: String,
    pub min_escrow_mist: i64,
    pub accept_window_min_ms: i64,
    pub accept_window_default_ms: i64,
    pub accept_window_max_ms: i64,
    pub delivery_window_min_ms: i64,
    pub delivery_window_max_ms: i64,
    pub settle_window_min_ms: i64,
    pub settle_window_max_ms: i64,
    pub release_grace_ms: i64,
    pub review_window_ms: i64,
    pub max_title_bytes: i64,
    pub max_uri_bytes: i64,
    pub active: bool,
    pub prior_mean_milli: i64,
    pub prior_weight_bps: i64,
    pub value_ref_mist: i64,
    pub min_value_weight_bps: i64,
    pub max_value_weight_bps: i64,
    pub cred_base_bps: i64,
    pub cred_step_bps: i64,
    pub cred_cap_n: i64,
    pub cred_max_bps: i64,
    pub overall_seller_weight_bps: i64,
    pub overall_buyer_weight_bps: i64,
    pub timestamp_ms: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

impl TaskConfigRow {
    pub fn scoring_params(&self) -> ScoringParams {
        ScoringParams {
            prior_mean_milli: self.prior_mean_milli.max(0) as u64,
            prior_weight_bps: self.prior_weight_bps.max(0) as u64,
            overall_seller_weight_bps: self.overall_seller_weight_bps.max(0) as u64,
            overall_buyer_weight_bps: self.overall_buyer_weight_bps.max(0) as u64,
        }
    }
}

/// One role's counts and derived score.
#[derive(Debug, Clone)]
pub struct RoleReputation {
    pub review_count: i64,
    /// Counts of 1 through 5 star reviews.
    pub histogram: [i64; 5],
    pub score_milli: i64,
    pub confidence_bps: i64,
    pub last_receipt_id: Option<String>,
    pub updated_at_ms: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct CredibilityRow {
    pub account_id: String,
    pub settled_count: i64,
    /// Decimal string: the on-chain volume is a `u128`.
    pub volume_mist: String,
    pub credibility_bps: i64,
}

/// An agent's own seller and buyer reputation, the blended overall score, and its
/// principal's credibility.
#[derive(Debug, Clone)]
pub struct AgentReputation {
    pub as_seller: RoleReputation,
    pub as_buyer: RoleReputation,
    pub overall_score_milli: i64,
    pub overall_confidence_bps: i64,
    pub credibility: Option<CredibilityRow>,
}

type AggregateTuple = (i64, i64, i64, i64, i64, i64, String, String, String, i64);

fn to_aggregate(row: &Option<AggregateTuple>) -> (Aggregate, Option<String>, Option<i64>) {
    match row {
        None => (Aggregate::default(), None, None),
        Some((count, s1, s2, s3, s4, s5, w, s, receipt, updated)) => (
            Aggregate {
                review_count: (*count).max(0) as u64,
                histogram: [*s1, *s2, *s3, *s4, *s5].map(|n| n.max(0) as u64),
                // Validated decimal strings; a malformed value degrades to empty, not wrong.
                weight_sum_bps: parse_sum(w).unwrap_or(0),
                weighted_star_sum: parse_sum(s).unwrap_or(0),
            },
            Some(receipt.clone()),
            Some(*updated),
        ),
    }
}

fn role_reputation(
    agg: &Aggregate,
    receipt: Option<String>,
    updated: Option<i64>,
    p: &ScoringParams,
) -> RoleReputation {
    let score = role_score(agg, p).unwrap_or(Score {
        score_milli: p.prior_mean_milli,
        confidence_bps: 0,
    });
    RoleReputation {
        review_count: agg.review_count as i64,
        histogram: agg.histogram.map(|n| n as i64),
        score_milli: score.score_milli as i64,
        confidence_bps: score.confidence_bps as i64,
        last_receipt_id: receipt,
        updated_at_ms: updated,
    }
}

pub(crate) async fn get_task_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<TaskConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = task_config_history::table
        .order(task_config_history::id.desc())
        .select(TaskConfigRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

async fn load_role(
    conn: &mut Connection<'_>,
    agent_object_id: &str,
    role: i16,
) -> anyhow::Result<Option<AggregateTuple>> {
    Ok(reputation_aggregates::table
        .filter(reputation_aggregates::subject_id.eq(agent_object_id))
        .filter(reputation_aggregates::role.eq(role))
        .select((
            reputation_aggregates::review_count,
            reputation_aggregates::star_1,
            reputation_aggregates::star_2,
            reputation_aggregates::star_3,
            reputation_aggregates::star_4,
            reputation_aggregates::star_5,
            reputation_aggregates::weight_sum_bps,
            reputation_aggregates::weighted_star_sum,
            reputation_aggregates::last_receipt_id,
            reputation_aggregates::updated_at_ms,
        ))
        .first::<AggregateTuple>(conn)
        .await
        .optional()?)
}

/// `None` only when the agent itself is unknown; an agent with no reviews returns the prior.
pub(crate) async fn get_agent_reputation(
    conn: &mut Connection<'_>,
    agent_object_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<AgentReputation>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let account_id = sub_agents::table
        .filter(sub_agents::agent_object_id.eq(agent_object_id))
        .select(sub_agents::account_id)
        .first::<String>(conn)
        .await
        .optional()?;
    let Some(account_id) = account_id else {
        metrics.requests_succeeded.inc();
        return Ok(None);
    };

    let params = task_config_history::table
        .order(task_config_history::id.desc())
        .select(TaskConfigRow::as_select())
        .first(conn)
        .await
        .optional()?
        .map(|c| c.scoring_params())
        .unwrap_or_default();

    let seller_row = load_role(conn, agent_object_id, ROLE_SELLER).await?;
    let buyer_row = load_role(conn, agent_object_id, ROLE_BUYER).await?;
    let (seller, seller_receipt, seller_at) = to_aggregate(&seller_row);
    let (buyer, buyer_receipt, buyer_at) = to_aggregate(&buyer_row);
    let overall = overall_score(&seller, &buyer, &params).unwrap_or(Score {
        score_milli: params.prior_mean_milli,
        confidence_bps: 0,
    });

    let credibility = credibility::table
        .filter(credibility::account_id.eq(&account_id))
        .select((
            credibility::account_id,
            credibility::settled_count,
            credibility::volume_mist,
            credibility::credibility_bps,
        ))
        .first::<(String, i64, String, i64)>(conn)
        .await
        .optional()?
        .map(
            |(account_id, settled_count, volume_mist, credibility_bps)| CredibilityRow {
                account_id,
                settled_count,
                volume_mist,
                credibility_bps,
            },
        );

    metrics.requests_succeeded.inc();
    Ok(Some(AgentReputation {
        as_seller: role_reputation(&seller, seller_receipt, seller_at, &params),
        as_buyer: role_reputation(&buyer, buyer_receipt, buyer_at, &params),
        overall_score_milli: overall.score_milli as i64,
        overall_confidence_bps: overall.confidence_bps as i64,
        credibility,
    }))
}
