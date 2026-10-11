// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Task market reads: tasks, receipts, config and reputation (scores derived from indexed sums).

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_reader::reputation::{
    Aggregate, ScoringParams, Score, overall_score, parse_sum, role_score,
};
use myso_indexer_alt_social_schema::schema::{
    agent_tasks, credibility, memory_accounts, reputation_aggregates, sub_agents, task_config_history,
    task_receipts, task_reviews,
};
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::{
    AgentReputationInfo, AgentTaskInfo, CredibilityInfo, OverallScoreInfo, PrincipalReputationInfo,
    ReputationScoreInfo, TaskConfigInfo, TaskDetailInfo, TaskReceiptInfo, TaskReviewInfo,
};

const ROLE_SELLER: i16 = 1;
const ROLE_BUYER: i16 = 2;

/// Which tasks `GET /tasks` returns. At least one scoping field is required, so the endpoint
/// never dumps the whole market.
#[derive(Debug, Default, Clone)]
pub struct TaskFilter {
    pub buyer_principal: Option<String>,
    pub buyer_agent_id: Option<String>,
    pub seller_principal: Option<String>,
    pub seller_agent_id: Option<String>,
    pub organization_id: Option<String>,
    pub room_group_id: Option<String>,
    /// Task status codes (0 offered ... 5 cancelled); empty means all.
    pub statuses: Vec<i16>,
}

impl TaskFilter {
    pub fn is_scoped(&self) -> bool {
        self.buyer_principal.is_some()
            || self.buyer_agent_id.is_some()
            || self.seller_principal.is_some()
            || self.seller_agent_id.is_some()
            || self.organization_id.is_some()
            || self.room_group_id.is_some()
    }
}

pub(crate) async fn list_tasks(
    db: &Db,
    filter: &TaskFilter,
    limit: i64,
    offset: i64,
) -> Result<Vec<AgentTaskInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = agent_tasks::table.into_boxed();
    if let Some(v) = &filter.buyer_principal {
        query = query.filter(agent_tasks::buyer_principal.eq(v));
    }
    if let Some(v) = &filter.buyer_agent_id {
        query = query.filter(agent_tasks::buyer_agent_id.eq(v));
    }
    if let Some(v) = &filter.seller_principal {
        query = query.filter(agent_tasks::seller_principal.eq(v));
    }
    if let Some(v) = &filter.seller_agent_id {
        query = query.filter(agent_tasks::seller_agent_id.eq(v));
    }
    if let Some(v) = &filter.organization_id {
        query = query.filter(agent_tasks::organization_id.eq(v));
    }
    if let Some(v) = &filter.room_group_id {
        query = query.filter(agent_tasks::room_group_id.eq(v));
    }
    if !filter.statuses.is_empty() {
        query = query.filter(agent_tasks::status.eq_any(filter.statuses.clone()));
    }
    let rows = query
        .order((agent_tasks::created_at_ms.desc(), agent_tasks::task_id.asc()))
        .limit(limit)
        .offset(offset)
        .select(AgentTaskInfo::as_select())
        .load(&mut conn)
        .await?;
    Ok(rows)
}

pub(crate) async fn get_task(db: &Db, task_id: &str) -> Result<Option<TaskDetailInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let task = agent_tasks::table
        .filter(agent_tasks::task_id.eq(task_id))
        .select(AgentTaskInfo::as_select())
        .first(&mut conn)
        .await
        .optional()?;
    let Some(task) = task else {
        return Ok(None);
    };
    let reviews = task_reviews::table
        .filter(task_reviews::task_id.eq(task_id))
        .order(task_reviews::rater_role.asc())
        .select(TaskReviewInfo::as_select())
        .load(&mut conn)
        .await?;
    Ok(Some(TaskDetailInfo { task, reviews }))
}

/// Receipts where the agent was seller or buyer, newest first.
pub(crate) async fn list_agent_receipts(
    db: &Db,
    agent_object_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<TaskReceiptInfo>, SocialError> {
    use diesel::BoolExpressionMethods;
    let mut conn = db.connect().await?;
    let rows = task_receipts::table
        .filter(
            task_receipts::seller_agent_id
                .eq(agent_object_id)
                .or(task_receipts::buyer_agent_id.eq(agent_object_id)),
        )
        .order((task_receipts::settled_at_ms.desc(), task_receipts::task_id.asc()))
        .limit(limit)
        .offset(offset)
        .select(TaskReceiptInfo::as_select())
        .load(&mut conn)
        .await?;
    Ok(rows)
}

pub(crate) async fn get_task_config(db: &Db) -> Result<Option<TaskConfigInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let row = task_config_history::table
        .order(task_config_history::id.desc())
        .select(TaskConfigInfo::as_select())
        .first(&mut conn)
        .await
        .optional()?;
    Ok(row)
}

fn scoring_params(config: Option<&TaskConfigInfo>) -> ScoringParams {
    match config {
        Some(c) => ScoringParams {
            prior_mean_milli: c.prior_mean_milli.max(0) as u64,
            prior_weight_bps: c.prior_weight_bps.max(0) as u64,
            overall_seller_weight_bps: c.overall_seller_weight_bps.max(0) as u64,
            overall_buyer_weight_bps: c.overall_buyer_weight_bps.max(0) as u64,
        },
        None => ScoringParams::default(),
    }
}

type AggregateRow = (i64, i64, i64, i64, i64, i64, String, String, i64, String, i64);

