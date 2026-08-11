// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    ProfileBadgeRow, ProfileByAddressResponse, ReservationStatus, SelectedBadgeInfo,
    SocialProofTokenInfo,
};
use myso_indexer_alt_social_schema::models::Profile as SchemaProfile;
use tracing::warn;

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::big_int::BigInt;
use crate::api::scalars::id::Id;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::ai_credit::AiCreditBalance;
use crate::api::types::blocked::{BlockedPlatformSummary, BlockedProfileSummary};
use crate::api::types::enterprise::{AuditLogConnection, AuditLogEntry, AuditLogFilterInput};
use crate::api::types::memory::{MemoryAccount, SubAgent};
use crate::api::types::mydata::MyDataRecord;
use crate::api::types::organization::AgenticOrganization;
use crate::api::types::platform::{PlatformMembershipPage, PlatformMembershipSummary};
use crate::api::types::pnl::{ProfilePnLWindow, ProfilePnLWindowStats};
use crate::api::types::post::{Post, PostPage};
use crate::api::types::profile_summary::ProfileSummary;
use crate::api::types::spt::{SptHolding, SptReservationHolding};
use crate::api::types::vesting::VestingWallet;

fn normalize_poc_outcomes(poc_outcomes: Option<Vec<i32>>) -> Option<Vec<i16>> {
    poc_outcomes.and_then(|xs| {
        let v: Vec<i16> = xs
            .into_iter()
            .filter_map(|x| i16::try_from(x).ok())
            .collect();
        (!v.is_empty()).then_some(v)
    })
}

#[derive(Clone)]
pub(crate) struct Profile {
    inner: ProfileByAddressResponse,
}

impl Profile {
    pub(crate) fn from_response(inner: ProfileByAddressResponse) -> Option<Self> {
        if inner.username.is_some() {
            Some(Self { inner })
        } else {
            None
        }
    }

    pub(crate) fn from_db(inner: SchemaProfile) -> Self {
        let response = ProfileByAddressResponse {
            id: Some(inner.id),
            owner_address: inner.owner_address,
            profile_id: inner.profile_id,
            username: Some(inner.username),
            display_name: inner.display_name,
            bio: inner.bio,
            profile_photo: inner.profile_photo,
            cover_photo: inner.cover_photo,
            website: inner.website,
            created_at: Some(inner.created_at.and_utc().timestamp_millis()),
            updated_at: Some(inner.updated_at.and_utc().timestamp_millis()),
            followers_count: inner.followers_count,
            following_count: inner.following_count,
            post_count: inner.post_count,
            blocked_count: inner.blocked_count,
            birthdate: inner.birthdate,
            location: inner.location,
            x_username: inner.x_username,
            block_list_address: None,
            social_proof_token_address: inner.social_proof_token_address,
            reservation_pool_address: inner.reservation_pool_address,
            social_proof_token: None,
            selected_badge: None,
            selected_badge_id: inner.selected_badge_id,
            selected_ecosystem_badge_id: inner.selected_ecosystem_badge_id,
            contract_version: inner.contract_version,
        };
        Self { inner: response }
    }

    fn owner_as_address(&self) -> myso_types::base_types::MySoAddress {
        MySoAddress::from_str(&self.inner.owner_address)
            .map(Into::into)
            .unwrap_or(myso_types::base_types::MySoAddress::ZERO)
    }
}

#[Object]
impl Profile {
    /// The profile's globally unique identifier.
    pub async fn id(&self) -> Id {
        Id::Profile(self.owner_as_address())
    }

    /// The wallet address that owns this profile.
    async fn address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// The profile's username.
    async fn username(&self) -> String {
        self.inner.username.clone().unwrap_or_default()
    }

    /// The profile's display name.
    async fn display_name(&self) -> Option<&str> {
        self.inner.display_name.as_deref()
    }

    /// The profile's bio.
    async fn bio(&self) -> Option<&str> {
        self.inner.bio.as_deref()
    }

    /// URL to the profile photo.
    async fn profile_photo(&self) -> Option<&str> {
        self.inner.profile_photo.as_deref()
    }

    /// URL to the cover photo.
    async fn cover_photo(&self) -> Option<&str> {
        self.inner.cover_photo.as_deref()
    }

    /// The profile's website.
    async fn website(&self) -> Option<&str> {
        self.inner.website.as_deref()
    }

