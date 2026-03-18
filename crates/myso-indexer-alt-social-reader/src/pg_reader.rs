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
use crate::platform::get_platform_blocked_profiles;
use crate::social_graph::{
    check_following, check_platform_blocked, check_profile_blocked, get_blocked_platforms,
    get_blocked_profiles, get_followers, get_following,
};
use crate::poc::{
    get_poc_analysis_for_post, get_poc_badges_for_post, get_poc_configuration,
    get_poc_disputes_for_post,
};
use crate::spot::{get_spot_record, list_spot_bets};
use crate::spt::{
    get_spt_holdings_by_holder, get_spt_pool, get_spt_pool_id_for_profile, get_spt_price_history,
    get_spt_transactions,
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
        get_followers(&mut conn, address, limit, offset, &self.metrics).await
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
}
