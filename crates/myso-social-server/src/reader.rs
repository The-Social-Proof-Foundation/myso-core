// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{
    BigInt, Bool, Date, Double, Integer, Jsonb, Nullable, SmallInt, Text, Timestamp, Timestamptz,
};
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::PgTextExpressionMethods;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::Profile;
use myso_indexer_alt_social_schema::schema::{
    blocked_events, blocked_profiles, comments, ecosystem_treasury, platform_blocked_profiles,
    platform_events, platform_memberships, platform_moderators, platforms, poc_analysis_results,
    poc_configuration, poc_dispute_votes, post_config, posts, profile_events,
    profile_subscription_services, profile_subscriptions, profiles, promotion_views, reactions,
    reposts, social_graph_relationships, spt_holdings, spt_reservations, subscription_revenue,
    unified_revenue, vesting_events, vesting_wallets, wallet_social_graph,
};
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
pub struct SptReservationPoolWithDisplayRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
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
    pub created_at_epoch: i64,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub icon: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub primary_label: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub secondary_label: Option<String>,
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
pub struct UnifiedRevenueRow {
    #[diesel(sql_type = Text)]
    pub revenue_source: String,
    #[diesel(sql_type = Text)]
    pub revenue_type: String,
    #[diesel(sql_type = Text)]
    pub creator_address: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_address: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = Text)]
    pub currency: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub content_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub content_type: Option<String>,
    #[diesel(sql_type = Text)]
    pub payer_address: String,
    #[diesel(sql_type = Text)]
    pub recipient_address: String,
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

#[derive(Debug, Serialize)]
pub struct SystemStatsResponse {
    pub profiles: i64,
    pub platforms: i64,
    pub total_posts: i64,
    pub total_comments: i64,
    pub total_reactions: i64,
    pub social_proof_tokens: i64,
    pub total_social_relationships: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct PostBasicRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = Text)]
    pub profile_id: String,
    #[diesel(sql_type = Text)]
    pub content: String,
    #[diesel(sql_type = Text)]
    pub post_type: String,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub deleted_at: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub reaction_count: i64,
    #[diesel(sql_type = BigInt)]
    pub comment_count: i64,
    #[diesel(sql_type = BigInt)]
    pub repost_count: i64,
    #[diesel(sql_type = BigInt)]
    pub tips_received: i64,
}

