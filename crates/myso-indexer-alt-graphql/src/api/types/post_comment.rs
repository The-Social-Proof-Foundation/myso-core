// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;

use myso_indexer_alt_social_reader::CommentRow;

#[derive(Clone)]
pub(crate) struct CommentSummary {
    inner: CommentRow,
}

impl CommentSummary {
    pub(crate) fn from_row(inner: CommentRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl CommentSummary {
    async fn comment_id(&self) -> &str {
        &self.inner.comment_id
    }

    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    async fn parent_comment_id(&self) -> Option<&str> {
        self.inner.parent_comment_id.as_deref()
    }

    async fn owner(&self) -> &str {
        &self.inner.owner
    }

    async fn profile_id(&self) -> &str {
        &self.inner.profile_id
    }

    async fn content(&self) -> &str {
        &self.inner.content
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    async fn reaction_count(&self) -> i64 {
        self.inner.reaction_count
    }

    async fn comment_count(&self) -> i64 {
        self.inner.comment_count
    }
}
