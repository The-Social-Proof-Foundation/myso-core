// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;
use myso_indexer_alt_social_reader::PromotedPostRow;

#[derive(Clone)]
pub(crate) struct Promotion {
    inner: PromotedPostRow,
    views: i64,
}

impl Promotion {
    pub(crate) fn from_row(inner: PromotedPostRow, views: i64) -> Self {
        Self { inner, views }
    }
}

#[Object]
impl Promotion {
    /// The promotion ID.
    async fn promotion_id(&self) -> &str {
        &self.inner.promotion_id
    }

    /// The post ID this promotion is for.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Total budget (in MIST).
    async fn budget(&self) -> i64 {
        self.inner.total_budget
    }

    /// Remaining budget (in MIST).
    async fn remaining_budget(&self) -> i64 {
        self.inner.remaining_budget
    }

    /// Promotion status: "active" or "inactive".
    async fn status(&self) -> &str {
        if self.inner.active {
            "active"
        } else {
            "inactive"
        }
    }

    /// Number of views.
    async fn views(&self) -> i64 {
        self.views
    }
}
