// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    CommentRow, PostDeletionEventRow, PostModerationEventRow, PostReportRow, PostRow as DbPost,
    PostTransferRow, ReactionRow, RepostRow, TipRow,
};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::date_time::DateTime;
use crate::api::scalars::id::Id;
use crate::api::scalars::json::Json;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::scalars::uint53::UInt53;
use crate::api::types::access::PostAccess;
use crate::api::types::media_asset::{CompositionAnalysis, MediaAsset, RevenueManifest};
use crate::api::types::media_asset_enums::{PostCompositionStatus, PostMonetizationStatus};
use crate::api::types::memory::SocialAttribution;
use crate::api::types::mydata::MyDataRecord;
use crate::api::types::poc::{PocAnalysisResult, PocBadge, PocDispute, PocRevenueRedirection};
use crate::api::types::post_enforcement::{
    ContainerUsageDenial, EmbeddedAssetBinding, PlaybackPolicy, UsageDecisionSnapshot,
    derive_playback_policy, parse_embedded_bindings, parse_usage_decisions, parse_usage_denials,
};
use crate::api::types::profile_summary::ProfileSummary;
use crate::api::types::promotion::Promotion;
use crate::api::types::spot::{
    SpotBet, SpotBetWithdrawal, SpotClaimVerdict, SpotPayout, SpotPostAnalysis, SpotRecord,
    SpotRefund, SpotResolution,
};

