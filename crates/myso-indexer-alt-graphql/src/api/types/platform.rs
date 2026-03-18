// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use chrono::DateTime;
use myso_indexer_alt_social_reader::PlatformRow as DbPlatform;
use myso_indexer_alt_social_schema::models::{
    PlatformMemberRow, PlatformModeratorRow, ProfilePlatformMembershipRow,
};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::id::Id;
use crate::api::scalars::json::Json;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::blocked::PlatformBlockedProfileSummary;
use crate::api::types::profile_summary::ProfileSummary;

fn to_iso8601_utc(dt: chrono::NaiveDateTime) -> String {
    DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

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

    /// When the platform was created (ISO 8601).
    async fn created_at(&self) -> String {
        self.inner
            .created_at
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string()
    }

    /// When the platform was last updated (ISO 8601).
    async fn updated_at(&self) -> String {
        self.inner
            .updated_at
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string()
    }

    /// Treasury balance.
    async fn treasury(&self) -> Option<i64> {
        self.inner.treasury
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

    /// Min on-chain age in days.
    async fn min_on_chain_age_days(&self) -> Option<i64> {
        self.inner.min_on_chain_age_days
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
        Some(rows.into_iter().map(PlatformBlockedProfileSummary::from_row).collect())
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
        Some(rows.into_iter().map(PlatformMemberSummary::from_row).collect())
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
        Some(rows.into_iter().map(PlatformModeratorSummary::from_row).collect())
    }
}

#[derive(Clone)]
pub(crate) struct PlatformMembershipSummary {
    pub platform_id: String,
    pub name: String,
    pub is_approved: bool,
    pub joined_at: chrono::NaiveDateTime,
}

impl PlatformMembershipSummary {
    pub(crate) fn from_row(row: ProfilePlatformMembershipRow) -> Self {
        Self {
            platform_id: row.platform_id,
            name: row.name,
            is_approved: row.is_approved,
            joined_at: row.joined_at,
        }
    }
}

#[Object]
impl PlatformMembershipSummary {
    async fn platform_id(&self) -> &str {
        &self.platform_id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn is_approved(&self) -> bool {
        self.is_approved
    }

    async fn joined_at(&self) -> String {
        to_iso8601_utc(self.joined_at)
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

    async fn joined_at(&self) -> String {
        to_iso8601_utc(self.joined_at)
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

    async fn created_at(&self) -> String {
        to_iso8601_utc(self.created_at)
    }
}