    /// When the profile was created (Unix ms from chain Clock at 0x6).
    async fn created_at(&self) -> Option<i64> {
        self.inner.created_at
    }

    /// On-chain contract version for this profile object.
    async fn contract_version(&self) -> i64 {
        self.inner.contract_version
    }

    /// When the profile was last updated (Unix ms from chain Clock at 0x6).
    async fn updated_at(&self) -> Option<i64> {
        self.inner.updated_at
    }

    /// The profile ID (object address).
    async fn profile_id(&self) -> Option<&str> {
        self.inner.profile_id.as_deref()
    }

    /// Number of followers.
    async fn followers_count(&self) -> i32 {
        self.inner.followers_count
    }

    /// Number of accounts this profile follows.
    async fn following_count(&self) -> i32 {
        self.inner.following_count
    }

    /// Number of posts.
    async fn post_count(&self) -> i32 {
        self.inner.post_count
    }

    /// Birthdate.
    async fn birthdate(&self) -> Option<&str> {
        self.inner.birthdate.as_deref()
    }

    /// Location.
    async fn location(&self) -> Option<&str> {
        self.inner.location.as_deref()
    }

    /// X (Twitter) username.
    async fn x_username(&self) -> Option<&str> {
        self.inner.x_username.as_deref()
    }

    /// Block list address.
    async fn block_list_address(&self) -> Option<&str> {
        self.inner.block_list_address.as_deref()
    }

    /// Social proof token address.
    async fn social_proof_token_address(&self) -> Option<&str> {
        self.inner.social_proof_token_address.as_deref()
    }

    /// Reservation pool address.
    async fn reservation_pool_address(&self) -> Option<&str> {
        self.inner.reservation_pool_address.as_deref()
    }

    /// Social proof token info (reservation pool fields match nested profile summaries; loads via social reader when configured).
    async fn social_proof_token(&self, ctx: &Context<'_>) -> Option<SocialProofToken> {
        if let Some(reader_opt) =
            ctx.data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()
        {
            if let Some(reader) = reader_opt.as_ref().as_ref() {
                if let Ok(Some(enriched)) = reader
                    .get_profile_summary_enriched(&self.inner.owner_address)
                    .await
                {
                    if let Some(ref spt) = enriched.social_proof_token {
                        return Some(SocialProofToken::from(spt));
                    }
                }
            }
        }
        self.inner
            .social_proof_token
            .as_ref()
            .map(SocialProofToken::from)
    }

    /// Selected badge info.
    async fn selected_badge(&self) -> Option<SelectedBadge> {
        self.inner.selected_badge.as_ref().map(SelectedBadge::from)
    }

    /// Selected badge ID.
    async fn selected_badge_id(&self) -> Option<&str> {
        self.inner.selected_badge_id.as_deref()
    }

    /// Selected ecosystem badge ID.
    async fn selected_ecosystem_badge_id(&self) -> Option<&str> {
        self.inner.selected_ecosystem_badge_id.as_deref()
    }

    /// Profile badges (paginated).
    async fn badges(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<ProfileBadge>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_profile_badges(&self.inner.owner_address, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(ProfileBadge::from_row).collect())
    }

    /// Followers (paginated).
    async fn followers(
        &self,
        ctx: &Context<'_>,
        viewer: Option<MySoAddress>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<ProfileSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        match reader
            .get_followers(
                &self.inner.owner_address,
                limit,
                offset,
                viewer_s.as_deref(),
            )
            .await
        {
            Ok(rows) => Some(rows.into_iter().map(ProfileSummary::from_row).collect()),
            Err(e) => {
                warn!(
                    owner_address = %self.inner.owner_address,
                    error = %e,
                    "followers query failed"
                );
                None
            }
        }
    }

    /// Following (paginated).
    async fn following(
        &self,
        ctx: &Context<'_>,
        viewer: Option<MySoAddress>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<ProfileSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        match reader
            .get_following(
                &self.inner.owner_address,
                limit,
                offset,
                viewer_s.as_deref(),
            )
            .await
        {
            Ok(rows) => Some(rows.into_iter().map(ProfileSummary::from_row).collect()),
            Err(e) => {
                warn!(
                    owner_address = %self.inner.owner_address,
                    error = %e,
                    "following query failed"
                );
                None
            }
        }
    }

