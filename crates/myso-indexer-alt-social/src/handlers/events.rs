// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! BCS-compatible event structs and parsing for myso-social Move events.
//! Field order must match the Move struct definitions exactly.

use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

/// Error returned when event contents fail to parse, for diagnostic logging.
#[derive(Debug)]
pub struct EventParseError {
    pub error: String,
    pub contents: Vec<u8>,
}

impl EventParseError {
    pub fn contents_hex_preview(&self, max_bytes: usize) -> String {
        let len = self.contents.len().min(max_bytes);
        hex::encode(&self.contents[..len])
    }
}

fn bcs_parse_err(e: bcs::Error, contents: &[u8]) -> EventParseError {
    EventParseError {
        error: e.to_string(),
        contents: contents.to_vec(),
    }
}

fn addr_to_string(addr: &AccountAddress) -> String {
    format!("0x{}", hex::encode(addr))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileCreatedEvent {
    profile_id: AccountAddress,
    display_name: String,
    username: String,
    bio: String,
    profile_picture: Option<String>,
    cover_photo: Option<String>,
    owner: AccountAddress,
    pub(super) created_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsFollowEvent {
    follower: AccountAddress,
    following: AccountAddress,
}

impl BcsFollowEvent {
    pub fn follower(&self) -> String {
        addr_to_string(&self.follower)
    }

    pub fn following(&self) -> String {
        addr_to_string(&self.following)
    }
}

#[derive(Debug, Deserialize)]
pub struct BcsUnfollowEvent {
    follower: AccountAddress,
    unfollowed: AccountAddress,
}

impl BcsUnfollowEvent {
    pub fn follower(&self) -> String {
        addr_to_string(&self.follower)
    }

    pub fn unfollowed(&self) -> String {
        addr_to_string(&self.unfollowed)
    }
}

#[derive(Debug, Deserialize)]
pub struct BcsUserBlockEvent {
    blocker: AccountAddress,
    blocked: AccountAddress,
}

impl BcsUserBlockEvent {
    pub fn blocker(&self) -> String {
        addr_to_string(&self.blocker)
    }

    pub fn blocked(&self) -> String {
        addr_to_string(&self.blocked)
    }
}

#[derive(Debug, Deserialize)]
pub struct BcsUserUnblockEvent {
    blocker: AccountAddress,
    unblocked: AccountAddress,
}

impl BcsUserUnblockEvent {
    pub fn blocker(&self) -> String {
        addr_to_string(&self.blocker)
    }

    pub fn unblocked(&self) -> String {
        addr_to_string(&self.unblocked)
    }
}

#[derive(Debug, Deserialize)]
pub struct BcsGovernanceRegistryCreatedEvent {
    registry_id: AccountAddress,
    registry_type: u8,
    delegate_count: u64,
    delegate_term_epochs: u64,
    proposal_submission_cost: u64,
    max_votes_per_user: u64,
    quadratic_base_cost: u64,
    voting_period_ms: u64,
    quorum_votes: u64,
    updated_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsDelegateNominatedEvent {
    nominee_address: AccountAddress,
    scheduled_term_start_epoch: u64,
    registry_type: u8,
}

#[derive(Debug, Deserialize)]
pub struct BcsDelegateElectedEvent {
    delegate_address: AccountAddress,
    term_start: u64,
    term_end: u64,
    registry_type: u8,
}

#[derive(Debug, Deserialize)]
pub struct BcsDelegateVotedEvent {
    target_address: AccountAddress,
    voter: AccountAddress,
    is_active_delegate: bool,
    upvote: bool,
    new_upvote_count: u64,
    new_downvote_count: u64,
    registry_type: u8,
}

#[derive(Debug, Deserialize)]
pub struct BcsProposalSubmittedEvent {
    proposal_id: AccountAddress,
    title: String,
    description: String,
    proposal_type: u8,
    reference_id: Option<AccountAddress>,
    metadata_json: Option<String>,
    submitter: AccountAddress,
    reward_amount: u64,
    submission_time: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsDelegateVoteEvent {
    proposal_id: AccountAddress,
    delegate_address: AccountAddress,
    approve: bool,
    vote_time: u64,
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsCommunityVoteEvent {
    proposal_id: AccountAddress,
    voter: AccountAddress,
    vote_weight: u64,
    approve: bool,
    vote_time: u64,
    vote_cost: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsAnonymousVoteEvent {
    proposal_id: AccountAddress,
    voter: AccountAddress,
    vote_time: u64,
    encrypted_vote_data: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct BcsProposalApprovedForVotingEvent {
    proposal_id: AccountAddress,
    voting_start_time: u64,
    voting_end_time: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsProposalRejectedEvent {
    proposal_id: AccountAddress,
    rejection_time: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsProposalApprovedEvent {
    proposal_id: AccountAddress,
    approval_time: u64,
    votes_for: u64,
    votes_against: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsProposalRejectedByCommunityEvent {
    proposal_id: AccountAddress,
    rejection_time: u64,
    votes_for: u64,
    votes_against: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsProposalImplementedEvent {
    proposal_id: AccountAddress,
    implementation_time: u64,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsRewardsDistributedEvent {
    proposal_id: AccountAddress,
    total_reward: u64,
    recipient_count: u64,
    distribution_time: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsVoteDecryptionFailedEvent {
    proposal_id: AccountAddress,
    voter: AccountAddress,
    failure_reason: String,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsProposalRescindedEvent {
    proposal_id: AccountAddress,
    submitter: AccountAddress,
    rescind_time: u64,
    refund_amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsGovernanceParametersUpdatedEvent {
    registry_type: u8,
    updated_by: AccountAddress,
    delegate_count: u64,
    delegate_term_epochs: u64,
    proposal_submission_cost: u64,
    max_votes_per_user: u64,
    quadratic_base_cost: u64,
    voting_period_ms: u64,
    quorum_votes: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsProfileUpdatedEvent {
    profile_id: AccountAddress,
    display_name: Option<String>,
    username: String,
    bio: String,
    profile_picture: Option<String>,
    cover_photo: Option<String>,
    owner: AccountAddress,
    updated_at: u64,
    facebook_username: Option<String>,
    github_username: Option<String>,
    instagram_username: Option<String>,
    linkedin_username: Option<String>,
    reddit_username: Option<String>,
    twitch_username: Option<String>,
    x_username: Option<String>,
    min_offer_amount: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct BcsBadgeAssignedEvent {
    profile_id: AccountAddress,
    badge_id: String,
    name: String,
    description: String,
    media_url: String,
    icon_url: String,
    platform_id: AccountAddress,
    assigned_by: AccountAddress,
    assigned_at: u64,
    badge_type: u8,
}

#[derive(Debug, Deserialize)]
pub struct BcsBadgeRevokedEvent {
    profile_id: AccountAddress,
    badge_id: String,
    platform_id: AccountAddress,
    revoked_by: AccountAddress,
    revoked_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsBadgeSelectedEvent {
    profile_id: AccountAddress,
    badge_id: String,
    selected_by: AccountAddress,
    selected_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsBadgeRemovedEvent {
    profile_id: AccountAddress,
    badge_id: String,
    platform_id: AccountAddress,
    removed_by: AccountAddress,
    removed_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPostCreatedEvent {
    post_id: AccountAddress,
    owner: AccountAddress,
    profile_id: AccountAddress,
    content: String,
    post_type: String,
    parent_post_id: Option<AccountAddress>,
    mentions: Option<Vec<AccountAddress>>,
    media_urls: Option<Vec<String>>,
    metadata_json: Option<String>,
    mydata_id: Option<AccountAddress>,
    promotion_id: Option<AccountAddress>,
    revenue_redirect_to: Option<AccountAddress>,
    revenue_redirect_percentage: Option<u64>,
    enable_spt: bool,
    enable_poc: bool,
    enable_spot: bool,
    spot_id: Option<AccountAddress>,
    spt_id: Option<AccountAddress>,
}

#[derive(Debug, Deserialize)]
pub struct BcsCommentCreatedEvent {
    comment_id: AccountAddress,
    post_id: AccountAddress,
    parent_comment_id: Option<AccountAddress>,
    owner: AccountAddress,
    profile_id: AccountAddress,
    content: String,
    mentions: Option<Vec<AccountAddress>>,
}

#[derive(Debug, Deserialize)]
pub struct BcsReactionEvent {
    object_id: AccountAddress,
    user: AccountAddress,
    reaction: String,
    is_post: bool,
}

#[derive(Debug, Deserialize)]
pub struct BcsRemoveReactionEvent {
    object_id: AccountAddress,
    user: AccountAddress,
    reaction: String,
    is_post: bool,
}

#[derive(Debug, Deserialize)]
pub struct BcsRepostEvent {
    repost_id: AccountAddress,
    original_id: AccountAddress,
    is_original_post: bool,
    owner: AccountAddress,
    profile_id: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsTipEvent {
    object_id: AccountAddress,
    from: AccountAddress,
    to: AccountAddress,
    amount: u64,
    is_post: bool,
}

#[derive(Debug, Deserialize)]
pub struct BcsPostModerationEvent {
    post_id: AccountAddress,
    platform_id: AccountAddress,
    removed: bool,
    moderated_by: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsPostReportedEvent {
    post_id: AccountAddress,
    reporter: AccountAddress,
    reason_code: u8,
    description: String,
    reported_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsCommentReportedEvent {
    comment_id: AccountAddress,
    reporter: AccountAddress,
    reason_code: u8,
    description: String,
    reported_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPostDeletedEvent {
    post_id: AccountAddress,
    owner: AccountAddress,
    profile_id: AccountAddress,
    post_type: String,
    deleted_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsCommentDeletedEvent {
    comment_id: AccountAddress,
    post_id: AccountAddress,
    owner: AccountAddress,
    profile_id: AccountAddress,
    deleted_at: u64,
}

// Proof of Creativity (PoC) event structs - field order matches proof_of_creativity.move
#[derive(Debug, Deserialize)]
pub struct BcsAnalysisSubmittedEvent {
    post_id: AccountAddress,
    media_type: u8,
    similarity_detected: bool,
    highest_similarity_score: u64,
    oracle_address: AccountAddress,
    timestamp: u64,
    reasoning: Option<String>,
    evidence_urls: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct BcsPocBadgeIssuedEvent {
    badge_id: AccountAddress,
    post_id: AccountAddress,
    media_type: u8,
    issued_by: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsRevenueRedirectionActivatedEvent {
    redirection_id: AccountAddress,
    accused_post_id: AccountAddress,
    original_post_id: AccountAddress,
    redirect_percentage: u64,
    similarity_score: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPocDisputeSubmittedEvent {
    dispute_id: AccountAddress,
    post_id: AccountAddress,
    disputer: AccountAddress,
    dispute_type: u8,
    stake_amount: u64,
    voting_start_epoch: u64,
    voting_end_epoch: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsDisputeVoteCastEvent {
    dispute_id: AccountAddress,
    voter: AccountAddress,
    vote_choice: u8,
    stake_amount: u64,
    total_uphold_stake: u64,
    total_overturn_stake: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPocDisputeResolvedEvent {
    dispute_id: AccountAddress,
    post_id: AccountAddress,
    resolution: u8,
    winning_side: u8,
    total_winning_stake: u64,
    total_losing_stake: u64,
    badge_revoked: bool,
    redirection_removed: bool,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsVotingRewardClaimedEvent {
    dispute_id: AccountAddress,
    voter: AccountAddress,
    original_stake: u64,
    reward_amount: u64,
    total_payout: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPocConfigUpdatedEvent {
    updated_by: AccountAddress,
    oracle_address: AccountAddress,
    image_threshold: u64,
    video_threshold: u64,
    audio_threshold: u64,
    revenue_redirect_percentage: u64,
    dispute_cost: u64,
    dispute_protocol_fee: u64,
    min_vote_stake: u64,
    max_vote_stake: u64,
    voting_duration_epochs: u64,
    max_reasoning_length: u64,
    max_evidence_urls: u64,
    max_votes_per_dispute: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsTokenPoolSyncNeededEvent {
    post_id: AccountAddress,
    timestamp: u64,
}

// Insurance event structs - field order matches insurance.move exactly
// Move ID type is 32 bytes, same as AccountAddress
#[derive(Debug, Deserialize)]
pub struct BcsConfigInitializedEvent {
    admin: AccountAddress,
    min_coverage_bps: u64,
    max_coverage_bps: u64,
    max_duration_ms: u64,
    fee_bps: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsConfigUpdatedEvent {
    updated_by: AccountAddress,
    enable_flag: bool,
    min_coverage_bps: u64,
    max_coverage_bps: u64,
    max_duration_ms: u64,
    fee_bps: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsUnderwriterVaultCreatedEvent {
    vault_id: AccountAddress,
    underwriter: AccountAddress,
    base_rate_bps_per_day: u64,
    utilization_multiplier_bps: u64,
    max_exposure_per_market: u64,
    max_exposure_per_user: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsUnderwriterVaultDepositedEvent {
    vault_id: AccountAddress,
    amount: u64,
    new_balance: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsUnderwriterVaultWithdrawnEvent {
    vault_id: AccountAddress,
    amount: u64,
    new_balance: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsCoveragePurchasedEvent {
    policy_id: AccountAddress,
    market_id: AccountAddress,
    insured: AccountAddress,
    option_id: u8,
    covered_amount: u64,
    coverage_bps: u64,
    premium_paid: u64,
    reserve_locked: u64,
    expiry_time_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsCoverageCancelledEvent {
    policy_id: AccountAddress,
    insured: AccountAddress,
    refunded_amount: u64,
    fee_paid: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsCoverageClaimedEvent {
    policy_id: AccountAddress,
    insured: AccountAddress,
    payout: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPolicyExpiredEvent {
    policy_id: AccountAddress,
    insured: AccountAddress,
    market_id: AccountAddress,
    vault_id: AccountAddress,
    reserve_released: u64,
    expiry_time_ms: u64,
}

// MyData marketplace event structs - field order matches mydata.move exactly
#[derive(Debug, Deserialize)]
pub struct BcsMyDataCreatedEvent {
    ip_id: AccountAddress,
    owner: AccountAddress,
    media_type: String,
    platform_id: Option<AccountAddress>,
    one_time_price: Option<u64>,
    subscription_price: Option<u64>,
    created_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPurchaseEvent {
    ip_id: AccountAddress,
    buyer: AccountAddress,
    price: u64,
    purchase_type: String,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsAccessGrantedEvent {
    ip_id: AccountAddress,
    user: AccountAddress,
    access_type: String,
    granted_by: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsMyDataRegisteredEvent {
    ip_id: AccountAddress,
    owner: AccountAddress,
    registered_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsMyDataUnregisteredEvent {
    ip_id: AccountAddress,
    owner: AccountAddress,
    unregistered_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsMyDataConfigUpdatedEvent {
    updated_by: AccountAddress,
    enable_flag: bool,
    max_tags: u64,
    max_subscription_days: u64,
    max_free_access_grants: u64,
    timestamp: u64,
}

// Social Proof of Truth (SPoT) event structs - field order matches social_proof_of_truth.move
#[derive(Debug, Deserialize)]
pub struct BcsSpotBetPlacedEvent {
    post_id: AccountAddress,
    user: AccountAddress,
    option_id: u8,
    amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotResolvedEvent {
    post_id: AccountAddress,
    outcome: u8,
    total_escrow: u64,
    fee_taken: u64,
    reasoning: String,
    evidence_urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotDaoRequiredEvent {
    post_id: AccountAddress,
    confidence_bps: u64,
    reasoning: String,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotPayoutEvent {
    post_id: AccountAddress,
    user: AccountAddress,
    amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotRefundEvent {
    post_id: AccountAddress,
    user: AccountAddress,
    amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotConfigUpdatedEvent {
    updated_by: AccountAddress,
    enable_flag: bool,
    confidence_threshold_bps: u64,
    resolution_window_epochs: u64,
    max_resolution_window_epochs: u64,
    payout_delay_ms: u64,
    fee_bps: u64,
    fee_split_bps_platform: u64,
    oracle_address: AccountAddress,
    max_single_bet: u64,
    max_bets_per_record: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotBetWithdrawnEvent {
    post_id: AccountAddress,
    user: AccountAddress,
    option_id: u8,
    amount: u64,
    fee_taken: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotRecordCreatedEvent {
    record_id: AccountAddress,
    post_id: AccountAddress,
    created_epoch: u64,
    betting_options: Vec<String>,
    resolution_window_epochs: Option<u64>,
    max_resolution_window_epochs: Option<u64>,
}

// Upgrade event structs - field order matches upgrade.move
#[derive(Debug, Deserialize)]
pub struct BcsUpgradeEvent {
    package_id: AccountAddress,
    version: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsObjectMigratedEvent {
    object_id: AccountAddress,
    object_type: String,
    old_version: u64,
    new_version: u64,
    migrated_by: AccountAddress,
}

// Profile subscription event structs - field order matches subscription.move
#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileSubscriptionServiceCreatedEvent {
    service_id: AccountAddress,
    profile_owner: AccountAddress,
    monthly_fee: u64,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileSubscriptionCreatedEvent {
    service_id: AccountAddress,
    subscriber: AccountAddress,
    expires_at: u64,
    monthly_fee: u64,
    auto_renew: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileSubscriptionRenewedEvent {
    subscription_id: AccountAddress,
    subscriber: AccountAddress,
    new_expires_at: u64,
    renewal_count: u64,
    auto_renewed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileSubscriptionCancelledEvent {
    subscription_id: AccountAddress,
    subscriber: AccountAddress,
    refunded_amount: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileSubscriptionUpdatedEvent {
    service_id: AccountAddress,
    old_fee: u64,
    new_fee: u64,
    updated_by: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsRenewalBalanceFundedEvent {
    subscription_id: AccountAddress,
    subscriber: AccountAddress,
    funded_amount: u64,
    new_balance: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileSubscriptionServiceDeactivatedEvent {
    service_id: AccountAddress,
    profile_owner: AccountAddress,
    deactivated_at: u64,
}

// Social Proof Token (SPT) event structs - field order matches social_proof_tokens.move
#[derive(Debug, Deserialize)]
pub struct BcsTokenPoolCreatedEvent {
    id: AccountAddress,
    token_type: u8,
    owner: AccountAddress,
    associated_id: AccountAddress,
    symbol: String,
    name: String,
    base_price: u64,
    quadratic_coefficient: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsTokenBoughtEvent {
    id: AccountAddress,
    buyer: AccountAddress,
    amount: u64,
    myso_amount: u64,
    fee_amount: u64,
    creator_fee: u64,
    platform_fee: u64,
    treasury_fee: u64,
    new_price: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsTokenSoldEvent {
    id: AccountAddress,
    seller: AccountAddress,
    amount: u64,
    myso_amount: u64,
    fee_amount: u64,
    creator_fee: u64,
    platform_fee: u64,
    treasury_fee: u64,
    new_price: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsReservationCreatedEvent {
    associated_id: AccountAddress,
    token_type: u8,
    reserver: AccountAddress,
    amount: u64,
    total_reserved: u64,
    threshold_met: bool,
    reserved_at: u64,
    fee_amount: u64,
    creator_fee: u64,
    platform_fee: u64,
    treasury_fee: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsReservationWithdrawnEvent {
    associated_id: AccountAddress,
    token_type: u8,
    reserver: AccountAddress,
    amount: u64,
    total_reserved: u64,
    withdrawn_at: u64,
    fee_amount: u64,
    creator_fee: u64,
    platform_fee: u64,
    treasury_fee: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsThresholdMetEvent {
    associated_id: AccountAddress,
    token_type: u8,
    owner: AccountAddress,
    total_reserved: u64,
    required_threshold: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsReservationPoolCreatedEvent {
    associated_id: AccountAddress,
    token_type: u8,
    owner: AccountAddress,
    required_threshold: u64,
    pool_object_id: AccountAddress,
    created_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSptConfigUpdatedEvent {
    updated_by: AccountAddress,
    timestamp: u64,
    total_fee_bps: u64,
    trading_creator_fee_bps: u64,
    trading_platform_fee_bps: u64,
    trading_treasury_fee_bps: u64,
    reservation_total_fee_bps: u64,
    reservation_creator_fee_bps: u64,
    reservation_platform_fee_bps: u64,
    reservation_treasury_fee_bps: u64,
    base_price: u64,
    quadratic_coefficient: u64,
    max_hold_percent_bps: u64,
    post_threshold: u64,
    profile_threshold: u64,
    max_individual_reservation_bps: u64,
    max_reservers_per_pool: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsTokensAddedEvent {
    owner: AccountAddress,
    pool_id: AccountAddress,
    amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsEmergencyKillSwitchEvent {
    admin: AccountAddress,
    trading_enabled: bool,
    timestamp: u64,
    reason: String,
}

#[derive(Debug, Deserialize)]
pub struct BcsPocRedirectionUpdatedEvent {
    pool_id: AccountAddress,
    post_id: AccountAddress,
    redirect_to: Option<AccountAddress>,
    redirect_percentage: Option<u64>,
    updated_by: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPlatformStatus {
    status: u8,
}

#[derive(Debug, Deserialize)]
pub struct BcsPlatformCreatedEvent {
    platform_id: AccountAddress,
    name: String,
    tagline: String,
    description: String,
    developer: AccountAddress,
    logo: String,
    terms_of_service: String,
    privacy_policy: String,
    platforms: Vec<String>,
    links: Vec<String>,
    primary_category: String,
    secondary_category: Option<String>,
    status: BcsPlatformStatus,
    release_date: String,
    wants_dao_governance: bool,
    governance_registry_id: Option<AccountAddress>,
    delegate_count: Option<u64>,
    delegate_term_epochs: Option<u64>,
    proposal_submission_cost: Option<u64>,
    min_on_chain_age_days: Option<u64>,
    max_votes_per_user: Option<u64>,
    quadratic_base_cost: Option<u64>,
    voting_period_epochs: Option<u64>,
    quorum_votes: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct BcsPlatformUpdatedEvent {
    platform_id: AccountAddress,
    name: String,
    tagline: String,
    description: String,
    terms_of_service: String,
    privacy_policy: String,
    platforms: Vec<String>,
    links: Vec<String>,
    primary_category: String,
    secondary_category: Option<String>,
    status: BcsPlatformStatus,
    release_date: String,
    shutdown_date: Option<String>,
    updated_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPlatformApprovalChangedEvent {
    platform_id: AccountAddress,
    approved: bool,
    changed_by: AccountAddress,
    reasoning: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsModeratorAddedEvent {
    platform_id: AccountAddress,
    moderator_address: AccountAddress,
    added_by: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsModeratorRemovedEvent {
    platform_id: AccountAddress,
    moderator_address: AccountAddress,
    removed_by: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsPlatformDeletedEvent {
    platform_id: AccountAddress,
    name: String,
    developer: AccountAddress,
    deleted_by: AccountAddress,
    timestamp: u64,
    reasoning: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsUserJoinedPlatformEvent {
    wallet_address: AccountAddress,
    platform_id: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsUserLeftPlatformEvent {
    wallet_address: AccountAddress,
    platform_id: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsTokenAirdropEvent {
    platform_id: AccountAddress,
    recipient: AccountAddress,
    amount: u64,
    reason_code: u8,
    executed_by: AccountAddress,
    timestamp: u64,
}

fn mentions_to_json(mentions: &Option<Vec<AccountAddress>>) -> Option<serde_json::Value> {
    mentions
        .as_ref()
        .map(|v| serde_json::json!(v.iter().map(addr_to_string).collect::<Vec<_>>()))
}

/// Parse BCS event contents using the module and event name to select the correct deserializer.
/// Falls back to JSON parsing when no BCS struct matches the event signature.
/// Returns `Err` with diagnostic info when BCS or JSON parsing fails.
pub fn parse_event_contents(
    module: &str,
    event_name: &str,
    contents: &[u8],
) -> Result<serde_json::Value, EventParseError> {
    let result = match module {
        "profile" => parse_profile_event(event_name, contents),
        "governance" => parse_governance_event(event_name, contents),
        "social_graph" => parse_social_graph_event(event_name, contents),
        "block_list" | "blocking" => parse_blocking_event(event_name, contents),
        "post" | "comment" | "reaction" | "repost" | "tip" => {
            parse_post_event(event_name, contents)
        }
        "platform" => parse_platform_event(event_name, contents),
        "poc" | "proof_of_creativity" => parse_poc_event(event_name, contents),
        "mydata" | "my_ip" => parse_mydata_event(event_name, contents),
        "insurance" => parse_insurance_event(event_name, contents),
        "social_proof_of_truth" | "spot" => parse_spot_event(event_name, contents),
        "social_proof_tokens" | "spt" => parse_spt_event(event_name, contents),
        "subscription" | "profile_subscription" => parse_subscription_event(event_name, contents),
        "upgrade" => parse_upgrade_event(event_name, contents),
        _ => Ok(None),
    };

    if let Ok(Some(v)) = result {
        return Ok(v);
    }

    match serde_json::from_slice::<serde_json::Value>(contents) {
        Ok(json) => return Ok(json),
        Err(json_err) => {
            if let Err(e) = result {
                return Err(e);
            }
            return Err(EventParseError {
                error: json_err.to_string(),
                contents: contents.to_vec(),
            });
        }
    }
}

fn parse_profile_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "ProfileCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "profile_id": addr_to_string(&ev.profile_id),
                "display_name": ev.display_name,
                "username": ev.username,
                "bio": ev.bio,
                "profile_picture": ev.profile_picture,
                "cover_photo": ev.cover_photo,
                "owner_address": addr_to_string(&ev.owner),
                "created_at": ev.created_at,
            })))
        }
        "ProfileUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "profile_id": addr_to_string(&ev.profile_id),
                "owner_address": addr_to_string(&ev.owner),
                "display_name": ev.display_name,
                "username": ev.username,
                "bio": ev.bio,
                "profile_picture": ev.profile_picture,
                "cover_photo": ev.cover_photo,
                "updated_at": ev.updated_at,
                "facebook_username": ev.facebook_username,
                "github_username": ev.github_username,
                "instagram_username": ev.instagram_username,
                "linkedin_username": ev.linkedin_username,
                "reddit_username": ev.reddit_username,
                "twitch_username": ev.twitch_username,
                "x_username": ev.x_username,
                "min_offer_amount": ev.min_offer_amount,
            })))
        }
        "BadgeAssignedEvent" => {
            let ev = bcs::from_bytes::<BcsBadgeAssignedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "profile_id": addr_to_string(&ev.profile_id),
                "badge_id": ev.badge_id,
                "name": ev.name,
                "description": ev.description,
                "media_url": ev.media_url,
                "icon_url": ev.icon_url,
                "platform_id": addr_to_string(&ev.platform_id),
                "assigned_by": addr_to_string(&ev.assigned_by),
                "assigned_at": ev.assigned_at,
                "badge_type": ev.badge_type,
            })))
        }
        "BadgeRevokedEvent" => {
            let ev = bcs::from_bytes::<BcsBadgeRevokedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "profile_id": addr_to_string(&ev.profile_id),
                "badge_id": ev.badge_id,
                "platform_id": addr_to_string(&ev.platform_id),
                "revoked_by": addr_to_string(&ev.revoked_by),
                "revoked_at": ev.revoked_at,
            })))
        }
        "BadgeSelectedEvent" => {
            let ev = bcs::from_bytes::<BcsBadgeSelectedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "profile_id": addr_to_string(&ev.profile_id),
                "badge_id": ev.badge_id,
                "selected_by": addr_to_string(&ev.selected_by),
                "selected_at": ev.selected_at,
            })))
        }
        "BadgeRemovedEvent" => {
            let ev = bcs::from_bytes::<BcsBadgeRemovedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "profile_id": addr_to_string(&ev.profile_id),
                "badge_id": ev.badge_id,
                "platform_id": addr_to_string(&ev.platform_id),
                "removed_by": addr_to_string(&ev.removed_by),
                "removed_at": ev.removed_at,
            })))
        }
        // PaidMessagingSettingsUpdatedEvent and other profile events fall through to JSON
        _ => Ok(None),
    }
}

fn parse_governance_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "GovernanceRegistryCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsGovernanceRegistryCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "registry_id": addr_to_string(&ev.registry_id),
                "registry_type": ev.registry_type,
                "delegate_count": ev.delegate_count,
                "delegate_term_epochs": ev.delegate_term_epochs,
                "proposal_submission_cost": ev.proposal_submission_cost,
                "max_votes_per_user": ev.max_votes_per_user,
                "quadratic_base_cost": ev.quadratic_base_cost,
                "voting_period_ms": ev.voting_period_ms,
                "quorum_votes": ev.quorum_votes,
                "updated_at": ev.updated_at,
            })))
        }
        "DelegateNominatedEvent" => {
            let ev = bcs::from_bytes::<BcsDelegateNominatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "nominee_address": addr_to_string(&ev.nominee_address),
                "registry_type": ev.registry_type,
                "scheduled_term_start_epoch": ev.scheduled_term_start_epoch,
            })))
        }
        "DelegateElectedEvent" => {
            let ev = bcs::from_bytes::<BcsDelegateElectedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "delegate_address": addr_to_string(&ev.delegate_address),
                "registry_type": ev.registry_type,
                "term_start": ev.term_start,
                "term_end": ev.term_end,
            })))
        }
        "DelegateVotedEvent" => {
            let ev = bcs::from_bytes::<BcsDelegateVotedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "target_address": addr_to_string(&ev.target_address),
                "voter": addr_to_string(&ev.voter),
                "registry_type": ev.registry_type,
                "is_active_delegate": ev.is_active_delegate,
                "upvote": ev.upvote,
                "new_upvote_count": ev.new_upvote_count,
                "new_downvote_count": ev.new_downvote_count,
            })))
        }
        "ProposalSubmittedEvent" => {
            let ev = bcs::from_bytes::<BcsProposalSubmittedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "title": ev.title,
                "description": ev.description,
                "proposal_type": ev.proposal_type,
                "reference_id": ev.reference_id.as_ref().map(addr_to_string),
                "metadata_json": ev.metadata_json,
                "submitter": addr_to_string(&ev.submitter),
                "reward_amount": ev.reward_amount,
                "submission_time": ev.submission_time,
            })))
        }
        "DelegateVoteEvent" => {
            let ev = bcs::from_bytes::<BcsDelegateVoteEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "delegate_address": addr_to_string(&ev.delegate_address),
                "approve": ev.approve,
                "vote_time": ev.vote_time,
                "reason": ev.reason,
            })))
        }
        "CommunityVoteEvent" => {
            let ev = bcs::from_bytes::<BcsCommunityVoteEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "voter": addr_to_string(&ev.voter),
                "vote_weight": ev.vote_weight,
                "approve": ev.approve,
                "vote_time": ev.vote_time,
                "vote_cost": ev.vote_cost,
            })))
        }
        "AnonymousVoteEvent" | "AnonymousVoteSubmittedEvent" => {
            let ev = bcs::from_bytes::<BcsAnonymousVoteEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "voter": addr_to_string(&ev.voter),
                "vote_time": ev.vote_time,
                "encrypted_vote_data": ev.encrypted_vote_data,
            })))
        }
        "ProposalApprovedForVotingEvent" => {
            let ev = bcs::from_bytes::<BcsProposalApprovedForVotingEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "voting_start_time": ev.voting_start_time,
                "voting_end_time": ev.voting_end_time,
            })))
        }
        "ProposalRejectedEvent" => {
            let ev = bcs::from_bytes::<BcsProposalRejectedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "rejection_time": ev.rejection_time,
            })))
        }
        "ProposalApprovedEvent" => {
            let ev = bcs::from_bytes::<BcsProposalApprovedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "approval_time": ev.approval_time,
                "votes_for": ev.votes_for,
                "votes_against": ev.votes_against,
            })))
        }
        "ProposalRejectedByCommunityEvent" => {
            let ev = bcs::from_bytes::<BcsProposalRejectedByCommunityEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "rejection_time": ev.rejection_time,
                "votes_for": ev.votes_for,
                "votes_against": ev.votes_against,
            })))
        }
        "ProposalImplementedEvent" => {
            let ev = bcs::from_bytes::<BcsProposalImplementedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "implementation_time": ev.implementation_time,
                "description": ev.description,
            })))
        }
        "RewardsDistributedEvent" => {
            let ev = bcs::from_bytes::<BcsRewardsDistributedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "total_reward": ev.total_reward,
                "recipient_count": ev.recipient_count,
                "distribution_time": ev.distribution_time,
            })))
        }
        "VoteDecryptionFailedEvent" => {
            let ev = bcs::from_bytes::<BcsVoteDecryptionFailedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "voter": addr_to_string(&ev.voter),
                "failure_reason": ev.failure_reason,
                "timestamp": ev.timestamp,
            })))
        }
        "ProposalRescindedEvent" => {
            let ev = bcs::from_bytes::<BcsProposalRescindedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "submitter": addr_to_string(&ev.submitter),
                "rescind_time": ev.rescind_time,
                "refund_amount": ev.refund_amount,
            })))
        }
        "GovernanceParametersUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsGovernanceParametersUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "registry_type": ev.registry_type,
                "updated_by": addr_to_string(&ev.updated_by),
                "delegate_count": ev.delegate_count,
                "delegate_term_epochs": ev.delegate_term_epochs,
                "proposal_submission_cost": ev.proposal_submission_cost,
                "max_votes_per_user": ev.max_votes_per_user,
                "quadratic_base_cost": ev.quadratic_base_cost,
                "voting_period_ms": ev.voting_period_ms,
                "quorum_votes": ev.quorum_votes,
                "timestamp": ev.timestamp,
            })))
        }
        _ => Ok(None),
    }
}

