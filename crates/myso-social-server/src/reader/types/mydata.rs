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
pub struct MyDataQueryBroadPoolInfo {
    #[diesel(sql_type = Text)]
    pub pool_id: String,
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
pub struct MyDataQuerySubPoolInfo {
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
pub struct MyDataQueryListingSubPoolInfo {
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
pub struct MyDataQuerySnapshotAnchorInfo {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub snapshot_id: String,
    #[diesel(sql_type = Text)]
    pub buyer_address: String,
    #[diesel(sql_type = BigInt)]
    pub price_paid: i64,
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
pub struct MyDataQueryDistributionRoundInfo {
    #[diesel(sql_type = Text)]
    pub snapshot_id: String,
    #[diesel(sql_type = BigInt)]
    pub total_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub contributor_count: i64,
    #[diesel(sql_type = Text)]
    pub merkle_root: String,
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
pub struct MyDataQueryMerkleRootInfo {
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
pub struct MyDataQueryClaimInfo {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub snapshot_id: String,
    #[diesel(sql_type = Text)]
    pub claimant: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub claimed_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}