// -----------------------------------------------------------------------------
// Post
// -----------------------------------------------------------------------------

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

    /// Profile of the post owner (username, display name, photo, etc.).
    async fn owner_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.owner).await
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

    /// On-chain contract version for this post object.
    async fn contract_version(&self) -> i64 {
        self.inner.contract_version
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

    /// Creator earnings credited on-chain to the post owner (`Post.tips_received`).
    async fn tips_received(&self) -> i64 {
        self.inner.tips_received
    }

    /// Total user spend on this post (sum of indexed tip rows with `is_post = true`).
    async fn total_tip_volume(&self) -> i64 {
        self.inner.total_tip_volume
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

    /// Optional linked MyData object id.
    async fn mydata_id(&self) -> Option<&str> {
        self.inner.mydata_id.as_deref()
    }

    /// Indexed MyData record when `mydata_id` is set and found in the indexer.
    async fn mydata(&self, ctx: &Context<'_>) -> Option<MyDataRecord> {
        let id = self.inner.mydata_id.as_deref()?;
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_mydata_record(id)
            .await
            .ok()
            .flatten()
            .map(MyDataRecord::from_row)
    }

    /// Parent post ID (for quote reposts).
    async fn parent_post_id(&self) -> Option<&str> {
        self.inner.parent_post_id.as_deref()
    }

    /// When the post was last updated (Unix timestamp in milliseconds).
    async fn updated_at(&self) -> Option<i64> {
        self.inner.updated_at
    }

    /// PoC record/badge ID for original content.
    async fn poc_id(&self) -> Option<&str> {
        self.inner.poc_id.as_deref()
    }

    /// Post ID receiving redirected revenue (for derivative content).
    async fn revenue_redirect_to(&self) -> Option<&str> {
        self.inner.revenue_redirect_to.as_deref()
    }

    /// Percentage of revenue redirected to the original creator.
    async fn revenue_redirect_percentage(&self) -> Option<i64> {
        self.inner.revenue_redirect_percentage
    }

    /// Whether Social Proof Token (SPT) features are enabled for this post.
    async fn enable_spt(&self) -> bool {
        self.inner.enable_spt
    }

    /// Oracle reasoning from PoC analysis.
    async fn poc_reasoning(&self) -> Option<&str> {
        self.inner.poc_reasoning.as_deref()
    }

    /// Evidence URLs from PoC analysis.
    async fn poc_evidence_urls(&self) -> Option<Json> {
        self.inner
            .poc_evidence_urls
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// Highest similarity score from PoC analysis (0-100).
    async fn poc_similarity_score(&self) -> Option<i64> {
        self.inner.poc_similarity_score
    }

    /// PoC outcome code from chain (legacy post-level PoC; deprecated for MediaAsset-centric posts).
    async fn poc_outcome(&self) -> Option<i32> {
        self.inner.poc_outcome.map(i32::from)
    }

    /// Composition verification status (0=none, 1=pending, 2=verified, 3=invalid, 4=partially_restricted).
    async fn composition_status(&self) -> Option<PostCompositionStatus> {
        self.inner
            .composition_status
            .map(|v| PostCompositionStatus::from(v))
    }

    /// Monetization eligibility status (0=none, 1=pending, 2=enabled, 3=restricted).
    async fn monetization_status(&self) -> Option<PostMonetizationStatus> {
        self.inner
            .monetization_status
            .map(|v| PostMonetizationStatus::from(v))
    }

    /// Canonical MediaAsset object ids referenced by this post.
    async fn media_asset_ids(&self) -> Option<Json> {
        self.inner
            .media_asset_ids
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// Resolved MediaAsset records for ids indexed on this post.
    async fn media_assets(&self, ctx: &Context<'_>) -> Option<Vec<MediaAsset>> {
        let ids = self.inner.media_asset_ids.as_ref()?;
        let arr = ids.as_array()?;
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let mut out = Vec::new();
        for id in arr {
            let Some(id_str) = id.as_str() else {
                continue;
            };
            if let Ok(Some(row)) = reader.get_media_asset_by_id(id_str).await {
                out.push(MediaAsset::from_row(row));
            }
        }
        Some(out)
    }

    /// Latest composition analysis sidecar for this post.
    async fn composition_analysis(&self, ctx: &Context<'_>) -> Option<CompositionAnalysis> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_composition_analysis_for_post(&self.inner.post_id)
            .await
            .ok()
            .flatten()
            .map(CompositionAnalysis::from_row)
    }

    /// Latest revenue manifest indexed for creator-attributable tip splits on this post.
    async fn revenue_manifest(&self, ctx: &Context<'_>) -> Option<RevenueManifest> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_revenue_manifest_for_post(&self.inner.post_id)
            .await
            .ok()
            .flatten()
            .map(RevenueManifest::from_row)
    }

    /// Embedded media asset bindings detected in this post's composition.
    async fn embedded_bindings(&self) -> Option<Vec<EmbeddedAssetBinding>> {
        self.inner
            .embedded_bindings
            .as_ref()
            .map(parse_embedded_bindings)
    }

    /// Effective usage decision snapshots per embedded binding.
    async fn usage_decisions(&self) -> Option<Vec<UsageDecisionSnapshot>> {
        self.inner
            .usage_decisions
            .as_ref()
            .map(parse_usage_decisions)
    }

    /// Active container usage denials scoped to bindings on this post.
    async fn usage_denials(&self) -> Option<Vec<ContainerUsageDenial>> {
        self.inner.usage_denials.as_ref().map(parse_usage_denials)
    }

    /// Derived playback/monetization restrictions from bindings and decision snapshots.
    async fn playback_policy(&self) -> PlaybackPolicy {
        derive_playback_policy(
            self.inner.embedded_bindings.as_ref(),
            self.inner.usage_decisions.as_ref(),
        )
    }

    /// Revenue redirect mode (none, wallet, escrow, treasury).
    async fn poc_redirection_kind(&self) -> Option<i32> {
        self.inner.poc_redirection_kind.map(i32::from)
    }

    /// True when this reservation amount's creator-fee slice hits an escrow revenue-manifest
    /// entry and Move requires `reserve_towards_post_with_platform` (vault).
    async fn post_reserve_requires_beneficiary_vault(
        &self,
        ctx: &Context<'_>,
        reservation_amount_mist: UInt53,
    ) -> bool {
        let amount = u64::from(reservation_amount_mist);
        if amount == 0 {
            return false;
        }
        let monetization_enabled = myso_indexer_alt_social_schema::models::MONETIZATION_ENABLED;
        if self
            .inner
            .monetization_status
            .is_some_and(|status| status != monetization_enabled)
        {
            return false;
        }
        let Some(reader_opt) = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()
        else {
            return false;
        };
        let Some(reader) = reader_opt.as_ref().as_ref() else {
            return false;
        };
        let Ok(Some(manifest)) = reader
            .get_revenue_manifest_for_post(&self.inner.post_id)
            .await
        else {
            return false;
        };
        let (creator_bps, platform_bps, treasury_bps, non_platform_to_creator_bps) =
            match reader.get_spt_exchange_config().await {
                Ok(Some(config)) => (
                    config.reservation_creator_fee_bps.max(0) as u64,
                    config.reservation_platform_fee_bps.max(0) as u64,
                    config.reservation_treasury_fee_bps.max(0) as u64,
                    config.non_platform_platform_to_creator_bps.max(0) as u64,
                ),
                _ => (
                    myso_indexer_alt_social_schema::models::DEFAULT_RESERVATION_CREATOR_FEE_BPS
                        as u64,
                    myso_indexer_alt_social_schema::models::DEFAULT_RESERVATION_PLATFORM_FEE_BPS
                        as u64,
                    myso_indexer_alt_social_schema::models::DEFAULT_RESERVATION_TREASURY_FEE_BPS
                        as u64,
                    5_000,
                ),
            };
        let creator_fee = myso_indexer_alt_social_schema::models::reservation_creator_fee_for_vault_check(
            amount,
            creator_bps,
            platform_bps,
            treasury_bps,
            non_platform_to_creator_bps,
            true,
        );
        myso_indexer_alt_social_schema::models::post_reserve_requires_beneficiary_vault_for_amount(
            &manifest.entries_json,
            creator_fee,
        )
    }

    /// Media type analyzed (1=image, 2=video, 3=audio).
    async fn poc_media_type(&self) -> Option<i16> {
        self.inner.poc_media_type
    }

    /// Oracle address that performed the PoC analysis.
    async fn poc_oracle_address(&self) -> Option<&str> {
        self.inner.poc_oracle_address.as_deref()
    }

    /// When the post was analyzed (epoch milliseconds).
    async fn poc_analyzed_at(&self) -> Option<i64> {
        self.inner.poc_analyzed_at
    }

    /// Linked SPT pool id when set.
    async fn spt_id(&self) -> Option<&str> {
        self.inner.spt_id.as_deref()
    }

    /// Platform id where this post was created (indexed from `PostCreatedEvent`).
    async fn platform_id(&self) -> Option<&str> {
        self.inner.platform_id.as_deref()
    }

    /// Permission bitfield from chain (`post.move` PERMISSION_*).
    async fn permissions(&self) -> Option<i32> {
        self.inner.permissions.map(i32::from)
    }

    /// Actor vs principal attribution for delegated sub-agent actions.
    async fn attribution(&self) -> SocialAttribution {
        SocialAttribution::from_post(&self.inner.owner, &self.inner)
    }

    /// Agentic organization attributed to this post, when indexed.
    async fn organization_id(&self) -> Option<&str> {
        self.inner.organization_id.as_deref()
    }

    /// Optional off-chain / indexed revenue recipient address.
    async fn revenue_recipient(&self) -> Option<&str> {
        self.inner.revenue_recipient.as_deref()
    }

    /// Content access model for this post.
    async fn access(&self) -> PostAccess {
        PostAccess::from(self.inner.resolve_post_access())
    }

    #[graphql(deprecation = "Use `access.kind` instead.")]
    async fn requires_subscription(&self) -> Option<bool> {
        Some(
            self.inner.resolve_post_access().kind
                == myso_indexer_alt_social_reader::PostAccessKind::ProfileSubscription,
        )
    }

    #[graphql(deprecation = "Use `access.subscriptionServiceId` instead.")]
    async fn subscription_service_id(&self) -> Option<String> {
        self.inner
            .resolve_post_access()
            .subscription_service_id
            .clone()
    }

    /// Minimum active profile-subscription plan price for a profile-gated post.
    /// One-time post pricing is exposed by the linked MyData record.
    async fn subscription_price(&self) -> Option<i64> {
        self.inner.subscription_price
    }

    /// Encrypted content hash when the post is paywalled / encrypted.
    async fn encrypted_content_hash(&self) -> Option<&str> {
        self.inner.encrypted_content_hash.as_deref()
    }

    async fn removed_from_platform(&self) -> Option<bool> {
        self.inner.removed_from_platform
    }

    async fn removed_by(&self) -> Option<&str> {
        self.inner.removed_by.as_deref()
    }

    /// Optional post metadata JSON.
    async fn metadata_json(&self) -> Option<Json> {
        self.inner
            .metadata_json
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// On-chain promotion object id when promoted.
    async fn promotion_id(&self) -> Option<&str> {
        self.inner.promotion_id.as_deref()
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
        Some(
            rows.into_iter()
                .map(PostTransferSummary::from_row)
                .collect(),
        )
    }

    /// User reports filed against this post (paginated, newest first).
    async fn reports(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PostReport>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_post_reports(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PostReport::from_row).collect())
    }

    /// Platform moderation actions on this post (paginated, newest first).
    async fn moderation_events(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PostModerationEvent>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_post_moderation_events(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PostModerationEvent::from_row)
                .collect(),
        )
    }

    /// Deletion events for this post (paginated, newest first).
    async fn deletion_events(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PostDeletionEvent>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_post_deletion_events(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PostDeletionEvent::from_row).collect())
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

    /// Multi-claim SPoT analysis status/counts for this post (null only if the post is missing).
    async fn spot_analysis(&self, ctx: &Context<'_>) -> Option<SpotPostAnalysis> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_spot_post_analysis(&self.inner.post_id)
            .await
            .ok()?;
        row.map(SpotPostAnalysis::from_row)
    }

    /// Past-claim verdicts for this post, in claim_index order (empty when none).
    async fn spot_verdicts(&self, ctx: &Context<'_>) -> Vec<SpotClaimVerdict> {
        let Some(reader_opt) = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()
        else {
            return Vec::new();
        };
        let Some(reader) = reader_opt.as_ref().as_ref() else {
            return Vec::new();
        };
        reader
            .list_spot_verdicts_for_post(&self.inner.post_id)
            .await
            .map(|rows| rows.into_iter().map(SpotClaimVerdict::from_row).collect())
            .unwrap_or_default()
    }

    /// Spot payouts for this post (paginated).
    async fn spot_payouts(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SpotPayout>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_spot_payouts(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SpotPayout::from_row).collect())
    }

    /// Spot refunds for this post (paginated).
    async fn spot_refunds(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SpotRefund>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_spot_refunds(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SpotRefund::from_row).collect())
    }

    /// Spot resolution for this post (1:1, null if not resolved).
    async fn spot_resolution(&self, ctx: &Context<'_>) -> Option<SpotResolution> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader.get_spot_resolution(&self.inner.post_id).await.ok()?;
        row.map(SpotResolution::from_row)
    }

    /// Spot bet withdrawals for this post (paginated).
    async fn spot_bet_withdrawals(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SpotBetWithdrawal>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_spot_bet_withdrawals(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SpotBetWithdrawal::from_row).collect())
    }

    /// Promotion for this post (null if not promoted).
    async fn promotion(&self, ctx: &Context<'_>) -> Option<Promotion> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_promotion_by_post_id(&self.inner.post_id)
            .await
            .ok()?;
        let row = row?;
        let views = reader
            .get_promotion_views_count(&row.promotion_id)
            .await
            .ok()?;
        Some(Promotion::from_row(row, views))
    }

    /// PoC badges for this post (paginated).
    async fn poc_badges(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PocBadge>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_poc_badges_for_post(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PocBadge::from_row).collect())
    }

    /// Revenue redirections for this post (as accused or original).
    async fn revenue_redirections(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PocRevenueRedirection>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_post_revenue_redirections(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PocRevenueRedirection::from_row)
                .collect(),
        )
    }

    /// Latest PoC analysis result for this post.
    async fn poc_analysis(&self, ctx: &Context<'_>) -> Option<Option<PocAnalysisResult>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_poc_analysis_for_post(&self.inner.post_id)
            .await
            .ok()?;
        Some(row.map(PocAnalysisResult::from_row))
    }

    /// PoC disputes for this post (paginated).
    async fn poc_disputes(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PocDispute>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_poc_disputes_for_post(&self.inner.post_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PocDispute::from_row).collect())
    }

    /// Successful PoC dispute submissions over this post's lifetime (`0`–`2`); not reset when PoC is cleared.
    async fn poc_disputes_submitted(&self) -> i32 {
        i32::from(self.inner.poc_disputes_submitted)
    }
}