fn load_aggregate(row: Option<AggregateRow>) -> (Aggregate, Option<String>, Option<i64>) {
    let Some((count, s1, s2, s3, s4, s5, w, s, _score, last_receipt, updated)) = row else {
        return (Aggregate::default(), None, None);
    };
    let aggregate = Aggregate {
        review_count: count.max(0) as u64,
        histogram: [s1, s2, s3, s4, s5].map(|n| n.max(0) as u64),
        // The indexer only stores validated decimal strings; a bad value degrades to empty
        // rather than serving a wrong score.
        weight_sum_bps: parse_sum(&w).unwrap_or(0),
        weighted_star_sum: parse_sum(&s).unwrap_or(0),
    };
    (aggregate, Some(last_receipt), Some(updated))
}

fn score_info(
    aggregate: &Aggregate,
    last_receipt_id: Option<String>,
    updated_at_ms: Option<i64>,
    params: &ScoringParams,
) -> ReputationScoreInfo {
    let score = role_score(aggregate, params).unwrap_or(Score {
        score_milli: params.prior_mean_milli,
        confidence_bps: 0,
    });
    ReputationScoreInfo {
        review_count: aggregate.review_count as i64,
        histogram: aggregate.histogram.map(|n| n as i64),
        score_milli: score.score_milli as i64,
        score: score.stars(),
        confidence_bps: score.confidence_bps as i64,
        last_receipt_id,
        updated_at_ms,
    }
}

fn overall_info(seller: &Aggregate, buyer: &Aggregate, params: &ScoringParams) -> OverallScoreInfo {
    let score = overall_score(seller, buyer, params).unwrap_or(Score {
        score_milli: params.prior_mean_milli,
        confidence_bps: 0,
    });
    OverallScoreInfo {
        score_milli: score.score_milli as i64,
        score: score.stars(),
        confidence_bps: score.confidence_bps as i64,
    }
}

async fn load_role(
    conn: &mut myso_pg_db::Connection<'_>,
    subject_id: &str,
    role: i16,
) -> Result<Option<AggregateRow>, SocialError> {
    let row = reputation_aggregates::table
        .filter(reputation_aggregates::subject_id.eq(subject_id))
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
            reputation_aggregates::bayesian_score_milli,
            reputation_aggregates::last_receipt_id,
            reputation_aggregates::updated_at_ms,
        ))
        .first::<AggregateRow>(conn)
        .await
        .optional()?;
    Ok(row)
}

async fn load_credibility(
    conn: &mut myso_pg_db::Connection<'_>,
    account_id: &str,
) -> Result<Option<CredibilityInfo>, SocialError> {
    let row = credibility::table
        .filter(credibility::account_id.eq(account_id))
        .select((
            credibility::account_id,
            credibility::settled_count,
            credibility::volume_mist,
            credibility::credibility_bps,
        ))
        .first::<(String, i64, String, i64)>(conn)
        .await
        .optional()?;
    Ok(row.map(|(account_id, settled_count, volume_mist, credibility_bps)| CredibilityInfo {
        account_id,
        settled_count,
        volume_mist,
        credibility_bps,
    }))
}

/// An agent with no reviews yet is a normal case: the response is the prior, not a 404.
/// Returns `None` only when the agent itself is unknown.
pub(crate) async fn get_agent_reputation(
    db: &Db,
    agent_object_id: &str,
) -> Result<Option<AgentReputationInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let account_id = sub_agents::table
        .filter(sub_agents::agent_object_id.eq(agent_object_id))
        .select(sub_agents::account_id)
        .first::<String>(&mut conn)
        .await
        .optional()?;
    let Some(account_id) = account_id else {
        return Ok(None);
    };
    let config = task_config_history::table
        .order(task_config_history::id.desc())
        .select(TaskConfigInfo::as_select())
        .first(&mut conn)
        .await
        .optional()?;
    let params = scoring_params(config.as_ref());

    let (seller, seller_receipt, seller_at) =
        load_aggregate(load_role(&mut conn, agent_object_id, ROLE_SELLER).await?);
    let (buyer, buyer_receipt, buyer_at) =
        load_aggregate(load_role(&mut conn, agent_object_id, ROLE_BUYER).await?);
    let credibility = load_credibility(&mut conn, &account_id).await?;
    Ok(Some(AgentReputationInfo {
        agent_object_id: agent_object_id.to_string(),
        as_seller: score_info(&seller, seller_receipt, seller_at, &params),
        as_buyer: score_info(&buyer, buyer_receipt, buyer_at, &params),
        overall: overall_info(&seller, &buyer, &params),
        credibility,
    }))
}

/// A principal that buys directly. Always answers (prior and no credibility for an unknown
/// principal), because "never traded" and "unknown" are the same to a counterparty.
pub(crate) async fn get_principal_reputation(
    db: &Db,
    principal: &str,
) -> Result<PrincipalReputationInfo, SocialError> {
    let mut conn = db.connect().await?;
    let account_id = memory_accounts::table
        .filter(memory_accounts::principal_owner.eq(principal))
        .select(memory_accounts::account_id)
        .first::<String>(&mut conn)
        .await
        .optional()?;
    let config = task_config_history::table
        .order(task_config_history::id.desc())
        .select(TaskConfigInfo::as_select())
        .first(&mut conn)
        .await
        .optional()?;
    let params = scoring_params(config.as_ref());

    let (buyer, receipt, at, credibility) = match &account_id {
        Some(id) => {
            let (agg, receipt, at) = load_aggregate(load_role(&mut conn, id, ROLE_BUYER).await?);
            (agg, receipt, at, load_credibility(&mut conn, id).await?)
        }
        None => (Aggregate::default(), None, None, None),
    };
    Ok(PrincipalReputationInfo {
        principal: principal.to_string(),
        account_id,
        as_human_buyer: score_info(&buyer, receipt, at, &params),
        overall: overall_info(&Aggregate::default(), &buyer, &params),
        credibility,
    })
}