fn parse_social_graph_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "FollowEvent" => {
            let ev = bcs::from_bytes::<BcsFollowEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "follower": ev.follower(),
                "following": ev.following(),
            })))
        }
        "UnfollowEvent" => {
            let ev = bcs::from_bytes::<BcsUnfollowEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "follower": ev.follower(),
                "unfollowed": ev.unfollowed(),
            })))
        }
        _ => Ok(None),
    }
}

fn parse_blocking_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "UserBlockEvent" | "UserBlockedEvent" => {
            let ev = bcs::from_bytes::<BcsUserBlockEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "blocker": ev.blocker(),
                "blocked": ev.blocked(),
            })))
        }
        "UserUnblockEvent" | "UserUnblockedEvent" => {
            let ev = bcs::from_bytes::<BcsUserUnblockEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "blocker": ev.blocker(),
                "unblocked": ev.unblocked(),
            })))
        }
        _ => Ok(None),
    }
}

fn parse_post_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "PostCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsPostCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "content": ev.content,
                "post_type": ev.post_type,
                "parent_post_id": ev.parent_post_id.as_ref().map(addr_to_string),
                "mentions": mentions_to_json(&ev.mentions),
                "media_urls": ev.media_urls,
                "metadata_json": ev.metadata_json,
                "mydata_id": ev.mydata_id.as_ref().map(addr_to_string),
                "promotion_id": ev.promotion_id.as_ref().map(addr_to_string),
                "revenue_redirect_to": ev.revenue_redirect_to.as_ref().map(addr_to_string),
                "revenue_redirect_percentage": ev.revenue_redirect_percentage,
                "enable_spt": ev.enable_spt,
                "enable_poc": ev.enable_poc,
                "enable_spot": ev.enable_spot,
                "spot_id": ev.spot_id.as_ref().map(addr_to_string),
                "spt_id": ev.spt_id.as_ref().map(addr_to_string),
            })))
        }
        "CommentCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsCommentCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "comment_id": addr_to_string(&ev.comment_id),
                "post_id": addr_to_string(&ev.post_id),
                "parent_comment_id": ev.parent_comment_id.as_ref().map(addr_to_string),
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "content": ev.content,
                "mentions": mentions_to_json(&ev.mentions),
            })))
        }
        "ReactionEvent" | "ReactionAddedEvent" => {
            let ev = bcs::from_bytes::<BcsReactionEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.object_id),
                "user_address": addr_to_string(&ev.user),
                "reaction_text": ev.reaction,
                "is_post": ev.is_post,
            })))
        }
        "RemoveReactionEvent" | "ReactionRemovedEvent" => {
            let ev = bcs::from_bytes::<BcsRemoveReactionEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.object_id),
                "user_address": addr_to_string(&ev.user),
                "reaction_text": ev.reaction,
                "is_post": ev.is_post,
            })))
        }
        "RepostEvent" | "RepostCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsRepostEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "repost_id": addr_to_string(&ev.repost_id),
                "original_id": addr_to_string(&ev.original_id),
                "original_post_id": addr_to_string(&ev.original_id),
                "is_original_post": ev.is_original_post,
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
            })))
        }
        "TipEvent" | "TipCreatedEvent" => {
            let ev =
                bcs::from_bytes::<BcsTipEvent>(contents).map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.object_id),
                "from": addr_to_string(&ev.from),
                "to": addr_to_string(&ev.to),
                "amount": ev.amount,
                "is_post": ev.is_post,
                "tip_time": 0u64,
            })))
        }
        "PostModerationEvent" | "PostModeratedEvent" | "CommentModerationEvent" => {
            let ev = bcs::from_bytes::<BcsPostModerationEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.post_id),
                "platform_id": addr_to_string(&ev.platform_id),
                "removed": ev.removed,
                "moderated_by": addr_to_string(&ev.moderated_by),
            })))
        }
        "PostReportedEvent" => {
            let ev = bcs::from_bytes::<BcsPostReportedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.post_id),
                "is_comment": false,
                "reporter": addr_to_string(&ev.reporter),
                "reason_code": ev.reason_code,
                "description": ev.description,
                "reported_at": ev.reported_at,
            })))
        }
        "CommentReportedEvent" => {
            let ev = bcs::from_bytes::<BcsCommentReportedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.comment_id),
                "is_comment": true,
                "reporter": addr_to_string(&ev.reporter),
                "reason_code": ev.reason_code,
                "description": ev.description,
                "reported_at": ev.reported_at,
            })))
        }
        "PostDeletedEvent" => {
            let ev = bcs::from_bytes::<BcsPostDeletedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.post_id),
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "is_post": true,
                "post_type": ev.post_type,
                "post_id": addr_to_string(&ev.post_id),
                "deleted_at": ev.deleted_at,
            })))
        }
        "CommentDeletedEvent" => {
            let ev = bcs::from_bytes::<BcsCommentDeletedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.comment_id),
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "is_post": false,
                "post_type": serde_json::Value::Null,
                "post_id": addr_to_string(&ev.post_id),
                "deleted_at": ev.deleted_at,
            })))
        }
        _ => Ok(None),
    }
}

