// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text, Timestamptz};
use serde::{Deserialize, Serialize};

use super::revenue::{
    CONTENT_TYPE_DATA, CONTENT_TYPE_POST, CONTENT_TYPE_SERVICE, CONTENT_TYPE_TOKEN, CURRENCY_MYSO,
    REVENUE_SOURCE_MYDATA, REVENUE_SOURCE_POSTS, REVENUE_SOURCE_SPT, REVENUE_SOURCE_SUBSCRIPTION,
    REVENUE_SOURCE_TIPS,
};
use crate::schema::{
    ecosystem_treasury, social_proof_tokens_config, social_proof_tokens_events,
    spt_exchange_config, spt_holdings, spt_pools, spt_price_history, spt_reservation_pools,
    spt_reservations, spt_revenue, spt_transactions, unified_revenue,
};

pub const TOKEN_TYPE_PROFILE: i16 = 1;
pub const TOKEN_TYPE_POST: i16 = 2;
pub const TRANSACTION_TYPE_BUY: &str = "BUY";
pub const TRANSACTION_TYPE_SELL: &str = "SELL";
pub const RESERVATION_POOL_STATUS_ACTIVE: &str = "active";
pub const RESERVATION_POOL_STATUS_THRESHOLD_MET: &str = "threshold_met";

pub const DEFAULT_TRADING_CREATOR_FEE_BPS: i64 = 100;
pub const DEFAULT_TRADING_PLATFORM_FEE_BPS: i64 = 25;
pub const DEFAULT_TRADING_TREASURY_FEE_BPS: i64 = 25;
pub const DEFAULT_RESERVATION_CREATOR_FEE_BPS: i64 = 100;
pub const DEFAULT_RESERVATION_PLATFORM_FEE_BPS: i64 = 25;
pub const DEFAULT_RESERVATION_TREASURY_FEE_BPS: i64 = 25;
pub const MAX_HOLD_PERCENT_BPS: i64 = 500;
pub const DEFAULT_BASE_PRICE: i64 = 100_000_000;
pub const DEFAULT_QUADRATIC_COEFFICIENT: i64 = 100_000;
pub const DEFAULT_POST_THRESHOLD: i64 = 1_000_000_000_000;
pub const DEFAULT_PROFILE_THRESHOLD: i64 = 10_000_000_000_000;
pub const DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS: i64 = 2000;
pub const DEFAULT_MAX_RESERVERS_PER_POOL: i64 = 1000;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_pools)]
pub struct NewSptPool {
    pub pool_id: String,
    pub token_type: i16,
    pub owner: String,
    pub associated_id: String,
    pub symbol: String,
    pub name: String,
    pub circulating_supply: i64,
    pub base_price: i64,
    pub quadratic_coefficient: i64,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

/// Query result for SPT holdings by holder (JOIN with pools + profiles).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct SptHoldingRow {
    #[diesel(sql_type = Text)]
    pub holder_address: String,
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = BigInt)]
    pub balance: i64,
    #[diesel(sql_type = Text)]
    pub profile_owner_address: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_username: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_display_name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_photo: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_bio: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_selected_badge_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_social_proof_token_address: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_reservation_pool_address: Option<String>,
}

/// Query result for user reservation holdings (from user_reservation_holdings view + profiles).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct UserReservationHoldingRow {
    #[diesel(sql_type = Text)]
    pub reserver_address: String,
    #[diesel(sql_type = Text)]
    pub pool_id: String,
    #[diesel(sql_type = Text)]
    pub associated_id: String,
    #[diesel(sql_type = SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub reserved_at: i64,
    #[diesel(sql_type = BigInt)]
    pub total_reserved: i64,
    #[diesel(sql_type = BigInt)]
    pub required_threshold: i64,
    #[diesel(sql_type = Bool)]
    pub threshold_met: bool,
    #[diesel(sql_type = Text)]
    pub pool_status: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_username: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_display_name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_photo: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_social_proof_token_address: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub profile_reservation_pool_address: Option<String>,
}

/// Query result for SPT pool with latest price (JOIN with spt_price_history).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
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
    #[diesel(sql_type = BigInt)]
    pub price: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub price_24h_ago: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub volume_24h: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub creator_earnings: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub platform_earnings: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub ecosystem_earnings: Option<i64>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_holdings)]
pub struct NewSptHolding {
    pub pool_id: String,
    pub holder_address: String,
    pub amount: i64,
    pub acquired_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = spt_transactions)]
pub struct SptTransaction {
    pub id: i32,
    pub pool_id: String,
    pub transaction_type: String,
    pub sender: String,
    pub amount: i64,
    pub myso_amount: i64,
    pub fee_amount: i64,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub price: i64,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_transactions)]
pub struct NewSptTransaction {
    pub pool_id: String,
    pub transaction_type: String,
    pub sender: String,
    pub amount: i64,
    pub myso_amount: i64,
    pub fee_amount: i64,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub price: i64,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_reservation_pools)]
pub struct NewSptReservationPool {
    pub pool_id: String,
    pub associated_id: String,
    pub token_type: i16,
    pub owner: String,
    pub total_reserved: i64,
    pub required_threshold: i64,
    pub status: String,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_reservations)]
