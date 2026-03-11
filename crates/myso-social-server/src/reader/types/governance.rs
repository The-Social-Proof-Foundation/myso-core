// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{
    BigInt, Bool, Date, Integer, Jsonb, Nullable, SmallInt, Text, Timestamptz,
};
use diesel::QueryableByName;
use serde::Serialize;

#[derive(Debug, Serialize, QueryableByName)]
pub struct ProposalRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub title: String,
    #[diesel(sql_type = Text)]
    pub description: String,
    #[diesel(sql_type = SmallInt)]
    pub proposal_type: i16,
    #[diesel(sql_type = Nullable<Text>)]
    pub reference_id: Option<String>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub metadata_json: Option<serde_json::Value>,
    #[diesel(sql_type = Text)]
    pub submitter: String,
    #[diesel(sql_type = BigInt)]
    pub submission_time: i64,
    #[diesel(sql_type = BigInt)]
    pub delegate_approval_count: i64,
    #[diesel(sql_type = BigInt)]
    pub delegate_rejection_count: i64,
    #[diesel(sql_type = BigInt)]
    pub community_votes_for: i64,
    #[diesel(sql_type = BigInt)]
    pub community_votes_against: i64,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub voting_start_time: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub voting_end_time: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub reward_pool: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub implemented_description: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub implementation_time: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub rescind_time: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub anonymous_voters_count: Option<i64>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct DelegateRow {
    #[diesel(sql_type = Text)]
    pub address: String,
    #[diesel(sql_type = SmallInt)]
    pub registry_type: i16,
    #[diesel(sql_type = BigInt)]
    pub upvotes: i64,
    #[diesel(sql_type = BigInt)]
    pub downvotes: i64,
    #[diesel(sql_type = BigInt)]
    pub proposals_reviewed: i64,
    #[diesel(sql_type = BigInt)]
    pub proposals_submitted: i64,
    #[diesel(sql_type = BigInt)]
    pub sided_winning_proposals: i64,
    #[diesel(sql_type = BigInt)]
    pub sided_losing_proposals: i64,
    #[diesel(sql_type = BigInt)]
    pub term_start: i64,
    #[diesel(sql_type = BigInt)]
    pub term_end: i64,
    #[diesel(sql_type = Bool)]
    pub is_active: bool,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct NominatedDelegateRow {
    #[diesel(sql_type = Text)]
    pub address: String,
    #[diesel(sql_type = SmallInt)]
    pub registry_type: i16,
    #[diesel(sql_type = BigInt)]
    pub upvotes: i64,
    #[diesel(sql_type = BigInt)]
    pub downvotes: i64,
    #[diesel(sql_type = BigInt)]
    pub scheduled_term_start_epoch: i64,
    #[diesel(sql_type = BigInt)]
    pub nomination_time: i64,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct GovernanceRegistryRow {
    #[diesel(sql_type = SmallInt)]
    pub registry_type: i16,
    #[diesel(sql_type = Text)]
    pub registry_id: String,
    #[diesel(sql_type = BigInt)]
    pub delegate_count: i64,
    #[diesel(sql_type = BigInt)]
    pub delegate_term_epochs: i64,
    #[diesel(sql_type = BigInt)]
    pub proposal_submission_cost: i64,
    #[diesel(sql_type = BigInt)]
    pub max_votes_per_user: i64,
    #[diesel(sql_type = BigInt)]
    pub voting_period_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub quorum_votes: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct DelegateRatingRow {
    #[diesel(sql_type = Text)]
    pub target_address: String,
    #[diesel(sql_type = Text)]
    pub voter_address: String,
    #[diesel(sql_type = SmallInt)]
    pub registry_type: i16,
    #[diesel(sql_type = Bool)]
    pub is_active_delegate: bool,
    #[diesel(sql_type = Bool)]
    pub upvote: bool,
    #[diesel(sql_type = BigInt)]
    pub rated_at: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct DelegateVoteRow {
    #[diesel(sql_type = Text)]
    pub proposal_id: String,
    #[diesel(sql_type = Text)]
    pub delegate_address: String,
    #[diesel(sql_type = Bool)]
    pub approve: bool,
    #[diesel(sql_type = BigInt)]
    pub vote_time: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct CommunityVoteRow {
    #[diesel(sql_type = Text)]
    pub proposal_id: String,
    #[diesel(sql_type = Text)]
    pub voter_address: String,
    #[diesel(sql_type = BigInt)]
    pub vote_weight: i64,
    #[diesel(sql_type = Bool)]
    pub approve: bool,
    #[diesel(sql_type = BigInt)]
    pub vote_time: i64,
    #[diesel(sql_type = BigInt)]
    pub vote_cost: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct RewardDistributionRow {
    #[diesel(sql_type = Text)]
    pub proposal_id: String,
    #[diesel(sql_type = Text)]
    pub recipient_address: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub distribution_time: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub distribution_type: Option<String>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct AnonymousVoteRow {
    #[diesel(sql_type = Text)]
    pub proposal_id: String,
    #[diesel(sql_type = Text)]
    pub voter_address: String,
    #[diesel(sql_type = BigInt)]
    pub submitted_at: i64,
    #[diesel(sql_type = SmallInt)]
    pub decryption_status: i16,
    #[diesel(sql_type = Bool)]
    pub processing_success: bool,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct VoteDecryptionFailureRow {
    #[diesel(sql_type = Text)]
    pub proposal_id: String,
    #[diesel(sql_type = Text)]
    pub voter_address: String,
    #[diesel(sql_type = Text)]
    pub failure_reason: String,
    #[diesel(sql_type = BigInt)]
    pub attempted_at: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct AnonymousVotingStatsRow {
    #[diesel(sql_type = BigInt)]
    pub total_anonymous_votes: i64,
    #[diesel(sql_type = BigInt)]
    pub successfully_decrypted: i64,
    #[diesel(sql_type = BigInt)]
    pub failed_decryptions: i64,
    #[diesel(sql_type = BigInt)]
    pub anonymous_votes_for: i64,
    #[diesel(sql_type = BigInt)]
    pub anonymous_votes_against: i64,
    #[diesel(sql_type = BigInt)]
    pub pending_decryption: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct AnonymousVotingTrendRow {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = BigInt)]
    pub total_votes: i64,
    #[diesel(sql_type = BigInt)]
    pub successful_decryptions: i64,
    #[diesel(sql_type = BigInt)]
    pub failed_decryptions: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct GovernanceEventRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub event_type: String,
    #[diesel(sql_type = SmallInt)]
    pub registry_type: i16,
    #[diesel(sql_type = Jsonb)]
    pub event_data: serde_json::Value,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
