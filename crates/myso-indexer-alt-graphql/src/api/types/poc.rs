// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    PocAnalysisResultRow, PocBadgeRow, PocDisputeRow, PocRevenueRedirectionRow,
};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::profile_summary::ProfileSummary;

#[derive(Clone)]
pub(crate) struct PocBadge {
    inner: PocBadgeRow,
}

impl PocBadge {
    pub(crate) fn from_row(inner: PocBadgeRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocBadge {
    /// Badge ID.
    async fn badge_id(&self) -> &str {
        &self.inner.badge_id
    }

    /// Post ID this badge was issued for.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Media type (1=image, 2=video, 3=audio).
    async fn media_type(&self) -> i16 {
        self.inner.media_type
    }

    /// Address of the oracle that issued the badge.
    async fn issued_by(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.issued_by)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// When the badge was issued (epoch milliseconds).
    async fn issued_at(&self) -> i64 {
        self.inner.issued_at
    }

    /// Whether the badge has been revoked.
    async fn revoked(&self) -> bool {
        self.inner.revoked
    }
}

#[derive(Clone)]
pub(crate) struct PocRevenueRedirection {
    inner: PocRevenueRedirectionRow,
}

impl PocRevenueRedirection {
    pub(crate) fn from_row(inner: PocRevenueRedirectionRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocRevenueRedirection {
    /// Redirection ID.
    async fn redirection_id(&self) -> &str {
        &self.inner.redirection_id
    }

    /// Post ID accused of being derivative (receiving redirected revenue).
    async fn accused_post_id(&self) -> &str {
        &self.inner.accused_post_id
    }

    /// Original post ID that receives the redirected revenue.
    async fn original_post_id(&self) -> &str {
        &self.inner.original_post_id
    }

    /// Percentage of revenue redirected to the original creator.
    async fn redirect_percentage(&self) -> i64 {
        self.inner.redirect_percentage
    }

    /// Similarity score from the analysis (0-100).
    async fn similarity_score(&self) -> i64 {
        self.inner.similarity_score
    }

    /// When the redirection was created (epoch milliseconds).
    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }
}

#[derive(Clone)]
pub(crate) struct PocAnalysisResult {
    inner: PocAnalysisResultRow,
}

impl PocAnalysisResult {
    pub(crate) fn from_row(inner: PocAnalysisResultRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocAnalysisResult {
    /// Post ID that was analyzed.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Whether similarity to existing content was detected.
    async fn similarity_detected(&self) -> bool {
        self.inner.similarity_detected
    }

    /// Highest similarity score from the analysis (0-100).
    async fn highest_similarity_score(&self) -> i64 {
        self.inner.highest_similarity_score
    }

    /// Media type analyzed (1=image, 2=video, 3=audio).
    async fn media_type(&self) -> i16 {
        self.inner.media_type
    }

    /// Address of the oracle that performed the analysis.
    async fn oracle_address(&self) -> &str {
        &self.inner.oracle_address
    }

    /// Address of the original creator if similarity was detected.
    async fn original_creator(&self) -> Option<&str> {
        self.inner.original_creator.as_deref()
    }

    /// When the analysis was performed (epoch milliseconds).
    async fn analysis_timestamp(&self) -> i64 {
        self.inner.analysis_timestamp
    }
}

#[derive(Clone)]
pub(crate) struct PocDispute {
    inner: PocDisputeRow,
}

impl PocDispute {
    pub(crate) fn from_row(inner: PocDisputeRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocDispute {
    /// Dispute ID.
    async fn dispute_id(&self) -> &str {
        &self.inner.dispute_id
    }

    /// Post ID being disputed.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Address of the disputer.
    async fn disputer(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.disputer)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the disputer.
    async fn disputer_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.disputer).await
    }

    /// Dispute type (1=challenge badge, 2=challenge redirection).
    async fn dispute_type(&self) -> i16 {
        self.inner.dispute_type
    }

    /// Evidence submitted by the disputer.
    async fn evidence(&self) -> &str {
        &self.inner.evidence
    }

    /// Status (1=voting, 2=resolved upheld, 3=resolved overturned).
    async fn status(&self) -> i16 {
        self.inner.status
    }

    /// Stake amount required to submit the dispute.
    async fn stake_amount(&self) -> i64 {
        self.inner.stake_amount
    }

    /// When the dispute was submitted (epoch milliseconds).
    async fn submitted_at(&self) -> i64 {
        self.inner.submitted_at
    }

    /// When the dispute was resolved (epoch milliseconds), if resolved.
    async fn resolved_at(&self) -> Option<i64> {
        self.inner.resolved_at
    }
}
