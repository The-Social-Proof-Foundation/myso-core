// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::profile_summary::ProfileSummary;

#[derive(Clone)]
pub(crate) struct BlockedProfileSummary {
    pub blocked_address: String,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    pub first_blocked_at: chrono::NaiveDateTime,
    pub last_blocked_at: chrono::NaiveDateTime,
}

impl BlockedProfileSummary {
    pub(crate) fn from_row(row: myso_indexer_alt_social_reader::BlockedProfileRow) -> Self {
        Self {
            blocked_address: row.blocked_address,
            blocked_username: row.blocked_username,
            blocked_display_name: row.blocked_display_name,
            blocked_profile_photo: row.blocked_profile_photo,
            first_blocked_at: row.first_blocked_at,
            last_blocked_at: row.last_blocked_at,
        }
    }
}

#[Object]
impl BlockedProfileSummary {
    async fn address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.blocked_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn username(&self) -> &str {
        &self.blocked_username
    }

    async fn display_name(&self) -> Option<&str> {
        self.blocked_display_name.as_deref()
    }

    async fn profile_photo(&self) -> Option<&str> {
        self.blocked_profile_photo.as_deref()
    }

    async fn first_blocked_at(&self) -> i64 {
        self.first_blocked_at.and_utc().timestamp_millis()
    }

    async fn last_blocked_at(&self) -> i64 {
        self.last_blocked_at.and_utc().timestamp_millis()
    }
}

#[derive(Clone)]
pub(crate) struct BlockedPlatformSummary {
    pub platform_id: String,
    pub platform_name: String,
    pub blocked_by: String,
    pub created_at: chrono::NaiveDateTime,
}

impl BlockedPlatformSummary {
    pub(crate) fn from_row(row: myso_indexer_alt_social_reader::BlockedPlatformRow) -> Self {
        Self {
            platform_id: row.platform_id,
            platform_name: row.platform_name,
            blocked_by: row.blocked_by,
            created_at: row.created_at,
        }
    }
}

#[Object]
impl BlockedPlatformSummary {
    async fn platform_id(&self) -> &str {
        &self.platform_id
    }

    /// Profile of the user who blocked the platform.
    async fn blocked_by_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.blocked_by).await
    }

    async fn platform_name(&self) -> &str {
        &self.platform_name
    }

    async fn blocked_by(&self) -> &str {
        &self.blocked_by
    }

    async fn created_at(&self) -> i64 {
        self.created_at.and_utc().timestamp_millis()
    }
}

#[derive(Clone)]
pub(crate) struct PlatformBlockedProfileSummary {
    pub wallet_address: String,
    pub blocked_by: String,
    pub created_at: chrono::NaiveDateTime,
}

impl PlatformBlockedProfileSummary {
    pub(crate) fn from_row(
        row: myso_indexer_alt_social_reader::PlatformBlockedProfileRow,
    ) -> Self {
        Self {
            wallet_address: row.wallet_address,
            blocked_by: row.blocked_by,
            created_at: row.created_at,
        }
    }
}

#[Object]
impl PlatformBlockedProfileSummary {
    async fn wallet_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.wallet_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the blocked user.
    async fn profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.wallet_address).await
    }

    async fn blocked_by(&self) -> &str {
        &self.blocked_by
    }

    async fn created_at(&self) -> i64 {
        self.created_at.and_utc().timestamp_millis()
    }
}
