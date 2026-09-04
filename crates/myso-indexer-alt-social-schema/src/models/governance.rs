// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use diesel::sql_types::{
    BigInt, Bool, Date, Integer, Jsonb, Nullable, SmallInt, Text, Timestamptz,
};
use diesel::QueryableByName;

type NullableJsonb = Nullable<Jsonb>;
use serde::{Deserialize, Serialize};

use crate::schema::{
    anonymous_votes, community_votes, delegate_ratings, delegate_votes, delegates,
    governance_events, governance_registries, nominated_delegates, proposals, reward_distributions,
    vote_decryption_failures,
};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = governance_registries)]
pub struct GovernanceRegistry {
    pub id: i32,
    pub registry_type: i16,
    pub delegate_count: i64,
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub max_votes_per_user: i64,
    pub quadratic_base_cost: i64,
    pub voting_period_ms: i64,
    pub quorum_votes: i64,
    pub last_delegate_panel_boundary_epoch: Option<i64>,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub registry_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = governance_registries)]
pub struct NewGovernanceRegistry {
    pub registry_type: i16,
    pub registry_id: String,
    pub delegate_count: i64,
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub max_votes_per_user: i64,
    pub quadratic_base_cost: i64,
    pub voting_period_ms: i64,
    pub quorum_votes: i64,
    pub last_delegate_panel_boundary_epoch: Option<i64>,
    pub updated_at: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = delegates)]
pub struct Delegate {
    pub id: i32,
    pub address: String,
    pub registry_type: i16,
    pub governance_registry_id: String,
    pub upvotes: i64,
    pub downvotes: i64,
    pub proposals_reviewed: i64,
    pub proposals_submitted: i64,
    pub sided_winning_proposals: i64,
    pub sided_losing_proposals: i64,
    pub term_start: i64,
    pub term_end: i64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegates)]
pub struct NewDelegate {
    pub address: String,
    pub registry_type: i16,
    pub governance_registry_id: String,
    pub upvotes: i64,
    pub downvotes: i64,
    pub proposals_reviewed: i64,
    pub proposals_submitted: i64,
    pub sided_winning_proposals: i64,
    pub sided_losing_proposals: i64,
    pub term_start: i64,
    pub term_end: i64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = nominated_delegates)]
pub struct NominatedDelegate {
    pub id: i32,
    pub address: String,
    pub registry_type: i16,
    pub upvotes: i64,
    pub downvotes: i64,
    pub scheduled_term_start_epoch: i64,
    pub nomination_time: i64,
    pub status: i16,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub governance_registry_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = nominated_delegates)]
pub struct NewNominatedDelegate {
    pub address: String,
    pub registry_type: i16,
    pub upvotes: i64,
    pub downvotes: i64,
    pub scheduled_term_start_epoch: i64,
    pub nomination_time: i64,
    pub status: i16,
    pub transaction_id: String,
    pub governance_registry_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = proposals)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: i16,
    pub reference_id: Option<String>,
    pub metadata_json: Option<serde_json::Value>,
    pub submitter: String,
    pub submission_time: i64,
    pub delegate_approval_count: i64,
    pub delegate_rejection_count: i64,
    pub community_votes_for: i64,
    pub community_votes_against: i64,
    pub status: i16,
    pub voting_start_time: Option<i64>,
    pub voting_end_time: Option<i64>,
    pub reward_pool: i64,
    pub implemented_description: Option<String>,
    pub implementation_time: Option<i64>,
    pub rescind_time: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub anonymous_votes_for: Option<i64>,
    pub anonymous_votes_against: Option<i64>,
    pub anonymous_voters_count: Option<i64>,
    pub pending_anonymous_decryption: Option<bool>,
    pub anonymous_decryption_completed_at: Option<i64>,
    pub rejection_time: Option<i64>,
    pub governance_registry_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = proposals)]
