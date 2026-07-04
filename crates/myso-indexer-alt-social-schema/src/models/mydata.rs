// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Date, Int4, Jsonb, Nullable, Text, Timestamptz};
use serde::{Deserialize, Serialize};

use crate::schema::{
    mydata_access_logs, mydata_broad_pools, mydata_claims, mydata_config, mydata_data,
    mydata_distribution_rounds, mydata_listing_sub_pools, mydata_merkle_roots, mydata_purchases,
    mydata_registry, mydata_revenue, mydata_snapshot_anchors, mydata_sub_pools,
    mydata_subscriptions,
};

// Constants matching social_contracts::mydata Move module
pub const PURCHASE_TYPE_ONE_TIME: &str = "one_time";
pub const PURCHASE_TYPE_SUBSCRIPTION: &str = "subscription";
pub const REVENUE_TYPE_ONE_TIME: &str = "one_time";
pub const REVENUE_TYPE_SUBSCRIPTION: &str = "subscription";
pub const REVENUE_TYPE_GRANT: &str = "grant";
pub const ACCESS_TYPE_ONE_TIME: &str = "one_time";
pub const ACCESS_TYPE_SUBSCRIPTION: &str = "subscription";
pub const ACCESS_TYPE_GRANT: &str = "grant";
pub const ACCESS_TYPE_PREVIEW: &str = "preview";
pub const ACCESS_TYPE_PRICING_UPDATE: &str = "pricing_update";
pub const ACCESS_TYPE_CONTENT_UPDATE: &str = "content_update";
pub const ACCESS_TYPE_REVOKED: &str = "revoked";
pub const DATA_QUALITY_HIGH: &str = "high";
pub const DATA_QUALITY_MEDIUM: &str = "medium";
pub const DATA_QUALITY_LOW: &str = "low";
pub const UPDATE_FREQUENCY_HOURLY: &str = "hourly";
pub const UPDATE_FREQUENCY_DAILY: &str = "daily";
pub const UPDATE_FREQUENCY_WEEKLY: &str = "weekly";
pub const UPDATE_FREQUENCY_MONTHLY: &str = "monthly";
pub const UPDATE_FREQUENCY_YEARLY: &str = "yearly";
pub const MAX_TAGS: usize = 10;
pub const MAX_SUBSCRIPTION_DAYS: i64 = 365;
pub const MAX_FREE_ACCESS_GRANTS: i64 = 100_000;

/// Query result for a mydata record (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataRecordRow {
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

/// Query result for a mydata subscription (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataSubscriptionRow {
    #[diesel(sql_type = Int4)]
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

/// Query result for a mydata revenue entry (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataRevenueRow {
    #[diesel(sql_type = Int4)]
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

/// Query result for a mydata access log (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataAccessLogRow {
    #[diesel(sql_type = Int4)]
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

/// Query result for mydata stats (aggregated).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataStatsRow {
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

/// Query result for daily revenue (time_bucket aggregate).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataDailyRevenueRow {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = BigInt)]
    pub daily_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub daily_transactions: i64,
}

/// Query result for access analytics (time_bucket aggregate).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataAccessAnalyticsRow {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = Text)]
    pub access_type: String,
    #[diesel(sql_type = BigInt)]
    pub unique_users: i64,
    #[diesel(sql_type = BigInt)]
    pub total_accesses: i64,
}

/// Query result for a mydata purchase (for GraphQL/reader).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataPurchaseRow {
    #[diesel(sql_type = Int4)]
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
    #[diesel(sql_type = Bool)]
    pub revoked: bool,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub revoked_at: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub revoked_by: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_data)]
pub struct MyDataData {
    pub mydata_id: String,
    pub owner: String,
    pub media_type: String,
    pub tags: serde_json::Value,
    pub platform_id: Option<String>,
    pub timestamp_start: i64,
    pub timestamp_end: Option<i64>,
    pub created_at: i64,
    pub last_updated: i64,
    pub one_time_price: Option<i64>,
    pub subscription_price: Option<i64>,
    pub subscription_duration_days: i64,
    pub geographic_region: Option<String>,
    pub data_quality: Option<String>,
    pub sample_size: Option<i64>,
    pub collection_method: Option<String>,
    pub is_updating: bool,
    pub update_frequency: Option<String>,
    pub version: i64,
    pub encrypted_content_hash: Option<String>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_data)]
pub struct NewMyDataData {
    pub mydata_id: String,
    pub owner: String,
    pub media_type: String,
    pub tags: serde_json::Value,
    pub platform_id: Option<String>,
    pub timestamp_start: i64,
    pub timestamp_end: Option<i64>,
    pub created_at: i64,
    pub last_updated: i64,
    pub one_time_price: Option<i64>,
    pub subscription_price: Option<i64>,
    pub subscription_duration_days: i64,
    pub geographic_region: Option<String>,
    pub data_quality: Option<String>,
    pub sample_size: Option<i64>,
    pub collection_method: Option<String>,
    pub is_updating: bool,
    pub update_frequency: Option<String>,
    pub version: i64,
    pub encrypted_content_hash: Option<String>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_purchases)]
