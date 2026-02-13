// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    anonymous_votes, blocked_events, blocked_profiles, community_votes, delegate_ratings,
    delegate_votes, delegates, governance_events, governance_registries, nominated_delegates,
    profile_badges, profile_events, profiles, proposals, reward_distributions,
    social_graph_events, social_graph_relationships, vote_decryption_failures,
};

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub owner_address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub website: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub cover_photo: Option<String>,
    pub profile_id: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub post_count: i32,
    pub min_offer_amount: Option<i64>,
    pub birthdate: Option<String>,
    pub current_location: Option<String>,
    pub raised_location: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub political_view: Option<String>,
    pub religion: Option<String>,
    pub education: Option<String>,
    pub primary_language: Option<String>,
    pub relationship_status: Option<String>,
    pub x_username: Option<String>,
    pub facebook_username: Option<String>,
    pub reddit_username: Option<String>,
    pub github_username: Option<String>,
    pub instagram_username: Option<String>,
    pub linkedin_username: Option<String>,
    pub twitch_username: Option<String>,
    pub social_proof_token_address: Option<String>,
    pub reservation_pool_address: Option<String>,
    pub selected_badge_id: Option<String>,
    pub paid_messaging_enabled: bool,
    pub paid_messaging_min_cost: Option<i64>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_graph_relationships)]
pub struct NewSocialGraphRelationship {
    pub follower_address: String,
    pub following_address: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_graph_events)]
pub struct NewSocialGraphEvent {
    pub event_type: String,
    pub follower_address: String,
    pub following_address: String,
    pub created_at: NaiveDateTime,
    pub event_id: Option<String>,
    pub raw_event_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = blocked_events)]
pub struct NewBlockedEvent {
    pub event_id: Option<String>,
    pub event_type: String,
    pub blocker_address: String,
    pub blocked_address: Option<String>,
    pub raw_event_data: Option<serde_json::Value>,
    pub processed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct NewBlockedProfile {
    pub blocker_address: String,
    pub blocked_address: String,
    pub blocked_profile_id: Option<String>,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    pub first_blocked_at: NaiveDateTime,
    pub last_blocked_at: NaiveDateTime,
    pub total_block_count: i32,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_badges)]
pub struct NewProfileBadge {
    pub profile_id: String,
    pub badge_id: String,
    pub badge_name: String,
    pub badge_description: Option<String>,
    pub badge_media_url: Option<String>,
    pub badge_icon_url: Option<String>,
    pub platform_id: String,
    pub assigned_by: String,
    pub assigned_at: i64,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub revoked_by: Option<String>,
    pub badge_type: i16,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_events)]
pub struct NewProfileEvent {
    pub event_type: String,
    pub profile_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = profiles)]
pub struct ProfileUpdateSet {
    pub updated_at: NaiveDateTime,
    pub display_name: Option<Option<String>>,
    pub bio: Option<Option<String>>,
    pub profile_photo: Option<Option<String>>,
    pub cover_photo: Option<Option<String>>,
    pub birthdate: Option<Option<String>>,
    pub current_location: Option<Option<String>>,
    pub raised_location: Option<Option<String>>,
    pub phone: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub gender: Option<Option<String>>,
    pub political_view: Option<Option<String>>,
    pub religion: Option<Option<String>>,
    pub education: Option<Option<String>>,
    pub primary_language: Option<Option<String>>,
    pub relationship_status: Option<Option<String>>,
    pub x_username: Option<Option<String>>,
    pub facebook_username: Option<Option<String>>,
    pub reddit_username: Option<Option<String>>,
    pub github_username: Option<Option<String>>,
    pub instagram_username: Option<Option<String>>,
    pub linkedin_username: Option<Option<String>>,
    pub twitch_username: Option<Option<String>>,
    pub min_offer_amount: Option<Option<i64>>,
    pub username: Option<String>,
    pub selected_badge_id: Option<Option<String>>,
    pub paid_messaging_enabled: Option<bool>,
    pub paid_messaging_min_cost: Option<Option<i64>>,
}

// =============================================================================
// GOVERNANCE MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = governance_registries)]
pub struct NewGovernanceRegistry {
    pub registry_type: i16,
    pub registry_id: String,
    pub delegate_count: i64,
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub min_on_chain_age_days: i64,
    pub max_votes_per_user: i64,
    pub quadratic_base_cost: i64,
    pub voting_period_ms: i64,
    pub quorum_votes: i64,
    pub updated_at: i64,
    pub transaction_id: String,
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
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegates)]
pub struct NewDelegate {
    pub address: String,
    pub registry_type: i16,
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
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegate_ratings)]
pub struct NewDelegateRating {
    pub target_address: String,
    pub voter_address: String,
    pub registry_type: i16,
    pub is_active_delegate: bool,
    pub upvote: bool,
    pub rated_at: i64,
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

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = governance_events)]
pub struct NewGovernanceEvent {
    pub event_type: String,
    pub registry_type: i16,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub anonymous_voting_related: Option<bool>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = anonymous_votes)]
pub struct NewAnonymousVote {
    pub proposal_id: String,
    pub voter_address: String,
    pub encrypted_vote_data: Option<Vec<u8>>,
    pub submitted_at: i64,
    pub decryption_status: i16,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
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
    pub implementation_time: Option<i64>,
    pub implemented_description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GovernanceRegistryUpdate {
    pub registry_type: i16,
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
