// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{vesting_events, vesting_wallets};

pub const VESTING_EVENT_TYPE_VESTED: &str = "TokensVested";
pub const VESTING_EVENT_TYPE_CLAIMED: &str = "TokensClaimed";
pub const PIECE_KIND_CLIFF: i64 = 0;
pub const PIECE_KIND_CONTINUOUS: i64 = 1;
pub const BPS_DENOMINATOR: i64 = 10_000;
pub const CURVE_FACTOR_LINEAR: i64 = 1000;
pub const CURVE_FACTOR_MIN: i64 = 100;
pub const CURVE_FACTOR_MAX: i64 = 10_000;
pub const MAX_VESTING_PIECES: usize = 10;
pub const MIN_CLAIM_THRESHOLD_DIVISOR: i64 = 1000;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct VestingPiece {
    pub kind: i64,
    pub time_offset: i64,
    pub duration: i64,
    pub amount_bps: i64,
    pub curve_factor: i64,
}

impl VestingPiece {
    pub fn piece_amount(&self, total_amount: i64) -> i64 {
        (total_amount * self.amount_bps) / BPS_DENOMINATOR
    }
}

pub fn apply_curve(progress_ratio: f64, curve_factor: i64) -> f64 {
    let precision = 1000.0;
    if curve_factor == 0 || curve_factor == CURVE_FACTOR_LINEAR {
        progress_ratio
    } else if curve_factor > CURVE_FACTOR_LINEAR {
        let steepness = (curve_factor - CURVE_FACTOR_LINEAR) as f64;
        let quadratic = progress_ratio * progress_ratio;
        (progress_ratio * (precision - steepness) + quadratic * steepness) / precision
    } else {
        let steepness = (CURVE_FACTOR_LINEAR - curve_factor) as f64;
        let sqrt_approx = progress_ratio.max(0.0).sqrt();
        (sqrt_approx * steepness + progress_ratio * (precision - steepness)) / precision
    }
}

pub fn vested_amount_for_piece(
    total_amount: i64,
    start_time: i64,
    current_time: i64,
    piece: &VestingPiece,
) -> i64 {
    if current_time < start_time {
        return 0;
    }
    let activation_time = start_time + piece.time_offset;
    if current_time < activation_time {
        return 0;
    }
    let alloc = piece.piece_amount(total_amount);
    if piece.kind == PIECE_KIND_CLIFF {
        return alloc;
    }
    if piece.kind != PIECE_KIND_CONTINUOUS || piece.duration <= 0 {
        return 0;
    }
    let end_time = activation_time + piece.duration;
    if current_time >= end_time {
        return alloc;
    }
    let elapsed = current_time - activation_time;
    let progress_ratio = elapsed as f64 / piece.duration as f64;
    let curved = apply_curve(progress_ratio, piece.curve_factor);
    (alloc as f64 * curved) as i64
}

pub fn calculate_total_vested(
    total_amount: i64,
    start_time: i64,
    current_time: i64,
    pieces: &[VestingPiece],
) -> i64 {
    if current_time < start_time {
        return 0;
    }
    let total_released: i64 = pieces
        .iter()
        .map(|p| vested_amount_for_piece(total_amount, start_time, current_time, p))
        .sum();
    total_released.min(total_amount)
}

pub fn finalize_claimable(
    capped: i64,
    remaining_balance: i64,
    total_amount: i64,
    current_time: i64,
    schedule_end: i64,
) -> i64 {
    if current_time >= schedule_end {
        return remaining_balance;
    }
    if capped == 0 {
        return 0;
    }
    let mut threshold = total_amount / MIN_CLAIM_THRESHOLD_DIVISOR;
    if threshold == 0 {
        threshold = 1;
    }
    if capped < threshold && capped < remaining_balance {
        0
    } else {
        capped
    }
}

