// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;
use myso_indexer_alt_social_reader::{InsurancePolicyRow, InsuranceVaultRow};

use crate::api::scalars::myso_address::MySoAddress;

#[derive(Clone)]
pub(crate) struct InsurancePolicy {
    inner: InsurancePolicyRow,
}

impl InsurancePolicy {
    pub(crate) fn from_row(inner: InsurancePolicyRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsurancePolicy {
    /// Unique policy identifier.
    async fn policy_id(&self) -> &str {
        &self.inner.policy_id
    }

    /// Market ID this policy covers.
    async fn market_id(&self) -> &str {
        &self.inner.market_id
    }

    /// Insured address.
    async fn insured(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.insured)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Covered amount.
    async fn covered_amount(&self) -> i64 {
        self.inner.covered_amount
    }

    /// Premium paid.
    async fn premium_paid(&self) -> i64 {
        self.inner.premium_paid
    }

    /// Policy status (1=ACTIVE, 2=CANCELLED, 3=CLAIMED, 4=EXPIRED).
    async fn status(&self) -> i16 {
        self.inner.status
    }
}

#[derive(Clone)]
pub(crate) struct InsuranceVault {
    inner: InsuranceVaultRow,
}

impl InsuranceVault {
    pub(crate) fn from_row(inner: InsuranceVaultRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceVault {
    /// Unique vault identifier.
    async fn vault_id(&self) -> &str {
        &self.inner.vault_id
    }

    /// Underwriter address.
    async fn underwriter(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.underwriter)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Capital balance.
    async fn capital_balance(&self) -> i64 {
        self.inner.capital_balance
    }
}
