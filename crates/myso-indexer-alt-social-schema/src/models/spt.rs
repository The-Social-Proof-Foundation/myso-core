// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text, Timestamptz};
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};

use super::revenue::{
    CONTENT_TYPE_DATA, CONTENT_TYPE_MESSAGING, CONTENT_TYPE_POST, CONTENT_TYPE_SERVICE,
    CONTENT_TYPE_TOKEN, CONTENT_TYPE_USERNAME, CURRENCY_MYSO, REVENUE_SOURCE_MESSAGING,
    REVENUE_SOURCE_MYDATA, REVENUE_SOURCE_POSTS, REVENUE_SOURCE_SPT, REVENUE_SOURCE_SUBSCRIPTION,
    REVENUE_SOURCE_TIPS, REVENUE_SOURCE_USERNAME_MARKETPLACE,
};
use crate::schema::{
    ecosystem_treasury, spt_config, spt_events, spt_holdings, spt_pools, spt_price_history,
    spt_reservation_pools, spt_reservations, spt_revenue, spt_swaps, spt_transactions,
    spt_transfers, unified_revenue,
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
    pub circulating_supply_24h_ago: Option<i64>,
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
    /// Populated on rows that are one leg of an SPT→SPT swap (the opposite pool).
    pub counterparty_pool_id: Option<String>,
    /// `true` when this BUY/SELL row is one leg of an SPT→SPT swap.
    pub is_swap_leg: bool,
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

impl NewSptTransaction {
    /// Maps a reservation ledger row into `spt_transactions` (pre-launch MYSO activity).
    pub fn from_reservation(reservation: &NewSptReservation, pool_id: String) -> Self {
        let creator_fee = reservation.creator_fee.unwrap_or(0);
        let platform_fee = reservation.platform_fee.unwrap_or(0);
        let treasury_fee = reservation.treasury_fee.unwrap_or(0);
        let transaction_type = if reservation.amount < 0 {
            TRANSACTION_TYPE_RESERVATION_WITHDRAW.to_string()
        } else {
            TRANSACTION_TYPE_RESERVATION.to_string()
        };
        Self {
            pool_id,
            transaction_type,
            sender: reservation.reserver_address.clone(),
            amount: 0,
            myso_amount: reservation.amount,
            fee_amount: creator_fee
                .saturating_add(platform_fee)
                .saturating_add(treasury_fee),
            creator_fee,
            platform_fee,
            treasury_fee,
            price: 0,
            created_at: reservation.created_at,
            time: reservation.time,
            transaction_id: reservation.transaction_id.clone(),
            organization_id: reservation.organization_id.clone(),
        }
    }
}

/// Insert row for an atomic SPT→SPT swap summary (`spt_swaps`).
///
/// SUMMARY ONLY: this row never mutates holdings/supply/price/revenue — those are
/// derived from the underlying `TokenSoldEvent` + `TokenBoughtEvent` legs.
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_swaps)]
pub struct NewSptSwap {
    pub transaction_id: String,
    pub trader: String,
    pub source_pool_id: String,
    pub dest_pool_id: String,
    /// nano-SPT sold from the source pool.
    pub sell_amount: i64,
    /// nano-SPT bought into the dest pool.
    pub dest_amount: i64,
    pub sell_myso_gross: i64,
    pub buy_myso_gross: i64,
    pub sell_fee_amount: i64,
    pub buy_fee_amount: i64,
    pub sell_creator_fee: i64,
    pub sell_platform_fee: i64,
    pub sell_treasury_fee: i64,
    pub buy_creator_fee: i64,
    pub buy_platform_fee: i64,
    pub buy_treasury_fee: i64,
    pub leftover_myso: i64,
    pub source_new_price: i64,
    pub dest_new_price: i64,
    pub organization_id: Option<String>,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = spt_swaps)]
pub struct SptSwap {
    pub id: i64,
    pub transaction_id: String,
    pub trader: String,
    pub source_pool_id: String,
    pub dest_pool_id: String,
    pub sell_amount: i64,
    pub dest_amount: i64,
    pub sell_myso_gross: i64,
    pub buy_myso_gross: i64,
    pub sell_fee_amount: i64,
    pub buy_fee_amount: i64,
    pub sell_creator_fee: i64,
    pub sell_platform_fee: i64,
    pub sell_treasury_fee: i64,
    pub buy_creator_fee: i64,
    pub buy_platform_fee: i64,
    pub buy_treasury_fee: i64,
    pub leftover_myso: i64,
    pub source_new_price: i64,
    pub dest_new_price: i64,
    pub organization_id: Option<String>,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
}

