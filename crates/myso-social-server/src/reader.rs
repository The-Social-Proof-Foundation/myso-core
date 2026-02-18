// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{
    BigInt, Bool, Date, Integer, Jsonb, Nullable, SmallInt, Text, Timestamp, Timestamptz,
};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::Profile;
use myso_indexer_alt_social_schema::schema::profiles;
use myso_pg_db::{Db, DbArgs};
use serde::Serialize;
use url::Url;

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataBasic {
    #[diesel(sql_type = Text)]
    pub mydata_id: String,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = Text)]
    pub media_type: String,
    #[diesel(sql_type = Jsonb)]
    pub tags: serde_json::Value,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_id: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub timestamp_start: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub timestamp_end: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = BigInt)]
    pub last_updated: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub one_time_price: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub subscription_price: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub subscription_duration_days: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub geographic_region: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub data_quality: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub sample_size: Option<i64>,
    #[diesel(sql_type = Bool)]
    pub is_updating: bool,
    #[diesel(sql_type = Nullable<Text>)]
    pub update_frequency: Option<String>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub enable_flag: bool,
    #[diesel(sql_type = BigInt)]
    pub max_tags: i64,
    #[diesel(sql_type = BigInt)]
    pub max_subscription_days: i64,
    #[diesel(sql_type = BigInt)]
    pub max_free_access_grants: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct PurchaseInfo {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub mydata_id: String,
    #[diesel(sql_type = Text)]
    pub buyer: String,
    #[diesel(sql_type = BigInt)]
    pub price: i64,
    #[diesel(sql_type = Text)]
    pub purchase_type: String,
    #[diesel(sql_type = BigInt)]
    pub purchase_time: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SubscriptionInfo {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub mydata_id: String,
    #[diesel(sql_type = Text)]
    pub subscriber: String,
    #[diesel(sql_type = BigInt)]
    pub subscription_start: i64,
    #[diesel(sql_type = BigInt)]
    pub subscription_end: i64,
    #[diesel(sql_type = BigInt)]
    pub price: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct RevenueInfo {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub mydata_id: String,
    #[diesel(sql_type = Text)]
    pub from_address: String,
    #[diesel(sql_type = Text)]
    pub to_address: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = Text)]
    pub revenue_type: String,
    #[diesel(sql_type = BigInt)]
    pub revenue_time: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct AccessLogInfo {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub mydata_id: String,
    #[diesel(sql_type = Text)]
    pub user_address: String,
    #[diesel(sql_type = Text)]
    pub access_type: String,
    #[diesel(sql_type = BigInt)]
    pub access_time: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataStatsResponse {
    #[diesel(sql_type = Text)]
    pub mydata_id: String,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = Text)]
    pub media_type: String,
    #[diesel(sql_type = BigInt)]
    pub total_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub purchase_count: i64,
    #[diesel(sql_type = BigInt)]
    pub subscription_count: i64,
    #[diesel(sql_type = BigInt)]
    pub access_count: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub one_time_price: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub subscription_price: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = BigInt)]
    pub last_updated: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct DailyRevenue {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = BigInt)]
    pub daily_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub daily_transactions: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct AccessAnalytics {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = Text)]
    pub access_type: String,
    #[diesel(sql_type = BigInt)]
    pub unique_users: i64,
    #[diesel(sql_type = BigInt)]
    pub total_accesses: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub enable_flag: bool,
    #[diesel(sql_type = BigInt)]
    pub min_coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_duration_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceVaultInfo {
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub underwriter: String,
    #[diesel(sql_type = BigInt)]
    pub capital_balance: i64,
    #[diesel(sql_type = BigInt)]
    pub reserved: i64,
    #[diesel(sql_type = BigInt)]
    pub base_rate_bps_per_day: i64,
    #[diesel(sql_type = BigInt)]
    pub utilization_multiplier_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_exposure_per_market: i64,
    #[diesel(sql_type = BigInt)]
    pub max_exposure_per_user: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = Timestamp)]
    pub created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceVaultRow {
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub underwriter: String,
    #[diesel(sql_type = BigInt)]
    pub capital_balance: i64,
    #[diesel(sql_type = BigInt)]
    pub reserved: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceVaultTransactionRow {
    #[diesel(sql_type = Text)]
    pub transaction_type: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub balance_after: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsuranceVaultExposureRow {
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub total_exposure: i64,
}

#[derive(Debug, Serialize)]
pub struct SpotRecordResponse {
    pub post_id: String,
    pub status: i16,
    pub outcome: Option<i16>,
    pub betting_options: Vec<String>,
    pub option_escrow: std::collections::HashMap<String, i64>,
    pub resolution_window_epochs: Option<i64>,
    pub max_resolution_window_epochs: Option<i64>,
    pub created_epoch: i64,
    pub last_resolution_epoch: Option<i64>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SpotBetRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub user_address: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub escrow_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub amm_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_epoch: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SpotTransferRow {
    #[diesel(sql_type = Text)]
    pub user_address: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_epoch: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptPoolRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = Text)]
    pub associated_id: String,
    #[diesel(sql_type = Text)]
    pub symbol: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = BigInt)]
    pub circulating_supply: i64,
    #[diesel(sql_type = BigInt)]
    pub base_price: i64,
    #[diesel(sql_type = BigInt)]
    pub quadratic_coefficient: i64,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptTransactionRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_type: String,
    #[diesel(sql_type = Text)]
    pub sender: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub myso_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub creator_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub treasury_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub price: i64,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptHoldingRow {
    #[diesel(sql_type = Text)]
    pub holder_address: String,
    #[diesel(sql_type = BigInt)]
    pub balance: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptPriceHistoryRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = BigInt)]
    pub price: i64,
    #[diesel(sql_type = BigInt)]
    pub circulating_supply: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptExchangeConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub post_threshold: i64,
    #[diesel(sql_type = BigInt)]
    pub profile_threshold: i64,
    #[diesel(sql_type = BigInt)]
    pub max_individual_reservation_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub total_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub trading_creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub trading_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub trading_treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub reservation_creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub reservation_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub reservation_treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_reservers_per_pool: i64,
    #[diesel(sql_type = BigInt)]
    pub base_price: i64,
    #[diesel(sql_type = BigInt)]
    pub quadratic_coefficient: i64,
    #[diesel(sql_type = BigInt)]
    pub max_hold_percent_bps: i64,
    #[diesel(sql_type = Bool)]
    pub trading_enabled: bool,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptReservationPoolRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub associated_id: String,
    #[diesel(sql_type = SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = BigInt)]
    pub total_reserved: i64,
    #[diesel(sql_type = BigInt)]
    pub required_threshold: i64,
    #[diesel(sql_type = Text)]
    pub status: String,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptReservationRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub reserver_address: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub reserved_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub fee_amount: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub creator_fee: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub platform_fee: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub treasury_fee: Option<i64>,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SptRevenueRow {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_type: String,
    #[diesel(sql_type = Text)]
    pub trader: String,
    #[diesel(sql_type = Text)]
    pub creator_address: String,
    #[diesel(sql_type = Text)]
    pub platform_address: String,
    #[diesel(sql_type = Text)]
    pub treasury_address: String,
    #[diesel(sql_type = BigInt)]
    pub creator_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub treasury_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub total_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub token_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub myso_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub token_price: i64,
    #[diesel(sql_type = BigInt)]
    pub revenue_time: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

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

#[derive(Debug, Serialize, QueryableByName)]
pub struct ProfileSubscriptionServiceInfo {
    #[diesel(sql_type = Text)]
    pub service_id: String,
    #[diesel(sql_type = Text)]
    pub profile_owner: String,
    #[diesel(sql_type = Text)]
    pub profile_id: String,
    #[diesel(sql_type = BigInt)]
    pub monthly_fee: i64,
    #[diesel(sql_type = Bool)]
    pub active: bool,
    #[diesel(sql_type = BigInt)]
    pub subscriber_count: i64,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub updated_at: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub username: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub display_name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_photo: Option<String>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct ProfileSubscriptionInfo {
    #[diesel(sql_type = Text)]
    pub subscription_id: String,
    #[diesel(sql_type = Text)]
    pub service_id: String,
    #[diesel(sql_type = Text)]
    pub subscriber: String,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = BigInt)]
    pub expires_at: i64,
    #[diesel(sql_type = Bool)]
    pub auto_renew: bool,
    #[diesel(sql_type = BigInt)]
    pub renewal_balance: i64,
    #[diesel(sql_type = BigInt)]
    pub renewal_count: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub cancelled_at: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub monthly_fee: i64,
    #[diesel(sql_type = Text)]
    pub profile_owner: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub username: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct ProfileSubscriptionRevenueRow {
    #[diesel(sql_type = Text)]
    pub service_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub subscription_id: Option<String>,
    #[diesel(sql_type = Text)]
    pub from_address: String,
    #[diesel(sql_type = Text)]
    pub to_address: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = Text)]
    pub revenue_type: String,
    #[diesel(sql_type = BigInt)]
    pub payment_time: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct UpgradeEventRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub package_id: String,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct ObjectMigratedEventRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub object_id: String,
    #[diesel(sql_type = Text)]
    pub object_type: String,
    #[diesel(sql_type = BigInt)]
    pub old_version: i64,
    #[diesel(sql_type = BigInt)]
    pub new_version: i64,
    #[diesel(sql_type = Text)]
    pub migrated_by: String,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SpotConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub enable_flag: bool,
    #[diesel(sql_type = BigInt)]
    pub confidence_threshold_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub resolution_window_epochs: i64,
    #[diesel(sql_type = BigInt)]
    pub max_resolution_window_epochs: i64,
    #[diesel(sql_type = BigInt)]
    pub payout_delay_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_split_bps_platform: i64,
    #[diesel(sql_type = Text)]
    pub oracle_address: String,
    #[diesel(sql_type = BigInt)]
    pub max_single_bet: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsurancePolicyInfo {
    #[diesel(sql_type = Text)]
    pub policy_id: String,
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = Text)]
    pub insured: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub covered_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub premium_paid: i64,
    #[diesel(sql_type = BigInt)]
    pub start_time_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub expiry_time_ms: i64,
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct InsurancePolicyRow {
    #[diesel(sql_type = Text)]
    pub policy_id: String,
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = Text)]
    pub insured: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub covered_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub premium_paid: i64,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
}

