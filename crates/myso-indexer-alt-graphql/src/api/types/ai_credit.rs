// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;
use myso_indexer_alt_social_schema::models::AiCreditBalanceRow;

#[derive(Clone)]
pub(crate) struct AiCreditBalance {
    inner: AiCreditBalanceRow,
}

impl AiCreditBalance {
    pub(crate) fn from_row(inner: AiCreditBalanceRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl AiCreditBalance {
    async fn balance_id(&self) -> &str {
        &self.inner.balance_id
    }

    async fn balance_mist(&self) -> i64 {
        self.inner.balance_mist
    }

    async fn credits(&self) -> i64 {
        self.inner.balance_mist / 1_000_000_000
    }

    async fn spent_total_mist(&self) -> i64 {
        self.inner.spent_total_mist
    }

    async fn daily_cap_mist(&self) -> Option<i64> {
        self.inner.daily_cap_mist
    }

    async fn monthly_cap_mist(&self) -> Option<i64> {
        self.inner.monthly_cap_mist
    }

    async fn active(&self) -> bool {
        self.inner.active
    }

    async fn settlement_nonce(&self) -> i64 {
        self.inner.settlement_nonce
    }
}
