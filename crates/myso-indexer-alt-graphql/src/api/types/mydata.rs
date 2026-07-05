// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::SocialPgReader;
use myso_indexer_alt_social_schema::models::{
    MyDataAccessAnalyticsRow, MyDataAccessLogRow, MyDataBroadPoolRow, MyDataClaimRow,
    MyDataDailyRevenueRow, MyDataDistributionRoundRow, MyDataListingSubPoolRow,
    MyDataMerkleRootRow, MyDataPurchaseRow, MyDataRecordRow, MyDataRevenueRow,
    MyDataSnapshotAnchorRow, MyDataStatsRow, MyDataSubPoolRow, MyDataSubscriptionRow,
};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::date_time::DateTime;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::platform::{Platform, resolve_platform_by_id};
use crate::api::types::profile_summary::ProfileSummary;

fn parse_tags(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

#[derive(Clone)]
pub(crate) struct MyDataRecord {
    inner: MyDataRecordRow,
}

impl MyDataRecord {
    pub(crate) fn from_row(inner: MyDataRecordRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataRecord {
    /// Unique MyData record identifier.
    async fn mydata_id(&self) -> &str {
        &self.inner.mydata_id
    }

    /// Owner address.
    async fn owner(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the MyData record owner.
    async fn owner_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.owner).await
    }

    /// Media type (e.g. "text", "audio", "image", "video").
    async fn media_type(&self) -> &str {
        &self.inner.media_type
    }

    /// Searchable tags.
    async fn tags(&self) -> Vec<String> {
        parse_tags(&self.inner.tags)
    }

    /// One-time purchase price (null if not for sale).
    async fn one_time_price(&self) -> Option<i64> {
        self.inner.one_time_price
    }

    /// Subscription price (null if not for sale).
    async fn subscription_price(&self) -> Option<i64> {
        self.inner.subscription_price
    }

    /// Optional platform identification.
    async fn platform_id(&self) -> Option<&str> {
        self.inner.platform_id.as_deref()
    }

    /// Platform details when `platform_id` is set and indexed.
    async fn platform(&self, ctx: &Context<'_>) -> Option<Platform> {
        let platform_id = self.inner.platform_id.as_deref()?;
        resolve_platform_by_id(ctx, platform_id).await
    }

    /// Start timestamp for time-range data.
    async fn timestamp_start(&self) -> i64 {
        self.inner.timestamp_start
    }

    /// End timestamp for time-range data (null if not applicable).
    async fn timestamp_end(&self) -> Option<i64> {
        self.inner.timestamp_end
    }

    /// Epoch timestamp when the record was created.
    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    /// Epoch timestamp when the record was last updated.
    async fn last_updated(&self) -> i64 {
        self.inner.last_updated
    }

    /// Subscription duration in days.
    async fn subscription_duration_days(&self) -> i64 {
        self.inner.subscription_duration_days
    }

    /// Geographic region for the data.
    async fn geographic_region(&self) -> Option<&str> {
        self.inner.geographic_region.as_deref()
    }

    /// Data quality indicator ("high", "medium", "low").
    async fn data_quality(&self) -> Option<&str> {
        self.inner.data_quality.as_deref()
    }

    /// Sample size for the dataset.
    async fn sample_size(&self) -> Option<i64> {
        self.inner.sample_size
    }

    /// How the dataset was collected (e.g. "cli", "automated").
    async fn collection_method(&self) -> Option<&str> {
        self.inner.collection_method.as_deref()
    }

    /// Whether this data updates over time.
    async fn is_updating(&self) -> bool {
        self.inner.is_updating
    }

    /// Update frequency (e.g. "daily", "weekly", "monthly").
    async fn update_frequency(&self) -> Option<&str> {
        self.inner.update_frequency.as_deref()
    }

    /// Purchases for this MyData record (paginated).
    async fn purchases(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<MyDataPurchase>> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_mydata_purchases(&self.inner.mydata_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(MyDataPurchase::from_row).collect())
    }

    /// Subscriptions for this MyData record (paginated).
    async fn subscriptions(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<MyDataSubscription>> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_mydata_subscriptions(&self.inner.mydata_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(MyDataSubscription::from_row).collect())
    }

    /// Revenue entries for this MyData record (paginated).
    async fn revenue(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<MyDataRevenue>> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(30).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_mydata_revenue(&self.inner.mydata_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(MyDataRevenue::from_row).collect())
    }

    /// Access logs for this MyData record (paginated).
    async fn access_logs(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<MyDataAccessLog>> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(200) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_mydata_access_logs(&self.inner.mydata_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(MyDataAccessLog::from_row).collect())
    }

    /// Aggregated stats for this MyData record.
    async fn stats(&self, ctx: &Context<'_>) -> Option<MyDataStats> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_mydata_stats(&self.inner.mydata_id)
            .await
            .ok()??;
        Some(MyDataStats::from_row(row))
    }

    /// Daily revenue timeline for this MyData record.
    async fn revenue_timeline(&self, ctx: &Context<'_>) -> Option<Vec<MyDataDailyRevenue>> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let rows = reader
            .get_mydata_revenue_timeline(&self.inner.mydata_id)
            .await
            .ok()?;
        Some(rows.into_iter().map(MyDataDailyRevenue::from_row).collect())
    }

    /// Access analytics for this MyData record.
    async fn access_analytics(&self, ctx: &Context<'_>) -> Option<Vec<MyDataAccessAnalytics>> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let rows = reader
            .get_mydata_access_analytics(&self.inner.mydata_id)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(MyDataAccessAnalytics::from_row)
                .collect(),
        )
    }

    /// Query-marketplace sub-pools this listing is assigned to (from `MyDataAssignedToSubPoolEvent`).
    async fn sub_pools(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<MyDataSubPool>> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_mydata_sub_pools_for_listing(&self.inner.mydata_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(MyDataSubPool::from_row).collect())
    }
}

#[derive(Clone)]
pub(crate) struct MyDataPurchase {
    inner: MyDataPurchaseRow,
}

impl MyDataPurchase {
    pub(crate) fn from_row(inner: MyDataPurchaseRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataPurchase {
    /// MyData record ID this purchase is for.
    async fn mydata_id(&self) -> &str {
        &self.inner.mydata_id
    }

    /// Buyer address.
    async fn buyer(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.buyer)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the buyer.
    async fn buyer_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.buyer).await
    }

    /// Price paid (gross).
    async fn price(&self) -> i64 {
        self.inner.price
    }

    /// Platform fee slice deducted from gross payment.
    async fn platform_fee(&self) -> i64 {
        self.inner.platform_fee
    }

    /// Ecosystem fee slice deducted from gross payment.
    async fn ecosystem_fee(&self) -> i64 {
        self.inner.ecosystem_fee
    }

    /// Net amount credited to listing owner after fees.
    async fn creator_amount(&self) -> i64 {
        self.inner.creator_amount
    }

    /// Platform treasury recipient when a platform fee was routed on-chain.
    async fn platform_id(&self) -> Option<MySoAddress> {
        self.inner
            .platform_address
            .as_ref()
            .and_then(|a| MySoAddress::from_str(a).ok().map(|addr| addr.into()))
    }

    /// Purchase type ("one_time" or "subscription").
    async fn purchase_type(&self) -> &str {
        &self.inner.purchase_type
    }

    /// Epoch timestamp when the purchase was made.
    async fn purchase_time(&self) -> i64 {
        self.inner.purchase_time
    }

    /// Indexer timestamp when the purchase was recorded.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// Whether this purchase access has been revoked by the owner.
    async fn revoked(&self) -> bool {
        self.inner.revoked
    }

    /// Epoch ms when access was revoked, if applicable.
    async fn revoked_at(&self) -> Option<i64> {
        self.inner.revoked_at
    }

    /// Address that revoked access, if applicable.
    async fn revoked_by(&self) -> Option<MySoAddress> {
        self.inner
            .revoked_by
            .as_ref()
            .and_then(|a| MySoAddress::from_str(a).ok().map(|addr| addr.into()))
    }
}

#[derive(Clone)]
pub(crate) struct MyDataSubscription {
    inner: MyDataSubscriptionRow,
}

impl MyDataSubscription {
    pub(crate) fn from_row(inner: MyDataSubscriptionRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataSubscription {
    /// Subscriber address.
    async fn subscriber(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.subscriber)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the subscriber.
    async fn subscriber_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.subscriber).await
    }

    /// Subscription start timestamp (epoch).
    async fn subscription_start(&self) -> i64 {
        self.inner.subscription_start
    }

    /// Subscription end timestamp (epoch).
    async fn subscription_end(&self) -> i64 {
        self.inner.subscription_end
    }

    /// Price paid.
    async fn price(&self) -> i64 {
        self.inner.price
    }

    /// Indexer timestamp when the subscription was recorded.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// Whether this subscription access has been revoked by the owner.
    async fn revoked(&self) -> bool {
        self.inner.revoked
    }

    /// Epoch ms when access was revoked, if applicable.
    async fn revoked_at(&self) -> Option<i64> {
        self.inner.revoked_at
    }

    /// Address that revoked access, if applicable.
    async fn revoked_by(&self) -> Option<MySoAddress> {
        self.inner
            .revoked_by
            .as_ref()
            .and_then(|a| MySoAddress::from_str(a).ok().map(|addr| addr.into()))
    }
}

#[derive(Clone)]
pub(crate) struct MyDataRevenue {
    inner: MyDataRevenueRow,
}

impl MyDataRevenue {
    pub(crate) fn from_row(inner: MyDataRevenueRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataRevenue {
    /// Address that paid (from).
    async fn from_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.from_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Address that received (to).
    async fn to_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.to_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Amount (gross).
    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    /// Platform fee slice deducted from gross payment.
    async fn platform_fee(&self) -> i64 {
        self.inner.platform_fee
    }

    /// Ecosystem fee slice deducted from gross payment.
    async fn ecosystem_fee(&self) -> i64 {
        self.inner.ecosystem_fee
    }

    /// Net amount credited to listing owner after fees.
    async fn creator_amount(&self) -> i64 {
        self.inner.creator_amount
    }

    /// Platform treasury recipient when a platform fee was routed on-chain.
    async fn platform_id(&self) -> Option<MySoAddress> {
        self.inner
            .platform_address
            .as_ref()
            .and_then(|a| MySoAddress::from_str(a).ok().map(|addr| addr.into()))
    }

    /// Revenue type ("one_time", "subscription", "grant").
    async fn revenue_type(&self) -> &str {
        &self.inner.revenue_type
    }

    /// Epoch timestamp when the revenue was recorded.
    async fn revenue_time(&self) -> i64 {
        self.inner.revenue_time
    }

    /// Indexer timestamp.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct MyDataAccessLog {
    inner: MyDataAccessLogRow,
}

impl MyDataAccessLog {
    pub(crate) fn from_row(inner: MyDataAccessLogRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataAccessLog {
    /// User address that accessed.
    async fn user_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.user_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the user.
    async fn user_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.user_address).await
    }

    /// Access type ("one_time", "subscription", "grant", "preview", etc.).
    async fn access_type(&self) -> &str {
        &self.inner.access_type
    }

    /// Epoch timestamp when access occurred.
    async fn access_time(&self) -> i64 {
        self.inner.access_time
    }

    /// Indexer timestamp.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct MyDataStats {
    inner: MyDataStatsRow,
}

impl MyDataStats {
    pub(crate) fn from_row(inner: MyDataStatsRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataStats {
    /// Total revenue for this record.
    async fn total_revenue(&self) -> i64 {
        self.inner.total_revenue
    }

    /// Number of one-time purchases.
    async fn purchase_count(&self) -> i64 {
        self.inner.purchase_count
    }

    /// Number of active subscriptions.
    async fn subscription_count(&self) -> i64 {
        self.inner.subscription_count
    }

    /// Number of access events.
    async fn access_count(&self) -> i64 {
        self.inner.access_count
    }

    /// Epoch timestamp when the record was created.
    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    /// Epoch timestamp when the record was last updated.
    async fn last_updated(&self) -> i64 {
        self.inner.last_updated
    }
}

#[derive(Clone)]
pub(crate) struct MyDataDailyRevenue {
    inner: MyDataDailyRevenueRow,
}

impl MyDataDailyRevenue {
    pub(crate) fn from_row(inner: MyDataDailyRevenueRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataDailyRevenue {
    /// Day (YYYY-MM-DD).
    async fn day(&self) -> String {
        self.inner.day.format("%Y-%m-%d").to_string()
    }

    /// Revenue for that day.
    async fn daily_revenue(&self) -> i64 {
        self.inner.daily_revenue
    }

    /// Number of transactions that day.
    async fn daily_transactions(&self) -> i64 {
        self.inner.daily_transactions
    }
}

#[derive(Clone)]
pub(crate) struct MyDataAccessAnalytics {
    inner: MyDataAccessAnalyticsRow,
}

impl MyDataAccessAnalytics {
    pub(crate) fn from_row(inner: MyDataAccessAnalyticsRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataAccessAnalytics {
    /// Day (YYYY-MM-DD).
    async fn day(&self) -> String {
        self.inner.day.format("%Y-%m-%d").to_string()
    }

    /// Access type.
    async fn access_type(&self) -> &str {
        &self.inner.access_type
    }

    /// Number of unique users that day.
    async fn unique_users(&self) -> i64 {
        self.inner.unique_users
    }

    /// Total access events that day.
    async fn total_accesses(&self) -> i64 {
        self.inner.total_accesses
    }
}

#[derive(Clone)]
pub(crate) struct MyDataBroadPool {
    inner: MyDataBroadPoolRow,
}

impl MyDataBroadPool {
    pub(crate) fn from_row(inner: MyDataBroadPoolRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataBroadPool {
    async fn pool_id(&self) -> &str {
        &self.inner.pool_id
    }

    async fn name(&self) -> &str {
        &self.inner.name
    }

    async fn created_at_ms(&self) -> i64 {
        self.inner.created_at_ms
    }

    async fn event_id(&self) -> &str {
        &self.inner.event_id
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }
}

#[derive(Clone)]
pub(crate) struct MyDataSubPool {
    inner: MyDataSubPoolRow,
}

impl MyDataSubPool {
    pub(crate) fn from_row(inner: MyDataSubPoolRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataSubPool {
    async fn sub_pool_id(&self) -> &str {
        &self.inner.sub_pool_id
    }

    async fn broad_pool_id(&self) -> &str {
        &self.inner.broad_pool_id
    }

    async fn name(&self) -> &str {
        &self.inner.name
    }

    async fn created_at_ms(&self) -> i64 {
        self.inner.created_at_ms
    }

    async fn event_id(&self) -> &str {
        &self.inner.event_id
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }
}

#[derive(Clone)]
pub(crate) struct MyDataListingSubPool {
    inner: MyDataListingSubPoolRow,
}

impl MyDataListingSubPool {
    pub(crate) fn from_row(inner: MyDataListingSubPoolRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataListingSubPool {
    async fn listing_id(&self) -> &str {
        &self.inner.listing_id
    }

    async fn sub_pool_id(&self) -> &str {
        &self.inner.sub_pool_id
    }

    async fn assigned_at_ms(&self) -> i64 {
        self.inner.assigned_at_ms
    }

    async fn event_id(&self) -> &str {
        &self.inner.event_id
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }
}

#[derive(Clone)]
pub(crate) struct MyDataSnapshotAnchor {
    inner: MyDataSnapshotAnchorRow,
}

impl MyDataSnapshotAnchor {
    pub(crate) fn from_row(inner: MyDataSnapshotAnchorRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataSnapshotAnchor {
    async fn id(&self) -> i32 {
        self.inner.id
    }

    async fn snapshot_id(&self) -> &str {
        &self.inner.snapshot_id
    }

    async fn buyer(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.buyer_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn buyer_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.buyer_address).await
    }

    async fn price_paid(&self) -> i64 {
        self.inner.price_paid
    }

    async fn created_at_ms(&self) -> i64 {
        self.inner.created_at_ms
    }

    async fn event_id(&self) -> &str {
        &self.inner.event_id
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    async fn manifest_hash(&self) -> Option<String> {
        self.inner.manifest_hash.clone()
    }

    async fn payment_reference(&self) -> Option<String> {
        self.inner.payment_reference.clone()
    }
}

#[derive(Clone)]
pub(crate) struct MyDataDistributionRound {
    inner: MyDataDistributionRoundRow,
}

impl MyDataDistributionRound {
    pub(crate) fn from_row(inner: MyDataDistributionRoundRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataDistributionRound {
    async fn snapshot_id(&self) -> &str {
        &self.inner.snapshot_id
    }

    async fn total_amount(&self) -> i64 {
        self.inner.total_amount
    }

    async fn contributor_count(&self) -> i64 {
        self.inner.contributor_count
    }

    async fn merkle_root(&self) -> &str {
        &self.inner.merkle_root
    }

    async fn published_at_ms(&self) -> i64 {
        self.inner.published_at_ms
    }

    async fn event_id(&self) -> &str {
        &self.inner.event_id
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }
}

#[derive(Clone)]
pub(crate) struct MyDataMerkleRoot {
    inner: MyDataMerkleRootRow,
}

impl MyDataMerkleRoot {
    pub(crate) fn from_row(inner: MyDataMerkleRootRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataMerkleRoot {
    async fn snapshot_id(&self) -> &str {
        &self.inner.snapshot_id
    }

    async fn root_hash(&self) -> &str {
        &self.inner.root_hash
    }

    async fn published_at_ms(&self) -> i64 {
        self.inner.published_at_ms
    }

    async fn event_id(&self) -> &str {
        &self.inner.event_id
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }
}

#[derive(Clone)]
pub(crate) struct MyDataClaim {
    inner: MyDataClaimRow,
}

impl MyDataClaim {
    pub(crate) fn from_row(inner: MyDataClaimRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataClaim {
    async fn id(&self) -> i32 {
        self.inner.id
    }

    async fn snapshot_id(&self) -> &str {
        &self.inner.snapshot_id
    }

    async fn claimant(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.claimant)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn claimant_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.claimant).await
    }

    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    /// Gross claim amount before on-chain fee deduction.
    async fn gross_amount(&self) -> i64 {
        self.inner.gross_amount
    }

    /// Platform fee slice deducted at claim.
    async fn platform_fee(&self) -> i64 {
        self.inner.platform_fee
    }

    /// Ecosystem fee slice deducted at claim.
    async fn ecosystem_fee(&self) -> i64 {
        self.inner.ecosystem_fee
    }

    /// Net amount paid to claimant after fees.
    async fn net_amount(&self) -> i64 {
        self.inner.net_amount
    }

    /// Platform treasury recipient when a platform fee was routed on-chain.
    async fn platform_id(&self) -> Option<MySoAddress> {
        self.inner
            .platform_address
            .as_ref()
            .and_then(|a| MySoAddress::from_str(a).ok().map(|addr| addr.into()))
    }

    async fn claimed_at_ms(&self) -> i64 {
        self.inner.claimed_at_ms
    }

    async fn event_id(&self) -> &str {
        &self.inner.event_id
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }
}
