// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;
use myso_indexer_alt_social_reader::PlatformRow as DbPlatform;

use crate::api::scalars::id::Id;

#[derive(Clone)]
pub(crate) struct Platform {
    inner: DbPlatform,
}

impl Platform {
    pub(crate) fn from_db(inner: DbPlatform) -> Self {
        Self { inner }
    }
}

#[Object]
impl Platform {
    /// The platform's globally unique identifier.
    pub async fn id(&self) -> Id {
        Id::Platform(self.inner.platform_id.clone())
    }

    /// The platform ID.
    async fn platform_id(&self) -> &str {
        &self.inner.platform_id
    }

    /// The platform name.
    async fn name(&self) -> &str {
        &self.inner.name
    }

    /// The platform tagline.
    async fn tagline(&self) -> &str {
        &self.inner.tagline
    }

    /// The platform description.
    async fn description(&self) -> Option<&str> {
        self.inner.description.as_deref()
    }

    /// URL to the platform logo.
    async fn logo(&self) -> Option<&str> {
        self.inner.logo.as_deref()
    }

    /// The developer's wallet address.
    async fn developer_address(&self) -> &str {
        &self.inner.developer_address
    }

    /// Whether the platform is approved.
    async fn is_approved(&self) -> bool {
        self.inner.is_approved
    }

    /// Primary category.
    async fn primary_category(&self) -> &str {
        &self.inner.primary_category
    }

    /// Secondary category.
    async fn secondary_category(&self) -> Option<&str> {
        self.inner.secondary_category.as_deref()
    }
}
