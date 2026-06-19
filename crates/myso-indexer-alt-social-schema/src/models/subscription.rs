// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    profile_subscription_services, profile_subscriptions, subscription_access_logs,
    subscription_events, subscription_revenue,
};

pub const MIN_SUBSCRIPTION_DURATION_DAYS: i64 = 1;
pub const MAX_SUBSCRIPTION_DURATION_DAYS: i64 = 365;
pub const MILLISECONDS_PER_DAY: i64 = 24 * 60 * 60 * 1000;
pub const MAX_RENEWAL_MONTHS: i32 = 120;
pub const THIRTY_DAYS_MS: i64 = 2_592_000_000;

pub const REVENUE_TYPE_RENEWAL: &str = "renewal";
pub const REVENUE_TYPE_AUTO_RENEWAL: &str = "auto_renewal";
pub const REVENUE_TYPE_REFUND: &str = "refund";

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_subscription_services)]
pub struct ProfileSubscriptionService {
    pub service_id: String,
    pub profile_owner: String,
    pub profile_id: String,
    pub monthly_fee: i64,
    pub active: bool,
    pub subscriber_count: i64,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_subscription_services)]
pub struct NewProfileSubscriptionService {
    pub service_id: String,
    pub profile_owner: String,
    pub profile_id: String,
    pub monthly_fee: i64,
    pub active: bool,
    pub subscriber_count: i64,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = profile_subscription_services)]
pub struct UpdateProfileSubscriptionService {
    pub monthly_fee: Option<i64>,
    pub active: Option<bool>,
    pub subscriber_count: Option<i64>,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_subscriptions)]
pub struct ProfileSubscription {
    pub subscription_id: String,
    pub service_id: String,
    pub subscriber: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub auto_renew: bool,
    pub renewal_balance: i64,
    pub renewal_count: i64,
    pub cancelled_at: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_subscriptions)]
pub struct NewProfileSubscription {
    pub subscription_id: String,
    pub service_id: String,
    pub subscriber: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub auto_renew: bool,
    pub renewal_balance: i64,
    pub renewal_count: i64,
    pub cancelled_at: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = profile_subscriptions)]
pub struct UpdateProfileSubscription {
    pub expires_at: Option<i64>,
    pub auto_renew: Option<bool>,
    pub renewal_balance: Option<i64>,
    pub renewal_count: Option<i64>,
    pub cancelled_at: Option<i64>,
    pub processing_success: Option<bool>,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = subscription_events)]
pub struct SubscriptionEvent {
    pub event_type: String,
    pub subscription_id: Option<String>,
    pub service_id: Option<String>,
    pub subscriber: Option<String>,
    pub event_data: serde_json::Value,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = subscription_events)]
pub struct NewSubscriptionEvent {
    pub event_type: String,
    pub subscription_id: Option<String>,
    pub service_id: Option<String>,
    pub subscriber: Option<String>,
    pub event_data: serde_json::Value,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = subscription_revenue)]
pub struct SubscriptionRevenue {
    pub service_id: String,
    pub subscription_id: Option<String>,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub payment_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = subscription_revenue)]
pub struct NewSubscriptionRevenue {
    pub service_id: String,
    pub subscription_id: Option<String>,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub payment_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = subscription_access_logs)]
pub struct SubscriptionAccessLog {
    pub subscription_id: String,
    pub subscriber: String,
    pub content_type: String,
    pub content_id: String,
    pub access_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = subscription_access_logs)]
pub struct NewSubscriptionAccessLog {
    pub subscription_id: String,
    pub subscriber: String,
    pub content_type: String,
    pub content_id: String,
    pub access_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}
