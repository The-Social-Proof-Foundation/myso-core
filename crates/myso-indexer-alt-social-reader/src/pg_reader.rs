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

use crate::metrics::DbReaderMetrics;
use crate::platform::PlatformRow;
use crate::post::PostRow;
use crate::profile::get_profile_badges;
use crate::profile::get_profile_by_address;
use crate::profile::get_profile_or_wallet_by_address;
use crate::profile::get_profiles;
use crate::platform::{get_platform_blocked_profiles, get_platform_members, get_platform_moderators};
use crate::social_graph::{
    check_following, check_platform_blocked, check_profile_blocked, get_blocked_platforms,
    get_blocked_profiles, get_followers, get_following, get_profile_platform_memberships,
    FollowSortBy, ProfileSummaryRow,
};
use crate::poc::{
    get_poc_analysis_for_post, get_poc_badges_for_post, get_poc_configuration,
    get_poc_disputes_for_post,
};
use crate::insurance::{
    get_insurance_policy, list_insurance_policies_by_insured, list_insurance_vaults,
};
use crate::mydata::{
    get_mydata_record, list_mydata_purchases_by_buyer, list_mydata_records_by_owner,
};
use crate::governance::{
    get_delegate_by_address, get_governance_registry_by_platform_id,
    get_governance_registry_by_type, get_governance_stats_by_registry_type, get_proposal_by_id,
    list_delegates, list_governance_registries, list_proposals,
};
use crate::revenue::get_platform_revenue_summary;
use crate::spot::{get_spot_record, list_spot_bets};
use crate::promotion::{
    get_promotion, get_promotion_by_post_id, get_promotion_views_count, list_promoted_posts,
};
use crate::spt::{
    get_spt_holdings_by_holder, get_spt_holdings_by_pool, get_spt_pool, get_spt_pool_id_for_profile,
    get_spt_price_history, get_spt_transactions, get_user_reservation_holdings, list_spt_pools,
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
    pub async fn get_profile_summary(
        &self,
        address: &str,
    ) -> anyhow::Result<ProfileSummaryRow> {
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
    ) -> anyhow::Result<Vec<crate::social_graph::ProfileSummaryRow>> {
        let mut conn = self.connect().await?;
        let (rows, _) = get_followers(
            &mut conn,
            address,
            FollowSortBy::Latest,
            None,
            None,
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
    ) -> anyhow::Result<Vec<crate::social_graph::ProfileSummaryRow>> {
        let mut conn = self.connect().await?;
        get_following(&mut conn, address, limit, offset, &self.metrics).await
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
    ) -> anyhow::Result<Vec<crate::SptHoldingRow>> {
        let mut conn = self.connect().await?;
        get_spt_holdings_by_pool(&mut conn, pool_id, limit, offset, &self.metrics).await
    }

    /// Get SPT pool by pool ID.
    pub async fn get_spt_pool(
        &self,
        pool_id: &str,
    ) -> anyhow::Result<Option<crate::SptPoolRow>> {
        let mut conn = self.connect().await?;
        get_spt_pool(&mut conn, pool_id, &self.metrics).await
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
    ) -> anyhow::Result<Vec<crate::SptTransaction>> {
        let mut conn = self.connect().await?;
        get_spt_transactions(&mut conn, pool_id, limit, offset, &self.metrics).await
    }

    /// Get user reservation holdings (reservation SPT positions).
    pub async fn get_user_reservation_holdings(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::UserReservationHoldingRow>> {
        let mut conn = self.connect().await?;
        get_user_reservation_holdings(&mut conn, address, limit, offset, &self.metrics).await
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
    pub async fn get_poc_configuration(
        &self,
    ) -> anyhow::Result<Option<crate::PocConfigRow>> {
        let mut conn = self.connect().await?;
        get_poc_configuration(&mut conn, &self.metrics).await
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
    pub async fn get_promotion_views_count(
        &self,
        promotion_id: &str,
    ) -> anyhow::Result<i64> {
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

    /// List governance proposals (paginated, optionally filtered by platform, status, proposal type).
    pub async fn list_proposals(
        &self,
        platform_id: Option<&str>,
        status: Option<i16>,
        proposal_type: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::ProposalRow>> {
        let mut conn = self.connect().await?;
        list_proposals(
            &mut conn,
            platform_id,
            status,
            proposal_type,
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
        list_delegates(&mut conn, registry_type, is_active, limit, offset, &self.metrics).await
    }

    /// Get a delegate by address.
    pub async fn get_delegate_by_address(
        &self,
        address: &str,
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::DelegateRow>> {
        let mut conn = self.connect().await?;
        get_delegate_by_address(&mut conn, address, &self.metrics).await
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
    ) -> anyhow::Result<Option<myso_indexer_alt_social_schema::models::PlatformRevenueSummaryRow>> {
        let mut conn = self.connect().await?;
        get_platform_revenue_summary(&mut conn, platform_address, &self.metrics).await
    }
}
