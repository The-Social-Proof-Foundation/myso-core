// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{SpotBetRow, SpotRecordRow};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::profile_summary::ProfileSummary;

fn parse_betting_options(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_option_escrow(value: &serde_json::Value) -> Vec<(i16, i64)> {
    value
        .as_object()
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| {
                    let option_id: i16 = k.parse().ok()?;
                    let amount: i64 = v.as_i64()?;
                    Some((option_id, amount))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn total_escrow_from_option_escrow(value: &serde_json::Value) -> i64 {
    parse_option_escrow(value).into_iter().map(|(_, amt)| amt).sum()
}

#[derive(Clone)]
pub(crate) struct SpotBet {
    inner: SpotBetRow,
}

impl SpotBet {
    pub(crate) fn from_row(inner: SpotBetRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotBet {
    /// Unique bet identifier.
    async fn bet_id(&self) -> String {
        self.inner.id.to_string()
    }

    /// Post ID this bet is for.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Address of the bettor.
    async fn better(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.user_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the bettor.
    async fn profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.user_address).await
    }

    /// Total amount staked (escrow + AMM).
    async fn amount(&self) -> i64 {
        self.inner.escrow_amount + self.inner.amm_amount
    }

    /// Option ID the bettor chose (outcome bet on).
    async fn outcome(&self) -> i16 {
        self.inner.option_id
    }

    /// Human-readable option label (e.g. "Yes", "No").
    async fn option_label(&self) -> Option<&str> {
        self.inner.option_label.as_deref()
    }

    /// Epoch timestamp when the bet was placed.
    async fn placed_at(&self) -> i64 {
        self.inner.timestamp_epoch
    }

    /// Escrow amount.
    async fn escrow_amount(&self) -> i64 {
        self.inner.escrow_amount
    }

    /// AMM amount.
    async fn amm_amount(&self) -> i64 {
        self.inner.amm_amount
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct SpotRecord {
    inner: SpotRecordRow,
}

impl SpotRecord {
    pub(crate) fn from_row(inner: SpotRecordRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotRecord {
    /// Unique record identifier.
    async fn record_id(&self) -> String {
        self.inner.id.to_string()
    }

    /// Post ID this record is for.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Resolved outcome (when resolved).
    async fn resolution(&self) -> Option<i16> {
        self.inner.outcome
    }

    /// Record status (1=open, 2=dao_required, 3=resolved, 4=refundable).
    async fn status(&self) -> i16 {
        self.inner.status
    }

    /// Betting option labels (e.g. ["Yes", "No"]).
    async fn betting_options(&self) -> Vec<String> {
        parse_betting_options(&self.inner.betting_options)
    }

    /// Escrow amount per option (option_id -> amount).
    async fn option_escrow(&self) -> Vec<SpotOptionEscrow> {
        parse_option_escrow(&self.inner.option_escrow)
            .into_iter()
            .map(|(option_id, amount)| SpotOptionEscrow { option_id, amount })
            .collect()
    }

    /// Epoch when the record was created.
    async fn created_epoch(&self) -> i64 {
        self.inner.created_epoch
    }

    /// Resolution window in epochs.
    async fn resolution_window_epochs(&self) -> Option<i64> {
        self.inner.resolution_window_epochs
    }

    /// Max resolution window in epochs.
    async fn max_resolution_window_epochs(&self) -> Option<i64> {
        self.inner.max_resolution_window_epochs
    }

    /// Last resolution epoch.
    async fn last_resolution_epoch(&self) -> Option<i64> {
        self.inner.last_resolution_epoch
    }

    /// Total escrow across all options.
    async fn total_escrow(&self) -> i64 {
        total_escrow_from_option_escrow(&self.inner.option_escrow)
    }
}

#[derive(Clone)]
pub(crate) struct SpotOptionEscrow {
    option_id: i16,
    amount: i64,
}

#[Object]
impl SpotOptionEscrow {
    /// Option ID.
    async fn option_id(&self) -> i16 {
        self.option_id
    }

    /// Escrow amount for this option.
    async fn amount(&self) -> i64 {
        self.amount
    }
}
