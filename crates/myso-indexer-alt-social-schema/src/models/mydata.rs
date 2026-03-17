// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    mydata_access_logs, mydata_config, mydata_data, mydata_purchases, mydata_registry,
    mydata_revenue, mydata_subscriptions,
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
pub const MILLISECONDS_PER_DAY: i64 = 86_400_000;
pub const MAX_FREE_ACCESS_GRANTS: i64 = 100_000;

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
    pub ip_id: String,
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
    pub ip_id: String,
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
    pub timestamp_ms: i64,
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
    pub timestamp_ms: i64,
    pub transaction_id: String,
}
