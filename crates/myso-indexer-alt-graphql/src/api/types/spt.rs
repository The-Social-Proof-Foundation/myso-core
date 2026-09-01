// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Enum;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    SptHoldingRow, SptPoolRow, SptPriceHistory as SptPriceHistoryRow,
    SptReservationVolumeBucket as SptReservationVolumeBucketRow, SptSortBy as SptSortByReader,
    SptSwap as SptSwapRow, SptTransaction as SptTransactionRow, SptTransfer as SptTransferRow,
    ViewerSocialContext, pct_change, pct_change_i128,
};
use myso_indexer_alt_social_schema::models::{SptReservationHoldingRow, TOKEN_TYPE_POST};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::big_int::BigInt;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::post::Post;
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

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SptReservationVolumeInterval {
    Event,
    FiveMin,
    Hour,
    Day,
}

impl From<SptReservationVolumeInterval>
    for myso_indexer_alt_social_reader::SptReservationVolumeInterval
{
    fn from(v: SptReservationVolumeInterval) -> Self {
        match v {
            SptReservationVolumeInterval::Event => Self::Event,
            SptReservationVolumeInterval::FiveMin => Self::FiveMin,
            SptReservationVolumeInterval::Hour => Self::Hour,
            SptReservationVolumeInterval::Day => Self::Day,
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

    /// Token balance held (nano-SPT).
    async fn amount(&self) -> BigInt {
        BigInt::from(self.inner.balance)
    }

    /// Token type (`1` = profile, `2` = post).
    async fn token_type(&self) -> i16 {
        self.inner.token_type
    }

    /// Profile or post object id this SPT is tied to (subject of the token).
    async fn associated_id(&self) -> &str {
        &self.inner.associated_id
    }

    /// Post for this holding when `tokenType` is post; null for profile SPTs or if the post is not indexed.
    async fn post(&self, ctx: &Context<'_>) -> Option<Post> {
        if self.inner.token_type != TOKEN_TYPE_POST {
            return None;
        }
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_post_by_id(&self.inner.associated_id)
            .await
            .ok()??;
        Some(Post::from_db(row))
    }

    /// Token pool owner / creator (author) profile, not the post content for post SPTs.
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
            blocked_by_viewer: None,
            blocked_by_subject: None,
            mutual_count: None,
            mutual_connections: None,
        })
    }

    /// Viewer follows the holder (requires `viewer` on [`SptPool.holders`]).
    async fn viewer_is_following(&self) -> Option<bool> {
        self.inner.viewer_is_following
    }

    /// Holder follows the viewer.
    async fn viewer_follows_viewer(&self) -> Option<bool> {
        self.inner.viewer_follows_viewer
    }

    /// Viewer blocked the holder.
    async fn blocked_by_viewer(&self) -> Option<bool> {
        self.inner.blocked_by_viewer
    }

    /// Holder blocked the viewer.
    async fn blocked_by_subject(&self) -> Option<bool> {
        self.inner.blocked_by_subject
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

    /// Total circulating supply (nano-SPT).
    async fn total_supply(&self) -> BigInt {
        BigInt::from(self.inner.circulating_supply)
    }

    /// Current price (smallest units).
    async fn price(&self) -> i64 {
        self.inner.price
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

    /// Market cap (price in MYSO smallest units × circulating supply in nano-SPT).
    async fn market_cap(&self) -> BigInt {
        let p = self.inner.price as i128;
        let s = self.inner.circulating_supply as i128;
        BigInt::from(p * s)
    }

    /// Price change percentage vs ~24h ago, or vs first indexed price when the pool is younger than 24h.
    async fn price_change_24h(&self) -> Option<f64> {
        self.inner
            .price_24h_ago
            .and_then(|prev| pct_change(self.inner.price, prev))
    }

    /// Circulating supply change percentage vs ~24h ago, or vs first indexed supply when the pool is younger than 24h.
    async fn circulating_supply_change_24h(&self) -> Option<f64> {
        self.inner
            .circulating_supply_24h_ago
            .and_then(|prev| pct_change(self.inner.circulating_supply, prev))
    }

    /// Market cap change percentage vs ~24h ago, or vs first indexed sample when the pool is younger than 24h.
    /// Uses price and circulating supply from the same historical row as `priceChange24H`.
    async fn market_cap_change_24h(&self) -> Option<f64> {
        let prev_price = self.inner.price_24h_ago?;
        let prev_supply = self.inner.circulating_supply_24h_ago?;
        pct_change_i128(
            (self.inner.price as i128) * (self.inner.circulating_supply as i128),
            (prev_price as i128) * (prev_supply as i128),
        )
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
        viewer: Option<MySoAddress>,
        prioritize_followed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptTransaction>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let prioritize = prioritize_followed.unwrap_or(false);
        let page = reader
            .get_spt_transactions(
                &self.inner.pool_id,
                limit,
                offset,
                viewer_s.as_deref(),
                prioritize,
            )
            .await
            .ok()?;
        Some(
            page.transactions
                .into_iter()
                .map(|tx| {
                    let vctx = page
                        .viewer_by_sender
                        .as_ref()
                        .and_then(|m| m.get(&tx.sender))
                        .copied();
                    SptTransaction::with_viewer(tx, vctx)
                })
                .collect(),
        )
    }

    /// Atomic SPT→SPT swaps where this pool is the source or destination.
    async fn swaps(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptSwap>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_spt_swaps_for_pool(&self.inner.pool_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SptSwap::from_row).collect())
    }

    /// P2P SPT transfers for this pool.
    async fn transfers(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptTransfer>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_spt_transfers_for_pool(&self.inner.pool_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SptTransfer::from_row).collect())
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
        viewer: Option<MySoAddress>,
        prioritize_followed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptHolding>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let prioritize = prioritize_followed.unwrap_or(false);
        let rows = reader
            .get_spt_holdings_by_pool(
                &self.inner.pool_id,
                limit,
                offset,
                viewer_s.as_deref(),
                prioritize,
            )
            .await
            .ok()?;
        Some(rows.into_iter().map(SptHolding::from_row).collect())
    }

    /// Current reservation holders for this pool’s reservation phase (same `associated_id`).
    async fn reservation_holders(
        &self,
        ctx: &Context<'_>,
        viewer: Option<MySoAddress>,
        prioritize_followed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptReservationHolding>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let reservation_pool_id = match reader
            .get_reservation_pool_id_for_associated_id(&self.inner.associated_id)
            .await
            .ok()?
        {
            Some(id) => id,
            None => return Some(vec![]),
        };
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let prioritize = prioritize_followed.unwrap_or(false);
        let rows = reader
            .get_reservation_holdings_for_pool(
                &reservation_pool_id,
                limit,
                offset,
                viewer_s.as_deref(),
                prioritize,
            )
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(SptReservationHolding::from_row)
                .collect(),
        )
    }

    /// Former reservation holders (withdrawn; latest indexed balance zero per reserver).
    async fn former_reservation_holders(
        &self,
        ctx: &Context<'_>,
        viewer: Option<MySoAddress>,
        prioritize_followed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptReservationHolding>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let reservation_pool_id = match reader
            .get_reservation_pool_id_for_associated_id(&self.inner.associated_id)
            .await
            .ok()?
        {
            Some(id) => id,
            None => return Some(vec![]),
        };
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let prioritize = prioritize_followed.unwrap_or(false);
        let rows = reader
            .get_former_reservation_holdings_for_pool(
                &reservation_pool_id,
                limit,
                offset,
                viewer_s.as_deref(),
                prioritize,
            )
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(SptReservationHolding::from_row)
                .collect(),
        )
    }
}

