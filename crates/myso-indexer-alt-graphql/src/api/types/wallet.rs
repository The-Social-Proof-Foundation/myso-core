// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;

use crate::api::scalars::id::Id;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::blocked::{BlockedPlatformSummary, BlockedProfileSummary};
use crate::api::types::mydata::MyDataRecord;
use crate::api::types::platform::{PlatformMembershipPage, PlatformMembershipSummary};
use crate::api::types::pnl::{ProfilePnLWindow, ProfilePnLWindowStats};
use crate::api::types::profile::{Profile, ProfileBadge, SelectedBadge};
use crate::api::types::profile_subscription::ProfileSubscription;
use crate::api::types::profile_summary::ProfileSummary;

#[derive(Clone)]
pub(crate) struct Wallet {
    address: String,
}

impl Wallet {
    pub(crate) fn new(address: MySoAddress) -> Self {
        Self {
            address: address.to_string(),
        }
    }
}

#[Object]
impl Wallet {
    async fn id(&self) -> Id {
        let addr = MySoAddress::from_str(&self.address)
            .map(Into::into)
            .unwrap_or(myso_types::base_types::MySoAddress::ZERO);
        Id::Address(addr)
    }

    async fn address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn profile(&self, ctx: &Context<'_>) -> Option<Profile> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_profile_or_wallet_by_address(&self.address)
            .await
            .ok()
            .and_then(Profile::from_response)
    }

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
            .get_wallet_platform_badges(&self.address, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(ProfileBadge::from_row).collect())
    }

    async fn selected_badge(&self, ctx: &Context<'_>) -> Option<SelectedBadge> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_wallet_selected_platform_badge(&self.address)
            .await
            .ok()??;
        Some(SelectedBadge::from(
            &myso_indexer_alt_social_reader::SelectedBadgeInfo {
                badge_id: row.badge_id,
                badge_name: row.badge_name,
                badge_icon_url: row.badge_icon_url,
                badge_media_url: row.badge_media_url,
                platform_id: row.platform_id,
                badge_type: row.badge_type,
            },
        ))
    }

    async fn selected_badge_id(&self, ctx: &Context<'_>) -> Option<String> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_wallet_selected_platform_badge(&self.address)
            .await
            .ok()
            .flatten()
            .map(|r| r.badge_id)
    }

    async fn selected_ecosystem_badge_id(&self, ctx: &Context<'_>) -> Option<String> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_resolved_selected_ecosystem_badge_id(&self.address)
            .await
            .ok()
            .flatten()
    }

    async fn selected_ecosystem_badge(&self, ctx: &Context<'_>) -> Option<SelectedBadge> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let id = reader
            .get_resolved_selected_ecosystem_badge_id(&self.address)
            .await
            .ok()
            .flatten()?;
        let rows = reader.get_profile_badges(&self.address, 50, 0).await.ok()?;
        rows.into_iter().find(|b| b.badge_id == id).map(|row| {
            SelectedBadge::from(&myso_indexer_alt_social_reader::SelectedBadgeInfo {
                badge_id: row.badge_id,
                badge_name: row.badge_name,
                badge_icon_url: row.badge_icon_url,
                badge_media_url: row.badge_media_url,
                platform_id: row.platform_id,
                badge_type: row.badge_type,
            })
        })
    }

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
            .get_profile_platform_memberships(&self.address, limit, offset)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PlatformMembershipSummary::from_row)
                .collect(),
        )
    }

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
            .count_profile_platform_memberships(&self.address)
            .await
            .ok()?;
        let rows = reader
            .get_profile_platform_memberships(&self.address, limit, offset)
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

    async fn platform_memberships_total(&self, ctx: &Context<'_>) -> Option<i64> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .count_profile_platform_memberships(&self.address)
            .await
            .ok()
    }

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
            .get_profile_pnl(&self.address, &windows_reader)
            .await
            .ok()?;
        Some(rows.into_iter().map(ProfilePnLWindowStats::from).collect())
    }

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
        reader
            .get_followers(&self.address, limit, offset, viewer_s.as_deref())
            .await
            .ok()
            .map(|rows| rows.into_iter().map(ProfileSummary::from_row).collect())
    }

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
        reader
            .get_following(&self.address, limit, offset, viewer_s.as_deref())
            .await
            .ok()
            .map(|rows| rows.into_iter().map(ProfileSummary::from_row).collect())
    }

    async fn followers_count(&self, ctx: &Context<'_>) -> Option<i32> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_profile_or_wallet_by_address(&self.address)
            .await
            .ok()
            .map(|r| r.followers_count)
    }

    async fn following_count(&self, ctx: &Context<'_>) -> Option<i32> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_profile_or_wallet_by_address(&self.address)
            .await
            .ok()
            .map(|r| r.following_count)
    }

    async fn blocked_count(&self, ctx: &Context<'_>) -> Option<i32> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_profile_or_wallet_by_address(&self.address)
            .await
            .ok()
            .map(|r| r.blocked_count)
    }

    async fn mutual_connections(
        &self,
        ctx: &Context<'_>,
        viewer: MySoAddress,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<ProfileSummary>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        reader
            .get_mutual_connections(&self.address, &viewer.to_string(), limit, offset)
            .await
            .ok()
            .map(|rows| rows.into_iter().map(ProfileSummary::from_row).collect())
    }

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
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        reader
            .get_follow_recommendations(
                &self.address,
                limit,
                offset,
                viewer_s.as_deref(),
                myso_indexer_alt_social_reader::MAX_MUTUAL_CONNECTIONS_LIMIT,
            )
            .await
            .ok()
            .map(|(rows, _)| rows.into_iter().map(ProfileSummary::from_row).collect())
    }

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
            .list_mydata_records_by_owner(&self.address, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(MyDataRecord::from_row).collect())
    }

    async fn subscriptions(
        &self,
        ctx: &Context<'_>,
        service_id: Option<async_graphql::ID>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<ProfileSubscription>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100);
        let offset = offset.unwrap_or(0);
        let service_filter = service_id.as_ref().map(|id| id.to_string());
        reader
            .list_active_profile_subscriptions_by_subscriber(
                &self.address,
                service_filter.as_deref(),
                limit,
                offset,
            )
            .await
            .ok()
            .map(|rows| {
                rows.into_iter()
                    .map(ProfileSubscription::from_row)
                    .collect()
            })
    }

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
        reader
            .get_blocked_profiles(&self.address, limit, offset)
            .await
            .ok()
            .map(|rows| {
                rows.into_iter()
                    .map(BlockedProfileSummary::from_row)
                    .collect()
            })
    }

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
        reader
            .get_blocked_platforms(&self.address, limit, offset)
            .await
            .ok()
            .map(|rows| {
                rows.into_iter()
                    .map(BlockedPlatformSummary::from_row)
                    .collect()
            })
    }
}