fn parse_platform_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "PlatformCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsPlatformCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "name": ev.name,
                "tagline": ev.tagline,
                "description": ev.description,
                "developer": addr_to_string(&ev.developer),
                "logo": ev.logo,
                "terms_of_service": ev.terms_of_service,
                "privacy_policy": ev.privacy_policy,
                "platforms": ev.platforms,
                "links": ev.links,
                "primary_category": ev.primary_category,
                "secondary_category": ev.secondary_category,
                "status": {"status": ev.status.status},
                "release_date": ev.release_date,
                "wants_dao_governance": ev.wants_dao_governance,
                "governance_registry_id": ev.governance_registry_id.as_ref().map(addr_to_string),
                "delegate_count": ev.delegate_count,
                "delegate_term_epochs": ev.delegate_term_epochs,
                "proposal_submission_cost": ev.proposal_submission_cost,
                "min_on_chain_age_days": ev.min_on_chain_age_days,
                "max_votes_per_user": ev.max_votes_per_user,
                "quadratic_base_cost": ev.quadratic_base_cost,
                "voting_period_epochs": ev.voting_period_epochs,
                "quorum_votes": ev.quorum_votes,
            })))
        }
        "PlatformUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsPlatformUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "name": ev.name,
                "tagline": ev.tagline,
                "description": ev.description,
                "terms_of_service": ev.terms_of_service,
                "privacy_policy": ev.privacy_policy,
                "platforms": ev.platforms,
                "links": ev.links,
                "primary_category": ev.primary_category,
                "secondary_category": ev.secondary_category,
                "status": {"status": ev.status.status},
                "release_date": ev.release_date,
                "shutdown_date": ev.shutdown_date,
                "updated_at": ev.updated_at,
            })))
        }
        "PlatformApprovalChangedEvent" => {
            let ev = bcs::from_bytes::<BcsPlatformApprovalChangedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "approved": ev.approved,
                "changed_by": addr_to_string(&ev.changed_by),
                "reasoning": ev.reasoning,
            })))
        }
        "ModeratorAddedEvent" => {
            let ev = bcs::from_bytes::<BcsModeratorAddedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "moderator_address": addr_to_string(&ev.moderator_address),
                "added_by": addr_to_string(&ev.added_by),
            })))
        }
        "ModeratorRemovedEvent" => {
            let ev = bcs::from_bytes::<BcsModeratorRemovedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "moderator_address": addr_to_string(&ev.moderator_address),
                "removed_by": addr_to_string(&ev.removed_by),
            })))
        }
        "PlatformDeletedEvent" => {
            let ev = bcs::from_bytes::<BcsPlatformDeletedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "name": ev.name,
                "developer": addr_to_string(&ev.developer),
                "deleted_by": addr_to_string(&ev.deleted_by),
                "timestamp": ev.timestamp,
                "reasoning": ev.reasoning,
            })))
        }
        "UserJoinedPlatformEvent" => {
            let ev = bcs::from_bytes::<BcsUserJoinedPlatformEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "wallet_address": addr_to_string(&ev.wallet_address),
                "platform_id": addr_to_string(&ev.platform_id),
                "timestamp": ev.timestamp,
            })))
        }
        "UserLeftPlatformEvent" => {
            let ev = bcs::from_bytes::<BcsUserLeftPlatformEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "wallet_address": addr_to_string(&ev.wallet_address),
                "platform_id": addr_to_string(&ev.platform_id),
                "timestamp": ev.timestamp,
            })))
        }
        "TokenAirdropEvent" => {
            let ev = bcs::from_bytes::<BcsTokenAirdropEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "recipient": addr_to_string(&ev.recipient),
                "amount": ev.amount,
                "reason_code": ev.reason_code,
                "executed_by": addr_to_string(&ev.executed_by),
                "timestamp": ev.timestamp,
            })))
        }
        _ => Ok(None),
    }
}

