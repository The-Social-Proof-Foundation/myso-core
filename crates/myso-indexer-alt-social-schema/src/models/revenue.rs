// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, NaiveDate, Utc};
use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Date, Double, Nullable, Text};
use serde::{Deserialize, Serialize};

use crate::schema::spt_revenue;

pub const SPT_TRANSACTION_TYPE_BUY: &str = "buy";
pub const SPT_TRANSACTION_TYPE_SELL: &str = "sell";

pub const REVENUE_SOURCE_SUBSCRIPTION: &str = "subscription";
pub const REVENUE_SOURCE_MYDATA: &str = "mydata";
pub const REVENUE_SOURCE_SPT: &str = "spt";
pub const REVENUE_SOURCE_TIPS: &str = "tips";
pub const REVENUE_SOURCE_POSTS: &str = "posts";
pub const REVENUE_SOURCE_MESSAGING: &str = "messaging";

pub const REVENUE_TYPE_SUBSCRIPTION_MONTHLY: &str = "monthly";
pub const REVENUE_TYPE_SUBSCRIPTION_RENEWAL: &str = "renewal";
pub const REVENUE_TYPE_SUBSCRIPTION_AUTO_RENEWAL: &str = "auto_renewal";
pub const REVENUE_TYPE_SUBSCRIPTION_REFUND: &str = "refund";

pub const REVENUE_TYPE_MYDATA_ONE_TIME: &str = "one_time";
pub const REVENUE_TYPE_MYDATA_SUBSCRIPTION: &str = "subscription";
pub const REVENUE_TYPE_MYDATA_GRANT: &str = "grant";
pub const REVENUE_TYPE_SUBSCRIPTION_CREATOR_AMOUNT: &str = "creator_amount";
pub const REVENUE_TYPE_SUBSCRIPTION_PLATFORM_FEE: &str = "platform_fee";
pub const REVENUE_TYPE_SUBSCRIPTION_ECOSYSTEM_FEE: &str = "ecosystem_fee";

pub const REVENUE_TYPE_MYDATA_CREATOR_AMOUNT: &str = "creator_amount";
pub const REVENUE_TYPE_MYDATA_PLATFORM_FEE: &str = "platform_fee";
pub const REVENUE_TYPE_MYDATA_ECOSYSTEM_FEE: &str = "ecosystem_fee";
pub const REVENUE_TYPE_MYDATA_MARKETPLACE_CLAIM: &str = "mydata_marketplace_claim";

pub const REVENUE_TYPE_SPT_CREATOR_FEE: &str = "creator_fee";
pub const REVENUE_TYPE_SPT_PLATFORM_FEE: &str = "platform_fee";
pub const REVENUE_TYPE_SPT_TREASURY_FEE: &str = "treasury_fee";

pub const REVENUE_TYPE_TIPS_POST: &str = "post_tip";
pub const REVENUE_TYPE_TIPS_PROFILE: &str = "profile_tip";
pub const REVENUE_TYPE_TIPS_COMMENT: &str = "comment_tip";

pub const REVENUE_TYPE_POSTS_MONETIZATION: &str = "post_monetization";
pub const REVENUE_TYPE_POSTS_PREMIUM: &str = "premium_content";

pub const REVENUE_TYPE_MESSAGING_CLAIM: &str = "messaging_claim";
pub const REVENUE_TYPE_MESSAGING_NET: &str = "messaging_net";
pub const REVENUE_TYPE_MESSAGING_PLATFORM_FEE: &str = "messaging_platform_fee";
pub const REVENUE_TYPE_MESSAGING_TREASURY_FEE: &str = "messaging_treasury_fee";
pub const REVENUE_TYPE_MESSAGING_REFUND: &str = "messaging_refund";

pub const CONTENT_TYPE_POST: &str = "post";
pub const CONTENT_TYPE_PROFILE: &str = "profile";
pub const CONTENT_TYPE_SERVICE: &str = "service";
pub const CONTENT_TYPE_DATA: &str = "data";
pub const CONTENT_TYPE_TOKEN: &str = "token";
pub const CONTENT_TYPE_COMMENT: &str = "comment";
pub const CONTENT_TYPE_MESSAGING: &str = "messaging";

pub const CURRENCY_MYSO: &str = "MYSO";

pub const MYSO_DECIMAL_PLACES: u32 = 9;
pub const MYSO_DECIMAL_FACTOR: i64 = 1_000_000_000;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = spt_revenue)]
pub struct SptRevenue {
    pub pool_id: String,
    pub transaction_type: String,
    pub trader: String,
    pub creator_address: String,
    pub platform_address: String,
    pub treasury_address: String,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub total_fee: i64,
    pub token_amount: i64,
    pub myso_amount: i64,
    pub token_price: i64,
    pub revenue_time: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

pub fn myso_from_blockchain_units(amount: i64) -> f64 {
    amount as f64 / MYSO_DECIMAL_FACTOR as f64
}

pub fn myso_to_blockchain_units(amount: f64) -> i64 {
    (amount * MYSO_DECIMAL_FACTOR as f64) as i64
}

pub fn format_myso_amount(amount: i64) -> String {
    let decimal_amount = myso_from_blockchain_units(amount);
    format!("{:.4} MYSO", decimal_amount)
}

pub fn calculate_percentage(part: i64, total: i64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

pub fn calculate_growth_rate(current: i64, previous: i64) -> Option<f64> {
    if previous == 0 {
        None
    } else {
        Some(((current - previous) as f64 / previous as f64) * 100.0)
    }
}

/// Query result for platform_revenue_summary view (12-month revenue metrics per platform).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct PlatformRevenueSummaryRow {
    #[diesel(sql_type = Text)]
    pub platform_address: String,
    #[diesel(sql_type = BigInt)]
    pub total_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub total_subscription_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub total_mydata_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub total_spt_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub total_messaging_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub total_transactions: i64,
    #[diesel(sql_type = BigInt)]
    pub total_creators: i64,
    #[diesel(sql_type = BigInt)]
    pub total_payers: i64,
    #[diesel(sql_type = Double)]
    pub avg_transaction_amount: f64,
    #[diesel(sql_type = BigInt)]
    pub active_months: i64,
    #[diesel(sql_type = Nullable<Date>)]
    pub last_active_month: Option<NaiveDate>,
}