/// Paginated posts for a profile (offset/limit + total count).
#[derive(Clone)]
pub(crate) struct PostPage {
    pub(crate) items: Vec<Post>,
    pub(crate) total_count: i64,
    pub(crate) limit: i64,
    pub(crate) offset: i64,
    total_pages: i64,
}

impl PostPage {
    pub(crate) fn new(items: Vec<Post>, total_count: i64, limit: i64, offset: i64) -> Self {
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + limit - 1) / limit
        };
        Self {
            items,
            total_count,
            limit,
            offset,
            total_pages,
        }
    }
}

#[Object]
impl PostPage {
    async fn items(&self) -> Vec<Post> {
        self.items.clone()
    }

    async fn total_count(&self) -> i64 {
        self.total_count
    }

    async fn limit(&self) -> i64 {
        self.limit
    }

    async fn offset(&self) -> i64 {
        self.offset
    }

    /// Total pages for this `limit` (0 when there are no posts).
    async fn total_pages(&self) -> i64 {
        self.total_pages
    }
}

// -----------------------------------------------------------------------------
// CommentSummary
// -----------------------------------------------------------------------------

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

    /// Profile of the comment owner (username, display name, photo, etc.).
    async fn owner_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.owner).await
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

    /// Commenter tip earnings credited on-chain (`Comment.tips_received`).
    async fn tips_received(&self) -> i64 {
        self.inner.tips_received
    }

    async fn attribution(&self) -> SocialAttribution {
        SocialAttribution::from_comment(&self.inner)
    }

    /// User reports filed against this comment (paginated, newest first).
    async fn reports(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PostReport>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_comment_reports(&self.inner.comment_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PostReport::from_row).collect())
    }

    /// Platform moderation actions on this comment (paginated, newest first).
    async fn moderation_events(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PostModerationEvent>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_comment_moderation_events(&self.inner.comment_id, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PostModerationEvent::from_row)
                .collect(),
        )
    }

    /// Deletion events for this comment (paginated, newest first).
    async fn deletion_events(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PostDeletionEvent>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_comment_deletion_events(&self.inner.comment_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PostDeletionEvent::from_row).collect())
    }
}