#[derive(Clone)]
pub(crate) struct SptTransaction {
    inner: SptTransactionRow,
    viewer_ctx: Option<ViewerSocialContext>,
}

impl SptTransaction {
    pub(crate) fn with_viewer(
        inner: SptTransactionRow,
        viewer_ctx: Option<ViewerSocialContext>,
    ) -> Self {
        Self { inner, viewer_ctx }
    }
}

#[Object]
impl SptTransaction {
    /// Transaction type (BUY or SELL).
    async fn r#type(&self) -> &str {
        &self.inner.transaction_type
    }

    /// Token amount (nano-SPT).
    async fn amount(&self) -> BigInt {
        BigInt::from(self.inner.amount)
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

    /// `true` when this BUY/SELL row is one leg of an atomic SPT→SPT swap.
    async fn is_swap_leg(&self) -> bool {
        self.inner.is_swap_leg
    }

    /// Opposite pool ID when this row is one leg of an SPT→SPT swap; null otherwise.
    async fn counterparty_pool_id(&self) -> Option<&str> {
        self.inner.counterparty_pool_id.as_deref()
    }

    /// Viewer follows the trade sender (requires `viewer` on [`SptPool.transactions`]).
    async fn viewer_is_following(&self) -> Option<bool> {
        self.viewer_ctx.map(|c| c.is_following)
    }

    /// Sender follows the viewer.
    async fn viewer_follows_viewer(&self) -> Option<bool> {
        self.viewer_ctx.map(|c| c.follows_viewer)
    }

    /// Viewer blocked the sender.
    async fn blocked_by_viewer(&self) -> Option<bool> {
        self.viewer_ctx.map(|c| c.blocked_by_viewer)
    }

    /// Sender blocked the viewer.
    async fn blocked_by_subject(&self) -> Option<bool> {
        self.viewer_ctx.map(|c| c.blocked_by_subject)
    }
}