fn parse_poc_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    let result = match event_name {
        "AnalysisSubmittedEvent" => {
            let ev = bcs::from_bytes::<BcsAnalysisSubmittedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "media_type": ev.media_type,
                "similarity_detected": ev.similarity_detected,
                "highest_similarity_score": ev.highest_similarity_score,
                "oracle_address": addr_to_string(&ev.oracle_address),
                "timestamp": ev.timestamp,
                "reasoning": ev.reasoning,
                "evidence_urls": ev.evidence_urls,
            })))
        }
        "PoCBadgeIssuedEvent" | "PocBadgeIssuedEvent" => {
            let ev = bcs::from_bytes::<BcsPocBadgeIssuedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "badge_id": addr_to_string(&ev.badge_id),
                "post_id": addr_to_string(&ev.post_id),
                "media_type": ev.media_type,
                "issued_by": addr_to_string(&ev.issued_by),
                "timestamp": ev.timestamp,
            })))
        }
        "RevenueRedirectionActivatedEvent" => {
            let ev = bcs::from_bytes::<BcsRevenueRedirectionActivatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "redirection_id": addr_to_string(&ev.redirection_id),
                "accused_post_id": addr_to_string(&ev.accused_post_id),
                "original_post_id": addr_to_string(&ev.original_post_id),
                "redirect_percentage": ev.redirect_percentage,
                "similarity_score": ev.similarity_score,
                "timestamp": ev.timestamp,
            })))
        }
        "PoCDisputeSubmittedEvent" | "PocDisputeSubmittedEvent" => {
            let ev = bcs::from_bytes::<BcsPocDisputeSubmittedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "dispute_id": addr_to_string(&ev.dispute_id),
                "post_id": addr_to_string(&ev.post_id),
                "disputer": addr_to_string(&ev.disputer),
                "dispute_type": ev.dispute_type,
                "stake_amount": ev.stake_amount,
                "voting_start_epoch": ev.voting_start_epoch,
                "voting_end_epoch": ev.voting_end_epoch,
                "timestamp": ev.timestamp,
            })))
        }
        "DisputeVoteCastEvent" => {
            let ev = bcs::from_bytes::<BcsDisputeVoteCastEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "dispute_id": addr_to_string(&ev.dispute_id),
                "voter": addr_to_string(&ev.voter),
                "vote_choice": ev.vote_choice,
                "stake_amount": ev.stake_amount,
                "total_uphold_stake": ev.total_uphold_stake,
                "total_overturn_stake": ev.total_overturn_stake,
                "timestamp": ev.timestamp,
            })))
        }
        "PoCDisputeResolvedEvent" | "PocDisputeResolvedEvent" => {
            let ev = bcs::from_bytes::<BcsPocDisputeResolvedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "dispute_id": addr_to_string(&ev.dispute_id),
                "post_id": addr_to_string(&ev.post_id),
                "resolution": ev.resolution,
                "winning_side": ev.winning_side,
                "total_winning_stake": ev.total_winning_stake,
                "total_losing_stake": ev.total_losing_stake,
                "badge_revoked": ev.badge_revoked,
                "redirection_removed": ev.redirection_removed,
                "timestamp": ev.timestamp,
            })))
        }
        "VotingRewardClaimedEvent" => {
            let ev = bcs::from_bytes::<BcsVotingRewardClaimedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "dispute_id": addr_to_string(&ev.dispute_id),
                "voter": addr_to_string(&ev.voter),
                "original_stake": ev.original_stake,
                "reward_amount": ev.reward_amount,
                "total_payout": ev.total_payout,
                "timestamp": ev.timestamp,
            })))
        }
        "PoCConfigUpdatedEvent" | "PocConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsPocConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "oracle_address": addr_to_string(&ev.oracle_address),
                "image_threshold": ev.image_threshold,
                "video_threshold": ev.video_threshold,
                "audio_threshold": ev.audio_threshold,
                "revenue_redirect_percentage": ev.revenue_redirect_percentage,
                "dispute_cost": ev.dispute_cost,
                "dispute_protocol_fee": ev.dispute_protocol_fee,
                "min_vote_stake": ev.min_vote_stake,
                "max_vote_stake": ev.max_vote_stake,
                "voting_duration_epochs": ev.voting_duration_epochs,
                "max_reasoning_length": ev.max_reasoning_length,
                "max_evidence_urls": ev.max_evidence_urls,
                "max_votes_per_dispute": ev.max_votes_per_dispute,
                "timestamp": ev.timestamp,
            })))
        }
        "TokenPoolSyncNeededEvent" => {
            let ev = bcs::from_bytes::<BcsTokenPoolSyncNeededEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "timestamp": ev.timestamp,
            })))
        }
        _ => Ok(None),
    };
    result
}