// -----------------------------------------------------------------------------
// ReactionSummary
// -----------------------------------------------------------------------------

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

    /// Profile of the user who reacted.
    async fn user_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.user_address).await
    }

    async fn reaction_text(&self) -> &str {
        &self.inner.reaction_text
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    async fn actor_address(&self) -> MySoAddress {
        MySoAddress::from_str(
            self.inner
                .actor_address
                .as_deref()
                .unwrap_or(&self.inner.user_address),
        )
        .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn principal_owner(&self) -> Option<MySoAddress> {
        self.inner
            .principal_owner
            .as_deref()
            .and_then(|s| MySoAddress::from_str(s).ok())
            .map(Into::into)
    }

    async fn sub_agent_id(&self) -> Option<&str> {
        self.inner.sub_agent_id.as_deref()
    }

    async fn action_identity_class(&self) -> i32 {
        i32::from(self.inner.action_identity_class.unwrap_or(0))
    }

    async fn attribution(&self) -> SocialAttribution {
        SocialAttribution::from_reaction(&self.inner)
    }
}

// -----------------------------------------------------------------------------
// RepostSummary
// -----------------------------------------------------------------------------

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

    /// Profile of the repost owner.
    async fn owner_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.owner).await
    }

    async fn profile_id(&self) -> &str {
        &self.inner.profile_id
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    async fn attribution(&self) -> SocialAttribution {
        SocialAttribution::from_repost(&self.inner)
    }
}

