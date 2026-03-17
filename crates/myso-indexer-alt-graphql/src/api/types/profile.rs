// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;
use myso_indexer_alt_social_reader::pg_reader::Profile as DbProfile;

use crate::api::scalars::id::Id;
use crate::api::scalars::myso_address::MySoAddress;

#[derive(Clone)]
pub(crate) struct Profile {
    inner: DbProfile,
}

impl Profile {
    pub(crate) fn from_db(inner: DbProfile) -> Self {
        Self { inner }
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
    async fn username(&self) -> &str {
        &self.inner.username
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
}
