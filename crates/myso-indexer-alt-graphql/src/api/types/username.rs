// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;
use myso_indexer_alt_social_reader::UsernameRegistryEntry;

/// Mirror of an on-chain username registry entry.
pub struct UsernameRegistry {
    inner: UsernameRegistryEntry,
}

impl UsernameRegistry {
    pub fn new(inner: UsernameRegistryEntry) -> Self {
        Self { inner }
    }
}

#[Object]
impl UsernameRegistry {
    async fn username(&self) -> &str {
        &self.inner.username
    }

    async fn profile_id(&self) -> &str {
        &self.inner.profile_id
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

/// Whether a username is available for registration.
pub struct UsernameAvailability {
    username: String,
    available: bool,
}

impl UsernameAvailability {
    pub fn new(username: String, available: bool) -> Self {
        Self { username, available }
    }
}

#[Object]
impl UsernameAvailability {
    async fn username(&self) -> &str {
        &self.username
    }

    async fn available(&self) -> bool {
        self.available
    }
}