pub struct NewSptReservation {
    pub pool_id: String,
    pub reserver_address: String,
    pub amount: i64,
    pub reserved_at: i64,
    pub fee_amount: Option<i64>,
    pub creator_fee: Option<i64>,
    pub platform_fee: Option<i64>,
    pub treasury_fee: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ecosystem_treasury)]
pub struct EcosystemTreasury {
    pub id: i32,
    pub treasury_address: String,
    pub updated_by: String,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = ecosystem_treasury)]
pub struct NewEcosystemTreasury {
    pub treasury_address: String,
    pub updated_by: String,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewEcosystemTreasury {
    pub fn from_event(
        treasury_address: String,
        updated_by: String,
        timestamp_ms: u64,
        transaction_id: String,
    ) -> Self {
        let timestamp_secs = (timestamp_ms / 1000) as i64;
        let time = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp_secs, 0)
            .unwrap_or_else(chrono::Utc::now);

        Self {
            treasury_address,
            updated_by,
            timestamp_ms: timestamp_ms as i64,
            time,
            transaction_id,
        }
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_exchange_config)]
pub struct NewSptExchangeConfig {
    pub updated_by: String,
    pub post_threshold: i64,
    pub profile_threshold: i64,
    pub max_individual_reservation_bps: i64,
    pub total_fee_bps: i64,
    pub creator_fee_bps: i64,
    pub platform_fee_bps: i64,
    pub treasury_fee_bps: i64,
    pub trading_creator_fee_bps: i64,
    pub trading_platform_fee_bps: i64,
    pub trading_treasury_fee_bps: i64,
    pub reservation_creator_fee_bps: i64,
    pub reservation_platform_fee_bps: i64,
    pub reservation_treasury_fee_bps: i64,
    pub max_reservers_per_pool: i64,
    pub base_price: i64,
    pub quadratic_coefficient: i64,
    pub max_hold_percent_bps: i64,
    pub trading_enabled: bool,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = spt_price_history)]
pub struct SptPriceHistory {
    pub id: i32,
    pub pool_id: String,
    pub price: i64,
    pub circulating_supply: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_price_history)]
pub struct NewSptPriceHistory {
    pub pool_id: String,
    pub price: i64,
    pub circulating_supply: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_revenue)]
pub struct NewSptRevenue {
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
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewSptRevenue {
    pub fn from_buy_event(
        pool_id: String,
        trader: String,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        token_amount: i64,
        myso_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            pool_id,
            transaction_type: TRANSACTION_TYPE_BUY.to_string(),
            trader,
            creator_address,
            platform_address,
            treasury_address,
            creator_fee,
            platform_fee,
            treasury_fee,
            total_fee: creator_fee + platform_fee + treasury_fee,
            token_amount,
            myso_amount,
            token_price,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }

    pub fn from_sell_event(
        pool_id: String,
        trader: String,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        token_amount: i64,
        myso_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            pool_id,
            transaction_type: TRANSACTION_TYPE_SELL.to_string(),
            trader,
            creator_address,
            platform_address,
            treasury_address,
            creator_fee,
            platform_fee,
            treasury_fee,
            total_fee: creator_fee + platform_fee + treasury_fee,
            token_amount,
            myso_amount,
            token_price,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = unified_revenue)]
pub struct UnifiedRevenue {
    pub revenue_source: String,
    pub revenue_type: String,
    pub creator_address: String,
    pub platform_address: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub payer_address: String,
    pub recipient_address: String,
    pub revenue_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = unified_revenue)]
pub struct NewUnifiedRevenue {
    pub revenue_source: String,
    pub revenue_type: String,
    pub creator_address: String,
    pub platform_address: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub payer_address: String,
    pub recipient_address: String,
    pub revenue_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewUnifiedRevenue {
    pub fn from_tip(
        revenue_type: String,
        creator_address: String,
        amount: i64,
        content_id: String,
        content_type: String,
        payer_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_TIPS.to_string(),
            revenue_type,
            creator_address: creator_address.clone(),
            platform_address: None,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(content_id),
            content_type: Some(content_type),
            payer_address,
            recipient_address: creator_address,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }

    pub fn from_spt(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        pool_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_SPT.to_string(),
            revenue_type,
            creator_address,
            platform_address,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(pool_id),
            content_type: Some(CONTENT_TYPE_TOKEN.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }

    pub fn from_subscription(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        service_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_SUBSCRIPTION.to_string(),
            revenue_type,
            creator_address,
            platform_address,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(service_id),
            content_type: Some(CONTENT_TYPE_SERVICE.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }

    pub fn from_mydata(
        revenue_type: String,
        creator_address: String,
        amount: i64,
        mydata_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_MYDATA.to_string(),
            revenue_type,
            creator_address,
            platform_address: None,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(mydata_id),
            content_type: Some(CONTENT_TYPE_DATA.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }

    pub fn from_post(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        post_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_POSTS.to_string(),
            revenue_type,
            creator_address,
            platform_address,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(post_id),
            content_type: Some(CONTENT_TYPE_POST.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_proof_tokens_config)]
pub struct NewSocialProofTokensConfig {
    pub trading_enabled: bool,
    pub admin_address: String,
    pub reason: String,
    pub timestamp_ms: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_proof_tokens_events)]
pub struct NewSocialProofTokensEvent {
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
