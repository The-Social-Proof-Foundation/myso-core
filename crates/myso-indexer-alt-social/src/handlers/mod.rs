// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Social events pipeline: processes myso-social events from checkpoints into social tables.
//!
//! Filters events by MYSO_SOCIAL_PACKAGE_ID, routes by module/event name, and inserts into
//! profiles, social_graph_relationships, social_graph_events, etc.

mod ai_credit;
mod ai_credit_handler;
mod blocking;
mod blocking_handler;
mod common;
mod events;
mod governance;
mod governance_handler;
mod insurance;
mod insurance_handler;
mod memory;
mod memory_handler;
mod messaging;
mod messaging_handler;
mod mydata;
mod mydata_handler;
mod mydata_object;
mod organization_stats;
pub mod organization_stats_rollup;
mod platform;
mod platform_handler;
mod poc;
mod post;
mod post_mydata;
mod posts_handler;
mod profile;
mod profiles_handler;
mod social_graph;
mod social_graph_handler;
mod spot;
mod spot_handler;
mod spt;
mod spt_handler;
mod sub_agent_registry_handler;
mod subscription;
mod subscription_handler;
mod upgrade;
mod upgrade_handler;

use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    GovernanceRegistryPanelBoundaryUpdate, GovernanceRegistryUpdate, NewAgentMemoryVault,
    NewAgenticOrganization, NewAiCreditAgentBudget, NewAiCreditBalance, NewAiCreditConfig,
    NewAiCreditEvent, NewAiCreditSpendApproval, NewAnonymousVote, NewAuditLog, NewBlockedEvent,
    NewBlockedProfile, NewComment, NewCommunityVote, NewDelegate, NewDelegateRating,
    NewDelegateVote, NewDeletionEvent, NewEcosystemTreasury, NewGovernanceEvent,
    NewGovernanceRegistry, NewInsuranceCoverageRoute, NewInsuranceEventLog,
    NewInsuranceMarketExposure, NewInsurancePolicy, NewInsurancePolicyEvent, NewInsuranceRouteFill,
    NewInsuranceRouterConfig, NewInsuranceUserExposure, NewInsuranceVault,
    NewInsuranceVaultTransaction, NewMemoryAccount, NewMemoryConfig, NewMessagingAgentGroup,
    NewMessagingConfig, NewModerationEvent, NewMyDataAccessLog, NewMyDataBroadPool, NewMyDataClaim,
    NewMyDataConfig, NewMyDataData, NewMyDataDistributionRound, NewMyDataListingSubPool,
    NewMyDataMerkleRoot, NewMyDataPurchase, NewMyDataRegistry, NewMyDataRevenue,
    NewMyDataSnapshotAnchor, NewMyDataSubPool, NewMyDataSubscription, NewNominatedDelegate,
    NewObjectMigratedEvent, NewOrgInvitation, NewOrgMemoryPermission, NewOrgRole,
    NewOrgRoleAssignment, NewOrganizationEvent, NewPaidMessageEscrow, NewPlatform, NewPlatformBlockedProfile, NewPlatformConfig,
    NewPlatformEvent, NewPlatformMembership, NewPlatformModerator, NewPlatformModeratorPermission,
    NewPlatformTokenAirdrop, NewPocAnalysisResult, NewPocBadge, NewPocConfiguration,
    NewPocCreatorIdentityLink, NewPocDispute, NewPocDisputeVote, NewPocRevenueRedirection,
    NewPocUsernameBeneficiary, NewPocUsernameBeneficiaryEvent, NewPost, NewPostTransfer,
    NewProfile, NewProfileBadge, NewProfileConfig, NewProfileEvent, NewUsernameListing,
    NewUsernameOffer, NewUsernameSaleFee, NewProfileSubscription, NewProfileSubscriptionService, NewProposal,
    NewReaction, NewReport, NewRepost, NewRewardDistribution, NewSocialGraphEvent,
    NewSocialGraphRelationship, NewSocialProofTokensConfig, NewSocialProofTokensEvent, NewSpotBet,
    NewSpotBetWithdrawal, NewSpotConfig, NewSpotEventLog, NewSpotPayout, NewSpotRecord,
    NewSpotRefund, NewSpotResolution, NewSptExchangeConfig, NewSptHolding, NewSptPool,
    NewSptPriceHistory, NewSptReservation, NewSptReservationPool, NewSptTransaction,
    NewSubAgentEvent, NewSubscriptionConfig, NewSubscriptionEvent, NewTip, NewUnifiedRevenue,
    NewUpgradeEvent, NewUsernameRegistry, NewVestingEvent, NewVestingWallet,
    NewVoteDecryptionFailure, ProposalUpdateSet,
};

