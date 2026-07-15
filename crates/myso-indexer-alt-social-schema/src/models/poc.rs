// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::Jsonb;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text, Timestamptz};
use serde::{Deserialize, Serialize};

use crate::schema::{
    poc_analysis_results, poc_badges, poc_config, poc_dispute_votes, poc_disputes,
    poc_revenue_redirections, poc_vault_claims, poc_vault_deposits,
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
    #[diesel(sql_type = Nullable<Text>)]
    pub original_creator: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub analysis_timestamp: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub reasoning: Option<String>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub evidence_urls: Option<serde_json::Value>,
}

/// Query result for POC revenue redirection.
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocRevenueRedirectionRow {
    #[diesel(sql_type = Text)]
    pub redirection_id: String,
    #[diesel(sql_type = Text)]
    pub accused_post_id: String,
    #[diesel(sql_type = Text)]
    pub original_post_id: String,
    #[diesel(sql_type = BigInt)]
    pub redirect_percentage: i64,
    #[diesel(sql_type = BigInt)]
    pub similarity_score: i64,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<Bool>)]
    pub removed: Option<bool>,
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
    #[diesel(sql_type = Nullable<Text>)]
    pub beneficiary_address: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub matched_anchor_id: Option<String>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub media_index: Option<i16>,
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
    pub voting_start_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub voting_end_ms: i64,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub winning_side: Option<i16>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub total_winning_stake: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub total_losing_stake: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub submitted_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub resolved_at: Option<i64>,
    #[diesel(sql_type = SmallInt)]
    pub dispute_round: i16,
    #[diesel(sql_type = BigInt)]
    pub effective_dispute_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub required_total_stake_quorum: i64,
    #[diesel(sql_type = Nullable<Bool>)]
    pub quorum_met: Option<bool>,
}

/// One vote on a PoC dispute (latest row per dispute_id + voter).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocDisputeVoteRow {
    #[diesel(sql_type = Text)]
    pub dispute_id: String,
    #[diesel(sql_type = Text)]
    pub voter: String,
    #[diesel(sql_type = SmallInt)]
    pub vote_choice: i16,
    #[diesel(sql_type = BigInt)]
    pub stake_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub voted_at: i64,
    #[diesel(sql_type = Nullable<Bool>)]
    pub reward_claimed: Option<bool>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub reward_amount: Option<i64>,
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
    #[diesel(sql_type = BigInt)]
    pub min_vote_stake: i64,
    #[diesel(sql_type = BigInt)]
    pub max_vote_stake: i64,
    #[diesel(sql_type = BigInt)]
    pub voting_duration_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub max_reasoning_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_evidence_urls: i64,
    #[diesel(sql_type = BigInt)]
    pub max_votes_per_dispute: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub dispute_governance_registry_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub oracle_address: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub claim_treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_referral_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub video_embedded_audio_redirect_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub dispute_quorum_base_stake: i64,
    #[diesel(sql_type = BigInt)]
    pub dispute_second_round_fee_multiplier_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub dispute_second_round_quorum_multiplier_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub username_beneficiary_join_referral_bps: i64,
    #[diesel(sql_type = SmallInt)]
    pub max_disputes_per_post: i16,
    #[diesel(sql_type = BigInt)]
    pub min_vault_deposit_amount: i64,
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_badges)]
pub struct NewPocBadge {
    pub badge_id: String,
    pub post_id: String,
    pub media_type: i16,
    pub issued_by: String,
    pub beneficiary_address: Option<String>,
    pub matched_anchor_id: Option<String>,
    pub media_index: Option<i16>,
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
    pub voting_start_ms: i64,
    pub voting_end_ms: i64,
    pub resolution: Option<i16>,
    pub winning_side: Option<i16>,
    pub total_winning_stake: Option<i64>,
    pub total_losing_stake: Option<i64>,
    pub submitted_at: i64,
    pub resolved_at: Option<i64>,
    pub transaction_id: String,
    pub dispute_round: i16,
    pub effective_dispute_fee: i64,
    pub required_total_stake_quorum: i64,
    pub quorum_met: Option<bool>,
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
#[diesel(table_name = poc_config)]
pub struct NewPocConfiguration {
    pub image_threshold: i64,
    pub video_threshold: i64,
    pub audio_threshold: i64,
    pub revenue_redirect_percentage: i64,
    pub dispute_cost: i64,
    pub min_vote_stake: i64,
    pub max_vote_stake: i64,
    pub voting_duration_ms: i64,
    pub max_reasoning_length: i64,
    pub max_evidence_urls: i64,
    pub max_votes_per_dispute: i64,
    pub dispute_governance_registry_id: Option<String>,
    pub oracle_address: Option<String>,
    pub claim_treasury_fee_bps: i64,
    pub max_referral_bps: i64,
    pub video_embedded_audio_redirect_bps: i64,
    pub dispute_quorum_base_stake: i64,
    pub dispute_second_round_fee_multiplier_bps: i64,
    pub dispute_second_round_quorum_multiplier_bps: i64,
    pub username_beneficiary_join_referral_bps: i64,
    pub max_disputes_per_post: i16,
    pub min_vault_deposit_amount: i64,
    pub updated_by: String,
    pub updated_at: i64,
    pub transaction_id: String,
    pub version: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_vault_deposits)]
pub struct NewPocVaultDeposit {
    pub vault_id: String,
    pub vault_routing_key: String,
    pub amount: i64,
    pub coin_type: String,
    pub source_post_id: Option<String>,
    pub occurred_at_ms: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_vault_claims)]
pub struct NewPocVaultClaim {
    pub vault_id: String,
    pub vault_routing_key: String,
    pub coin_type: String,
    pub referrer_address: Option<String>,
    pub treasury_amount: i64,
    pub referrer_amount: i64,
    pub beneficiary_amount: i64,
    pub occurred_at_ms: i64,
    pub transaction_id: String,
    pub claim_kind: Option<String>,
    pub gross_amount: i64,
}

/// Sentinel `coin_type` in `poc_vault_coin_balances` seeded from legacy single-balance column.
pub const POC_VAULT_LEGACY_AGGREGATE_COIN_TYPE: &str = "__legacy_aggregate__";

/// Latest metadata row for a PoC beneficiary vault object.
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocBeneficiaryVaultRow {
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub vault_routing_key: String,
    #[diesel(sql_type = BigInt)]
    pub updated_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// One deposit into a PoC beneficiary vault (append-only log).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocVaultDepositRow {
    #[diesel(sql_type = BigInt)]
    pub id: i64,
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub vault_routing_key: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = Text)]
    pub coin_type: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub source_post_id: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub occurred_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// One claim from a PoC beneficiary vault (append-only log).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocVaultClaimRow {
    #[diesel(sql_type = BigInt)]
    pub id: i64,
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub vault_routing_key: String,
    #[diesel(sql_type = Text)]
    pub coin_type: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub referrer_address: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub treasury_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub referrer_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub beneficiary_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub occurred_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub claim_kind: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub gross_amount: i64,
}

/// One `(vault_id, coin_type)` balance row from `poc_vault_coin_balances`.
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PocVaultCoinBalanceRow {
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub coin_type: String,
    #[diesel(sql_type = BigInt)]
    pub balance: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at_ms: i64,
}
