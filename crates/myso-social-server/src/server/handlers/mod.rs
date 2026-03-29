// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

mod governance;
mod health;
mod insurance;
mod mydata;
mod platforms;
mod poc;
mod posts;
mod profiles;
mod promotions;
mod revenue;
mod search;
mod social_graph;
mod spot;
mod spt;
mod subscription;
mod system;
mod upgrade;
mod vesting;

pub use governance::{
    get_governance_anonymous_voting_trends, get_governance_delegate,
    get_governance_delegate_proposals, get_governance_delegate_ratings, get_governance_proposal,
    get_governance_proposal_anonymous_stats, get_governance_proposal_anonymous_votes,
    get_governance_proposal_community_votes, get_governance_proposal_decryption_failures,
    get_governance_registry, get_governance_registry_by_platform, list_governance_delegates,
    list_governance_events, list_governance_nominees, list_governance_proposals,
    list_governance_registries,
};
pub use health::health_check;
pub use insurance::{
    get_insurance_config, get_insurance_policy, get_insurance_vault, get_insurance_vault_exposures,
    list_insurance_market_policies, list_insurance_policies, list_insurance_vault_transactions,
    list_insurance_vaults,
};
pub use mydata::{
    get_creator_mydata, get_mydata_access_analytics, get_mydata_access_logs, get_mydata_by_id,
    get_mydata_configuration, get_mydata_purchases, get_mydata_revenue,
    get_mydata_revenue_timeline, get_mydata_stats, get_mydata_subscriptions, get_popular_mydata,
    list_mydata,
};
pub use platforms::{
    check_platform_membership, get_platform_approval, get_platform_blocked, get_platform_by_id,
    get_platform_events, get_platform_members, get_platform_moderators, list_platforms,
    list_platforms_approved,
};
pub use poc::{
    get_poc_analytics, get_poc_badge_by_id, get_poc_configuration, get_poc_dispute_by_id,
    get_poc_dispute_votes, list_poc_analysis_results, list_poc_badges, list_poc_disputes,
    list_poc_revenue_redirections,
};
pub use posts::{
    get_post_by_id, get_post_comments, get_post_config, get_post_poc_badges, get_post_promotion,
    get_post_reactions, get_post_reposts, get_post_revenue_redirections, get_post_transfers,
    get_trending_posts, list_posts,
};
pub use profiles::{
    get_profile_badges, get_profile_blocked, get_profile_blocked_platforms,
    get_profile_blocking_history, get_profile_by_address, get_profile_by_username,
    get_profile_events, get_profile_followers, get_profile_following, get_profile_offers,
    get_profile_platform_events, get_profile_platform_memberships, get_profile_posts,
    get_profile_sale_fees, get_profile_social_stats, latest_profiles,
};
pub use promotions::{
    get_promotion_hourly, get_promotion_stats, get_promotion_time_series, get_promotion_views,
    get_spending_trends, get_top_performing_promotions, list_promotions,
};
pub use revenue::{
    get_creator_revenue_stats, get_platform_revenue_stats, get_revenue_chart_data,
    get_revenue_dashboard, get_revenue_leaderboard, get_service_performance,
    get_subscription_analytics, get_treasury_current, get_treasury_history, get_unified_revenue,
};
pub use search::search;
pub use social_graph::{
    check_platform_blocked, check_profile_blocked, check_social_graph_following, get_badge_by_id,
    get_social_graph_chart_data, list_badges,
};
pub use spot::{
    get_spot_configuration, get_spot_record, list_spot_bets, list_spot_payouts, list_spot_refunds,
};
pub use spt::{
    get_spt_analytics_top_performers, get_spt_config, get_spt_creator_revenue_streams,
    get_spt_liquidity_profile, get_spt_market_sentiment, get_spt_pool,
    get_spt_pool_by_associated_id, get_spt_pool_holdings, get_spt_pool_price_history,
    get_spt_pool_revenue, get_spt_pool_transactions, get_spt_popular,
    get_spt_portfolio_performance, get_spt_reservation_pool, get_spt_user_holdings,
    get_spt_reservation_pool_volume_history, get_spt_user_reservations, list_spt_pools,
    list_spt_reservation_pool_reservations, list_spt_reservation_pools,
};
pub use subscription::{
    check_subscription_access, get_profile_subscription_service, get_subscriber_summary,
    get_subscription_by_id, get_subscription_revenue_by_service, get_subscription_status,
    list_profile_subscription_services, list_subscription_revenue, list_subscription_services,
    list_subscriptions, list_subscriptions_by_subscriber,
};
pub use system::{check_username_availability, get_system_stats};
pub use upgrade::{list_object_migrated_events, list_upgrade_events};
pub use vesting::{
    get_user_vesting_wallets, get_vesting_analytics, get_vesting_claimable,
    get_vesting_leaderboard, get_vesting_wallet, get_vesting_wallet_events, list_vesting_events,
    list_vesting_wallets, list_vesting_wallets_active,
};