fn parse_mydata_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    let result = match event_name {
        "MyDataCreatedEvent" | "DataCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsMyDataCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "ip_id": addr_to_string(&ev.ip_id),
                "owner": addr_to_string(&ev.owner),
                "media_type": ev.media_type,
                "platform_id": ev.platform_id.as_ref().map(addr_to_string),
                "one_time_price": ev.one_time_price,
                "subscription_price": ev.subscription_price,
                "created_at": ev.created_at,
            })))
        }
        "PurchaseEvent" | "DataPurchasedEvent" => {
            let ev = bcs::from_bytes::<BcsPurchaseEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "ip_id": addr_to_string(&ev.ip_id),
                "buyer": addr_to_string(&ev.buyer),
                "price": ev.price,
                "purchase_type": ev.purchase_type,
                "timestamp": ev.timestamp,
            })))
        }
        "AccessGrantedEvent" | "DataAccessGrantedEvent" => {
            let ev = bcs::from_bytes::<BcsAccessGrantedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "ip_id": addr_to_string(&ev.ip_id),
                "user": addr_to_string(&ev.user),
                "access_type": ev.access_type,
                "granted_by": addr_to_string(&ev.granted_by),
                "timestamp": ev.timestamp,
            })))
        }
        "MyDataRegisteredEvent" | "IPRegisteredEvent" => {
            let ev = bcs::from_bytes::<BcsMyDataRegisteredEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "ip_id": addr_to_string(&ev.ip_id),
                "owner": addr_to_string(&ev.owner),
                "registered_at": ev.registered_at,
            })))
        }
        "MyDataUnregisteredEvent" | "IPUnregisteredEvent" => {
            let ev = bcs::from_bytes::<BcsMyDataUnregisteredEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "ip_id": addr_to_string(&ev.ip_id),
                "owner": addr_to_string(&ev.owner),
                "unregistered_at": ev.unregistered_at,
            })))
        }
        "MyDataConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsMyDataConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "enable_flag": ev.enable_flag,
                "max_tags": ev.max_tags,
                "max_subscription_days": ev.max_subscription_days,
                "max_free_access_grants": ev.max_free_access_grants,
                "timestamp": ev.timestamp,
            })))
        }
        _ => Ok(None),
    };
    result
}

