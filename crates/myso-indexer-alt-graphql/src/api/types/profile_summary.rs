// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;
use myso_indexer_alt_social_reader::ProfileSummaryRow;

use crate::api::scalars::myso_address::MySoAddress;

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
}
