// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::PlatformRow as DbPlatform;
use myso_indexer_alt_social_reader::SocialPgReader;
use myso_indexer_alt_social_schema::models::{
    PlatformMemberRow, PlatformModeratorRow, ProfilePlatformMembershipRow,
};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::id::Id;
use crate::api::scalars::json::Json;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::blocked::PlatformBlockedProfileSummary;
use crate::api::types::profile_summary::ProfileSummary;
use crate::error::RpcError;

fn platform_status_to_text(status: i16) -> &'static str {
    match status {
        0 => "Development",
        1 => "Alpha",
        2 => "Beta",
        3 => "Live",
        4 => "Maintenance",
        5 => "Sunset",
        6 => "Shutdown",
        _ => "Unknown",
    }
}

#[derive(Clone)]
pub(crate) struct Platform {
    inner: DbPlatform,
}

impl Platform {
    pub(crate) fn from_db(inner: DbPlatform) -> Self {
        Self { inner }
    }
}

/// Resolve a platform by on-chain platform id. Returns None when social DB is not configured.
pub(crate) async fn resolve_platform_by_id(
    ctx: &Context<'_>,
    platform_id: &str,
) -> Option<Platform> {
    let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
    let reader = reader_opt.as_ref().as_ref()?;
    let row = reader.get_platform_by_id(platform_id).await.ok()??;
    Some(Platform::from_db(row))
}

#[Object]
impl Platform {
    /// The platform's globally unique identifier.
    pub async fn id(&self) -> Id {
        Id::Platform(self.inner.platform_id.clone())
    }

    /// The platform ID.
    async fn platform_id(&self) -> &str {
        &self.inner.platform_id
    }

    /// The platform name.
    async fn name(&self) -> &str {
        &self.inner.name
    }

    /// The platform tagline.
    async fn tagline(&self) -> &str {
        &self.inner.tagline
    }

    /// The platform description.
    async fn description(&self) -> Option<&str> {
        self.inner.description.as_deref()
    }

    /// URL to the platform logo.
    async fn logo(&self) -> Option<&str> {
        self.inner.logo.as_deref()
    }

    /// URL to the platform cover photo.
    async fn cover_photo(&self) -> Option<&str> {
        self.inner.cover_photo.as_deref()
    }

    /// Screenshot and video preview URLs (JSON array).
    async fn media_previews(&self) -> Option<Json> {
        self.inner
            .media_previews
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// The developer's wallet address.
    async fn developer_address(&self) -> &str {
        &self.inner.developer_address
    }

    /// Profile of the platform developer.
    async fn developer_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.developer_address).await
    }

    /// Whether the platform is approved.
    async fn is_approved(&self) -> bool {
        self.inner.is_approved
    }

    /// Primary category.
    async fn primary_category(&self) -> &str {
        &self.inner.primary_category
    }

    /// Secondary category.
    async fn secondary_category(&self) -> Option<&str> {
        self.inner.secondary_category.as_deref()
    }

    /// Terms of service.
    async fn terms_of_service(&self) -> Option<&str> {
        self.inner.terms_of_service.as_deref()
    }

    /// Privacy policy.
    async fn privacy_policy(&self) -> Option<&str> {
        self.inner.privacy_policy.as_deref()
    }

    /// Links (JSON).
    async fn links(&self) -> Option<Json> {
        self.inner
            .links
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// Platform names (e.g. Twitter, Instagram) as JSON array.
    async fn platform_names(&self) -> Option<Json> {
        self.inner
            .platform_names
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// Platform status as text (Development, Alpha, Beta, Live, etc.).
    async fn status_text(&self) -> &str {
        platform_status_to_text(self.inner.status)
    }

    /// Release date.
    async fn release_date(&self) -> Option<&str> {
        self.inner.release_date.as_deref()
    }

    /// Shutdown date.
    async fn shutdown_date(&self) -> Option<&str> {
        self.inner.shutdown_date.as_deref()
    }

    /// When the platform was created (epoch milliseconds).
    async fn created_at(&self) -> i64 {
        self.inner.created_at.and_utc().timestamp_millis()
    }

    /// When the platform was last updated (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at.and_utc().timestamp_millis()
    }

    /// Whether the platform wants DAO governance.
    async fn wants_dao_governance(&self) -> Option<bool> {
        self.inner.wants_dao_governance
    }

    /// Governance registry ID.
    async fn governance_registry_id(&self) -> Option<&str> {
        self.inner.governance_registry_id.as_deref()
    }

    /// Delegate count for governance.
    async fn delegate_count(&self) -> Option<i64> {
        self.inner.delegate_count
    }

    /// Delegate term epochs.
    async fn delegate_term_epochs(&self) -> Option<i64> {
        self.inner.delegate_term_epochs
    }

    /// Max votes per user.
    async fn max_votes_per_user(&self) -> Option<i64> {
        self.inner.max_votes_per_user
    }

    /// Proposal submission cost.
    async fn proposal_submission_cost(&self) -> Option<i64> {
        self.inner.proposal_submission_cost
    }

    /// Quadratic base cost.
    async fn quadratic_base_cost(&self) -> Option<i64> {
        self.inner.quadratic_base_cost
    }

    /// Quorum votes.
    async fn quorum_votes(&self) -> Option<i64> {
        self.inner.quorum_votes
    }

    /// Voting period epochs.
    async fn voting_period_epochs(&self) -> Option<i64> {
        self.inner.voting_period_epochs
    }

    /// Wallets blocked by this platform (paginated).
    async fn blocked_profiles(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PlatformBlockedProfileSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_platform_blocked_profiles(&self.inner.platform_id, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PlatformBlockedProfileSummary::from_row)
                .collect(),
        )
    }

    /// Members of this platform (paginated).
    async fn members(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PlatformMemberSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_platform_members(&self.inner.platform_id, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PlatformMemberSummary::from_row)
                .collect(),
        )
    }

    /// Moderators of this platform (paginated).
    async fn moderators(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PlatformModeratorSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_platform_moderators(&self.inner.platform_id, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PlatformModeratorSummary::from_row)
                .collect(),
        )
    }

    /// Member, platform-block, and moderator flags for a wallet (single DB round-trip).
    async fn user_access(
        &self,
        ctx: &Context<'_>,
        user: MySoAddress,
    ) -> Option<Result<PlatformUserAccess, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_platform_user_access(&self.inner.platform_id, &user.to_string())
                .await
                .map_err(Into::into)
                .map(PlatformUserAccess::from_row),
        )
    }
}