fn parse_insurance_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    let result = match event_name {
        "ConfigInitializedEvent" => {
            let ev = bcs::from_bytes::<BcsConfigInitializedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "admin": addr_to_string(&ev.admin),
                "min_coverage_bps": ev.min_coverage_bps,
                "max_coverage_bps": ev.max_coverage_bps,
                "max_duration_ms": ev.max_duration_ms,
                "fee_bps": ev.fee_bps,
            })))
        }
        "ConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "enable_flag": ev.enable_flag,
                "min_coverage_bps": ev.min_coverage_bps,
                "max_coverage_bps": ev.max_coverage_bps,
                "max_duration_ms": ev.max_duration_ms,
                "fee_bps": ev.fee_bps,
                "timestamp": ev.timestamp,
            })))
        }
        "UnderwriterVaultCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsUnderwriterVaultCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "vault_id": addr_to_string(&ev.vault_id),
                "underwriter": addr_to_string(&ev.underwriter),
                "base_rate_bps_per_day": ev.base_rate_bps_per_day,
                "utilization_multiplier_bps": ev.utilization_multiplier_bps,
                "max_exposure_per_market": ev.max_exposure_per_market,
                "max_exposure_per_user": ev.max_exposure_per_user,
            })))
        }
        "UnderwriterVaultDepositedEvent" => {
            let ev = bcs::from_bytes::<BcsUnderwriterVaultDepositedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "vault_id": addr_to_string(&ev.vault_id),
                "amount": ev.amount,
                "new_balance": ev.new_balance,
            })))
        }
        "UnderwriterVaultWithdrawnEvent" => {
            let ev = bcs::from_bytes::<BcsUnderwriterVaultWithdrawnEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "vault_id": addr_to_string(&ev.vault_id),
                "amount": ev.amount,
                "new_balance": ev.new_balance,
            })))
        }
        "CoveragePurchasedEvent" => {
            let ev = bcs::from_bytes::<BcsCoveragePurchasedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "policy_id": addr_to_string(&ev.policy_id),
                "market_id": addr_to_string(&ev.market_id),
                "insured": addr_to_string(&ev.insured),
                "option_id": ev.option_id,
                "covered_amount": ev.covered_amount,
                "coverage_bps": ev.coverage_bps,
                "premium_paid": ev.premium_paid,
                "reserve_locked": ev.reserve_locked,
                "expiry_time_ms": ev.expiry_time_ms,
            })))
        }
        "CoverageCancelledEvent" => {
            let ev = bcs::from_bytes::<BcsCoverageCancelledEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "policy_id": addr_to_string(&ev.policy_id),
                "insured": addr_to_string(&ev.insured),
                "refunded_amount": ev.refunded_amount,
                "fee_paid": ev.fee_paid,
            })))
        }
        "CoverageClaimedEvent" => {
            let ev = bcs::from_bytes::<BcsCoverageClaimedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "policy_id": addr_to_string(&ev.policy_id),
                "insured": addr_to_string(&ev.insured),
                "payout": ev.payout,
            })))
        }
        "PolicyExpiredEvent" => {
            let ev = bcs::from_bytes::<BcsPolicyExpiredEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "policy_id": addr_to_string(&ev.policy_id),
                "insured": addr_to_string(&ev.insured),
                "market_id": addr_to_string(&ev.market_id),
                "vault_id": addr_to_string(&ev.vault_id),
                "reserve_released": ev.reserve_released,
                "expiry_time_ms": ev.expiry_time_ms,
            })))
        }
        _ => Ok(None),
    };
    result
}

fn parse_spot_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    let result = match event_name {
        "SpotBetPlacedEvent" | "BetPlacedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotBetPlacedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "user": addr_to_string(&ev.user),
                "option_id": ev.option_id,
                "amount": ev.amount,
            })))
        }
        "SpotResolvedEvent" | "ResolvedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotResolvedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "outcome": ev.outcome,
                "total_escrow": ev.total_escrow,
                "fee_taken": ev.fee_taken,
                "reasoning": ev.reasoning,
                "evidence_urls": ev.evidence_urls,
            })))
        }
        "SpotDaoRequiredEvent" | "DaoRequiredEvent" => {
            let ev = bcs::from_bytes::<BcsSpotDaoRequiredEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "confidence_bps": ev.confidence_bps,
                "reasoning": ev.reasoning,
            })))
        }
        "SpotPayoutEvent" | "PayoutEvent" => {
            let ev = bcs::from_bytes::<BcsSpotPayoutEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "user": addr_to_string(&ev.user),
                "amount": ev.amount,
            })))
        }
        "SpotRefundEvent" | "RefundEvent" => {
            let ev = bcs::from_bytes::<BcsSpotRefundEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "user": addr_to_string(&ev.user),
                "amount": ev.amount,
            })))
        }
        "SpotConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "enable_flag": ev.enable_flag,
                "confidence_threshold_bps": ev.confidence_threshold_bps,
                "resolution_window_epochs": ev.resolution_window_epochs,
                "max_resolution_window_epochs": ev.max_resolution_window_epochs,
                "payout_delay_ms": ev.payout_delay_ms,
                "fee_bps": ev.fee_bps,
                "fee_split_bps_platform": ev.fee_split_bps_platform,
                "oracle_address": addr_to_string(&ev.oracle_address),
                "max_single_bet": ev.max_single_bet,
                "max_bets_per_record": ev.max_bets_per_record,
                "timestamp": ev.timestamp,
            })))
        }
        "SpotBetWithdrawnEvent" | "BetWithdrawnEvent" => {
            let ev = bcs::from_bytes::<BcsSpotBetWithdrawnEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "user": addr_to_string(&ev.user),
                "option_id": ev.option_id,
                "amount": ev.amount,
                "fee_taken": ev.fee_taken,
            })))
        }
        "SpotRecordCreatedEvent" | "RecordCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotRecordCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "record_id": addr_to_string(&ev.record_id),
                "post_id": addr_to_string(&ev.post_id),
                "created_epoch": ev.created_epoch,
                "betting_options": ev.betting_options,
                "resolution_window_epochs": ev.resolution_window_epochs,
                "max_resolution_window_epochs": ev.max_resolution_window_epochs,
            })))
        }
        _ => Ok(None),
    };
    result
}

fn parse_spt_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    let result = match event_name {
        "TokenPoolCreatedEvent" | "PoolCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsTokenPoolCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "id": addr_to_string(&ev.id),
                "token_type": ev.token_type,
                "owner": addr_to_string(&ev.owner),
                "associated_id": addr_to_string(&ev.associated_id),
                "symbol": ev.symbol,
                "name": ev.name,
                "base_price": ev.base_price,
                "quadratic_coefficient": ev.quadratic_coefficient,
            })))
        }
        "TokenBoughtEvent" | "BuyEvent" => {
            let ev = bcs::from_bytes::<BcsTokenBoughtEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "id": addr_to_string(&ev.id),
                "buyer": addr_to_string(&ev.buyer),
                "amount": ev.amount,
                "myso_amount": ev.myso_amount,
                "fee_amount": ev.fee_amount,
                "creator_fee": ev.creator_fee,
                "platform_fee": ev.platform_fee,
                "treasury_fee": ev.treasury_fee,
                "new_price": ev.new_price,
            })))
        }
        "TokenSoldEvent" | "SellEvent" => {
            let ev = bcs::from_bytes::<BcsTokenSoldEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "id": addr_to_string(&ev.id),
                "seller": addr_to_string(&ev.seller),
                "amount": ev.amount,
                "myso_amount": ev.myso_amount,
                "fee_amount": ev.fee_amount,
                "creator_fee": ev.creator_fee,
                "platform_fee": ev.platform_fee,
                "treasury_fee": ev.treasury_fee,
                "new_price": ev.new_price,
            })))
        }
        "ReservationCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsReservationCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "associated_id": addr_to_string(&ev.associated_id),
                "token_type": ev.token_type,
                "reserver": addr_to_string(&ev.reserver),
                "amount": ev.amount,
                "total_reserved": ev.total_reserved,
                "threshold_met": ev.threshold_met,
                "reserved_at": ev.reserved_at,
                "fee_amount": ev.fee_amount,
                "creator_fee": ev.creator_fee,
                "platform_fee": ev.platform_fee,
                "treasury_fee": ev.treasury_fee,
            })))
        }
        "ReservationWithdrawnEvent" => {
            let ev = bcs::from_bytes::<BcsReservationWithdrawnEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "associated_id": addr_to_string(&ev.associated_id),
                "token_type": ev.token_type,
                "reserver": addr_to_string(&ev.reserver),
                "amount": ev.amount,
                "total_reserved": ev.total_reserved,
                "withdrawn_at": ev.withdrawn_at,
                "fee_amount": ev.fee_amount,
                "creator_fee": ev.creator_fee,
                "platform_fee": ev.platform_fee,
                "treasury_fee": ev.treasury_fee,
            })))
        }
        "ThresholdMetEvent" => {
            let ev = bcs::from_bytes::<BcsThresholdMetEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "associated_id": addr_to_string(&ev.associated_id),
                "token_type": ev.token_type,
                "owner": addr_to_string(&ev.owner),
                "total_reserved": ev.total_reserved,
                "required_threshold": ev.required_threshold,
                "timestamp": ev.timestamp,
            })))
        }
        "ReservationPoolCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsReservationPoolCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "associated_id": addr_to_string(&ev.associated_id),
                "token_type": ev.token_type,
                "owner": addr_to_string(&ev.owner),
                "required_threshold": ev.required_threshold,
                "pool_object_id": addr_to_string(&ev.pool_object_id),
                "created_at": ev.created_at,
            })))
        }
        "ConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsSptConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "timestamp": ev.timestamp,
                "total_fee_bps": ev.total_fee_bps,
                "trading_creator_fee_bps": ev.trading_creator_fee_bps,
                "trading_platform_fee_bps": ev.trading_platform_fee_bps,
                "trading_treasury_fee_bps": ev.trading_treasury_fee_bps,
                "reservation_total_fee_bps": ev.reservation_total_fee_bps,
                "reservation_creator_fee_bps": ev.reservation_creator_fee_bps,
                "reservation_platform_fee_bps": ev.reservation_platform_fee_bps,
                "reservation_treasury_fee_bps": ev.reservation_treasury_fee_bps,
                "base_price": ev.base_price,
                "quadratic_coefficient": ev.quadratic_coefficient,
                "max_hold_percent_bps": ev.max_hold_percent_bps,
                "post_threshold": ev.post_threshold,
                "profile_threshold": ev.profile_threshold,
                "max_individual_reservation_bps": ev.max_individual_reservation_bps,
                "max_reservers_per_pool": ev.max_reservers_per_pool,
            })))
        }
        "TokensAddedEvent" => {
            let ev = bcs::from_bytes::<BcsTokensAddedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "owner": addr_to_string(&ev.owner),
                "pool_id": addr_to_string(&ev.pool_id),
                "amount": ev.amount,
            })))
        }
        "EmergencyKillSwitchEvent" => {
            let ev = bcs::from_bytes::<BcsEmergencyKillSwitchEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "admin": addr_to_string(&ev.admin),
                "trading_enabled": ev.trading_enabled,
                "timestamp": ev.timestamp,
                "reason": ev.reason,
            })))
        }
        "PocRedirectionUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsPocRedirectionUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "pool_id": addr_to_string(&ev.pool_id),
                "post_id": addr_to_string(&ev.post_id),
                "redirect_to": ev.redirect_to.as_ref().map(addr_to_string),
                "redirect_percentage": ev.redirect_percentage,
                "updated_by": addr_to_string(&ev.updated_by),
                "timestamp": ev.timestamp,
            })))
        }
        _ => Ok(None),
    };
    result
}

