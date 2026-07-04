// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text, Timestamptz};
use serde::{Deserialize, Serialize};

use super::revenue::{
    CONTENT_TYPE_DATA, CONTENT_TYPE_MESSAGING, CONTENT_TYPE_POST, CONTENT_TYPE_SERVICE,
    CONTENT_TYPE_TOKEN, CURRENCY_MYSO, REVENUE_SOURCE_MESSAGING, REVENUE_SOURCE_MYDATA,
    REVENUE_SOURCE_POSTS, REVENUE_SOURCE_SPT, REVENUE_SOURCE_SUBSCRIPTION, REVENUE_SOURCE_TIPS,
};
use crate::schema::{
    ecosystem_treasury, spt_config, spt_events, spt_holdings, spt_pools, spt_price_history,
    spt_reservation_pools, spt_reservations, spt_revenue, spt_transactions, unified_revenue,
};

pub const TOKEN_TYPE_PROFILE: i16 = 1;
pub const TOKEN_TYPE_POST: i16 = 2;
pub const TRANSACTION_TYPE_BUY: &str = "BUY";
pub const TRANSACTION_TYPE_SELL: &str = "SELL";
pub const TRANSACTION_TYPE_RESERVATION: &str = "RESERVATION";
pub const TRANSACTION_TYPE_RESERVATION_WITHDRAW: &str = "RESERVATION_WITHDRAW";
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

/// On-chain SPT amounts use `10^9` nano-SPT per 1.0 display token (same as MYSO decimals).
pub const SPT_AMOUNT_NANO_SCALE: i64 = 1_000_000_000;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_pools)]
pub struct NewSptPool {
    pub pool_id: String,
    pub token_type: i16,
    pub owner: String,
    pub associated_id: String,
    /// nano-SPT: `10^9` units per 1.0 display token (`spt_pools.circulating_supply`).
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
    #[diesel(sql_type = SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = Text)]
    pub associated_id: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(sql_type = Nullable<Bool>)]
    pub viewer_is_following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(sql_type = Nullable<Bool>)]
    pub viewer_follows_viewer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(sql_type = Nullable<Bool>)]
    pub blocked_by_viewer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(sql_type = Nullable<Bool>)]
    pub blocked_by_subject: Option<bool>,
}

/// Query result for reservation holdings (from `spt_reservation_holdings` view + profiles).
#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct SptReservationHoldingRow {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(sql_type = Nullable<Bool>)]
    pub viewer_is_following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(sql_type = Nullable<Bool>)]
    pub viewer_follows_viewer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(sql_type = Nullable<Bool>)]
    pub blocked_by_viewer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diesel(sql_type = Nullable<Bool>)]
    pub blocked_by_subject: Option<bool>,
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
    /// nano-SPT: `10^9` units per 1.0 display token (`spt_holdings.amount`).
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
    pub organization_id: Option<String>,
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
    pub created_at: i64,
    pub fee_amount: Option<i64>,
    pub creator_fee: Option<i64>,
    pub platform_fee: Option<i64>,
    pub treasury_fee: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ecosystem_treasury)]
pub struct EcosystemTreasury {
    pub id: i32,
    pub treasury_address: String,
    pub updated_by: String,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub version: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = ecosystem_treasury)]
pub struct NewEcosystemTreasury {
    pub treasury_address: String,
    pub updated_by: String,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub version: i64,
}

impl NewEcosystemTreasury {
    pub fn from_event(
        treasury_address: String,
        updated_by: String,
        updated_at: u64,
        transaction_id: String,
    ) -> Self {
        let timestamp_secs = (updated_at / 1000) as i64;
        let time = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp_secs, 0)
            .unwrap_or_else(chrono::Utc::now);

        Self {
            treasury_address,
            updated_by,
            updated_at: updated_at as i64,
            time,
            transaction_id,
            version: 0,
        }
    }
}

/// `trading_enabled` is [`None`] for fee/threshold config events; kill-switch events set [`Some`].
///
/// When [`Self::apply_trading_enabled_only`] is true (emergency kill switch), the indexer must only
/// update trading toggle and metadata columns on `spt_exchange_config`, not fee/threshold fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub non_platform_platform_to_creator_bps: i64,
    pub non_platform_platform_to_treasury_bps: i64,
    pub trading_enabled: Option<bool>,
    /// Kill-switch path: apply only `updated_by`, `trading_enabled`, `updated_at`, `transaction_id`.
    #[serde(default)]
    pub apply_trading_enabled_only: bool,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub version: i64,
}