pub struct NewProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: i16,
    pub reference_id: Option<String>,
    pub metadata_json: Option<serde_json::Value>,
    pub submitter: String,
    pub submission_time: i64,
    pub delegate_approval_count: i64,
    pub delegate_rejection_count: i64,
    pub community_votes_for: i64,
    pub community_votes_against: i64,
    pub status: i16,
    pub voting_start_time: Option<i64>,
    pub voting_end_time: Option<i64>,
    pub reward_pool: i64,
    pub transaction_id: String,
    pub governance_registry_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = delegate_ratings)]
pub struct DelegateRating {
    pub id: i32,
    pub target_address: String,
    pub voter_address: String,
    pub registry_type: i16,
    pub is_active_delegate: bool,
    /// 0 = down, 1 = up, 2 = cleared (vote withdrawn).
    pub vote_kind: i16,
    pub rated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub governance_registry_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegate_ratings)]
pub struct NewDelegateRating {
    pub target_address: String,
    pub voter_address: String,
    pub registry_type: i16,
    pub governance_registry_id: String,
    pub is_active_delegate: bool,
    pub vote_kind: i16,
    pub rated_at: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = delegate_votes)]
pub struct DelegateVote {
    pub id: i32,
    pub proposal_id: String,
    pub delegate_address: String,
    pub approve: bool,
    pub vote_time: i64,
    pub reason: Option<String>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegate_votes)]
pub struct NewDelegateVote {
    pub proposal_id: String,
    pub delegate_address: String,
    pub approve: bool,
    pub vote_time: i64,
    pub reason: Option<String>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = community_votes)]
pub struct CommunityVote {
    pub id: i32,
    pub proposal_id: String,
    pub voter_address: String,
    pub vote_weight: i64,
    pub approve: bool,
    pub vote_time: i64,
    pub vote_cost: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = community_votes)]
pub struct NewCommunityVote {
    pub proposal_id: String,
    pub voter_address: String,
    pub vote_weight: i64,
    pub approve: bool,
    pub vote_time: i64,
    pub vote_cost: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = reward_distributions)]
pub struct RewardDistribution {
    pub id: i32,
    pub proposal_id: String,
    pub recipient_address: String,
    pub amount: i64,
    pub distribution_time: i64,
    pub distribution_type: Option<String>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = reward_distributions)]
pub struct NewRewardDistribution {
    pub proposal_id: String,
    pub recipient_address: String,
    pub amount: i64,
    pub distribution_time: i64,
    pub distribution_type: Option<String>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = governance_events)]
pub struct GovernanceEvent {
    pub id: i32,
    pub event_type: String,
    pub registry_type: i16,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub anonymous_voting_related: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub proposal_id: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = governance_events)]
pub struct NewGovernanceEvent {
    pub event_type: String,
    pub registry_type: i16,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub anonymous_voting_related: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub proposal_id: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = anonymous_votes)]
pub struct AnonymousVote {
    pub id: i32,
    pub proposal_id: String,
    pub voter_address: String,
    pub encrypted_vote_data: Vec<u8>,
    pub submitted_at: i64,
    pub decrypted: Option<bool>,
    pub decrypted_at: Option<i64>,
    pub decrypted_vote: Option<i16>,
    pub decryption_status: Option<i16>,
    pub decryption_error: Option<String>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: Option<bool>,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = anonymous_votes)]
pub struct NewAnonymousVote {
    pub proposal_id: String,
    pub voter_address: String,
    pub encrypted_vote_data: Vec<u8>,
    pub submitted_at: i64,
    pub decryption_status: i16,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = vote_decryption_failures)]
pub struct VoteDecryptionFailure {
    pub id: i32,
    pub proposal_id: String,
    pub voter_address: String,
    pub failure_reason: String,
    pub attempted_at: i64,
    pub encrypted_vote_length: Option<i32>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = vote_decryption_failures)]
pub struct NewVoteDecryptionFailure {
    pub proposal_id: String,
    pub voter_address: String,
    pub failure_reason: String,
    pub attempted_at: i64,
    pub encrypted_vote_length: Option<i32>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = proposals)]
