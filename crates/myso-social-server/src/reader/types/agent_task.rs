// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Nullable, Text};
use myso_indexer_alt_social_schema::schema::{
    agent_tasks, task_config_history, task_receipts, task_reviews,
};
use serde::Serialize;

/// `GET /tasks` and `GET /tasks/:task_id`.
#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = agent_tasks)]
#[serde(rename_all = "camelCase")]
pub struct AgentTaskInfo {
    pub task_id: String,
    pub buyer_principal: String,
    pub buyer_agent_id: Option<String>,
    pub buyer_memory_account_id: String,
    pub seller_principal: String,
    pub seller_agent_id: String,
    pub seller_memory_account_id: String,
    pub organization_id: Option<String>,
    pub room_group_id: Option<String>,
    pub platform_id: Option<String>,
    pub title: String,
    pub scope_digest: String,
    pub amount_mist: i64,
    pub accept_window_ms: i64,
    pub delivery_window_ms: i64,
    pub settle_window_ms: i64,
    pub platform_fee_bps: i64,
    pub ecosystem_fee_bps: i64,
    pub terms_digest: String,
    pub status: i16,
    pub close_kind: Option<i16>,
    pub settlement_kind: Option<i16>,
    pub refund_kind: Option<i16>,
    pub created_at_ms: i64,
    pub accept_deadline_ms: i64,
    pub accepted_at_ms: Option<i64>,
    pub delivery_deadline_ms: Option<i64>,
    pub delivered_at_ms: Option<i64>,
    pub settle_deadline_ms: Option<i64>,
    pub settled_at_ms: Option<i64>,
    pub refunded_at_ms: Option<i64>,
    pub cancelled_at_ms: Option<i64>,
    pub review_deadline_ms: Option<i64>,
    pub deliverable_digest: Option<String>,
    pub deliverable_uri: Option<String>,
    pub platform_fee_mist: Option<i64>,
    pub ecosystem_fee_mist: Option<i64>,
    pub net_amount_mist: Option<i64>,
    pub refund_proposer: i16,
    pub refund_proposed_at_ms: Option<i64>,
    pub buyer_credibility_bps: Option<i64>,
    pub seller_credibility_bps: Option<i64>,
    pub buyer_review_stars: Option<i16>,
    pub seller_review_stars: Option<i16>,
    pub transaction_id: String,
}

/// `GET /agents/:id/receipts`.
#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = task_receipts)]
#[serde(rename_all = "camelCase")]
pub struct TaskReceiptInfo {
    pub task_id: String,
    pub receipt_id: String,
    pub buyer_principal: String,
    pub buyer_agent_id: Option<String>,
    pub seller_principal: String,
    pub seller_agent_id: String,
    pub organization_id: Option<String>,
    pub room_group_id: Option<String>,
    pub settlement_kind: i16,
    pub amount_mist: i64,
    pub platform_fee_mist: i64,
    pub ecosystem_fee_mist: i64,
    pub net_amount_mist: i64,
    pub deliverable_digest: Option<String>,
    pub deliverable_uri: Option<String>,
    pub buyer_credibility_bps: i64,
    pub seller_credibility_bps: i64,
    pub settle_deadline_ms: i64,
    pub review_deadline_ms: i64,
    pub settled_at_ms: i64,
    pub transaction_id: String,
}

/// One indexed task lifecycle event. Duplicate per-party outbox copies are collapsed by
/// `(event_type, source_event_id)`; status is the indexed task snapshot at that transition.
#[derive(Debug, Clone, Serialize, diesel::QueryableByName)]
#[serde(rename_all = "camelCase")]
pub struct TaskHistoryInfo {
    #[diesel(sql_type = Text)]
    pub event_type: String,
    #[diesel(sql_type = BigInt)]
    pub occurred_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub source_event_id: String,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub status: Option<i64>,
}

/// Reviews on one task (at most one per direction).
#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = task_reviews)]
#[serde(rename_all = "camelCase")]
pub struct TaskReviewInfo {
    pub task_id: String,
    pub rater_role: i16,
    pub rater_principal: String,
    pub rater_agent_id: Option<String>,
    pub reviewee_principal: String,
    pub reviewee_agent_id: Option<String>,
    pub stars: i16,
    pub rater_credibility_bps: i64,
    pub value_weight_bps: i64,
    pub base_weight_bps: i64,
    pub prior_pair_reviews: i64,
    pub weight_bps: i64,
    pub reviewed_at_ms: i64,
    pub transaction_id: String,
}

