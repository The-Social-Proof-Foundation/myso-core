// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;
use chrono::DateTime;
use myso_indexer_alt_social_reader::{
    ProfileByAddressResponse, ReservationStatus, SelectedBadgeInfo, SocialProofTokenInfo,
};
use myso_indexer_alt_social_schema::models::Profile as SchemaProfile;

use crate::api::scalars::id::Id;
use crate::api::scalars::myso_address::MySoAddress;

fn to_iso8601_utc(dt: chrono::NaiveDateTime) -> String {
    DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

#[derive(Clone)]
pub(crate) struct Profile {
    inner: ProfileByAddressResponse,
}

impl Profile {
    pub(crate) fn from_response(inner: ProfileByAddressResponse) -> Option<Self> {
        if inner.username.is_some() {
            Some(Self { inner })
        } else {
            None
        }
    }

    pub(crate) fn from_db(inner: SchemaProfile) -> Self {
        let response = ProfileByAddressResponse {
            id: Some(inner.id),
            owner_address: inner.owner_address,
            profile_id: inner.profile_id,
            username: Some(inner.username),
            display_name: inner.display_name,
            bio: inner.bio,
            profile_photo: inner.profile_photo,
            cover_photo: inner.cover_photo,
            website: inner.website,
            created_at: Some(to_iso8601_utc(inner.created_at)),
            updated_at: Some(to_iso8601_utc(inner.updated_at)),
            followers_count: inner.followers_count,
            following_count: inner.following_count,
            post_count: inner.post_count,
            min_offer_amount: inner.min_offer_amount,
            birthdate: inner.birthdate,
            current_location: inner.current_location,
            raised_location: inner.raised_location,
            phone: inner.phone,
            email: inner.email,
            gender: inner.gender,
            political_view: inner.political_view,
            religion: inner.religion,
            education: inner.education,
            primary_language: inner.primary_language,
            relationship_status: inner.relationship_status,
            x_username: inner.x_username,
            mastodon_username: None,
            facebook_username: inner.facebook_username,
            reddit_username: inner.reddit_username,
            github_username: inner.github_username,
            block_list_address: None,
            social_proof_token_address: inner.social_proof_token_address,
            reservation_pool_address: inner.reservation_pool_address,
            social_proof_token: None,
            selected_badge: None,
            selected_badge_id: inner.selected_badge_id,
            selected_ecosystem_badge_id: inner.selected_ecosystem_badge_id,
        };
        Self { inner: response }
    }

    fn owner_as_address(&self) -> myso_types::base_types::MySoAddress {
        MySoAddress::from_str(&self.inner.owner_address)
            .map(Into::into)
            .unwrap_or(myso_types::base_types::MySoAddress::ZERO)
    }
}

#[Object]
impl Profile {
    /// The profile's globally unique identifier.
    pub async fn id(&self) -> Id {
        Id::Profile(self.owner_as_address())
    }

    /// The wallet address that owns this profile.
    async fn address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// The profile's username.
    async fn username(&self) -> String {
        self.inner.username.clone().unwrap_or_default()
    }

    /// The profile's display name.
    async fn display_name(&self) -> Option<&str> {
        self.inner.display_name.as_deref()
    }

    /// The profile's bio.
    async fn bio(&self) -> Option<&str> {
        self.inner.bio.as_deref()
    }

    /// URL to the profile photo.
    async fn profile_photo(&self) -> Option<&str> {
        self.inner.profile_photo.as_deref()
    }

    /// URL to the cover photo.
    async fn cover_photo(&self) -> Option<&str> {
        self.inner.cover_photo.as_deref()
    }

    /// The profile's website.
    async fn website(&self) -> Option<&str> {
        self.inner.website.as_deref()
    }

    /// When the profile was created (ISO 8601).
    async fn created_at(&self) -> Option<&str> {
        self.inner.created_at.as_deref()
    }

    /// When the profile was last updated (ISO 8601).
    async fn updated_at(&self) -> Option<&str> {
        self.inner.updated_at.as_deref()
    }

    /// The profile ID (object address).
    async fn profile_id(&self) -> Option<&str> {
        self.inner.profile_id.as_deref()
    }

    /// Number of followers.
    async fn followers_count(&self) -> i32 {
        self.inner.followers_count
    }

    /// Number of accounts this profile follows.
    async fn following_count(&self) -> i32 {
        self.inner.following_count
    }

    /// Number of posts.
    async fn post_count(&self) -> i32 {
        self.inner.post_count
    }

    /// Minimum offer amount for profile sales.
    async fn min_offer_amount(&self) -> Option<i64> {
        self.inner.min_offer_amount
    }

    /// Birthdate.
    async fn birthdate(&self) -> Option<&str> {
        self.inner.birthdate.as_deref()
    }

    /// Current location.
    async fn current_location(&self) -> Option<&str> {
        self.inner.current_location.as_deref()
    }

    /// Raised location.
    async fn raised_location(&self) -> Option<&str> {
        self.inner.raised_location.as_deref()
    }

    /// Phone (encrypted).
    async fn phone(&self) -> Option<&str> {
        self.inner.phone.as_deref()
    }

    /// Email (encrypted).
    async fn email(&self) -> Option<&str> {
        self.inner.email.as_deref()
    }

    /// Gender.
    async fn gender(&self) -> Option<&str> {
        self.inner.gender.as_deref()
    }

    /// Political view.
    async fn political_view(&self) -> Option<&str> {
        self.inner.political_view.as_deref()
    }

    /// Religion.
    async fn religion(&self) -> Option<&str> {
        self.inner.religion.as_deref()
    }

    /// Education.
    async fn education(&self) -> Option<&str> {
        self.inner.education.as_deref()
    }

    /// Primary language.
    async fn primary_language(&self) -> Option<&str> {
        self.inner.primary_language.as_deref()
    }

    /// Relationship status.
    async fn relationship_status(&self) -> Option<&str> {
        self.inner.relationship_status.as_deref()
    }

    /// X (Twitter) username.
    async fn x_username(&self) -> Option<&str> {
        self.inner.x_username.as_deref()
    }

    /// Mastodon username.
    async fn mastodon_username(&self) -> Option<&str> {
        self.inner.mastodon_username.as_deref()
    }

    /// Facebook username.
    async fn facebook_username(&self) -> Option<&str> {
        self.inner.facebook_username.as_deref()
    }

    /// Reddit username.
    async fn reddit_username(&self) -> Option<&str> {
        self.inner.reddit_username.as_deref()
    }

    /// GitHub username.
    async fn github_username(&self) -> Option<&str> {
        self.inner.github_username.as_deref()
    }

    /// Block list address.
    async fn block_list_address(&self) -> Option<&str> {
        self.inner.block_list_address.as_deref()
    }

    /// Social proof token address.
    async fn social_proof_token_address(&self) -> Option<&str> {
        self.inner.social_proof_token_address.as_deref()
    }

    /// Reservation pool address.
    async fn reservation_pool_address(&self) -> Option<&str> {
        self.inner.reservation_pool_address.as_deref()
    }

    /// Social proof token info.
    async fn social_proof_token(&self) -> Option<SocialProofToken> {
        self.inner
            .social_proof_token
            .as_ref()
            .map(SocialProofToken::from)
    }

    /// Selected badge info.
    async fn selected_badge(&self) -> Option<SelectedBadge> {
        self.inner.selected_badge.as_ref().map(SelectedBadge::from)
    }

    /// Selected badge ID.
    async fn selected_badge_id(&self) -> Option<&str> {
        self.inner.selected_badge_id.as_deref()
    }

    /// Selected ecosystem badge ID.
    async fn selected_ecosystem_badge_id(&self) -> Option<&str> {
        self.inner.selected_ecosystem_badge_id.as_deref()
    }
}

#[derive(Clone)]
pub(crate) struct SocialProofToken {
    inner: SocialProofTokenInfo,
}

impl From<&SocialProofTokenInfo> for SocialProofToken {
    fn from(inner: &SocialProofTokenInfo) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[Object]
impl SocialProofToken {
    async fn pool_id(&self) -> Option<&str> {
        self.inner.pool_id.as_deref()
    }

    async fn token_address(&self) -> Option<&str> {
        self.inner.token_address.as_deref()
    }

    async fn is_active(&self) -> bool {
        self.inner.is_active
    }

    async fn reservation_pool_id(&self) -> Option<&str> {
        self.inner.reservation_pool_id.as_deref()
    }

    async fn reservation_percentage(&self) -> f64 {
        self.inner.reservation_percentage
    }

    async fn reservation_status(&self) -> String {
        match &self.inner.reservation_status {
            ReservationStatus::Active => "active".to_string(),
            ReservationStatus::ThresholdMet => "threshold_met".to_string(),
            ReservationStatus::Inactive => "inactive".to_string(),
            ReservationStatus::None => "none".to_string(),
        }
    }

    async fn total_reserved(&self) -> i64 {
        self.inner.total_reserved
    }

    async fn required_threshold(&self) -> i64 {
        self.inner.required_threshold
    }
}

#[derive(Clone)]
pub(crate) struct SelectedBadge {
    inner: SelectedBadgeInfo,
}

impl From<&SelectedBadgeInfo> for SelectedBadge {
    fn from(inner: &SelectedBadgeInfo) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[Object]
impl SelectedBadge {
    async fn badge_id(&self) -> &str {
        &self.inner.badge_id
    }

    async fn badge_name(&self) -> &str {
        &self.inner.badge_name
    }

    async fn badge_icon_url(&self) -> Option<&str> {
        self.inner.badge_icon_url.as_deref()
    }

    async fn badge_media_url(&self) -> Option<&str> {
        self.inner.badge_media_url.as_deref()
    }

    async fn platform_id(&self) -> &str {
        &self.inner.platform_id
    }

    async fn badge_type(&self) -> i16 {
        self.inner.badge_type
    }
}