// -----------------------------------------------------------------------------
// TipSummary
// -----------------------------------------------------------------------------

#[derive(Clone)]
pub(crate) struct TipSummary {
    inner: TipRow,
}

impl TipSummary {
    pub(crate) fn from_row(inner: TipRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl TipSummary {
    async fn tipper(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.tipper)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn recipient(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.recipient)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the tip recipient.
    async fn recipient_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.recipient).await
    }

    /// Profile of the tipper.
    async fn tipper_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.tipper).await
    }

    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }

    async fn organization_id(&self) -> Option<&str> {
        self.inner.organization_id.as_deref()
    }
}

// -----------------------------------------------------------------------------
// PostTransferSummary
// -----------------------------------------------------------------------------

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

    /// Profile of the new owner.
    async fn new_owner_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.new_owner).await
    }

    /// Profile of the previous owner.
    async fn previous_owner_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.previous_owner).await
    }

    async fn transferred_at(&self) -> i64 {
        self.inner.transferred_at
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

// -----------------------------------------------------------------------------
// PostReport
// -----------------------------------------------------------------------------

#[derive(Clone)]
pub(crate) struct PostReport {
    inner: PostReportRow,
}

impl PostReport {
    pub(crate) fn from_row(inner: PostReportRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PostReport {
    /// Database id for this report row.
    async fn report_id(&self) -> i32 {
        self.inner.id
    }

    /// Post or comment id that was reported.
    async fn object_id(&self) -> &str {
        &self.inner.object_id
    }

    /// True if the report targets a comment; false if it targets a post.
    async fn is_comment(&self) -> bool {
        self.inner.is_comment
    }

    async fn reporter(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.reporter)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the reporter.
    async fn reporter_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.reporter).await
    }

    /// On-chain user report reason code.
    async fn reason_code(&self) -> i32 {
        i32::from(self.inner.reason_code)
    }

    async fn description(&self) -> &str {
        &self.inner.description
    }

    /// When the report was filed (Unix ms from myso::clock::Clock at 0x6).
    async fn reported_at(&self) -> i64 {
        self.inner.reported_at
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

// -----------------------------------------------------------------------------
// PostDeletionEvent
// -----------------------------------------------------------------------------

#[derive(Clone)]
pub(crate) struct PostDeletionEvent {
    inner: PostDeletionEventRow,
}

impl PostDeletionEvent {
    pub(crate) fn from_row(inner: PostDeletionEventRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PostDeletionEvent {
    /// Database id for this deletion row.
    async fn event_id(&self) -> i32 {
        self.inner.id
    }

    /// Post or comment id that was deleted.
    async fn object_id(&self) -> &str {
        &self.inner.object_id
    }

    async fn owner(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the owner (at deletion time).
    async fn owner_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.owner).await
    }

    async fn profile_id(&self) -> &str {
        &self.inner.profile_id
    }

    /// True if the deleted object was a post; false if a comment.
    async fn is_post(&self) -> bool {
        self.inner.is_post
    }

    async fn post_type(&self) -> Option<&str> {
        self.inner.post_type.as_deref()
    }

    /// Parent post id (for comment deletions) or the post id for post deletions, when set.
    async fn post_id(&self) -> Option<&str> {
        self.inner.post_id.as_deref()
    }

    /// Epoch milliseconds when the chain recorded deletion.
    async fn deleted_at(&self) -> i64 {
        self.inner.deleted_at
    }

    /// Hypertable timestamp for this row.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

// -----------------------------------------------------------------------------
// PostModerationEvent
// -----------------------------------------------------------------------------

#[derive(Clone)]
pub(crate) struct PostModerationEvent {
    inner: PostModerationEventRow,
}

impl PostModerationEvent {
    pub(crate) fn from_row(inner: PostModerationEventRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PostModerationEvent {
    /// Database id for this moderation row.
    async fn event_id(&self) -> i32 {
        self.inner.id
    }

    /// Post or comment id that was moderated.
    async fn object_id(&self) -> &str {
        &self.inner.object_id
    }

    async fn platform_id(&self) -> &str {
        &self.inner.platform_id
    }

    async fn removed(&self) -> bool {
        self.inner.removed
    }

    async fn moderated_by(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.moderated_by)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the moderator.
    async fn moderated_by_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.moderated_by).await
    }

    /// When the action was recorded (epoch milliseconds; set by the indexer when absent on-chain).
    async fn moderated_at(&self) -> i64 {
        self.inner.moderated_at
    }

    /// Hypertable timestamp for this row.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}