    /// Follow suggestions for the browsing viewer while viewing this profile.
    ///
    /// Candidates come from this profile's friends-of-friends. Results exclude accounts
    /// the viewer (or profile, when `viewer` is omitted) already follows, require overlap
    /// through the viewer's following graph, and are ranked by that overlap (`mutualCount`).
    async fn recommendations(
        &self,
        ctx: &Context<'_>,
        viewer: Option<MySoAddress>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<ProfileSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let (rows, _) = reader
            .get_follow_recommendations(
                &self.inner.owner_address,
                limit,
                offset,
                viewer_s.as_deref(),
                myso_indexer_alt_social_reader::MAX_MUTUAL_CONNECTIONS_LIMIT,
            )
            .await
            .ok()?;
        Some(rows.into_iter().map(ProfileSummary::from_row).collect())
    }

    /// Profiles this user has blocked (paginated).
    async fn blocked_profiles(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<BlockedProfileSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_blocked_profiles(&self.inner.owner_address, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(BlockedProfileSummary::from_row)
                .collect(),
        )
    }

    /// Platforms that have blocked this profile (paginated).
    async fn blocked_platforms(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<BlockedPlatformSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_blocked_platforms(&self.inner.owner_address, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(BlockedPlatformSummary::from_row)
                .collect(),
        )
    }

