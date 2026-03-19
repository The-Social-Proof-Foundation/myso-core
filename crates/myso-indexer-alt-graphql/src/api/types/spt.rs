// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Enum;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    SptHoldingRow, SptPriceHistory as SptPriceHistoryRow, SptPoolRow, SptSortBy as SptSortByReader,
    SptTransaction as SptTransactionRow,
};
use myso_indexer_alt_social_schema::models::UserReservationHoldingRow;

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::profile_summary::ProfileSummary;

fn to_iso8601_utc(dt: chrono::DateTime<chrono::Utc>) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Default)]
pub enum SptOrder {
    #[default]
    Desc,
    Asc,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Default)]
pub enum SptSortBy {
    #[default]
    Price,
    MarketCap,
    PriceChange24h,
    Volume24h,
    CreatorEarnings,
    PlatformEarnings,
    EcosystemEarnings,
    TotalEarnings,
    CreatedAt,
}

impl From<SptSortBy> for SptSortByReader {
    fn from(v: SptSortBy) -> Self {
        match v {
            SptSortBy::Price => SptSortByReader::Price,
            SptSortBy::MarketCap => SptSortByReader::MarketCap,
            SptSortBy::PriceChange24h => SptSortByReader::PriceChange24h,
            SptSortBy::Volume24h => SptSortByReader::Volume24h,
            SptSortBy::CreatorEarnings => SptSortByReader::CreatorEarnings,
            SptSortBy::PlatformEarnings => SptSortByReader::PlatformEarnings,
            SptSortBy::EcosystemEarnings => SptSortByReader::EcosystemEarnings,
            SptSortBy::TotalEarnings => SptSortByReader::TotalEarnings,
            SptSortBy::CreatedAt => SptSortByReader::CreatedAt,
        }
    }
}

#[derive(Clone)]
pub(crate) struct SptHolding {
    inner: SptHoldingRow,
}

impl SptHolding {
    pub(crate) fn from_row(inner: SptHoldingRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SptHolding {
    /// Holder wallet address.
    async fn address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.holder_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Token balance held.
    async fn amount(&self) -> i64 {
        self.inner.balance
    }

    /// Profile whose token this holding represents.
    async fn profile(&self) -> ProfileSummary {
        ProfileSummary::from_row(myso_indexer_alt_social_reader::ProfileSummaryRow {
            owner_address: self.inner.profile_owner_address.clone(),
            username: self.inner.profile_username.clone(),
            display_name: self.inner.profile_display_name.clone(),
            profile_photo: self.inner.profile_photo.clone(),
            bio: self.inner.profile_bio.clone(),
            selected_badge_id: self.inner.profile_selected_badge_id.clone(),
            social_proof_token_address: self.inner.profile_social_proof_token_address.clone(),
            reservation_pool_address: self.inner.profile_reservation_pool_address.clone(),
            followers_count: None,
            following_count: None,
            post_count: None,
            blocked_count: None,
            is_following: None,
            follows_viewer: None,
        })
    }
}

#[derive(Clone)]
pub(crate) struct SptPool {
    inner: SptPoolRow,
}

impl SptPool {
    pub(crate) fn from_row(inner: SptPoolRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SptPool {
    /// Pool ID.
    async fn pool_id(&self) -> &str {
        &self.inner.pool_id
    }

    /// Total circulating supply.
    async fn total_supply(&self) -> i64 {
        self.inner.circulating_supply
    }

    /// Current price (smallest units).
    async fn price(&self) -> i64 {
        self.inner.price
    }

    /// Token symbol.
    async fn symbol(&self) -> &str {
        &self.inner.symbol
    }

    /// Token name.
    async fn name(&self) -> &str {
        &self.inner.name
    }

    /// Pool owner address.
    async fn owner(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the pool owner.
    async fn owner_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.owner).await
    }

    /// Token type (1=profile, 2=post).
    async fn token_type(&self) -> i16 {
        self.inner.token_type
    }

    /// Market cap (price * circulating supply).
    async fn market_cap(&self) -> i64 {
        self.inner.price.saturating_mul(self.inner.circulating_supply)
    }

    /// 24-hour price change (percentage).
    async fn price_change_24h(&self) -> Option<f64> {
        self.inner.price_24h_ago.and_then(|prev| {
            if prev > 0 {
                Some(((self.inner.price - prev) as f64 / prev as f64) * 100.0)
            } else {
                None
            }
        })
    }

    /// 24-hour trading volume (MYSO).
    async fn volume_24h(&self) -> Option<i64> {
        self.inner.volume_24h
    }

    /// Total creator fees earned.
    async fn creator_earnings(&self) -> Option<i64> {
        self.inner.creator_earnings
    }

    /// Total platform fees earned.
    async fn platform_earnings(&self) -> Option<i64> {
        self.inner.platform_earnings
    }

    /// Total ecosystem/treasury fees earned.
    async fn ecosystem_earnings(&self) -> Option<i64> {
        self.inner.ecosystem_earnings
    }

    /// Recent transactions for this pool.
    async fn transactions(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptTransaction>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_spt_transactions(&self.inner.pool_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SptTransaction::from_row).collect())
    }

    /// Price history for this pool.
    async fn price_history(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptPriceHistory>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_spt_price_history(&self.inner.pool_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SptPriceHistory::from_row).collect())
    }

    /// Current holders of this token (paginated, ordered by balance DESC).
    async fn holders(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptHolding>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_spt_holdings_by_pool(&self.inner.pool_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SptHolding::from_row).collect())
    }
}

