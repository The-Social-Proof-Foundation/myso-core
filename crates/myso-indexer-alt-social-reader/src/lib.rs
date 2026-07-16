// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod governance;

pub use governance::{DelegateRatingViewerTarget, delegate_rating_viewer_lookup_key};
pub mod access;
pub mod ai_credit;
pub mod enterprise;
pub mod insurance;
pub mod memory;
pub mod messaging;
mod metrics;
pub mod mydata;
pub mod org_leaderboard;
pub mod org_stats;
pub mod organization;
pub mod pg_reader;
pub mod platform;
pub mod pnl;
pub mod poc;
pub mod poc_username_beneficiary;
pub mod post;
pub mod profile;
pub mod promotion;
pub mod revenue;
pub mod social_graph;
pub mod spot;
pub mod spt;
pub mod subscription;
pub mod username;
pub mod vesting;

pub use access::{
    MyDataAccessConfigurationKind, PostAccessKind, ResolvedPostAccess,
    resolve_mydata_access_configuration_kind, resolve_post_access,
};
pub use ai_credit::AiCreditConfigRow;
pub use enterprise::{AgentSpendBreakdownEntry, AuditLogFilter};
pub use insurance::{
    InsuranceCoverageRouteRow, InsuranceModuleEventRow, InsurancePolicyEventHistoryRow,
    InsuranceRouteFillRow, InsuranceRouterConfigRow, InsuranceUserExposureAggRow,
    InsuranceVaultExposureRow, InsuranceVaultTransactionRow,
};
pub use memory::MemoryConfigRow;
pub use memory::{SocialAttributionRow, SubAgentListResult};
pub use messaging::{MessagingAgentGroupRow, MessagingConfigRow, PaidMessageEscrowRow};
pub use metrics::standalone_reader_metrics;
pub use myso_indexer_alt_social_schema::models::{
    AgenticOrganizationRow, MemoryAccountRow, SubAgentRow,
};
pub use myso_indexer_alt_social_schema::models::{
    DelegateRow, GovernanceRegistryRow, GovernanceStatsRow, InsurancePolicyRow, InsuranceVaultRow,
    MyDataBroadPoolRow, MyDataClaimRow, MyDataDistributionRoundRow, MyDataListingSubPoolRow,
    MyDataMerkleRootRow, MyDataPurchaseRow, MyDataRecordRow, MyDataSnapshotAnchorRow,
    MyDataSnapshotEscrowRow, MyDataSubPoolRow, PlatformRevenueSummaryRow, PromotedPostRow,
    PromotionHourlyRow, PromotionStatsRow, PromotionTimeSeriesRow, PromotionViewRow, ProposalRow,
    SpotBetRow, SpotBetWithdrawalRow, SpotPayoutRow, SpotRecordRow, SpotRefundRow,
    SpotResolutionRow,
};
pub use myso_indexer_alt_social_schema::models::{
    PocAnalysisResultRow, PocBadgeRow, PocBeneficiaryVaultRow, PocConfigRow,
    PocCreatorIdentityLinkRow, PocDisputeRow, PocDisputeVoteRow, PocRevenueRedirectionRow,
    PocUsernameBeneficiaryRow, PocVaultClaimRow, PocVaultCoinBalanceRow, PocVaultDepositRow,
};
pub use myso_indexer_alt_social_schema::models::{
    PostDeletionEventRow, PostModerationEventRow, SptHoldingRow, SptPoolRow, SptPriceHistory,
    SptTransaction,
};
pub use org_leaderboard::{
    OrganizationCategoryInfo, OrganizationLeaderboardEntry, OrganizationLeaderboardResult,
    OrganizationLeaderboardSort, org_type_from_slug, organization_categories,
};
pub use org_stats::{OrganizationStatistics, OrganizationStatsWindow};
pub use organization::AgenticOrganizationListResult;
pub use pg_reader::SocialPgReader;
pub use platform::{
    PlatformBlockedProfileRow, PlatformConfigRow, PlatformRow, PlatformUserAccessRow,
};
pub use pnl::{ProfilePnLWindow, ProfilePnLWindowResult, get_profile_pnl_for_windows};
pub use poc::{
    get_poc_beneficiary_vault_by_beneficiary_address_for_conn,
    get_poc_beneficiary_vault_by_vault_id_for_conn, list_poc_vault_claims_for_vault_for_conn,
    list_poc_vault_coin_balances_for_vault_for_conn, list_poc_vault_deposits_for_vault_for_conn,
};
pub use poc_username_beneficiary::{
    get_by_id as get_poc_username_beneficiary_by_id,
    get_by_username as get_poc_username_beneficiary_by_username,
    get_creator_identity_link as get_poc_creator_identity_link,
    get_creator_identity_link_by_wallet as get_poc_creator_identity_link_by_wallet,
    get_poc_creator_identity_link_for_conn, get_poc_username_beneficiary_by_id_for_conn,
    get_poc_username_beneficiary_by_username_for_conn,
    has_active as has_active_poc_username_beneficiary, is_username_available_for_registration,
    list_username_beneficiaries, list_username_beneficiaries_for_conn,
};
pub use post::{
    CommentRow, PostReportRow, PostRow, PostTransferRow, ReactionRow, RepostRow, TipRow,
};
pub use profile::{
    ProfileBadgeRow, ProfileByAddressResponse, ProfileConfigRow, ReservationStatus,
    SelectedBadgeInfo, SocialProofTokenInfo, UniversalUserResult,
};
pub use social_graph::{
    BlockedPlatformRow, BlockedProfileRow, DEFAULT_MUTUAL_CONNECTIONS_LIMIT,
    MAX_MUTUAL_CONNECTIONS_LIMIT, ProfileSummaryRow, ViewerSocialContext,
    clamp_mutual_connections_limit, get_follow_recommendations_standalone,
};
pub use spot::{
    SpotClaimEarningsRow, SpotClaimRow, SpotClaimVerdictRow, SpotCreatorStatsRow,
    SpotMarketEarningsRow, SpotMarketRow, SpotPendingCreatorPayoutRow, SpotPostAnalysisRow,
    SpotPostEarningsRow, SpotRouteRow,
};
pub use spt::{
    SptReservationVolumeBucket, SptReservationVolumeInterval, SptSortBy, SptTransactionsWithViewer,
};
pub use subscription::{
    ProfileSubscriptionPlanRow, ProfileSubscriptionRow, ProfileSubscriptionServiceRow,
    SubscriptionConfigRow,
};
pub use username::{
    InvalidUsername, UsernameAvailabilityDetail, UsernameRegistryEntry, canonical_username_key,
};

