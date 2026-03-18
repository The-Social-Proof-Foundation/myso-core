// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;

use crate::api::scalars::myso_address::MySoAddress;

use myso_indexer_alt_social_reader::ReactionRow;

#[derive(Clone)]
pub(crate) struct ReactionSummary {
    inner: ReactionRow,
}

impl ReactionSummary {
    pub(crate) fn from_row(inner: ReactionRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ReactionSummary {
    async fn user_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.user_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn reaction_text(&self) -> &str {
        &self.inner.reaction_text
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }
}