#[derive(Clone)]
pub struct Reader {
    db: Db,
}

impl Reader {
    pub async fn new(database_url: Url, db_args: DbArgs) -> Result<Self, anyhow::Error> {
        let db = Db::for_read(database_url, db_args).await?;
        let _ = db.connect().await?;
        Ok(Self { db })
    }

    pub async fn get_profiles(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Profile>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = profiles::table
            .order_by(profiles::id.desc())
            .limit(limit)
            .offset(offset)
            .select(Profile::as_select())
            .load::<Profile>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_profile_count(&self) -> Result<i64, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let count: i64 = profiles::table.count().get_result(&mut conn).await?;
        Ok(count)
    }

    pub async fn get_profile_by_address(
        &self,
        address: &str,
    ) -> Result<Option<Profile>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = profiles::table
            .filter(profiles::owner_address.eq(address))
            .select(Profile::as_select())
            .first::<Profile>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_profile_by_username(
        &self,
        username: &str,
    ) -> Result<Option<Profile>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = profiles::table
            .filter(profiles::username.eq(username))
            .select(Profile::as_select())
            .first::<Profile>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_mydata_by_id(
        &self,
        mydata_id: &str,
    ) -> Result<Option<MyDataBasic>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT mydata_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
                   created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
                   geographic_region, data_quality, sample_size, is_updating, update_frequency
            FROM mydata_data
            WHERE mydata_id = $1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(mydata_id)
            .get_result::<MyDataBasic>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn list_mydata(
        &self,
        limit: i64,
        offset: i64,
        creator: Option<&str>,
        media_type: Option<&str>,
        platform_id: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<Vec<MyDataBasic>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let sort_clause = match sort_by {
            Some("price") => " ORDER BY COALESCE(one_time_price, subscription_price) DESC",
            Some("updated") => " ORDER BY last_updated DESC",
            _ => " ORDER BY created_at DESC",
        };
        let query = format!(
            "
            SELECT mydata_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
                   created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
                   geographic_region, data_quality, sample_size, is_updating, update_frequency
            FROM mydata_data
            WHERE ($1::text IS NULL OR owner = $1)
              AND ($2::text IS NULL OR media_type = $2)
              AND ($3::text IS NULL OR platform_id = $3)
            {}
            LIMIT $4 OFFSET $5
            ",
            sort_clause
        );
        let results = diesel::sql_query(&query)
            .bind::<Nullable<Text>, _>(creator)
            .bind::<Nullable<Text>, _>(media_type)
            .bind::<Nullable<Text>, _>(platform_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<MyDataBasic>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_mydata_configuration(
        &self,
    ) -> Result<Option<MyDataConfigInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT updated_by, enable_flag, max_tags, max_subscription_days,
                   max_free_access_grants, timestamp_ms, time, transaction_id
            FROM mydata_config
            ORDER BY time DESC
            LIMIT 1
        ";
        let result = diesel::sql_query(query)
            .get_result::<MyDataConfigInfo>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_popular_mydata(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataBasic>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT DISTINCT
                d.mydata_id, d.owner, d.media_type, d.tags, d.platform_id, d.timestamp_start, d.timestamp_end,
                d.created_at, d.last_updated, d.one_time_price, d.subscription_price, d.subscription_duration_days,
                d.geographic_region, d.data_quality, d.sample_size, d.is_updating, d.update_frequency
            FROM mydata_data d
            LEFT JOIN mydata_purchases p ON d.mydata_id = p.mydata_id
            LEFT JOIN mydata_revenue r ON d.mydata_id = r.mydata_id
            LEFT JOIN mydata_access_logs a ON d.mydata_id = a.mydata_id
            WHERE (d.one_time_price IS NOT NULL OR d.subscription_price IS NOT NULL)
            GROUP BY d.mydata_id, d.owner, d.media_type, d.tags, d.platform_id, d.timestamp_start, d.timestamp_end,
                     d.created_at, d.last_updated, d.one_time_price, d.subscription_price, d.subscription_duration_days,
                     d.geographic_region, d.data_quality, d.sample_size, d.is_updating, d.update_frequency
            ORDER BY (COUNT(p.id) + COUNT(r.id) + COUNT(a.id)) DESC, d.created_at DESC
            LIMIT $1 OFFSET $2
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<MyDataBasic>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_mydata_purchases(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PurchaseInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT id, mydata_id, buyer, price, purchase_type, purchase_time, time, transaction_id
            FROM mydata_purchases
            WHERE mydata_id = $1
            ORDER BY purchase_time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(mydata_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<PurchaseInfo>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_mydata_subscriptions(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SubscriptionInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT id, mydata_id, subscriber, subscription_start, subscription_end, price, time, transaction_id
            FROM mydata_subscriptions
            WHERE mydata_id = $1
            ORDER BY subscription_start DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(mydata_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SubscriptionInfo>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_mydata_revenue(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RevenueInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT id, mydata_id, from_address, to_address, amount, revenue_type, revenue_time, time, transaction_id
            FROM mydata_revenue
            WHERE mydata_id = $1
            ORDER BY revenue_time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(mydata_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<RevenueInfo>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_mydata_access_logs(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AccessLogInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT id, mydata_id, user_address, access_type, access_time, time, transaction_id
            FROM mydata_access_logs
            WHERE mydata_id = $1
            ORDER BY access_time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(mydata_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<AccessLogInfo>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_creator_mydata(
        &self,
        creator: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataBasic>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT mydata_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
                   created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
                   geographic_region, data_quality, sample_size, is_updating, update_frequency
            FROM mydata_data
            WHERE owner = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(creator)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<MyDataBasic>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_mydata_stats(
        &self,
        mydata_id: &str,
    ) -> Result<Option<MyDataStatsResponse>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT
                d.mydata_id, d.owner, d.media_type,
                COALESCE((SELECT SUM(amount) FROM mydata_revenue WHERE mydata_id = $1), 0) as total_revenue,
                (SELECT COUNT(*) FROM mydata_purchases WHERE mydata_id = $1) as purchase_count,
                (SELECT COUNT(*) FROM mydata_subscriptions WHERE mydata_id = $1) as subscription_count,
                (SELECT COUNT(*) FROM mydata_access_logs WHERE mydata_id = $1) as access_count,
                d.one_time_price, d.subscription_price, d.created_at, d.last_updated
            FROM mydata_data d
            WHERE d.mydata_id = $1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(mydata_id)
            .get_result::<MyDataStatsResponse>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_mydata_revenue_timeline(
        &self,
        mydata_id: &str,
    ) -> Result<Vec<DailyRevenue>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT
                time_bucket('1 day', to_timestamp(revenue_time))::date as day,
                SUM(amount) as daily_revenue,
                COUNT(*) as daily_transactions
            FROM mydata_revenue
            WHERE mydata_id = $1
            GROUP BY time_bucket('1 day', to_timestamp(revenue_time))
            ORDER BY day DESC
            LIMIT 30
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(mydata_id)
            .load::<DailyRevenue>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_mydata_access_analytics(
        &self,
        mydata_id: &str,
    ) -> Result<Vec<AccessAnalytics>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT
                time_bucket('1 day', to_timestamp(access_time))::date as day,
                access_type,
                COUNT(DISTINCT user_address) as unique_users,
                COUNT(*) as total_accesses
            FROM mydata_access_logs
            WHERE mydata_id = $1
            GROUP BY time_bucket('1 day', to_timestamp(access_time)), access_type
            ORDER BY day DESC, access_type
            LIMIT 100
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(mydata_id)
            .load::<AccessAnalytics>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_insurance_configuration(
        &self,
    ) -> Result<Option<InsuranceConfigInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT updated_by, enable_flag, min_coverage_bps, max_coverage_bps, max_duration_ms,
                   fee_bps, version, timestamp_ms, time, transaction_id
            FROM insurance_config
            ORDER BY time DESC
            LIMIT 1
        ";
        let result = diesel::sql_query(query)
            .get_result::<InsuranceConfigInfo>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn list_insurance_vaults(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InsuranceVaultRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT vault_id, underwriter, capital_balance, reserved
            FROM insurance_vaults
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<InsuranceVaultRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_insurance_vault(
        &self,
        vault_id: &str,
    ) -> Result<Option<InsuranceVaultInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT vault_id, underwriter, capital_balance, reserved, base_rate_bps_per_day,
                   utilization_multiplier_bps, max_exposure_per_market, max_exposure_per_user,
                   version, created_at, updated_at
            FROM insurance_vaults
            WHERE vault_id = $1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(vault_id)
            .get_result::<InsuranceVaultInfo>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn list_insurance_vault_transactions(
        &self,
        vault_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InsuranceVaultTransactionRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT transaction_type, amount, balance_after, timestamp_ms
            FROM insurance_vault_transactions
            WHERE vault_id = $1
            ORDER BY time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(vault_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<InsuranceVaultTransactionRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_insurance_vault_exposures(
        &self,
        vault_id: &str,
    ) -> Result<Vec<InsuranceVaultExposureRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT market_id, option_id, SUM(reserved_amount) as total_exposure
            FROM insurance_market_exposures
            WHERE vault_id = $1
            GROUP BY market_id, option_id
            ORDER BY total_exposure DESC
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(vault_id)
            .load::<InsuranceVaultExposureRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_insurance_policies(
        &self,
        insured: Option<&str>,
        market_id: Option<&str>,
        vault_id: Option<&str>,
        status: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InsurancePolicyRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT policy_id, market_id, insured, option_id, covered_amount, premium_paid, status
            FROM insurance_policies
            WHERE ($1::text IS NULL OR insured = $1)
              AND ($2::text IS NULL OR market_id = $2)
              AND ($3::text IS NULL OR vault_id = $3)
              AND ($4::smallint IS NULL OR status = $4)
            ORDER BY created_at DESC
            LIMIT $5 OFFSET $6
        ";
        let results = diesel::sql_query(query)
            .bind::<Nullable<Text>, _>(insured)
            .bind::<Nullable<Text>, _>(market_id)
            .bind::<Nullable<Text>, _>(vault_id)
            .bind::<Nullable<SmallInt>, _>(status)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<InsurancePolicyRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_insurance_policy(
        &self,
        policy_id: &str,
    ) -> Result<Option<InsurancePolicyInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT policy_id, market_id, insured, option_id, covered_amount, coverage_bps,
                   premium_paid, start_time_ms, expiry_time_ms, vault_id, status
            FROM insurance_policies
            WHERE policy_id = $1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(policy_id)
            .get_result::<InsurancePolicyInfo>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn list_insurance_market_policies(
        &self,
        market_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InsurancePolicyRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT policy_id, market_id, insured, option_id, covered_amount, premium_paid, status
            FROM insurance_policies
            WHERE market_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(market_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<InsurancePolicyRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spot_record(
        &self,
        post_id: &str,
    ) -> Result<Option<SpotRecordResponse>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT post_id, status, outcome, betting_options, option_escrow, resolution_window_epochs,
                   max_resolution_window_epochs, created_epoch, last_resolution_epoch
            FROM spot_records
            WHERE post_id = $1
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            post_id: String,
            #[diesel(sql_type = SmallInt)]
            status: i16,
            #[diesel(sql_type = Nullable<SmallInt>)]
            outcome: Option<i16>,
            #[diesel(sql_type = Nullable<Jsonb>)]
            betting_options: Option<serde_json::Value>,
            #[diesel(sql_type = Nullable<Jsonb>)]
            option_escrow: Option<serde_json::Value>,
            #[diesel(sql_type = Nullable<BigInt>)]
            resolution_window_epochs: Option<i64>,
            #[diesel(sql_type = Nullable<BigInt>)]
            max_resolution_window_epochs: Option<i64>,
            #[diesel(sql_type = BigInt)]
            created_epoch: i64,
            #[diesel(sql_type = Nullable<BigInt>)]
            last_resolution_epoch: Option<i64>,
        }
        let result = diesel::sql_query(query)
            .bind::<Text, _>(post_id)
            .get_result::<Row>(&mut conn)
            .await
            .optional()?;
        Ok(result.map(|r| {
            let betting_options: Vec<String> = r
                .betting_options
                .and_then(|v| serde_json::from_value::<Vec<String>>(v).ok())
                .unwrap_or_default();
            let option_escrow: std::collections::HashMap<String, i64> = r
                .option_escrow
                .and_then(|v| {
                    serde_json::from_value::<std::collections::HashMap<String, i64>>(v).ok()
                })
                .unwrap_or_default();
            SpotRecordResponse {
                post_id: r.post_id,
                status: r.status,
                outcome: r.outcome,
                betting_options,
                option_escrow,
                resolution_window_epochs: r.resolution_window_epochs,
                max_resolution_window_epochs: r.max_resolution_window_epochs,
                created_epoch: r.created_epoch,
                last_resolution_epoch: r.last_resolution_epoch,
            }
        }))
    }

    pub async fn list_spot_bets(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SpotBetRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT post_id, user_address, option_id, escrow_amount, amm_amount, timestamp_epoch
            FROM spot_bets
            WHERE post_id = $1
            ORDER BY time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(post_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SpotBetRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_spot_payouts(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SpotTransferRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT user_address, amount, timestamp_epoch
            FROM spot_payouts
            WHERE post_id = $1
            ORDER BY time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(post_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SpotTransferRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_spot_refunds(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SpotTransferRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT user_address, amount, timestamp_epoch
            FROM spot_refunds
            WHERE post_id = $1
            ORDER BY time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(post_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SpotTransferRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spot_configuration(
        &self,
    ) -> Result<Option<SpotConfigInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT updated_by, enable_flag, confidence_threshold_bps, resolution_window_epochs,
                   max_resolution_window_epochs, payout_delay_ms, fee_bps, fee_split_bps_platform,
                   oracle_address, max_single_bet, version, timestamp_ms, time, transaction_id
            FROM spot_config
            ORDER BY time DESC
            LIMIT 1
        ";
        let result = diesel::sql_query(query)
            .get_result::<SpotConfigInfo>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn list_proposals(
        &self,
        limit: i64,
        offset: i64,
        status: Option<i16>,
        proposal_type: Option<i16>,
        submitter: Option<&str>,
    ) -> Result<Vec<ProposalRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT id, title, description, proposal_type, reference_id, metadata_json, submitter,
                   submission_time, delegate_approval_count, delegate_rejection_count,
                   community_votes_for, community_votes_against, status, voting_start_time,
                   voting_end_time, reward_pool, implemented_description, implementation_time,
                   rescind_time, anonymous_voters_count
            FROM (SELECT DISTINCT ON (id) * FROM proposals ORDER BY id, time DESC) p
            WHERE ($1::smallint IS NULL OR status = $1)
              AND ($2::smallint IS NULL OR proposal_type = $2)
              AND ($3::text IS NULL OR submitter = $3)
            ORDER BY submission_time DESC
            LIMIT $4 OFFSET $5
        ";
        let results = diesel::sql_query(query)
            .bind::<Nullable<SmallInt>, _>(status)
            .bind::<Nullable<SmallInt>, _>(proposal_type)
            .bind::<Nullable<Text>, _>(submitter)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProposalRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_proposal_by_id(
        &self,
        id: &str,
    ) -> Result<Option<ProposalRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT id, title, description, proposal_type, reference_id, metadata_json, submitter,
                   submission_time, delegate_approval_count, delegate_rejection_count,
                   community_votes_for, community_votes_against, status, voting_start_time,
                   voting_end_time, reward_pool, implemented_description, implementation_time,
                   rescind_time, anonymous_voters_count
            FROM proposals
            WHERE id = $1 AND time = (SELECT max(time) FROM proposals WHERE id = $1)
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(id)
            .get_result::<ProposalRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_proposal_delegate_votes(
        &self,
        proposal_id: &str,
    ) -> Result<Vec<DelegateVoteRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT proposal_id, delegate_address, approve, vote_time, reason
            FROM delegate_votes
            WHERE proposal_id = $1
            ORDER BY vote_time DESC
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(proposal_id)
            .load::<DelegateVoteRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_proposal_community_votes_count(
        &self,
        proposal_id: &str,
    ) -> Result<i64, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT COUNT(*)::bigint FROM community_votes WHERE proposal_id = $1
        ";
        #[derive(QueryableByName)]
        struct CountRow {
            #[diesel(sql_type = BigInt)]
            count: i64,
        }
        let row = diesel::sql_query(query)
            .bind::<Text, _>(proposal_id)
            .get_result::<CountRow>(&mut conn)
            .await?;
        Ok(row.count)
    }

    pub async fn get_proposal_community_votes(
        &self,
        proposal_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CommunityVoteRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT proposal_id, voter_address, vote_weight, approve, vote_time, vote_cost
            FROM community_votes
            WHERE proposal_id = $1
            ORDER BY vote_time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(proposal_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<CommunityVoteRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_proposal_reward_distributions(
        &self,
        proposal_id: &str,
    ) -> Result<Vec<RewardDistributionRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT proposal_id, recipient_address, amount, distribution_time, distribution_type
            FROM reward_distributions
            WHERE proposal_id = $1
            ORDER BY distribution_time DESC
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(proposal_id)
            .load::<RewardDistributionRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_delegates(
        &self,
        limit: i64,
        offset: i64,
        registry_type: Option<i16>,
        is_active: Option<bool>,
    ) -> Result<Vec<DelegateRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT address, registry_type, upvotes, downvotes, proposals_reviewed, proposals_submitted,
                   sided_winning_proposals, sided_losing_proposals, term_start, term_end, is_active
            FROM (SELECT DISTINCT ON (address, registry_type) * FROM delegates ORDER BY address, registry_type, time DESC) d
            WHERE ($1::smallint IS NULL OR registry_type = $1)
              AND ($2::bool IS NULL OR is_active = $2)
            ORDER BY upvotes DESC
            LIMIT $3 OFFSET $4
        ";
        let results = diesel::sql_query(query)
            .bind::<Nullable<SmallInt>, _>(registry_type)
            .bind::<Nullable<Bool>, _>(is_active)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<DelegateRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_delegate_by_address(
        &self,
        address: &str,
    ) -> Result<Option<DelegateRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT address, registry_type, upvotes, downvotes, proposals_reviewed, proposals_submitted,
                   sided_winning_proposals, sided_losing_proposals, term_start, term_end, is_active
            FROM (SELECT DISTINCT ON (address, registry_type) * FROM delegates ORDER BY address, registry_type, time DESC) d
            WHERE address = $1
            LIMIT 1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .get_result::<DelegateRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_delegate_proposals(
        &self,
        address: &str,
    ) -> Result<Vec<ProposalRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT id, title, description, proposal_type, reference_id, metadata_json, submitter,
                   submission_time, delegate_approval_count, delegate_rejection_count,
                   community_votes_for, community_votes_against, status, voting_start_time,
                   voting_end_time, reward_pool, implemented_description, implementation_time,
                   rescind_time, anonymous_voters_count
            FROM proposals
            WHERE id IN (SELECT proposal_id FROM delegate_votes WHERE delegate_address = $1)
              AND time = (SELECT max(time) FROM proposals p2 WHERE p2.id = proposals.id)
            ORDER BY submission_time DESC
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .load::<ProposalRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_delegate_ratings(
        &self,
        address: &str,
    ) -> Result<Vec<DelegateRatingRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT target_address, voter_address, registry_type, is_active_delegate, upvote, rated_at
            FROM delegate_ratings
            WHERE target_address = $1
            ORDER BY rated_at DESC
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .load::<DelegateRatingRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_nominees(
        &self,
        limit: i64,
        offset: i64,
        registry_type: Option<i16>,
        status: Option<i16>,
    ) -> Result<Vec<NominatedDelegateRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT address, registry_type, upvotes, downvotes, scheduled_term_start_epoch,
                   nomination_time, status
            FROM (SELECT DISTINCT ON (address, registry_type) * FROM nominated_delegates ORDER BY address, registry_type, time DESC) n
            WHERE ($1::smallint IS NULL OR registry_type = $1)
              AND ($2::smallint IS NULL OR status = $2)
            ORDER BY upvotes DESC
            LIMIT $3 OFFSET $4
        ";
        let results = diesel::sql_query(query)
            .bind::<Nullable<SmallInt>, _>(registry_type)
            .bind::<Nullable<SmallInt>, _>(status)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<NominatedDelegateRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_governance_registries(
        &self,
    ) -> Result<Vec<GovernanceRegistryRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT registry_type, registry_id, delegate_count, delegate_term_epochs,
                   proposal_submission_cost, max_votes_per_user, voting_period_ms, quorum_votes
            FROM governance_registries
        ";
        let results = diesel::sql_query(query)
            .load::<GovernanceRegistryRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_governance_registry_by_type(
        &self,
        registry_type: i16,
    ) -> Result<Option<GovernanceRegistryRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT registry_type, registry_id, delegate_count, delegate_term_epochs,
                   proposal_submission_cost, max_votes_per_user, voting_period_ms, quorum_votes
            FROM governance_registries
            WHERE registry_type = $1
        ";
        let result = diesel::sql_query(query)
            .bind::<SmallInt, _>(registry_type)
            .get_result::<GovernanceRegistryRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn list_governance_events(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GovernanceEventRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT id, event_type, registry_type, event_data, event_id, created_at
            FROM governance_events
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<GovernanceEventRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_proposal_anonymous_stats(
        &self,
        proposal_id: &str,
    ) -> Result<Option<AnonymousVotingStatsRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT
                COUNT(*)::bigint as total_anonymous_votes,
                COUNT(*) FILTER (WHERE decryption_status = 1)::bigint as successfully_decrypted,
                COUNT(*) FILTER (WHERE decryption_status = 2)::bigint as failed_decryptions,
                COUNT(*) FILTER (WHERE decrypted_vote = 1)::bigint as anonymous_votes_for,
                COUNT(*) FILTER (WHERE decrypted_vote = 0)::bigint as anonymous_votes_against,
                COUNT(*) FILTER (WHERE decryption_status = 0)::bigint as pending_decryption
            FROM anonymous_votes
            WHERE proposal_id = $1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(proposal_id)
            .get_result::<AnonymousVotingStatsRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_proposal_anonymous_votes(
        &self,
        proposal_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AnonymousVoteRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT proposal_id, voter_address, submitted_at, decryption_status, processing_success
            FROM anonymous_votes
            WHERE proposal_id = $1
            ORDER BY submitted_at DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(proposal_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<AnonymousVoteRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_proposal_decryption_failures(
        &self,
        proposal_id: &str,
    ) -> Result<Vec<VoteDecryptionFailureRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT proposal_id, voter_address, failure_reason, attempted_at
            FROM vote_decryption_failures
            WHERE proposal_id = $1
            ORDER BY attempted_at DESC
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(proposal_id)
            .load::<VoteDecryptionFailureRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_anonymous_voting_trends(
        &self,
        limit: i64,
    ) -> Result<Vec<AnonymousVotingTrendRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT day::date as day, SUM(total_anonymous_votes)::bigint as total_votes,
                   SUM(successfully_decrypted)::bigint as successful_decryptions,
                   SUM(failed_decryptions)::bigint as failed_decryptions
            FROM anonymous_voting_daily_stats
            GROUP BY day
            ORDER BY day DESC
            LIMIT $1
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .load::<AnonymousVotingTrendRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spt_pool(
        &self,
        pool_id: &str,
    ) -> Result<Option<SptPoolRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pool_id, token_type, owner, associated_id, symbol, name, circulating_supply,
                   base_price, quadratic_coefficient, created_at, time, transaction_id
            FROM spt_pools
            WHERE pool_id = $1
            ORDER BY time DESC
            LIMIT 1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .get_result::<SptPoolRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn list_spt_pools(
        &self,
        limit: i64,
        offset: i64,
        owner: Option<&str>,
        token_type: Option<i16>,
    ) -> Result<Vec<SptPoolRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pool_id, token_type, owner, associated_id, symbol, name, circulating_supply,
                   base_price, quadratic_coefficient, created_at, time, transaction_id
            FROM (SELECT DISTINCT ON (pool_id) * FROM spt_pools ORDER BY pool_id, time DESC) p
            WHERE ($1::text IS NULL OR owner = $1)
              AND ($2::smallint IS NULL OR token_type = $2)
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
        ";
        let results = diesel::sql_query(query)
            .bind::<Nullable<Text>, _>(owner)
            .bind::<Nullable<SmallInt>, _>(token_type)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SptPoolRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spt_transactions(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptTransactionRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pool_id, transaction_type, sender, amount, myso_amount, fee_amount,
                   creator_fee, platform_fee, treasury_fee, price, created_at, time, transaction_id
            FROM spt_transactions
            WHERE pool_id = $1
            ORDER BY time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SptTransactionRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spt_holdings(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptHoldingRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT holder_address, SUM(amount)::bigint as balance
            FROM spt_holdings
            WHERE pool_id = $1
            GROUP BY holder_address
            HAVING SUM(amount) != 0
            ORDER BY balance DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SptHoldingRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spt_price_history(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptPriceHistoryRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pool_id, price, circulating_supply, time, transaction_id
            FROM spt_price_history
            WHERE pool_id = $1
            ORDER BY time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SptPriceHistoryRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spt_exchange_config(
        &self,
    ) -> Result<Option<SptExchangeConfigRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT updated_by, post_threshold, profile_threshold, max_individual_reservation_bps,
                   total_fee_bps, creator_fee_bps, platform_fee_bps, treasury_fee_bps,
                   trading_creator_fee_bps, trading_platform_fee_bps, trading_treasury_fee_bps,
                   reservation_creator_fee_bps, reservation_platform_fee_bps, reservation_treasury_fee_bps,
                   max_reservers_per_pool, base_price, quadratic_coefficient, max_hold_percent_bps,
                   trading_enabled, updated_at, time, transaction_id
            FROM spt_exchange_config
            ORDER BY time DESC
            LIMIT 1
        ";
        let result = diesel::sql_query(query)
            .get_result::<SptExchangeConfigRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_spt_reservation_pool(
        &self,
        pool_id: &str,
    ) -> Result<Option<SptReservationPoolRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pool_id, associated_id, token_type, owner, total_reserved, required_threshold,
                   status, created_at, time, transaction_id
            FROM spt_reservation_pools
            WHERE pool_id = $1 OR associated_id = $1
            ORDER BY time DESC
            LIMIT 1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .get_result::<SptReservationPoolRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn list_spt_reservations(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptReservationRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pool_id, reserver_address, amount, reserved_at, fee_amount, creator_fee,
                   platform_fee, treasury_fee, time, transaction_id
            FROM spt_reservations
            WHERE pool_id = $1
               OR pool_id = (
                   SELECT 'reservation_pool_' || associated_id
                   FROM spt_reservation_pools
                   WHERE pool_id = $1 OR associated_id = $1
                   ORDER BY time DESC
                   LIMIT 1
               )
            ORDER BY time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SptReservationRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spt_revenue(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptRevenueRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pool_id, transaction_type, trader, creator_address, platform_address,
                   treasury_address, creator_fee, platform_fee, treasury_fee, total_fee,
                   token_amount, myso_amount, token_price, revenue_time, time, transaction_id
            FROM spt_revenue
            WHERE pool_id = $1
            ORDER BY time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(pool_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<SptRevenueRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_upgrade_events(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UpgradeEventRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT id, package_id, version, event_id, transaction_id, created_at
            FROM upgrade_events
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<UpgradeEventRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_object_migrated_events(
        &self,
        limit: i64,
        offset: i64,
        object_id_filter: Option<&str>,
    ) -> Result<Vec<ObjectMigratedEventRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = if let Some(object_id) = object_id_filter {
            let query = "
                SELECT id, object_id, object_type, old_version, new_version, migrated_by,
                       event_id, transaction_id, created_at
                FROM object_migrated_events
                WHERE object_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
            ";
            diesel::sql_query(query)
                .bind::<Text, _>(object_id)
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .load::<ObjectMigratedEventRow>(&mut conn)
        } else {
            let query = "
                SELECT id, object_id, object_type, old_version, new_version, migrated_by,
                       event_id, transaction_id, created_at
                FROM object_migrated_events
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
            ";
            diesel::sql_query(query)
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .load::<ObjectMigratedEventRow>(&mut conn)
        }
        .await?;
        Ok(results)
    }

    pub async fn get_profile_subscription_service(
        &self,
        service_id: &str,
    ) -> Result<Option<ProfileSubscriptionServiceInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT s.service_id, s.profile_owner, s.profile_id, s.monthly_fee, s.active,
                   s.subscriber_count, s.created_at, s.updated_at,
                   p.username, p.display_name, p.profile_photo
            FROM profile_subscription_services s
            LEFT JOIN profiles p ON p.owner_address = s.profile_owner
            WHERE s.service_id = $1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(service_id)
            .get_result::<ProfileSubscriptionServiceInfo>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_profile_subscription_services_by_owner(
        &self,
        profile_owner: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionServiceInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT s.service_id, s.profile_owner, s.profile_id, s.monthly_fee, s.active,
                   s.subscriber_count, s.created_at, s.updated_at,
                   p.username, p.display_name, p.profile_photo
            FROM profile_subscription_services s
            LEFT JOIN profiles p ON p.owner_address = s.profile_owner
            WHERE s.profile_owner = $1
            ORDER BY s.created_at DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(profile_owner)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionServiceInfo>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_active_subscriptions_by_subscriber(
        &self,
        subscriber: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let now_ms = chrono::Utc::now().timestamp_millis();
        let query = "
            SELECT sub.subscription_id, sub.service_id, sub.subscriber, sub.created_at,
                   sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
                   sub.cancelled_at, s.monthly_fee, s.profile_owner,
                   p.username, p.display_name
            FROM (
                SELECT DISTINCT ON (subscription_id) *
                FROM profile_subscriptions
                WHERE subscriber = $1 AND cancelled_at IS NULL AND expires_at > $2
                ORDER BY subscription_id, time DESC
            ) sub
            JOIN profile_subscription_services s ON s.service_id = sub.service_id
            LEFT JOIN profiles p ON p.owner_address = s.profile_owner
            ORDER BY sub.expires_at DESC
            LIMIT $3 OFFSET $4
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(subscriber)
            .bind::<BigInt, _>(now_ms)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionInfo>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_subscription_by_id(
        &self,
        subscription_id: &str,
    ) -> Result<Option<ProfileSubscriptionInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT sub.subscription_id, sub.service_id, sub.subscriber, sub.created_at,
                   sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
                   sub.cancelled_at, s.monthly_fee, s.profile_owner,
                   p.username, p.display_name
            FROM (
                SELECT * FROM profile_subscriptions
                WHERE subscription_id = $1
                ORDER BY time DESC
                LIMIT 1
            ) sub
            JOIN profile_subscription_services s ON s.service_id = sub.service_id
            LEFT JOIN profiles p ON p.owner_address = s.profile_owner
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(subscription_id)
            .get_result::<ProfileSubscriptionInfo>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_subscription_revenue_by_service(
        &self,
        service_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionRevenueRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT service_id, subscription_id, from_address, to_address, amount,
                   revenue_type, payment_time, time, transaction_id
            FROM subscription_revenue
            WHERE service_id = $1
            ORDER BY time DESC
            LIMIT $2 OFFSET $3
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(service_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionRevenueRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn check_subscription_access(
        &self,
        subscriber: &str,
        service_id: &str,
    ) -> Result<bool, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let now_ms = chrono::Utc::now().timestamp_millis();
        let query = "
            SELECT 1
            FROM (
                SELECT DISTINCT ON (subscription_id) subscription_id, expires_at, cancelled_at
                FROM profile_subscriptions
                WHERE subscriber = $1 AND service_id = $2
                ORDER BY subscription_id, time DESC
            ) sub
            WHERE sub.cancelled_at IS NULL AND sub.expires_at > $3
            LIMIT 1
        ";
        #[derive(QueryableByName)]
        struct ExistsRow {
            #[diesel(sql_type = Integer)]
            _exists: i32,
        }
        let result = diesel::sql_query(query)
            .bind::<Text, _>(subscriber)
            .bind::<Text, _>(service_id)
            .bind::<BigInt, _>(now_ms)
            .get_result::<ExistsRow>(&mut conn)
            .await
            .optional()?;
        Ok(result.is_some())
    }
}
