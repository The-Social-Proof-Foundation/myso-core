// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Enum;
use async_graphql::Object;
use myso_indexer_alt_social_reader::UsernameAvailabilityDetail;
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

/// Active on-chain reservation for a username (PoC beneficiary provision or marketplace listing).
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UsernameLockReason {
    Beneficiary,
    Marketplace,
}

fn lock_reason_from_str(reason: &str) -> Option<UsernameLockReason> {
    match reason {
        "beneficiary" => Some(UsernameLockReason::Beneficiary),
        "marketplace" => Some(UsernameLockReason::Marketplace),
        _ => None,
    }
}

/// Why a username is unavailable for registration.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UsernameUnavailableReason {
    RegistryClaimed,
    BeneficiaryProvisioned,
    MarketplaceListed,
}

/// Whether a username is available for registration.
pub struct UsernameAvailability {
    inner: UsernameAvailabilityDetail,
}

impl UsernameAvailability {
    pub fn new(inner: UsernameAvailabilityDetail) -> Self {
        Self { inner }
    }
}

#[Object]
impl UsernameAvailability {
    async fn username(&self) -> &str {
        &self.inner.username
    }

    async fn available(&self) -> bool {
        self.inner.available
    }

    async fn registry_claimed(&self) -> bool {
        self.inner.registry_claimed
    }

    async fn beneficiary_provisioned(&self) -> bool {
        self.inner.beneficiary_provisioned
    }

    async fn marketplace_listed(&self) -> bool {
        self.inner.marketplace_listed
    }

    async fn registry_profile_id(&self) -> Option<&str> {
        self.inner.registry_profile_id.as_deref()
    }

    async fn beneficiary_id(&self) -> Option<&str> {
        self.inner.beneficiary_id.as_deref()
    }

    async fn listing_seller_profile_id(&self) -> Option<&str> {
        self.inner.listing_seller_profile_id.as_deref()
    }

    async fn lock_reasons(&self) -> Vec<UsernameLockReason> {
        self.inner
            .lock_reasons
            .iter()
            .filter_map(|reason| lock_reason_from_str(reason))
            .collect()
    }

    async fn unavailable_reasons(&self) -> Vec<UsernameUnavailableReason> {
        let mut reasons = Vec::new();
        if self.inner.registry_claimed {
            reasons.push(UsernameUnavailableReason::RegistryClaimed);
        }
        if self.inner.beneficiary_provisioned {
            reasons.push(UsernameUnavailableReason::BeneficiaryProvisioned);
        }
        if self.inner.marketplace_listed {
            reasons.push(UsernameUnavailableReason::MarketplaceListed);
        }
        reasons
    }
}