#[derive(Clone)]
pub(crate) struct SptSwap {
    inner: SptSwapRow,
}

impl SptSwap {
    pub(crate) fn from_row(inner: SptSwapRow) -> Self {
        Self { inner }
    }
}

/// Atomic SPT→SPT swap summary. SUMMARY ONLY: the per-pool balance/price/fee effects are
/// reflected by the underlying BUY/SELL [`SptTransaction`] legs (see `isSwapLeg`).
#[Object]
impl SptSwap {
    /// Transaction digest shared by both swap legs.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// Trader wallet address.
    async fn trader(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.trader)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Source pool the trader sold from.
    async fn source_pool_id(&self) -> &str {
        &self.inner.source_pool_id
    }

    /// Destination pool the trader bought into.
    async fn dest_pool_id(&self) -> &str {
        &self.inner.dest_pool_id
    }

    /// Nano-SPT sold from the source pool.
    async fn sell_amount(&self) -> BigInt {
        BigInt::from(self.inner.sell_amount)
    }

    /// Nano-SPT bought into the destination pool.
    async fn dest_amount(&self) -> BigInt {
        BigInt::from(self.inner.dest_amount)
    }

    /// Gross MYSO realized from the sell leg.
    async fn sell_myso_gross(&self) -> BigInt {
        BigInt::from(self.inner.sell_myso_gross)
    }

    /// Gross MYSO spent on the buy leg.
    async fn buy_myso_gross(&self) -> BigInt {
        BigInt::from(self.inner.buy_myso_gross)
    }

    /// Total fees on the sell leg.
    async fn sell_fee_amount(&self) -> BigInt {
        BigInt::from(self.inner.sell_fee_amount)
    }

    /// Total fees on the buy leg.
    async fn buy_fee_amount(&self) -> BigInt {
        BigInt::from(self.inner.buy_fee_amount)
    }

    /// Creator fee on the sell leg.
    async fn sell_creator_fee(&self) -> BigInt {
        BigInt::from(self.inner.sell_creator_fee)
    }

    /// Platform fee on the sell leg.
    async fn sell_platform_fee(&self) -> BigInt {
        BigInt::from(self.inner.sell_platform_fee)
    }

    /// Treasury fee on the sell leg.
    async fn sell_treasury_fee(&self) -> BigInt {
        BigInt::from(self.inner.sell_treasury_fee)
    }

    /// Creator fee on the buy leg.
    async fn buy_creator_fee(&self) -> BigInt {
        BigInt::from(self.inner.buy_creator_fee)
    }

    /// Platform fee on the buy leg.
    async fn buy_platform_fee(&self) -> BigInt {
        BigInt::from(self.inner.buy_platform_fee)
    }

    /// Treasury fee on the buy leg.
    async fn buy_treasury_fee(&self) -> BigInt {
        BigInt::from(self.inner.buy_treasury_fee)
    }

    /// MYSO returned to the trader after the buy leg (unspent proceeds).
    async fn leftover_myso(&self) -> BigInt {
        BigInt::from(self.inner.leftover_myso)
    }

    /// Source pool price after the swap (smallest units).
    async fn source_new_price(&self) -> i64 {
        self.inner.source_new_price
    }

    /// Destination pool price after the swap (smallest units).
    async fn dest_new_price(&self) -> i64 {
        self.inner.dest_new_price
    }

    /// Swap timestamp (ISO 8601).
    async fn timestamp(&self) -> String {
        to_iso8601_utc(self.inner.time)
    }
}

#[derive(Clone)]
pub(crate) struct SptTransfer {
    inner: SptTransferRow,
}

impl SptTransfer {
    pub(crate) fn from_row(inner: SptTransferRow) -> Self {
        Self { inner }
    }
}

/// P2P SPT transfer summary. Holdings are updated via delta rows; supply/price unchanged.
#[Object]
impl SptTransfer {
    /// Transaction digest.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// Pool ID.
    async fn pool_id(&self) -> &str {
        &self.inner.pool_id
    }