    /// Platforms this profile has joined (paginated). Each row includes platform fields in one SQL round-trip.
    async fn platform_memberships(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PlatformMembershipSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_profile_platform_memberships(&self.inner.owner_address, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PlatformMembershipSummary::from_row)
                .collect(),
        )
    }

    /// Same rows as `platformMemberships` plus total count and `totalPages` for UI pagers.
    async fn platform_memberships_page(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<PlatformMembershipPage> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let total_count = reader
            .count_profile_platform_memberships(&self.inner.owner_address)
            .await
            .ok()?;
        let rows = reader
            .get_profile_platform_memberships(&self.inner.owner_address, limit, offset)
            .await
            .ok()?;
        let items = rows
            .into_iter()
            .map(PlatformMembershipSummary::from_row)
            .collect();
        Some(PlatformMembershipPage::new(
            items,
            total_count,
            limit,
            offset,
        ))
    }

    /// Total number of platform memberships for this profile (same filter as `platformMemberships`).
    async fn platform_memberships_total(&self, ctx: &Context<'_>) -> Option<i64> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .count_profile_platform_memberships(&self.inner.owner_address)
            .await
            .ok()
    }

    /// Vesting wallets owned by this profile (paginated).
    async fn vesting_wallets(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<VestingWallet>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_vesting_wallets(Some(&self.inner.owner_address), false, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(VestingWallet::from_row).collect())
    }

    /// SPT holdings for this profile (paginated).
    async fn spt_holdings(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptHolding>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_spt_holdings_by_holder(&self.inner.owner_address, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(SptHolding::from_row).collect())
    }

    /// Reservation SPT holdings for this profile (paginated).
    async fn reservation_holdings(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptReservationHolding>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        match reader
            .get_spt_reservation_holdings_for_reserver(&self.inner.owner_address, limit, offset)
            .await
        {
            Ok(rows) => Some(
                rows.into_iter()
                    .map(SptReservationHolding::from_row)
                    .collect(),
            ),
            Err(e) => {
                warn!(
                    owner_address = %self.inner.owner_address,
                    error = %e,
                    "reservation_holdings query failed"
                );
                None
            }
        }
    }

    /// Posts by this profile (paginated, newest first). Matches REST `GET /profiles/:address/posts`
    /// scope: `posts.owner` is this profile's wallet **or** `posts.profile_id` is this profile's object id.
    async fn posts(
        &self,
        ctx: &Context<'_>,
        post_type: Option<String>,
        poc_outcomes: Option<Vec<i32>>,
        include_removed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<Post>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let poc_outcomes_i16 = normalize_poc_outcomes(poc_outcomes);
        let rows = reader
            .list_posts_for_profile(
                &self.inner.owner_address,
                self.inner.profile_id.as_deref(),
                post_type.as_deref(),
                poc_outcomes_i16,
                include_removed.unwrap_or(false),
                limit,
                offset,
            )
            .await
            .ok()?;
        Some(rows.into_iter().map(Post::from_db).collect())
    }

    /// Same rows as `posts` with total count and `totalPages` for pagers.
    async fn posts_page(
        &self,
        ctx: &Context<'_>,
        post_type: Option<String>,
        poc_outcomes: Option<Vec<i32>>,
        include_removed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<PostPage> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let poc_outcomes_i16 = normalize_poc_outcomes(poc_outcomes);
        let include_removed = include_removed.unwrap_or(false);
        let total_count = reader
            .count_posts_for_profile(
                &self.inner.owner_address,
                self.inner.profile_id.as_deref(),
                post_type.as_deref(),
                poc_outcomes_i16.clone(),
                include_removed,
            )
            .await
            .ok()?;
        let rows = reader
            .list_posts_for_profile(
                &self.inner.owner_address,
                self.inner.profile_id.as_deref(),
                post_type.as_deref(),
                poc_outcomes_i16,
                include_removed,
                limit,
                offset,
            )
            .await
            .ok()?;
        let items = rows.into_iter().map(Post::from_db).collect();
        Some(PostPage::new(items, total_count, limit, offset))
    }

    /// Total post count for this profile (same filters as `posts` / `postsPage`).
    async fn posts_total_count(
        &self,
        ctx: &Context<'_>,
        post_type: Option<String>,
        poc_outcomes: Option<Vec<i32>>,
        include_removed: Option<bool>,
    ) -> Option<i64> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let poc_outcomes_i16 = normalize_poc_outcomes(poc_outcomes);
        reader
            .count_posts_for_profile(
                &self.inner.owner_address,
                self.inner.profile_id.as_deref(),
                post_type.as_deref(),
                poc_outcomes_i16,
                include_removed.unwrap_or(false),
            )
            .await
            .ok()
    }

    /// Linked memory account object id for this profile's sub-agent registry.
    async fn memory_account_id(&self, ctx: &Context<'_>) -> Option<String> {
        let profile_id = self.inner.profile_id.as_deref()?;
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_profile_memory_account_id(profile_id)
            .await
            .ok()
            .flatten()
    }

    /// Memory account row for this profile owner.
    async fn memory_account(&self, ctx: &Context<'_>) -> Option<MemoryAccount> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_memory_account_by_owner(&self.inner.owner_address)
            .await
            .ok()
            .flatten()
            .map(MemoryAccount::from_row)
    }

    /// AI credit balance for this profile owner (1 MYSO = 1 credit).
    async fn ai_credit_balance(&self, ctx: &Context<'_>) -> Option<AiCreditBalance> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_ai_credit_balance_by_owner(&self.inner.owner_address)
            .await
            .ok()
            .flatten()
            .map(AiCreditBalance::from_row)
    }

    async fn audit_log(
        &self,
        ctx: &Context<'_>,
        filter: Option<AuditLogFilterInput>,
        #[graphql(default = 20)] limit: i32,
        #[graphql(default = 0)] offset: i32,
    ) -> Option<AuditLogConnection> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let filter: AuditLogFilterInput = filter.unwrap_or_default();
        reader
            .list_audit_logs_for_actor(
                &self.inner.owner_address,
                &filter.into(),
                limit as i64,
                offset as i64,
            )
            .await
            .ok()
            .map(|rows| AuditLogConnection {
                entries: rows.into_iter().map(AuditLogEntry::from_row).collect(),
            })
    }

    /// Sub-agents registered under this profile's memory account.
    async fn sub_agents(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SubAgent>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        reader
            .list_sub_agents(
                &self.inner.owner_address,
                active_only.unwrap_or(true),
                limit,
                offset,
            )
            .await
            .ok()
            .map(|result| {
                result
                    .sub_agents
                    .into_iter()
                    .map(SubAgent::from_row)
                    .collect()
            })
    }

    async fn sub_agents_total_count(
        &self,
        ctx: &Context<'_>,
        active_only: Option<bool>,
    ) -> Option<i64> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_sub_agents(&self.inner.owner_address, active_only.unwrap_or(true), 1, 0)
            .await
            .ok()
            .map(|result| result.total_count)
    }

    /// Agentic organizations owned by this profile (max 8 per user on-chain).
    async fn agentic_organizations(
        &self,
        ctx: &Context<'_>,
        org_type: Option<i32>,
        active_only: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<AgenticOrganization>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(8).min(8) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let org_type = org_type.and_then(|v| i16::try_from(v).ok());
        reader
            .list_agentic_organizations_by_owner(
                &self.inner.owner_address,
                org_type,
                active_only.unwrap_or(true),
                limit,
                offset,
            )
            .await
            .ok()
            .map(|result| {
                result
                    .organizations
                    .into_iter()
                    .map(AgenticOrganization::from_row)
                    .collect()
            })
    }

    async fn sub_agent(&self, ctx: &Context<'_>, derived_address: MySoAddress) -> Option<SubAgent> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_sub_agent(&derived_address.to_string())
            .await
            .ok()
            .flatten()
            .map(SubAgent::from_row)
    }

    async fn sub_agent_by_object_id(
        &self,
        ctx: &Context<'_>,
        agent_object_id: String,
    ) -> Option<SubAgent> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_sub_agent_by_object_id(&agent_object_id)
            .await
            .ok()
            .flatten()
            .map(SubAgent::from_row)
    }

    /// MyData records owned by this profile (paginated).
    async fn mydata_records(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<MyDataRecord>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_mydata_records_by_owner(&self.inner.owner_address, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(MyDataRecord::from_row).collect())
    }

    /// Cash-flow P&L for this profile's owner wallet (MYSO base units; not realized/FIFO P&L).
    /// When `windows` is omitted, defaults to 7d, 30d, and all-time.
    async fn pnl(
        &self,
        ctx: &Context<'_>,
        windows: Option<Vec<ProfilePnLWindow>>,
    ) -> Option<Vec<ProfilePnLWindowStats>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let windows = windows.unwrap_or_else(|| {
            vec![
                ProfilePnLWindow::Days7,
                ProfilePnLWindow::Days30,
                ProfilePnLWindow::All,
            ]
        });
        let windows_reader: Vec<myso_indexer_alt_social_reader::ProfilePnLWindow> =
            windows.into_iter().map(Into::into).collect();
        let rows = reader
            .get_profile_pnl(&self.inner.owner_address, &windows_reader)
            .await
            .ok()?;
        Some(rows.into_iter().map(ProfilePnLWindowStats::from).collect())
    }
}

