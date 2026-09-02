// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    profile_subscription_plans, profile_subscription_services, profile_subscriptions,
    subscription_access_logs, subscription_config, subscription_events, subscription_revenue,
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
    pub plan_count: i64,
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
    pub plan_count: i64,
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
    pub plan_count: Option<i64>,
    pub active: Option<bool>,
    pub subscriber_count: Option<i64>,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_subscription_plans)]
pub struct ProfileSubscriptionPlan {
    pub plan_id: String,
    pub service_id: String,
    pub title: String,
    pub description: Option<String>,
    pub price: i64,
    pub duration_ms: i64,
    pub tier_level: Option<i64>,
    pub platform_id: Option<String>,
    pub coin_type: String,
    pub active: bool,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_subscription_plans)]
pub struct NewProfileSubscriptionPlan {
    pub plan_id: String,
    pub service_id: String,
    pub title: String,
    pub description: Option<String>,
    pub price: i64,
    pub duration_ms: i64,
    pub tier_level: Option<i64>,
    pub platform_id: Option<String>,
    pub coin_type: String,
    pub active: bool,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = profile_subscription_plans)]
pub struct UpdateProfileSubscriptionPlan {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<i64>,
    pub duration_ms: Option<i64>,
    pub tier_level: Option<i64>,
    pub platform_id: Option<String>,
    pub coin_type: Option<String>,
    pub active: Option<bool>,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profile_subscriptions)]
pub struct ProfileSubscription {
    pub subscription_id: String,
    pub service_id: String,
    pub plan_id: String,
    pub tier_level: Option<i64>,
    pub platform_id: Option<String>,
    pub price: i64,
    pub duration_ms: i64,
    pub subscriber: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub auto_renew: bool,
    pub renewal_balance: i64,
    pub renewal_count: i64,
    pub coin_type: String,
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
    pub plan_id: String,
    pub tier_level: Option<i64>,
    pub platform_id: Option<String>,
    pub price: i64,
    pub duration_ms: i64,
    pub subscriber: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub auto_renew: bool,
    pub renewal_balance: i64,
    pub renewal_count: i64,
    pub coin_type: String,
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
    pub plan_id: Option<String>,
    pub tier_level: Option<i64>,
    pub platform_id: Option<String>,
    pub price: Option<i64>,
    pub duration_ms: Option<i64>,
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
    pub platform_fee: i64,
    pub ecosystem_fee: i64,
    pub creator_amount: i64,
    pub platform_address: Option<String>,
    pub revenue_type: String,
    pub coin_type: String,
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
    pub platform_fee: i64,
    pub ecosystem_fee: i64,
    pub creator_amount: i64,
    pub platform_address: Option<String>,
    pub revenue_type: String,
    pub coin_type: String,
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

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = subscription_config)]
pub struct NewSubscriptionConfig {
    pub updated_by: String,
    pub default_billing_period_ms: i64,
    pub max_renewal_months: i64,
    pub platform_fee_bps: i64,
    pub ecosystem_fee_bps: i64,
    pub non_platform_platform_to_creator_bps: i64,
    pub non_platform_platform_to_treasury_bps: i64,
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewSubscriptionConfig {
    pub fn from_event(
        updated_by: String,
        default_billing_period_ms: u64,
        max_renewal_months: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        version: u64,
        updated_at: u64,
        transaction_id: String,
    ) -> Self {
        let time = chrono::DateTime::<chrono::Utc>::from_timestamp((updated_at / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now);
        Self {
            updated_by,
            default_billing_period_ms: default_billing_period_ms as i64,
            max_renewal_months: max_renewal_months as i64,
            platform_fee_bps: platform_fee_bps as i64,
            ecosystem_fee_bps: ecosystem_fee_bps as i64,
            non_platform_platform_to_creator_bps: non_platform_platform_to_creator_bps as i64,
            non_platform_platform_to_treasury_bps: non_platform_platform_to_treasury_bps as i64,
            version: version as i64,
            updated_at: updated_at as i64,
            time,
            transaction_id,
        }
    }
}
