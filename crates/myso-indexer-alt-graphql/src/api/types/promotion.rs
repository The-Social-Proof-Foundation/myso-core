// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    PromotedPostRow, PromotionHourlyRow, PromotionStatsRow, PromotionTimeSeriesRow,
    PromotionViewRow,
};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::profile_summary::ProfileSummary;

#[derive(Clone)]
pub(crate) struct Promotion {
    inner: PromotedPostRow,
    views: i64,
}

impl Promotion {
    pub(crate) fn from_row(inner: PromotedPostRow, views: i64) -> Self {
        Self { inner, views }
    }
}

#[Object]
impl Promotion {
    /// The promotion ID.
    async fn promotion_id(&self) -> &str {
        &self.inner.promotion_id
    }

    /// The post ID this promotion is for.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Owner address.
    async fn owner(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile ID of the promotion owner.
    async fn profile_id(&self) -> &str {
        &self.inner.profile_id
    }

    /// Payment per view (in MIST).
    async fn payment_per_view(&self) -> i64 {
        self.inner.payment_per_view
    }

    /// Total budget (in MIST).
    async fn budget(&self) -> i64 {
        self.inner.total_budget
    }

    /// Remaining budget (in MIST).
    async fn remaining_budget(&self) -> i64 {
        self.inner.remaining_budget
    }

    /// Promotion status: "active" or "inactive".
    async fn status(&self) -> &str {
        if self.inner.active {
            "active"
        } else {
            "inactive"
        }
    }

    /// Number of views.
    async fn views(&self) -> i64 {
        self.views
    }

    /// Creation timestamp (epoch ms).
    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    /// Individual view records (paginated).
    async fn views_detail(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PromotionView>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_promotion_views(&self.inner.promotion_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PromotionView::from_row).collect())
    }

    /// Aggregated stats for this promotion.
    async fn stats(&self, ctx: &Context<'_>) -> Option<PromotionStats> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_promotion_stats(&self.inner.promotion_id)
            .await
            .ok()??;
        Some(PromotionStats::from_row(row))
    }

    /// Daily time series (last 30 days).
    async fn time_series(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
    ) -> Option<Vec<PromotionTimeSeries>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(30).min(90) as i64;
        let rows = reader
            .get_promotion_time_series(&self.inner.promotion_id, limit)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PromotionTimeSeries::from_row)
                .collect(),
        )
    }

    /// Hourly aggregates (last 7 days).
    async fn hourly(&self, ctx: &Context<'_>, limit: Option<u64>) -> Option<Vec<PromotionHourly>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(24).min(168) as i64;
        let rows = reader
            .get_promotion_hourly(&self.inner.promotion_id, limit)
            .await
            .ok()?;
        Some(rows.into_iter().map(PromotionHourly::from_row).collect())
    }
}

#[derive(Clone)]
pub(crate) struct PromotionView {
    inner: PromotionViewRow,
}

impl PromotionView {
    pub(crate) fn from_row(inner: PromotionViewRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PromotionView {
    /// Post ID.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Promotion ID.
    async fn promotion_id(&self) -> &str {
        &self.inner.promotion_id
    }

    /// Viewer address.
    async fn viewer(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.viewer)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the viewer.
    async fn viewer_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.viewer).await
    }

    /// Gross payment amount for this view (advertiser budget debit, MIST).
    async fn payment_amount(&self) -> i64 {
        self.inner.payment_amount
    }

    /// Platform fee taken from the gross (MIST).
    async fn platform_fee(&self) -> i64 {
        self.inner.platform_fee
    }

    /// Ecosystem fee taken from the gross (MIST).
    async fn ecosystem_fee(&self) -> i64 {
        self.inner.ecosystem_fee
    }

    /// Net MYSO transferred to the viewer (MIST).
    async fn recipient_amount(&self) -> i64 {
        self.inner.recipient_amount
    }

    /// View duration (ms).
    async fn view_duration(&self) -> i64 {
        self.inner.view_duration
    }

    /// Platform where the view occurred.
    async fn platform_id(&self) -> &str {
        &self.inner.platform_id
    }

    /// View timestamp (epoch ms).
    async fn timestamp(&self) -> i64 {
        self.inner.timestamp
    }
}

#[derive(Clone)]
pub(crate) struct PromotionStats {
    inner: PromotionStatsRow,
}

impl PromotionStats {
    pub(crate) fn from_row(inner: PromotionStatsRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PromotionStats {
    /// Total view count.
    async fn total_views(&self) -> i64 {
        self.inner.total_views
    }

    /// Total amount spent (MIST).
    /// Advertiser gross spend across confirmed views (sum of `payment_amount`, MIST).
    async fn total_spent(&self) -> i64 {
        self.inner.total_spent
    }

    /// Remaining budget (MIST).
    async fn remaining_budget(&self) -> i64 {
        self.inner.remaining_budget
    }
}

#[derive(Clone)]
pub(crate) struct PromotionTimeSeries {
    inner: PromotionTimeSeriesRow,
}

impl PromotionTimeSeries {
    pub(crate) fn from_row(inner: PromotionTimeSeriesRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PromotionTimeSeries {
    /// Day (YYYY-MM-DD).
    async fn day(&self) -> String {
        self.inner.day.format("%Y-%m-%d").to_string()
    }

    /// View count for this day.
    async fn views(&self) -> i64 {
        self.inner.views
    }

    /// Amount spent this day (MIST).
    async fn spent(&self) -> i64 {
        self.inner.spent
    }
}

#[derive(Clone)]
pub(crate) struct PromotionHourly {
    inner: PromotionHourlyRow,
}

impl PromotionHourly {
    pub(crate) fn from_row(inner: PromotionHourlyRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PromotionHourly {
    /// Hour of day (0-23).
    async fn hour(&self) -> i32 {
        self.inner.hour
    }

    /// View count for this hour.
    async fn views(&self) -> i64 {
        self.inner.views
    }

    /// Amount spent this hour (MIST).
    async fn spent(&self) -> i64 {
        self.inner.spent
    }
}