#[derive(Debug, Serialize)]
pub struct ProfileEventRow {
    pub event_type: String,
    pub profile_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PlatformMembershipRow {
    pub platform_id: String,
    pub name: String,
    pub is_approved: bool,
    pub joined_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct ProfilePlatformEventRow {
    pub event_type: String,
    pub platform_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub event_id: Option<String>,
    pub event_data: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct BlockedEventRow {
    pub event_type: String,
    pub blocked_address: Option<String>,
    pub processed_at: chrono::NaiveDateTime,
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct ProfileBadgeRow {
    #[diesel(sql_type = Text)]
    pub badge_id: String,
    #[diesel(sql_type = Text)]
    pub badge_name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub badge_description: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub badge_media_url: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub badge_icon_url: Option<String>,
    #[diesel(sql_type = Text)]
    pub platform_id: String,
    #[diesel(sql_type = Text)]
    pub assigned_by: String,
    #[diesel(sql_type = BigInt)]
    pub assigned_at: i64,
    #[diesel(sql_type = Bool)]
    pub revoked: bool,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub revoked_at: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub revoked_by: Option<String>,
    #[diesel(sql_type = SmallInt)]
    pub badge_type: i16,
}

#[derive(Debug, Serialize)]
pub struct SocialGraphAddressRow {
    pub address: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct SocialStatsRow {
    pub followers_count: i64,
    pub following_count: i64,
    pub blocked_count: i64,
}

#[derive(Debug, Serialize)]
pub struct BlockedProfileRow {
    pub blocked_address: String,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    pub first_blocked_at: chrono::NaiveDateTime,
    pub last_blocked_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct BlockedPlatformRow {
    pub platform_id: String,
    pub name: String,
    pub blocked_by: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SocialGraphChartRow {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = Text)]
    pub event_type: String,
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct PlatformRow {
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub developer_address: String,
    pub status: i16,
    pub is_approved: bool,
    pub primary_category: String,
    pub secondary_category: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct PlatformModeratorRow {
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PlatformBlockedProfileRow {
    pub wallet_address: String,
    pub blocked_by: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PlatformApprovalRow {
    pub is_approved: bool,
    pub approval_changed_at: Option<chrono::NaiveDateTime>,
    pub approved_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PlatformMemberRow {
    pub wallet_address: String,
    pub joined_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PlatformEventRow {
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PostConfigRow {
    pub updated_by: String,
    pub max_content_length: i64,
    pub max_media_urls: i64,
    pub max_mentions: i64,
    pub max_metadata_size: i64,
    pub max_description_length: i64,
    pub max_reaction_length: i64,
    pub commenter_tip_percentage: i64,
    pub repost_tip_percentage: i64,
    pub version: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize)]
pub struct CommentRow {
    pub comment_id: String,
    pub post_id: String,
    pub parent_comment_id: Option<String>,
    pub owner: String,
    pub profile_id: String,
    pub content: String,
    pub created_at: i64,
    pub reaction_count: i64,
    pub comment_count: i64,
}

#[derive(Debug, Serialize)]
pub struct ReactionRow {
    pub user_address: String,
    pub reaction_text: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct RepostRow {
    pub repost_id: String,
    pub original_post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct PromotedPostRow {
    pub promotion_id: String,
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub payment_per_view: i64,
    pub total_budget: i64,
    pub remaining_budget: i64,
    pub active: bool,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct PromotionViewRow {
    pub post_id: String,
    pub promotion_id: String,
    pub viewer: String,
    pub payment_amount: i64,
    pub view_duration: i64,
    pub platform_id: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct PromotionStatsRow {
    pub total_views: i64,
    pub total_spent: i64,
    pub remaining_budget: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct PromotionTimeSeriesRow {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = BigInt)]
    pub views: i64,
    #[diesel(sql_type = BigInt)]
    pub spent: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct PromotionHourlyRow {
    #[diesel(sql_type = Integer)]
    pub hour: i32,
    #[diesel(sql_type = BigInt)]
    pub views: i64,
    #[diesel(sql_type = BigInt)]
    pub spent: i64,
}

#[derive(Debug, Serialize)]
pub struct PocBadgeRow {
    pub badge_id: String,
    pub post_id: String,
    pub media_type: i16,
    pub issued_by: String,
    pub issued_at: i64,
    pub revoked: bool,
}

#[derive(Debug, Serialize)]
pub struct PocRevenueRedirectionRow {
    pub redirection_id: String,
    pub accused_post_id: String,
    pub original_post_id: String,
    pub redirect_percentage: i64,
    pub similarity_score: i64,
    pub created_at: i64,
    pub removed: bool,
}

#[derive(Debug, Serialize)]
pub struct PocAnalysisResultRow {
    pub post_id: String,
    pub media_type: i16,
    pub similarity_detected: bool,
    pub highest_similarity_score: i64,
    pub oracle_address: String,
    pub analysis_timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct PocDisputeRow {
    pub dispute_id: String,
    pub post_id: String,
    pub disputer: String,
    pub dispute_type: i16,
    pub evidence: String,
    pub status: i16,
    pub stake_amount: i64,
    pub submitted_at: i64,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PocDisputeVoteRow {
    pub dispute_id: String,
    pub voter: String,
    pub vote_choice: i16,
    pub stake_amount: i64,
    pub voted_at: i64,
}

#[derive(Debug, Serialize)]
pub struct PocConfigRow {
    pub image_threshold: i64,
    pub video_threshold: i64,
    pub audio_threshold: i64,
    pub revenue_redirect_percentage: i64,
    pub dispute_cost: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize)]
pub struct VestingWalletRow {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub start_time: i64,
    pub duration: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct VestingEventRow {
    pub wallet_id: String,
    pub event_type: String,
    pub amount: i64,
    pub event_time: i64,
}

#[derive(Debug, Serialize)]
pub struct SubscriberSummaryRow {
    pub active_subscriptions: i64,
    pub total_revenue: i64,
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

    pub async fn list_spt_reservation_pools(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<SptReservationPoolWithDisplayRow>, i64), crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = r#"
        WITH latest_reservation_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_reservation_pools
            ORDER BY pool_id, time DESC
        ),
        latest_profiles AS (
            SELECT DISTINCT ON (profile_id) *
            FROM profiles
            WHERE profile_id IS NOT NULL
            ORDER BY profile_id, updated_at DESC
        ),
        latest_posts AS (
            SELECT DISTINCT ON (post_id) *
            FROM posts
            ORDER BY post_id, time DESC
        )
        SELECT
            rp.id, rp.pool_id, rp.associated_id, rp.token_type, rp.owner,
            rp.total_reserved, rp.required_threshold, rp.status,
            rp.created_at as created_at_epoch, rp.time as created_at, rp.transaction_id,
            CASE
                WHEN rp.token_type = 1 THEN prof.profile_photo
                WHEN rp.token_type = 2 THEN
                    CASE
                        WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                            CASE
                                WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                ELSE NULL
                            END
                        ELSE NULL
                    END
                ELSE NULL
            END as icon,
            CASE
                WHEN rp.token_type = 1 THEN
                    CASE
                        WHEN prof.profile_id IS NOT NULL THEN COALESCE(prof.display_name, prof.username)
                        ELSE 'Anonymous wallet'
                    END
                WHEN rp.token_type = 2 THEN post.content
                ELSE NULL
            END as primary_label,
            CASE
                WHEN rp.token_type = 1 THEN prof.username
                ELSE NULL
            END as secondary_label
        FROM latest_reservation_pools rp
        LEFT JOIN latest_profiles prof ON
            rp.token_type = 1 AND
            (CASE
                WHEN rp.associated_id LIKE 'profile_%' THEN SUBSTRING(rp.associated_id FROM 9)
                ELSE rp.associated_id
            END) = prof.profile_id
        LEFT JOIN latest_posts post ON
            rp.token_type = 2 AND
            (CASE
                WHEN rp.associated_id LIKE 'post_%' THEN SUBSTRING(rp.associated_id FROM 6)
                ELSE rp.associated_id
            END) = post.post_id
        WHERE rp.status = 'active' OR rp.status = 'threshold_met'
        ORDER BY rp.total_reserved DESC
        LIMIT $1 OFFSET $2
        "#;
        let pools: Vec<SptReservationPoolWithDisplayRow> = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(&mut conn)
            .await?;
        let count_query = r#"
        WITH latest_reservation_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_reservation_pools
            ORDER BY pool_id, time DESC
        )
        SELECT COUNT(*) as count
        FROM latest_reservation_pools
        WHERE status = 'active' OR status = 'threshold_met'
        "#;
        #[derive(QueryableByName)]
        struct CountRow {
            #[diesel(sql_type = BigInt)]
            count: i64,
        }
        let total: CountRow = diesel::sql_query(count_query).get_result(&mut conn).await?;
        Ok((pools, total.count))
    }

    pub async fn get_spt_analytics_top_performers(
        &self,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = r#"
        WITH current_prices AS (
            SELECT DISTINCT ON (pool_id) pool_id, price as current_price
            FROM spt_price_history
            ORDER BY pool_id, time DESC
        ),
        previous_prices AS (
            SELECT DISTINCT ON (pool_id) pool_id, price as previous_price
            FROM spt_price_history
            WHERE time < NOW() - INTERVAL '24 hours'
            ORDER BY pool_id, time DESC
        ),
        current_volume AS (
            SELECT pool_id, COALESCE(SUM(myso_amount), 0) as vol
            FROM spt_transactions
            WHERE time > NOW() - INTERVAL '24 hours'
            GROUP BY pool_id
        ),
        previous_volume AS (
            SELECT pool_id, COALESCE(SUM(myso_amount), 0) as vol
            FROM spt_transactions
            WHERE time BETWEEN NOW() - INTERVAL '48 hours' AND NOW() - INTERVAL '24 hours'
            GROUP BY pool_id
        ),
        pool_info AS (
            SELECT DISTINCT ON (pool_id) pool_id, name, symbol
            FROM spt_pools
            ORDER BY pool_id, time DESC
        )
        SELECT
            p.pool_id, p.name, p.symbol,
            COALESCE(cp.current_price, 0) as current_price,
            COALESCE(pp.previous_price, 0) as previous_price,
            COALESCE(cv.vol, 0) as current_volume,
            COALESCE(pv.vol, 0) as previous_volume,
            CASE WHEN COALESCE(pp.previous_price, 0) = 0 THEN 0.0
                 ELSE (COALESCE(cp.current_price, 0) - COALESCE(pp.previous_price, 0)) * 100.0 / pp.previous_price
            END as price_change_percentage,
            CASE WHEN COALESCE(pv.vol, 0) = 0 THEN 0.0
                 ELSE (COALESCE(cv.vol, 0) - COALESCE(pv.vol, 0)) * 100.0 / pv.vol
            END as volume_change_percentage
        FROM pool_info p
        LEFT JOIN current_prices cp ON p.pool_id = cp.pool_id
        LEFT JOIN previous_prices pp ON p.pool_id = pp.pool_id
        LEFT JOIN current_volume cv ON p.pool_id = cv.pool_id
        LEFT JOIN previous_volume pv ON p.pool_id = pv.pool_id
        ORDER BY price_change_percentage DESC NULLS LAST
        LIMIT 50
        "#;
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            pool_id: String,
            #[diesel(sql_type = Text)]
            name: String,
            #[diesel(sql_type = Text)]
            symbol: String,
            #[diesel(sql_type = BigInt)]
            current_price: i64,
            #[diesel(sql_type = BigInt)]
            previous_price: i64,
            #[diesel(sql_type = BigInt)]
            current_volume: i64,
            #[diesel(sql_type = BigInt)]
            previous_volume: i64,
            #[diesel(sql_type = Double)]
            price_change_percentage: f64,
            #[diesel(sql_type = Double)]
            volume_change_percentage: f64,
        }
        let rows: Vec<Row> = diesel::sql_query(query).load(&mut conn).await?;
        Ok(rows
            .into_iter()
            .map(|r| {
                serde_json::json!({
                    "pool_id": r.pool_id,
                    "name": r.name,
                    "symbol": r.symbol,
                    "current_price": r.current_price,
                    "previous_price": r.previous_price,
                    "current_volume": r.current_volume,
                    "previous_volume": r.previous_volume,
                    "price_change_percentage": r.price_change_percentage,
                    "volume_change_percentage": r.volume_change_percentage
                })
            })
            .collect())
    }

    pub async fn get_spt_portfolio_performance(
        &self,
        address: &str,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = r#"
        WITH latest_holdings AS (
            SELECT DISTINCT ON (pool_id) pool_id, amount
            FROM spt_holdings
            WHERE holder_address = $1
            ORDER BY pool_id, time DESC
        ),
        initial_tx AS (
            SELECT DISTINCT ON (pool_id) pool_id, price
            FROM spt_transactions
            WHERE sender = $1 AND transaction_type = 'BUY'
            ORDER BY pool_id, time ASC
        ),
        current_prices AS (
            SELECT DISTINCT ON (pool_id) pool_id, price
            FROM spt_price_history
            ORDER BY pool_id, time DESC
        ),
        pool_info AS (
            SELECT DISTINCT ON (pool_id) pool_id, name, symbol
            FROM spt_pools
            ORDER BY pool_id, time DESC
        )
        SELECT 
            h.pool_id, p.name, p.symbol, h.amount,
            h.amount * cp.price as current_value,
            COALESCE(it.price * h.amount, 0) as initial_value,
            CASE WHEN COALESCE(it.price * h.amount, 0) = 0 THEN 0.0
                 ELSE ((h.amount * cp.price) - (it.price * h.amount)) * 100.0 / (it.price * h.amount)
            END as roi_percentage
        FROM latest_holdings h
        JOIN pool_info p ON h.pool_id = p.pool_id
        JOIN current_prices cp ON h.pool_id = cp.pool_id
        LEFT JOIN initial_tx it ON h.pool_id = it.pool_id
        WHERE h.amount > 0
        "#;
        #[derive(QueryableByName)]
        struct HoldingRow {
            #[diesel(sql_type = Text)]
            pool_id: String,
            #[diesel(sql_type = Text)]
            name: String,
            #[diesel(sql_type = Text)]
            symbol: String,
            #[diesel(sql_type = BigInt)]
            amount: i64,
            #[diesel(sql_type = BigInt)]
            current_value: i64,
            #[diesel(sql_type = BigInt)]
            initial_value: i64,
            #[diesel(sql_type = Double)]
            roi_percentage: f64,
        }
        let holdings: Vec<HoldingRow> = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .load(&mut conn)
            .await?;
        let current_value: i64 = holdings.iter().map(|h| h.current_value).sum();
        let initial_value: i64 = holdings.iter().map(|h| h.initial_value).sum();
        let roi = if initial_value > 0 {
            (current_value - initial_value) as f64 * 100.0 / initial_value as f64
        } else {
            0.0
        };
        let holdings_json: Vec<serde_json::Value> = holdings
            .into_iter()
            .map(|h| {
                serde_json::json!({
                    "pool_id": h.pool_id,
                    "name": h.name,
                    "symbol": h.symbol,
                    "amount": h.amount,
                    "current_value": h.current_value,
                    "initial_value": h.initial_value,
                    "roi_percentage": h.roi_percentage
                })
            })
            .collect();
        Ok(serde_json::json!({
            "address": address,
            "current_value": current_value,
            "initial_investment": initial_value,
            "roi_percentage": roi,
            "holdings": holdings_json,
            "value_history": []
        }))
    }

    pub async fn get_spt_creator_revenue_streams(
        &self,
        address: &str,
        from_ts: chrono::DateTime<chrono::Utc>,
        to_ts: chrono::DateTime<chrono::Utc>,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = r#"
        WITH token_pools AS (
            SELECT DISTINCT ON (pool_id) pool_id, name, symbol
            FROM spt_pools
            WHERE owner = $1
            ORDER BY pool_id, time DESC
        ),
        buy_rev AS (
            SELECT pool_id, SUM(creator_fee) as buy_revenue, COUNT(*) as buy_count
            FROM spt_transactions
            WHERE transaction_type = 'BUY' AND pool_id IN (SELECT pool_id FROM token_pools)
              AND time >= $2 AND time <= $3
            GROUP BY pool_id
        ),
        sell_rev AS (
            SELECT pool_id, SUM(creator_fee) as sell_revenue, COUNT(*) as sell_count
            FROM spt_transactions
            WHERE transaction_type = 'SELL' AND pool_id IN (SELECT pool_id FROM token_pools)
              AND time >= $2 AND time <= $3
            GROUP BY pool_id
        )
        SELECT 
            tp.pool_id, tp.name, tp.symbol,
            COALESCE(bt.buy_revenue, 0) as buy_revenue,
            COALESCE(st.sell_revenue, 0) as sell_revenue,
            COALESCE(bt.buy_revenue, 0) + COALESCE(st.sell_revenue, 0) as total_revenue,
            COALESCE(bt.buy_count, 0) + COALESCE(st.sell_count, 0) as transactions_count
        FROM token_pools tp
        LEFT JOIN buy_rev bt ON tp.pool_id = bt.pool_id
        LEFT JOIN sell_rev st ON tp.pool_id = st.pool_id
        ORDER BY total_revenue DESC
        "#;
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            pool_id: String,
            #[diesel(sql_type = Text)]
            name: String,
            #[diesel(sql_type = Text)]
            symbol: String,
            #[diesel(sql_type = BigInt)]
            buy_revenue: i64,
            #[diesel(sql_type = BigInt)]
            sell_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_revenue: i64,
            #[diesel(sql_type = BigInt)]
            transactions_count: i64,
        }
        let rows: Vec<Row> = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .bind::<Timestamptz, _>(from_ts)
            .bind::<Timestamptz, _>(to_ts)
            .load(&mut conn)
            .await?;
        let total_revenue: i64 = rows.iter().map(|r| r.total_revenue).sum();
        let token_pools: Vec<serde_json::Value> = rows
            .into_iter()
            .map(|r| {
                serde_json::json!({
                    "pool_id": r.pool_id,
                    "name": r.name,
                    "symbol": r.symbol,
                    "total_revenue": r.total_revenue,
                    "buy_revenue": r.buy_revenue,
                    "sell_revenue": r.sell_revenue,
                    "transactions_count": r.transactions_count
                })
            })
            .collect();
        Ok(serde_json::json!({
            "address": address,
            "total_revenue": total_revenue,
            "token_pools": token_pools,
            "revenue_by_period": []
        }))
    }

    pub async fn get_spt_market_sentiment(
        &self,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = r#"
        WITH current_volume AS (
            SELECT
                COALESCE(SUM(CASE WHEN transaction_type = 'BUY' THEN myso_amount ELSE 0 END), 0) as buy_volume,
                COALESCE(SUM(CASE WHEN transaction_type = 'SELL' THEN myso_amount ELSE 0 END), 0) as sell_volume,
                COALESCE(COUNT(*), 0) as transaction_count,
                COALESCE(COUNT(DISTINCT CASE WHEN transaction_type = 'BUY' THEN sender END), 0) as unique_buyers,
                COALESCE(COUNT(DISTINCT CASE WHEN transaction_type = 'SELL' THEN sender END), 0) as unique_sellers
            FROM spt_transactions
            WHERE time > NOW() - INTERVAL '24 hours'
        ),
        previous_volume AS (
            SELECT COALESCE(SUM(myso_amount), 0) as total_volume
            FROM spt_transactions
            WHERE time BETWEEN NOW() - INTERVAL '48 hours' AND NOW() - INTERVAL '24 hours'
        )
        SELECT 
            c.buy_volume, c.sell_volume, c.transaction_count,
            c.unique_buyers, c.unique_sellers,
            CASE WHEN COALESCE(p.total_volume, 0) = 0 THEN 0.0
                 ELSE ((c.buy_volume + c.sell_volume) - p.total_volume) * 100.0 / p.total_volume
            END as volume_change_percentage,
            CASE WHEN (c.buy_volume + c.sell_volume) = 0 THEN 0.0
                 ELSE (c.buy_volume - c.sell_volume) * 1.0 / (c.buy_volume + c.sell_volume)
            END as sentiment_score
        FROM current_volume c
        CROSS JOIN previous_volume p
        "#;
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = BigInt)]
            buy_volume: i64,
            #[diesel(sql_type = BigInt)]
            sell_volume: i64,
            #[diesel(sql_type = BigInt)]
            transaction_count: i64,
            #[diesel(sql_type = BigInt)]
            unique_buyers: i64,
            #[diesel(sql_type = BigInt)]
            unique_sellers: i64,
            #[diesel(sql_type = Double)]
            volume_change_percentage: f64,
            #[diesel(sql_type = Double)]
            sentiment_score: f64,
        }
        let row: Row = diesel::sql_query(query).get_result(&mut conn).await?;
        Ok(serde_json::json!({
            "overall_sentiment": row.sentiment_score,
            "buy_volume_24h": row.buy_volume,
            "sell_volume_24h": row.sell_volume,
            "transaction_count_24h": row.transaction_count,
            "unique_buyers_24h": row.unique_buyers,
            "unique_sellers_24h": row.unique_sellers,
            "volume_change_percentage": row.volume_change_percentage,
            "price_momentum": []
        }))
    }

    pub async fn get_spt_liquidity_profile(
        &self,
        pool_id: &str,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let pool_query = "
            SELECT name, symbol
            FROM spt_pools
            WHERE pool_id = $1
            ORDER BY time DESC
            LIMIT 1
        ";
        #[derive(QueryableByName)]
        struct PoolRow {
            #[diesel(sql_type = Text)]
            name: String,
            #[diesel(sql_type = Text)]
            symbol: String,
        }
        let pool_info: Option<PoolRow> = diesel::sql_query(pool_query)
            .bind::<Text, _>(pool_id)
            .get_result(&mut conn)
            .await
            .optional()?;
        let (name, symbol) = pool_info
            .map(|p| (p.name, p.symbol))
            .unwrap_or_else(|| ("Unknown".to_string(), "?".to_string()));

        let metrics_query = "
            SELECT 
                COALESCE(SUM(myso_amount), 0) as total_volume,
                COALESCE(COUNT(*), 0) as transaction_count,
                COALESCE(MAX(myso_amount), 0) as largest_transaction,
                COALESCE(COUNT(DISTINCT sender), 0) as unique_traders_count,
                COALESCE(SUM(CASE WHEN transaction_type = 'BUY' THEN myso_amount ELSE 0 END), 0) as buy_volume,
                COALESCE(SUM(CASE WHEN transaction_type = 'SELL' THEN myso_amount ELSE 0 END), 0) as sell_volume
            FROM spt_transactions
            WHERE pool_id = $1 AND time > NOW() - INTERVAL '24 hours'
        ";
        #[derive(QueryableByName)]
        struct MetricsRow {
            #[diesel(sql_type = BigInt)]
            total_volume: i64,
            #[diesel(sql_type = BigInt)]
            transaction_count: i64,
            #[diesel(sql_type = BigInt)]
            largest_transaction: i64,
            #[diesel(sql_type = BigInt)]
            unique_traders_count: i64,
            #[diesel(sql_type = BigInt)]
            buy_volume: i64,
            #[diesel(sql_type = BigInt)]
            sell_volume: i64,
        }
        let metrics: MetricsRow = diesel::sql_query(metrics_query)
            .bind::<Text, _>(pool_id)
            .get_result(&mut conn)
            .await?;
        let avg_tx = if metrics.transaction_count > 0 {
            metrics.total_volume / metrics.transaction_count
        } else {
            0
        };
        let buy_sell_ratio = if metrics.sell_volume > 0 {
            metrics.buy_volume as f64 / metrics.sell_volume as f64
        } else {
            0.0
        };
        Ok(serde_json::json!({
            "pool_id": pool_id,
            "name": name,
            "symbol": symbol,
            "total_volume_24h": metrics.total_volume,
            "transaction_count_24h": metrics.transaction_count,
            "average_transaction_size": avg_tx,
            "largest_transaction": metrics.largest_transaction,
            "unique_traders_count": metrics.unique_traders_count,
            "buy_volume_24h": metrics.buy_volume,
            "sell_volume_24h": metrics.sell_volume,
            "buy_sell_ratio": buy_sell_ratio,
            "reservation_metrics": {}
        }))
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

    pub async fn get_revenue_dashboard(
        &self,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let dashboard_query = r#"
            SELECT revenue_source, total_revenue_24h, total_transactions_24h, largest_transaction_24h
            FROM revenue_dashboard_24h
            ORDER BY total_revenue_24h DESC
        "#;
        #[derive(QueryableByName)]
        struct DashboardRow {
            #[diesel(sql_type = Text)]
            revenue_source: String,
            #[diesel(sql_type = BigInt)]
            total_revenue_24h: i64,
            #[diesel(sql_type = BigInt)]
            total_transactions_24h: i64,
            #[diesel(sql_type = BigInt)]
            largest_transaction_24h: i64,
        }
        let dashboard_rows: Vec<DashboardRow> =
            diesel::sql_query(dashboard_query).load(&mut conn).await?;
        let total_revenue_24h: i64 = dashboard_rows.iter().map(|r| r.total_revenue_24h).sum();
        let total_transactions_24h: i64 = dashboard_rows
            .iter()
            .map(|r| r.total_transactions_24h)
            .sum();
        let largest_transaction_24h = dashboard_rows
            .iter()
            .map(|r| r.largest_transaction_24h)
            .max()
            .unwrap_or(0);

        let unique_query = r#"
            SELECT COUNT(DISTINCT creator_address) as unique_creators_24h,
                   COUNT(DISTINCT payer_address) as unique_payers_24h
            FROM unified_revenue
            WHERE time >= NOW() - INTERVAL '24 hours' AND amount > 0 AND currency = 'MYS'
        "#;
        #[derive(QueryableByName)]
        struct UniqueRow {
            #[diesel(sql_type = BigInt)]
            unique_creators_24h: i64,
            #[diesel(sql_type = BigInt)]
            unique_payers_24h: i64,
        }
        let unique: UniqueRow = diesel::sql_query(unique_query)
            .get_result(&mut conn)
            .await?;

        fn pct(a: i64, b: i64) -> f64 {
            if b == 0 {
                0.0
            } else {
                (a as f64 / b as f64) * 100.0
            }
        }
        let revenue_by_source: Vec<serde_json::Value> = dashboard_rows
            .into_iter()
            .map(|r| {
                serde_json::json!({
                    "revenue_source": r.revenue_source,
                    "total_revenue": r.total_revenue_24h,
                    "transaction_count": r.total_transactions_24h,
                    "percentage_of_total": pct(r.total_revenue_24h, total_revenue_24h),
                    "growth_rate": serde_json::Value::Null
                })
            })
            .collect();

        let top_creators = self.get_revenue_leaderboard_internal(10, 0, None).await?;
        let recent_trends = self.get_revenue_chart_data_internal(None, 24).await?;

        Ok(serde_json::json!({
            "total_revenue_24h": total_revenue_24h,
            "total_transactions_24h": total_transactions_24h,
            "unique_creators_24h": unique.unique_creators_24h,
            "unique_payers_24h": unique.unique_payers_24h,
            "largest_transaction_24h": largest_transaction_24h,
            "revenue_by_source": revenue_by_source,
            "top_creators": top_creators,
            "recent_trends": recent_trends
        }))
    }

    async fn get_revenue_leaderboard_internal(
        &self,
        limit: i64,
        min_revenue: i64,
        revenue_source: Option<&str>,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let mut query = "
            SELECT creator_address, total_revenue, total_subscription_revenue,
                   total_mydata_revenue, total_spt_revenue, total_tips_revenue,
                   total_transactions, total_unique_payers,
                   ROW_NUMBER() OVER (ORDER BY total_revenue DESC) as rank
            FROM spt_creator_revenue_summary
            WHERE total_revenue >= $1
        "
        .to_string();
        if let Some(src) = revenue_source {
            match src {
                "subscription" => query.push_str(" AND total_subscription_revenue > 0"),
                "mydata" => query.push_str(" AND total_mydata_revenue > 0"),
                "spt" => query.push_str(" AND total_spt_revenue > 0"),
                "tips" => query.push_str(" AND total_tips_revenue > 0"),
                _ => {}
            }
        }
        query.push_str(" ORDER BY total_revenue DESC LIMIT $2");
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            creator_address: String,
            #[diesel(sql_type = BigInt)]
            total_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_subscription_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_mydata_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_spt_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_tips_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_transactions: i64,
            #[diesel(sql_type = BigInt)]
            total_unique_payers: i64,
            #[diesel(sql_type = BigInt)]
            rank: i64,
        }
        let rows: Vec<Row> = diesel::sql_query(query)
            .bind::<BigInt, _>(min_revenue)
            .bind::<BigInt, _>(limit)
            .load(&mut conn)
            .await?;
        Ok(rows
            .into_iter()
            .map(|r| {
                serde_json::json!({
                    "rank": r.rank,
                    "creator_address": r.creator_address,
                    "total_revenue": r.total_revenue,
                    "revenue_breakdown": {
                        "subscription_revenue": r.total_subscription_revenue,
                        "mydata_revenue": r.total_mydata_revenue,
                        "spt_revenue": r.total_spt_revenue,
                        "tips_revenue": r.total_tips_revenue,
                        "posts_revenue": 0
                    },
                    "growth_rate": serde_json::Value::Null,
                    "transaction_count": r.total_transactions,
                    "unique_payers": r.total_unique_payers
                })
            })
            .collect())
    }

    pub async fn get_revenue_leaderboard(
        &self,
        limit: i64,
        min_revenue: i64,
        revenue_source: Option<&str>,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        self.get_revenue_leaderboard_internal(limit, min_revenue, revenue_source)
            .await
    }

    async fn get_revenue_chart_data_internal(
        &self,
        creator_address: Option<&str>,
        hours: i64,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let start_time = chrono::Utc::now() - chrono::Duration::hours(hours);
        let start_naive = start_time.naive_utc();
        let query = if creator_address.is_some() {
            "SELECT time_bucket('1 hour', time) as bucket, revenue_source, SUM(amount) as total_revenue,
                    COUNT(*) as transaction_count, COUNT(DISTINCT creator_address) as unique_creators,
                    COUNT(DISTINCT payer_address) as unique_payers
             FROM unified_revenue WHERE time >= $1 AND creator_address = $2
             GROUP BY bucket, revenue_source ORDER BY bucket ASC"
        } else {
            "SELECT time_bucket('1 hour', time) as bucket, revenue_source, SUM(amount) as total_revenue,
                    COUNT(*) as transaction_count, COUNT(DISTINCT creator_address) as unique_creators,
                    COUNT(DISTINCT payer_address) as unique_payers
             FROM unified_revenue WHERE time >= $1
             GROUP BY bucket, revenue_source ORDER BY bucket ASC"
        };
        #[derive(QueryableByName)]
        struct ChartRow {
            #[diesel(sql_type = Timestamp)]
            bucket: chrono::NaiveDateTime,
            #[diesel(sql_type = Text)]
            revenue_source: String,
            #[diesel(sql_type = BigInt)]
            total_revenue: i64,
            #[diesel(sql_type = BigInt)]
            transaction_count: i64,
            #[diesel(sql_type = BigInt)]
            unique_creators: i64,
            #[diesel(sql_type = BigInt)]
            unique_payers: i64,
        }
        let rows: Vec<ChartRow> = if let Some(addr) = creator_address {
            diesel::sql_query(query)
                .bind::<Timestamp, _>(start_naive)
                .bind::<Text, _>(addr)
                .load(&mut conn)
                .await?
        } else {
            diesel::sql_query(query)
                .bind::<Timestamp, _>(start_naive)
                .load(&mut conn)
                .await?
        };
        Ok(rows
            .into_iter()
            .map(|r| {
                serde_json::json!({
                    "timestamp": r.bucket.and_utc().to_rfc3339(),
                    "revenue_source": r.revenue_source,
                    "total_revenue": r.total_revenue,
                    "transaction_count": r.transaction_count,
                    "unique_creators": r.unique_creators,
                    "unique_payers": r.unique_payers
                })
            })
            .collect())
    }

    pub async fn get_revenue_chart_data(
        &self,
        creator_address: Option<&str>,
        period: &str,
        start_date: chrono::NaiveDateTime,
        end_date: chrono::NaiveDateTime,
        _points: i64,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        let time_bucket = match period {
            "hour" => "1 hour",
            "day" => "1 day",
            "week" => "1 week",
            "month" => "1 month",
            _ => "1 day",
        };
        let mut conn = self.db.connect().await?;
        let (query, has_creator) = if creator_address.is_some() {
            (
                format!(
                    "SELECT time_bucket('{}', time) as bucket, revenue_source, SUM(amount) as total_revenue,
                            COUNT(*) as transaction_count, COUNT(DISTINCT creator_address) as unique_creators,
                            COUNT(DISTINCT payer_address) as unique_payers
                     FROM unified_revenue WHERE time BETWEEN $1 AND $2 AND creator_address = $3
                     GROUP BY bucket, revenue_source ORDER BY bucket ASC",
                    time_bucket
                ),
                true,
            )
        } else {
            (
                format!(
                    "SELECT time_bucket('{}', time) as bucket, revenue_source, SUM(amount) as total_revenue,
                            COUNT(*) as transaction_count, COUNT(DISTINCT creator_address) as unique_creators,
                            COUNT(DISTINCT payer_address) as unique_payers
                     FROM unified_revenue WHERE time BETWEEN $1 AND $2
                     GROUP BY bucket, revenue_source ORDER BY bucket ASC",
                    time_bucket
                ),
                false,
            )
        };
        #[derive(QueryableByName)]
        struct ChartRow {
            #[diesel(sql_type = Timestamp)]
            bucket: chrono::NaiveDateTime,
            #[diesel(sql_type = Text)]
            revenue_source: String,
            #[diesel(sql_type = BigInt)]
            total_revenue: i64,
            #[diesel(sql_type = BigInt)]
            transaction_count: i64,
            #[diesel(sql_type = BigInt)]
            unique_creators: i64,
            #[diesel(sql_type = BigInt)]
            unique_payers: i64,
        }
        let rows: Vec<ChartRow> = if has_creator {
            diesel::sql_query(&query)
                .bind::<Timestamp, _>(start_date)
                .bind::<Timestamp, _>(end_date)
                .bind::<Text, _>(creator_address.unwrap())
                .load(&mut conn)
                .await?
        } else {
            diesel::sql_query(&query)
                .bind::<Timestamp, _>(start_date)
                .bind::<Timestamp, _>(end_date)
                .load(&mut conn)
                .await?
        };
        Ok(rows
            .into_iter()
            .map(|r| {
                serde_json::json!({
                    "timestamp": r.bucket.and_utc().to_rfc3339(),
                    "revenue_source": r.revenue_source,
                    "total_revenue": r.total_revenue,
                    "transaction_count": r.transaction_count,
                    "unique_creators": r.unique_creators,
                    "unique_payers": r.unique_payers
                })
            })
            .collect())
    }

    pub async fn get_unified_revenue(
        &self,
        creator_address: Option<&str>,
        platform_address: Option<&str>,
        revenue_source: Option<&str>,
        revenue_type: Option<&str>,
        content_id: Option<&str>,
        content_type: Option<&str>,
        start_date: Option<chrono::NaiveDateTime>,
        end_date: Option<chrono::NaiveDateTime>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<UnifiedRevenueRow>, i64, i64), crate::error::SocialError> {
        use diesel::dsl::sum;
        let mut conn = self.db.connect().await?;
        let mut query = unified_revenue::table.into_boxed();
        if let Some(a) = creator_address {
            query = query.filter(unified_revenue::creator_address.eq(a));
        }
        if let Some(a) = platform_address {
            query = query.filter(unified_revenue::platform_address.eq(a));
        }
        if let Some(s) = revenue_source {
            query = query.filter(unified_revenue::revenue_source.eq(s));
        }
        if let Some(t) = revenue_type {
            query = query.filter(unified_revenue::revenue_type.eq(t));
        }
        if let Some(c) = content_id {
            query = query.filter(unified_revenue::content_id.eq(c));
        }
        if let Some(c) = content_type {
            query = query.filter(unified_revenue::content_type.eq(c));
        }
        if let Some(d) = start_date {
            query = query.filter(unified_revenue::time.ge(
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
            ));
        }
        if let Some(d) = end_date {
            query = query.filter(unified_revenue::time.le(
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
            ));
        }
        let total_count: i64 = {
            let mut q = unified_revenue::table.into_boxed();
            if let Some(a) = creator_address {
                q = q.filter(unified_revenue::creator_address.eq(a));
            }
            if let Some(a) = platform_address {
                q = q.filter(unified_revenue::platform_address.eq(a));
            }
            if let Some(s) = revenue_source {
                q = q.filter(unified_revenue::revenue_source.eq(s));
            }
            if let Some(t) = revenue_type {
                q = q.filter(unified_revenue::revenue_type.eq(t));
            }
            if let Some(c) = content_id {
                q = q.filter(unified_revenue::content_id.eq(c));
            }
            if let Some(c) = content_type {
                q = q.filter(unified_revenue::content_type.eq(c));
            }
            if let Some(d) = start_date {
                q = q.filter(unified_revenue::time.ge(
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
                ));
            }
            if let Some(d) = end_date {
                q = q.filter(unified_revenue::time.le(
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
                ));
            }
            q.count().get_result(&mut conn).await?
        };
        let total_amount: Option<bigdecimal::BigDecimal> = {
            let mut q = unified_revenue::table.into_boxed();
            if let Some(a) = creator_address {
                q = q.filter(unified_revenue::creator_address.eq(a));
            }
            if let Some(a) = platform_address {
                q = q.filter(unified_revenue::platform_address.eq(a));
            }
            if let Some(s) = revenue_source {
                q = q.filter(unified_revenue::revenue_source.eq(s));
            }
            if let Some(t) = revenue_type {
                q = q.filter(unified_revenue::revenue_type.eq(t));
            }
            if let Some(c) = content_id {
                q = q.filter(unified_revenue::content_id.eq(c));
            }
            if let Some(c) = content_type {
                q = q.filter(unified_revenue::content_type.eq(c));
            }
            if let Some(d) = start_date {
                q = q.filter(unified_revenue::time.ge(
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
                ));
            }
            if let Some(d) = end_date {
                q = q.filter(unified_revenue::time.le(
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
                ));
            }
            q.select(sum(unified_revenue::amount))
                .get_result(&mut conn)
                .await?
        };
        let total_amount: i64 = total_amount
            .and_then(|bd| bigdecimal::ToPrimitive::to_i64(&bd))
            .unwrap_or(0);
        let rows: Vec<(
            String,
            String,
            String,
            Option<String>,
            i64,
            String,
            Option<String>,
            Option<String>,
            String,
            String,
            i64,
            chrono::DateTime<chrono::Utc>,
            String,
        )> = query
            .order_by(unified_revenue::time.desc())
            .limit(limit)
            .offset(offset)
            .select((
                unified_revenue::revenue_source,
                unified_revenue::revenue_type,
                unified_revenue::creator_address,
                unified_revenue::platform_address,
                unified_revenue::amount,
                unified_revenue::currency,
                unified_revenue::content_id,
                unified_revenue::content_type,
                unified_revenue::payer_address,
                unified_revenue::recipient_address,
                unified_revenue::revenue_time,
                unified_revenue::time,
                unified_revenue::transaction_id,
            ))
            .load(&mut conn)
            .await?;
        let records: Vec<UnifiedRevenueRow> = rows
            .into_iter()
            .map(
                |(
                    revenue_source,
                    revenue_type,
                    creator_address,
                    platform_address,
                    amount,
                    currency,
                    content_id,
                    content_type,
                    payer_address,
                    recipient_address,
                    revenue_time,
                    time,
                    transaction_id,
                )| UnifiedRevenueRow {
                    revenue_source,
                    revenue_type,
                    creator_address,
                    platform_address,
                    amount,
                    currency,
                    content_id,
                    content_type,
                    payer_address,
                    recipient_address,
                    revenue_time,
                    time,
                    transaction_id,
                },
            )
            .collect();
        Ok((records, total_count, total_amount))
    }

    pub async fn get_creator_revenue_stats(
        &self,
        creator_address: &str,
    ) -> Result<Option<serde_json::Value>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT creator_address, total_revenue, total_subscription_revenue, total_mydata_revenue,
                   total_spt_revenue, total_tips_revenue, total_transactions, total_unique_payers,
                   largest_single_transaction, active_days, last_revenue_date
            FROM spt_creator_revenue_summary WHERE creator_address = $1
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            creator_address: String,
            #[diesel(sql_type = BigInt)]
            total_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_subscription_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_mydata_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_spt_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_tips_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_transactions: i64,
            #[diesel(sql_type = BigInt)]
            total_unique_payers: i64,
            #[diesel(sql_type = BigInt)]
            largest_single_transaction: i64,
            #[diesel(sql_type = BigInt)]
            active_days: i64,
            #[diesel(sql_type = Nullable<Timestamptz>)]
            last_revenue_date: Option<chrono::DateTime<chrono::Utc>>,
        }
        let result: Option<Row> = diesel::sql_query(query)
            .bind::<Text, _>(creator_address)
            .get_result(&mut conn)
            .await
            .optional()?;
        Ok(result.map(|r| {
            serde_json::json!({
                "creator_address": r.creator_address,
                "total_revenue": r.total_revenue,
                "subscription_revenue": r.total_subscription_revenue,
                "mydata_revenue": r.total_mydata_revenue,
                "spt_revenue": r.total_spt_revenue,
                "tips_revenue": r.total_tips_revenue,
                "posts_revenue": 0,
                "total_transactions": r.total_transactions,
                "unique_payers": r.total_unique_payers,
                "largest_transaction": r.largest_single_transaction,
                "active_days": r.active_days,
                "last_revenue_date": r.last_revenue_date.map(|d| d.to_rfc3339()),
                "revenue_rank": serde_json::Value::Null
            })
        }))
    }

    pub async fn get_platform_revenue_stats(
        &self,
        platform_address: &str,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT platform_address, total_revenue, total_subscription_revenue, total_mydata_revenue,
                   total_spt_revenue, total_transactions, total_creators, total_payers,
                   avg_transaction_amount, active_months, last_active_month
            FROM platform_revenue_summary WHERE platform_address = $1
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            platform_address: String,
            #[diesel(sql_type = BigInt)]
            total_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_subscription_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_mydata_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_spt_revenue: i64,
            #[diesel(sql_type = BigInt)]
            total_transactions: i64,
            #[diesel(sql_type = BigInt)]
            total_creators: i64,
            #[diesel(sql_type = BigInt)]
            total_payers: i64,
            #[diesel(sql_type = Double)]
            avg_transaction_amount: f64,
            #[diesel(sql_type = BigInt)]
            active_months: i64,
            #[diesel(sql_type = Nullable<Date>)]
            last_active_month: Option<chrono::NaiveDate>,
        }
        let result: Option<Row> = diesel::sql_query(query)
            .bind::<Text, _>(platform_address)
            .get_result(&mut conn)
            .await
            .optional()?;
        Ok(result.map_or_else(
            || {
                serde_json::json!({
                    "platform_address": platform_address,
                    "total_revenue": 0,
                    "subscription_revenue": 0,
                    "mydata_revenue": 0,
                    "spt_revenue": 0,
                    "total_transactions": 0,
                    "unique_creators": 0,
                    "unique_payers": 0,
                    "avg_transaction_amount": 0.0,
                    "active_months": 0,
                    "last_active_month": serde_json::Value::Null
                })
            },
            |r| {
                serde_json::json!({
                    "platform_address": r.platform_address,
                    "total_revenue": r.total_revenue,
                    "subscription_revenue": r.total_subscription_revenue,
                    "mydata_revenue": r.total_mydata_revenue,
                    "spt_revenue": r.total_spt_revenue,
                    "total_transactions": r.total_transactions,
                    "unique_creators": r.total_creators,
                    "unique_payers": r.total_payers,
                    "avg_transaction_amount": r.avg_transaction_amount,
                    "active_months": r.active_months,
                    "last_active_month": r.last_active_month.map(|d| d.format("%Y-%m-%d").to_string())
                })
            },
        ))
    }

    pub async fn get_current_treasury(
        &self,
    ) -> Result<Option<serde_json::Value>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let row: Option<(String, String, i64, chrono::DateTime<chrono::Utc>, String)> =
            ecosystem_treasury::table
                .order_by(ecosystem_treasury::time.desc())
                .limit(1)
                .select((
                    ecosystem_treasury::treasury_address,
                    ecosystem_treasury::updated_by,
                    ecosystem_treasury::timestamp_ms,
                    ecosystem_treasury::time,
                    ecosystem_treasury::transaction_id,
                ))
                .get_result(&mut conn)
                .await
                .optional()?;
        Ok(row.map(
            |(treasury_address, updated_by, timestamp_ms, time, transaction_id)| {
                serde_json::json!({
                    "treasury_address": treasury_address,
                    "updated_by": updated_by,
                    "timestamp_ms": timestamp_ms,
                    "time": time.timestamp(),
                    "transaction_id": transaction_id
                })
            },
        ))
    }

    pub async fn get_subscription_analytics(
        &self,
        service_id: Option<&str>,
        _profile_owner: Option<&str>,
        start_date: chrono::NaiveDateTime,
        end_date: chrono::NaiveDateTime,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        use diesel::dsl::sum;
        let mut conn = self.db.connect().await?;
        let start_dt =
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(start_date, chrono::Utc);
        let end_dt =
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(end_date, chrono::Utc);

        let total_subscriptions: i64 = {
            let mut q = profile_subscriptions::table.into_boxed();
            q = q.filter(profile_subscriptions::time.between(start_dt, end_dt));
            if let Some(sid) = service_id {
                q = q.filter(profile_subscriptions::service_id.eq(sid));
            }
            q.count().get_result(&mut conn).await?
        };
        let active_subscriptions: i64 = {
            let mut q = profile_subscriptions::table.into_boxed();
            q = q.filter(profile_subscriptions::time.between(start_dt, end_dt));
            if let Some(sid) = service_id {
                q = q.filter(profile_subscriptions::service_id.eq(sid));
            }
            q.filter(profile_subscriptions::cancelled_at.is_null())
                .count()
                .get_result(&mut conn)
                .await?
        };
        let cancelled_subscriptions: i64 = {
            let mut q = profile_subscriptions::table.into_boxed();
            q = q.filter(profile_subscriptions::time.between(start_dt, end_dt));
            if let Some(sid) = service_id {
                q = q.filter(profile_subscriptions::service_id.eq(sid));
            }
            q.filter(profile_subscriptions::cancelled_at.is_not_null())
                .count()
                .get_result(&mut conn)
                .await?
        };

        let churn_rate = if total_subscriptions > 0 {
            cancelled_subscriptions as f64 / total_subscriptions as f64
        } else {
            0.0
        };

        let mut rev_query = subscription_revenue::table.into_boxed();
        rev_query = rev_query.filter(subscription_revenue::time.between(start_dt, end_dt));
        if let Some(sid) = service_id {
            rev_query = rev_query.filter(subscription_revenue::service_id.eq(sid));
        }
        let total_revenue: Option<bigdecimal::BigDecimal> = rev_query
            .select(sum(subscription_revenue::amount))
            .get_result(&mut conn)
            .await?;
        let total_revenue: i64 = total_revenue
            .and_then(|bd| bigdecimal::ToPrimitive::to_i64(&bd))
            .unwrap_or(0);
        let monthly_recurring_revenue = if total_revenue > 0 {
            total_revenue / 30
        } else {
            0
        };

        let service_id_str = service_id.unwrap_or("all").to_string();
        Ok(serde_json::json!({
            "service_id": service_id_str,
            "total_revenue": total_revenue,
            "active_subscriptions": active_subscriptions,
            "cancelled_subscriptions": cancelled_subscriptions,
            "monthly_recurring_revenue": monthly_recurring_revenue,
            "churn_rate": churn_rate,
            "average_subscription_duration": 30.0,
            "total_renewals": 0,
            "auto_renewal_rate": 0.0,
            "refund_rate": 0.0,
            "growth_metrics": []
        }))
    }

    pub async fn get_service_performance(
        &self,
        profile_owner: Option<&str>,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let mut query = profile_subscription_services::table.into_boxed();
        if let Some(owner) = profile_owner {
            query = query.filter(profile_subscription_services::profile_owner.eq(owner));
        }
        let rows: Vec<(String, String, String, i64, bool, i64, i64)> = query
            .select((
                profile_subscription_services::service_id,
                profile_subscription_services::profile_owner,
                profile_subscription_services::profile_id,
                profile_subscription_services::monthly_fee,
                profile_subscription_services::active,
                profile_subscription_services::subscriber_count,
                profile_subscription_services::created_at,
            ))
            .load(&mut conn)
            .await?;
        let services: Vec<serde_json::Value> = rows
            .into_iter()
            .map(
                |(
                    service_id,
                    profile_owner,
                    profile_id,
                    monthly_fee,
                    _active,
                    subscriber_count,
                    _created_at,
                )| {
                    let mrr = monthly_fee * subscriber_count;
                    serde_json::json!({
                        "service_id": service_id,
                        "profile_owner": profile_owner,
                        "profile_id": profile_id,
                        "monthly_fee": monthly_fee,
                        "total_subscribers": subscriber_count,
                        "active_subscribers": subscriber_count,
                        "total_revenue": mrr,
                        "monthly_recurring_revenue": mrr,
                        "churn_rate": 0.0,
                        "average_lifetime_value": 0.0,
                        "conversion_rate": 0.0
                    })
                },
            )
            .collect();
        Ok(services)
    }

    pub async fn get_treasury_history(
        &self,
        limit: i64,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let rows: Vec<(String, String, i64, chrono::DateTime<chrono::Utc>, String)> =
            ecosystem_treasury::table
                .order_by(ecosystem_treasury::time.desc())
                .limit(limit)
                .select((
                    ecosystem_treasury::treasury_address,
                    ecosystem_treasury::updated_by,
                    ecosystem_treasury::timestamp_ms,
                    ecosystem_treasury::time,
                    ecosystem_treasury::transaction_id,
                ))
                .load(&mut conn)
                .await?;
        Ok(rows
            .into_iter()
            .map(
                |(treasury_address, updated_by, timestamp_ms, time, transaction_id)| {
                    serde_json::json!({
                        "treasury_address": treasury_address,
                        "updated_by": updated_by,
                        "timestamp_ms": timestamp_ms,
                        "time": time.timestamp(),
                        "transaction_id": transaction_id
                    })
                },
            )
            .collect())
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

    pub async fn get_system_stats(&self) -> Result<SystemStatsResponse, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let profiles_count: i64 = profiles::table.count().get_result(&mut conn).await?;
        let platforms_count: i64 = platforms::table.count().get_result(&mut conn).await?;
        let social_relationships_count: i64 = social_graph_relationships::table
            .count()
            .get_result(&mut conn)
            .await?;
        let query = "
            SELECT
                (SELECT COUNT(*) FROM posts WHERE deleted_at IS NULL)::bigint as total_posts,
                (SELECT COUNT(*) FROM comments WHERE deleted_at IS NULL)::bigint as total_comments,
                (SELECT COUNT(*) FROM reactions)::bigint as total_reactions,
                (SELECT COUNT(*) FROM spt_pools)::bigint as social_proof_tokens
        ";
        #[derive(QueryableByName)]
        struct StatsRow {
            #[diesel(sql_type = BigInt)]
            total_posts: i64,
            #[diesel(sql_type = BigInt)]
            total_comments: i64,
            #[diesel(sql_type = BigInt)]
            total_reactions: i64,
            #[diesel(sql_type = BigInt)]
            social_proof_tokens: i64,
        }
        let row = diesel::sql_query(query)
            .get_result::<StatsRow>(&mut conn)
            .await?;
        Ok(SystemStatsResponse {
            profiles: profiles_count,
            platforms: platforms_count,
            total_posts: row.total_posts,
            total_comments: row.total_comments,
            total_reactions: row.total_reactions,
            social_proof_tokens: row.social_proof_tokens,
            total_social_relationships: social_relationships_count,
        })
    }

    pub async fn check_username_availability(
        &self,
        username: &str,
        exclude_address: Option<&str>,
    ) -> Result<bool, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let count: i64 = match exclude_address {
            Some(addr) => {
                profiles::table
                    .filter(profiles::username.eq(username))
                    .filter(profiles::owner_address.ne(addr))
                    .count()
                    .get_result(&mut conn)
                    .await?
            }
            None => {
                profiles::table
                    .filter(profiles::username.eq(username))
                    .count()
                    .get_result(&mut conn)
                    .await?
            }
        };
        Ok(count == 0)
    }

    pub async fn get_profile_posts(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostBasicRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let profile_id_opt: Option<String> = profiles::table
            .filter(profiles::owner_address.eq(address))
            .select(profiles::profile_id)
            .first::<Option<String>>(&mut conn)
            .await
            .optional()?
            .flatten();
        let query = "
            SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
                   reaction_count, comment_count, repost_count, tips_received
            FROM posts
            WHERE (owner = $1 OR ($2::text IS NOT NULL AND profile_id = $2))
              AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .bind::<Nullable<Text>, _>(profile_id_opt.as_deref())
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<PostBasicRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_profile_events(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileEventRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let profile_id_opt: Option<String> = profiles::table
            .filter(profiles::owner_address.eq(address))
            .select(profiles::profile_id)
            .first::<Option<String>>(&mut conn)
            .await
            .optional()?
            .flatten();
        let profile_ids: Vec<String> = if let Some(pid) = &profile_id_opt {
            vec![address.to_string(), pid.clone()]
        } else {
            vec![address.to_string()]
        };
        let results = profile_events::table
            .filter(profile_events::profile_id.eq_any(&profile_ids))
            .order_by(profile_events::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                profile_events::event_type,
                profile_events::profile_id,
                profile_events::event_data,
                profile_events::event_id,
                profile_events::created_at,
            ))
            .load::<(
                String,
                String,
                serde_json::Value,
                Option<String>,
                chrono::NaiveDateTime,
            )>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(event_type, profile_id, event_data, event_id, created_at)| ProfileEventRow {
                    event_type,
                    profile_id,
                    event_data,
                    event_id,
                    created_at,
                },
            )
            .collect())
    }

    pub async fn get_profile_platform_memberships(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformMembershipRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT p.platform_id, p.name, p.is_approved, pm.joined_at
            FROM platform_memberships pm
            INNER JOIN platforms p ON pm.platform_id = p.platform_id
            WHERE pm.wallet_address = $1
            ORDER BY pm.joined_at DESC
            LIMIT $2 OFFSET $3
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            platform_id: String,
            #[diesel(sql_type = Text)]
            name: String,
            #[diesel(sql_type = Bool)]
            is_approved: bool,
            #[diesel(sql_type = Timestamp)]
            joined_at: chrono::NaiveDateTime,
        }
        let results = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|r| PlatformMembershipRow {
                platform_id: r.platform_id,
                name: r.name,
                is_approved: r.is_approved,
                joined_at: r.joined_at,
            })
            .collect())
    }

    pub async fn get_profile_platform_events(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<ProfilePlatformEventRow>, i64), crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        #[derive(QueryableByName)]
        struct CountRow {
            #[diesel(sql_type = BigInt)]
            count: i64,
        }
        let total: i64 = diesel::sql_query(
            "SELECT COUNT(*)::bigint as count FROM platform_events
             WHERE event_type IN ('UserJoinedPlatform', 'UserLeftPlatform')
             AND event_data->>'wallet_address' = $1",
        )
        .bind::<Text, _>(address)
        .get_result::<CountRow>(&mut conn)
        .await?
        .count;
        let query = "
            SELECT event_type, platform_id, created_at, event_id, event_data
            FROM platform_events
            WHERE event_type IN ('UserJoinedPlatform', 'UserLeftPlatform')
            AND event_data->>'wallet_address' = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            event_type: String,
            #[diesel(sql_type = Text)]
            platform_id: String,
            #[diesel(sql_type = Timestamp)]
            created_at: chrono::NaiveDateTime,
            #[diesel(sql_type = Nullable<Text>)]
            event_id: Option<String>,
            #[diesel(sql_type = Jsonb)]
            event_data: serde_json::Value,
        }
        let results = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?;
        let events = results
            .into_iter()
            .map(|r| ProfilePlatformEventRow {
                event_type: r.event_type,
                platform_id: r.platform_id,
                created_at: r.created_at,
                event_id: r.event_id,
                event_data: r.event_data,
            })
            .collect();
        Ok((events, total))
    }

    pub async fn get_blocking_history(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BlockedEventRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = blocked_events::table
            .filter(blocked_events::blocker_address.eq(address))
            .order_by(blocked_events::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                blocked_events::event_type,
                blocked_events::blocked_address,
                blocked_events::processed_at,
                blocked_events::event_id,
            ))
            .load::<(
                String,
                Option<String>,
                chrono::NaiveDateTime,
                Option<String>,
            )>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(event_type, blocked_address, processed_at, event_id)| BlockedEventRow {
                    event_type,
                    blocked_address,
                    processed_at,
                    event_id,
                },
            )
            .collect())
    }

    pub async fn get_profile_badges(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileBadgeRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let profile_id_opt: Option<String> = profiles::table
            .filter(profiles::owner_address.eq(address))
            .select(profiles::profile_id)
            .first::<Option<String>>(&mut conn)
            .await
            .optional()?
            .flatten();
        let id2 = profile_id_opt.as_deref().unwrap_or(address).to_string();
        let query = "
            SELECT pb.badge_id, pb.badge_name, pb.badge_description, pb.badge_media_url,
                   pb.badge_icon_url, pb.platform_id, pb.assigned_by, pb.assigned_at,
                   pb.revoked, pb.revoked_at, pb.revoked_by, pb.badge_type
            FROM (
                SELECT DISTINCT ON (badge_id) *
                FROM profile_badges
                WHERE profile_id = $1 OR profile_id = $2
                ORDER BY badge_id, time DESC
            ) pb
            WHERE pb.revoked = false
            ORDER BY pb.assigned_at DESC
            LIMIT $3 OFFSET $4
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .bind::<Text, _>(&id2)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileBadgeRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_following(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SocialGraphAddressRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let profile_id_opt: Option<String> = profiles::table
            .filter(profiles::owner_address.eq(address))
            .select(profiles::profile_id)
            .first::<Option<String>>(&mut conn)
            .await
            .optional()?
            .flatten();
        let addrs: Vec<&str> = if let Some(ref pid) = profile_id_opt {
            vec![address, pid.as_str()]
        } else {
            vec![address]
        };
        let results = social_graph_relationships::table
            .filter(social_graph_relationships::follower_address.eq_any(&addrs))
            .order_by(social_graph_relationships::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                social_graph_relationships::following_address,
                social_graph_relationships::created_at,
            ))
            .load::<(String, chrono::NaiveDateTime)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|(address, created_at)| SocialGraphAddressRow {
                address,
                created_at,
            })
            .collect())
    }

    pub async fn get_followers(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SocialGraphAddressRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let profile_id_opt: Option<String> = profiles::table
            .filter(profiles::owner_address.eq(address))
            .select(profiles::profile_id)
            .first::<Option<String>>(&mut conn)
            .await
            .optional()?
            .flatten();
        let addrs: Vec<&str> = if let Some(ref pid) = profile_id_opt {
            vec![address, pid.as_str()]
        } else {
            vec![address]
        };
        let results = social_graph_relationships::table
            .filter(social_graph_relationships::following_address.eq_any(&addrs))
            .order_by(social_graph_relationships::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                social_graph_relationships::follower_address,
                social_graph_relationships::created_at,
            ))
            .load::<(String, chrono::NaiveDateTime)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|(address, created_at)| SocialGraphAddressRow {
                address,
                created_at,
            })
            .collect())
    }

    pub async fn get_social_stats(
        &self,
        address: &str,
    ) -> Result<SocialStatsRow, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let profile = profiles::table
            .filter(profiles::owner_address.eq(address))
            .select((
                profiles::followers_count,
                profiles::following_count,
                profiles::blocked_count,
            ))
            .first::<(i32, i32, i32)>(&mut conn)
            .await
            .optional()?;
        if let Some((followers_count, following_count, blocked_count)) = profile {
            return Ok(SocialStatsRow {
                followers_count: followers_count as i64,
                following_count: following_count as i64,
                blocked_count: blocked_count as i64,
            });
        }
        let ws = wallet_social_graph::table
            .filter(wallet_social_graph::wallet_address.eq(address))
            .select((
                wallet_social_graph::followers_count,
                wallet_social_graph::following_count,
                wallet_social_graph::blocked_count,
            ))
            .first::<(i32, i32, i32)>(&mut conn)
            .await
            .optional()?;
        if let Some((followers_count, following_count, blocked_count)) = ws {
            return Ok(SocialStatsRow {
                followers_count: followers_count as i64,
                following_count: following_count as i64,
                blocked_count: blocked_count as i64,
            });
        }
        Err(crate::error::SocialError::not_found(format!(
            "Profile or wallet '{}'",
            address
        )))
    }

    pub async fn get_blocked_profiles(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BlockedProfileRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = blocked_profiles::table
            .filter(blocked_profiles::blocker_address.eq(address))
            .order_by(blocked_profiles::last_blocked_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                blocked_profiles::blocked_address,
                blocked_profiles::blocked_username,
                blocked_profiles::blocked_display_name,
                blocked_profiles::blocked_profile_photo,
                blocked_profiles::first_blocked_at,
                blocked_profiles::last_blocked_at,
            ))
            .load::<(
                String,
                String,
                Option<String>,
                Option<String>,
                chrono::NaiveDateTime,
                chrono::NaiveDateTime,
            )>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(
                    blocked_address,
                    blocked_username,
                    blocked_display_name,
                    blocked_profile_photo,
                    first_blocked_at,
                    last_blocked_at,
                )| BlockedProfileRow {
                    blocked_address,
                    blocked_username,
                    blocked_display_name,
                    blocked_profile_photo,
                    first_blocked_at,
                    last_blocked_at,
                },
            )
            .collect())
    }

    pub async fn get_blocked_platforms(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BlockedPlatformRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT p.platform_id, p.name, pbp.blocked_by, pbp.created_at
            FROM platform_blocked_profiles pbp
            INNER JOIN platforms p ON pbp.platform_id = p.platform_id
            WHERE pbp.wallet_address = $1
            ORDER BY pbp.created_at DESC
            LIMIT $2 OFFSET $3
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            platform_id: String,
            #[diesel(sql_type = Text)]
            name: String,
            #[diesel(sql_type = Text)]
            blocked_by: String,
            #[diesel(sql_type = Timestamp)]
            created_at: chrono::NaiveDateTime,
        }
        let results = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|r| BlockedPlatformRow {
                platform_id: r.platform_id,
                name: r.name,
                blocked_by: r.blocked_by,
                created_at: r.created_at,
            })
            .collect())
    }

    pub async fn check_following(
        &self,
        follower: &str,
        following: &str,
    ) -> Result<bool, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let follower_profile: Option<(String, Option<String>)> = profiles::table
            .filter(
                profiles::owner_address
                    .eq(follower)
                    .or(profiles::profile_id.eq(follower)),
            )
            .select((profiles::owner_address, profiles::profile_id))
            .first(&mut conn)
            .await
            .optional()?;
        let following_profile: Option<(String, Option<String>)> = profiles::table
            .filter(
                profiles::owner_address
                    .eq(following)
                    .or(profiles::profile_id.eq(following)),
            )
            .select((profiles::owner_address, profiles::profile_id))
            .first(&mut conn)
            .await
            .optional()?;
        let follower_addrs: Vec<String> = match &follower_profile {
            Some((owner, pid)) => {
                let mut v = vec![owner.clone()];
                if let Some(p) = pid {
                    v.push(p.clone());
                }
                v
            }
            None => vec![follower.to_string()],
        };
        let following_addrs: Vec<String> = match &following_profile {
            Some((owner, pid)) => {
                let mut v = vec![owner.clone()];
                if let Some(p) = pid {
                    v.push(p.clone());
                }
                v
            }
            None => vec![following.to_string()],
        };
        let follower_refs: Vec<&str> = follower_addrs.iter().map(String::as_str).collect();
        let following_refs: Vec<&str> = following_addrs.iter().map(String::as_str).collect();
        let count: i64 = social_graph_relationships::table
            .filter(social_graph_relationships::follower_address.eq_any(&follower_refs))
            .filter(social_graph_relationships::following_address.eq_any(&following_refs))
            .count()
            .get_result(&mut conn)
            .await?;
        Ok(count > 0)
    }

    pub async fn get_social_graph_chart_data(
        &self,
        limit: i64,
    ) -> Result<Vec<SocialGraphChartRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT DATE(created_at) as day, event_type, COUNT(*)::bigint as count
            FROM social_graph_events
            WHERE created_at >= NOW() - INTERVAL '30 days'
            GROUP BY DATE(created_at), event_type
            ORDER BY day ASC, event_type
            LIMIT $1
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .load::<SocialGraphChartRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn check_profile_blocked(
        &self,
        blocker: &str,
        blocked: &str,
    ) -> Result<bool, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let count: i64 = blocked_profiles::table
            .filter(blocked_profiles::blocker_address.eq(blocker))
            .filter(blocked_profiles::blocked_address.eq(blocked))
            .count()
            .get_result(&mut conn)
            .await?;
        Ok(count > 0)
    }

    pub async fn check_platform_blocked(
        &self,
        profile_address: &str,
        platform_id: &str,
    ) -> Result<bool, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let count: i64 = platform_blocked_profiles::table
            .filter(platform_blocked_profiles::wallet_address.eq(profile_address))
            .filter(platform_blocked_profiles::platform_id.eq(platform_id))
            .count()
            .get_result(&mut conn)
            .await?;
        Ok(count > 0)
    }

    pub async fn list_badges(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileBadgeRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT DISTINCT ON (badge_id) badge_id, badge_name, badge_description, badge_media_url,
                   badge_icon_url, platform_id, assigned_by, assigned_at, revoked, revoked_at, revoked_by, badge_type
            FROM profile_badges
            WHERE revoked = false
            ORDER BY badge_id, time DESC
            LIMIT $1 OFFSET $2
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileBadgeRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_badge_by_id(
        &self,
        badge_id: &str,
    ) -> Result<Option<ProfileBadgeRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT badge_id, badge_name, badge_description, badge_media_url, badge_icon_url,
                   platform_id, assigned_by, assigned_at, revoked, revoked_at, revoked_by, badge_type
            FROM (
                SELECT DISTINCT ON (badge_id) *
                FROM profile_badges
                WHERE badge_id = $1
                ORDER BY badge_id, time DESC
            ) sub
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(badge_id)
            .get_result::<ProfileBadgeRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn list_platforms(
        &self,
        approved_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let mut query = platforms::table
            .filter(platforms::deleted_at.is_null())
            .into_boxed();
        if approved_only {
            query = query.filter(platforms::is_approved.eq(true));
        }
        let results = query
            .order_by(platforms::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                platforms::platform_id,
                platforms::name,
                platforms::tagline,
                platforms::description,
                platforms::logo,
                platforms::developer_address,
                platforms::status,
                platforms::is_approved,
                platforms::primary_category,
                platforms::secondary_category,
                platforms::created_at,
                platforms::updated_at,
                platforms::deleted_at,
            ))
            .load::<(
                String,
                String,
                String,
                Option<String>,
                Option<String>,
                String,
                i16,
                bool,
                String,
                Option<String>,
                chrono::NaiveDateTime,
                chrono::NaiveDateTime,
                Option<chrono::NaiveDateTime>,
            )>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(
                    platform_id,
                    name,
                    tagline,
                    description,
                    logo,
                    developer_address,
                    status,
                    is_approved,
                    primary_category,
                    secondary_category,
                    created_at,
                    updated_at,
                    deleted_at,
                )| {
                    PlatformRow {
                        platform_id,
                        name,
                        tagline,
                        description,
                        logo,
                        developer_address,
                        status,
                        is_approved,
                        primary_category,
                        secondary_category,
                        created_at,
                        updated_at,
                        deleted_at,
                    }
                },
            )
            .collect())
    }

    pub async fn get_platform_by_id(
        &self,
        platform_id: &str,
    ) -> Result<Option<PlatformRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = platforms::table
            .filter(platforms::platform_id.eq(platform_id))
            .filter(platforms::deleted_at.is_null())
            .select((
                platforms::platform_id,
                platforms::name,
                platforms::tagline,
                platforms::description,
                platforms::logo,
                platforms::developer_address,
                platforms::status,
                platforms::is_approved,
                platforms::primary_category,
                platforms::secondary_category,
                platforms::created_at,
                platforms::updated_at,
                platforms::deleted_at,
            ))
            .first::<(
                String,
                String,
                String,
                Option<String>,
                Option<String>,
                String,
                i16,
                bool,
                String,
                Option<String>,
                chrono::NaiveDateTime,
                chrono::NaiveDateTime,
                Option<chrono::NaiveDateTime>,
            )>(&mut conn)
            .await
            .optional()?;
        Ok(result.map(
            |(
                platform_id,
                name,
                tagline,
                description,
                logo,
                developer_address,
                status,
                is_approved,
                primary_category,
                secondary_category,
                created_at,
                updated_at,
                deleted_at,
            )| {
                PlatformRow {
                    platform_id,
                    name,
                    tagline,
                    description,
                    logo,
                    developer_address,
                    status,
                    is_approved,
                    primary_category,
                    secondary_category,
                    created_at,
                    updated_at,
                    deleted_at,
                }
            },
        ))
    }

    pub async fn get_platform_moderators(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformModeratorRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = platform_moderators::table
            .filter(platform_moderators::platform_id.eq(platform_id))
            .order_by(platform_moderators::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                platform_moderators::moderator_address,
                platform_moderators::added_by,
                platform_moderators::created_at,
            ))
            .load::<(String, String, chrono::NaiveDateTime)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(moderator_address, added_by, created_at)| PlatformModeratorRow {
                    moderator_address,
                    added_by,
                    created_at,
                },
            )
            .collect())
    }

    pub async fn get_platform_approval(
        &self,
        platform_id: &str,
    ) -> Result<Option<PlatformApprovalRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = platforms::table
            .filter(platforms::platform_id.eq(platform_id))
            .filter(platforms::deleted_at.is_null())
            .select((
                platforms::is_approved,
                platforms::approval_changed_at,
                platforms::approved_by,
            ))
            .first::<(bool, Option<chrono::NaiveDateTime>, Option<String>)>(&mut conn)
            .await
            .optional()?;
        Ok(result.map(
            |(is_approved, approval_changed_at, approved_by)| PlatformApprovalRow {
                is_approved,
                approval_changed_at,
                approved_by,
            },
        ))
    }

    pub async fn get_platform_blocked_profiles(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformBlockedProfileRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = platform_blocked_profiles::table
            .filter(platform_blocked_profiles::platform_id.eq(platform_id))
            .order_by(platform_blocked_profiles::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                platform_blocked_profiles::wallet_address,
                platform_blocked_profiles::blocked_by,
                platform_blocked_profiles::created_at,
            ))
            .load::<(String, String, chrono::NaiveDateTime)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(wallet_address, blocked_by, created_at)| PlatformBlockedProfileRow {
                    wallet_address,
                    blocked_by,
                    created_at,
                },
            )
            .collect())
    }

    pub async fn get_platform_members(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformMemberRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = platform_memberships::table
            .filter(platform_memberships::platform_id.eq(platform_id))
            .order_by(platform_memberships::joined_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                platform_memberships::wallet_address,
                platform_memberships::joined_at,
            ))
            .load::<(String, chrono::NaiveDateTime)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|(wallet_address, joined_at)| PlatformMemberRow {
                wallet_address,
                joined_at,
            })
            .collect())
    }

    pub async fn check_platform_membership(
        &self,
        platform_id: &str,
        profile_address: &str,
    ) -> Result<bool, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let count: i64 = platform_memberships::table
            .filter(platform_memberships::platform_id.eq(platform_id))
            .filter(platform_memberships::wallet_address.eq(profile_address))
            .count()
            .get_result(&mut conn)
            .await?;
        Ok(count > 0)
    }

    pub async fn get_platform_events(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformEventRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = platform_events::table
            .filter(platform_events::platform_id.eq(platform_id))
            .order_by(platform_events::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                platform_events::event_type,
                platform_events::event_data,
                platform_events::event_id,
                platform_events::created_at,
            ))
            .load::<(
                String,
                serde_json::Value,
                Option<String>,
                chrono::NaiveDateTime,
            )>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(event_type, event_data, event_id, created_at)| PlatformEventRow {
                    event_type,
                    event_data,
                    event_id,
                    created_at,
                },
            )
            .collect())
    }

    pub async fn list_posts(
        &self,
        owner: Option<&str>,
        post_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostBasicRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let mut query = posts::table
            .filter(posts::deleted_at.is_null())
            .into_boxed();
        if let Some(o) = owner {
            query = query.filter(posts::owner.eq(o));
        }
        if let Some(pt) = post_type {
            query = query.filter(posts::post_type.eq(pt));
        }
        let results = query
            .order_by(posts::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                posts::post_id,
                posts::owner,
                posts::profile_id,
                posts::content,
                posts::post_type,
                posts::created_at,
                posts::deleted_at,
                posts::reaction_count,
                posts::comment_count,
                posts::repost_count,
                posts::tips_received,
            ))
            .load::<(
                String,
                String,
                String,
                String,
                String,
                i64,
                Option<i64>,
                i64,
                i64,
                i64,
                i64,
            )>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(
                    post_id,
                    owner,
                    profile_id,
                    content,
                    post_type,
                    created_at,
                    deleted_at,
                    reaction_count,
                    comment_count,
                    repost_count,
                    tips_received,
                )| {
                    PostBasicRow {
                        post_id,
                        owner,
                        profile_id,
                        content,
                        post_type,
                        created_at,
                        deleted_at,
                        reaction_count,
                        comment_count,
                        repost_count,
                        tips_received,
                    }
                },
            )
            .collect())
    }

    pub async fn get_post_config(
        &self,
    ) -> Result<Option<PostConfigRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = post_config::table
            .order_by(post_config::time.desc())
            .limit(1)
            .select((
                post_config::updated_by,
                post_config::max_content_length,
                post_config::max_media_urls,
                post_config::max_mentions,
                post_config::max_metadata_size,
                post_config::max_description_length,
                post_config::max_reaction_length,
                post_config::commenter_tip_percentage,
                post_config::repost_tip_percentage,
                post_config::version,
                post_config::updated_at,
            ))
            .first::<(String, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>(&mut conn)
            .await
            .optional()?;
        Ok(result.map(
            |(
                updated_by,
                max_content_length,
                max_media_urls,
                max_mentions,
                max_metadata_size,
                max_description_length,
                max_reaction_length,
                commenter_tip_percentage,
                repost_tip_percentage,
                version,
                updated_at,
            )| {
                PostConfigRow {
                    updated_by,
                    max_content_length,
                    max_media_urls,
                    max_mentions,
                    max_metadata_size,
                    max_description_length,
                    max_reaction_length,
                    commenter_tip_percentage,
                    repost_tip_percentage,
                    version,
                    updated_at,
                }
            },
        ))
    }

    pub async fn get_trending_posts(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostBasicRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
                   reaction_count, comment_count, repost_count, tips_received
            FROM posts
            WHERE deleted_at IS NULL
            ORDER BY (reaction_count + comment_count + repost_count) DESC, created_at DESC
            LIMIT $1 OFFSET $2
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<PostBasicRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_post_by_id(
        &self,
        post_id: &str,
    ) -> Result<Option<PostBasicRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
                   reaction_count, comment_count, repost_count, tips_received
            FROM posts
            WHERE (post_id = $1 OR id = $1) AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT 1
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(post_id)
            .get_result::<PostBasicRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_post_comments(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CommentRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = comments::table
            .filter(comments::post_id.eq(post_id))
            .filter(comments::deleted_at.is_null())
            .order_by(comments::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                comments::comment_id,
                comments::post_id,
                comments::parent_comment_id,
                comments::owner,
                comments::profile_id,
                comments::content,
                comments::created_at,
                comments::reaction_count,
                comments::comment_count,
            ))
            .load::<(
                String,
                String,
                Option<String>,
                String,
                String,
                String,
                i64,
                i64,
                i64,
            )>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(
                    comment_id,
                    post_id,
                    parent_comment_id,
                    owner,
                    profile_id,
                    content,
                    created_at,
                    reaction_count,
                    comment_count,
                )| {
                    CommentRow {
                        comment_id,
                        post_id,
                        parent_comment_id,
                        owner,
                        profile_id,
                        content,
                        created_at,
                        reaction_count,
                        comment_count,
                    }
                },
            )
            .collect())
    }

    pub async fn get_post_reactions(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ReactionRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = reactions::table
            .filter(reactions::object_id.eq(post_id))
            .filter(reactions::is_post.eq(true))
            .order_by(reactions::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                reactions::user_address,
                reactions::reaction_text,
                reactions::created_at,
            ))
            .load::<(String, String, i64)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|(user_address, reaction_text, created_at)| ReactionRow {
                user_address,
                reaction_text,
                created_at,
            })
            .collect())
    }

    pub async fn get_post_reposts(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RepostRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = reposts::table
            .filter(reposts::original_post_id.eq(post_id))
            .filter(reposts::is_original_post.eq(true))
            .order_by(reposts::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                reposts::repost_id,
                reposts::original_post_id,
                reposts::owner,
                reposts::profile_id,
                reposts::created_at,
            ))
            .load::<(String, String, String, String, i64)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(repost_id, original_post_id, owner, profile_id, created_at)| RepostRow {
                    repost_id,
                    original_post_id,
                    owner,
                    profile_id,
                    created_at,
                },
            )
            .collect())
    }

    pub async fn list_promotions(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PromotedPostRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT promotion_id, post_id, owner, profile_id, payment_per_view, total_budget,
                   remaining_budget, active, created_at
            FROM (
                SELECT DISTINCT ON (promotion_id) *
                FROM promoted_posts
                ORDER BY promotion_id, time DESC
            ) sub
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            promotion_id: String,
            #[diesel(sql_type = Text)]
            post_id: String,
            #[diesel(sql_type = Text)]
            owner: String,
            #[diesel(sql_type = Text)]
            profile_id: String,
            #[diesel(sql_type = BigInt)]
            payment_per_view: i64,
            #[diesel(sql_type = BigInt)]
            total_budget: i64,
            #[diesel(sql_type = BigInt)]
            remaining_budget: i64,
            #[diesel(sql_type = Bool)]
            active: bool,
            #[diesel(sql_type = BigInt)]
            created_at: i64,
        }
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|r| PromotedPostRow {
                promotion_id: r.promotion_id,
                post_id: r.post_id,
                owner: r.owner,
                profile_id: r.profile_id,
                payment_per_view: r.payment_per_view,
                total_budget: r.total_budget,
                remaining_budget: r.remaining_budget,
                active: r.active,
                created_at: r.created_at,
            })
            .collect())
    }

    pub async fn get_promotion_by_post_id(
        &self,
        post_id: &str,
    ) -> Result<Option<PromotedPostRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT promotion_id, post_id, owner, profile_id, payment_per_view, total_budget,
                   remaining_budget, active, created_at
            FROM (
                SELECT DISTINCT ON (promotion_id) *
                FROM promoted_posts
                WHERE post_id = $1
                ORDER BY promotion_id, time DESC
            ) sub
            LIMIT 1
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            promotion_id: String,
            #[diesel(sql_type = Text)]
            post_id: String,
            #[diesel(sql_type = Text)]
            owner: String,
            #[diesel(sql_type = Text)]
            profile_id: String,
            #[diesel(sql_type = BigInt)]
            payment_per_view: i64,
            #[diesel(sql_type = BigInt)]
            total_budget: i64,
            #[diesel(sql_type = BigInt)]
            remaining_budget: i64,
            #[diesel(sql_type = Bool)]
            active: bool,
            #[diesel(sql_type = BigInt)]
            created_at: i64,
        }
        let result = diesel::sql_query(query)
            .bind::<Text, _>(post_id)
            .get_result::<Row>(&mut conn)
            .await
            .optional()?;
        Ok(result.map(|r| PromotedPostRow {
            promotion_id: r.promotion_id,
            post_id: r.post_id,
            owner: r.owner,
            profile_id: r.profile_id,
            payment_per_view: r.payment_per_view,
            total_budget: r.total_budget,
            remaining_budget: r.remaining_budget,
            active: r.active,
            created_at: r.created_at,
        }))
    }

    pub async fn get_promotion_views(
        &self,
        promotion_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PromotionViewRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = promotion_views::table
            .filter(promotion_views::promotion_id.eq(promotion_id))
            .order_by(promotion_views::timestamp.desc())
            .limit(limit)
            .offset(offset)
            .select((
                promotion_views::post_id,
                promotion_views::promotion_id,
                promotion_views::viewer,
                promotion_views::payment_amount,
                promotion_views::view_duration,
                promotion_views::platform_id,
                promotion_views::timestamp,
            ))
            .load::<(String, String, String, i64, i64, String, i64)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(
                    post_id,
                    promotion_id,
                    viewer,
                    payment_amount,
                    view_duration,
                    platform_id,
                    timestamp,
                )| {
                    PromotionViewRow {
                        post_id,
                        promotion_id,
                        viewer,
                        payment_amount,
                        view_duration,
                        platform_id,
                        timestamp,
                    }
                },
            )
            .collect())
    }

    pub async fn get_promotion_stats(
        &self,
        promotion_id: &str,
    ) -> Result<Option<PromotionStatsRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let views_query = "
            SELECT COUNT(*)::bigint as cnt, COALESCE(SUM(payment_amount), 0)::bigint as spent
            FROM promotion_views WHERE promotion_id = $1
        ";
        #[derive(QueryableByName)]
        struct ViewsRow {
            #[diesel(sql_type = BigInt)]
            cnt: i64,
            #[diesel(sql_type = BigInt)]
            spent: i64,
        }
        let views = diesel::sql_query(views_query)
            .bind::<Text, _>(promotion_id)
            .get_result::<ViewsRow>(&mut conn)
            .await?;
        let budget_query = "
            SELECT remaining_budget as val FROM (
                SELECT DISTINCT ON (promotion_id) remaining_budget
                FROM promoted_posts WHERE promotion_id = $1
                ORDER BY promotion_id, time DESC
            ) sub
        ";
        #[derive(QueryableByName)]
        struct BudgetRow {
            #[diesel(sql_type = BigInt)]
            val: i64,
        }
        let remaining: Option<i64> = diesel::sql_query(budget_query)
            .bind::<Text, _>(promotion_id)
            .get_result::<BudgetRow>(&mut conn)
            .await
            .optional()?
            .map(|r| r.val);
        Ok(Some(PromotionStatsRow {
            total_views: views.cnt,
            total_spent: views.spent,
            remaining_budget: remaining.unwrap_or(0),
        }))
    }

    pub async fn get_promotion_time_series(
        &self,
        promotion_id: &str,
        limit: i64,
    ) -> Result<Vec<PromotionTimeSeriesRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT DATE(to_timestamp(timestamp/1000)) as day,
                   COUNT(*)::bigint as views,
                   COALESCE(SUM(payment_amount), 0)::bigint as spent
            FROM promotion_views
            WHERE promotion_id = $1
              AND timestamp >= EXTRACT(EPOCH FROM NOW() - INTERVAL '30 days') * 1000
            GROUP BY DATE(to_timestamp(timestamp/1000))
            ORDER BY day ASC
            LIMIT $2
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(promotion_id)
            .bind::<BigInt, _>(limit)
            .load::<PromotionTimeSeriesRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_promotion_hourly(
        &self,
        promotion_id: &str,
        limit: i64,
    ) -> Result<Vec<PromotionHourlyRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT EXTRACT(HOUR FROM to_timestamp(timestamp/1000))::int as hour,
                   COUNT(*)::bigint as views,
                   COALESCE(SUM(payment_amount), 0)::bigint as spent
            FROM promotion_views
            WHERE promotion_id = $1
              AND timestamp >= EXTRACT(EPOCH FROM NOW() - INTERVAL '7 days') * 1000
            GROUP BY EXTRACT(HOUR FROM to_timestamp(timestamp/1000))
            ORDER BY hour ASC
            LIMIT $2
        ";
        let results = diesel::sql_query(query)
            .bind::<Text, _>(promotion_id)
            .bind::<BigInt, _>(limit)
            .load::<PromotionHourlyRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_top_performing_promotions(
        &self,
        limit: i64,
    ) -> Result<Vec<PromotedPostRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pp.promotion_id, pp.post_id, pp.owner, pp.profile_id, pp.payment_per_view,
                   pp.total_budget, pp.remaining_budget, pp.active, pp.created_at
            FROM (
                SELECT DISTINCT ON (promotion_id) promotion_id, post_id, owner, profile_id,
                       payment_per_view, total_budget, remaining_budget, active, created_at
                FROM promoted_posts
                ORDER BY promotion_id, time DESC
            ) pp
            JOIN (
                SELECT promotion_id, COUNT(*) as view_count
                FROM promotion_views
                GROUP BY promotion_id
            ) pv ON pp.promotion_id = pv.promotion_id
            ORDER BY pv.view_count DESC
            LIMIT $1
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            promotion_id: String,
            #[diesel(sql_type = Text)]
            post_id: String,
            #[diesel(sql_type = Text)]
            owner: String,
            #[diesel(sql_type = Text)]
            profile_id: String,
            #[diesel(sql_type = BigInt)]
            payment_per_view: i64,
            #[diesel(sql_type = BigInt)]
            total_budget: i64,
            #[diesel(sql_type = BigInt)]
            remaining_budget: i64,
            #[diesel(sql_type = Bool)]
            active: bool,
            #[diesel(sql_type = BigInt)]
            created_at: i64,
        }
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .load::<Row>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|r| PromotedPostRow {
                promotion_id: r.promotion_id,
                post_id: r.post_id,
                owner: r.owner,
                profile_id: r.profile_id,
                payment_per_view: r.payment_per_view,
                total_budget: r.total_budget,
                remaining_budget: r.remaining_budget,
                active: r.active,
                created_at: r.created_at,
            })
            .collect())
    }

    pub async fn get_spending_trends(
        &self,
        limit: i64,
    ) -> Result<Vec<PromotionTimeSeriesRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT DATE(to_timestamp(timestamp/1000)) as day,
                   COUNT(*)::bigint as views,
                   COALESCE(SUM(payment_amount), 0)::bigint as spent
            FROM promotion_views
            WHERE timestamp >= EXTRACT(EPOCH FROM NOW() - INTERVAL '30 days') * 1000
            GROUP BY DATE(to_timestamp(timestamp/1000))
            ORDER BY day ASC
            LIMIT $1
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .load::<PromotionTimeSeriesRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_poc_badges(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocBadgeRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT badge_id, post_id, media_type, issued_by, issued_at, revoked
            FROM (
                SELECT DISTINCT ON (badge_id) *
                FROM poc_badges
                ORDER BY badge_id, time DESC
            ) sub
            WHERE revoked = false
            ORDER BY issued_at DESC
            LIMIT $1 OFFSET $2
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            badge_id: String,
            #[diesel(sql_type = Text)]
            post_id: String,
            #[diesel(sql_type = SmallInt)]
            media_type: i16,
            #[diesel(sql_type = Text)]
            issued_by: String,
            #[diesel(sql_type = BigInt)]
            issued_at: i64,
            #[diesel(sql_type = Bool)]
            revoked: bool,
        }
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|r| PocBadgeRow {
                badge_id: r.badge_id,
                post_id: r.post_id,
                media_type: r.media_type,
                issued_by: r.issued_by,
                issued_at: r.issued_at,
                revoked: r.revoked,
            })
            .collect())
    }

    pub async fn get_poc_badge_by_id(
        &self,
        badge_id: &str,
    ) -> Result<Option<PocBadgeRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT badge_id, post_id, media_type, issued_by, issued_at, revoked
            FROM (
                SELECT DISTINCT ON (badge_id) *
                FROM poc_badges
                WHERE badge_id = $1
                ORDER BY badge_id, time DESC
            ) sub
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            badge_id: String,
            #[diesel(sql_type = Text)]
            post_id: String,
            #[diesel(sql_type = SmallInt)]
            media_type: i16,
            #[diesel(sql_type = Text)]
            issued_by: String,
            #[diesel(sql_type = BigInt)]
            issued_at: i64,
            #[diesel(sql_type = Bool)]
            revoked: bool,
        }
        let result = diesel::sql_query(query)
            .bind::<Text, _>(badge_id)
            .get_result::<Row>(&mut conn)
            .await
            .optional()?;
        Ok(result.map(|r| PocBadgeRow {
            badge_id: r.badge_id,
            post_id: r.post_id,
            media_type: r.media_type,
            issued_by: r.issued_by,
            issued_at: r.issued_at,
            revoked: r.revoked,
        }))
    }

    pub async fn list_poc_revenue_redirections(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocRevenueRedirectionRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT redirection_id, accused_post_id, original_post_id, redirect_percentage,
                   similarity_score, created_at, removed
            FROM (
                SELECT DISTINCT ON (redirection_id) *
                FROM poc_revenue_redirections
                ORDER BY redirection_id, time DESC
            ) sub
            WHERE removed = false
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            redirection_id: String,
            #[diesel(sql_type = Text)]
            accused_post_id: String,
            #[diesel(sql_type = Text)]
            original_post_id: String,
            #[diesel(sql_type = BigInt)]
            redirect_percentage: i64,
            #[diesel(sql_type = BigInt)]
            similarity_score: i64,
            #[diesel(sql_type = BigInt)]
            created_at: i64,
            #[diesel(sql_type = Bool)]
            removed: bool,
        }
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|r| PocRevenueRedirectionRow {
                redirection_id: r.redirection_id,
                accused_post_id: r.accused_post_id,
                original_post_id: r.original_post_id,
                redirect_percentage: r.redirect_percentage,
                similarity_score: r.similarity_score,
                created_at: r.created_at,
                removed: r.removed,
            })
            .collect())
    }

    pub async fn list_poc_analysis_results(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocAnalysisResultRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = poc_analysis_results::table
            .order_by(poc_analysis_results::analysis_timestamp.desc())
            .limit(limit)
            .offset(offset)
            .select((
                poc_analysis_results::post_id,
                poc_analysis_results::media_type,
                poc_analysis_results::similarity_detected,
                poc_analysis_results::highest_similarity_score,
                poc_analysis_results::oracle_address,
                poc_analysis_results::analysis_timestamp,
            ))
            .load::<(String, i16, bool, i64, String, i64)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(
                    post_id,
                    media_type,
                    similarity_detected,
                    highest_similarity_score,
                    oracle_address,
                    analysis_timestamp,
                )| {
                    PocAnalysisResultRow {
                        post_id,
                        media_type,
                        similarity_detected,
                        highest_similarity_score,
                        oracle_address,
                        analysis_timestamp,
                    }
                },
            )
            .collect())
    }

    pub async fn list_poc_disputes(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocDisputeRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT dispute_id, post_id, disputer, dispute_type, evidence, status,
                   stake_amount, submitted_at, resolved_at
            FROM (
                SELECT DISTINCT ON (dispute_id) *
                FROM poc_disputes
                ORDER BY dispute_id, time DESC
            ) sub
            ORDER BY submitted_at DESC
            LIMIT $1 OFFSET $2
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            dispute_id: String,
            #[diesel(sql_type = Text)]
            post_id: String,
            #[diesel(sql_type = Text)]
            disputer: String,
            #[diesel(sql_type = SmallInt)]
            dispute_type: i16,
            #[diesel(sql_type = Text)]
            evidence: String,
            #[diesel(sql_type = SmallInt)]
            status: i16,
            #[diesel(sql_type = BigInt)]
            stake_amount: i64,
            #[diesel(sql_type = BigInt)]
            submitted_at: i64,
            #[diesel(sql_type = Nullable<BigInt>)]
            resolved_at: Option<i64>,
        }
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|r| PocDisputeRow {
                dispute_id: r.dispute_id,
                post_id: r.post_id,
                disputer: r.disputer,
                dispute_type: r.dispute_type,
                evidence: r.evidence,
                status: r.status,
                stake_amount: r.stake_amount,
                submitted_at: r.submitted_at,
                resolved_at: r.resolved_at,
            })
            .collect())
    }

    pub async fn get_poc_dispute_by_id(
        &self,
        dispute_id: &str,
    ) -> Result<Option<PocDisputeRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT dispute_id, post_id, disputer, dispute_type, evidence, status,
                   stake_amount, submitted_at, resolved_at
            FROM (
                SELECT DISTINCT ON (dispute_id) *
                FROM poc_disputes
                WHERE dispute_id = $1
                ORDER BY dispute_id, time DESC
            ) sub
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            dispute_id: String,
            #[diesel(sql_type = Text)]
            post_id: String,
            #[diesel(sql_type = Text)]
            disputer: String,
            #[diesel(sql_type = SmallInt)]
            dispute_type: i16,
            #[diesel(sql_type = Text)]
            evidence: String,
            #[diesel(sql_type = SmallInt)]
            status: i16,
            #[diesel(sql_type = BigInt)]
            stake_amount: i64,
            #[diesel(sql_type = BigInt)]
            submitted_at: i64,
            #[diesel(sql_type = Nullable<BigInt>)]
            resolved_at: Option<i64>,
        }
        let result = diesel::sql_query(query)
            .bind::<Text, _>(dispute_id)
            .get_result::<Row>(&mut conn)
            .await
            .optional()?;
        Ok(result.map(|r| PocDisputeRow {
            dispute_id: r.dispute_id,
            post_id: r.post_id,
            disputer: r.disputer,
            dispute_type: r.dispute_type,
            evidence: r.evidence,
            status: r.status,
            stake_amount: r.stake_amount,
            submitted_at: r.submitted_at,
            resolved_at: r.resolved_at,
        }))
    }

    pub async fn get_poc_dispute_votes(
        &self,
        dispute_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocDisputeVoteRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = poc_dispute_votes::table
            .filter(poc_dispute_votes::dispute_id.eq(dispute_id))
            .order_by(poc_dispute_votes::voted_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                poc_dispute_votes::dispute_id,
                poc_dispute_votes::voter,
                poc_dispute_votes::vote_choice,
                poc_dispute_votes::stake_amount,
                poc_dispute_votes::voted_at,
            ))
            .load::<(String, String, i16, i64, i64)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(dispute_id, voter, vote_choice, stake_amount, voted_at)| PocDisputeVoteRow {
                    dispute_id,
                    voter,
                    vote_choice,
                    stake_amount,
                    voted_at,
                },
            )
            .collect())
    }

    pub async fn get_poc_configuration(
        &self,
    ) -> Result<Option<PocConfigRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = poc_configuration::table
            .order_by(poc_configuration::updated_at.desc())
            .limit(1)
            .select((
                poc_configuration::image_threshold,
                poc_configuration::video_threshold,
                poc_configuration::audio_threshold,
                poc_configuration::revenue_redirect_percentage,
                poc_configuration::dispute_cost,
                poc_configuration::updated_at,
            ))
            .first::<(i64, i64, i64, i64, i64, i64)>(&mut conn)
            .await
            .optional()?;
        Ok(result.map(
            |(
                image_threshold,
                video_threshold,
                audio_threshold,
                revenue_redirect_percentage,
                dispute_cost,
                updated_at,
            )| {
                PocConfigRow {
                    image_threshold,
                    video_threshold,
                    audio_threshold,
                    revenue_redirect_percentage,
                    dispute_cost,
                    updated_at,
                }
            },
        ))
    }

    pub async fn get_post_poc_badges(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocBadgeRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT badge_id, post_id, media_type, issued_by, issued_at, revoked
            FROM (
                SELECT DISTINCT ON (badge_id) *
                FROM poc_badges
                WHERE post_id = $1
                ORDER BY badge_id, time DESC
            ) sub
            WHERE revoked = false
            ORDER BY issued_at DESC
            LIMIT $2 OFFSET $3
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            badge_id: String,
            #[diesel(sql_type = Text)]
            post_id: String,
            #[diesel(sql_type = SmallInt)]
            media_type: i16,
            #[diesel(sql_type = Text)]
            issued_by: String,
            #[diesel(sql_type = BigInt)]
            issued_at: i64,
            #[diesel(sql_type = Bool)]
            revoked: bool,
        }
        let results = diesel::sql_query(query)
            .bind::<Text, _>(post_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|r| PocBadgeRow {
                badge_id: r.badge_id,
                post_id: r.post_id,
                media_type: r.media_type,
                issued_by: r.issued_by,
                issued_at: r.issued_at,
                revoked: r.revoked,
            })
            .collect())
    }

    pub async fn get_post_revenue_redirections(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocRevenueRedirectionRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT redirection_id, accused_post_id, original_post_id, redirect_percentage,
                   similarity_score, created_at, removed
            FROM (
                SELECT DISTINCT ON (redirection_id) *
                FROM poc_revenue_redirections
                WHERE accused_post_id = $1 OR original_post_id = $1
                ORDER BY redirection_id, time DESC
            ) sub
            WHERE removed = false
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        ";
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = Text)]
            redirection_id: String,
            #[diesel(sql_type = Text)]
            accused_post_id: String,
            #[diesel(sql_type = Text)]
            original_post_id: String,
            #[diesel(sql_type = BigInt)]
            redirect_percentage: i64,
            #[diesel(sql_type = BigInt)]
            similarity_score: i64,
            #[diesel(sql_type = BigInt)]
            created_at: i64,
            #[diesel(sql_type = Bool)]
            removed: bool,
        }
        let results = diesel::sql_query(query)
            .bind::<Text, _>(post_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(|r| PocRevenueRedirectionRow {
                redirection_id: r.redirection_id,
                accused_post_id: r.accused_post_id,
                original_post_id: r.original_post_id,
                redirect_percentage: r.redirect_percentage,
                similarity_score: r.similarity_score,
                created_at: r.created_at,
                removed: r.removed,
            })
            .collect())
    }

    pub async fn get_poc_analytics(&self) -> Result<serde_json::Value, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        #[derive(QueryableByName)]
        struct CountRow {
            #[diesel(sql_type = BigInt)]
            count: i64,
        }
        let badges_count: i64 = diesel::sql_query(
            "SELECT COUNT(DISTINCT badge_id)::bigint as count FROM poc_badges WHERE revoked = false",
        )
        .get_result::<CountRow>(&mut conn)
        .await
        .map(|r| r.count)?;
        let disputes_count: i64 = diesel::sql_query(
            "SELECT COUNT(DISTINCT dispute_id)::bigint as count FROM poc_disputes",
        )
        .get_result::<CountRow>(&mut conn)
        .await
        .map(|r| r.count)?;
        let redirections_count: i64 = diesel::sql_query(
            "SELECT COUNT(DISTINCT redirection_id)::bigint as count FROM poc_revenue_redirections WHERE removed = false",
        )
        .get_result::<CountRow>(&mut conn)
        .await
        .map(|r| r.count)?;
        Ok(serde_json::json!({
            "total_badges": badges_count,
            "total_disputes": disputes_count,
            "total_revenue_redirections": redirections_count,
        }))
    }

    pub async fn list_subscriptions(
        &self,
        subscriber: Option<&str>,
        service_id: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT sub.subscription_id, sub.service_id, sub.subscriber, sub.created_at,
                   sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
                   sub.cancelled_at, s.monthly_fee, s.profile_owner,
                   p.username, p.display_name
            FROM (
                SELECT DISTINCT ON (subscription_id) *
                FROM profile_subscriptions
                WHERE ($1::text IS NULL OR subscriber = $1)
                  AND ($2::text IS NULL OR service_id = $2)
                ORDER BY subscription_id, time DESC
            ) sub
            JOIN profile_subscription_services s ON s.service_id = sub.service_id
            LEFT JOIN profiles p ON p.owner_address = s.profile_owner
            ORDER BY sub.expires_at DESC
            LIMIT $3 OFFSET $4
        ";
        let results = diesel::sql_query(query)
            .bind::<Nullable<Text>, _>(subscriber)
            .bind::<Nullable<Text>, _>(service_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionInfo>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_subscription_services(
        &self,
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
            ORDER BY s.subscriber_count DESC, s.created_at DESC
            LIMIT $1 OFFSET $2
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionServiceInfo>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_subscription_revenue(
        &self,
        service_id: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionRevenueRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let (query, bind_service) = if service_id.is_some() {
            (
                "SELECT service_id, subscription_id, from_address, to_address, amount,
                        revenue_type, payment_time, time, transaction_id
                 FROM subscription_revenue WHERE service_id = $1
                 ORDER BY time DESC LIMIT $2 OFFSET $3",
                true,
            )
        } else {
            (
                "SELECT service_id, subscription_id, from_address, to_address, amount,
                        revenue_type, payment_time, time, transaction_id
                 FROM subscription_revenue
                 ORDER BY time DESC LIMIT $1 OFFSET $2",
                false,
            )
        };
        let results = if bind_service {
            diesel::sql_query(query)
                .bind::<Text, _>(service_id.unwrap())
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .load::<ProfileSubscriptionRevenueRow>(&mut conn)
                .await?
        } else {
            diesel::sql_query(query)
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .load::<ProfileSubscriptionRevenueRow>(&mut conn)
                .await?
        };
        Ok(results)
    }

    pub async fn get_subscriber_summary(
        &self,
        address: &str,
    ) -> Result<SubscriberSummaryRow, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let now_ms = chrono::Utc::now().timestamp_millis();
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = BigInt)]
            active: i64,
            #[diesel(sql_type = BigInt)]
            revenue: i64,
        }
        let query = "
            SELECT
                (SELECT COUNT(DISTINCT subscription_id)::bigint FROM profile_subscriptions
                 WHERE subscriber = $1 AND cancelled_at IS NULL AND expires_at > $2) as active,
                (SELECT COALESCE(SUM(amount), 0)::bigint FROM subscription_revenue
                 WHERE from_address = $1) as revenue
        ";
        let row = diesel::sql_query(query)
            .bind::<Text, _>(address)
            .bind::<BigInt, _>(now_ms)
            .get_result::<Row>(&mut conn)
            .await?;
        Ok(SubscriberSummaryRow {
            active_subscriptions: row.active,
            total_revenue: row.revenue,
        })
    }

    pub async fn list_vesting_wallets(
        &self,
        active_only: bool,
        owner: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VestingWalletRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let mut query = vesting_wallets::table.into_boxed();
        if active_only {
            query = query.filter(vesting_wallets::remaining_balance.gt(0));
        }
        if let Some(o) = owner {
            query = query.filter(vesting_wallets::owner_address.eq(o));
        }
        let results = query
            .order_by(vesting_wallets::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                vesting_wallets::wallet_id,
                vesting_wallets::owner_address,
                vesting_wallets::total_amount,
                vesting_wallets::claimed_amount,
                vesting_wallets::remaining_balance,
                vesting_wallets::start_time,
                vesting_wallets::duration,
                vesting_wallets::created_at,
            ))
            .load::<(
                String,
                String,
                i64,
                i64,
                i64,
                i64,
                i64,
                chrono::NaiveDateTime,
            )>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(
                    wallet_id,
                    owner_address,
                    total_amount,
                    claimed_amount,
                    remaining_balance,
                    start_time,
                    duration,
                    created_at,
                )| {
                    VestingWalletRow {
                        wallet_id,
                        owner_address,
                        total_amount,
                        claimed_amount,
                        remaining_balance,
                        start_time,
                        duration,
                        created_at,
                    }
                },
            )
            .collect())
    }

    pub async fn get_vesting_wallet_by_id(
        &self,
        wallet_id: &str,
    ) -> Result<Option<VestingWalletRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = vesting_wallets::table
            .filter(vesting_wallets::wallet_id.eq(wallet_id))
            .select((
                vesting_wallets::wallet_id,
                vesting_wallets::owner_address,
                vesting_wallets::total_amount,
                vesting_wallets::claimed_amount,
                vesting_wallets::remaining_balance,
                vesting_wallets::start_time,
                vesting_wallets::duration,
                vesting_wallets::created_at,
            ))
            .first::<(
                String,
                String,
                i64,
                i64,
                i64,
                i64,
                i64,
                chrono::NaiveDateTime,
            )>(&mut conn)
            .await
            .optional()?;
        Ok(result.map(
            |(
                wallet_id,
                owner_address,
                total_amount,
                claimed_amount,
                remaining_balance,
                start_time,
                duration,
                created_at,
            )| {
                VestingWalletRow {
                    wallet_id,
                    owner_address,
                    total_amount,
                    claimed_amount,
                    remaining_balance,
                    start_time,
                    duration,
                    created_at,
                }
            },
        ))
    }

    pub async fn get_vesting_wallet_events(
        &self,
        wallet_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VestingEventRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = vesting_events::table
            .filter(vesting_events::wallet_id.eq(wallet_id))
            .order_by(vesting_events::event_time.desc())
            .limit(limit)
            .offset(offset)
            .select((
                vesting_events::wallet_id,
                vesting_events::event_type,
                vesting_events::amount,
                vesting_events::event_time,
            ))
            .load::<(String, String, i64, i64)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(wallet_id, event_type, amount, event_time)| VestingEventRow {
                    wallet_id,
                    event_type,
                    amount,
                    event_time,
                },
            )
            .collect())
    }

    pub async fn get_vesting_claimable(
        &self,
        wallet_id: &str,
    ) -> Result<Option<i64>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = vesting_wallets::table
            .filter(vesting_wallets::wallet_id.eq(wallet_id))
            .select(vesting_wallets::remaining_balance)
            .first::<i64>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_user_vesting_wallets(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VestingWalletRow>, crate::error::SocialError> {
        self.list_vesting_wallets(false, Some(address), limit, offset)
            .await
    }

    pub async fn list_vesting_events(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VestingEventRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = vesting_events::table
            .order_by(vesting_events::event_time.desc())
            .limit(limit)
            .offset(offset)
            .select((
                vesting_events::wallet_id,
                vesting_events::event_type,
                vesting_events::amount,
                vesting_events::event_time,
            ))
            .load::<(String, String, i64, i64)>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(wallet_id, event_type, amount, event_time)| VestingEventRow {
                    wallet_id,
                    event_type,
                    amount,
                    event_time,
                },
            )
            .collect())
    }

    pub async fn get_vesting_analytics(
        &self,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        #[derive(QueryableByName)]
        struct CountRow {
            #[diesel(sql_type = BigInt)]
            count: i64,
        }
        #[derive(QueryableByName)]
        struct SumRow {
            #[diesel(sql_type = BigInt)]
            total: i64,
        }
        let wallets: i64 =
            diesel::sql_query("SELECT COUNT(*)::bigint as count FROM vesting_wallets")
                .get_result::<CountRow>(&mut conn)
                .await
                .map(|r| r.count)?;
        let total_vested: i64 = diesel::sql_query(
            "SELECT COALESCE(SUM(total_amount), 0)::bigint as total FROM vesting_wallets",
        )
        .get_result::<SumRow>(&mut conn)
        .await
        .map(|r| r.total)?;
        let total_claimed: i64 = diesel::sql_query(
            "SELECT COALESCE(SUM(claimed_amount), 0)::bigint as total FROM vesting_wallets",
        )
        .get_result::<SumRow>(&mut conn)
        .await
        .map(|r| r.total)?;
        Ok(serde_json::json!({
            "total_wallets": wallets,
            "total_vested": total_vested,
            "total_claimed": total_claimed,
            "total_remaining": total_vested - total_claimed,
        }))
    }

    pub async fn get_vesting_leaderboard(
        &self,
        limit: i64,
    ) -> Result<Vec<VestingWalletRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = vesting_wallets::table
            .order_by(vesting_wallets::remaining_balance.desc())
            .limit(limit)
            .select((
                vesting_wallets::wallet_id,
                vesting_wallets::owner_address,
                vesting_wallets::total_amount,
                vesting_wallets::claimed_amount,
                vesting_wallets::remaining_balance,
                vesting_wallets::start_time,
                vesting_wallets::duration,
                vesting_wallets::created_at,
            ))
            .load::<(
                String,
                String,
                i64,
                i64,
                i64,
                i64,
                i64,
                chrono::NaiveDateTime,
            )>(&mut conn)
            .await?;
        Ok(results
            .into_iter()
            .map(
                |(
                    wallet_id,
                    owner_address,
                    total_amount,
                    claimed_amount,
                    remaining_balance,
                    start_time,
                    duration,
                    created_at,
                )| {
                    VestingWalletRow {
                        wallet_id,
                        owner_address,
                        total_amount,
                        claimed_amount,
                        remaining_balance,
                        start_time,
                        duration,
                        created_at,
                    }
                },
            )
            .collect())
    }

    pub async fn get_spt_pool_by_associated_id(
        &self,
        associated_id: &str,
    ) -> Result<Option<SptPoolRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pool_id, token_type, owner, associated_id, symbol, name,
                   circulating_supply, base_price, quadratic_coefficient, created_at, time, transaction_id
            FROM (SELECT DISTINCT ON (pool_id) * FROM spt_pools WHERE associated_id = $1 ORDER BY pool_id, time DESC) p
        ";
        let result = diesel::sql_query(query)
            .bind::<Text, _>(associated_id)
            .get_result::<SptPoolRow>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_spt_popular(
        &self,
        limit: i64,
    ) -> Result<Vec<SptPoolRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT pool_id, token_type, owner, associated_id, symbol, name,
                   circulating_supply, base_price, quadratic_coefficient, created_at, time, transaction_id
            FROM (SELECT DISTINCT ON (pool_id) * FROM spt_pools ORDER BY pool_id, time DESC) p
            ORDER BY circulating_supply DESC
            LIMIT $1
        ";
        let results = diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .load::<SptPoolRow>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spt_user_holdings(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(String, i64, i64)>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = spt_holdings::table
            .filter(spt_holdings::holder_address.eq(address))
            .order_by(spt_holdings::acquired_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                spt_holdings::pool_id,
                spt_holdings::amount,
                spt_holdings::acquired_at,
            ))
            .load::<(String, i64, i64)>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_spt_user_reservations(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(String, i64, i64)>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = spt_reservations::table
            .filter(spt_reservations::reserver_address.eq(address))
            .order_by(spt_reservations::reserved_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                spt_reservations::pool_id,
                spt_reservations::amount,
                spt_reservations::reserved_at,
            ))
            .load::<(String, i64, i64)>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn search(
        &self,
        q: &str,
        limit: i64,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;

        let profiles = self.search_profiles_bm25(&mut conn, q, limit).await?;
        let posts = self.search_posts_bm25(&mut conn, q, limit).await?;
        let platforms_count: i64 = platforms::table
            .filter(platforms::name.ilike(&format!("%{}%", q)))
            .filter(platforms::deleted_at.is_null())
            .count()
            .get_result(&mut conn)
            .await?;

        Ok(serde_json::json!({
            "profiles": profiles,
            "posts": posts,
            "platforms_count": platforms_count,
        }))
    }

    async fn search_profiles_bm25(
        &self,
        conn: &mut myso_pg_db::Connection<'_>,
        q: &str,
        limit: i64,
    ) -> Result<Vec<Profile>, crate::error::SocialError> {
        let exact_match: Vec<Profile> = profiles::table
            .filter(profiles::owner_address.eq(q))
            .limit(1)
            .select(Profile::as_select())
            .load(conn)
            .await?;

        let bm25_query = r#"
            SELECT id, owner_address, username, display_name, bio, profile_photo, website,
                   created_at, updated_at, cover_photo, profile_id, followers_count, following_count,
                   blocked_count, post_count, min_offer_amount, birthdate, current_location, raised_location,
                   phone, email, gender, political_view, religion, education, primary_language,
                   relationship_status, x_username, facebook_username, reddit_username, github_username,
                   instagram_username, linkedin_username, twitch_username, social_proof_token_address,
                   reservation_pool_address, selected_badge_id, selected_ecosystem_badge_id,
                   paid_messaging_enabled, paid_messaging_min_cost
            FROM profiles
            WHERE search_text <@> to_bm25query($1, 'idx_profiles_search_bm25') < -0.1
            ORDER BY search_text <@> to_bm25query($1, 'idx_profiles_search_bm25')
            LIMIT $2
        "#;

        #[derive(QueryableByName)]
        struct ProfileRow {
            #[diesel(sql_type = Integer)]
            id: i32,
            #[diesel(sql_type = Text)]
            owner_address: String,
            #[diesel(sql_type = Text)]
            username: String,
            #[diesel(sql_type = Nullable<Text>)]
            display_name: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            bio: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            profile_photo: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            website: Option<String>,
            #[diesel(sql_type = Timestamp)]
            created_at: chrono::NaiveDateTime,
            #[diesel(sql_type = Timestamp)]
            updated_at: chrono::NaiveDateTime,
            #[diesel(sql_type = Nullable<Text>)]
            cover_photo: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            profile_id: Option<String>,
            #[diesel(sql_type = Integer)]
            followers_count: i32,
            #[diesel(sql_type = Integer)]
            following_count: i32,
            #[diesel(sql_type = Integer)]
            blocked_count: i32,
            #[diesel(sql_type = Integer)]
            post_count: i32,
            #[diesel(sql_type = Nullable<BigInt>)]
            min_offer_amount: Option<i64>,
            #[diesel(sql_type = Nullable<Text>)]
            birthdate: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            current_location: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            raised_location: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            phone: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            email: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            gender: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            political_view: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            religion: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            education: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            primary_language: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            relationship_status: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            x_username: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            facebook_username: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            reddit_username: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            github_username: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            instagram_username: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            linkedin_username: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            twitch_username: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            social_proof_token_address: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            reservation_pool_address: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            selected_badge_id: Option<String>,
            #[diesel(sql_type = Nullable<Text>)]
            selected_ecosystem_badge_id: Option<String>,
            #[diesel(sql_type = Bool)]
            paid_messaging_enabled: bool,
            #[diesel(sql_type = Nullable<BigInt>)]
            paid_messaging_min_cost: Option<i64>,
        }

        let bm25_profiles: Vec<ProfileRow> = diesel::sql_query(bm25_query)
            .bind::<Text, _>(q)
            .bind::<BigInt, _>(limit)
            .load(conn)
            .await?;

        let exact_ids: std::collections::HashSet<i32> = exact_match.iter().map(|p| p.id).collect();
        let bm25_profiles: Vec<Profile> = bm25_profiles
            .into_iter()
            .filter(|p| !exact_ids.contains(&p.id))
            .map(|p| Profile {
                id: p.id,
                owner_address: p.owner_address,
                username: p.username,
                display_name: p.display_name,
                bio: p.bio,
                profile_photo: p.profile_photo,
                website: p.website,
                created_at: p.created_at,
                updated_at: p.updated_at,
                cover_photo: p.cover_photo,
                profile_id: p.profile_id,
                followers_count: p.followers_count,
                following_count: p.following_count,
                blocked_count: p.blocked_count,
                post_count: p.post_count,
                min_offer_amount: p.min_offer_amount,
                birthdate: p.birthdate,
                current_location: p.current_location,
                raised_location: p.raised_location,
                phone: p.phone,
                email: p.email,
                gender: p.gender,
                political_view: p.political_view,
                religion: p.religion,
                education: p.education,
                primary_language: p.primary_language,
                relationship_status: p.relationship_status,
                x_username: p.x_username,
                facebook_username: p.facebook_username,
                reddit_username: p.reddit_username,
                github_username: p.github_username,
                instagram_username: p.instagram_username,
                linkedin_username: p.linkedin_username,
                twitch_username: p.twitch_username,
                social_proof_token_address: p.social_proof_token_address,
                reservation_pool_address: p.reservation_pool_address,
                selected_badge_id: p.selected_badge_id,
                selected_ecosystem_badge_id: p.selected_ecosystem_badge_id,
                paid_messaging_enabled: p.paid_messaging_enabled,
                paid_messaging_min_cost: p.paid_messaging_min_cost,
                search_text: None,
            })
            .collect();

        let mut results = exact_match;
        results.extend(bm25_profiles);
        results.truncate(limit as usize);
        Ok(results)
    }

    async fn search_posts_bm25(
        &self,
        conn: &mut myso_pg_db::Connection<'_>,
        q: &str,
        limit: i64,
    ) -> Result<Vec<PostBasicRow>, crate::error::SocialError> {
        let query = r#"
            WITH ranked AS (
                SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
                       reaction_count, comment_count, repost_count, tips_received,
                       content <@> to_bm25query($1, 'idx_posts_content_bm25') as score,
                       ROW_NUMBER() OVER (PARTITION BY post_id ORDER BY time DESC) as rn
                FROM posts
                WHERE deleted_at IS NULL
                  AND content <@> to_bm25query($1, 'idx_posts_content_bm25') < -0.1
            )
            SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
                   reaction_count, comment_count, repost_count, tips_received
            FROM ranked
            WHERE rn = 1
            ORDER BY score
            LIMIT $2
        "#;

        let results = diesel::sql_query(query)
            .bind::<Text, _>(q)
            .bind::<BigInt, _>(limit)
            .load::<PostBasicRow>(conn)
            .await?;

        Ok(results)
    }
}