#[derive(Clone)]
pub(crate) struct SocialProofToken {
    inner: SocialProofTokenInfo,
}

impl From<&SocialProofTokenInfo> for SocialProofToken {
    fn from(inner: &SocialProofTokenInfo) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[Object]
impl SocialProofToken {
    async fn pool_id(&self) -> Option<&str> {
        self.inner.pool_id.as_deref()
    }

    async fn token_address(&self) -> Option<&str> {
        self.inner.token_address.as_deref()
    }

    async fn is_active(&self) -> bool {
        self.inner.is_active
    }

    async fn reservation_pool_id(&self) -> Option<&str> {
        self.inner.reservation_pool_id.as_deref()
    }

    async fn reservation_percentage(&self) -> f64 {
        self.inner.reservation_percentage
    }

    async fn reservation_status(&self) -> String {
        match &self.inner.reservation_status {
            ReservationStatus::Active => "active".to_string(),
            ReservationStatus::ThresholdMet => "threshold_met".to_string(),
            ReservationStatus::Inactive => "inactive".to_string(),
            ReservationStatus::None => "none".to_string(),
        }
    }

    async fn total_reserved(&self) -> BigInt {
        BigInt::from(self.inner.total_reserved)
    }

    async fn required_threshold(&self) -> BigInt {
        BigInt::from(self.inner.required_threshold)
    }

    /// Circulating supply in nano-SPT (`10^9` units per display token).
    /// After launch from a reservation pool, initial supply on-chain is `(total_reserved * 10^9) / base_price`
    /// (nano-MYSO reserved × scale ÷ pool `base_price` in MYSO smallest units), before further trades.
    async fn circulating_supply(&self) -> Option<BigInt> {
        self.inner.circulating_supply.map(BigInt::from)
    }

    async fn base_price(&self) -> Option<i64> {
        self.inner.base_price
    }

    async fn current_price(&self) -> Option<i64> {
        self.inner.current_price
    }

    /// Market cap: current price (MYSO smallest units) × circulating supply (nano-SPT).
    async fn market_cap(&self) -> Option<BigInt> {
        self.inner
            .market_cap
            .as_ref()
            .and_then(|s| BigInt::from_str(s).ok())
    }

    /// Price change percentage vs ~24h ago, or vs first indexed price when the pool is younger than 24h.
    async fn price_change_24h(&self) -> Option<f64> {
        self.inner.price_change_24h
    }

    async fn volume_24h(&self) -> Option<i64> {
        self.inner.volume_24h
    }