fn parse_subscription_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    let result = match event_name {
        "ProfileSubscriptionServiceCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileSubscriptionServiceCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "service_id": addr_to_string(&ev.service_id),
                "profile_owner": addr_to_string(&ev.profile_owner),
                "monthly_fee": ev.monthly_fee,
                "created_at": ev.created_at,
            })))
        }
        "ProfileSubscriptionCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileSubscriptionCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "service_id": addr_to_string(&ev.service_id),
                "subscriber": addr_to_string(&ev.subscriber),
                "expires_at": ev.expires_at,
                "monthly_fee": ev.monthly_fee,
                "auto_renew": ev.auto_renew,
            })))
        }
        "ProfileSubscriptionRenewedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileSubscriptionRenewedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "subscription_id": addr_to_string(&ev.subscription_id),
                "subscriber": addr_to_string(&ev.subscriber),
                "new_expires_at": ev.new_expires_at,
                "renewal_count": ev.renewal_count,
                "auto_renewed": ev.auto_renewed,
            })))
        }
        "ProfileSubscriptionCancelledEvent" => {
            let ev = bcs::from_bytes::<BcsProfileSubscriptionCancelledEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "subscription_id": addr_to_string(&ev.subscription_id),
                "subscriber": addr_to_string(&ev.subscriber),
                "refunded_amount": ev.refunded_amount,
            })))
        }
        "ProfileSubscriptionUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileSubscriptionUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "service_id": addr_to_string(&ev.service_id),
                "old_fee": ev.old_fee,
                "new_fee": ev.new_fee,
                "updated_by": addr_to_string(&ev.updated_by),
            })))
        }
        "RenewalBalanceFundedEvent" => {
            let ev = bcs::from_bytes::<BcsRenewalBalanceFundedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "subscription_id": addr_to_string(&ev.subscription_id),
                "subscriber": addr_to_string(&ev.subscriber),
                "funded_amount": ev.funded_amount,
                "new_balance": ev.new_balance,
                "timestamp": ev.timestamp,
            })))
        }
        "ProfileSubscriptionServiceDeactivatedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileSubscriptionServiceDeactivatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "service_id": addr_to_string(&ev.service_id),
                "profile_owner": addr_to_string(&ev.profile_owner),
                "deactivated_at": ev.deactivated_at,
            })))
        }
        _ => Ok(None),
    };
    result
}