pub fn calculate_vesting_claimable(
    total_amount: i64,
    start_time: i64,
    schedule_end: i64,
    pieces: &[VestingPiece],
    claimed_amount: i64,
    current_time: i64,
    remaining_balance: i64,
) -> i64 {
    if current_time < start_time {
        return 0;
    }
    if current_time >= schedule_end {
        return remaining_balance;
    }
    let total_vested = calculate_total_vested(total_amount, start_time, current_time, pieces);
    let newly_claimable = (total_vested - claimed_amount).max(0);
    let capped = newly_claimable.min(remaining_balance);
    finalize_claimable(
        capped,
        remaining_balance,
        total_amount,
        current_time,
        schedule_end,
    )
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = vesting_wallets)]
pub struct VestingWallet {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub start_time: i64,
    pub schedule_end: i64,
    pub pieces: serde_json::Value,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = vesting_wallets)]
pub struct NewVestingWallet {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub start_time: i64,
    pub schedule_end: i64,
    pub pieces: serde_json::Value,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = vesting_wallets)]
pub struct UpdateVestingWallet {
    pub claimed_amount: Option<i64>,
    pub remaining_balance: Option<i64>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = vesting_events)]
pub struct VestingEvent {
    pub id: i32,
    pub wallet_id: String,
    pub event_type: String,
    pub owner_address: String,
    pub amount: i64,
    pub remaining_balance: Option<i64>,
    pub start_time: Option<i64>,
    pub schedule_end: Option<i64>,
    pub pieces: Option<serde_json::Value>,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = vesting_events)]
pub struct NewVestingEvent {
    pub wallet_id: String,
    pub event_type: String,
    pub owner_address: String,
    pub amount: i64,
    pub remaining_balance: Option<i64>,
    pub start_time: Option<i64>,
    pub schedule_end: Option<i64>,
    pub pieces: Option<serde_json::Value>,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

pub fn parse_pieces(value: &serde_json::Value) -> Vec<VestingPiece> {
    value
        .as_array()
        .map(|arr| {
            arr.iter()
                .take(MAX_VESTING_PIECES)
                .filter_map(|p| {
                    Some(VestingPiece {
                        kind: p.get("kind")?.as_i64()?,
                        time_offset: p.get("time_offset")?.as_i64()?,
                        duration: p.get("duration")?.as_i64()?,
                        amount_bps: p.get("amount_bps")?.as_i64()?,
                        curve_factor: p.get("curve_factor")?.as_i64()?,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linear_pieces(duration: i64) -> Vec<VestingPiece> {
        vec![VestingPiece {
            kind: PIECE_KIND_CONTINUOUS,
            time_offset: 0,
            duration,
            amount_bps: BPS_DENOMINATOR,
            curve_factor: CURVE_FACTOR_LINEAR,
        }]
    }

    #[test]
    fn threshold_suppresses_sub_point_one_percent() {
        let pieces = linear_pieces(10_000);
        let total = 1_000_000;
        let start = 2000;
        let end = 12_000;
        // 0.05% elapsed => 500 vested; threshold is 1000
        let claimable = calculate_vesting_claimable(total, start, end, &pieces, 0, 2005, total);
        assert_eq!(claimable, 0);
        // 1% elapsed => 10_000 vested
        let claimable = calculate_vesting_claimable(total, start, end, &pieces, 0, 2100, total);
        assert_eq!(claimable, 10_000);
    }

    #[test]
    fn end_of_schedule_sweeps_remaining_balance() {
        let pieces = linear_pieces(10_000);
        let claimable = calculate_vesting_claimable(1003, 2000, 12_000, &pieces, 0, 12_000, 1003);
        assert_eq!(claimable, 1003);
    }

    #[test]
    fn cliff_lump_unlock() {
        let pieces = vec![
            VestingPiece {
                kind: PIECE_KIND_CONTINUOUS,
                time_offset: 0,
                duration: 10_000,
                amount_bps: 7500,
                curve_factor: CURVE_FACTOR_LINEAR,
            },
            VestingPiece {
                kind: PIECE_KIND_CLIFF,
                time_offset: 5000,
                duration: 0,
                amount_bps: 2500,
                curve_factor: 0,
            },
        ];
        let total = 10_000_000_000_i64;
        let claimable = calculate_vesting_claimable(total, 2000, 12_000, &pieces, 0, 7000, total);
        assert_eq!(claimable, 6_250_000_000);
    }
}