/// Combined username registry + PoC beneficiary availability for REST services.
pub async fn get_username_availability_for_db(
    db: &myso_pg_db::Db,
    username: &str,
    exclude_address: Option<&str>,
) -> anyhow::Result<UsernameAvailabilityDetail> {
    let mut conn = db.connect().await?;
    username::get_username_availability(
        &mut conn,
        username,
        exclude_address,
        standalone_reader_metrics(),
    )
    .await
}
pub use vesting::{
    VestingLeaderboardEntry, VestingLeaderboardResponse, VestingWalletRow, VestingWalletWithStatus,
};

/// Standalone DB access for services that share this crate's SQL (e.g. `myso-social-server`).
pub async fn get_agentic_organization_for_db(
    db: &myso_pg_db::Db,
    organization_id: &str,
) -> anyhow::Result<Option<AgenticOrganizationRow>> {
    let mut conn = db.connect().await?;
    organization::get_agentic_organization(&mut conn, organization_id, standalone_reader_metrics())
        .await
}

pub async fn list_agentic_organizations_by_owner_for_db(
    db: &myso_pg_db::Db,
    principal_owner: &str,
    org_type: Option<i16>,
    active_only: bool,
    limit: i64,
    offset: i64,
) -> anyhow::Result<AgenticOrganizationListResult> {
    let mut conn = db.connect().await?;
    organization::list_agentic_organizations_by_owner(
        &mut conn,
        principal_owner,
        org_type,
        active_only,
        limit,
        offset,
        standalone_reader_metrics(),
    )
    .await
}

pub async fn get_organization_statistics_for_db(
    db: &myso_pg_db::Db,
    organization_id: &str,
    window: OrganizationStatsWindow,
) -> anyhow::Result<Option<OrganizationStatistics>> {
    let mut conn = db.connect().await?;
    let org = organization::get_agentic_organization(
        &mut conn,
        organization_id,
        standalone_reader_metrics(),
    )
    .await?;
    let Some(org) = org else {
        return Ok(None);
    };
    org_stats::get_organization_statistics(&mut conn, &org, window, standalone_reader_metrics())
        .await
        .map(Some)
}

pub async fn get_organization_leaderboard_for_db(
    db: &myso_pg_db::Db,
    sort: OrganizationLeaderboardSort,
    org_type: i16,
    window: OrganizationStatsWindow,
    limit: i64,
    offset: i64,
) -> anyhow::Result<OrganizationLeaderboardResult> {
    let mut conn = db.connect().await?;
    org_leaderboard::get_organization_leaderboard(
        &mut conn,
        sort,
        org_type,
        window,
        limit,
        offset,
        standalone_reader_metrics(),
    )
    .await
}

pub async fn list_agent_spend_breakdown_for_db(
    db: &myso_pg_db::Db,
    organization_id: &str,
    window: OrganizationStatsWindow,
    limit: i64,
) -> anyhow::Result<Vec<AgentSpendBreakdownEntry>> {
    let mut conn = db.connect().await?;
    enterprise::list_agent_spend_breakdown(
        &mut conn,
        organization_id,
        window,
        limit,
        standalone_reader_metrics(),
    )
    .await
}

pub async fn list_spend_approvals_by_org_for_db(
    db: &myso_pg_db::Db,
    organization_id: &str,
    status: Option<&str>,
    agent_object_id: Option<&str>,
    limit: i64,
) -> anyhow::Result<Vec<myso_indexer_alt_social_schema::models::AiCreditSpendApprovalRow>> {
    let mut conn = db.connect().await?;
    enterprise::list_spend_approvals_by_org(
        &mut conn,
        organization_id,
        status,
        agent_object_id,
        limit,
        standalone_reader_metrics(),
    )
    .await
}