#[derive(Clone)]
pub(crate) struct PlatformUserAccess {
    is_member: bool,
    is_blocked: bool,
    is_moderator: bool,
}

impl PlatformUserAccess {
    pub(crate) fn from_row(row: myso_indexer_alt_social_reader::PlatformUserAccessRow) -> Self {
        Self {
            is_member: row.is_member,
            is_blocked: row.is_blocked,
            is_moderator: row.is_moderator,
        }
    }
}

#[Object]
impl PlatformUserAccess {
    /// Whether the wallet is a member of this platform.
    async fn is_member(&self) -> bool {
        self.is_member
    }

    /// Whether the platform has blocked this wallet.
    async fn is_blocked(&self) -> bool {
        self.is_blocked
    }

    /// Whether the wallet is a moderator of this platform.
    async fn is_moderator(&self) -> bool {
        self.is_moderator
    }
}

#[derive(Clone)]
pub(crate) struct PlatformMembershipSummary {
    row: ProfilePlatformMembershipRow,
}

impl PlatformMembershipSummary {
    pub(crate) fn from_row(row: ProfilePlatformMembershipRow) -> Self {
        Self { row }
    }
}

#[Object]
impl PlatformMembershipSummary {
    /// `platform_memberships` row id (stable for this wallet+platform join record).
    async fn membership_id(&self) -> i32 {
        self.row.membership_id
    }

    /// Surrogate key from the `platforms` table.
    async fn platform_db_id(&self) -> i32 {
        self.row.platform_db_id
    }

    async fn platform_id(&self) -> &str {
        &self.row.platform_id
    }

    async fn name(&self) -> &str {
        &self.row.name
    }

    async fn tagline(&self) -> &str {
        &self.row.tagline
    }

    async fn description(&self) -> Option<&str> {
        self.row.description.as_deref()
    }

    async fn logo(&self) -> Option<&str> {
        self.row.logo.as_deref()
    }

    async fn cover_photo(&self) -> Option<&str> {
        self.row.cover_photo.as_deref()
    }

