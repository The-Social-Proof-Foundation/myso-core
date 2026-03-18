// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;

use crate::api::scalars::myso_address::MySoAddress;

use myso_indexer_alt_social_reader::PostTransferRow;

#[derive(Clone)]
pub(crate) struct PostTransferSummary {
    inner: PostTransferRow,
}

impl PostTransferSummary {
    pub(crate) fn from_row(inner: PostTransferRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PostTransferSummary {
    async fn previous_owner(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.previous_owner)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn new_owner(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.new_owner)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn transferred_at(&self) -> i64 {
        self.inner.transferred_at
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}
