// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Bool, Date, Integer, Jsonb, Nullable, Text, Timestamptz};
use diesel::QueryableByName;
use serde::Serialize;

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
    #[diesel(sql_type = Nullable<Text>)]
    pub collection_method: Option<String>,
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
    /// Whether buyers may start new broad-pool/snapshot marketplace rounds.
    pub marketplace_enabled: bool,
    #[diesel(sql_type = BigInt)]
    pub max_tags: i64,
    #[diesel(sql_type = BigInt)]
    pub max_subscription_days: i64,
    #[diesel(sql_type = BigInt)]
    pub max_free_access_grants: i64,
    #[diesel(sql_type = BigInt)]
    pub max_encryption_id_bytes: i64,
    #[diesel(sql_type = BigInt)]
    pub p2p_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub p2p_ecosystem_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub mydata_marketplace_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub mydata_marketplace_ecosystem_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_creator_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_treasury_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
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
    #[diesel(sql_type = BigInt)]
    pub platform_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub ecosystem_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub creator_amount: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_address: Option<String>,
    #[diesel(sql_type = Text)]
    pub purchase_type: String,
    #[diesel(sql_type = BigInt)]
    pub purchase_time: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Bool)]
    pub revoked: bool,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub revoked_at: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub revoked_by: Option<String>,
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
    #[diesel(sql_type = Bool)]
    pub revoked: bool,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub revoked_at: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub revoked_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MyDataHasAccessResponse {
    pub mydata_id: String,
    pub user_address: String,
    pub has_access: bool,
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
    #[diesel(sql_type = BigInt)]
    pub platform_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub ecosystem_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub creator_amount: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_address: Option<String>,
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
pub struct MyDataBroadPoolInfo {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_address: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub created_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataSubPoolInfo {
    #[diesel(sql_type = Text)]
    pub sub_pool_id: String,
    #[diesel(sql_type = Text)]
    pub broad_pool_id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = BigInt)]
    pub created_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataListingSubPoolInfo {
    #[diesel(sql_type = Text)]
    pub listing_id: String,
    #[diesel(sql_type = Text)]
    pub sub_pool_id: String,
    #[diesel(sql_type = BigInt)]
    pub assigned_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataSnapshotAnchorInfo {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub snapshot_id: String,
    #[diesel(sql_type = Text)]
    pub buyer_address: String,
    #[diesel(sql_type = BigInt)]
    pub price_paid: i64,
    #[diesel(sql_type = Text)]
    pub source_pool_id: String,
    #[diesel(sql_type = Text)]
    pub source_sub_pool_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_address: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub initial_escrow: i64,
    #[diesel(sql_type = BigInt)]
    pub created_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Nullable<Text>)]
    pub manifest_hash: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub payment_reference: Option<String>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataDistributionRoundInfo {
    #[diesel(sql_type = Text)]
    pub snapshot_id: String,
    #[diesel(sql_type = BigInt)]
    pub total_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub contributor_count: i64,
    #[diesel(sql_type = Text)]
    pub merkle_root: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_address: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub claim_deadline_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub published_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataSnapshotEscrowInfo {
    #[diesel(sql_type = Text)]
    pub snapshot_id: String,
    #[diesel(sql_type = BigInt)]
    pub total_funded: i64,
    #[diesel(sql_type = BigInt)]
    pub total_claimed: i64,
    #[diesel(sql_type = BigInt)]
    pub remaining_amount: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub claim_deadline_ms: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub reclaimed_at_ms: Option<i64>,
    #[diesel(sql_type = Text)]
    pub status: String,
    #[diesel(sql_type = BigInt)]
    pub updated_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataMerkleRootInfo {
    #[diesel(sql_type = Text)]
    pub snapshot_id: String,
    #[diesel(sql_type = Text)]
    pub root_hash: String,
    #[diesel(sql_type = BigInt)]
    pub published_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct MyDataClaimInfo {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub snapshot_id: String,
    #[diesel(sql_type = Text)]
    pub claimant: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub gross_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub ecosystem_fee: i64,
    #[diesel(sql_type = BigInt)]
    pub net_amount: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_address: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub claimed_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}
