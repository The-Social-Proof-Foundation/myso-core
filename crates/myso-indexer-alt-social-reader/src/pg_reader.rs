// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Context;
use anyhow::bail;
use async_graphql::dataloader::DataLoader;
use myso_indexer_alt_metrics::db::DbConnectionStatsCollector;
use prometheus::Registry;
use url::Url;

use myso_pg_db as db;

use crate::governance::{
    get_anonymous_voting_trends, get_delegate_by_address, get_delegate_proposals,
    get_delegate_ratings, get_governance_registry_by_platform_id, get_governance_registry_by_type,
    get_governance_stats_by_registry_type, get_proposal_anonymous_stats,
    get_proposal_anonymous_votes, get_proposal_by_id, get_proposal_community_votes,
    get_proposal_community_votes_count, get_proposal_decryption_failures,
    get_proposal_delegate_votes, get_proposal_reward_distributions, list_delegates,
    list_governance_events, list_governance_registries, list_nominated_delegates, list_proposals,
};
use crate::insurance::{
    get_insurance_config, get_insurance_policy, get_insurance_vault, get_insurance_vault_exposures,
    list_insurance_market_policies, list_insurance_policies, list_insurance_policies_by_insured,
    list_insurance_vault_transactions, list_insurance_vaults,
};
use crate::metrics::DbReaderMetrics;
use crate::mydata::{
    get_mydata_access_analytics, get_mydata_access_logs, get_mydata_config, get_mydata_purchases,
    get_mydata_record, get_mydata_revenue, get_mydata_revenue_timeline, get_mydata_stats,
    get_mydata_subscriptions, get_popular_mydata, list_mydata, list_mydata_purchases_by_buyer,
    list_mydata_records_by_owner,
};
use crate::platform::PlatformRow;
use crate::platform::{
    get_platform_blocked_profiles, get_platform_members, get_platform_moderators,
    get_platform_user_access,
};
use crate::pnl::{ProfilePnLWindow, ProfilePnLWindowResult, get_profile_pnl_for_windows};
use crate::poc::{
    get_poc_analysis_for_post, get_poc_badges_for_post, get_poc_configuration,
    get_poc_disputes_for_post, get_post_revenue_redirections,
};
use crate::post::PostRow;
use crate::profile::get_profile_badges;
use crate::profile::get_profile_by_address;
use crate::profile::get_profile_or_wallet_by_address;
use crate::profile::get_profiles;
use crate::promotion::{
    get_promotion, get_promotion_by_post_id, get_promotion_hourly, get_promotion_stats,
    get_promotion_time_series, get_promotion_views, get_promotion_views_count, get_spending_trends,
    get_top_performing_promotions, list_promoted_posts,
};
use crate::revenue::get_platform_revenue_summary;
use crate::social_graph::{
    FollowSortBy, ProfileSummaryRow, ViewerSocialContext, batch_viewer_social_context,
    check_following, check_platform_blocked, check_profile_blocked,
    count_profile_platform_memberships, get_blocked_platforms, get_blocked_profiles, get_followers,
    get_following, get_profile_platform_memberships, resolve_profile_address,
};
use crate::spot::{
    get_spot_config, get_spot_record, get_spot_resolution, list_spot_bet_withdrawals,
    list_spot_bets, list_spot_payouts, list_spot_refunds,
};
use crate::spt::SptReservationVolumeInterval;
use crate::spt::{
    SptTransactionsWithViewer, get_former_reservation_holdings_for_pool,
    get_reservation_holdings_for_pool, get_reservation_pool_id_for_associated_id,
    get_spt_exchange_config, get_spt_holdings_by_holder, get_spt_holdings_by_pool, get_spt_pool,
    get_spt_pool_id_for_profile, get_spt_price_history, get_spt_reservation_volume_history,
    get_spt_transactions, get_user_reservation_holdings, list_spt_pools,
};
use crate::vesting::{get_vesting_leaderboard, get_vesting_wallet, list_vesting_wallets};

pub use myso_indexer_alt_social_schema::models::Profile;

/// Reader for the social postgres database. Connects to the database populated by
/// myso-indexer-alt-social and provides query methods for profiles, posts, platforms,
/// and social graph data.
#[derive(Clone)]
pub struct SocialPgReader {
    db: Option<db::Db>,
    metrics: Arc<DbReaderMetrics>,
}

impl SocialPgReader {
    /// Create a new social database reader. If `database_url` is `None`, the reader
    /// will not accept any connection requests (they will all fail).
    ///
    /// `prefix` is used to prefix the metrics collected by this reader.
    pub async fn new(
        prefix: Option<&str>,
        database_url: Option<Url>,
        db_args: db::DbArgs,
        registry: &Registry,
    ) -> anyhow::Result<Self> {
        let db = if let Some(database_url) = database_url {
            let db = db::Db::for_read(database_url, db_args)
                .await
                .context("Failed to create social database for reading")?;

            registry
                .register(Box::new(DbConnectionStatsCollector::new(
                    prefix,
                    db.clone(),
                )))
                .context("Failed to register social database connection stats collector")?;

            Some(db)
        } else {
            None
        };

        let metrics = DbReaderMetrics::new(prefix, registry);

        Ok(Self { db, metrics })
    }

    /// Create a data loader backed by this reader.
    pub fn as_data_loader(&self) -> DataLoader<Self> {
        DataLoader::new(self.clone(), tokio::spawn)
    }