/// Insert row for a P2P SPT transfer (`spt_transfers`).
///
/// Holdings are updated via separate `spt_holdings` deltas (from− / to+).
/// Supply, price, and revenue are intentionally untouched.
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_transfers)]
pub struct NewSptTransfer {
    pub transaction_id: String,
    pub pool_id: String,
    pub from_address: String,
    pub to_address: String,
    /// nano-SPT transferred.
    pub amount: i64,
    pub organization_id: Option<String>,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = spt_transfers)]
pub struct SptTransfer {
    pub id: i64,
    pub transaction_id: String,
    pub pool_id: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub organization_id: Option<String>,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
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

/// Event-layer SPT config before merge/insert. Kill-switch events set [`apply_trading_enabled_only`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSptConfigEvent {
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
    pub admin_address: Option<String>,
    pub reason: Option<String>,
    #[serde(default)]
    pub apply_trading_enabled_only: bool,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub version: i64,
}

#[derive(Debug, Clone, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = spt_config)]
pub struct InsertSptConfig {
    pub trading_enabled: bool,
    pub admin_address: String,
    pub reason: String,
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
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl InsertSptConfig {
    pub fn from_event(event: &NewSptConfigEvent, trading_enabled: bool) -> Self {
        Self {
            trading_enabled,
            admin_address: event.admin_address.clone().unwrap_or_default(),
            reason: event.reason.clone().unwrap_or_default(),
            updated_by: event.updated_by.clone(),
            post_threshold: event.post_threshold,
            profile_threshold: event.profile_threshold,
            max_individual_reservation_bps: event.max_individual_reservation_bps,
            total_fee_bps: event.total_fee_bps,
            creator_fee_bps: event.creator_fee_bps,
            platform_fee_bps: event.platform_fee_bps,
            treasury_fee_bps: event.treasury_fee_bps,
            trading_creator_fee_bps: event.trading_creator_fee_bps,
            trading_platform_fee_bps: event.trading_platform_fee_bps,
            trading_treasury_fee_bps: event.trading_treasury_fee_bps,
            reservation_creator_fee_bps: event.reservation_creator_fee_bps,
            reservation_platform_fee_bps: event.reservation_platform_fee_bps,
            reservation_treasury_fee_bps: event.reservation_treasury_fee_bps,
            max_reservers_per_pool: event.max_reservers_per_pool,
            base_price: event.base_price,
            quadratic_coefficient: event.quadratic_coefficient,
            max_hold_percent_bps: event.max_hold_percent_bps,
            non_platform_platform_to_creator_bps: event.non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps: event.non_platform_platform_to_treasury_bps,
            version: event.version,
            updated_at: event.updated_at,
            time: event.time,
            transaction_id: event.transaction_id.clone(),
        }
    }

    fn with_version(mut self, version: i64) -> Self {
        self.version = version;
        self
    }
}

pub fn merge_spt_config(prev: &InsertSptConfig, incoming: &NewSptConfigEvent) -> InsertSptConfig {
    let trading_enabled = incoming.trading_enabled.unwrap_or(prev.trading_enabled);
    let version = if incoming.version > 0 {
        incoming.version
    } else {
        prev.version + 1
    };

    if incoming.apply_trading_enabled_only {
        return InsertSptConfig {
            trading_enabled,
            admin_address: incoming
                .admin_address
                .clone()
                .filter(|v| !v.is_empty())
                .unwrap_or_else(|| prev.admin_address.clone()),
            reason: incoming
                .reason
                .clone()
                .filter(|v| !v.is_empty())
                .unwrap_or_else(|| prev.reason.clone()),
            updated_by: if incoming.updated_by.is_empty() {
                prev.updated_by.clone()
            } else {
                incoming.updated_by.clone()
            },
            version,
            updated_at: incoming.updated_at,
            time: incoming.time,
            transaction_id: incoming.transaction_id.clone(),
            ..prev.clone()
        };
    }

    let mut merged = InsertSptConfig::from_event(incoming, trading_enabled).with_version(version);
    merged.admin_address = incoming
        .admin_address
        .clone()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| prev.admin_address.clone());
    merged.reason = incoming
        .reason
        .clone()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| prev.reason.clone());
    merged
}