    async fn creator_earnings(&self) -> Option<i64> {
        self.inner.creator_earnings
    }

    async fn platform_earnings(&self) -> Option<i64> {
        self.inner.platform_earnings
    }

    async fn ecosystem_earnings(&self) -> Option<i64> {
        self.inner.ecosystem_earnings
    }

    async fn owner(&self) -> Option<MySoAddress> {
        self.inner
            .owner
            .as_ref()
            .and_then(|s| MySoAddress::from_str(s).ok())
    }

    async fn created_at(&self) -> Option<i64> {
        self.inner.created_at
    }

    async fn token_type(&self) -> Option<i16> {
        self.inner.token_type
    }

    /// Current reservation holders for this token’s reservation pool.
    async fn reservation_holders(
        &self,
        ctx: &Context<'_>,
        viewer: Option<MySoAddress>,
        prioritize_followed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptReservationHolding>> {
        let pool_id = self.inner.reservation_pool_id.as_deref()?;
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let prioritize = prioritize_followed.unwrap_or(false);
        let rows = reader
            .get_reservation_holdings_for_pool(
                pool_id,
                limit,
                offset,
                viewer_s.as_deref(),
                prioritize,
            )
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(SptReservationHolding::from_row)
                .collect(),
        )
    }

    /// Former reservation holders (withdrawn; latest indexed balance zero per reserver).
    async fn former_reservation_holders(
        &self,
        ctx: &Context<'_>,
        viewer: Option<MySoAddress>,
        prioritize_followed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SptReservationHolding>> {
        let pool_id = self.inner.reservation_pool_id.as_deref()?;
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let prioritize = prioritize_followed.unwrap_or(false);
        let rows = reader
            .get_former_reservation_holdings_for_pool(
                pool_id,
                limit,
                offset,
                viewer_s.as_deref(),
                prioritize,
            )
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(SptReservationHolding::from_row)
                .collect(),
        )
    }
}

#[derive(Clone)]
pub(crate) struct SelectedBadge {
    inner: SelectedBadgeInfo,
}

impl From<&SelectedBadgeInfo> for SelectedBadge {
    fn from(inner: &SelectedBadgeInfo) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[Object]
impl SelectedBadge {
    async fn badge_id(&self) -> &str {
        &self.inner.badge_id
    }

    async fn badge_name(&self) -> &str {
        &self.inner.badge_name
    }

    async fn badge_icon_url(&self) -> Option<&str> {
        self.inner.badge_icon_url.as_deref()
    }

    async fn badge_media_url(&self) -> Option<&str> {
        self.inner.badge_media_url.as_deref()
    }

    async fn platform_id(&self) -> &str {
        &self.inner.platform_id
    }

    async fn badge_type(&self) -> i16 {
        self.inner.badge_type
    }
}

// -----------------------------------------------------------------------------
// ProfileBadge
// -----------------------------------------------------------------------------

#[derive(Clone)]
pub(crate) struct ProfileBadge {
    inner: ProfileBadgeRow,
}

impl ProfileBadge {
    pub(crate) fn from_row(inner: ProfileBadgeRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ProfileBadge {
    /// Badge ID.
    async fn badge_id(&self) -> &str {
        &self.inner.badge_id
    }

    /// Badge name.
    async fn badge_name(&self) -> &str {
        &self.inner.badge_name
    }

    /// Badge description.
    async fn badge_description(&self) -> Option<&str> {
        self.inner.badge_description.as_deref()
    }

    /// Badge media URL.
    async fn badge_media_url(&self) -> Option<&str> {
        self.inner.badge_media_url.as_deref()
    }

    /// Badge icon URL.
    async fn badge_icon_url(&self) -> Option<&str> {
        self.inner.badge_icon_url.as_deref()
    }

    /// Platform ID.
    async fn platform_id(&self) -> &str {
        &self.inner.platform_id
    }

    /// Address that assigned the badge.
    async fn assigned_by(&self) -> &str {
        &self.inner.assigned_by
    }

    /// Profile of the user who assigned the badge.
    async fn assigned_by_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.assigned_by).await
    }

    /// When the badge was assigned (epoch ms).
    async fn assigned_at(&self) -> i64 {
        self.inner.assigned_at
    }

    /// Badge type.
    async fn badge_type(&self) -> i16 {
        self.inner.badge_type
    }
}