pub struct MyDataPurchase {
    pub id: i32,
    pub mydata_id: String,
    pub buyer: String,
    pub price: i64,
    pub purchase_type: String,
    pub purchase_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub revoked_by: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_purchases)]
pub struct NewMyDataPurchase {
    pub mydata_id: String,
    pub buyer: String,
    pub price: i64,
    pub purchase_type: String,
    pub purchase_time: i64,
    pub transaction_id: String,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_subscriptions)]
pub struct MyDataSubscription {
    pub id: i32,
    pub mydata_id: String,
    pub subscriber: String,
    pub subscription_start: i64,
    pub subscription_end: i64,
    pub price: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub revoked_by: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_subscriptions)]
pub struct NewMyDataSubscription {
    pub mydata_id: String,
    pub subscriber: String,
    pub subscription_start: i64,
    pub subscription_end: i64,
    pub price: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_revenue)]
pub struct MyDataRevenue {
    pub id: i32,
    pub mydata_id: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub revenue_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_revenue)]
pub struct NewMyDataRevenue {
    pub mydata_id: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub revenue_time: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_access_logs)]
pub struct MyDataAccessLog {
    pub id: i32,
    pub mydata_id: String,
    pub user_address: String,
    pub access_type: String,
    pub access_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_access_logs)]
pub struct NewMyDataAccessLog {
    pub mydata_id: String,
    pub user_address: String,
    pub access_type: String,
    pub access_time: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_registry)]
pub struct MyDataRegistry {
    pub mydata_id: String,
    pub owner: String,
    pub registered_at: i64,
    pub unregistered_at: Option<i64>,
    pub is_active: bool,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_registry)]
pub struct NewMyDataRegistry {
    pub mydata_id: String,
    pub owner: String,
    pub registered_at: i64,
    pub unregistered_at: Option<i64>,
    pub is_active: bool,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_config)]
pub struct MyDataConfig {
    pub id: i32,
    pub updated_by: String,
    pub enable_flag: bool,
    pub max_tags: i64,
    pub max_subscription_days: i64,
    pub max_free_access_grants: i64,
    pub max_encryption_id_bytes: i64,
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_config)]
pub struct NewMyDataConfig {
    pub updated_by: String,
    pub enable_flag: bool,
    pub max_tags: i64,
    pub max_subscription_days: i64,
    pub max_free_access_grants: i64,
    pub max_encryption_id_bytes: i64,
    pub version: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}

// --- MyData marketplace (indexed from social_contracts::mydata) ---

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_broad_pools)]
pub struct MyDataBroadPool {
    pub pool_id: String,
    pub name: String,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_broad_pools)]
pub struct NewMyDataBroadPool {
    pub pool_id: String,
    pub name: String,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_sub_pools)]
pub struct MyDataSubPool {
    pub sub_pool_id: String,
    pub broad_pool_id: String,
    pub name: String,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_sub_pools)]
pub struct NewMyDataSubPool {
    pub sub_pool_id: String,
    pub broad_pool_id: String,
    pub name: String,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_listing_sub_pools)]
pub struct MyDataListingSubPool {
    pub listing_id: String,
    pub sub_pool_id: String,
    pub assigned_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_listing_sub_pools)]
pub struct NewMyDataListingSubPool {
    pub listing_id: String,
    pub sub_pool_id: String,
    pub assigned_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_merkle_roots)]
pub struct MyDataMerkleRoot {
    pub snapshot_id: String,
    pub root_hash: String,
    pub published_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_merkle_roots)]
pub struct NewMyDataMerkleRoot {
    pub snapshot_id: String,
    pub root_hash: String,
    pub published_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_snapshot_anchors)]
pub struct MyDataSnapshotAnchor {
    pub id: i32,
    pub snapshot_id: String,
    pub buyer_address: String,
    pub price_paid: i64,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub manifest_hash: Option<String>,
    pub payment_reference: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_snapshot_anchors)]
pub struct NewMyDataSnapshotAnchor {
    pub snapshot_id: String,
    pub buyer_address: String,
    pub price_paid: i64,
    pub created_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub manifest_hash: Option<String>,
    pub payment_reference: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_distribution_rounds)]
pub struct MyDataDistributionRound {
    pub snapshot_id: String,
    pub total_amount: i64,
    pub contributor_count: i64,
    pub merkle_root: String,
    pub published_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_distribution_rounds)]
pub struct NewMyDataDistributionRound {
    pub snapshot_id: String,
    pub total_amount: i64,
    pub contributor_count: i64,
    pub merkle_root: String,
    pub published_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = mydata_claims)]
pub struct MyDataClaim {
    pub id: i32,
    pub snapshot_id: String,
    pub claimant: String,
    pub amount: i64,
    pub claimed_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_claims)]
pub struct NewMyDataClaim {
    pub snapshot_id: String,
    pub claimant: String,
    pub amount: i64,
    pub claimed_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataBroadPoolRow {
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

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataSubPoolRow {
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

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataListingSubPoolRow {
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

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataSnapshotAnchorRow {
    #[diesel(sql_type = Int4)]
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

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataDistributionRoundRow {
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

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataMerkleRootRow {
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

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MyDataClaimRow {
    #[diesel(sql_type = Int4)]
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
