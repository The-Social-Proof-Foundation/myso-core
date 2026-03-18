// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;
use myso_indexer_alt_social_reader::ProfileBadgeRow;

#[derive(Clone)]
pub(crate) struct ProfileBadge {
    inner: ProfileBadgeRow,
}

impl ProfileBadge {
    pub(crate) fn from_row(inner: ProfileBadgeRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ProfileBadge {
    /// Badge ID.
    async fn badge_id(&self) -> &str {
        &self.inner.badge_id
    }

    /// Badge name.
    async fn badge_name(&self) -> &str {
        &self.inner.badge_name
    }

    /// Badge description.
    async fn badge_description(&self) -> Option<&str> {
        self.inner.badge_description.as_deref()
    }

    /// Badge media URL.
    async fn badge_media_url(&self) -> Option<&str> {
        self.inner.badge_media_url.as_deref()
    }

    /// Badge icon URL.
    async fn badge_icon_url(&self) -> Option<&str> {
        self.inner.badge_icon_url.as_deref()
    }

    /// Platform ID.
    async fn platform_id(&self) -> &str {
        &self.inner.platform_id
    }

    /// Address that assigned the badge.
    async fn assigned_by(&self) -> &str {
        &self.inner.assigned_by
    }

    /// When the badge was assigned (epoch ms).
    async fn assigned_at(&self) -> i64 {
        self.inner.assigned_at
    }

    /// Badge type.
    async fn badge_type(&self) -> i16 {
        self.inner.badge_type
    }
}