/// `GET /task/config`: the latest `TaskConfigUpdated`.
#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(table_name = task_config_history)]
#[serde(rename_all = "camelCase")]
pub struct TaskConfigInfo {
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
}

/// A task with both reviews, for the detail pane.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDetailInfo {
    #[serde(flatten)]
    pub task: AgentTaskInfo,
    pub reviews: Vec<TaskReviewInfo>,
    /// Present once the task has gone to a jury; the full case is at `/tasks/:task_id/dispute`.
    pub dispute: Option<super::jury::TaskDisputeInfo>,
}

/// Scores for one subject and role, derived from the indexed sums with the current prior.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReputationScoreInfo {
    pub review_count: i64,
    /// Counts of 1 through 5 star reviews.
    pub histogram: [i64; 5],
    pub score_milli: i64,
    /// `score_milli / 1000`, for display.
    pub score: f64,
    pub confidence_bps: i64,
    pub last_receipt_id: Option<String>,
    pub updated_at_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverallScoreInfo {
    pub score_milli: i64,
    pub score: f64,
    pub confidence_bps: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredibilityInfo {
    pub account_id: String,
    pub settled_count: i64,
    pub volume_mist: String,
    pub credibility_bps: i64,
}

/// `GET /agents/:id/reputation`: the agent's own scores; credibility is its principal's.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentReputationInfo {
    pub agent_object_id: String,
    pub as_seller: ReputationScoreInfo,
    pub as_buyer: ReputationScoreInfo,
    pub overall: OverallScoreInfo,
    pub credibility: Option<CredibilityInfo>,
}

/// `GET /principals/:address/reputation`: a principal buying directly (no agent).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrincipalReputationInfo {
    pub principal: String,
    pub account_id: Option<String>,
    pub as_human_buyer: ReputationScoreInfo,
    pub overall: OverallScoreInfo,
    pub credibility: Option<CredibilityInfo>,
}

/// `GET /task/stats`: market-wide counts and volume, for the Task Market overview.
#[derive(Debug, Clone, Serialize, QueryableByName, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatsInfo {
    /// Offered, waiting for the seller to accept.
    #[diesel(sql_type = BigInt)]
    pub open_offers: i64,
    /// Accepted, work in progress.
    #[diesel(sql_type = BigInt)]
    pub accepted: i64,
    #[diesel(sql_type = BigInt)]
    pub delivered: i64,
    #[diesel(sql_type = BigInt)]
    pub disputed: i64,
    #[diesel(sql_type = BigInt)]
    pub settled: i64,
    #[diesel(sql_type = BigInt)]
    pub refunded: i64,
    #[diesel(sql_type = BigInt)]
    pub cancelled: i64,
    /// MIST currently held in escrow (offered, accepted, delivered or disputed tasks).
    #[diesel(sql_type = BigInt)]
    pub total_escrowed_mist: i64,
    /// MIST paid out across every settled task, before fees.
    #[diesel(sql_type = BigInt)]
    pub total_settled_mist: i64,
    #[diesel(sql_type = BigInt)]
    pub settled_last_24h: i64,
    /// Seller agents with open work or a settlement in the last 24 hours.
    #[diesel(sql_type = BigInt)]
    pub active_agents: i64,
}

/// `GET /agents/top`: a seller agent ranked by its on-chain reputation.
#[derive(Debug, Clone, Serialize, QueryableByName, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TopAgentInfo {
    /// MemoryAccount object required when creating a task offer for this seller.
    #[diesel(sql_type = Text)]
    pub seller_account_id: String,
    #[diesel(sql_type = Text)]
    pub agent_object_id: String,
    /// Indexed capability bitset; current chain state remains authoritative.
    #[diesel(sql_type = BigInt)]
    pub capabilities: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub expires_at_ms: Option<i64>,
    #[diesel(sql_type = Text)]
    pub label: String,
    #[diesel(sql_type = Text)]
    pub principal: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub organization_id: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub review_count: i64,
    #[diesel(sql_type = BigInt)]
    pub score_milli: i64,
    /// `score_milli / 1000`, for display.
    #[diesel(sql_type = Double)]
    pub score: f64,
    #[diesel(sql_type = BigInt)]
    pub confidence_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub settled_count: i64,
    #[diesel(sql_type = Text)]
    pub volume_mist: String,
    #[diesel(sql_type = BigInt)]
    pub credibility_bps: i64,
}
