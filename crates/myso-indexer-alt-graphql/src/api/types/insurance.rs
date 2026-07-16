// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Value;
use myso_indexer_alt_social_reader::{
    InsuranceCoverageRouteRow, InsuranceModuleEventRow, InsurancePolicyEventHistoryRow,
    InsurancePolicyRow, InsuranceRouteFillRow, InsuranceUserExposureAggRow,
    InsuranceVaultExposureRow, InsuranceVaultRow, InsuranceVaultTransactionRow,
};

use crate::api::scalars::json::Json;
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

    /// Raw utilization × risk multiplier before minimum premium floor.
    async fn premium_raw(&self) -> i64 {
        self.inner.premium_raw
    }

    /// Implied probability of insured option winning (`p_win`), basis points out of 10_000.
    async fn implied_probability_bps(&self) -> i64 {
        self.inner.implied_probability_bps
    }

    /// Combined risk multiplier in basis points (post caps; premium ≈ base × this / 10_000 before floor).
    async fn risk_multiplier_bps(&self) -> i64 {
        self.inner.risk_multiplier_bps
    }

    /// Vault utilization curve component before SPoT risk layering.
    async fn base_premium(&self) -> i64 {
        self.inner.base_premium
    }

    /// Total SPoT pool (`total_option_escrow`) at quote time.
    async fn market_total_amount(&self) -> i64 {
        self.inner.market_total_amount
    }

    /// Option-side SPoT escrow (`option_amount`) at quote time.
    async fn option_escrow_amount(&self) -> i64 {
        self.inner.option_escrow_amount
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

    /// On-chain contract version for this policy object.
    async fn contract_version(&self) -> i64 {
        self.inner.contract_version
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

    /// Aggregated route object id when this policy was bought via `route_buy_coverage_4`.
    async fn route_id(&self) -> Option<MySoAddress> {
        self.inner
            .route_id
            .as_ref()
            .and_then(|s| MySoAddress::from_str(s).ok())
    }

    /// Zero-based leg index within the route (only when `route_id` is set).
    async fn route_leg_index(&self) -> Option<i16> {
        self.inner.route_leg_index
    }

    /// Portion of premium swept to the insurance backstop pool for this leg.
    async fn backstop_sweep_amount(&self) -> i64 {
        self.inner.backstop_sweep_amount
    }

    /// Indexed lifecycle rows for this policy (`insurance_policy_events`). Returns null when social DB not configured.
    async fn policy_events(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<InsurancePolicyEvent>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_insurance_policy_events_for_policy(&self.inner.policy_id, limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(InsurancePolicyEvent::from_row).collect()),
        )
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

    /// Maximum exposure reserved per option (0 = unlimited).
    async fn max_exposure_per_option(&self) -> i64 {
        self.inner.max_exposure_per_option
    }

    /// Whether the vault accepts new coverage.
    async fn enabled(&self) -> bool {
        self.inner.enabled
    }

    /// Whether the vault is admin-paused for new coverage.
    async fn paused(&self) -> bool {
        self.inner.paused
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
                .map(|v| {
                    v.into_iter()
                        .map(InsuranceVaultTransaction::from_row)
                        .collect()
                }),
        )
    }

    /// Vault exposures by market/option. Returns empty when social DB not configured.
    async fn exposures(
        &self,
        ctx: &Context<'_>,
    ) -> Option<Result<Vec<InsuranceVaultExposure>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_insurance_vault_exposures(&self.inner.vault_id)
                .await
                .map_err(Into::into)
                .map(|v| {
                    v.into_iter()
                        .map(InsuranceVaultExposure::from_row)
                        .collect()
                }),
        )
    }

    /// Per-insured reserved totals for this vault (`insurance_user_exposures`). Returns null when social DB not configured.
    async fn user_exposure_totals(
        &self,
        ctx: &Context<'_>,
    ) -> Option<Result<Vec<InsuranceVaultUserExposure>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .list_insurance_user_exposure_totals_for_vault(&self.inner.vault_id)
                .await
                .map_err(Into::into)
                .map(|v| {
                    v.into_iter()
                        .map(InsuranceVaultUserExposure::from_row)
                        .collect()
                }),
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

#[derive(Clone)]
pub(crate) struct InsuranceModuleEvent {
    inner: InsuranceModuleEventRow,
}