    /// Check if this reader has a database available.
    pub fn has_database(&self) -> bool {
        self.db.is_some()
    }

    /// Acquire a connection to the database.
    pub async fn connect(&self) -> anyhow::Result<db::Connection<'_>> {
        let Some(db) = &self.db else {
            bail!("No social database to connect to");
        };

        db.connect()
            .await
            .context("Failed to connect to social database")
    }

    /// Get a profile by owner address.
    pub async fn get_profile_by_address(&self, address: &str) -> anyhow::Result<Option<Profile>> {
        let mut conn = self.connect().await?;
        get_profile_by_address(&mut conn, address, &self.metrics).await
    }

    /// Get profile by address, or wallet-only data when no profile exists.
    pub async fn get_profile_or_wallet_by_address(
        &self,
        address: &str,
    ) -> anyhow::Result<crate::profile::ProfileByAddressResponse> {
        let mut conn = self.connect().await?;
        get_profile_or_wallet_by_address(&mut conn, address, &self.metrics).await
    }

    /// Get profile summary for a single address. Supports both profile and wallet-only addresses.
    /// - Profile exists: returns profile data (username, display_name, photo, etc.) + followers_count, following_count
    /// - Wallet only: returns address + followers_count, following_count from wallet_social_graph
    pub async fn get_profile_summary(&self, address: &str) -> anyhow::Result<ProfileSummaryRow> {
        let response = self.get_profile_or_wallet_by_address(address).await?;
        Ok(ProfileSummaryRow {
            owner_address: response.owner_address,
            username: response.username,
            display_name: response.display_name,
            profile_photo: response.profile_photo,
            bio: response.bio,
            selected_badge_id: response.selected_badge_id,
            social_proof_token_address: response.social_proof_token_address,
            reservation_pool_address: response.reservation_pool_address,
            followers_count: Some(response.followers_count),
            following_count: Some(response.following_count),
            post_count: Some(response.post_count),
            blocked_count: Some(response.blocked_count),
            is_following: None,
            follows_viewer: None,
            blocked_by_viewer: None,
            blocked_by_subject: None,
        })
    }

    /// Get enriched profile summary (badge, SPT, reservation %) for a single address.
    pub async fn get_profile_summary_enriched(
        &self,
        address: &str,
    ) -> anyhow::Result<Option<crate::profile::UniversalUserResult>> {
        let mut conn = self.connect().await?;
        crate::profile::get_profile_summary_enriched(&mut conn, address, &self.metrics).await
    }

    /// Get profiles with pagination.
    pub async fn get_profiles(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<Profile>> {
        let mut conn = self.connect().await?;
        get_profiles(&mut conn, limit, offset, &self.metrics).await
    }

    /// Get a post by ID.
    pub async fn get_post_by_id(&self, post_id: &str) -> anyhow::Result<Option<PostRow>> {
        let mut conn = self.connect().await?;
        crate::post::get_post_by_id(&mut conn, post_id, &self.metrics).await
    }

    /// List posts with optional filters.
    pub async fn list_posts(
        &self,
        owner: Option<&str>,
        post_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<PostRow>> {
        let mut conn = self.connect().await?;
        crate::post::list_posts(&mut conn, owner, post_type, limit, offset, &self.metrics).await
    }

    /// Posts for a profile (owner or profile_id), same scope as REST profile posts.
    pub async fn list_posts_for_profile(
        &self,
        owner_address: &str,
        profile_id: Option<&str>,
        post_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::post::PostRow>> {
        let mut conn = self.connect().await?;
        crate::post::list_posts_for_profile(
            &mut conn,
            owner_address,
            profile_id,
            post_type,
            limit,
            offset,
            &self.metrics,
        )
        .await
    }

    /// Get a platform by ID.
    pub async fn get_platform_by_id(
        &self,
        platform_id: &str,
    ) -> anyhow::Result<Option<PlatformRow>> {
        let mut conn = self.connect().await?;
        crate::platform::get_platform_by_id(&mut conn, platform_id, &self.metrics).await
    }

    /// Get a platform by governance registry ID (first platform that references the registry).
    pub async fn get_platform_by_registry_id(
        &self,
        registry_id: &str,
    ) -> anyhow::Result<Option<PlatformRow>> {
        let mut conn = self.connect().await?;
        crate::platform::get_platform_by_registry_id(&mut conn, registry_id, &self.metrics).await
    }

    /// List platforms with optional approved filter.
    pub async fn list_platforms(
        &self,
        approved_only: bool,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<PlatformRow>> {
        let mut conn = self.connect().await?;
        crate::platform::list_platforms(&mut conn, approved_only, limit, offset, &self.metrics)
            .await
    }

    /// Get profile badges by owner address.
    pub async fn get_profile_badges(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::profile::ProfileBadgeRow>> {
        let mut conn = self.connect().await?;
        get_profile_badges(&mut conn, address, limit, offset, &self.metrics).await
    }

    /// Get followers of a profile (by owner address).
    pub async fn get_followers(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
        viewer: Option<&str>,
    ) -> anyhow::Result<Vec<crate::social_graph::ProfileSummaryRow>> {
        let mut conn = self.connect().await?;
        let (rows, _) = get_followers(
            &mut conn,
            address,
            FollowSortBy::Latest,
            None,
            viewer,
            limit,
            offset,
            &self.metrics,
        )
        .await?;
        Ok(rows)
    }

    /// Get accounts that a profile follows (by owner address).
    pub async fn get_following(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
        viewer: Option<&str>,
    ) -> anyhow::Result<Vec<crate::social_graph::ProfileSummaryRow>> {
        let mut conn = self.connect().await?;
        get_following(&mut conn, address, viewer, limit, offset, &self.metrics).await
    }

    /// Check if follower follows following.
    pub async fn check_following(
        &self,
        follower_address: &str,
        following_address: &str,
    ) -> anyhow::Result<bool> {
        let mut conn = self.connect().await?;
        check_following(
            &mut conn,
            follower_address,
            following_address,
            &self.metrics,
        )
        .await
    }

    /// Batch follow/block edges from `viewer` to each subject address (wallet or profile id).
    pub async fn batch_viewer_social_context_for_addresses(
        &self,
        subject_addresses: &[String],
        viewer: &str,
    ) -> anyhow::Result<std::collections::HashMap<String, ViewerSocialContext>> {
        let mut conn = self.connect().await?;
        let (v_pid, v_owner) = resolve_profile_address(&mut conn, viewer).await?;
        batch_viewer_social_context(&mut conn, subject_addresses, &v_pid, &v_owner).await
    }

    /// Get profiles blocked by this user.
    pub async fn get_blocked_profiles(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::social_graph::BlockedProfileRow>> {
        let mut conn = self.connect().await?;
        get_blocked_profiles(&mut conn, address, limit, offset, &self.metrics).await
    }

    /// Get platforms that have blocked this profile.
    pub async fn get_blocked_platforms(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::social_graph::BlockedPlatformRow>> {
        let mut conn = self.connect().await?;
        get_blocked_platforms(&mut conn, address, limit, offset, &self.metrics).await
    }

    /// Get profiles blocked by this platform.
    pub async fn get_platform_blocked_profiles(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::platform::PlatformBlockedProfileRow>> {
        let mut conn = self.connect().await?;
        get_platform_blocked_profiles(&mut conn, platform_id, limit, offset, &self.metrics).await
    }

    /// Get platforms this profile has joined (paginated).
    pub async fn get_profile_platform_memberships(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::ProfilePlatformMembershipRow>>
    {
        let mut conn = self.connect().await?;
        get_profile_platform_memberships(&mut conn, address, limit, offset, &self.metrics).await
    }

    /// Count platforms this profile has joined (same join/filter as [`get_profile_platform_memberships`]).
    pub async fn count_profile_platform_memberships(&self, address: &str) -> anyhow::Result<i64> {
        let mut conn = self.connect().await?;
        count_profile_platform_memberships(&mut conn, address, &self.metrics).await
    }

    /// Get members of a platform (paginated).
    pub async fn get_platform_members(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::PlatformMemberRow>> {
        let mut conn = self.connect().await?;
        get_platform_members(&mut conn, platform_id, limit, offset, &self.metrics).await
    }

    /// Get moderators of a platform (paginated).
    pub async fn get_platform_moderators(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::PlatformModeratorRow>> {
        let mut conn = self.connect().await?;
        get_platform_moderators(&mut conn, platform_id, limit, offset, &self.metrics).await
    }

    /// Check if blocker has blocked blocked.
    pub async fn check_profile_blocked(
        &self,
        blocker: &str,
        blocked: &str,
    ) -> anyhow::Result<bool> {
        let mut conn = self.connect().await?;
        check_profile_blocked(&mut conn, blocker, blocked, &self.metrics).await
    }

    /// Check if platform has blocked this profile.
    pub async fn check_platform_blocked(
        &self,
        profile_address: &str,
        platform_id: &str,
    ) -> anyhow::Result<bool> {
        let mut conn = self.connect().await?;
        check_platform_blocked(&mut conn, profile_address, platform_id, &self.metrics).await
    }

    /// Member, platform-block, and moderator flags for one wallet (single query).
    pub async fn get_platform_user_access(
        &self,
        platform_id: &str,
        user_address: &str,
    ) -> anyhow::Result<crate::platform::PlatformUserAccessRow> {
        let mut conn = self.connect().await?;
        get_platform_user_access(&mut conn, platform_id, user_address, &self.metrics).await
    }

    /// Get a comment by ID.
    pub async fn get_comment_by_id(
        &self,
        comment_id: &str,
    ) -> anyhow::Result<Option<crate::post::CommentRow>> {
        let mut conn = self.connect().await?;
        crate::post::get_comment_by_id(&mut conn, comment_id, &self.metrics).await
    }

    /// Get comments for a post.
    pub async fn get_post_comments(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::post::CommentRow>> {
        let mut conn = self.connect().await?;
        crate::post::get_post_comments(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get reactions for a post.
    pub async fn get_post_reactions(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::post::ReactionRow>> {
        let mut conn = self.connect().await?;
        crate::post::get_post_reactions(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get reposts for a post.
    pub async fn get_post_reposts(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::post::RepostRow>> {
        let mut conn = self.connect().await?;
        crate::post::get_post_reposts(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get tips for a post.
    pub async fn get_post_tips(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::post::TipRow>> {
        let mut conn = self.connect().await?;
        crate::post::get_post_tips(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get transfers for a post.
    pub async fn get_post_transfers(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::post::PostTransferRow>> {
        let mut conn = self.connect().await?;
        crate::post::get_post_transfers(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get a vesting wallet by ID.
    pub async fn get_vesting_wallet(
        &self,
        wallet_id: &str,
    ) -> anyhow::Result<Option<crate::vesting::VestingWalletWithStatus>> {
        let mut conn = self.connect().await?;
        get_vesting_wallet(&mut conn, wallet_id, &self.metrics).await
    }

    /// List vesting wallets with optional owner and active-only filters.
    pub async fn list_vesting_wallets(
        &self,
        owner: Option<&str>,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::vesting::VestingWalletWithStatus>> {
        let mut conn = self.connect().await?;
        list_vesting_wallets(&mut conn, owner, active_only, limit, offset, &self.metrics).await
    }

    /// Get vesting leaderboard.
    pub async fn get_vesting_leaderboard(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<crate::vesting::VestingLeaderboardResponse> {
        let mut conn = self.connect().await?;
        get_vesting_leaderboard(&mut conn, limit, offset, &self.metrics).await
    }

    /// Get SPT holdings for a holder address.
    pub async fn get_spt_holdings_by_holder(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::SptHoldingRow>> {
        let mut conn = self.connect().await?;
        get_spt_holdings_by_holder(&mut conn, address, limit, offset, &self.metrics).await
    }

    /// Get SPT holders for a pool (token-centric).
    pub async fn get_spt_holdings_by_pool(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
        viewer: Option<&str>,
        prioritize_followed: bool,
    ) -> anyhow::Result<Vec<crate::SptHoldingRow>> {
        let mut conn = self.connect().await?;
        get_spt_holdings_by_pool(
            &mut conn,
            pool_id,
            limit,
            offset,
            viewer,
            prioritize_followed,
            &self.metrics,
        )
        .await
    }

    /// Get SPT pool by pool ID.
    pub async fn get_spt_pool(&self, pool_id: &str) -> anyhow::Result<Option<crate::SptPoolRow>> {
        let mut conn = self.connect().await?;
        get_spt_pool(&mut conn, pool_id, &self.metrics).await
    }

    /// Aggregated reservation deposit / withdrawal volume by hour or day (MYSO base units).
    pub async fn get_spt_reservation_volume_history(
        &self,
        pool_id: &str,
        interval: SptReservationVolumeInterval,
        limit: i64,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
    ) -> anyhow::Result<Vec<crate::spt::SptReservationVolumeBucket>> {
        let mut conn = self.connect().await?;
        get_spt_reservation_volume_history(
            &mut conn,
            pool_id,
            interval,
            limit,
            from,
            to,
            &self.metrics,
        )
        .await
    }

    /// Get SPT price history for a pool.
    pub async fn get_spt_price_history(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::SptPriceHistory>> {
        let mut conn = self.connect().await?;
        get_spt_price_history(&mut conn, pool_id, limit, offset, &self.metrics).await
    }

    /// Get pool ID for a profile's token (profile tokens have associated_id = 'profile_' || address).
    pub async fn get_spt_pool_id_for_profile(
        &self,
        profile_address: &str,
    ) -> anyhow::Result<Option<String>> {
        let mut conn = self.connect().await?;
        get_spt_pool_id_for_profile(&mut conn, profile_address, &self.metrics).await
    }

    /// Get SPT transactions for a pool.
    pub async fn get_spt_transactions(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
        viewer: Option<&str>,
        prioritize_followed: bool,
    ) -> anyhow::Result<SptTransactionsWithViewer> {
        let mut conn = self.connect().await?;
        get_spt_transactions(
            &mut conn,
            pool_id,
            limit,
            offset,
            viewer,
            prioritize_followed,
            &self.metrics,
        )
        .await
    }

    /// Get user reservation holdings (reservation SPT positions).
    pub async fn get_user_reservation_holdings(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::UserReservationHoldingRow>>
    {
        let mut conn = self.connect().await?;
        get_user_reservation_holdings(&mut conn, address, limit, offset, &self.metrics).await
    }

    /// Reservation pool id for an SPT `associated_id` (e.g. `profile_0x...`).
    pub async fn get_reservation_pool_id_for_associated_id(
        &self,
        associated_id: &str,
    ) -> anyhow::Result<Option<String>> {
        let mut conn = self.connect().await?;
        get_reservation_pool_id_for_associated_id(&mut conn, associated_id, &self.metrics).await
    }

    /// Current reservation holders for a reservation pool (positive balances only).
    pub async fn get_reservation_holdings_for_pool(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
        viewer: Option<&str>,
        prioritize_followed: bool,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::UserReservationHoldingRow>>
    {
        let mut conn = self.connect().await?;
        get_reservation_holdings_for_pool(
            &mut conn,
            pool_id,
            limit,
            offset,
            viewer,
            prioritize_followed,
            &self.metrics,
        )
        .await
    }

    /// Former reservation holders (latest indexed row per reserver has amount 0).
    pub async fn get_former_reservation_holdings_for_pool(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
        viewer: Option<&str>,
        prioritize_followed: bool,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::UserReservationHoldingRow>>
    {
        let mut conn = self.connect().await?;
        get_former_reservation_holdings_for_pool(
            &mut conn,
            pool_id,
            limit,
            offset,
            viewer,
            prioritize_followed,
            &self.metrics,
        )
        .await
    }

    /// List SPT pools with optional token type filter and sorting.
    pub async fn list_spt_pools(
        &self,
        token_type: Option<i16>,
        sort_by: crate::spt::SptSortBy,
        ascending: bool,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::SptPoolRow>> {
        let mut conn = self.connect().await?;
        list_spt_pools(
            &mut conn,
            token_type,
            sort_by,
            ascending,
            limit,
            offset,
            &self.metrics,
        )
        .await
    }

    /// Get latest POC analysis for a post.
    pub async fn get_poc_analysis_for_post(
        &self,
        post_id: &str,
    ) -> anyhow::Result<Option<crate::PocAnalysisResultRow>> {
        let mut conn = self.connect().await?;
        get_poc_analysis_for_post(&mut conn, post_id, &self.metrics).await
    }

    /// Get POC badges for a post (non-revoked only).
    pub async fn get_poc_badges_for_post(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::PocBadgeRow>> {
        let mut conn = self.connect().await?;
        get_poc_badges_for_post(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get revenue redirections for a post (as accused or original).
    pub async fn get_post_revenue_redirections(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::PocRevenueRedirectionRow>> {
        let mut conn = self.connect().await?;
        get_post_revenue_redirections(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get POC disputes for a post.
    pub async fn get_poc_disputes_for_post(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::PocDisputeRow>> {
        let mut conn = self.connect().await?;
        get_poc_disputes_for_post(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get latest POC configuration.
    pub async fn get_poc_configuration(&self) -> anyhow::Result<Option<crate::PocConfigRow>> {
        let mut conn = self.connect().await?;
        get_poc_configuration(&mut conn, &self.metrics).await
    }

    /// Get latest SPT exchange configuration.
    pub async fn get_spt_exchange_config(
        &self,
    ) -> anyhow::Result<Option<crate::spt::SptExchangeConfigRow>> {
        let mut conn = self.connect().await?;
        get_spt_exchange_config(&mut conn, &self.metrics).await
    }

    /// Get latest post configuration.
    pub async fn get_post_config(&self) -> anyhow::Result<Option<crate::post::PostConfigRow>> {
        let mut conn = self.connect().await?;
        crate::post::get_post_config(&mut conn, &self.metrics).await
    }

    /// Get latest SPoT configuration.
    pub async fn get_spot_config(&self) -> anyhow::Result<Option<crate::spot::SpotConfigRow>> {
        let mut conn = self.connect().await?;
        get_spot_config(&mut conn, &self.metrics).await
    }

    /// Get latest MyData configuration.
    pub async fn get_mydata_config(
        &self,
    ) -> anyhow::Result<Option<crate::mydata::MyDataConfigRow>> {
        let mut conn = self.connect().await?;
        get_mydata_config(&mut conn, &self.metrics).await
    }

    /// Get latest insurance configuration.
    pub async fn get_insurance_config(
        &self,
    ) -> anyhow::Result<Option<crate::insurance::InsuranceConfigRow>> {
        let mut conn = self.connect().await?;
        get_insurance_config(&mut conn, &self.metrics).await
    }

    /// Get spot record for a post (1:1).
    pub async fn get_spot_record(
        &self,
        post_id: &str,
    ) -> anyhow::Result<Option<crate::SpotRecordRow>> {
        let mut conn = self.connect().await?;
        get_spot_record(&mut conn, post_id, &self.metrics).await
    }

    /// List spot bets for a post (paginated).
    pub async fn list_spot_bets(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::SpotBetRow>> {
        let mut conn = self.connect().await?;
        list_spot_bets(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// List spot payouts for a post (paginated).
    pub async fn list_spot_payouts(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::SpotPayoutRow>> {
        let mut conn = self.connect().await?;
        list_spot_payouts(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// List spot refunds for a post (paginated).
    pub async fn list_spot_refunds(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::SpotRefundRow>> {
        let mut conn = self.connect().await?;
        list_spot_refunds(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get spot resolution for a post (1:1, null if not resolved).
    pub async fn get_spot_resolution(
        &self,
        post_id: &str,
    ) -> anyhow::Result<Option<crate::SpotResolutionRow>> {
        let mut conn = self.connect().await?;
        get_spot_resolution(&mut conn, post_id, &self.metrics).await
    }

    /// List spot bet withdrawals for a post (paginated).
    pub async fn list_spot_bet_withdrawals(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::SpotBetWithdrawalRow>> {
        let mut conn = self.connect().await?;
        list_spot_bet_withdrawals(&mut conn, post_id, limit, offset, &self.metrics).await
    }

    /// Get a promotion by ID.
    pub async fn get_promotion(
        &self,
        promotion_id: &str,
    ) -> anyhow::Result<Option<crate::PromotedPostRow>> {
        let mut conn = self.connect().await?;
        get_promotion(&mut conn, promotion_id, &self.metrics).await
    }

    /// Get promotion for a post by post ID.
    pub async fn get_promotion_by_post_id(
        &self,
        post_id: &str,
    ) -> anyhow::Result<Option<crate::PromotedPostRow>> {
        let mut conn = self.connect().await?;
        get_promotion_by_post_id(&mut conn, post_id, &self.metrics).await
    }

    /// Get view count for a promotion.
    pub async fn get_promotion_views_count(&self, promotion_id: &str) -> anyhow::Result<i64> {
        let mut conn = self.connect().await?;
        get_promotion_views_count(&mut conn, promotion_id, &self.metrics).await
    }

    /// List promoted posts (paginated, optionally filtered by platform).
    pub async fn list_promoted_posts(
        &self,
        platform_id: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::PromotedPostRow>> {
        let mut conn = self.connect().await?;
        list_promoted_posts(&mut conn, platform_id, limit, offset, &self.metrics).await
    }

    /// Get promotion views (paginated).
    pub async fn get_promotion_views(
        &self,
        promotion_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::PromotionViewRow>> {
        let mut conn = self.connect().await?;
        get_promotion_views(&mut conn, promotion_id, limit, offset, &self.metrics).await
    }

    /// Get aggregated stats for a promotion.
    pub async fn get_promotion_stats(
        &self,
        promotion_id: &str,
    ) -> anyhow::Result<Option<crate::PromotionStatsRow>> {
        let mut conn = self.connect().await?;
        get_promotion_stats(&mut conn, promotion_id, &self.metrics).await
    }

    /// Get daily time series for a promotion (last 30 days).
    pub async fn get_promotion_time_series(
        &self,
        promotion_id: &str,
        limit: i64,
    ) -> anyhow::Result<Vec<crate::PromotionTimeSeriesRow>> {
        let mut conn = self.connect().await?;
        get_promotion_time_series(&mut conn, promotion_id, limit, &self.metrics).await
    }

    /// Get hourly aggregates for a promotion (last 7 days).
    pub async fn get_promotion_hourly(
        &self,
        promotion_id: &str,
        limit: i64,
    ) -> anyhow::Result<Vec<crate::PromotionHourlyRow>> {
        let mut conn = self.connect().await?;
        get_promotion_hourly(&mut conn, promotion_id, limit, &self.metrics).await
    }

    /// Get top performing promotions by view count.
    pub async fn get_top_performing_promotions(
        &self,
        limit: i64,
    ) -> anyhow::Result<Vec<crate::PromotedPostRow>> {
        let mut conn = self.connect().await?;
        get_top_performing_promotions(&mut conn, limit, &self.metrics).await
    }

    /// Get global spending trends (last 30 days).
    pub async fn get_spending_trends(
        &self,
        limit: i64,
    ) -> anyhow::Result<Vec<crate::PromotionTimeSeriesRow>> {
        let mut conn = self.connect().await?;
        get_spending_trends(&mut conn, limit, &self.metrics).await
    }

    /// Get a mydata record by ID.
    pub async fn get_mydata_record(
        &self,
        mydata_id: &str,
    ) -> anyhow::Result<Option<crate::MyDataRecordRow>> {
        let mut conn = self.connect().await?;
        get_mydata_record(&mut conn, mydata_id, &self.metrics).await
    }

    /// List mydata records by owner (paginated).
    pub async fn list_mydata_records_by_owner(
        &self,
        owner: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::MyDataRecordRow>> {
        let mut conn = self.connect().await?;
        list_mydata_records_by_owner(&mut conn, owner, limit, offset, &self.metrics).await
    }

    /// List mydata records (paginated, optionally filtered by creator, media_type, platform_id).
    pub async fn list_mydata(
        &self,
        creator: Option<&str>,
        media_type: Option<&str>,
        platform_id: Option<&str>,
        sort_by: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::MyDataRecordRow>> {
        let mut conn = self.connect().await?;
        list_mydata(
            &mut conn,
            creator,
            media_type,
            platform_id,
            sort_by,
            limit,
            offset,
            &self.metrics,
        )
        .await
    }

    /// Get popular mydata records (ordered by purchase + revenue + access counts).
    pub async fn get_popular_mydata(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::MyDataRecordRow>> {
        let mut conn = self.connect().await?;
        get_popular_mydata(&mut conn, limit, offset, &self.metrics).await
    }

    /// List mydata purchases by buyer (paginated).
    pub async fn list_mydata_purchases_by_buyer(
        &self,
        buyer: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::MyDataPurchaseRow>> {
        let mut conn = self.connect().await?;
        list_mydata_purchases_by_buyer(&mut conn, buyer, limit, offset, &self.metrics).await
    }

    /// List mydata purchases for a record (paginated).
    pub async fn get_mydata_purchases(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::MyDataPurchaseRow>> {
        let mut conn = self.connect().await?;
        get_mydata_purchases(&mut conn, mydata_id, limit, offset, &self.metrics).await
    }

    /// List mydata subscriptions for a record (paginated).
    pub async fn get_mydata_subscriptions(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::MyDataSubscriptionRow>> {
        let mut conn = self.connect().await?;
        get_mydata_subscriptions(&mut conn, mydata_id, limit, offset, &self.metrics).await
    }

    /// List mydata revenue for a record (paginated).
    pub async fn get_mydata_revenue(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::MyDataRevenueRow>> {
        let mut conn = self.connect().await?;
        get_mydata_revenue(&mut conn, mydata_id, limit, offset, &self.metrics).await
    }

    /// List mydata access logs for a record (paginated).
    pub async fn get_mydata_access_logs(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::MyDataAccessLogRow>> {
        let mut conn = self.connect().await?;
        get_mydata_access_logs(&mut conn, mydata_id, limit, offset, &self.metrics).await
    }

    /// Get mydata stats for a record.
    pub async fn get_mydata_stats(
        &self,
        mydata_id: &str,
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::MyDataStatsRow>> {
        let mut conn = self.connect().await?;
        get_mydata_stats(&mut conn, mydata_id, &self.metrics).await
    }

    /// Get mydata revenue timeline (daily aggregates).
    pub async fn get_mydata_revenue_timeline(
        &self,
        mydata_id: &str,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::MyDataDailyRevenueRow>> {
        let mut conn = self.connect().await?;
        get_mydata_revenue_timeline(&mut conn, mydata_id, &self.metrics).await
    }

    /// Get mydata access analytics (daily aggregates by access type).
    pub async fn get_mydata_access_analytics(
        &self,
        mydata_id: &str,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::MyDataAccessAnalyticsRow>> {
        let mut conn = self.connect().await?;
        get_mydata_access_analytics(&mut conn, mydata_id, &self.metrics).await
    }

    /// Get insurance policy by ID.
    pub async fn get_insurance_policy(
        &self,
        policy_id: &str,
    ) -> anyhow::Result<Option<crate::InsurancePolicyRow>> {
        let mut conn = self.connect().await?;
        get_insurance_policy(&mut conn, policy_id, &self.metrics).await
    }

    /// List insurance policies by insured address (paginated).
    pub async fn list_insurance_policies_by_insured(
        &self,
        insured: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::InsurancePolicyRow>> {
        let mut conn = self.connect().await?;
        list_insurance_policies_by_insured(&mut conn, insured, limit, offset, &self.metrics).await
    }

    /// List insurance vaults (paginated).
    pub async fn list_insurance_vaults(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::InsuranceVaultRow>> {
        let mut conn = self.connect().await?;
        list_insurance_vaults(&mut conn, limit, offset, &self.metrics).await
    }

    /// Get insurance vault by ID.
    pub async fn get_insurance_vault(
        &self,
        vault_id: &str,
    ) -> anyhow::Result<Option<crate::InsuranceVaultRow>> {
        let mut conn = self.connect().await?;
        get_insurance_vault(&mut conn, vault_id, &self.metrics).await
    }

    /// List insurance vault transactions (paginated).
    pub async fn list_insurance_vault_transactions(
        &self,
        vault_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::insurance::InsuranceVaultTransactionRow>> {
        let mut conn = self.connect().await?;
        list_insurance_vault_transactions(&mut conn, vault_id, limit, offset, &self.metrics).await
    }

    /// Get insurance vault exposures by market/option.
    pub async fn get_insurance_vault_exposures(
        &self,
        vault_id: &str,
    ) -> anyhow::Result<Vec<crate::insurance::InsuranceVaultExposureRow>> {
        let mut conn = self.connect().await?;
        get_insurance_vault_exposures(&mut conn, vault_id, &self.metrics).await
    }

    /// List insurance policies with optional filters (paginated).
    pub async fn list_insurance_policies(
        &self,
        insured: Option<&str>,
        market_id: Option<&str>,
        vault_id: Option<&str>,
        status: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::InsurancePolicyRow>> {
        let mut conn = self.connect().await?;
        list_insurance_policies(
            &mut conn,
            insured,
            market_id,
            vault_id,
            status,
            limit,
            offset,
            &self.metrics,
        )
        .await
    }

    /// List insurance policies by market (paginated).
    pub async fn list_insurance_market_policies(
        &self,
        market_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<crate::InsurancePolicyRow>> {
        let mut conn = self.connect().await?;
        list_insurance_market_policies(&mut conn, market_id, limit, offset, &self.metrics).await
    }

    /// List governance proposals (paginated, optionally filtered by platform, status, proposal type, submitter).
    /// When `platform_id` is set, `proposal_type` is ignored in favor of the platform's governance registry type.
    pub async fn list_proposals(
        &self,
        platform_id: Option<&str>,
        status: Option<i16>,
        proposal_type: Option<i16>,
        submitter: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::ProposalRow>> {
        let mut conn = self.connect().await?;
        list_proposals(
            &mut conn,
            platform_id,
            status,
            proposal_type,
            submitter,
            limit,
            offset,
            &self.metrics,
        )
        .await
    }

    /// Get a proposal by ID.
    pub async fn get_proposal_by_id(
        &self,
        id: &str,
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::ProposalRow>> {
        let mut conn = self.connect().await?;
        get_proposal_by_id(&mut conn, id, &self.metrics).await
    }

    /// List delegates (paginated, optionally filtered by registry type and active status).
    pub async fn list_delegates(
        &self,
        registry_type: Option<i16>,
        is_active: Option<bool>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::DelegateRow>> {
        let mut conn = self.connect().await?;
        list_delegates(
            &mut conn,
            registry_type,
            is_active,
            limit,
            offset,
            &self.metrics,
        )
        .await
    }

    /// Get a delegate by address.
    pub async fn get_delegate_by_address(
        &self,
        address: &str,
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::DelegateRow>> {
        let mut conn = self.connect().await?;
        get_delegate_by_address(&mut conn, address, &self.metrics).await
    }

    /// List nominated delegates (paginated). With `platform_id`, scopes to that platform's governance
    /// registry (same as `proposals(platformId:)`). Without it, returns only ecosystem/PoC rows
    /// (`governance_registry_id` NULL); omnibus queries exclude `registry_type = platform`.
    pub async fn list_nominated_delegates(
        &self,
        platform_id: Option<&str>,
        registry_type: Option<i16>,
        status: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::NominatedDelegateRow>> {
        let mut conn = self.connect().await?;
        list_nominated_delegates(
            &mut conn,
            platform_id,
            registry_type,
            status,
            limit,
            offset,
            &self.metrics,
        )
        .await
    }

    /// List governance registries, optionally filtered by registry type.
    pub async fn list_governance_registries(
        &self,
        registry_type: Option<i16>,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::GovernanceRegistryRow>> {
        let mut conn = self.connect().await?;
        list_governance_registries(&mut conn, registry_type, &self.metrics).await
    }

    /// Get governance registry by type.
    pub async fn get_governance_registry_by_type(
        &self,
        registry_type: i16,
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::GovernanceRegistryRow>> {
        let mut conn = self.connect().await?;
        get_governance_registry_by_type(&mut conn, registry_type, &self.metrics).await
    }

    /// Get governance registry for a platform (by platform ID).
    pub async fn get_governance_registry_by_platform_id(
        &self,
        platform_id: &str,
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::GovernanceRegistryRow>> {
        let mut conn = self.connect().await?;
        get_governance_registry_by_platform_id(&mut conn, platform_id, &self.metrics).await
    }

    /// Get delegate votes for a proposal (paginated).
    pub async fn get_proposal_delegate_votes(
        &self,
        proposal_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::DelegateVoteRow>> {
        let mut conn = self.connect().await?;
        get_proposal_delegate_votes(&mut conn, proposal_id, limit, offset, &self.metrics).await
    }

    /// Get count of community votes for a proposal.
    pub async fn get_proposal_community_votes_count(
        &self,
        proposal_id: &str,
    ) -> anyhow::Result<i64> {
        let mut conn = self.connect().await?;
        get_proposal_community_votes_count(&mut conn, proposal_id, &self.metrics).await
    }

    /// Get community votes for a proposal (paginated).
    pub async fn get_proposal_community_votes(
        &self,
        proposal_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::CommunityVoteRow>> {
        let mut conn = self.connect().await?;
        get_proposal_community_votes(&mut conn, proposal_id, limit, offset, &self.metrics).await
    }

    /// Get reward distributions for a proposal.
    pub async fn get_proposal_reward_distributions(
        &self,
        proposal_id: &str,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::RewardDistributionRow>> {
        let mut conn = self.connect().await?;
        get_proposal_reward_distributions(&mut conn, proposal_id, &self.metrics).await
    }

    /// Get proposals voted on by a delegate.
    pub async fn get_delegate_proposals(
        &self,
        address: &str,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::ProposalRow>> {
        let mut conn = self.connect().await?;
        get_delegate_proposals(&mut conn, address, &self.metrics).await
    }

    /// Get delegate ratings for an address.
    pub async fn get_delegate_ratings(
        &self,
        address: &str,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::DelegateRatingRow>> {
        let mut conn = self.connect().await?;
        get_delegate_ratings(&mut conn, address, &self.metrics).await
    }

    /// List governance events (paginated).
    pub async fn list_governance_events(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::GovernanceEventRow>> {
        let mut conn = self.connect().await?;
        list_governance_events(&mut conn, limit, offset, &self.metrics).await
    }

    /// Get anonymous voting stats for a proposal.
    pub async fn get_proposal_anonymous_stats(
        &self,
        proposal_id: &str,
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::AnonymousVotingStatsRow>>
    {
        let mut conn = self.connect().await?;
        get_proposal_anonymous_stats(&mut conn, proposal_id, &self.metrics).await
    }

    /// Get anonymous votes for a proposal (paginated).
    pub async fn get_proposal_anonymous_votes(
        &self,
        proposal_id: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::AnonymousVoteRow>> {
        let mut conn = self.connect().await?;
        get_proposal_anonymous_votes(&mut conn, proposal_id, limit, offset, &self.metrics).await
    }

    /// Get vote decryption failures for a proposal.
    pub async fn get_proposal_decryption_failures(
        &self,
        proposal_id: &str,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::VoteDecryptionFailureRow>> {
        let mut conn = self.connect().await?;
        get_proposal_decryption_failures(&mut conn, proposal_id, &self.metrics).await
    }

    /// Get anonymous voting trends (daily aggregates).
    pub async fn get_anonymous_voting_trends(
        &self,
        limit: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::AnonymousVotingTrendRow>> {
        let mut conn = self.connect().await?;
        get_anonymous_voting_trends(&mut conn, limit, &self.metrics).await
    }

    /// Get governance stats by registry type (from governance_stats view).
    pub async fn get_governance_stats_by_registry_type(
        &self,
        registry_type: i16,
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::GovernanceStatsRow>> {
        let mut conn = self.connect().await?;
        get_governance_stats_by_registry_type(&mut conn, registry_type, &self.metrics).await
    }

    /// Get platform revenue summary (from platform_revenue_summary view).
    pub async fn get_platform_revenue_summary(
        &self,
        platform_address: &str,
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::PlatformRevenueSummaryRow>>
    {
        let mut conn = self.connect().await?;
        get_platform_revenue_summary(&mut conn, platform_address, &self.metrics).await
    }

    /// Cash-flow P&L for a profile owner wallet across the given windows (MYSO base units).
    pub async fn get_profile_pnl(
        &self,
        owner_address: &str,
        windows: &[ProfilePnLWindow],
    ) -> anyhow::Result<Vec<ProfilePnLWindowResult>> {
        self.metrics.requests_received.inc();
        let _guard = self.metrics.latency.start_timer();
        let mut conn = self.connect().await?;
        let out = get_profile_pnl_for_windows(&mut conn, owner_address, windows).await?;
        self.metrics.requests_succeeded.inc();
        Ok(out)
    }
}
