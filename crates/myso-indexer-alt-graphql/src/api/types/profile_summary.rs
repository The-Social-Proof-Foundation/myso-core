// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::ProfileSummaryRow;

use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::profile::{SelectedBadge, SocialProofToken};

#[derive(Clone)]
pub(crate) struct ProfileSummary {
    inner: ProfileSummaryRow,
}

impl ProfileSummary {
    pub(crate) fn from_row(inner: ProfileSummaryRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ProfileSummary {
    /// Wallet address.
    async fn address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Username.
    async fn username(&self) -> Option<&str> {
        self.inner.username.as_deref()
    }

    /// Display name.
    async fn display_name(&self) -> Option<&str> {
        self.inner.display_name.as_deref()
    }

    /// Profile photo URL.
    async fn profile_photo(&self) -> Option<&str> {
        self.inner.profile_photo.as_deref()
    }

    /// Profile bio.
    async fn bio(&self) -> Option<&str> {
        self.inner.bio.as_deref()
    }

    /// Selected badge ID.
    async fn selected_badge_id(&self) -> Option<&str> {
        self.inner.selected_badge_id.as_deref()
    }

    /// Social proof token address.
    async fn social_proof_token_address(&self) -> Option<&str> {
        self.inner.social_proof_token_address.as_deref()
    }

    /// Reservation pool address.
    async fn reservation_pool_address(&self) -> Option<&str> {
        self.inner.reservation_pool_address.as_deref()
    }

    /// Number of followers. Present for both profile and wallet-only addresses.
    async fn followers_count(&self) -> Option<i32> {
        self.inner.followers_count
    }

    /// Number of accounts this address follows. Present for both profile and wallet-only addresses.
    async fn following_count(&self) -> Option<i32> {
        self.inner.following_count
    }

    /// Number of posts. Present for both profile and wallet-only addresses.
    async fn post_count(&self) -> Option<i32> {
        self.inner.post_count
    }

    /// Number of blocked accounts. Present for both profile and wallet-only addresses.
    async fn blocked_count(&self) -> Option<i32> {
        self.inner.blocked_count
    }

    /// When the parent query passed `viewer`, whether the viewer follows this address.
    async fn is_following(&self) -> Option<bool> {
        self.inner.is_following
    }

    /// When `viewer` was passed, whether this address follows the viewer ("follows you").
    async fn follows_viewer(&self) -> Option<bool> {
        self.inner.follows_viewer
    }

    /// When `viewer` was passed, whether the viewer has blocked this address.
    async fn blocked_by_viewer(&self) -> Option<bool> {
        self.inner.blocked_by_viewer
    }

    /// When `viewer` was passed, whether this address has blocked the viewer.
    async fn blocked_by_subject(&self) -> Option<bool> {
        self.inner.blocked_by_subject
    }

    /// Selected badge info (when present).
    async fn selected_badge(&self, ctx: &Context<'_>) -> Option<SelectedBadge> {
        if self.inner.selected_badge_id.is_none() {
            return None;
        }
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let enriched = reader
            .get_profile_summary_enriched(&self.inner.owner_address)
            .await
            .ok()??;
        enriched.selected_badge.as_ref().map(SelectedBadge::from)
    }

    /// Reservation percentage (when profile has SPT/reservation pool).
    async fn reservation_percentage(&self, ctx: &Context<'_>) -> Option<f64> {
        if self.inner.social_proof_token_address.is_none()
            && self.inner.reservation_pool_address.is_none()
        {
            return None;
        }
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let enriched = reader
            .get_profile_summary_enriched(&self.inner.owner_address)
            .await
            .ok()??;
        enriched
            .social_proof_token
            .as_ref()
            .map(|spt| spt.reservation_percentage)
    }

    /// Social proof token info (when present).
    async fn social_proof_token(&self, ctx: &Context<'_>) -> Option<SocialProofToken> {
        if self.inner.social_proof_token_address.is_none()
            && self.inner.reservation_pool_address.is_none()
        {
            return None;
        }
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let enriched = reader
            .get_profile_summary_enriched(&self.inner.owner_address)
            .await
            .ok()??;
        enriched
            .social_proof_token
            .as_ref()
            .map(SocialProofToken::from)
    }
}