pub use ai_credit_handler::AiCreditHandler;
pub use blocking_handler::BlockingHandler;
pub use governance_handler::GovernanceHandler;
pub use insurance_handler::InsuranceHandler;
pub use memory_handler::MemoryHandler;
pub use messaging_handler::MessagingHandler;
pub use mydata_handler::MyDataHandler;
pub use platform_handler::PlatformHandler;
pub use posts_handler::PostsHandler;
pub use profiles_handler::ProfilesHandler;
pub use social_graph_handler::SocialGraphHandler;
pub use spot_handler::SpotHandler;
pub use spt_handler::SptHandler;
pub use sub_agent_registry_handler::SubAgentRegistryHandler;
pub use subscription_handler::SubscriptionHandler;
pub use upgrade_handler::UpgradeHandler;

#[derive(Debug, Clone)]
pub enum SocialEventRow {
    Profile(NewProfile),
    ProfileUpdate(ProfileUpdate),
    ProfileXUsernameUpdate {
        profile_id: String,
        owner_address: String,
        x_username: Option<String>,
    },
    UsernameRegistryUpsert(NewUsernameRegistry),
    UsernameRegistryDelete {
        username: String,
    },
    UsernameRegistryReassign {
        username: String,
        new_profile_id: String,
        transaction_id: String,
    },
    ProfileUsernameSet {
        profile_id: String,
        username: String,
    },
    ProfileUsernameClear {
        profile_id: String,
    },
    EcosystemTreasury(NewEcosystemTreasury),
    ProfileConfig(NewProfileConfig),
    SocialGraphRelationship(NewSocialGraphRelationship),
    SocialGraphEvent(NewSocialGraphEvent),
    SocialGraphUnfollow {
        follower_address: String,
        following_address: String,
    },
    BlockedEvent(NewBlockedEvent),
    BlockedProfile(NewBlockedProfile),
    BlockedProfileDelete {
        blocker_address: String,
        blocked_address: String,
    },
    ProfileEvent(NewProfileEvent),
    UsernameListing(NewUsernameListing),
    UsernameListingStatusUpdate {
        username: String,
        status: String,
        cancelled_at: Option<i64>,
        transaction_id: String,
    },
    UsernameOffer(NewUsernameOffer),
    UsernameSaleFee(NewUsernameSaleFee),
    ProfileBadge(NewProfileBadge),
    ProfileBadgeRevoke {
        profile_id: String,
        badge_id: String,
        revoked_at: i64,
        revoked_by: String,
    },
    GovernanceRegistry(NewGovernanceRegistry),
    GovernanceRegistryUpdate(GovernanceRegistryUpdate),
    GovernanceRegistryPanelBoundary(GovernanceRegistryPanelBoundaryUpdate),
    NominatedDelegate(NewNominatedDelegate),
    Delegate(NewDelegate),
    Proposal(NewProposal),
    ProposalUpdate {
        proposal_id: String,
        set: ProposalUpdateSet,
        governance_event: Option<(String, serde_json::Value, String)>,
        submitter_filter: Option<String>,
    },
    DelegateRating(NewDelegateRating),
    DelegateVote(NewDelegateVote),
    CommunityVote(NewCommunityVote),
    RewardDistribution(NewRewardDistribution),
    GovernanceEvent(NewGovernanceEvent),
    GovernanceEventFromProposal {
        proposal_id: String,
        event_type: String,
        event_data: serde_json::Value,
        event_id: String,
        anonymous_voting_related: Option<bool>,
    },
    AnonymousVote(NewAnonymousVote),
    VoteDecryptionFailure(NewVoteDecryptionFailure),
    Post(NewPost),
    Comment(NewComment),
    Reaction(NewReaction),
    RemoveReaction {
        object_id: String,
        user_address: String,
        reaction_text: String,
        is_post: bool,
    },
    Repost(NewRepost),
    Tip(NewTip),
    ModerationEvent(NewModerationEvent),
    Report(NewReport),
    DeletionEvent(NewDeletionEvent),
    PostCommentCountIncrement {
        post_id: String,
        delta: i64,
    },
    PostCommentCountDecrementByComment {
        comment_id: String,
        owner: String,
    },
    ProfilePostCountIncrement {
        owner_address: String,
    },
    ProfilePostCountDecrement {
        owner_address: String,
    },
    PostRepostCountIncrement {
        original_id: String,
        is_original_post: bool,
    },
    PostTipsReceivedIncrement {
        object_id: String,
        /// Tip recipient; must match `posts.owner` or `comments.owner` for the row
        /// updated by on-chain `tips_received` (excludes PoC redirect to `original_creator`).
        recipient: String,
        amount: i64,
        is_post: bool,
    },
    PostModerationUpdate {
        object_id: String,
        removed: bool,
        moderated_by: String,
    },
    PostDeletedAtUpdate {
        object_id: String,
        owner: String,
        deleted_at: i64,
    },
    CommentDeletedAtUpdate {
        object_id: String,
        owner: String,
        deleted_at: i64,
    },
    PostContentUpdate {
        object_id: String,
        content: String,
        media_urls: Option<serde_json::Value>,
        mentions: Option<serde_json::Value>,
        metadata_json: Option<serde_json::Value>,
        is_post: bool,
        updated_at: i64,
    },
    PostOwnerUpdate {
        object_id: String,
        new_owner: String,
        is_post: bool,
    },
    PostTransfer(NewPostTransfer),
    PostConfig {
        updated_by: String,
        max_content_length: i64,
        max_media_urls: i64,
        max_mentions: i64,
        max_metadata_size: i64,
        max_description_length: i64,
        max_reaction_length: i64,
        commenter_tip_percentage: i64,
        repost_tip_percentage: i64,
        min_promotion_amount: i64,
        max_promotion_amount: i64,
        min_view_duration_ms: i64,
        version: Option<i64>,
        updated_at: i64,
        transaction_id: String,
    },
    PromotedPost {
        post_id: String,
        owner: String,
        profile_id: String,
        payment_per_view: i64,
        total_budget: i64,
        created_at: i64,
        transaction_id: String,
    },
    PromotionView {
        promotion_id: String,
        viewer: String,
        payment_amount: i64,
        view_duration: i64,
        platform_id: String,
        timestamp: i64,
        transaction_id: String,
    },
    PromotionStatusEvent {
        promotion_id: String,
        toggled_by: String,
        new_status: bool,
        timestamp: i64,
        transaction_id: String,
    },
    PromotionBudgetEvent {
        promotion_id: String,
        owner: String,
        withdrawn_amount: i64,
        timestamp: i64,
        transaction_id: String,
    },
    Platform(NewPlatform),
    PlatformConfig(NewPlatformConfig),
    PlatformUpdate {
        platform_id: String,
        name: String,
        tagline: String,
        description: Option<String>,
        logo: Option<String>,
        terms_of_service: Option<String>,
        privacy_policy: Option<String>,
        platform_names: Option<serde_json::Value>,
        links: Option<serde_json::Value>,
        cover_photo: Option<String>,
        media_previews: Option<serde_json::Value>,
        status: i16,
        release_date: Option<String>,
        shutdown_date: Option<String>,
        updated_at: chrono::NaiveDateTime,
        primary_category: String,
        secondary_category: Option<String>,
    },
    PlatformApprovalChange {
        platform_id: String,
        is_approved: bool,
        approved_by: String,
        changed_at: chrono::NaiveDateTime,
    },
    PlatformModerator(NewPlatformModerator),
    PlatformModeratorRemove {
        platform_id: String,
        moderator_address: String,
    },
    PlatformModeratorPermissionGrant(NewPlatformModeratorPermission),
    PlatformModeratorPermissionRevoke {
        platform_id: String,
        moderator_address: String,
        permission_type: String,
        revoked_at: chrono::NaiveDateTime,
    },
    PlatformModeratorPermissionRevokeAll {
        platform_id: String,
        moderator_address: String,
        revoked_at: chrono::NaiveDateTime,
    },
    PlatformBlockedProfile(NewPlatformBlockedProfile),
    PlatformBlockedProfileRemove {
        platform_id: String,
        wallet_address: String,
    },
    PlatformMembership(NewPlatformMembership),
    PlatformMembershipRemove {
        platform_id: String,
        wallet_address: String,
    },
    PlatformTokenAirdrop(NewPlatformTokenAirdrop),
    PlatformEvent(NewPlatformEvent),
    PlatformDeleted {
        platform_id: String,
        deleted_at: chrono::NaiveDateTime,
    },
    PocBadge(NewPocBadge),
    PocAnalysisResult(NewPocAnalysisResult),
    PocRevenueRedirection(NewPocRevenueRedirection),
    PocDispute(NewPocDispute),
    PostPocDisputesSubmitted {
        post_id: String,
        poc_disputes_submitted: i16,
    },
    PocDisputeVote(NewPocDisputeVote),
    PocConfiguration(NewPocConfiguration),
    PostPocUpdate {
        post_id: String,
        poc_reasoning: Option<String>,
        poc_evidence_urls: Option<serde_json::Value>,
        poc_similarity_score: Option<i64>,
        poc_media_type: Option<i16>,
        poc_oracle_address: Option<String>,
        poc_analyzed_at: Option<i64>,
    },
    PostPocResultApplied {
        post_id: String,
        poc_outcome: i16,
        poc_redirection_kind: i16,
        similarity_detected: bool,
        timestamp_ms: i64,
    },
    PostPocBadgePointer {
        post_id: String,
        poc_badge_object_id: String,
    },
    PocBeneficiaryVaultDeposit {
        vault_id: String,
        vault_routing_key: String,
        coin_type: String,
        amount: i64,
        source_post_id: Option<String>,
        timestamp_ms: i64,
        transaction_id: String,
    },
    PocBeneficiaryVaultClaimed {
        vault_id: String,
        vault_routing_key: String,
        coin_type: String,
        referrer_address: Option<String>,
        treasury_amount: i64,
        referrer_amount: i64,
        beneficiary_amount: i64,
        join_referral_applied: bool,
        claim_kind: Option<String>,
        timestamp_ms: i64,
        transaction_id: String,
    },
    PocUsernameBeneficiary(NewPocUsernameBeneficiary),
    PocUsernameBeneficiaryClaimed {
        beneficiary_id: String,
        username: String,
        profile_id: String,
        claimed_by: String,
        wallet: String,
        oracle_evidence_hash: String,
        claimed_at_ms: i64,
        transaction_id: String,
    },
    PocUsernameBeneficiaryEnded {
        beneficiary_id: String,
        username: String,
        ended_by: String,
        end_reason_code: i16,
        swept_mys_amount: i64,
        ended_at_ms: i64,
        transaction_id: String,
    },
    PocUsernameBeneficiaryJoinReferralPaid {
        vault_id: String,
        join_referrer: Option<String>,
        join_referral_paid_at_ms: i64,
        transaction_id: String,
    },
    PocCreatorIdentityLink(NewPocCreatorIdentityLink),
    PocUsernameBeneficiaryEvent(NewPocUsernameBeneficiaryEvent),
    PostRevenueRedirectUpdate {
        post_id: String,
        revenue_redirect_to: String,
        revenue_redirect_percentage: i64,
        poc_redirection_kind: i16,
    },
    PocDisputeResolved {
        dispute_id: String,
        post_id: String,
        resolution: i16,
        winning_side: i16,
        total_winning_stake: i64,
        total_losing_stake: i64,
        resolved_at: i64,
        badge_revoked: bool,
        redirection_removed: bool,
        quorum_met: bool,
    },
    PocVoteRewardClaimed {
        dispute_id: String,
        voter: String,
        reward_amount: i64,
    },
    MyDataData(NewMyDataData),
    MyDataPurchase(NewMyDataPurchase),
    MyDataSubscription(NewMyDataSubscription),
    MyDataRevenue(NewMyDataRevenue),
    MyDataAccessLog(NewMyDataAccessLog),
    MyDataRegistry(NewMyDataRegistry),
    MyDataRegistryUpdate {
        mydata_id: String,
        owner: String,
        unregistered_at: i64,
        transaction_id: String,
    },
    MyDataConfig(NewMyDataConfig),
    MyDataContentUpdate {
        mydata_id: String,
        last_updated: i64,
        transaction_id: String,
    },
    MyDataAccessRevoke {
        mydata_id: String,
        user: String,
        access_type: String,
        revoked_at: i64,
        revoked_by: String,
        transaction_id: String,
    },
    MyDataBroadPool(NewMyDataBroadPool),
    MyDataSubPool(NewMyDataSubPool),
    MyDataListingSubPoolsReplace {
        listing_id: String,
        rows: Vec<NewMyDataListingSubPool>,
    },
    MyDataSnapshotAnchor(NewMyDataSnapshotAnchor),
    MyDataDistributionRound(NewMyDataDistributionRound),
    MyDataMerkleRoot(NewMyDataMerkleRoot),
    MyDataClaim(NewMyDataClaim),
    InsuranceConfig(insurance::InsuranceConfigSnapshot),
    InsuranceRouterConfig(NewInsuranceRouterConfig),
    InsuranceVault(NewInsuranceVault),
    InsuranceVaultTransaction(NewInsuranceVaultTransaction),
    InsuranceVaultBalanceUpdate {
        vault_id: String,
        new_balance: i64,
    },
    InsurancePolicy(NewInsurancePolicy),
    InsurancePolicyEvent(NewInsurancePolicyEvent),
    InsuranceMarketExposure(NewInsuranceMarketExposure),
    InsuranceUserExposure(NewInsuranceUserExposure),
    InsuranceEventLog(NewInsuranceEventLog),
    InsuranceCoverageRoute(NewInsuranceCoverageRoute),
    InsuranceRouteFill(NewInsuranceRouteFill),
    InsuranceVaultOperationalUpdate {
        vault_id: String,
        max_exposure_per_option: i64,
        enabled: bool,
        paused: bool,
        max_exposure_per_market: i64,
        max_exposure_per_user: i64,
        base_rate_bps_per_day: i64,
        utilization_multiplier_bps: i64,
    },
    InsurancePolicyStatusUpdate {
        policy_id: String,
        status: i16,
    },
    InsurancePolicyEventFromPolicy {
        policy_id: String,
        event_type: String,
        refunded_amount: Option<i64>,
        fee_paid: Option<i64>,
        payout: Option<i64>,
        reserve_released: Option<i64>,
        timestamp_ms: i64,
        transaction_id: String,
    },
    NominatedDelegateStatusUpdate {
        address: String,
        registry_type: i16,
        status: i16,
        governance_registry_id: String,
    },
    DelegateVoteCountsUpdate {
        target_address: String,
        registry_type: i16,
        is_active_delegate: bool,
        upvotes: i64,
        downvotes: i64,
        governance_registry_id: String,
    },
    ProposalDelegateVoteIncrement {
        proposal_id: String,
        approve: bool,
    },
    DelegateProposalsReviewedIncrement {
        proposal_id: String,
        delegate_address: String,
    },
    ProposalCommunityVoteUpdate {
        proposal_id: String,
        votes_for_delta: i64,
        votes_against_delta: i64,
        reward_pool_delta: i64,
    },
    ProposalOutcomeApplyDelegateSidedUpdates {
        proposal_id: String,
        approvers_win: bool,
    },
    DelegateProposalsSubmittedIncrement {
        proposal_id: String,
        submitter: String,
    },
    ProposalAnonymousVotersIncrement {
        proposal_id: String,
    },
    SpotBet(NewSpotBet),
    SpotResolution(NewSpotResolution),
    SpotPayout(NewSpotPayout),
    SpotRefund(NewSpotRefund),
    SpotEventLog(NewSpotEventLog),
    SpotConfig(NewSpotConfig),
    SpotBetWithdrawal(NewSpotBetWithdrawal),
    SpotRecordUpsert(NewSpotRecord),
    SpotRecordUpdate {
        post_id: String,
        status: i16,
        outcome: Option<i16>,
        last_resolution_at_ms: i64,
    },
    SpotRecordGovernanceUpdate {
        spot_record_id: String,
        post_id: String,
        active_proposal_id: Option<String>,
        oracle_proposed_outcome: Option<i16>,
        proposed_outcome: Option<i16>,
        dao_escalated_at_ms: Option<i64>,
        status: Option<i16>,
    },
    SptPool(NewSptPool),
    SptTransaction(NewSptTransaction),
    SptHolding(NewSptHolding),
    SptPoolSupplyUpdate {
        pool_id: String,
        delta: i64,
    },
    SptPriceHistory(NewSptPriceHistory),
    SptLaunchHoldingsFromReservations {
        pool_id: String,
        associated_id: String,
        owner: String,
        circulating_supply: i64,
        total_reserved_at_launch: i64,
        created_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        transaction_id: String,
    },
    SptReservationPool(NewSptReservationPool),
    SptReservation {
        associated_id: String,
        reservation: NewSptReservation,
        token_type: i16,
        total_reserved: i64,
        threshold_met: bool,
        created_at: i64,
    },
    SptReservationPoolUpdate {
        pool_id: String,
        associated_id: String,
        total_reserved: i64,
        status: Option<String>,
        required_threshold: Option<i64>,
    },
    SptExchangeConfig(NewSptExchangeConfig),
    SocialProofTokensConfig(NewSocialProofTokensConfig),
    SocialProofTokensEvent(NewSocialProofTokensEvent),
    UnifiedRevenue(NewUnifiedRevenue),
    SptBuySellRevenueData {
        pool_id: String,
        associated_id: String,
        token_type: i16,
        trader: String,
        transaction_type: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        amount: i64,
        myso_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    },
    UpgradeEvent(NewUpgradeEvent),
    ObjectMigratedEvent(NewObjectMigratedEvent),
    VestingWallet(NewVestingWallet),
    VestingEvent(NewVestingEvent),
    VestingWalletClaimUpdate {
        wallet_id: String,
        claimed_amount: i64,
        remaining_balance: i64,
    },
    VestingWalletDelete {
        wallet_id: String,
    },
    ProfileSubscriptionService(NewProfileSubscriptionService),
    ProfileSubscription(NewProfileSubscription),
    SubscriptionEvent(NewSubscriptionEvent),
    SubscriptionConfig(NewSubscriptionConfig),
    ProfileSubscriptionServiceSubscriberIncrement {
        service_id: String,
    },
    ProfileSubscriptionUpdate {
        subscription_id: String,
        expires_at: i64,
        renewal_count: i64,
    },
    ProfileSubscriptionCancel {
        subscription_id: String,
    },
    ProfileSubscriptionServiceUpdate {
        service_id: String,
        monthly_fee: i64,
        updated_at: i64,
    },
    ProfileSubscriptionRenewalBalanceUpdate {
        subscription_id: String,
        new_balance: i64,
    },
    ProfileSubscriptionServiceDeactivate {
        service_id: String,
        updated_at: i64,
    },
    ProfileSubscriptionServiceSubscriberDecrementBySubscription {
        subscription_id: String,
    },
    SubscriptionRevenueFromCreated {
        service_id: String,
        subscription_id: String,
        from_address: String,
        amount: i64,
        platform_fee: i64,
        ecosystem_fee: i64,
        creator_amount: i64,
        platform_address: Option<String>,
        revenue_type: String,
        payment_time: i64,
        transaction_id: String,
    },
    SubscriptionRevenueFromRefund {
        subscription_id: String,
        subscriber: String,
        refunded_amount: i64,
        transaction_id: String,
    },
    SubscriptionRevenueFromRenewal {
        subscription_id: String,
        subscriber: String,
        new_expires_at: i64,
        renewal_count: i64,
        auto_renewed: bool,
        platform_fee: i64,
        ecosystem_fee: i64,
        creator_amount: i64,
        platform_address: Option<String>,
        transaction_id: String,
    },
    MemoryAccount(NewMemoryAccount),
    MemoryConfig(NewMemoryConfig),
    ProfileMemoryAccountLink {
        profile_id: String,
        memory_account_id: String,
    },
    MemoryAccountActiveUpdate {
        account_id: String,
        active: bool,
    },
    AgentMemoryVault(NewAgentMemoryVault),
    SubAgentEvent(NewSubAgentEvent),
    AgenticOrganizationUpsert(NewAgenticOrganization),
    AgenticOrganizationMetadataUpdate {
        organization_id: String,
        name: Option<String>,
        description: Option<String>,
    },
    AgenticOrganizationCategoryUpdate {
        organization_id: String,
        org_type: i16,
        previous_org_type: i16,
        updated_at_ms: i64,
    },
    AgenticOrganizationDeactivate {
        organization_id: String,
        deactivated_at_ms: i64,
    },
    AgenticOrganizationMemoryGroupSet {
        organization_id: String,
        group_id: String,
    },
    OrganizationEvent(NewOrganizationEvent),
    OrganizationStatsInit {
        organization_id: String,
        activity_at_ms: i64,
    },
    OrganizationAgentRegistered {
        organization_id: String,
        active: bool,
        depth: i16,
        parent_object_id: Option<String>,
        agent_object_id: String,
        activity_at_ms: i64,
    },
    OrganizationAgentActiveDelta {
        agent_object_id: String,
        active_delta: i32,
        activity_at_ms: i64,
    },
    AiCreditBalanceUpsert(NewAiCreditBalance),
    AiCreditBalanceBalanceUpdate {
        balance_id: String,
        balance_mist: i64,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    AiCreditBalanceCapsUpdate {
        balance_id: String,
        daily_cap_mist: Option<i64>,
        monthly_cap_mist: Option<i64>,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    AiCreditBalanceSettlementUpdate {
        balance_id: String,
        settlement_nonce: i64,
        spent_increment_mist: i64,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    AiCreditBalanceActiveUpdate {
        balance_id: String,
        active: bool,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    AiCreditAgentBudgetUpsert(NewAiCreditAgentBudget),
    AiCreditAgentBudgetDisable {
        balance_id: String,
        agent_object_id: String,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    AiCreditAgentBudgetSpendUpdate {
        balance_id: String,
        agent_object_id: String,
        spent_increment_mist: i64,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    AiCreditUsageLineSettle {
        receipt_id: String,
        settlement_tx: String,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    ProfileAiCreditBalanceLink {
        profile_id: String,
        ai_credit_balance_id: String,
    },
    AiCreditConfigUpsert(NewAiCreditConfig),
    AiCreditConfigLimitsUpdate {
        updated_by: String,
        max_single_settlement_mist: i64,
        receipt_ttl_ms: i64,
        updated_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        transaction_id: String,
    },
    AiCreditConfigPubkeyUpdate {
        updated_by: String,
        oracle_pubkey_hex: String,
        updated_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        transaction_id: String,
    },
    AiCreditConfigMarkupUpdate {
        updated_by: String,
        oracle_markup_bps: i64,
        updated_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        transaction_id: String,
    },
    AiCreditConfigMinDepositUpdate {
        updated_by: String,
        min_deposit_mist: i64,
        updated_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        transaction_id: String,
    },
    AiCreditEvent(NewAiCreditEvent),
    // ==== Enterprise workforce foundation rows ====
    /// Upsert one org memory permission bit (handlers expand event masks per bit).
    OrgMemoryPermissionUpsert(NewOrgMemoryPermission),
    OrgRoleUpsert(NewOrgRole),
    OrgRoleAssignmentUpsert(NewOrgRoleAssignment),
    OrgRoleAssignmentRevoke {
        organization_id: String,
        member_address: String,
        role_name: String,
        revoked_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    OrgInvitationUpsert(NewOrgInvitation),
    OrgInvitationRespond {
        organization_id: String,
        invitee_address: String,
        status: String,
        responded_at_ms: i64,
        responded_by: String,
        granted_mask: Option<i64>,
        event_id: String,
        transaction_id: String,
    },
    /// Unified audit-log row (written in the same commit as the domain update).
    AuditLog(NewAuditLog),
    /// Approved-state upsert from `AiCreditSpendApproved` (overwrites requested rows).
    AiCreditSpendApprovalUpsert(NewAiCreditSpendApproval),
    /// Status transition (consumed / revoked / expired) keyed by (balance, agent).
    AiCreditSpendApprovalStatus {
        balance_id: String,
        agent_object_id: String,
        status: String,
        consumed_amount_mist: Option<i64>,
        event_id: String,
    },
    /// Tier 1 org AI-credit spend increment; the handler resolves the org via
    /// `sub_agents.organization_id` and backfills the usage line's org attribution.
    AiCreditOrgSpendFromAgent {
        agent_object_id: String,
        amount_mist: i64,
        receipt_id: Option<String>,
        activity_at_ms: i64,
    },
    MessagingConfig(NewMessagingConfig),
    PaidMessageEscrow(NewPaidMessageEscrow),
    MessagingAgentGroup(NewMessagingAgentGroup),
    MessagingOrgOutboundSpend {
        payer: String,
        amount: i64,
        counterparty: Option<String>,
        activity_at_ms: i64,
    },
    MessagingOrgInboundRevenue {
        recipient: String,
        amount: i64,
        counterparty: Option<String>,
        activity_at_ms: i64,
    },
    MessagingAgentGroupOrgActivity {
        organization_id: Option<String>,
        activity_at_ms: i64,
    },
}

#[derive(Debug, Clone)]
pub struct ProfileUpdate {
    pub profile_id: String,
    pub owner_address: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub cover_photo: Option<String>,
    pub website: Option<String>,
    pub birthdate: Option<String>,
    pub location: Option<String>,
    pub x_username: Option<String>,
    pub username: Option<String>,
    pub selected_badge_id: Option<Option<String>>,
    pub selected_ecosystem_badge_id: Option<Option<String>>,
    pub reservation_pool_address: Option<Option<String>>,
    pub social_proof_token_address: Option<Option<String>>,
}

impl FieldCount for SocialEventRow {
    const FIELD_COUNT: usize = 162;
}

// SocialEvents pipeline removed: profile and post events now handled by ProfilesHandler and PostsHandler.
// SocialEventRow is retained for use by domain handlers (profile::handle_profile_event, post::handle_post_event)
// and from_social conversion in ProfilesHandler and PostsHandler.