#[derive(Debug, Clone, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::spt_exchange_config)]
pub struct InsertSptExchangeConfig {
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
    pub non_platform_platform_to_creator_bps: i64,
    pub non_platform_platform_to_treasury_bps: i64,
    pub trading_enabled: bool,
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl InsertSptExchangeConfig {
    pub fn from_row(row: &NewSptExchangeConfig, trading_enabled: bool) -> Self {
        Self {
            updated_by: row.updated_by.clone(),
            post_threshold: row.post_threshold,
            profile_threshold: row.profile_threshold,
            max_individual_reservation_bps: row.max_individual_reservation_bps,
            total_fee_bps: row.total_fee_bps,
            creator_fee_bps: row.creator_fee_bps,
            platform_fee_bps: row.platform_fee_bps,
            treasury_fee_bps: row.treasury_fee_bps,
            trading_creator_fee_bps: row.trading_creator_fee_bps,
            trading_platform_fee_bps: row.trading_platform_fee_bps,
            trading_treasury_fee_bps: row.trading_treasury_fee_bps,
            reservation_creator_fee_bps: row.reservation_creator_fee_bps,
            reservation_platform_fee_bps: row.reservation_platform_fee_bps,
            reservation_treasury_fee_bps: row.reservation_treasury_fee_bps,
            max_reservers_per_pool: row.max_reservers_per_pool,
            base_price: row.base_price,
            quadratic_coefficient: row.quadratic_coefficient,
            max_hold_percent_bps: row.max_hold_percent_bps,
            non_platform_platform_to_creator_bps: row.non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps: row.non_platform_platform_to_treasury_bps,
            trading_enabled,
            version: row.version,
            updated_at: row.updated_at,
            time: row.time,
            transaction_id: row.transaction_id.clone(),
        }
    }

    fn with_version(mut self, version: i64) -> Self {
        self.version = version;
        self
    }
}

pub fn merge_spt_exchange_config(
    prev: &InsertSptExchangeConfig,
    incoming: &NewSptExchangeConfig,
) -> InsertSptExchangeConfig {
    let trading_enabled = incoming
        .trading_enabled
        .unwrap_or(prev.trading_enabled);
    let version = if incoming.version > 0 {
        incoming.version
    } else {
        prev.version + 1
    };

    if incoming.apply_trading_enabled_only {
        return InsertSptExchangeConfig {
            updated_by: incoming.updated_by.clone(),
            trading_enabled,
            version,
            updated_at: incoming.updated_at,
            time: incoming.time,
            transaction_id: incoming.transaction_id.clone(),
            ..prev.clone()
        };
    }

    InsertSptExchangeConfig::from_row(incoming, trading_enabled).with_version(version)
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::spt_exchange_config)]
#[diesel(treat_none_as_null = false)]
pub struct SptExchangeConfigChangeset {
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
    pub non_platform_platform_to_creator_bps: i64,
    pub non_platform_platform_to_treasury_bps: i64,
    pub trading_enabled: Option<bool>,
    pub updated_at: i64,
    pub transaction_id: String,
}

impl From<&NewSptExchangeConfig> for SptExchangeConfigChangeset {
    fn from(c: &NewSptExchangeConfig) -> Self {
        Self {
            updated_by: c.updated_by.clone(),
            post_threshold: c.post_threshold,
            profile_threshold: c.profile_threshold,
            max_individual_reservation_bps: c.max_individual_reservation_bps,
            total_fee_bps: c.total_fee_bps,
            creator_fee_bps: c.creator_fee_bps,
            platform_fee_bps: c.platform_fee_bps,
            treasury_fee_bps: c.treasury_fee_bps,
            trading_creator_fee_bps: c.trading_creator_fee_bps,
            trading_platform_fee_bps: c.trading_platform_fee_bps,
            trading_treasury_fee_bps: c.trading_treasury_fee_bps,
            reservation_creator_fee_bps: c.reservation_creator_fee_bps,
            reservation_platform_fee_bps: c.reservation_platform_fee_bps,
            reservation_treasury_fee_bps: c.reservation_treasury_fee_bps,
            max_reservers_per_pool: c.max_reservers_per_pool,
            base_price: c.base_price,
            quadratic_coefficient: c.quadratic_coefficient,
            max_hold_percent_bps: c.max_hold_percent_bps,
            non_platform_platform_to_creator_bps: c.non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps: c.non_platform_platform_to_treasury_bps,
            trading_enabled: c.trading_enabled,
            updated_at: c.updated_at,
            transaction_id: c.transaction_id.clone(),
        }
    }
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