pub fn default_spt_config() -> InsertSptConfig {
    InsertSptConfig {
        trading_enabled: true,
        admin_address: String::new(),
        reason: String::new(),
        updated_by: String::new(),
        post_threshold: DEFAULT_POST_THRESHOLD,
        profile_threshold: DEFAULT_PROFILE_THRESHOLD,
        max_individual_reservation_bps: DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS,
        total_fee_bps: DEFAULT_TRADING_CREATOR_FEE_BPS
            + DEFAULT_TRADING_PLATFORM_FEE_BPS
            + DEFAULT_TRADING_TREASURY_FEE_BPS,
        creator_fee_bps: DEFAULT_TRADING_CREATOR_FEE_BPS,
        platform_fee_bps: DEFAULT_TRADING_PLATFORM_FEE_BPS,
        treasury_fee_bps: DEFAULT_TRADING_TREASURY_FEE_BPS,
        trading_creator_fee_bps: DEFAULT_TRADING_CREATOR_FEE_BPS,
        trading_platform_fee_bps: DEFAULT_TRADING_PLATFORM_FEE_BPS,
        trading_treasury_fee_bps: DEFAULT_TRADING_TREASURY_FEE_BPS,
        reservation_creator_fee_bps: DEFAULT_RESERVATION_CREATOR_FEE_BPS,
        reservation_platform_fee_bps: DEFAULT_RESERVATION_PLATFORM_FEE_BPS,
        reservation_treasury_fee_bps: DEFAULT_RESERVATION_TREASURY_FEE_BPS,
        max_reservers_per_pool: DEFAULT_MAX_RESERVERS_PER_POOL,
        base_price: DEFAULT_BASE_PRICE,
        quadratic_coefficient: DEFAULT_QUADRATIC_COEFFICIENT,
        max_hold_percent_bps: MAX_HOLD_PERCENT_BPS,
        non_platform_platform_to_creator_bps: 5000,
        non_platform_platform_to_treasury_bps: 5000,
        version: 0,
        updated_at: 0,
        time: chrono::Utc::now(),
        transaction_id: String::new(),
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
        currency: String,
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
            currency,
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

    pub fn from_username_marketplace(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        username: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_USERNAME_MARKETPLACE.to_string(),
            revenue_type,
            creator_address,
            platform_address,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(username),
            content_type: Some(CONTENT_TYPE_USERNAME.to_string()),
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
        platform_address: Option<String>,
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
            platform_address,
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

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_events)]
pub struct NewSocialProofTokensEvent {
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod spt_config_merge_tests {
    use super::{default_spt_config, merge_spt_config, NewSptConfigEvent};

    #[test]
    fn kill_switch_preserves_fee_fields() {
        let prev = default_spt_config();
        let incoming = NewSptConfigEvent {
            updated_by: "0xadmin".to_string(),
            post_threshold: 0,
            profile_threshold: 0,
            max_individual_reservation_bps: 0,
            total_fee_bps: 0,
            creator_fee_bps: 0,
            platform_fee_bps: 0,
            treasury_fee_bps: 0,
            trading_creator_fee_bps: 0,
            trading_platform_fee_bps: 0,
            trading_treasury_fee_bps: 0,
            reservation_creator_fee_bps: 0,
            reservation_platform_fee_bps: 0,
            reservation_treasury_fee_bps: 0,
            max_reservers_per_pool: 0,
            base_price: 0,
            quadratic_coefficient: 0,
            max_hold_percent_bps: 0,
            non_platform_platform_to_creator_bps: 0,
            non_platform_platform_to_treasury_bps: 0,
            trading_enabled: Some(false),
            admin_address: Some("0xadmin".to_string()),
            reason: Some("halt".to_string()),
            apply_trading_enabled_only: true,
            updated_at: 1,
            time: chrono::Utc::now(),
            transaction_id: "tx".to_string(),
            version: 0,
        };
        let merged = merge_spt_config(&prev, &incoming);
        assert!(!merged.trading_enabled);
        assert_eq!(merged.base_price, prev.base_price);
        assert_eq!(merged.admin_address, "0xadmin");
    }

    #[test]
    fn config_update_preserves_kill_switch_audit_fields() {
        let mut prev = default_spt_config();
        prev.admin_address = "0xkill_admin".to_string();
        prev.reason = "halt".to_string();
        let incoming = NewSptConfigEvent {
            updated_by: "0xconfig_admin".to_string(),
            post_threshold: 2,
            profile_threshold: 3,
            max_individual_reservation_bps: 100,
            total_fee_bps: 150,
            creator_fee_bps: 100,
            platform_fee_bps: 25,
            treasury_fee_bps: 25,
            trading_creator_fee_bps: 100,
            trading_platform_fee_bps: 25,
            trading_treasury_fee_bps: 25,
            reservation_creator_fee_bps: 100,
            reservation_platform_fee_bps: 25,
            reservation_treasury_fee_bps: 25,
            max_reservers_per_pool: 1000,
            base_price: 200,
            quadratic_coefficient: 300,
            max_hold_percent_bps: 500,
            non_platform_platform_to_creator_bps: 5000,
            non_platform_platform_to_treasury_bps: 5000,
            trading_enabled: Some(true),
            admin_address: None,
            reason: None,
            apply_trading_enabled_only: false,
            updated_at: 42,
            time: chrono::Utc::now(),
            transaction_id: "tx-config".to_string(),
            version: 0,
        };
        let merged = merge_spt_config(&prev, &incoming);
        assert_eq!(merged.base_price, 200);
        assert_eq!(merged.admin_address, "0xkill_admin");
        assert_eq!(merged.reason, "halt");
        assert_eq!(merged.updated_by, "0xconfig_admin");
    }
}

#[cfg(test)]
mod new_spt_transaction_from_reservation_tests {
    use super::{
        NewSptReservation, NewSptTransaction, TRANSACTION_TYPE_RESERVATION,
        TRANSACTION_TYPE_RESERVATION_WITHDRAW,
    };

    fn sample_reservation(amount: i64) -> NewSptReservation {
        NewSptReservation {
            pool_id: "reservation_pool_0xabc".to_string(),
            reserver_address: "0xreserver".to_string(),
            amount,
            reserved_at: 1_700_000_000_000,
            created_at: 1_700_000_000_100,
            fee_amount: Some(75_000_000),
            creator_fee: Some(56_250_000),
            platform_fee: Some(0),
            treasury_fee: Some(18_750_000),
            time: chrono::DateTime::from_timestamp_millis(1_700_000_000_000).unwrap(),
            transaction_id: "tx:0".to_string(),
            organization_id: Some("org-1".to_string()),
        }
    }

    #[test]
    fn deposit_maps_to_reservation_type_with_myso_amount() {
        let reservation = sample_reservation(4_925_000_000);
        let tx = NewSptTransaction::from_reservation(&reservation, "0xpool".to_string());
        assert_eq!(tx.transaction_type, TRANSACTION_TYPE_RESERVATION);
        assert_eq!(tx.pool_id, "0xpool");
        assert_eq!(tx.sender, "0xreserver");
        assert_eq!(tx.amount, 0);
        assert_eq!(tx.myso_amount, 4_925_000_000);
        assert_eq!(tx.fee_amount, 75_000_000);
        assert_eq!(tx.creator_fee, 56_250_000);
        assert_eq!(tx.platform_fee, 0);
        assert_eq!(tx.treasury_fee, 18_750_000);
        assert_eq!(tx.price, 0);
        assert_eq!(tx.created_at, 1_700_000_000_100);
        assert_eq!(tx.transaction_id, "tx:0");
        assert_eq!(tx.organization_id.as_deref(), Some("org-1"));
    }

    #[test]
    fn withdraw_maps_to_reservation_withdraw_with_negative_myso_amount() {
        let reservation = sample_reservation(-1_000_000_000);
        let tx = NewSptTransaction::from_reservation(&reservation, "0xpool".to_string());
        assert_eq!(tx.transaction_type, TRANSACTION_TYPE_RESERVATION_WITHDRAW);
        assert_eq!(tx.myso_amount, -1_000_000_000);
    }
}
