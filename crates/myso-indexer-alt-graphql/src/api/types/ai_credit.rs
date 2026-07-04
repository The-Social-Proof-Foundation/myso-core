// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Context, Object};
use myso_indexer_alt_social_schema::models::{AiCreditBalanceRow, AiCreditUsageLineRow};

use crate::api::scalars::date_time::DateTime;
use crate::api::types::enterprise::{AiCreditAgentBudget, SpendApproval};

#[derive(Clone)]
pub(crate) struct AiCreditUsageLine {
    inner: AiCreditUsageLineRow,
}

impl AiCreditUsageLine {
    fn from_row(row: AiCreditUsageLineRow) -> Self {
        Self { inner: row }
    }
}

#[Object]
impl AiCreditUsageLine {
    async fn receipt_id(&self) -> &str {
        &self.inner.receipt_id
    }

    async fn amount_mist(&self) -> i64 {
        self.inner.amount_mist
    }

    async fn usage_kind(&self) -> i16 {
        self.inner.usage_kind
    }

    async fn model_id(&self) -> Option<&str> {
        self.inner.model_id.as_deref()
    }

    async fn settled(&self) -> bool {
        self.inner.settled
    }

    async fn settlement_tx(&self) -> Option<&str> {
        self.inner.settlement_tx.as_deref()
    }

    async fn created_at(&self) -> DateTime {
        DateTime::from_chrono(self.inner.created_at)
    }
}

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

    async fn usage_history(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 20)] first: i32,
    ) -> Option<Vec<AiCreditUsageLine>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = first.clamp(1, 100) as i64;
        reader
            .list_ai_credit_usage_lines(&self.inner.balance_id, limit)
            .await
            .ok()
            .map(|rows| rows.into_iter().map(AiCreditUsageLine::from_row).collect())
    }

    async fn pending_approvals(&self, ctx: &Context<'_>) -> Option<Vec<SpendApproval>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_pending_spend_approvals_for_balance(&self.inner.balance_id)
            .await
            .ok()
            .map(|rows| rows.into_iter().map(SpendApproval::from_row).collect())
    }

    async fn agent_budgets(&self, ctx: &Context<'_>) -> Option<Vec<AiCreditAgentBudget>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_agent_budgets_for_balance(&self.inner.balance_id)
            .await
            .ok()
            .map(|rows| {
                rows.into_iter()
                    .map(AiCreditAgentBudget::from_row)
                    .collect()
            })
    }
}
