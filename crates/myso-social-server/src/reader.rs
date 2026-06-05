// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

mod governance;
mod insurance;
pub mod memory;
mod mydata;
mod platform;
mod poc;
mod post;
pub mod profile;
mod promotion;
mod revenue;
mod search;
mod social_graph;
mod spot;
mod spt;
mod subscription;
mod system;
mod types;
mod upgrade;
mod vesting;

pub use types::*;

use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{MemoryAccountRow, Profile, SubAgentRow};
use myso_indexer_alt_social_schema::schema::{
    profile_subscription_services, profile_subscriptions, subscription_revenue,
};
use myso_pg_db::{Db, DbArgs};
use url::Url;

// All type definitions are in the types module and re-exported via pub use types::*

#[derive(Clone)]
pub struct Reader {
    db: Db,
}

impl Reader {
    pub async fn new(database_url: Url, db_args: DbArgs) -> Result<Self, anyhow::Error> {
        let db = Db::for_read(database_url, db_args).await?;
        let _ = db.connect().await?;
        Ok(Self { db })
    }

    pub async fn get_profiles(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Profile>, crate::error::SocialError> {
        profile::get_profiles(&self.db, limit, offset).await
    }

    pub async fn get_profiles_enriched(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<crate::reader::UniversalUserResult>, crate::error::SocialError> {
        profile::get_profiles_enriched(&self.db, limit, offset).await
    }

    pub async fn get_profile_count(&self) -> Result<i64, crate::error::SocialError> {
        profile::get_profile_count(&self.db).await
    }

    pub async fn get_profile_by_address(
        &self,
        address: &str,
    ) -> Result<Option<Profile>, crate::error::SocialError> {
        profile::get_profile_by_address(&self.db, address).await
    }

    pub async fn get_profile_or_wallet_by_address(
        &self,
        address: &str,
    ) -> Result<ProfileByAddressResponse, crate::error::SocialError> {
        profile::get_profile_or_wallet_by_address(&self.db, address).await
    }

    pub async fn get_profile_by_username(
        &self,
        username: &str,
    ) -> Result<Option<Profile>, crate::error::SocialError> {
        profile::get_profile_by_username(&self.db, username).await
    }

    pub async fn get_profile_daily_stats_chart(
        &self,
        query: &SocialGraphChartQuery,
    ) -> Result<ProfileDailyStatsChartData, crate::error::SocialError> {
        profile::get_profile_daily_stats_chart(&self.db, query).await
    }

    pub async fn list_profile_offers(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileOffer>, crate::error::SocialError> {
        profile::list_profile_offers(&self.db, address, limit, offset).await
    }

    pub async fn list_profile_sale_fees(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSaleFee>, crate::error::SocialError> {
        profile::list_profile_sale_fees(&self.db, address, limit, offset).await
    }

    pub async fn list_post_transfers(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostTransfer>, crate::error::SocialError> {
        post::list_post_transfers(&self.db, post_id, limit, offset).await
    }

    pub async fn list_post_reports(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostReport>, crate::error::SocialError> {
        post::list_post_reports(&self.db, post_id, limit, offset).await
    }

    pub async fn list_post_moderation_events(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostModerationEventRow>, crate::error::SocialError> {
        post::list_post_moderation_events(&self.db, post_id, limit, offset).await
    }

    pub async fn list_post_deletion_events(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostDeletionEventRow>, crate::error::SocialError> {
        post::list_post_deletion_events(&self.db, post_id, limit, offset).await
    }

    pub async fn get_mydata_by_id(
        &self,
        mydata_id: &str,
    ) -> Result<Option<MyDataBasic>, crate::error::SocialError> {
        mydata::get_mydata_by_id(&self.db, mydata_id).await
    }

    pub async fn list_mydata(
        &self,
        limit: i64,
        offset: i64,
        creator: Option<&str>,
        media_type: Option<&str>,
        platform_id: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<Vec<MyDataBasic>, crate::error::SocialError> {
        mydata::list_mydata(
            &self.db,
            limit,
            offset,
            creator,
            media_type,
            platform_id,
            sort_by,
        )
        .await
    }

    pub async fn get_mydata_configuration(
        &self,
    ) -> Result<Option<MyDataConfigInfo>, crate::error::SocialError> {
        mydata::get_mydata_configuration(&self.db).await
    }

    pub async fn get_popular_mydata(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataBasic>, crate::error::SocialError> {
        mydata::get_popular_mydata(&self.db, limit, offset).await
    }

    pub async fn get_mydata_purchases(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PurchaseInfo>, crate::error::SocialError> {
        mydata::get_mydata_purchases(&self.db, mydata_id, limit, offset).await
    }

    pub async fn get_mydata_subscriptions(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SubscriptionInfo>, crate::error::SocialError> {
        mydata::get_mydata_subscriptions(&self.db, mydata_id, limit, offset).await
    }

    pub async fn get_mydata_revenue(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RevenueInfo>, crate::error::SocialError> {
        mydata::get_mydata_revenue(&self.db, mydata_id, limit, offset).await
    }

    pub async fn get_mydata_access_logs(
        &self,
        mydata_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AccessLogInfo>, crate::error::SocialError> {
        mydata::get_mydata_access_logs(&self.db, mydata_id, limit, offset).await
    }

    pub async fn get_creator_mydata(
        &self,
        creator: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataBasic>, crate::error::SocialError> {
        mydata::get_creator_mydata(&self.db, creator, limit, offset).await
    }

    pub async fn get_mydata_stats(
        &self,
        mydata_id: &str,
    ) -> Result<Option<MyDataStatsResponse>, crate::error::SocialError> {
        mydata::get_mydata_stats(&self.db, mydata_id).await
    }

    pub async fn get_mydata_revenue_timeline(
        &self,
        mydata_id: &str,
    ) -> Result<Vec<DailyRevenue>, crate::error::SocialError> {
        mydata::get_mydata_revenue_timeline(&self.db, mydata_id).await
    }

    pub async fn get_mydata_access_analytics(
        &self,
        mydata_id: &str,
    ) -> Result<Vec<AccessAnalytics>, crate::error::SocialError> {
        mydata::get_mydata_access_analytics(&self.db, mydata_id).await
    }

    pub async fn list_mydata_broad_pools(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataBroadPoolInfo>, crate::error::SocialError> {
        mydata::list_mydata_broad_pools(&self.db, limit, offset).await
    }

    pub async fn list_mydata_sub_pools_for_broad_pool(
        &self,
        broad_pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataSubPoolInfo>, crate::error::SocialError> {
        mydata::list_mydata_sub_pools_for_broad_pool(&self.db, broad_pool_id, limit, offset)
            .await
    }

    pub async fn list_mydata_sub_pools_for_listing(
        &self,
        listing_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataSubPoolInfo>, crate::error::SocialError> {
        mydata::list_mydata_sub_pools_for_listing(&self.db, listing_id, limit, offset).await
    }

    pub async fn list_mydata_listings_for_sub_pool(
        &self,
        sub_pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataListingSubPoolInfo>, crate::error::SocialError> {
        mydata::list_mydata_listings_for_sub_pool(&self.db, sub_pool_id, limit, offset).await
    }

    pub async fn get_mydata_snapshot_anchor(
        &self,
        snapshot_id: &str,
    ) -> Result<Option<MyDataSnapshotAnchorInfo>, crate::error::SocialError> {
        mydata::get_mydata_snapshot_anchor(&self.db, snapshot_id).await
    }

    pub async fn get_mydata_distribution_round(
        &self,
        snapshot_id: &str,
    ) -> Result<Option<MyDataDistributionRoundInfo>, crate::error::SocialError> {
        mydata::get_mydata_distribution_round(&self.db, snapshot_id).await
    }

    pub async fn list_mydata_distribution_rounds(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataDistributionRoundInfo>, crate::error::SocialError> {
        mydata::list_mydata_distribution_rounds(&self.db, limit, offset).await
    }

    pub async fn get_mydata_merkle_root(
        &self,
        snapshot_id: &str,
    ) -> Result<Option<MyDataMerkleRootInfo>, crate::error::SocialError> {
        mydata::get_mydata_merkle_root(&self.db, snapshot_id).await
    }

    pub async fn list_mydata_claims_for_snapshot(
        &self,
        snapshot_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MyDataClaimInfo>, crate::error::SocialError> {
        mydata::list_mydata_claims_for_snapshot(&self.db, snapshot_id, limit, offset).await
    }

    pub async fn get_insurance_configuration(
        &self,
    ) -> Result<Option<InsuranceConfigInfo>, crate::error::SocialError> {
        insurance::get_insurance_configuration(&self.db).await
    }

    pub async fn list_insurance_vaults(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InsuranceVaultRow>, crate::error::SocialError> {
        insurance::list_insurance_vaults(&self.db, limit, offset).await
    }

    pub async fn get_insurance_vault(
        &self,
        vault_id: &str,
    ) -> Result<Option<InsuranceVaultInfo>, crate::error::SocialError> {
        insurance::get_insurance_vault(&self.db, vault_id).await
    }

    pub async fn list_insurance_vault_transactions(
        &self,
        vault_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InsuranceVaultTransactionRow>, crate::error::SocialError> {
        insurance::list_insurance_vault_transactions(&self.db, vault_id, limit, offset).await
    }

    pub async fn get_insurance_vault_exposures(
        &self,
        vault_id: &str,
    ) -> Result<Vec<InsuranceVaultExposureRow>, crate::error::SocialError> {
        insurance::get_insurance_vault_exposures(&self.db, vault_id).await
    }

    pub async fn list_insurance_policies(
        &self,
        insured: Option<&str>,
        market_id: Option<&str>,
        vault_id: Option<&str>,
        status: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InsurancePolicyRow>, crate::error::SocialError> {
        insurance::list_insurance_policies(
            &self.db, insured, market_id, vault_id, status, limit, offset,
        )
        .await
    }

    pub async fn get_insurance_policy(
        &self,
        policy_id: &str,
    ) -> Result<Option<InsurancePolicyInfo>, crate::error::SocialError> {
        insurance::get_insurance_policy(&self.db, policy_id).await
    }

    pub async fn list_insurance_market_policies(
        &self,
        market_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InsurancePolicyRow>, crate::error::SocialError> {
        insurance::list_insurance_market_policies(&self.db, market_id, limit, offset).await
    }

    pub async fn get_spot_record(
        &self,
        post_id: &str,
    ) -> Result<Option<SpotRecordResponse>, crate::error::SocialError> {
        spot::get_spot_record(&self.db, post_id).await
    }

    pub async fn list_spot_bets(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SpotBetRow>, crate::error::SocialError> {
        spot::list_spot_bets(&self.db, post_id, limit, offset).await
    }

    pub async fn list_spot_payouts(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SpotTransferRow>, crate::error::SocialError> {
        spot::list_spot_payouts(&self.db, post_id, limit, offset).await
    }

    pub async fn list_spot_refunds(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SpotTransferRow>, crate::error::SocialError> {
        spot::list_spot_refunds(&self.db, post_id, limit, offset).await
    }

    pub async fn get_spot_configuration(
        &self,
    ) -> Result<Option<SpotConfigInfo>, crate::error::SocialError> {
        spot::get_spot_configuration(&self.db).await
    }

    pub async fn list_contested_spot_records(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SpotRecordResponse>, crate::error::SocialError> {
        spot::list_contested_spot_records(&self.db, limit, offset).await
    }

    pub async fn list_proposals(
        &self,
        limit: i64,
        offset: i64,
        status: Option<i16>,
        proposal_type: Option<i16>,
        platform_id: Option<&str>,
        submitter: Option<&str>,
    ) -> Result<Vec<ProposalRow>, crate::error::SocialError> {
        governance::list_proposals(
            &self.db,
            limit,
            offset,
            status,
            proposal_type,
            platform_id,
            submitter,
        )
        .await
    }

    pub async fn get_proposal_by_id(
        &self,
        id: &str,
    ) -> Result<Option<ProposalRow>, crate::error::SocialError> {
        governance::get_proposal_by_id(&self.db, id).await
    }

    pub async fn get_proposal_delegate_votes(
        &self,
        proposal_id: &str,
    ) -> Result<Vec<DelegateVoteRow>, crate::error::SocialError> {
        governance::get_proposal_delegate_votes(&self.db, proposal_id).await
    }

    pub async fn get_proposal_community_votes_count(
        &self,
        proposal_id: &str,
    ) -> Result<i64, crate::error::SocialError> {
        governance::get_proposal_community_votes_count(&self.db, proposal_id).await
    }

    pub async fn get_proposal_community_votes(
        &self,
        proposal_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CommunityVoteRow>, crate::error::SocialError> {
        governance::get_proposal_community_votes(&self.db, proposal_id, limit, offset).await
    }

    pub async fn get_proposal_reward_distributions(
        &self,
        proposal_id: &str,
    ) -> Result<Vec<RewardDistributionRow>, crate::error::SocialError> {
        governance::get_proposal_reward_distributions(&self.db, proposal_id).await
    }

    pub async fn list_delegates(
        &self,
        limit: i64,
        offset: i64,
        registry_type: Option<i16>,
        is_active: Option<bool>,
    ) -> Result<Vec<DelegateRow>, crate::error::SocialError> {
        governance::list_delegates(&self.db, limit, offset, registry_type, is_active).await
    }

    pub async fn get_delegate_by_address(
        &self,
        address: &str,
        registry_type: Option<i16>,
        governance_registry_id: Option<&str>,
    ) -> Result<Option<DelegateRow>, crate::error::SocialError> {
        governance::get_delegate_by_address(
            &self.db,
            address,
            registry_type,
            governance_registry_id,
        )
        .await
    }

    pub async fn get_delegate_proposals(
        &self,
        address: &str,
    ) -> Result<Vec<ProposalRow>, crate::error::SocialError> {
        governance::get_delegate_proposals(&self.db, address).await
    }

    pub async fn get_delegate_ratings(
        &self,
        address: &str,
    ) -> Result<Vec<DelegateRatingRow>, crate::error::SocialError> {
        governance::get_delegate_ratings(&self.db, address).await
    }

    pub async fn list_nominees(
        &self,
        limit: i64,
        offset: i64,
        platform_id: Option<&str>,
        registry_type: Option<i16>,
        status: Option<i16>,
    ) -> Result<Vec<NominatedDelegateRow>, crate::error::SocialError> {
        governance::list_nominees(&self.db, limit, offset, platform_id, registry_type, status).await
    }

    pub async fn list_governance_registries(
        &self,
    ) -> Result<Vec<GovernanceRegistryRow>, crate::error::SocialError> {
        governance::list_governance_registries(&self.db).await
    }

    pub async fn get_governance_registry_by_type(
        &self,
        registry_type: i16,
    ) -> Result<Option<GovernanceRegistryRow>, crate::error::SocialError> {
        governance::get_governance_registry_by_type(&self.db, registry_type).await
    }

    pub async fn get_governance_registry_by_platform_id(
        &self,
        platform_id: &str,
    ) -> Result<Option<GovernanceRegistryRow>, crate::error::SocialError> {
        governance::get_governance_registry_by_platform_id(&self.db, platform_id).await
    }

    pub async fn list_governance_events(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GovernanceEventRow>, crate::error::SocialError> {
        governance::list_governance_events(&self.db, limit, offset).await
    }

    pub async fn get_proposal_anonymous_stats(
        &self,
        proposal_id: &str,
    ) -> Result<Option<AnonymousVotingStatsRow>, crate::error::SocialError> {
        governance::get_proposal_anonymous_stats(&self.db, proposal_id).await
    }

    pub async fn get_proposal_anonymous_votes(
        &self,
        proposal_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AnonymousVoteRow>, crate::error::SocialError> {
        governance::get_proposal_anonymous_votes(&self.db, proposal_id, limit, offset).await
    }

    pub async fn get_proposal_decryption_failures(
        &self,
        proposal_id: &str,
    ) -> Result<Vec<VoteDecryptionFailureRow>, crate::error::SocialError> {
        governance::get_proposal_decryption_failures(&self.db, proposal_id).await
    }

    pub async fn get_anonymous_voting_trends(
        &self,
        limit: i64,
    ) -> Result<Vec<AnonymousVotingTrendRow>, crate::error::SocialError> {
        governance::get_anonymous_voting_trends(&self.db, limit).await
    }

    pub async fn get_spt_pool(
        &self,
        pool_id: &str,
    ) -> Result<Option<SptPoolRow>, crate::error::SocialError> {
        spt::get_spt_pool(&self.db, pool_id).await
    }

    pub async fn list_spt_pools(
        &self,
        limit: i64,
        offset: i64,
        owner: Option<&str>,
        token_type: Option<i16>,
    ) -> Result<Vec<SptPoolRow>, crate::error::SocialError> {
        spt::list_spt_pools(&self.db, limit, offset, owner, token_type).await
    }

    pub async fn get_spt_transactions(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptTransactionRow>, crate::error::SocialError> {
        spt::get_spt_transactions(&self.db, pool_id, limit, offset).await
    }

    pub async fn get_spt_holdings(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptHoldingRow>, crate::error::SocialError> {
        spt::get_spt_holdings(&self.db, pool_id, limit, offset).await
    }

    pub async fn get_spt_price_history(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptPriceHistoryRow>, crate::error::SocialError> {
        spt::get_spt_price_history(&self.db, pool_id, limit, offset).await
    }

    pub async fn get_spt_exchange_config(
        &self,
    ) -> Result<Option<SptExchangeConfigRow>, crate::error::SocialError> {
        spt::get_spt_exchange_config(&self.db).await
    }

    pub async fn get_spt_reservation_pool(
        &self,
        pool_id: &str,
    ) -> Result<Option<SptReservationPoolRow>, crate::error::SocialError> {
        spt::get_spt_reservation_pool(&self.db, pool_id).await
    }

    pub async fn list_spt_reservation_pools(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<SptReservationPoolWithDisplayRow>, i64), crate::error::SocialError> {
        spt::list_spt_reservation_pools(&self.db, limit, offset).await
    }

    pub async fn get_spt_analytics_top_performers(
        &self,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        spt::get_spt_analytics_top_performers(&self.db).await
    }

    pub async fn get_spt_portfolio_performance(
        &self,
        address: &str,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        spt::get_spt_portfolio_performance(&self.db, address).await
    }

    pub async fn get_spt_creator_revenue_streams(
        &self,
        address: &str,
        from_ts: chrono::DateTime<chrono::Utc>,
        to_ts: chrono::DateTime<chrono::Utc>,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        spt::get_spt_creator_revenue_streams(&self.db, address, from_ts, to_ts).await
    }

    pub async fn get_spt_market_sentiment(
        &self,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        spt::get_spt_market_sentiment(&self.db).await
    }

    pub async fn get_spt_liquidity_profile(
        &self,
        pool_id: &str,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        spt::get_spt_liquidity_profile(&self.db, pool_id).await
    }

    pub async fn list_spt_reservations(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptReservationRow>, crate::error::SocialError> {
        spt::list_spt_reservations(&self.db, pool_id, limit, offset).await
    }

    pub async fn get_spt_reservation_volume_history(
        &self,
        pool_id: &str,
        trunc: &str,
        limit: i64,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<SptReservationVolumeBucketRow>, crate::error::SocialError> {
        spt::get_spt_reservation_volume_history(&self.db, pool_id, trunc, limit, from, to).await
    }

    pub async fn get_spt_revenue(
        &self,
        pool_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptRevenueRow>, crate::error::SocialError> {
        spt::get_spt_revenue(&self.db, pool_id, limit, offset).await
    }

    pub async fn get_revenue_dashboard(
        &self,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        revenue::get_revenue_dashboard(&self.db).await
    }

    pub async fn get_revenue_leaderboard(
        &self,
        limit: i64,
        min_revenue: i64,
        revenue_source: Option<&str>,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        revenue::get_revenue_leaderboard(&self.db, limit, min_revenue, revenue_source).await
    }

    pub async fn get_revenue_chart_data(
        &self,
        creator_address: Option<&str>,
        period: &str,
        start_date: chrono::NaiveDateTime,
        end_date: chrono::NaiveDateTime,
        _points: i64,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        revenue::get_revenue_chart_data(
            &self.db,
            creator_address,
            period,
            start_date,
            end_date,
            _points,
        )
        .await
    }

    pub async fn get_unified_revenue(
        &self,
        creator_address: Option<&str>,
        platform_address: Option<&str>,
        revenue_source: Option<&str>,
        revenue_type: Option<&str>,
        content_id: Option<&str>,
        content_type: Option<&str>,
        start_date: Option<chrono::NaiveDateTime>,
        end_date: Option<chrono::NaiveDateTime>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<UnifiedRevenue>, i64, i64), crate::error::SocialError> {
        revenue::get_unified_revenue(
            &self.db,
            creator_address,
            platform_address,
            revenue_source,
            revenue_type,
            content_id,
            content_type,
            start_date,
            end_date,
            limit,
            offset,
        )
        .await
    }

    pub async fn get_creator_revenue_stats(
        &self,
        creator_address: &str,
    ) -> Result<Option<serde_json::Value>, crate::error::SocialError> {
        revenue::get_creator_revenue_stats(&self.db, creator_address).await
    }

    pub async fn get_platform_revenue_stats(
        &self,
        platform_address: &str,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        revenue::get_platform_revenue_stats(&self.db, platform_address).await
    }

    pub async fn get_current_treasury(
        &self,
    ) -> Result<Option<serde_json::Value>, crate::error::SocialError> {
        revenue::get_current_treasury(&self.db).await
    }

    pub async fn get_subscription_analytics(
        &self,
        service_id: Option<&str>,
        _profile_owner: Option<&str>,
        start_date: chrono::NaiveDateTime,
        end_date: chrono::NaiveDateTime,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        use diesel::dsl::sum;
        let mut conn = self.db.connect().await?;
        let start_dt =
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(start_date, chrono::Utc);
        let end_dt =
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(end_date, chrono::Utc);

        let total_subscriptions: i64 = {
            let mut q = profile_subscriptions::table.into_boxed();
            q = q.filter(profile_subscriptions::time.between(start_dt, end_dt));
            if let Some(sid) = service_id {
                q = q.filter(profile_subscriptions::service_id.eq(sid));
            }
            q.count().get_result(&mut conn).await?
        };
        let active_subscriptions: i64 = {
            let mut q = profile_subscriptions::table.into_boxed();
            q = q.filter(profile_subscriptions::time.between(start_dt, end_dt));
            if let Some(sid) = service_id {
                q = q.filter(profile_subscriptions::service_id.eq(sid));
            }
            q.filter(profile_subscriptions::cancelled_at.is_null())
                .count()
                .get_result(&mut conn)
                .await?
        };
        let cancelled_subscriptions: i64 = {
            let mut q = profile_subscriptions::table.into_boxed();
            q = q.filter(profile_subscriptions::time.between(start_dt, end_dt));
            if let Some(sid) = service_id {
                q = q.filter(profile_subscriptions::service_id.eq(sid));
            }
            q.filter(profile_subscriptions::cancelled_at.is_not_null())
                .count()
                .get_result(&mut conn)
                .await?
        };

        let churn_rate = if total_subscriptions > 0 {
            cancelled_subscriptions as f64 / total_subscriptions as f64
        } else {
            0.0
        };

        let mut rev_query = subscription_revenue::table.into_boxed();
        rev_query = rev_query.filter(subscription_revenue::time.between(start_dt, end_dt));
        if let Some(sid) = service_id {
            rev_query = rev_query.filter(subscription_revenue::service_id.eq(sid));
        }
        let total_revenue: Option<bigdecimal::BigDecimal> = rev_query
            .select(sum(subscription_revenue::amount))
            .get_result(&mut conn)
            .await?;
        let total_revenue: i64 = total_revenue
            .and_then(|bd| bigdecimal::ToPrimitive::to_i64(&bd))
            .unwrap_or(0);
        let monthly_recurring_revenue = if total_revenue > 0 {
            total_revenue / 30
        } else {
            0
        };

        let service_id_str = service_id.unwrap_or("all").to_string();
        Ok(serde_json::json!({
            "service_id": service_id_str,
            "total_revenue": total_revenue,
            "active_subscriptions": active_subscriptions,
            "cancelled_subscriptions": cancelled_subscriptions,
            "monthly_recurring_revenue": monthly_recurring_revenue,
            "churn_rate": churn_rate,
            "average_subscription_duration": 30.0,
            "total_renewals": 0,
            "auto_renewal_rate": 0.0,
            "refund_rate": 0.0,
            "growth_metrics": []
        }))
    }

    pub async fn get_service_performance(
        &self,
        profile_owner: Option<&str>,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let mut query = profile_subscription_services::table.into_boxed();
        if let Some(owner) = profile_owner {
            query = query.filter(profile_subscription_services::profile_owner.eq(owner));
        }
        let rows: Vec<(String, String, String, i64, bool, i64, i64)> = query
            .select((
                profile_subscription_services::service_id,
                profile_subscription_services::profile_owner,
                profile_subscription_services::profile_id,
                profile_subscription_services::monthly_fee,
                profile_subscription_services::active,
                profile_subscription_services::subscriber_count,
                profile_subscription_services::created_at,
            ))
            .load(&mut conn)
            .await?;
        let services: Vec<serde_json::Value> = rows
            .into_iter()
            .map(
                |(
                    service_id,
                    profile_owner,
                    profile_id,
                    monthly_fee,
                    _active,
                    subscriber_count,
                    _created_at,
                )| {
                    let mrr = monthly_fee * subscriber_count;
                    serde_json::json!({
                        "service_id": service_id,
                        "profile_owner": profile_owner,
                        "profile_id": profile_id,
                        "monthly_fee": monthly_fee,
                        "total_subscribers": subscriber_count,
                        "active_subscribers": subscriber_count,
                        "total_revenue": mrr,
                        "monthly_recurring_revenue": mrr,
                        "churn_rate": 0.0,
                        "average_lifetime_value": 0.0,
                        "conversion_rate": 0.0
                    })
                },
            )
            .collect();
        Ok(services)
    }

    pub async fn get_treasury_history(
        &self,
        limit: i64,
    ) -> Result<Vec<serde_json::Value>, crate::error::SocialError> {
        revenue::get_treasury_history(&self.db, limit).await
    }

    pub async fn get_upgrade_events(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UpgradeEventRow>, crate::error::SocialError> {
        upgrade::get_upgrade_events(&self.db, limit, offset).await
    }

    pub async fn get_object_migrated_events(
        &self,
        limit: i64,
        offset: i64,
        object_id_filter: Option<&str>,
    ) -> Result<Vec<ObjectMigratedEventRow>, crate::error::SocialError> {
        upgrade::get_object_migrated_events(&self.db, limit, offset, object_id_filter).await
    }

    pub async fn get_profile_subscription_service(
        &self,
        service_id: &str,
    ) -> Result<Option<ProfileSubscriptionServiceInfo>, crate::error::SocialError> {
        subscription::get_profile_subscription_service(&self.db, service_id).await
    }

    pub async fn get_profile_subscription_services_by_owner(
        &self,
        profile_owner: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionServiceInfo>, crate::error::SocialError> {
        subscription::get_profile_subscription_services_by_owner(
            &self.db,
            profile_owner,
            limit,
            offset,
        )
        .await
    }

    pub async fn get_active_subscriptions_by_subscriber(
        &self,
        subscriber: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionInfo>, crate::error::SocialError> {
        subscription::get_active_subscriptions_by_subscriber(&self.db, subscriber, limit, offset)
            .await
    }

    pub async fn get_subscription_by_id(
        &self,
        subscription_id: &str,
    ) -> Result<Option<ProfileSubscriptionInfo>, crate::error::SocialError> {
        subscription::get_subscription_by_id(&self.db, subscription_id).await
    }

    pub async fn get_subscription_revenue_by_service(
        &self,
        service_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionRevenueRow>, crate::error::SocialError> {
        subscription::get_subscription_revenue_by_service(&self.db, service_id, limit, offset).await
    }

    pub async fn check_subscription_access(
        &self,
        subscriber: &str,
        service_id: &str,
    ) -> Result<bool, crate::error::SocialError> {
        subscription::check_subscription_access(&self.db, subscriber, service_id).await
    }

    pub async fn get_system_stats(&self) -> Result<SystemStatsResponse, crate::error::SocialError> {
        system::get_system_stats(&self.db).await
    }

    pub async fn check_username_availability(
        &self,
        username: &str,
        exclude_address: Option<&str>,
    ) -> Result<bool, crate::error::SocialError> {
        system::check_username_availability(&self.db, username, exclude_address).await
    }

    pub async fn get_profile_posts(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<myso_indexer_alt_social_reader::PostRow>, crate::error::SocialError> {
        social_graph::get_profile_posts(&self.db, address, limit, offset).await
    }

    pub async fn get_profile_pnl(
        &self,
        address: &str,
        windows: &[myso_indexer_alt_social_reader::ProfilePnLWindow],
    ) -> Result<
        Vec<myso_indexer_alt_social_reader::ProfilePnLWindowResult>,
        crate::error::SocialError,
    > {
        let mut conn = self.db.connect().await?;
        myso_indexer_alt_social_reader::get_profile_pnl_for_windows(&mut conn, address, windows)
            .await
            .map_err(|e| crate::error::SocialError::internal(e.to_string()))
    }

    pub async fn get_profile_events(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileEventRow>, crate::error::SocialError> {
        social_graph::get_profile_events(&self.db, address, limit, offset).await
    }

    pub async fn get_profile_platform_memberships(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfilePlatformMembershipRow>, crate::error::SocialError> {
        social_graph::get_profile_platform_memberships(&self.db, address, limit, offset).await
    }

    pub async fn get_profile_platform_events(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<ProfilePlatformEventRow>, i64), crate::error::SocialError> {
        social_graph::get_profile_platform_events(&self.db, address, limit, offset).await
    }

    pub async fn get_blocking_history(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BlockedEventRow>, crate::error::SocialError> {
        social_graph::get_blocking_history(&self.db, address, limit, offset).await
    }

    pub async fn get_profile_badges(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileBadgeRow>, crate::error::SocialError> {
        social_graph::get_profile_badges(&self.db, address, limit, offset).await
    }

    pub async fn get_following(
        &self,
        address: &str,
        query: &FollowsQuery,
    ) -> Result<(Vec<FollowDetail>, PaginationInfo), crate::error::SocialError> {
        social_graph::get_following(&self.db, address, query).await
    }

    pub async fn get_followers(
        &self,
        address: &str,
        query: &FollowsQuery,
    ) -> Result<(Vec<FollowDetail>, PaginationInfo), crate::error::SocialError> {
        social_graph::get_followers(&self.db, address, query).await
    }

    pub async fn get_follow_recommendations(
        &self,
        address: &str,
        query: &FollowsQuery,
    ) -> Result<(Vec<RecommendationDetail>, PaginationInfo), crate::error::SocialError> {
        social_graph::get_follow_recommendations(&self.db, address, query).await
    }

    pub async fn get_social_stats(
        &self,
        address: &str,
    ) -> Result<FollowStatsRow, crate::error::SocialError> {
        social_graph::get_social_stats(&self.db, address).await
    }

    pub async fn get_blocked_profiles(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BlockedProfileRow>, crate::error::SocialError> {
        social_graph::get_blocked_profiles(&self.db, address, limit, offset).await
    }

    pub async fn get_blocked_platforms(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BlockedPlatformRow>, crate::error::SocialError> {
        social_graph::get_blocked_platforms(&self.db, address, limit, offset).await
    }

    pub async fn check_following(
        &self,
        follower: &str,
        following: &str,
    ) -> Result<(bool, bool), crate::error::SocialError> {
        social_graph::check_following(&self.db, follower, following).await
    }

    pub async fn get_social_graph_chart_data(
        &self,
        query: &SocialGraphChartQuery,
    ) -> Result<SocialGraphChartData, crate::error::SocialError> {
        social_graph::get_social_graph_chart_data(&self.db, query).await
    }

    pub async fn check_profile_blocked(
        &self,
        blocker: &str,
        blocked: &str,
    ) -> Result<bool, crate::error::SocialError> {
        social_graph::check_profile_blocked(&self.db, blocker, blocked).await
    }

    pub async fn check_platform_blocked(
        &self,
        profile_address: &str,
        platform_id: &str,
    ) -> Result<bool, crate::error::SocialError> {
        social_graph::check_platform_blocked(&self.db, profile_address, platform_id).await
    }

    pub async fn list_badges(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileBadgeRow>, crate::error::SocialError> {
        social_graph::list_badges(&self.db, limit, offset).await
    }

    pub async fn get_badge_by_id(
        &self,
        badge_id: &str,
    ) -> Result<Option<ProfileBadgeRow>, crate::error::SocialError> {
        social_graph::get_badge_by_id(&self.db, badge_id).await
    }

    pub async fn list_platforms(
        &self,
        approved_only: bool,
        governance: Option<bool>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformRow>, crate::error::SocialError> {
        platform::list_platforms(&self.db, approved_only, governance, limit, offset).await
    }

    pub async fn get_platform_by_id(
        &self,
        platform_id: &str,
    ) -> Result<Option<PlatformRow>, crate::error::SocialError> {
        platform::get_platform_by_id(&self.db, platform_id).await
    }

    pub async fn get_platform_moderators(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformModeratorRow>, crate::error::SocialError> {
        platform::get_platform_moderators(&self.db, platform_id, limit, offset).await
    }

    pub async fn get_platform_approval(
        &self,
        platform_id: &str,
    ) -> Result<Option<PlatformApprovalRow>, crate::error::SocialError> {
        platform::get_platform_approval(&self.db, platform_id).await
    }

    pub async fn get_platform_blocked_profiles(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformBlockedProfileRow>, crate::error::SocialError> {
        platform::get_platform_blocked_profiles(&self.db, platform_id, limit, offset).await
    }

    pub async fn get_platform_members(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PlatformMemberRow>, crate::error::SocialError> {
        platform::get_platform_members(&self.db, platform_id, limit, offset).await
    }

    pub async fn check_platform_membership(
        &self,
        platform_id: &str,
        profile_address: &str,
    ) -> Result<bool, crate::error::SocialError> {
        platform::check_platform_membership(&self.db, platform_id, profile_address).await
    }

    pub async fn get_platform_events(
        &self,
        platform_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<PlatformEventRow>, i64), crate::error::SocialError> {
        platform::get_platform_events(&self.db, platform_id, limit, offset).await
    }

    pub async fn list_posts(
        &self,
        owner: Option<&str>,
        post_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostBasicRow>, crate::error::SocialError> {
        post::list_posts(&self.db, owner, post_type, limit, offset).await
    }

    pub async fn get_memory_account_by_owner(
        &self,
        owner: &str,
    ) -> Result<Option<MemoryAccountRow>, crate::error::SocialError> {
        memory::get_memory_account_by_owner(&self.db, owner).await
    }

    pub async fn list_sub_agents(
        &self,
        principal_owner: &str,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<memory::SubAgentListResponse, crate::error::SocialError> {
        memory::list_sub_agents(&self.db, principal_owner, active_only, limit, offset).await
    }

    pub async fn get_sub_agent(
        &self,
        derived_address: &str,
    ) -> Result<Option<SubAgentRow>, crate::error::SocialError> {
        memory::get_sub_agent(&self.db, derived_address).await
    }

    pub async fn get_sub_agent_by_object_id(
        &self,
        agent_object_id: &str,
    ) -> Result<Option<SubAgentRow>, crate::error::SocialError> {
        memory::get_sub_agent_by_object_id(&self.db, agent_object_id).await
    }

    pub async fn list_sub_agent_children(
        &self,
        parent_object_id: &str,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SubAgentRow>, crate::error::SocialError> {
        memory::list_sub_agent_children(
            &self.db,
            parent_object_id,
            active_only,
            limit,
            offset,
        )
        .await
    }

    pub async fn get_post_config(
        &self,
    ) -> Result<Option<PostConfigRow>, crate::error::SocialError> {
        post::get_post_config(&self.db).await
    }

    pub async fn get_trending_posts(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PostBasicRow>, crate::error::SocialError> {
        post::get_trending_posts(&self.db, limit, offset).await
    }

    pub async fn get_post_by_id(
        &self,
        post_id: &str,
    ) -> Result<Option<PostBasicRow>, crate::error::SocialError> {
        post::get_post_by_id(&self.db, post_id).await
    }

    pub async fn get_post_comments(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CommentRow>, crate::error::SocialError> {
        post::get_post_comments(&self.db, post_id, limit, offset).await
    }

    pub async fn get_post_reactions(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ReactionRow>, crate::error::SocialError> {
        post::get_post_reactions(&self.db, post_id, limit, offset).await
    }

    pub async fn get_post_reposts(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RepostRow>, crate::error::SocialError> {
        post::get_post_reposts(&self.db, post_id, limit, offset).await
    }

    pub async fn list_promotions(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PromotedPostRow>, crate::error::SocialError> {
        promotion::list_promotions(&self.db, limit, offset).await
    }

    pub async fn get_promotion_by_post_id(
        &self,
        post_id: &str,
    ) -> Result<Option<PromotedPostRow>, crate::error::SocialError> {
        promotion::get_promotion_by_post_id(&self.db, post_id).await
    }

    pub async fn get_promotion_views(
        &self,
        promotion_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PromotionViewRow>, crate::error::SocialError> {
        promotion::get_promotion_views(&self.db, promotion_id, limit, offset).await
    }

    pub async fn get_promotion_stats(
        &self,
        promotion_id: &str,
    ) -> Result<Option<PromotionStatsRow>, crate::error::SocialError> {
        promotion::get_promotion_stats(&self.db, promotion_id).await
    }

    pub async fn get_promotion_time_series(
        &self,
        promotion_id: &str,
        limit: i64,
    ) -> Result<Vec<PromotionTimeSeriesRow>, crate::error::SocialError> {
        promotion::get_promotion_time_series(&self.db, promotion_id, limit).await
    }

    pub async fn get_promotion_hourly(
        &self,
        promotion_id: &str,
        limit: i64,
    ) -> Result<Vec<PromotionHourlyRow>, crate::error::SocialError> {
        promotion::get_promotion_hourly(&self.db, promotion_id, limit).await
    }

    pub async fn get_top_performing_promotions(
        &self,
        limit: i64,
    ) -> Result<Vec<PromotedPostRow>, crate::error::SocialError> {
        promotion::get_top_performing_promotions(&self.db, limit).await
    }

    pub async fn get_spending_trends(
        &self,
        limit: i64,
    ) -> Result<Vec<PromotionTimeSeriesRow>, crate::error::SocialError> {
        promotion::get_spending_trends(&self.db, limit).await
    }

    pub async fn list_poc_badges(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocBadgeRow>, crate::error::SocialError> {
        poc::list_poc_badges(&self.db, limit, offset).await
    }

    pub async fn get_poc_badge_by_id(
        &self,
        badge_id: &str,
    ) -> Result<Option<PocBadgeRow>, crate::error::SocialError> {
        poc::get_poc_badge_by_id(&self.db, badge_id).await
    }

    pub async fn list_poc_revenue_redirections(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocRevenueRedirectionRow>, crate::error::SocialError> {
        poc::list_poc_revenue_redirections(&self.db, limit, offset).await
    }

    pub async fn list_poc_analysis_results(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocAnalysisResultRow>, crate::error::SocialError> {
        poc::list_poc_analysis_results(&self.db, limit, offset).await
    }

    pub async fn list_poc_disputes(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocDisputeRow>, crate::error::SocialError> {
        poc::list_poc_disputes(&self.db, limit, offset).await
    }

    pub async fn get_poc_dispute_by_id(
        &self,
        dispute_id: &str,
    ) -> Result<Option<PocDisputeRow>, crate::error::SocialError> {
        poc::get_poc_dispute_by_id(&self.db, dispute_id).await
    }

    pub async fn get_poc_dispute_votes(
        &self,
        dispute_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocDisputeVoteRow>, crate::error::SocialError> {
        poc::get_poc_dispute_votes(&self.db, dispute_id, limit, offset).await
    }

    pub async fn get_poc_analytics(&self) -> Result<serde_json::Value, crate::error::SocialError> {
        poc::get_poc_analytics(&self.db).await
    }

    pub async fn get_poc_configuration(
        &self,
    ) -> Result<Option<PocConfigRow>, crate::error::SocialError> {
        poc::get_poc_configuration(&self.db).await
    }

    pub async fn get_poc_beneficiary_vault_by_vault_id(
        &self,
        vault_id: &str,
    ) -> Result<Option<PocBeneficiaryVaultRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        myso_indexer_alt_social_reader::get_poc_beneficiary_vault_by_vault_id_for_conn(
            &mut conn, vault_id,
        )
        .await
        .map_err(|e| crate::error::SocialError::internal(e.to_string()))
    }

    pub async fn get_poc_beneficiary_vault_by_beneficiary_address(
        &self,
        beneficiary_address: &str,
    ) -> Result<Option<PocBeneficiaryVaultRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        myso_indexer_alt_social_reader::get_poc_beneficiary_vault_by_beneficiary_address_for_conn(
            &mut conn,
            beneficiary_address,
        )
        .await
        .map_err(|e| crate::error::SocialError::internal(e.to_string()))
    }

    pub async fn list_poc_beneficiary_vault_coin_balances(
        &self,
        vault_id: &str,
    ) -> Result<Vec<PocVaultCoinBalanceRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        myso_indexer_alt_social_reader::list_poc_vault_coin_balances_for_vault_for_conn(
            &mut conn, vault_id,
        )
        .await
        .map_err(|e| crate::error::SocialError::internal(e.to_string()))
    }

    pub async fn list_poc_vault_deposits_for_vault(
        &self,
        vault_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocVaultDepositRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        myso_indexer_alt_social_reader::list_poc_vault_deposits_for_vault_for_conn(
            &mut conn, vault_id, limit, offset,
        )
        .await
        .map_err(|e| crate::error::SocialError::internal(e.to_string()))
    }

    pub async fn list_poc_vault_claims_for_vault(
        &self,
        vault_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocVaultClaimRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        myso_indexer_alt_social_reader::list_poc_vault_claims_for_vault_for_conn(
            &mut conn, vault_id, limit, offset,
        )
        .await
        .map_err(|e| crate::error::SocialError::internal(e.to_string()))
    }

    pub async fn get_post_poc_badges(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocBadgeRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT badge_id, post_id, media_type, issued_by, issued_at,
                   COALESCE(revoked, false) AS revoked,
                   beneficiary_address, matched_anchor_id, media_index
            FROM (
                SELECT DISTINCT ON (badge_id) *
                FROM poc_badges
                WHERE post_id = $1
                ORDER BY badge_id, time DESC
            ) sub
            WHERE COALESCE(revoked, false) = false
            ORDER BY issued_at DESC
            LIMIT $2 OFFSET $3
        ";
        diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(post_id)
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .bind::<diesel::sql_types::BigInt, _>(offset)
            .load::<PocBadgeRow>(&mut conn)
            .await
            .map_err(|e| crate::error::SocialError::internal(e.to_string()))
    }

    pub async fn get_post_revenue_redirections(
        &self,
        post_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PocRevenueRedirectionRow>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT redirection_id, accused_post_id, original_post_id, redirect_percentage,
                   similarity_score, created_at, removed
            FROM (
                SELECT DISTINCT ON (redirection_id) *
                FROM poc_revenue_redirections
                WHERE accused_post_id = $1 OR original_post_id = $1
                ORDER BY redirection_id, time DESC
            ) sub
            WHERE COALESCE(removed, false) = false
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        ";
        diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(post_id)
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .bind::<diesel::sql_types::BigInt, _>(offset)
            .load::<PocRevenueRedirectionRow>(&mut conn)
            .await
            .map_err(|e| crate::error::SocialError::internal(e.to_string()))
    }
    pub async fn list_subscriptions(
        &self,
        subscriber: Option<&str>,
        service_id: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionInfo>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let query = "
            SELECT sub.subscription_id, sub.service_id, sub.subscriber, sub.created_at,
                   sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
                   sub.cancelled_at, s.monthly_fee, s.profile_owner,
                   p.username, p.display_name
            FROM (
                SELECT DISTINCT ON (subscription_id) *
                FROM profile_subscriptions
                WHERE ($1::text IS NULL OR subscriber = $1)
                  AND ($2::text IS NULL OR service_id = $2)
                ORDER BY subscription_id, time DESC
            ) sub
            JOIN profile_subscription_services s ON s.service_id = sub.service_id
            LEFT JOIN profiles p ON p.owner_address = s.profile_owner
            ORDER BY sub.expires_at DESC
            LIMIT $3 OFFSET $4
        ";
        let results = diesel::sql_query(query)
            .bind::<Nullable<Text>, _>(subscriber)
            .bind::<Nullable<Text>, _>(service_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionInfo>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn list_subscription_services(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionServiceInfo>, crate::error::SocialError> {
        subscription::list_subscription_services(&self.db, limit, offset).await
    }

    pub async fn list_subscription_revenue(
        &self,
        service_id: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProfileSubscriptionRevenueRow>, crate::error::SocialError> {
        subscription::list_subscription_revenue(&self.db, service_id, limit, offset).await
    }

    pub async fn get_subscriber_summary(
        &self,
        address: &str,
    ) -> Result<SubscriberSummaryRow, crate::error::SocialError> {
        subscription::get_subscriber_summary(&self.db, address).await
    }

    pub async fn list_vesting_wallets(
        &self,
        active_only: bool,
        owner: Option<&str>,
        limit: i64,
        offset: i64,
        page: i64,
    ) -> Result<VestingWalletsResponse, crate::error::SocialError> {
        vesting::list_vesting_wallets(&self.db, active_only, owner, limit, offset, page).await
    }

    pub async fn get_vesting_wallet_by_id(
        &self,
        wallet_id: &str,
    ) -> Result<Option<VestingWalletWithStatus>, crate::error::SocialError> {
        vesting::get_vesting_wallet_by_id(&self.db, wallet_id).await
    }

    pub async fn get_vesting_wallet_events(
        &self,
        wallet_id: &str,
        limit: i64,
        offset: i64,
        page: i64,
    ) -> Result<VestingEventsResponse, crate::error::SocialError> {
        vesting::get_vesting_wallet_events(&self.db, wallet_id, limit, offset, page).await
    }

    pub async fn get_vesting_claimable(
        &self,
        wallet_id: &str,
    ) -> Result<Option<ClaimableResponse>, crate::error::SocialError> {
        vesting::get_vesting_claimable(&self.db, wallet_id).await
    }

    pub async fn get_user_vesting_wallets(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
        page: i64,
    ) -> Result<VestingWalletsResponse, crate::error::SocialError> {
        vesting::get_user_vesting_wallets(&self.db, address, limit, offset, page).await
    }

    pub async fn list_vesting_events(
        &self,
        limit: i64,
        offset: i64,
        page: i64,
        owner_address: Option<&str>,
    ) -> Result<VestingEventsResponse, crate::error::SocialError> {
        vesting::list_vesting_events(&self.db, limit, offset, page, owner_address).await
    }

    pub async fn get_vesting_analytics(
        &self,
    ) -> Result<VestingAnalyticsResponse, crate::error::SocialError> {
        vesting::get_vesting_analytics(&self.db).await
    }

    pub async fn get_vesting_leaderboard(
        &self,
        limit: i64,
        offset: i64,
        page: i64,
    ) -> Result<VestingLeaderboardResponse, crate::error::SocialError> {
        vesting::get_vesting_leaderboard(&self.db, limit, offset, page).await
    }

    pub async fn get_spt_pool_by_associated_id(
        &self,
        associated_id: &str,
    ) -> Result<Option<SptPoolRow>, crate::error::SocialError> {
        spt::get_spt_pool_by_associated_id(&self.db, associated_id).await
    }

    pub async fn get_spt_popular(
        &self,
        limit: i64,
    ) -> Result<Vec<SptPoolRow>, crate::error::SocialError> {
        spt::get_spt_popular(&self.db, limit).await
    }

    pub async fn get_spt_user_holdings(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptUserHoldingItem>, crate::error::SocialError> {
        spt::get_spt_user_holdings(&self.db, address, limit, offset).await
    }

    pub async fn get_spt_user_holdings_with_reservations(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptUserHoldingItem>, crate::error::SocialError> {
        spt::get_spt_user_holdings_with_reservations(&self.db, address, limit, offset).await
    }

    pub async fn get_spt_user_reservations(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SptUserHoldingItem>, crate::error::SocialError> {
        spt::get_spt_user_reservations(&self.db, address, limit, offset).await
    }

    pub async fn search(
        &self,
        q: &str,
        limit: i64,
    ) -> Result<serde_json::Value, crate::error::SocialError> {
        search::search(&self.db, q, limit).await
    }

    pub async fn search_profiles(
        &self,
        q: &str,
        limit: i64,
    ) -> Result<Vec<Profile>, crate::error::SocialError> {
        search::search_profiles(&self.db, q, limit).await
    }

    pub async fn search_posts(
        &self,
        q: &str,
        limit: i64,
    ) -> Result<Vec<PostBasicRow>, crate::error::SocialError> {
        search::search_posts(&self.db, q, limit).await
    }
}