    pub fn from_reservation_event(
        pool_id: String,
        withdraw: bool,
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
        time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let transaction_type = if withdraw {
            TRANSACTION_TYPE_RESERVATION_WITHDRAW
        } else {
            TRANSACTION_TYPE_RESERVATION
        };
        Self {
            pool_id,
            transaction_type: transaction_type.to_string(),
            trader,
            creator_address,
            platform_address,
            treasury_address,
            creator_fee,
            platform_fee,
            treasury_fee,
            total_fee: creator_fee
                .saturating_add(platform_fee)
                .saturating_add(treasury_fee),
            token_amount,
            myso_amount,
            token_price,
            revenue_time,
            time,
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
    pub organization_id: Option<String>,
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
    pub organization_id: Option<String>,
}

impl NewUnifiedRevenue {
    pub fn from_tip(
        revenue_type: String,
        creator_address: String,
        amount: i64,
        currency: String,
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
            currency,
            content_id: Some(content_id),
            content_type: Some(content_type),
            payer_address,
            recipient_address: creator_address,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
            organization_id: None,
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
            organization_id: None,
        }
    }

    pub fn from_spt_at_time(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        pool_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
        time: chrono::DateTime<chrono::Utc>,
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
            time,
            transaction_id,
            organization_id: None,
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
            organization_id: None,
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
            organization_id: None,
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
            organization_id: None,
        }
    }

    pub fn from_messaging_at_time(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        content_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
        time: chrono::DateTime<chrono::Utc>,
        organization_id: Option<String>,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_MESSAGING.to_string(),
            revenue_type,
            creator_address,
            platform_address,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(content_id),
            content_type: Some(CONTENT_TYPE_MESSAGING.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            time,
            transaction_id,
            organization_id,
        }
    }
}

#[derive(Debug, Clone, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = spt_config)]
pub struct NewSocialProofTokensConfig {
    pub trading_enabled: bool,
    pub admin_address: String,
    pub reason: String,
    pub updated_by: String,
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

pub fn merge_social_proof_tokens_config(
    prev: &NewSocialProofTokensConfig,
    incoming: &NewSocialProofTokensConfig,
) -> NewSocialProofTokensConfig {
    let version = if incoming.version > 0 {
        incoming.version
    } else {
        prev.version + 1
    };
    NewSocialProofTokensConfig {
        trading_enabled: incoming.trading_enabled,
        admin_address: if incoming.admin_address.is_empty() {
            prev.admin_address.clone()
        } else {
            incoming.admin_address.clone()
        },
        reason: if incoming.reason.is_empty() {
            prev.reason.clone()
        } else {
            incoming.reason.clone()
        },
        updated_by: if incoming.updated_by.is_empty() {
            prev.updated_by.clone()
        } else {
            incoming.updated_by.clone()
        },
        version,
        updated_at: incoming.updated_at,
        time: incoming.time,
        transaction_id: incoming.transaction_id.clone(),
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_events)]
pub struct NewSocialProofTokensEvent {
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod spt_exchange_config_changeset_tests {
    use super::{NewSptExchangeConfig, SptExchangeConfigChangeset};
    use crate::schema::spt_exchange_config;
    use diesel::debug_query;
    use diesel::pg::Pg;

    fn sample_row(trading_enabled: Option<bool>) -> NewSptExchangeConfig {
        NewSptExchangeConfig {
            updated_by: "0x1".to_string(),
            post_threshold: 1,
            profile_threshold: 1,
            max_individual_reservation_bps: 1,
            total_fee_bps: 1,
            creator_fee_bps: 1,
            platform_fee_bps: 1,
            treasury_fee_bps: 1,
            trading_creator_fee_bps: 1,
            trading_platform_fee_bps: 1,
            trading_treasury_fee_bps: 1,
            reservation_creator_fee_bps: 1,
            reservation_platform_fee_bps: 1,
            reservation_treasury_fee_bps: 1,
            max_reservers_per_pool: 1,
            base_price: 1,
            quadratic_coefficient: 1,
            max_hold_percent_bps: 1,
            non_platform_platform_to_creator_bps: 1,
            non_platform_platform_to_treasury_bps: 1,
            trading_enabled,
            apply_trading_enabled_only: false,
            version: 1,
            updated_at: 0,
            time: chrono::DateTime::UNIX_EPOCH,
            transaction_id: "tx".to_string(),
        }
    }

    #[test]
    fn as_changeset_omits_trading_enabled_when_unset() {
        let row = sample_row(None);
        let q =
            diesel::update(spt_exchange_config::table).set(SptExchangeConfigChangeset::from(&row));
        let sql = debug_query::<Pg, _>(&q).to_string();
        assert!(
            !sql.to_lowercase().contains("trading_enabled"),
            "SET should not touch trading_enabled when None: {sql}"
        );
    }

    #[test]
    fn as_changeset_sets_trading_enabled_when_some() {
        let row = sample_row(Some(true));
        let q =
            diesel::update(spt_exchange_config::table).set(SptExchangeConfigChangeset::from(&row));
        let sql = debug_query::<Pg, _>(&q).to_string();
        assert!(
            sql.to_lowercase().contains("trading_enabled"),
            "SET should include trading_enabled when Some: {sql}"
        );
    }
}
