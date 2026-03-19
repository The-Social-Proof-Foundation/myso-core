// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    InsurancePolicyRow, InsuranceVaultExposureRow, InsuranceVaultRow,
    InsuranceVaultTransactionRow,
};

use crate::api::scalars::myso_address::MySoAddress;
use crate::error::RpcError;

#[derive(Clone)]
pub(crate) struct InsurancePolicy {
    inner: InsurancePolicyRow,
}

impl InsurancePolicy {
    pub(crate) fn from_row(inner: InsurancePolicyRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsurancePolicy {
    /// Unique policy identifier.
    async fn policy_id(&self) -> &str {
        &self.inner.policy_id
    }

    /// Market ID this policy covers.
    async fn market_id(&self) -> &str {
        &self.inner.market_id
    }

    /// Insured address.
    async fn insured(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.insured)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Option ID (0 or 1 for binary markets).
    async fn option_id(&self) -> i16 {
        self.inner.option_id
    }

    /// Covered amount.
    async fn covered_amount(&self) -> i64 {
        self.inner.covered_amount
    }

    /// Coverage in basis points.
    async fn coverage_bps(&self) -> i64 {
        self.inner.coverage_bps
    }

    /// Premium paid.
    async fn premium_paid(&self) -> i64 {
        self.inner.premium_paid
    }

    /// Policy start time (epoch milliseconds).
    async fn start_time_ms(&self) -> i64 {
        self.inner.start_time_ms
    }

    /// Policy expiry time (epoch milliseconds).
    async fn expiry_time_ms(&self) -> i64 {
        self.inner.expiry_time_ms
    }

    /// Vault ID backing this policy.
    async fn vault_id(&self) -> &str {
        &self.inner.vault_id
    }

    /// Policy status (1=ACTIVE, 2=CANCELLED, 3=CLAIMED, 4=EXPIRED).
    async fn status(&self) -> i16 {
        self.inner.status
    }

    /// Created at (epoch milliseconds).
    async fn created_at(&self) -> i64 {
        self.inner.created_at.and_utc().timestamp_millis()
    }

    /// Updated at (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at.and_utc().timestamp_millis()
    }

    /// Transaction ID of the policy creation.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct InsuranceVault {
    inner: InsuranceVaultRow,
}

impl InsuranceVault {
    pub(crate) fn from_row(inner: InsuranceVaultRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceVault {
    /// Unique vault identifier.
    async fn vault_id(&self) -> &str {
        &self.inner.vault_id
    }

    /// Underwriter address.
    async fn underwriter(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.underwriter)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Capital balance.
    async fn capital_balance(&self) -> i64 {
        self.inner.capital_balance
    }

    /// Reserved amount (locked for active policies).
    async fn reserved(&self) -> i64 {
        self.inner.reserved
    }

    /// Base rate in basis points per day.
    async fn base_rate_bps_per_day(&self) -> i64 {
        self.inner.base_rate_bps_per_day
    }

    /// Utilization multiplier in basis points.
    async fn utilization_multiplier_bps(&self) -> i64 {
        self.inner.utilization_multiplier_bps
    }

    /// Maximum exposure per market.
    async fn max_exposure_per_market(&self) -> i64 {
        self.inner.max_exposure_per_market
    }

    /// Maximum exposure per user.
    async fn max_exposure_per_user(&self) -> i64 {
        self.inner.max_exposure_per_user
    }

    /// Vault version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Created at (epoch milliseconds).
    async fn created_at(&self) -> i64 {
        self.inner.created_at.and_utc().timestamp_millis()
    }

    /// Updated at (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at.and_utc().timestamp_millis()
    }

    /// Transaction ID of the vault creation.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// Vault transactions (paginated). Returns empty when social DB not configured.
    async fn transactions(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<InsuranceVaultTransaction>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_insurance_vault_transactions(&self.inner.vault_id, limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(InsuranceVaultTransaction::from_row).collect()),
        )
    }

    /// Vault exposures by market/option. Returns empty when social DB not configured.
    async fn exposures(&self, ctx: &Context<'_>) -> Option<Result<Vec<InsuranceVaultExposure>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_insurance_vault_exposures(&self.inner.vault_id)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(InsuranceVaultExposure::from_row).collect()),
        )
    }
}

#[derive(Clone)]
pub(crate) struct InsuranceVaultTransaction {
    inner: InsuranceVaultTransactionRow,
}

impl InsuranceVaultTransaction {
    pub(crate) fn from_row(inner: InsuranceVaultTransactionRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceVaultTransaction {
    /// Transaction type (e.g. deposit, withdraw, reserve, release).
    async fn transaction_type(&self) -> &str {
        &self.inner.transaction_type
    }

    /// Amount (positive for credits, negative for debits).
    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    /// Balance after this transaction.
    async fn balance_after(&self) -> i64 {
        self.inner.balance_after
    }

    /// Timestamp (epoch milliseconds).
    async fn timestamp_ms(&self) -> i64 {
        self.inner.timestamp_ms
    }
}

#[derive(Clone)]
pub(crate) struct InsuranceVaultExposure {
    inner: InsuranceVaultExposureRow,
}

impl InsuranceVaultExposure {
    pub(crate) fn from_row(inner: InsuranceVaultExposureRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceVaultExposure {
    /// Market ID.
    async fn market_id(&self) -> &str {
        &self.inner.market_id
    }

    /// Option ID (0 or 1).
    async fn option_id(&self) -> i16 {
        self.inner.option_id
    }

    /// Total exposure (reserved amount) for this market/option.
    async fn total_exposure(&self) -> i64 {
        self.inner.total_exposure
    }
}
