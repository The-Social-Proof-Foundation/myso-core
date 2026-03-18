// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;
use myso_indexer_alt_social_reader::{SpotBetRow, SpotRecordRow};

use crate::api::scalars::myso_address::MySoAddress;

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

    /// Total amount staked (escrow + AMM).
    async fn amount(&self) -> i64 {
        self.inner.escrow_amount + self.inner.amm_amount
    }

    /// Option ID the bettor chose (outcome bet on).
    async fn outcome(&self) -> i16 {
        self.inner.option_id
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
}