pub struct ProposalUpdateSet {
    pub status: Option<i16>,
    pub voting_start_time: Option<i64>,
    pub voting_end_time: Option<i64>,
    pub reward_pool: Option<i64>,
    pub community_votes_for: Option<i64>,
    pub community_votes_against: Option<i64>,
    pub rescind_time: Option<i64>,
    pub rejection_time: Option<i64>,
    pub implementation_time: Option<i64>,
    pub implemented_description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GovernanceRegistryUpdate {
    pub registry_type: i16,
    /// On-chain GovernanceDAO object id. Set for platform registries so updates match the correct row.
    pub registry_id: Option<String>,
    pub delegate_count: i64,
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub max_votes_per_user: i64,
    pub quadratic_base_cost: i64,
    pub voting_period_ms: i64,
    pub quorum_votes: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone)]
pub struct GovernanceRegistryPanelBoundaryUpdate {
    pub registry_id: String,
    pub last_delegate_panel_boundary_epoch: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}

/// Config for a governance registry (voting params). Used by GraphQL and API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRegistryConfig {
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub max_votes_per_user: i64,
    pub quadratic_base_cost: i64,
    pub voting_period_ms: i64,
    pub quorum_votes: i64,
}

/// Query result for a proposal (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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
    #[diesel(sql_type = NullableJsonb)]
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
    pub rejection_time: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub anonymous_voters_count: Option<i64>,
    #[diesel(sql_type = Text)]
    pub governance_registry_id: String,
}

/// Query result for a delegate (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct DelegateRow {
    #[diesel(sql_type = Text)]
    pub address: String,
    #[diesel(sql_type = SmallInt)]
    pub registry_type: i16,
    #[diesel(sql_type = Text)]
    pub governance_registry_id: String,
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

/// Query result for governance_stats view (per on-chain registry instance).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct GovernanceStatsRow {
    #[diesel(sql_type = Text)]
    pub registry_id: String,
    #[diesel(sql_type = SmallInt)]
    pub registry_type: i16,
    #[diesel(sql_type = BigInt)]
    pub active_delegates: i64,
    #[diesel(sql_type = BigInt)]
    pub pending_nominees: i64,
    #[diesel(sql_type = BigInt)]
    pub submitted_proposals: i64,
    #[diesel(sql_type = BigInt)]
    pub in_review_proposals: i64,
    #[diesel(sql_type = BigInt)]
    pub voting_proposals: i64,
    #[diesel(sql_type = BigInt)]
    pub approved_proposals: i64,
    #[diesel(sql_type = BigInt)]
    pub rejected_proposals: i64,
    #[diesel(sql_type = BigInt)]
    pub implemented_proposals: i64,
    #[diesel(sql_type = BigInt)]
    pub rescinded_proposals: i64,
}

/// Query result for a governance registry (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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
    pub quadratic_base_cost: i64,
    #[diesel(sql_type = BigInt)]
    pub voting_period_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub quorum_votes: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub last_delegate_panel_boundary_epoch: Option<i64>,
}

/// Query result for a nominated delegate (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct NominatedDelegateRow {
    #[diesel(sql_type = Text)]
    pub address: String,
    #[diesel(sql_type = SmallInt)]
    pub registry_type: i16,
    #[diesel(sql_type = Text)]
    pub governance_registry_id: String,
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

/// Query result for a delegate rating (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct DelegateRatingRow {
    #[diesel(sql_type = Text)]
    pub target_address: String,
    #[diesel(sql_type = Text)]
    pub voter_address: String,
    #[diesel(sql_type = SmallInt)]
    pub registry_type: i16,
    #[diesel(sql_type = Text)]
    pub governance_registry_id: String,
    #[diesel(sql_type = Bool)]
    pub is_active_delegate: bool,
    #[diesel(sql_type = SmallInt)]
    pub vote_kind: i16,
    #[diesel(sql_type = BigInt)]
    pub rated_at: i64,
}

/// Query result for a delegate vote (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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

/// Query result for a community vote (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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

/// Query result for a reward distribution (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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

/// Query result for an anonymous vote (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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

/// Query result for a vote decryption failure (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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

/// Query result for anonymous voting stats (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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

/// Query result for anonymous voting trend (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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

/// Query result for a governance event (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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
    #[diesel(sql_type = Nullable<Text>)]
    pub governance_registry_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub proposal_id: Option<String>,
}