fn parse_upgrade_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    let result = match event_name {
        "UpgradeEvent" => {
            let ev = bcs::from_bytes::<BcsUpgradeEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "package_id": addr_to_string(&ev.package_id),
                "version": ev.version,
            })))
        }
        "ObjectMigratedEvent" => {
            let ev = bcs::from_bytes::<BcsObjectMigratedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.object_id),
                "object_type": ev.object_type,
                "old_version": ev.old_version,
                "new_version": ev.new_version,
                "migrated_by": addr_to_string(&ev.migrated_by),
            })))
        }
        _ => Ok(None),
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_profile_created_event() {
        let contents: Vec<u8> = vec![
            217, 0, 116, 168, 192, 241, 38, 98, 208, 170, 122, 181, 129, 167, 139, 149, 123, 82,
            250, 151, 203, 248, 61, 180, 210, 122, 244, 238, 112, 6, 36, 122, 12, 66, 114, 97, 110,
            100, 111, 110, 32, 83, 104, 97, 119, 7, 98, 114, 97, 110, 100, 111, 110, 36, 87, 101,
            98, 56, 32, 100, 101, 118, 101, 108, 111, 112, 101, 114, 32, 97, 110, 100, 32, 99, 114,
            121, 112, 116, 111, 32, 101, 110, 116, 104, 117, 115, 105, 97, 115, 116, 1, 31, 104,
            116, 116, 112, 115, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47,
            112, 114, 111, 102, 105, 108, 101, 46, 106, 112, 103, 1, 29, 104, 116, 116, 112, 115,
            58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47, 99, 111, 118, 101,
            114, 46, 112, 110, 103, 156, 200, 104, 135, 157, 106, 255, 74, 171, 250, 160, 27, 141,
            86, 246, 73, 253, 178, 164, 32, 199, 252, 9, 96, 225, 249, 235, 52, 206, 192, 0, 124,
            5, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = parse_event_contents("profile", "ProfileCreatedEvent", &contents);
        assert!(
            result.is_ok(),
            "parse_event_contents should succeed for ProfileCreatedEvent"
        );
        let json = result.unwrap();
        assert_eq!(json["display_name"], "Brandon Shaw");
        assert_eq!(json["username"], "brandon");
        assert_eq!(json["bio"], "Web8 developer and crypto enthusiast");
        assert_eq!(json["created_at"], 5);
        assert!(json["owner_address"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn test_parse_profile_created_event_user_bytes() {
        let contents: Vec<u8> = vec![
            106, 84, 17, 200, 209, 12, 198, 212, 190, 149, 151, 187, 181, 214, 161, 0, 95, 58, 225,
            214, 6, 214, 120, 6, 10, 193, 238, 125, 190, 143, 172, 168, 12, 66, 114, 97, 110, 100,
            111, 110, 32, 83, 104, 97, 119, 7, 98, 114, 97, 110, 100, 111, 110, 36, 87, 101, 98,
            56, 32, 100, 101, 118, 101, 108, 111, 112, 101, 114, 32, 97, 110, 100, 32, 99, 114,
            121, 112, 116, 111, 32, 101, 110, 116, 104, 117, 115, 105, 97, 115, 116, 1, 31, 104,
            116, 116, 112, 115, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47,
            112, 114, 111, 102, 105, 108, 101, 46, 106, 112, 103, 1, 29, 104, 116, 116, 112, 115,
            58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47, 99, 111, 118, 101,
            114, 46, 112, 110, 103, 156, 200, 104, 135, 157, 106, 255, 74, 171, 250, 160, 27, 141,
            86, 246, 73, 253, 178, 164, 32, 199, 252, 9, 96, 225, 249, 235, 52, 206, 192, 0, 124,
            8, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = parse_event_contents("profile", "ProfileCreatedEvent", &contents);
        assert!(
            result.is_ok(),
            "parse_event_contents should succeed for user's ProfileCreatedEvent bytes"
        );
        let json = result.unwrap();
        assert_eq!(json["display_name"], "Brandon Shaw");
        assert_eq!(json["username"], "brandon");
        assert_eq!(json["bio"], "Web8 developer and crypto enthusiast");
        assert_eq!(json["created_at"], 8);
        assert!(json["owner_address"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn test_bcs_option_string_encoding() {
        use move_core_types::account_address::AccountAddress;
        let addr = AccountAddress::from_hex_literal("0x50c1").unwrap();
        let ev = BcsProfileCreatedEvent {
            profile_id: addr,
            display_name: "Test".to_string(),
            username: "test".to_string(),
            bio: "".to_string(),
            profile_picture: None,
            cover_photo: None,
            owner: addr,
            created_at: 1,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize");
        let ev2: BcsProfileCreatedEvent = bcs::from_bytes(&bytes).expect("deserialize");
        assert_eq!(ev2.profile_picture, None);
        assert_eq!(ev2.cover_photo, None);

        let ev_some = BcsProfileCreatedEvent {
            profile_picture: Some("https://example.com/img.png".to_string()),
            cover_photo: Some("https://example.com/cover.png".to_string()),
            ..ev
        };
        let bytes_some = bcs::to_bytes(&ev_some).expect("serialize");
        let ev_some2: BcsProfileCreatedEvent = bcs::from_bytes(&bytes_some).expect("deserialize");
        assert_eq!(ev_some2.profile_picture, ev_some.profile_picture);
        assert_eq!(ev_some2.cover_photo, ev_some.cover_photo);
    }

    #[test]
    fn test_parse_profile_created_event_from_live_transaction() {
        let contents: Vec<u8> = vec![
            139, 177, 144, 228, 18, 55, 143, 78, 24, 197, 72, 170, 170, 104, 23, 44, 125, 207, 190,
            22, 208, 90, 199, 66, 59, 154, 162, 74, 201, 44, 94, 231, 12, 66, 114, 97, 110, 100,
            111, 110, 32, 83, 104, 97, 119, 7, 98, 114, 97, 110, 100, 111, 110, 36, 87, 101, 98,
            56, 32, 100, 101, 118, 101, 108, 111, 112, 101, 114, 32, 97, 110, 100, 32, 99, 114,
            121, 112, 116, 111, 32, 101, 110, 116, 104, 117, 115, 105, 97, 115, 116, 1, 31, 104,
            116, 116, 112, 115, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47,
            112, 114, 111, 102, 105, 108, 101, 46, 106, 112, 103, 1, 29, 104, 116, 116, 112, 115,
            58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47, 99, 111, 118, 101,
            114, 46, 112, 110, 103, 156, 200, 104, 135, 157, 106, 255, 74, 171, 250, 160, 27, 141,
            86, 246, 73, 253, 178, 164, 32, 199, 252, 9, 96, 225, 249, 235, 52, 206, 192, 0, 124,
            1, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = parse_event_contents("profile", "ProfileCreatedEvent", &contents);
        assert!(
            result.is_ok(),
            "parse_event_contents should succeed for live ProfileCreatedEvent bytes"
        );
        let json = result.unwrap();
        assert_eq!(json["display_name"], "Brandon Shaw");
        assert_eq!(json["username"], "brandon");
        assert_eq!(json["bio"], "Web8 developer and crypto enthusiast");
        assert_eq!(json["profile_picture"], "https://example.com/profile.jpg");
        assert_eq!(json["cover_photo"], "https://example.com/cover.png");
    }

    #[test]
    fn test_bcs_profile_created_event_round_trip() {
        let contents: Vec<u8> = vec![
            217, 0, 116, 168, 192, 241, 38, 98, 208, 170, 122, 181, 129, 167, 139, 149, 123, 82,
            250, 151, 203, 248, 61, 180, 210, 122, 244, 238, 112, 6, 36, 122, 12, 66, 114, 97, 110,
            100, 111, 110, 32, 83, 104, 97, 119, 7, 98, 114, 97, 110, 100, 111, 110, 36, 87, 101,
            98, 56, 32, 100, 101, 118, 101, 108, 111, 112, 101, 114, 32, 97, 110, 100, 32, 99, 114,
            121, 112, 116, 111, 32, 101, 110, 116, 104, 117, 115, 105, 97, 115, 116, 1, 31, 104,
            116, 116, 112, 115, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47,
            112, 114, 111, 102, 105, 108, 101, 46, 106, 112, 103, 1, 29, 104, 116, 116, 112, 115,
            58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47, 99, 111, 118, 101,
            114, 46, 112, 110, 103, 156, 200, 104, 135, 157, 106, 255, 74, 171, 250, 160, 27, 141,
            86, 246, 73, 253, 178, 164, 32, 199, 252, 9, 96, 225, 249, 235, 52, 206, 192, 0, 124,
            5, 0, 0, 0, 0, 0, 0, 0,
        ];
        let ev: BcsProfileCreatedEvent =
            bcs::from_bytes(&contents).expect("fixture bytes should deserialize");
        let serialized = bcs::to_bytes(&ev).expect("BcsProfileCreatedEvent should serialize");
        let ev_round_trip: BcsProfileCreatedEvent =
            bcs::from_bytes(&serialized).expect("round-trip bytes should deserialize");
        assert_eq!(ev.created_at, ev_round_trip.created_at);
        assert_eq!(ev.display_name, ev_round_trip.display_name);
        assert_eq!(ev.username, ev_round_trip.username);
        assert_eq!(ev.bio, ev_round_trip.bio);
        assert_eq!(ev.profile_picture, ev_round_trip.profile_picture);
        assert_eq!(ev.cover_photo, ev_round_trip.cover_photo);
    }

    #[test]
    fn test_parse_post_created_event_json_fallback() {
        let json = r#"{"post_id":"0x123","owner":"0x456","profile_id":"0x789","content":"hello","post_type":"post","parent_post_id":null,"mentions":null,"media_urls":null,"metadata_json":null,"mydata_id":null,"promotion_id":null,"revenue_redirect_to":null,"revenue_redirect_percentage":null,"enable_spt":false,"enable_poc":false,"enable_spot":false,"spot_id":null,"spt_id":null}"#;
        let result = parse_event_contents("post", "PostCreatedEvent", json.as_bytes());
        assert!(
            result.is_ok(),
            "parse_event_contents should succeed for PostCreatedEvent JSON"
        );
        let parsed = result.unwrap();
        assert_eq!(parsed["post_id"], "0x123");
        assert_eq!(parsed["content"], "hello");
    }

    #[test]
    fn test_parse_platform_created_event_json_fallback() {
        let json = r#"{"platform_id":"0xabc","name":"Test","tagline":"Tag","description":"Desc","developer":"0xdef","logo":"","terms_of_service":"","privacy_policy":"","platforms":[],"links":[],"primary_category":"Social","secondary_category":null,"status":{"status":0},"release_date":"2024-01-01","wants_dao_governance":false,"governance_registry_id":null,"delegate_count":null,"delegate_term_epochs":null,"proposal_submission_cost":null,"min_on_chain_age_days":null,"max_votes_per_user":null,"quadratic_base_cost":null,"voting_period_epochs":null,"quorum_votes":null}"#;
        let result = parse_event_contents("platform", "PlatformCreatedEvent", json.as_bytes());
        assert!(
            result.is_ok(),
            "parse_event_contents should succeed for PlatformCreatedEvent JSON"
        );
        let parsed = result.unwrap();
        assert_eq!(parsed["platform_id"], "0xabc");
        assert_eq!(parsed["name"], "Test");
    }

    #[test]
    fn test_parse_governance_registry_created_event() {
        let contents: Vec<u8> = vec![
            100, 127, 122, 106, 253, 121, 113, 7, 80, 14, 70, 113, 185, 17, 144, 225, 233, 109,
            169, 54, 79, 166, 56, 144, 151, 211, 15, 115, 163, 148, 80, 156, 0, 3, 0, 0, 0, 0, 0,
            0, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 225, 245, 5, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0,
            128, 150, 152, 0, 0, 0, 0, 0, 0, 132, 12, 36, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 1,
            205, 99, 86, 156, 1, 0, 0,
        ];
        let result =
            parse_event_contents("governance", "GovernanceRegistryCreatedEvent", &contents);
        assert!(result.is_ok(), "parse_event_contents should succeed");
        let json = result.unwrap();
        assert_eq!(json["registry_type"], 0);
        assert_eq!(json["delegate_count"], 3);
        assert_eq!(json["delegate_term_epochs"], 90);
        assert_eq!(json["proposal_submission_cost"], 100_000_000);
        assert_eq!(json["max_votes_per_user"], 10);
        assert_eq!(json["quadratic_base_cost"], 10_000_000);
        assert_eq!(json["quorum_votes"], 20);
        assert!(json["registry_id"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn test_parse_subscription_events_bcs_round_trip() {
        use move_core_types::account_address::AccountAddress;

        let addr1 = AccountAddress::from_hex_literal("0x1").unwrap();
        let addr2 = AccountAddress::from_hex_literal("0x2").unwrap();

        let ev = BcsProfileSubscriptionServiceCreatedEvent {
            service_id: addr1,
            profile_owner: addr2,
            monthly_fee: 1000,
            created_at: 12345,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize");
        let result = parse_event_contents(
            "subscription",
            "ProfileSubscriptionServiceCreatedEvent",
            &bytes,
        );
        assert!(result.is_ok(), "BCS parse should succeed");
        let json = result.unwrap();
        assert!(json["service_id"].as_str().unwrap().starts_with("0x"));
        assert_eq!(json["monthly_fee"], 1000);
        assert_eq!(json["created_at"], 12345);

        let ev2 = BcsProfileSubscriptionCreatedEvent {
            service_id: addr1,
            subscriber: addr2,
            expires_at: 99999,
            monthly_fee: 500,
            auto_renew: true,
        };
        let bytes2 = bcs::to_bytes(&ev2).expect("serialize");
        let result2 =
            parse_event_contents("subscription", "ProfileSubscriptionCreatedEvent", &bytes2);
        assert!(result2.is_ok());
        let json2 = result2.unwrap();
        assert_eq!(json2["expires_at"], 99999);
        assert_eq!(json2["monthly_fee"], 500);
        assert_eq!(json2["auto_renew"], true);
    }

    #[test]
    fn test_parse_subscription_events_json_fallback() {
        let json = r#"{"service_id":"0x123","subscriber":"0x456","expires_at":1000,"monthly_fee":100,"auto_renew":true}"#;
        let result = parse_event_contents(
            "subscription",
            "ProfileSubscriptionCreatedEvent",
            json.as_bytes(),
        );
        assert!(result.is_ok(), "JSON fallback should succeed");
        let parsed = result.unwrap();
        assert_eq!(parsed["service_id"], "0x123");
        assert_eq!(parsed["monthly_fee"], 100);
        assert_eq!(parsed["auto_renew"], true);
    }
}