impl InsuranceModuleEvent {
    pub(crate) fn from_row(inner: InsuranceModuleEventRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceModuleEvent {
    async fn id(&self) -> i32 {
        self.inner.id
    }

    async fn event_type(&self) -> &str {
        &self.inner.event_type
    }

    async fn event_data(&self) -> Json {
        Json::try_from(self.inner.event_data.clone()).unwrap_or_else(|_| Json::from(Value::Null))
    }

    async fn event_id(&self) -> &str {
        &self.inner.event_id
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at.timestamp_millis()
    }
}

#[derive(Clone)]
pub(crate) struct InsurancePolicyEvent {
    inner: InsurancePolicyEventHistoryRow,
}

impl InsurancePolicyEvent {
    pub(crate) fn from_row(inner: InsurancePolicyEventHistoryRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsurancePolicyEvent {
    async fn id(&self) -> i32 {
        self.inner.id
    }

    async fn policy_id(&self) -> &str {
        &self.inner.policy_id
    }

    async fn event_type(&self) -> &str {
        &self.inner.event_type
    }

    async fn market_id(&self) -> &str {
        &self.inner.market_id
    }

    async fn insured(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.insured)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn option_id(&self) -> i16 {
        self.inner.option_id
    }

    async fn covered_amount(&self) -> i64 {
        self.inner.covered_amount
    }

    async fn coverage_bps(&self) -> i64 {
        self.inner.coverage_bps
    }

    async fn premium_paid(&self) -> i64 {
        self.inner.premium_paid
    }

    async fn reserve_locked(&self) -> i64 {
        self.inner.reserve_locked
    }

    async fn premium_raw(&self) -> Option<i64> {
        self.inner.premium_raw
    }

    async fn implied_probability_bps(&self) -> Option<i64> {
        self.inner.implied_probability_bps
    }

    async fn risk_multiplier_bps(&self) -> Option<i64> {
        self.inner.risk_multiplier_bps
    }

    async fn base_premium(&self) -> Option<i64> {
        self.inner.base_premium
    }

    async fn market_total_amount(&self) -> Option<i64> {
        self.inner.market_total_amount
    }

    async fn option_escrow_amount(&self) -> Option<i64> {
        self.inner.option_escrow_amount
    }

    async fn refunded_amount(&self) -> Option<i64> {
        self.inner.refunded_amount
    }

    async fn fee_paid(&self) -> Option<i64> {
        self.inner.fee_paid
    }

    async fn payout(&self) -> Option<i64> {
        self.inner.payout
    }

    async fn timestamp_ms(&self) -> i64 {
        self.inner.timestamp_ms
    }

    async fn time(&self) -> i64 {
        self.inner.time.timestamp_millis()
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct InsuranceVaultUserExposure {
    inner: InsuranceUserExposureAggRow,
}

impl InsuranceVaultUserExposure {
    pub(crate) fn from_row(inner: InsuranceUserExposureAggRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceVaultUserExposure {
    async fn insured(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.insured)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn total_reserved(&self) -> i64 {
        self.inner.total_reserved
    }
}

#[derive(Clone)]
pub(crate) struct InsuranceCoverageRoute {
    inner: InsuranceCoverageRouteRow,
}

impl InsuranceCoverageRoute {
    pub(crate) fn from_row(inner: InsuranceCoverageRouteRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceCoverageRoute {
    async fn route_id(&self) -> &str {
        &self.inner.route_id
    }

    async fn contract_version(&self) -> i64 {
        self.inner.contract_version
    }

    async fn insured(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.insured)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn market_id(&self) -> &str {
        &self.inner.market_id
    }

    async fn option_id(&self) -> i16 {
        self.inner.option_id
    }

    async fn coverage_bps(&self) -> i64 {
        self.inner.coverage_bps
    }

    async fn duration_ms(&self) -> i64 {
        self.inner.duration_ms
    }

    async fn total_covered(&self) -> i64 {
        self.inner.total_covered
    }

    async fn total_premium(&self) -> i64 {
        self.inner.total_premium
    }

    async fn total_reserve(&self) -> i64 {
        self.inner.total_reserve
    }

    async fn total_backstop_sweep(&self) -> i64 {
        self.inner.total_backstop_sweep
    }

    async fn expiry_time_ms(&self) -> i64 {
        self.inner.expiry_time_ms
    }

    async fn policy_ids(&self) -> Json {
        Json::try_from(self.inner.policy_ids.clone()).unwrap_or_else(|_| Json::from(Value::Null))
    }

    async fn vault_ids(&self) -> Json {
        Json::try_from(self.inner.vault_ids.clone()).unwrap_or_else(|_| Json::from(Value::Null))
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at.and_utc().timestamp_millis()
    }
}

#[derive(Clone)]
pub(crate) struct InsuranceRouteFill {
    inner: InsuranceRouteFillRow,
}

impl InsuranceRouteFill {
    pub(crate) fn from_row(inner: InsuranceRouteFillRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceRouteFill {
    async fn id(&self) -> i64 {
        self.inner.id
    }

    async fn route_id(&self) -> &str {
        &self.inner.route_id
    }

    async fn leg_index(&self) -> i16 {
        self.inner.leg_index
    }

    async fn vault_id(&self) -> &str {
        &self.inner.vault_id
    }

    async fn policy_id(&self) -> &str {
        &self.inner.policy_id
    }

    async fn covered_amount(&self) -> i64 {
        self.inner.covered_amount
    }

    async fn premium_paid(&self) -> i64 {
        self.inner.premium_paid
    }

    async fn reserve_locked(&self) -> i64 {
        self.inner.reserve_locked
    }

    async fn backstop_sweep_amount(&self) -> i64 {
        self.inner.backstop_sweep_amount
    }

    async fn event_id(&self) -> &str {
        &self.inner.event_id
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn timestamp_ms(&self) -> i64 {
        self.inner.timestamp_ms
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at.and_utc().timestamp_millis()
    }
}
