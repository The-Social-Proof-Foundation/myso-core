// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text};
use serde::{Deserialize, Serialize};

use crate::schema::{
    poc_analysis_results, poc_badges, poc_configuration, poc_dispute_votes, poc_disputes,
    poc_revenue_redirections,
};

pub const MEDIA_TYPE_IMAGE: i16 = 1;
pub const MEDIA_TYPE_VIDEO: i16 = 2;
pub const MEDIA_TYPE_AUDIO: i16 = 3;
pub const DISPUTE_STATUS_VOTING: i16 = 1;
pub const DISPUTE_STATUS_RESOLVED_UPHELD: i16 = 2;
pub const DISPUTE_STATUS_RESOLVED_OVERTURNED: i16 = 3;
pub const VOTE_UPHOLD: i16 = 1;
pub const VOTE_OVERTURN: i16 = 2;

/// Query result for POC analysis (latest per post).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocAnalysisResultRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Bool)]
    pub similarity_detected: bool,
    #[diesel(sql_type = BigInt)]
    pub highest_similarity_score: i64,
    #[diesel(sql_type = SmallInt)]
    pub media_type: i16,
    #[diesel(sql_type = Text)]
    pub oracle_address: String,
    #[diesel(sql_type = BigInt)]
    pub analysis_timestamp: i64,
}

/// Query result for POC badge (non-revoked badges for a post).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocBadgeRow {
    #[diesel(sql_type = Text)]
    pub badge_id: String,
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = SmallInt)]
    pub media_type: i16,
    #[diesel(sql_type = Text)]
    pub issued_by: String,
    #[diesel(sql_type = BigInt)]
    pub issued_at: i64,
    #[diesel(sql_type = Bool)]
    pub revoked: bool,
}

/// Query result for POC dispute.
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocDisputeRow {
    #[diesel(sql_type = Text)]
    pub dispute_id: String,
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub disputer: String,
    #[diesel(sql_type = SmallInt)]
    pub dispute_type: i16,
    #[diesel(sql_type = Text)]
    pub evidence: String,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub resolution: Option<i16>,
    #[diesel(sql_type = BigInt)]
    pub stake_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub submitted_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub resolved_at: Option<i64>,
}

/// Query result for latest POC configuration.
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocConfigRow {
    #[diesel(sql_type = BigInt)]
    pub image_threshold: i64,
    #[diesel(sql_type = BigInt)]
    pub video_threshold: i64,
    #[diesel(sql_type = BigInt)]
    pub audio_threshold: i64,
    #[diesel(sql_type = BigInt)]
    pub revenue_redirect_percentage: i64,
    #[diesel(sql_type = BigInt)]
    pub dispute_cost: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub oracle_address: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_badges)]
pub struct NewPocBadge {
    pub badge_id: String,
    pub post_id: String,
    pub media_type: i16,
    pub issued_by: String,
    pub issued_at: i64,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_revenue_redirections)]
pub struct NewPocRevenueRedirection {
    pub redirection_id: String,
    pub accused_post_id: String,
    pub original_post_id: String,
    pub redirect_percentage: i64,
    pub similarity_score: i64,
    pub created_at: i64,
    pub removed: bool,
    pub removed_at: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_analysis_results)]
pub struct NewPocAnalysisResult {
    pub post_id: String,
    pub media_type: i16,
    pub similarity_detected: bool,
    pub highest_similarity_score: i64,
    pub oracle_address: String,
    pub original_creator: Option<String>,
    pub analysis_timestamp: i64,
    pub transaction_id: String,
    pub reasoning: Option<String>,
    pub evidence_urls: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_disputes)]
pub struct NewPocDispute {
    pub dispute_id: String,
    pub post_id: String,
    pub disputer: String,
    pub dispute_type: i16,
    pub evidence: String,
    pub status: i16,
    pub stake_amount: i64,
    pub voting_start_epoch: i64,
    pub voting_end_epoch: i64,
    pub resolution: Option<i16>,
    pub winning_side: Option<i16>,
    pub total_winning_stake: Option<i64>,
    pub total_losing_stake: Option<i64>,
    pub submitted_at: i64,
    pub resolved_at: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_dispute_votes)]
pub struct NewPocDisputeVote {
    pub dispute_id: String,
    pub voter: String,
    pub vote_choice: i16,
    pub stake_amount: i64,
    pub voted_at: i64,
    pub reward_claimed: bool,
    pub reward_amount: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_configuration)]
pub struct NewPocConfiguration {
    pub image_threshold: i64,
    pub video_threshold: i64,
    pub audio_threshold: i64,
    pub revenue_redirect_percentage: i64,
    pub dispute_cost: i64,
    pub dispute_protocol_fee: i64,
    pub min_vote_stake: i64,
    pub max_vote_stake: i64,
    pub voting_duration_epochs: i64,
    pub max_reasoning_length: i64,
    pub max_evidence_urls: i64,
    pub max_votes_per_dispute: i64,
    pub oracle_address: Option<String>,
    pub updated_by: String,
    pub updated_at: i64,
    pub transaction_id: String,
}
