// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;
use chrono::DateTime;
use myso_indexer_alt_social_reader::{
    VestingLeaderboardEntry as LeaderboardEntryRow, VestingWalletWithStatus as VestingWalletRow,
};

use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::profile_summary::ProfileSummary;

fn to_iso8601_utc(dt: chrono::NaiveDateTime) -> String {
    DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

#[derive(Clone)]
pub(crate) struct VestingWallet {
    inner: VestingWalletRow,
}

impl VestingWallet {
    pub(crate) fn from_row(inner: VestingWalletRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl VestingWallet {
    /// Vesting wallet ID (object address).
    async fn wallet_id(&self) -> &str {
        &self.inner.wallet.wallet_id
    }

    /// Owner address.
    async fn owner_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.wallet.owner_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Total amount vested.
    async fn total_amount(&self) -> i64 {
        self.inner.wallet.total_amount
    }

    /// Vesting start time (ms since epoch).
    async fn start_time(&self) -> i64 {
        self.inner.wallet.start_time
    }

    /// Vesting duration (ms).
    async fn duration(&self) -> i64 {
        self.inner.wallet.duration
    }

    /// Curve factor (1000 = linear).
    async fn curve_factor(&self) -> i64 {
        self.inner.wallet.curve_factor
    }

    /// Amount already claimed.
    async fn claimed_amount(&self) -> i64 {
        self.inner.wallet.claimed_amount
    }

    /// Remaining balance.
    async fn remaining_balance(&self) -> i64 {
        self.inner.wallet.remaining_balance
    }

    /// Claimed percentage (0-100).
    async fn claimed_percentage(&self) -> f64 {
        self.inner.claimed_percentage
    }

    /// Vesting progress (0.0-1.0).
    async fn vesting_progress(&self) -> f64 {
        self.inner.vesting_progress
    }

    /// Whether vesting has started.
    async fn has_started(&self) -> bool {
        self.inner.has_started
    }

    /// Whether vesting has ended.
    async fn has_ended(&self) -> bool {
        self.inner.has_ended
    }

    /// End time (ms since epoch).
    async fn end_time(&self) -> i64 {
        self.inner.end_time
    }

    /// When the wallet was created (ISO 8601).
    async fn created_at(&self) -> String {
        to_iso8601_utc(self.inner.wallet.created_at)
    }

    /// Transaction ID that created the wallet.
    async fn transaction_id(&self) -> &str {
        &self.inner.wallet.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct VestingLeaderboardEntry {
    inner: LeaderboardEntryRow,
}

impl VestingLeaderboardEntry {
    pub(crate) fn from_row(inner: LeaderboardEntryRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl VestingLeaderboardEntry {
    /// Owner address.
    async fn owner_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Total amount vested across all wallets.
    async fn total_vested(&self) -> i64 {
        self.inner.total_vested
    }

    /// Total amount claimed across all wallets.
    async fn total_claimed(&self) -> i64 {
        self.inner.total_claimed
    }

    /// Number of active vesting wallets.
    async fn active_wallets(&self) -> i64 {
        self.inner.active_wallets
    }

    /// Number of completed vesting wallets.
    async fn completed_wallets(&self) -> i64 {
        self.inner.completed_wallets
    }

    /// User profile summary.
    async fn user(&self) -> ProfileSummary {
        ProfileSummary::from_row(self.inner.user.clone())
    }
}

#[derive(Clone)]
pub(crate) struct VestingLeaderboardResponse {
    pub entries: Vec<VestingLeaderboardEntry>,
    pub total: i64,
}

#[Object]
impl VestingLeaderboardResponse {
    /// Leaderboard entries.
    async fn entries(&self) -> &[VestingLeaderboardEntry] {
        &self.entries
    }

    /// Total number of unique owners.
    async fn total(&self) -> i64 {
        self.total
    }
}
