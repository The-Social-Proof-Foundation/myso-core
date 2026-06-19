// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    VestingLeaderboardEntry as LeaderboardEntryRow, VestingWalletWithStatus as VestingWalletRow,
};
use myso_indexer_alt_social_schema::models::{PIECE_KIND_CLIFF, VestingPiece};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::profile_summary::ProfileSummary;

#[derive(Clone)]
pub(crate) struct GraphVestingPiece {
    inner: VestingPiece,
    total_amount: i64,
}

#[Object]
impl GraphVestingPiece {
    async fn kind(&self) -> &str {
        if self.inner.kind == PIECE_KIND_CLIFF {
            "CLIFF_LUMP"
        } else {
            "CONTINUOUS_VEST"
        }
    }

    async fn time_offset(&self) -> i64 {
        self.inner.time_offset
    }

    async fn duration(&self) -> i64 {
        self.inner.duration
    }

    async fn amount_bps(&self) -> i64 {
        self.inner.amount_bps
    }

    async fn curve_factor(&self) -> i64 {
        self.inner.curve_factor
    }

    async fn piece_amount(&self) -> i64 {
        self.inner.piece_amount(self.total_amount)
    }
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
    async fn wallet_id(&self) -> &str {
        &self.inner.wallet.wallet_id
    }

    async fn owner_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.wallet.owner_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn owner_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.wallet.owner_address).await
    }

    async fn total_amount(&self) -> i64 {
        self.inner.wallet.total_amount
    }

    async fn start_time(&self) -> i64 {
        self.inner.wallet.start_time
    }

    async fn schedule_end(&self) -> i64 {
        self.inner.wallet.schedule_end
    }

    async fn pieces(&self) -> Vec<GraphVestingPiece> {
        self.inner
            .wallet
            .pieces
            .iter()
            .map(|p| GraphVestingPiece {
                inner: *p,
                total_amount: self.inner.wallet.total_amount,
            })
            .collect()
    }

    async fn next_cliff(&self) -> Option<GraphVestingPiece> {
        let now = chrono::Utc::now().timestamp_millis();
        self.inner
            .wallet
            .pieces
            .iter()
            .filter(|p| p.kind == PIECE_KIND_CLIFF)
            .filter(|p| self.inner.wallet.start_time + p.time_offset > now)
            .min_by_key(|p| p.time_offset)
            .map(|p| GraphVestingPiece {
                inner: *p,
                total_amount: self.inner.wallet.total_amount,
            })
    }

    async fn claimable_amount(&self) -> i64 {
        self.inner.claimable_amount
    }

    async fn claimed_amount(&self) -> i64 {
        self.inner.wallet.claimed_amount
    }

    async fn remaining_balance(&self) -> i64 {
        self.inner.wallet.remaining_balance
    }

    async fn claimed_percentage(&self) -> f64 {
        self.inner.claimed_percentage
    }

    async fn vesting_progress(&self) -> f64 {
        self.inner.vesting_progress
    }

    async fn has_started(&self) -> bool {
        self.inner.has_started
    }

    async fn has_ended(&self) -> bool {
        self.inner.has_ended
    }

    async fn end_time(&self) -> i64 {
        self.inner.end_time
    }

    async fn created_at(&self) -> i64 {
        self.inner.wallet.created_at.and_utc().timestamp_millis()
    }

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
    async fn owner_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn total_vested(&self) -> i64 {
        self.inner.total_vested
    }

    async fn total_claimed(&self) -> i64 {
        self.inner.total_claimed
    }

    async fn active_wallets(&self) -> i64 {
        self.inner.active_wallets
    }

    async fn completed_wallets(&self) -> i64 {
        self.inner.completed_wallets
    }

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
    async fn entries(&self) -> &[VestingLeaderboardEntry] {
        &self.entries
    }

    async fn total(&self) -> i64 {
        self.total
    }
}