    /// Sender wallet.
    async fn from(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.from_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Recipient wallet.
    async fn to(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.to_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Nano-SPT transferred.
    async fn amount(&self) -> BigInt {
        BigInt::from(self.inner.amount)
    }

    /// Transfer timestamp (ISO 8601).
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

    /// Circulating supply at this point (nano-SPT).
    async fn circulating_supply(&self) -> BigInt {
        BigInt::from(self.inner.circulating_supply)
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
    inner: SptReservationHoldingRow,
}

impl SptReservationHolding {
    pub(crate) fn from_row(inner: SptReservationHoldingRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SptReservationHolding {
    /// Wallet that made the reservation.
    async fn reserver(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.reserver_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the reserver (when indexed).
    async fn reserver_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.reserver_address).await
    }

    /// Reservation pool ID.
    async fn pool_id(&self) -> &str {
        &self.inner.pool_id
    }

    /// Reserved amount.
    async fn amount(&self) -> BigInt {
        BigInt::from(self.inner.amount)
    }

    /// Epoch timestamp when reserved.
    async fn reserved_at(&self) -> i64 {
        self.inner.reserved_at
    }

    /// Token type (`1` = profile, `2` = post).
    async fn token_type(&self) -> i16 {
        self.inner.token_type
    }

    /// Profile or post object id this reservation pool is for (subject of the token).
    async fn associated_id(&self) -> &str {
        &self.inner.associated_id
    }

    /// Post for this reservation when `tokenType` is post; null for profile SPTs or if the post is not indexed.
    async fn post(&self, ctx: &Context<'_>) -> Option<Post> {
        if self.inner.token_type != TOKEN_TYPE_POST {
            return None;
        }
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_post_by_id(&self.inner.associated_id)
            .await
            .ok()??;
        Some(Post::from_db(row))
    }

    /// Token pool owner / creator (author) profile, not the post content for post SPTs.
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
            blocked_by_viewer: None,
            blocked_by_subject: None,
            mutual_count: None,
            mutual_connections: None,
        })
    }

    async fn viewer_is_following(&self) -> Option<bool> {
        self.inner.viewer_is_following
    }

    async fn viewer_follows_viewer(&self) -> Option<bool> {
        self.inner.viewer_follows_viewer
    }

    async fn blocked_by_viewer(&self) -> Option<bool> {
        self.inner.blocked_by_viewer
    }

    async fn blocked_by_subject(&self) -> Option<bool> {
        self.inner.blocked_by_subject
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
    async fn total_reserved(&self) -> BigInt {
        BigInt::from(self.inner.total_reserved)
    }

    /// Required threshold for the pool.
    async fn required_threshold(&self) -> BigInt {
        BigInt::from(self.inner.required_threshold)
    }
}

#[derive(Clone)]
pub(crate) struct SptReservationVolumeBucket {
    inner: SptReservationVolumeBucketRow,
}

impl SptReservationVolumeBucket {
    pub(crate) fn from_row(inner: SptReservationVolumeBucketRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SptReservationVolumeBucket {
    /// Start of the bucket, Unix epoch seconds (UTC).
    async fn bucket_start(&self) -> i64 {
        self.inner.bucket_start
    }

    /// Exclusive end of the bucket, Unix epoch seconds (UTC).
    async fn bucket_end(&self) -> i64 {
        self.inner.bucket_end
    }

    /// Earliest indexer time in this bucket, Unix epoch seconds (UTC).
    async fn earliest_at(&self) -> i64 {
        self.inner.earliest_at
    }

    /// Latest indexer time in this bucket, Unix epoch seconds (UTC).
    async fn latest_at(&self) -> i64 {
        self.inner.latest_at
    }

    /// MYSO volume from reservation deposits (positive `amount` rows) in this bucket.
    async fn deposit_volume(&self) -> BigInt {
        BigInt::from(self.inner.deposit_volume)
    }

    /// MYSO volume from reservation withdrawals (negative `amount` rows) in this bucket.
    async fn withdrawal_volume(&self) -> BigInt {
        BigInt::from(self.inner.withdrawal_volume)
    }

    /// Number of deposit events in the bucket.
    async fn deposit_count(&self) -> i64 {
        self.inner.deposit_count
    }

    /// Number of withdrawal events in the bucket.
    async fn withdrawal_count(&self) -> i64 {
        self.inner.withdrawal_count
    }
}
