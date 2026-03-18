// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;

use crate::api::scalars::myso_address::MySoAddress;

use myso_indexer_alt_social_reader::RepostRow;

#[derive(Clone)]
pub(crate) struct RepostSummary {
    inner: RepostRow,
}

impl RepostSummary {
    pub(crate) fn from_row(inner: RepostRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl RepostSummary {
    async fn repost_id(&self) -> &str {
        &self.inner.repost_id
    }

    async fn original_post_id(&self) -> &str {
        &self.inner.original_post_id
    }

    async fn owner(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn profile_id(&self) -> &str {
        &self.inner.profile_id
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }
}
