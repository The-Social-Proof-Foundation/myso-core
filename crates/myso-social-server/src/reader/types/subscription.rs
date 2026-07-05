// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Bool, Nullable, Text, Timestamptz};
use diesel::QueryableByName;
use serde::Serialize;

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
    pub payment_time: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Serialize)]
pub struct SubscriberSummaryRow {
    pub active_subscriptions: i64,
    pub total_revenue: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub billing_period_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub max_renewal_months: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub ecosystem_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_creator_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_treasury_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
}
