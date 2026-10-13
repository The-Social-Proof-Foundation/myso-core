// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    agent_tasks, credibility, reputation_aggregates, task_config_history, task_disputes,
    task_receipts, task_reviews,
};

pub const TASK_STATUS_OFFERED: i16 = 0;
pub const TASK_STATUS_ACCEPTED: i16 = 1;
pub const TASK_STATUS_DELIVERED: i16 = 2;
pub const TASK_STATUS_SETTLED: i16 = 3;
pub const TASK_STATUS_REFUNDED: i16 = 4;
pub const TASK_STATUS_CANCELLED: i16 = 5;
/// Escrow frozen while a jury (or the settlement window after a failed jury) decides.
pub const TASK_STATUS_DISPUTED: i16 = 6;

pub const DISPUTE_STATE_OPEN: i16 = 1;
pub const DISPUTE_STATE_CANCELLED: i16 = 2;
pub const DISPUTE_STATE_RESOLVED: i16 = 3;

/// Inserted by `TaskOffered`; later events update the row in order.
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = agent_tasks)]
pub struct NewAgentTask {
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
    pub created_at_ms: i64,
    pub accept_deadline_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

/// Inserted by `TaskDisputed`; later dispute events update it.
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = task_disputes)]
pub struct NewTaskDispute {
    pub task_id: String,
    pub case_id: String,
    pub opened_by_role: i16,
    pub state: i16,
    pub fee_each_mist: i64,
    pub buyer_paid_mist: i64,
    pub seller_paid_mist: i64,
    pub opened_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = task_receipts)]
pub struct NewTaskReceipt {
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
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = task_reviews)]
pub struct NewTaskReview {
    pub task_id: String,
    pub rater_role: i16,
    pub receipt_id: String,
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
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = task_config_history)]
pub struct NewTaskConfig {
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
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

/// One row per (subject, role). The sums are u128 on-chain, stored as decimal strings.
#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = reputation_aggregates)]
pub struct NewReputationAggregate {
    pub subject_id: String,
    pub role: i16,
    pub subject_kind: i16,
    pub principal_owner: String,
    pub organization_id: Option<String>,
    pub review_count: i64,
    pub star_1: i64,
    pub star_2: i64,
    pub star_3: i64,
    pub star_4: i64,
    pub star_5: i64,
    pub weight_sum_bps: String,
    pub weighted_star_sum: String,
    pub bayesian_score_milli: i64,
    pub confidence_bps: i64,
    pub last_receipt_id: String,
    pub updated_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = credibility)]
pub struct NewCredibility {
    pub account_id: String,
    pub principal_owner: String,
    pub settled_count: i64,
    pub volume_mist: String,
    pub credibility_bps: i64,
    pub updated_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}
