// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;
use myso_indexer_alt_social_reader::subscription::{
    ProfileSubscriptionRow, ProfileSubscriptionServiceRow,
};

#[derive(Clone)]
pub(crate) struct ProfileSubscriptionService {
    inner: ProfileSubscriptionServiceRow,
}

impl ProfileSubscriptionService {
    pub(crate) fn from_row(inner: ProfileSubscriptionServiceRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ProfileSubscriptionService {
    async fn service_id(&self) -> &str {
        &self.inner.service_id
    }

    async fn profile_owner(&self) -> &str {
        &self.inner.profile_owner
    }

    async fn profile_id(&self) -> &str {
        &self.inner.profile_id
    }

    async fn monthly_fee(&self) -> i64 {
        self.inner.monthly_fee
    }

    async fn active(&self) -> bool {
        self.inner.active
    }

    async fn subscriber_count(&self) -> i64 {
        self.inner.subscriber_count
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    async fn updated_at(&self) -> Option<i64> {
        self.inner.updated_at
    }
}

#[derive(Clone)]
pub(crate) struct ProfileSubscription {
    inner: ProfileSubscriptionRow,
}

impl ProfileSubscription {
    pub(crate) fn from_row(inner: ProfileSubscriptionRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ProfileSubscription {
    async fn subscription_id(&self) -> &str {
        &self.inner.subscription_id
    }

    async fn service_id(&self) -> &str {
        &self.inner.service_id
    }

    async fn subscriber(&self) -> &str {
        &self.inner.subscriber
    }

    async fn profile_owner(&self) -> &str {
        &self.inner.profile_owner
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    async fn expires_at(&self) -> i64 {
        self.inner.expires_at
    }

    async fn auto_renew(&self) -> bool {
        self.inner.auto_renew
    }

    async fn renewal_balance(&self) -> i64 {
        self.inner.renewal_balance
    }

    async fn renewal_count(&self) -> i64 {
        self.inner.renewal_count
    }

    async fn cancelled_at(&self) -> Option<i64> {
        self.inner.cancelled_at
    }

    async fn monthly_fee(&self) -> i64 {
        self.inner.monthly_fee
    }

    async fn active(&self) -> bool {
        self.inner.cancelled_at.is_none()
            && self.inner.expires_at > chrono::Utc::now().timestamp_millis()
    }
}

#[derive(Clone)]
pub(crate) struct SubscriptionAccess {
    has_access: bool,
    expires_at: Option<i64>,
}

impl SubscriptionAccess {
    pub(crate) fn new(has_access: bool, expires_at: Option<i64>) -> Self {
        Self {
            has_access,
            expires_at,
        }
    }
}

#[Object]
impl SubscriptionAccess {
    async fn has_access(&self) -> bool {
        self.has_access
    }

    async fn expires_at(&self) -> Option<i64> {
        self.expires_at
    }
}