    async fn media_previews(&self) -> Option<Json> {
        self.row
            .media_previews
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    async fn developer_address(&self) -> &str {
        &self.row.developer_address
    }

    /// Profile of the platform developer (optional; loads via social reader when selected).
    async fn developer_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.row.developer_address).await
    }

    async fn terms_of_service(&self) -> Option<&str> {
        self.row.terms_of_service.as_deref()
    }

    async fn privacy_policy(&self) -> Option<&str> {
        self.row.privacy_policy.as_deref()
    }

    /// Alternate names / labels (JSON array in the DB `platforms` column).
    async fn platform_names(&self) -> Option<Json> {
        self.row
            .platform_names
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// External links (JSON array in DB).
    async fn links(&self) -> Option<Json> {
        self.row
            .links
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// Platform status code (indexer contract): 0=Development, 1=Alpha, 2=Beta, 3=Live, 4=Maintenance, 5=Sunset, 6=Shutdown.
    async fn status(&self) -> i32 {
        self.row.status as i32
    }

    async fn status_text(&self) -> &str {
        platform_status_to_text(self.row.status)
    }

    async fn release_date(&self) -> Option<&str> {
        self.row.release_date.as_deref()
    }

    async fn shutdown_date(&self) -> Option<&str> {
        self.row.shutdown_date.as_deref()
    }

    /// When the platform row was created (epoch ms).
    async fn created_at(&self) -> i64 {
        self.row.platform_created_at.and_utc().timestamp_millis()
    }

    /// When the platform row was last updated (epoch ms).
    async fn updated_at(&self) -> i64 {
        self.row.platform_updated_at.and_utc().timestamp_millis()
    }

    async fn is_approved(&self) -> bool {
        self.row.is_approved
    }

    async fn approval_changed_at(&self) -> Option<i64> {
        self.row
            .approval_changed_at
            .map(|t| t.and_utc().timestamp_millis())
    }

    async fn approved_by(&self) -> Option<&str> {
        self.row.approved_by.as_deref()
    }

    /// When this wallet joined the platform (epoch ms).
    async fn joined_at(&self) -> i64 {
        self.row.joined_at.and_utc().timestamp_millis()
    }

    async fn wants_dao_governance(&self) -> Option<bool> {
        self.row.wants_dao_governance
    }

    async fn governance_registry_id(&self) -> Option<&str> {
        self.row.governance_registry_id.as_deref()
    }

    async fn delegate_count(&self) -> Option<i64> {
        self.row.delegate_count
    }

    async fn delegate_term_epochs(&self) -> Option<i64> {
        self.row.delegate_term_epochs
    }

    async fn max_votes_per_user(&self) -> Option<i64> {
        self.row.max_votes_per_user
    }

    async fn proposal_submission_cost(&self) -> Option<i64> {
        self.row.proposal_submission_cost
    }

    async fn quadratic_base_cost(&self) -> Option<i64> {
        self.row.quadratic_base_cost
    }

    async fn quorum_votes(&self) -> Option<i64> {
        self.row.quorum_votes
    }

    async fn voting_period_epochs(&self) -> Option<i64> {
        self.row.voting_period_epochs
    }

    /// On-chain treasury is not mirrored in Postgres; read the platform object via RPC when needed.
    async fn treasury_address(&self) -> Option<String> {
        None
    }

    async fn version(&self) -> Option<i64> {
        self.row.version
    }

    async fn primary_category(&self) -> &str {
        &self.row.primary_category
    }

    async fn secondary_category(&self) -> Option<&str> {
        self.row.secondary_category.as_deref()
    }

    /// Present when the platform is soft-deleted (epoch ms).
    async fn deleted_at(&self) -> Option<i64> {
        self.row.deleted_at.map(|t| t.and_utc().timestamp_millis())
    }

    async fn moderator_count(&self) -> i64 {
        self.row.moderator_count
    }

    async fn blocked_profiles_count(&self) -> i64 {
        self.row.blocked_profiles_count
    }
}

/// Paginated platform memberships for a profile (offset/limit + total count).
#[derive(Clone)]
pub(crate) struct PlatformMembershipPage {
    pub(crate) items: Vec<PlatformMembershipSummary>,
    pub(crate) total_count: i64,
    pub(crate) limit: i64,
    pub(crate) offset: i64,
    total_pages: i64,
}

impl PlatformMembershipPage {
    pub(crate) fn new(
        items: Vec<PlatformMembershipSummary>,
        total_count: i64,
        limit: i64,
        offset: i64,
    ) -> Self {
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
impl PlatformMembershipPage {
    async fn items(&self) -> Vec<PlatformMembershipSummary> {
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

    /// Total pages for this `limit` (0 when there are no memberships).
    async fn total_pages(&self) -> i64 {
        self.total_pages
    }
}

#[derive(Clone)]
pub(crate) struct PlatformMemberSummary {
    pub wallet_address: String,
    pub joined_at: chrono::NaiveDateTime,
}

impl PlatformMemberSummary {
    pub(crate) fn from_row(row: PlatformMemberRow) -> Self {
        Self {
            wallet_address: row.wallet_address,
            joined_at: row.joined_at,
        }
    }
}

#[Object]
impl PlatformMemberSummary {
    async fn wallet_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.wallet_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the platform member.
    async fn profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.wallet_address).await
    }

    async fn joined_at(&self) -> i64 {
        self.joined_at.and_utc().timestamp_millis()
    }
}

#[derive(Clone)]
pub(crate) struct PlatformModeratorSummary {
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: chrono::NaiveDateTime,
}

impl PlatformModeratorSummary {
    pub(crate) fn from_row(row: PlatformModeratorRow) -> Self {
        Self {
            moderator_address: row.moderator_address,
            added_by: row.added_by,
            created_at: row.created_at,
        }
    }
}

#[Object]
impl PlatformModeratorSummary {
    async fn moderator_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.moderator_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the moderator.
    async fn moderator_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.moderator_address).await
    }

    async fn added_by(&self) -> &str {
        &self.added_by
    }

    /// Profile of the user who added the moderator.
    async fn added_by_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.added_by).await
    }

    async fn created_at(&self) -> i64 {
        self.created_at.and_utc().timestamp_millis()
    }
}
