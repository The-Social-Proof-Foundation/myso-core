// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::PostRow as DbPost;

use crate::api::scalars::id::Id;
use crate::api::scalars::json::Json;
use crate::api::types::post_comment::CommentSummary;
use crate::api::types::post_reaction::ReactionSummary;
use crate::api::types::post_repost::RepostSummary;
use crate::api::types::post_tip::TipSummary;
use crate::api::types::post_transfer::PostTransferSummary;
use crate::api::types::promotion::Promotion;
use crate::api::types::spot::{SpotBet, SpotRecord};

#[derive(Clone)]
pub(crate) struct Post {
    inner: DbPost,
}

impl Post {
    pub(crate) fn from_db(inner: DbPost) -> Self {
        Self { inner }
    }
}

#[Object]
impl Post {
    /// The post's globally unique identifier.
    pub async fn id(&self) -> Id {
        Id::Post(self.inner.post_id.clone())
    }

    /// The post ID.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// The wallet address of the post owner.
    async fn owner(&self) -> &str {
        &self.inner.owner
    }

    /// The profile ID of the post owner.
    async fn profile_id(&self) -> &str {
        &self.inner.profile_id
    }

    /// The post content.
    async fn content(&self) -> &str {
        &self.inner.content
    }

    /// The post type.
    async fn post_type(&self) -> &str {
        &self.inner.post_type
    }

    /// When the post was created (Unix timestamp in milliseconds).
    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    /// Number of reactions.
    async fn reaction_count(&self) -> i64 {
        self.inner.reaction_count
    }

    /// Number of comments.
    async fn comment_count(&self) -> i64 {
        self.inner.comment_count
    }

    /// Number of reposts.
    async fn repost_count(&self) -> i64 {
        self.inner.repost_count
    }

    /// Total tips received.
    async fn tips_received(&self) -> i64 {
        self.inner.tips_received
    }

    /// Media URLs (JSON array).
    async fn media_urls(&self) -> Option<Json> {
        self.inner
            .media_urls
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// Mentions (JSON).
    async fn mentions(&self) -> Option<Json> {
        self.inner
            .mentions
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// Parent post ID (for quote reposts).
    async fn parent_post_id(&self) -> Option<&str> {
        self.inner.parent_post_id.as_deref()
    }

    /// When the post was last updated (Unix timestamp in milliseconds).
    async fn updated_at(&self) -> Option<i64> {
        self.inner.updated_at
    }

    /// Comments on this post (paginated).
    async fn comments(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<CommentSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_post_comments(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(CommentSummary::from_row).collect())
    }

    /// Reactions on this post (paginated).
    async fn reactions(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<ReactionSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_post_reactions(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(ReactionSummary::from_row).collect())
    }

    /// Reposts of this post (paginated).
    async fn reposts(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<RepostSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_post_reposts(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(RepostSummary::from_row).collect())
    }

    /// Tips received for this post (paginated).
    async fn tips(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<TipSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_post_tips(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(TipSummary::from_row).collect())
    }

    /// Ownership transfers for this post (paginated).
    async fn transfers(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PostTransferSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_post_transfers(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PostTransferSummary::from_row).collect())
    }

    /// Spot bets for this post (paginated).
    async fn spot_bets(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SpotBet>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_spot_bets(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SpotBet::from_row).collect())
    }

    /// Spot record for this post (1:1, null if no record).
    async fn spot_record(&self, ctx: &Context<'_>) -> Option<SpotRecord> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader.get_spot_record(&self.inner.post_id).await.ok()?;
        row.map(SpotRecord::from_row)
    }

    /// Promotion for this post (null if not promoted).
    async fn promotion(&self, ctx: &Context<'_>) -> Option<Promotion> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader.get_promotion_by_post_id(&self.inner.post_id).await.ok()?;
        let row = row?;
        let views = reader.get_promotion_views_count(&row.promotion_id).await.ok()?;
        Some(Promotion::from_row(row, views))
    }
}
