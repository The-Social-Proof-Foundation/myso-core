// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use super::social_graph::{PaginationInfo, UniversalUserResult};

#[derive(Debug, Clone, Serialize)]
pub struct VestingWalletRow {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub start_time: i64,
    pub duration: i64,
    pub curve_factor: i64,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VestingWalletWithStatus {
    pub claimed_percentage: f64,
    pub is_fully_claimed: bool,
    pub has_started: bool,
    pub has_ended: bool,
    pub vesting_progress: f64,
    pub end_time: i64,
    #[serde(flatten)]
    pub wallet: VestingWalletRow,
}

impl VestingWalletWithStatus {
    pub fn from_wallet(wallet: VestingWalletRow, current_time_ms: u64) -> Self {
        let claimed_percentage = if wallet.total_amount == 0 {
            0.0
        } else {
            (wallet.claimed_amount as f64 / wallet.total_amount as f64) * 100.0
        };
        let end_time = wallet.start_time + wallet.duration;
        let has_started = wallet.start_time <= (current_time_ms as i64);
        let has_ended = (current_time_ms as i64) >= end_time;
        let vesting_progress = {
            let current_time = current_time_ms as i64;
            if current_time <= wallet.start_time {
                0.0
            } else if current_time >= end_time {
                1.0
            } else {
                let elapsed = current_time - wallet.start_time;
                elapsed as f64 / wallet.duration as f64
            }
        };
        Self {
            claimed_percentage,
            is_fully_claimed: wallet.remaining_balance == 0,
            has_started,
            has_ended,
            vesting_progress,
            end_time,
            wallet,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VestingWalletWithProfile {
    #[serde(flatten)]
    pub wallet: VestingWalletWithStatus,
    #[serde(flatten)]
    pub user: UniversalUserResult,
}

#[derive(Debug, Serialize)]
pub struct VestingEventRow {
    pub id: i32,
    pub wallet_id: String,
    pub event_type: String,
    pub owner_address: String,
    pub amount: i64,
    pub remaining_balance: Option<i64>,
    pub start_time: Option<i64>,
    pub duration: Option<i64>,
    pub curve_factor: Option<i64>,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Serialize)]
pub struct VestingWalletsResponse {
    pub wallets: Vec<VestingWalletWithProfile>,
    pub total: i64,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct VestingEventsResponse {
    pub events: Vec<VestingEventRow>,
    pub total: i64,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct ClaimableResponse {
    pub wallet_id: String,
    pub claimable_amount: i64,
    pub current_progress: f64,
    pub vesting_status: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize)]
pub struct VestingAnalyticsResponse {
    pub total_wallets: i64,
    pub total_vested_amount: i64,
    pub total_claimed_amount: i64,
    pub total_remaining_amount: i64,
    pub active_wallets: i64,
    pub completed_wallets: i64,
    pub average_vesting_duration: f64,
    pub most_common_curve_factor: i64,
}

#[derive(Debug, Serialize)]
pub struct VestingLeaderboardEntry {
    pub owner_address: String,
    pub total_vested: i64,
    pub total_claimed: i64,
    pub active_wallets: i64,
    pub completed_wallets: i64,
    #[serde(flatten)]
    pub user: UniversalUserResult,
}

#[derive(Debug, Serialize)]
pub struct VestingLeaderboardResponse {
    pub entries: Vec<VestingLeaderboardEntry>,
    pub total: i64,
}