#[derive(Clone)]
pub(crate) struct SptTransaction {
    inner: SptTransactionRow,
}

impl SptTransaction {
    pub(crate) fn from_row(inner: SptTransactionRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SptTransaction {
    /// Transaction type (BUY or SELL).
    async fn r#type(&self) -> &str {
        &self.inner.transaction_type
    }

    /// Token amount.
    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    /// Sender address.
    async fn from(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.sender)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Counterparty (pool ID).
    async fn to(&self) -> Option<&str> {
        Some(&self.inner.pool_id)
    }

    /// Transaction timestamp (ISO 8601).
    async fn timestamp(&self) -> String {
        to_iso8601_utc(self.inner.time)
    }
}

#[derive(Clone)]
pub(crate) struct SptPriceHistory {
    inner: SptPriceHistoryRow,
}

impl SptPriceHistory {
    pub(crate) fn from_row(inner: SptPriceHistoryRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SptPriceHistory {
    /// Pool ID.
    async fn pool_id(&self) -> &str {
        &self.inner.pool_id
    }

    /// Price at this point.
    async fn price(&self) -> i64 {
        self.inner.price
    }

    /// Circulating supply at this point.
    async fn circulating_supply(&self) -> i64 {
        self.inner.circulating_supply
    }

    /// Timestamp (ISO 8601).
    async fn timestamp(&self) -> String {
        to_iso8601_utc(self.inner.time)
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct SptReservationHolding {
    inner: UserReservationHoldingRow,
}

impl SptReservationHolding {
    pub(crate) fn from_row(inner: UserReservationHoldingRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SptReservationHolding {
    /// Reservation pool ID.
    async fn pool_id(&self) -> &str {
        &self.inner.pool_id
    }

    /// Reserved amount.
    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    /// Epoch timestamp when reserved.
    async fn reserved_at(&self) -> i64 {
        self.inner.reserved_at
    }

    /// Profile whose token this reservation is for.
    async fn profile(&self) -> ProfileSummary {
        ProfileSummary::from_row(myso_indexer_alt_social_reader::ProfileSummaryRow {
            owner_address: self.inner.owner.clone(),
            username: self.inner.profile_username.clone(),
            display_name: self.inner.profile_display_name.clone(),
            profile_photo: self.inner.profile_photo.clone(),
            bio: None,
            selected_badge_id: None,
            social_proof_token_address: self.inner.profile_social_proof_token_address.clone(),
            reservation_pool_address: self.inner.profile_reservation_pool_address.clone(),
            followers_count: None,
            following_count: None,
            post_count: None,
            blocked_count: None,
            is_following: None,
            follows_viewer: None,
        })
    }

    /// Whether the reservation pool threshold is met.
    async fn threshold_met(&self) -> bool {
        self.inner.threshold_met
    }

    /// Pool status (e.g. active, threshold_met).
    async fn pool_status(&self) -> &str {
        &self.inner.pool_status
    }

    /// Total reserved across all reservers in this pool.
    async fn total_reserved(&self) -> i64 {
        self.inner.total_reserved
    }

    /// Required threshold for the pool.
    async fn required_threshold(&self) -> i64 {
        self.inner.required_threshold
    }
}
