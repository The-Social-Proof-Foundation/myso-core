// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VestingWalletRow {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub start_time: i64,
    pub duration: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct VestingEventRow {
    pub wallet_id: String,
    pub event_type: String,
    pub amount: i64,
    pub event_time: i64,
}
