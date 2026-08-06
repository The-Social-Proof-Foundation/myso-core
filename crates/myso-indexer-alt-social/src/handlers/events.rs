// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! BCS-compatible event structs and parsing for myso-social Move events.
//! Field order must match the Move struct definitions exactly.

use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

use super::access::{self, post_access_json_from_bcs, BcsPostAccess};

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

impl std::fmt::Display for EventParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (hex preview: {})",
            self.error,
            self.contents_hex_preview(32)
        )
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

/// Move `myso::object::ID` BCS layout (`bytes: address`).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BcsMoveObjectId {
    bytes: AccountAddress,
}

fn move_object_id_to_string(id: &BcsMoveObjectId) -> String {
    addr_to_string(&id.bytes)
}

fn optional_move_object_id_json(id: &Option<BcsMoveObjectId>) -> Option<String> {
    id.as_ref().map(move_object_id_to_string)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileCreatedEvent {
    profile_id: AccountAddress,
    display_name: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPaidMessagingPolicyUpdated {
    pub wallet: AccountAddress,
    pub enabled: bool,
    pub min_cost: Option<u64>,
}

impl BcsPaidMessagingPolicyUpdated {
    pub fn wallet(&self) -> String {
        addr_to_string(&self.wallet)
    }
}

#[derive(Debug, Deserialize)]
pub struct BcsMessagingConfigUpdatedEvent {
    updated_by: AccountAddress,
    timestamp: u64,
    paid_msg_platform_fee_bps: u64,
    paid_msg_treasury_fee_bps: u64,
    payment_expiration_ms: u64,
    min_reply_chars: u32,
    max_dedupe_key_bytes: u64,
}

#[derive(Debug, Deserialize)]
struct BcsPaidMessageSent {
    group_id: BcsMoveObjectId,
    seq: u64,
    payer: AccountAddress,
    recipient: AccountAddress,
    amount: u64,
    created_at_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsMessageDigestSent {
    group_id: BcsMoveObjectId,
    seq: u64,
    sender: AccountAddress,
    recipient: AccountAddress,
    content_digest: Vec<u8>,
    content_uri: String,
    created_at_ms: u64,
}

#[derive(Debug, Deserialize)]
struct BcsPaidMessageReplied {
    group_id: BcsMoveObjectId,
    paid_msg_seq: u64,
    recipient: AccountAddress,
    reply_char_count: u32,
}

#[derive(Debug, Deserialize)]
struct BcsPaymentClaimed {
    group_id: BcsMoveObjectId,
    seq: u64,
    recipient: AccountAddress,
    amount: u64,
    claimed_at_ms: u64,
}

#[derive(Debug, Deserialize)]
struct BcsPaymentClaimedSettled {
    group_id: BcsMoveObjectId,
    seq: u64,
    recipient: AccountAddress,
    total_amount: u64,
    platform_fee: u64,
    treasury_fee: u64,
    net_amount: u64,
    platform_fee_recipient: AccountAddress,
    ecosystem_fee_recipient: AccountAddress,
    claimed_at_ms: u64,
}

#[derive(Debug, Deserialize)]
struct BcsPaymentRefunded {
    group_id: BcsMoveObjectId,
    seq: u64,
    payer: AccountAddress,
    amount: u64,
    refunded_at_ms: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAgentGroupCreated {
    group_id: BcsMoveObjectId,
    creator_actor: AccountAddress,
    creator_principal: AccountAddress,
    creator_sub_agent_id: Option<BcsMoveObjectId>,
    creator_identity_class: u64,
    organization_id: Option<BcsMoveObjectId>,
    group_name: String,
    group_uuid: String,
    created_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsEcosystemTreasuryUpdatedEvent {
    updated_by: AccountAddress,
    new_treasury_address: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsProfileConfigUpdatedEvent {
    updated_by: AccountAddress,
    max_vesting_pieces: u64,
    curve_factor_min: u64,
    curve_factor_max: u64,
    curve_precision: u64,
    min_claim_threshold_divisor: u64,
    min_username_length: u64,
    max_username_length: u64,
    username_sale_fee_bps: u64,
    timestamp: u64,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsDelegateNominatedEvent {
    nominee_address: AccountAddress,
    scheduled_term_start_epoch: u64,
    registry_type: u8,
}

/// Pre-vote-counts on-chain layout (BCS replay compatibility).
#[derive(Debug, Deserialize, Serialize)]
pub struct BcsDelegateElectedEventV0 {
    delegate_address: AccountAddress,
    term_start: u64,
    term_end: u64,
    registry_type: u8,
}

/// BCS byte length for [`BcsDelegateElectedEventV0`]: 32 (address) + 8 + 8 + 1.
const DELEGATE_ELECTED_BCS_V0_LEN: usize = 32 + 8 + 8 + 1;

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsDelegateElectedEvent {
    delegate_address: AccountAddress,
    term_start: u64,
    term_end: u64,
    registry_type: u8,
    upvotes: u64,
    downvotes: u64,
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
pub struct BcsDelegateVoteClearedEvent {
    target_address: AccountAddress,
    voter: AccountAddress,
    is_active_delegate: bool,
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
pub struct BcsProposalRewardPoolForfeitedToTreasuryEvent {
    proposal_id: AccountAddress,
    recipient: AccountAddress,
    amount: u64,
    reason: u8,
    registry_type: u8,
    treasury_route: u8,
    forfeited_time: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsProposalImplementationRewardToSubmitterEvent {
    proposal_id: AccountAddress,
    submitter: AccountAddress,
    amount: u64,
    registry_type: u8,
    sent_time: u64,
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
pub struct BcsDelegatePanelRefreshedEvent {
    registry_id: AccountAddress,
    boundary_epoch: u64,
    registry_type: u8,
    executed_at_epoch: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsProfileUpdatedEvent {
    profile_id: AccountAddress,
    display_name: Option<String>,
    bio: String,
    profile_picture: Option<String>,
    cover_photo: Option<String>,
    owner: AccountAddress,
    updated_at: u64,
    x_username: Option<String>,
    website: Option<String>,
    birthdate: Option<String>,
    location: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsUsernameClaimedEvent {
    username: String,
    profile_id: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUsernameReservedEvent {
    pub(crate) username: String,
    pub(crate) reason: u8,
    pub(crate) reserved_by: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUsernameReleasedEvent {
    pub(crate) username: String,
    pub(crate) reason: u8,
    pub(crate) released_by: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsUsernameReassignedEvent {
    username: String,
    profile_id: AccountAddress,
    admin: AccountAddress,
    reason_code: u8,
    prior_username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsUsernameListingCreatedEvent {
    username: String,
    seller: AccountAddress,
    seller_profile_id: AccountAddress,
    min_price: u64,
    created_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsUsernameListingCancelledEvent {
    username: String,
    seller: AccountAddress,
    seller_profile_id: AccountAddress,
    cancelled_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsUsernameOfferCreatedEvent {
    username: String,
    seller_profile_id: AccountAddress,
    buyer: AccountAddress,
    buyer_profile_id: AccountAddress,
    amount: u64,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUsernameOfferAcceptedEvent {
    pub(crate) username: String,
    pub(crate) replacement_username: String,
    pub(crate) seller: AccountAddress,
    pub(crate) seller_profile_id: AccountAddress,
    pub(crate) buyer: AccountAddress,
    pub(crate) buyer_profile_id: AccountAddress,
    pub(crate) amount: u64,
    pub(crate) accepted_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsUsernameOfferRejectedEvent {
    username: String,
    seller_profile_id: AccountAddress,
    buyer: AccountAddress,
    buyer_profile_id: AccountAddress,
    rejected_by: AccountAddress,
    amount: u64,
    rejected_at: u64,
    is_revoked: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUsernameSaleSettledEvent {
    pub(crate) listed_username: String,
    pub(crate) replacement_username: String,
    pub(crate) seller: AccountAddress,
    pub(crate) seller_profile_id: AccountAddress,
    pub(crate) buyer: AccountAddress,
    pub(crate) buyer_profile_id: AccountAddress,
    pub(crate) amount: u64,
    pub(crate) settled_at: u64,
    pub(crate) prior_buyer_username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsUsernameSaleFeeEvent {
    username: String,
    seller: AccountAddress,
    seller_profile_id: AccountAddress,
    buyer: AccountAddress,
    buyer_profile_id: AccountAddress,
    sale_amount: u64,
    fee_amount: u64,
    fee_recipient: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileXUsernameUpdatedEvent {
    pub(crate) profile_id: AccountAddress,
    pub(crate) owner: AccountAddress,
    pub(crate) x_username: Option<String>,
    pub(crate) updated_by: AccountAddress,
    pub(crate) updated_at: u64,
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
pub struct BcsEcosystemBadgeSelectionClearedEvent {
    profile_id: AccountAddress,
    cleared_by: AccountAddress,
    cleared_at: u64,
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
pub struct BcsVestingPieceEvent {
    kind: u8,
    time_offset: u64,
    duration: u64,
    amount_bps: u64,
    curve_factor: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsTokensVestedEvent {
    wallet_id: AccountAddress,
    owner: AccountAddress,
    total_amount: u64,
    start_time: u64,
    schedule_end: u64,
    pieces: Vec<BcsVestingPieceEvent>,
    vested_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsTokensClaimedEvent {
    wallet_id: AccountAddress,
    owner: AccountAddress,
    claimed_amount: u64,
    remaining_balance: u64,
    claimed_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsVestingWalletDeletedEvent {
    wallet_id: AccountAddress,
    owner: AccountAddress,
    deleted_at: u64,
}

/// Matches Move `post::PostCreatedEvent`.
#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPostCreatedEvent {
    post_id: AccountAddress,
    owner: AccountAddress,
    profile_id: AccountAddress,
    platform_id: AccountAddress,
    permissions: u8,
    content: String,
    post_type: String,
    parent_post_id: Option<AccountAddress>,
    mentions: Option<Vec<AccountAddress>>,
    media_urls: Option<Vec<String>>,
    metadata_json: Option<String>,
    access: BcsPostAccess,
    promotion_id: Option<AccountAddress>,
    revenue_redirect_to: Option<AccountAddress>,
    revenue_redirect_percentage: Option<u64>,
    enable_spt: bool,
    spt_id: Option<AccountAddress>,
    poc_redirection_kind: u8,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    organization_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
}

struct ParsedPostCreated {
    post_id: AccountAddress,
    owner: AccountAddress,
    profile_id: AccountAddress,
    platform_id: AccountAddress,
    permissions: u8,
    content: String,
    post_type: String,
    parent_post_id: Option<AccountAddress>,
    mentions: Option<Vec<AccountAddress>>,
    media_urls: Option<Vec<String>>,
    metadata_json: Option<String>,
    post_access_kind: String,
    mydata_id: Option<AccountAddress>,
    subscription_service_id: Option<AccountAddress>,
    requires_subscription: bool,
    promotion_id: Option<AccountAddress>,
    revenue_redirect_to: Option<AccountAddress>,
    revenue_redirect_percentage: Option<u64>,
    enable_spt: bool,
    spt_id: Option<AccountAddress>,
    poc_redirection_kind: u8,
    actor_address: AccountAddress,
    sub_agent_id: Option<String>,
    organization_id: Option<String>,
    action_identity_class: u8,
}

impl From<BcsPostCreatedEvent> for ParsedPostCreated {
    fn from(ev: BcsPostCreatedEvent) -> Self {
        let access_fields = access::post_access_fields_from_bcs(&ev.access);
        Self {
            post_id: ev.post_id,
            owner: ev.owner,
            profile_id: ev.profile_id,
            platform_id: ev.platform_id,
            permissions: ev.permissions,
            content: ev.content,
            post_type: ev.post_type,
            parent_post_id: ev.parent_post_id,
            mentions: ev.mentions,
            media_urls: ev.media_urls,
            metadata_json: ev.metadata_json,
            post_access_kind: access_fields.post_access_kind,
            mydata_id: access_fields
                .mydata_id
                .as_ref()
                .and_then(|s| AccountAddress::from_hex_literal(s).ok()),
            subscription_service_id: access_fields
                .subscription_service_id
                .as_ref()
                .and_then(|s| AccountAddress::from_hex_literal(s).ok()),
            requires_subscription: access_fields.requires_subscription.unwrap_or(false),
            promotion_id: ev.promotion_id,
            revenue_redirect_to: ev.revenue_redirect_to,
            revenue_redirect_percentage: ev.revenue_redirect_percentage,
            enable_spt: ev.enable_spt,
            spt_id: ev.spt_id,
            poc_redirection_kind: ev.poc_redirection_kind,
            actor_address: ev.actor_address,
            sub_agent_id: optional_move_object_id_json(&ev.sub_agent_id),
            organization_id: optional_move_object_id_json(&ev.organization_id),
            action_identity_class: ev.action_identity_class,
        }
    }
}

fn bcs_post_created_from_bytes(contents: &[u8]) -> Result<ParsedPostCreated, EventParseError> {
    match bcs::from_bytes::<BcsPostCreatedEvent>(contents) {
        Ok(ev) => Ok(ParsedPostCreated::from(ev)),
        Err(e) => Err(EventParseError {
            error: format!("PostCreatedEvent BCS: {}", e),
            contents: contents.to_vec(),
        }),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsCommentCreatedEvent {
    comment_id: AccountAddress,
    post_id: AccountAddress,
    parent_comment_id: Option<AccountAddress>,
    owner: AccountAddress,
    profile_id: AccountAddress,
    content: String,
    mentions: Option<Vec<AccountAddress>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsCommentCreatedEventWithOrganization {
    comment_id: AccountAddress,
    post_id: AccountAddress,
    parent_comment_id: Option<AccountAddress>,
    owner: AccountAddress,
    profile_id: AccountAddress,
    content: String,
    mentions: Option<Vec<AccountAddress>>,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    organization_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsCommentCreatedEventWithAttribution {
    comment_id: AccountAddress,
    post_id: AccountAddress,
    parent_comment_id: Option<AccountAddress>,
    owner: AccountAddress,
    profile_id: AccountAddress,
    content: String,
    mentions: Option<Vec<AccountAddress>>,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
}

struct ParsedCommentCreated {
    comment_id: AccountAddress,
    post_id: AccountAddress,
    parent_comment_id: Option<AccountAddress>,
    owner: AccountAddress,
    profile_id: AccountAddress,
    content: String,
    mentions: Option<Vec<AccountAddress>>,
    actor_address: AccountAddress,
    sub_agent_id: Option<String>,
    organization_id: Option<String>,
    action_identity_class: u8,
}

fn bcs_comment_created_from_bytes(
    contents: &[u8],
) -> Result<ParsedCommentCreated, EventParseError> {
    if let Ok(ev) = bcs::from_bytes::<BcsCommentCreatedEventWithOrganization>(contents) {
        return Ok(ParsedCommentCreated {
            comment_id: ev.comment_id,
            post_id: ev.post_id,
            parent_comment_id: ev.parent_comment_id,
            owner: ev.owner,
            profile_id: ev.profile_id,
            content: ev.content,
            mentions: ev.mentions,
            actor_address: ev.actor_address,
            sub_agent_id: optional_move_object_id_json(&ev.sub_agent_id),
            organization_id: optional_move_object_id_json(&ev.organization_id),
            action_identity_class: ev.action_identity_class,
        });
    }
    if let Ok(ev) = bcs::from_bytes::<BcsCommentCreatedEventWithAttribution>(contents) {
        return Ok(ParsedCommentCreated {
            comment_id: ev.comment_id,
            post_id: ev.post_id,
            parent_comment_id: ev.parent_comment_id,
            owner: ev.owner,
            profile_id: ev.profile_id,
            content: ev.content,
            mentions: ev.mentions,
            actor_address: ev.actor_address,
            sub_agent_id: optional_move_object_id_json(&ev.sub_agent_id),
            organization_id: None,
            action_identity_class: ev.action_identity_class,
        });
    }
    match bcs::from_bytes::<BcsCommentCreatedEvent>(contents) {
        Ok(ev) => Ok(ParsedCommentCreated {
            comment_id: ev.comment_id,
            post_id: ev.post_id,
            parent_comment_id: ev.parent_comment_id,
            owner: ev.owner,
            profile_id: ev.profile_id,
            content: ev.content,
            mentions: ev.mentions,
            actor_address: ev.owner,
            sub_agent_id: None,
            organization_id: None,
            action_identity_class: 0,
        }),
        Err(e) => Err(EventParseError {
            error: format!("CommentCreatedEvent BCS: {}", e),
            contents: contents.to_vec(),
        }),
    }
}

/// On-chain Move sets `user` to `actor_address`; prefer `actor_address` for sub-agent attribution.
fn canonical_reaction_user(user: AccountAddress, actor_address: AccountAddress) -> AccountAddress {
    let _legacy_user = user;
    actor_address
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsReactionEventWithOrganization {
    object_id: AccountAddress,
    _user: AccountAddress,
    reaction: String,
    is_post: bool,
    principal_owner: AccountAddress,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    organization_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsReactionEvent {
    object_id: AccountAddress,
    _user: AccountAddress,
    reaction: String,
    is_post: bool,
    principal_owner: AccountAddress,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
}

struct ParsedReactionEvent {
    object_id: AccountAddress,
    reaction: String,
    is_post: bool,
    principal_owner: AccountAddress,
    actor_address: AccountAddress,
    sub_agent_id: Option<String>,
    organization_id: Option<String>,
    action_identity_class: u8,
}

fn parse_reaction_event(ev: BcsReactionEvent) -> ParsedReactionEvent {
    let actor_address = canonical_reaction_user(ev._user, ev.actor_address);
    ParsedReactionEvent {
        object_id: ev.object_id,
        reaction: ev.reaction,
        is_post: ev.is_post,
        principal_owner: ev.principal_owner,
        actor_address,
        sub_agent_id: optional_move_object_id_json(&ev.sub_agent_id),
        organization_id: None,
        action_identity_class: ev.action_identity_class,
    }
}

fn parse_reaction_event_with_organization(
    ev: BcsReactionEventWithOrganization,
) -> ParsedReactionEvent {
    let actor_address = canonical_reaction_user(ev._user, ev.actor_address);
    ParsedReactionEvent {
        object_id: ev.object_id,
        reaction: ev.reaction,
        is_post: ev.is_post,
        principal_owner: ev.principal_owner,
        actor_address,
        sub_agent_id: optional_move_object_id_json(&ev.sub_agent_id),
        organization_id: optional_move_object_id_json(&ev.organization_id),
        action_identity_class: ev.action_identity_class,
    }
}

fn bcs_reaction_from_bytes(contents: &[u8]) -> Result<ParsedReactionEvent, EventParseError> {
    if let Ok(ev) = bcs::from_bytes::<BcsReactionEventWithOrganization>(contents) {
        return Ok(parse_reaction_event_with_organization(ev));
    }
    match bcs::from_bytes::<BcsReactionEvent>(contents) {
        Ok(ev) => Ok(parse_reaction_event(ev)),
        Err(e) => Err(EventParseError {
            error: format!("ReactionEvent BCS: {}", e),
            contents: contents.to_vec(),
        }),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsRemoveReactionEventWithOrganization {
    object_id: AccountAddress,
    user: AccountAddress,
    reaction: String,
    is_post: bool,
    principal_owner: AccountAddress,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    organization_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsRemoveReactionEvent {
    object_id: AccountAddress,
    user: AccountAddress,
    reaction: String,
    is_post: bool,
    principal_owner: AccountAddress,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
}

fn bcs_remove_reaction_from_bytes(contents: &[u8]) -> Result<ParsedReactionEvent, EventParseError> {
    if let Ok(ev) = bcs::from_bytes::<BcsRemoveReactionEventWithOrganization>(contents) {
        let actor_address = canonical_reaction_user(ev.user, ev.actor_address);
        return Ok(ParsedReactionEvent {
            object_id: ev.object_id,
            reaction: ev.reaction,
            is_post: ev.is_post,
            principal_owner: ev.principal_owner,
            actor_address,
            sub_agent_id: optional_move_object_id_json(&ev.sub_agent_id),
            organization_id: optional_move_object_id_json(&ev.organization_id),
            action_identity_class: ev.action_identity_class,
        });
    }
    match bcs::from_bytes::<BcsRemoveReactionEvent>(contents) {
        Ok(ev) => {
            let actor_address = canonical_reaction_user(ev.user, ev.actor_address);
            Ok(ParsedReactionEvent {
                object_id: ev.object_id,
                reaction: ev.reaction,
                is_post: ev.is_post,
                principal_owner: ev.principal_owner,
                actor_address,
                sub_agent_id: optional_move_object_id_json(&ev.sub_agent_id),
                organization_id: None,
                action_identity_class: ev.action_identity_class,
            })
        }
        Err(e) => Err(EventParseError {
            error: format!("RemoveReactionEvent BCS: {}", e),
            contents: contents.to_vec(),
        }),
    }
}

#[derive(Debug, Deserialize)]
pub struct BcsRepostEvent {
    repost_id: AccountAddress,
    original_id: AccountAddress,
    is_original_post: bool,
    owner: AccountAddress,
    profile_id: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsRepostEventWithOrganization {
    repost_id: AccountAddress,
    original_id: AccountAddress,
    is_original_post: bool,
    owner: AccountAddress,
    profile_id: AccountAddress,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    organization_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
}

#[derive(Debug, Deserialize)]
pub struct BcsRepostEventWithAttribution {
    repost_id: AccountAddress,
    original_id: AccountAddress,
    is_original_post: bool,
    owner: AccountAddress,
    profile_id: AccountAddress,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsRepostRemovedEvent {
    repost_id: AccountAddress,
    original_id: AccountAddress,
    owner: AccountAddress,
    actor_address: AccountAddress,
    sub_agent_id: Option<BcsMoveObjectId>,
    organization_id: Option<BcsMoveObjectId>,
    action_identity_class: u8,
    removed_at: u64,
}

struct ParsedRepostEvent {
    repost_id: AccountAddress,
    original_id: AccountAddress,
    is_original_post: bool,
    owner: AccountAddress,
    profile_id: AccountAddress,
    actor_address: AccountAddress,
    sub_agent_id: Option<String>,
    organization_id: Option<String>,
    action_identity_class: u8,
}

fn bcs_repost_from_bytes(contents: &[u8]) -> Result<ParsedRepostEvent, EventParseError> {
    if let Ok(ev) = bcs::from_bytes::<BcsRepostEventWithOrganization>(contents) {
        return Ok(ParsedRepostEvent {
            repost_id: ev.repost_id,
            original_id: ev.original_id,
            is_original_post: ev.is_original_post,
            owner: ev.owner,
            profile_id: ev.profile_id,
            actor_address: ev.actor_address,
            sub_agent_id: optional_move_object_id_json(&ev.sub_agent_id),
            organization_id: optional_move_object_id_json(&ev.organization_id),
            action_identity_class: ev.action_identity_class,
        });
    }
    if let Ok(ev) = bcs::from_bytes::<BcsRepostEventWithAttribution>(contents) {
        return Ok(ParsedRepostEvent {
            repost_id: ev.repost_id,
            original_id: ev.original_id,
            is_original_post: ev.is_original_post,
            owner: ev.owner,
            profile_id: ev.profile_id,
            actor_address: ev.actor_address,
            sub_agent_id: optional_move_object_id_json(&ev.sub_agent_id),
            organization_id: None,
            action_identity_class: ev.action_identity_class,
        });
    }
    match bcs::from_bytes::<BcsRepostEvent>(contents) {
        Ok(ev) => Ok(ParsedRepostEvent {
            repost_id: ev.repost_id,
            original_id: ev.original_id,
            is_original_post: ev.is_original_post,
            owner: ev.owner,
            profile_id: ev.profile_id,
            actor_address: ev.owner,
            sub_agent_id: None,
            organization_id: None,
            action_identity_class: 0,
        }),
        Err(e) => Err(EventParseError {
            error: format!("RepostEvent BCS: {}", e),
            contents: contents.to_vec(),
        }),
    }
}

fn optional_addr_json(addr: &Option<AccountAddress>) -> Option<String> {
    addr.as_ref().map(addr_to_string)
}

#[derive(Debug, Deserialize)]
pub struct BcsMemoryAccountCreatedEvent {
    account_id: AccountAddress,
    owner: AccountAddress,
    profile_id: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsMemoryConfigUpdatedEvent {
    updated_by: AccountAddress,
    max_organizations_per_user: u8,
    org_category_update_cooldown_ms: u64,
    max_agent_depth: u8,
    max_label_length: u64,
    max_org_name_length: u64,
    max_org_description_length: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSubAgentRegisteredEvent {
    account_id: AccountAddress,
    principal_owner: AccountAddress,
    profile_id: AccountAddress,
    organization_id: AccountAddress,
    agent_object_id: AccountAddress,
    derived_address: AccountAddress,
    label: String,
    identity_class: u8,
    role_tags: u64,
    capabilities: u64,
    delegatable_caps: u64,
    register_scope: u8,
    approval_required_caps: u64,
    max_action_spend: Option<u64>,
    platform_scope: Option<AccountAddress>,
    parent_object_id: Option<AccountAddress>,
    depth: u8,
    registered_by: AccountAddress,
    expires_at: Option<u64>,
    active: bool,
    created_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSubAgentUpdatedEvent {
    account_id: AccountAddress,
    principal_owner: AccountAddress,
    profile_id: AccountAddress,
    organization_id: AccountAddress,
    agent_object_id: AccountAddress,
    derived_address: AccountAddress,
    label: String,
    identity_class: u8,
    role_tags: u64,
    capabilities: u64,
    delegatable_caps: u64,
    register_scope: u8,
    approval_required_caps: u64,
    max_action_spend: Option<u64>,
    platform_scope: Option<AccountAddress>,
    parent_object_id: Option<AccountAddress>,
    depth: u8,
    registered_by: AccountAddress,
    expires_at: Option<u64>,
    active: bool,
    created_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSubAgentDeactivatedEvent {
    account_id: AccountAddress,
    principal_owner: AccountAddress,
    profile_id: AccountAddress,
    agent_object_id: AccountAddress,
    derived_address: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsSubAgentRevokedEvent {
    account_id: AccountAddress,
    principal_owner: AccountAddress,
    profile_id: AccountAddress,
    agent_object_id: AccountAddress,
    derived_address: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsSubAgentsClearedOnTransferEvent {
    account_id: AccountAddress,
    principal_owner: AccountAddress,
    profile_id: AccountAddress,
    previous_owner: AccountAddress,
    new_owner: AccountAddress,
    revoked_count: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMemoryAccountDeactivatedEvent {
    account_id: AccountAddress,
    owner: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMemoryAccountReactivatedEvent {
    account_id: AccountAddress,
    owner: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMemoryAccountMigratedEvent {
    account_id: AccountAddress,
    from: u64,
    to: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMemoryRegistryMigratedEvent {
    registry_id: AccountAddress,
    from: u64,
    to: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsAgentMemoryVaultCreatedEvent {
    vault_id: AccountAddress,
    agent_object_id: AccountAddress,
    memory_account_id: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsAgenticOrganizationCreatedEvent {
    organization_id: AccountAddress,
    account_id: AccountAddress,
    principal_owner: AccountAddress,
    profile_id: AccountAddress,
    name: Option<String>,
    description: Option<String>,
    org_type: u8,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsAgenticOrganizationUpdatedEvent {
    organization_id: AccountAddress,
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsAgenticOrganizationCategoryUpdatedEvent {
    organization_id: AccountAddress,
    org_type: u8,
    previous_org_type: u8,
    updated_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsAgenticOrganizationDeactivatedEvent {
    organization_id: AccountAddress,
    deactivated_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsOrgMemoryGroupCreatedEvent {
    group_id: AccountAddress,
    organization_id: AccountAddress,
    account_id: AccountAddress,
    principal_owner: AccountAddress,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsOrgMemoryPermissionGrantedEvent {
    organization_id: AccountAddress,
    account_id: AccountAddress,
    group_id: AccountAddress,
    member: AccountAddress,
    permissions_mask: u64,
    granted_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsOrgMemoryPermissionRevokedEvent {
    organization_id: AccountAddress,
    account_id: AccountAddress,
    group_id: AccountAddress,
    member: AccountAddress,
    permissions_mask: u64,
    revoked_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsOrgRoleDefinedEvent {
    organization_id: AccountAddress,
    account_id: AccountAddress,
    role_name: String,
    mask: u64,
    previous_mask: Option<u64>,
    defined_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsOrgRoleAssignedEvent {
    organization_id: AccountAddress,
    account_id: AccountAddress,
    group_id: AccountAddress,
    member: AccountAddress,
    role_name: String,
    mask: u64,
    granted_mask: u64,
    assigned_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsOrgRoleRevokedEvent {
    organization_id: AccountAddress,
    account_id: AccountAddress,
    group_id: AccountAddress,
    member: AccountAddress,
    role_name: String,
    revoked_mask: u64,
    revoked_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsOrgInvitationCreatedEvent {
    organization_id: AccountAddress,
    account_id: AccountAddress,
    invitee: AccountAddress,
    role_name: Option<String>,
    permissions_mask: u64,
    invited_by: AccountAddress,
    timestamp_ms: u64,
    expires_at_ms: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsOrgInvitationAcceptedEvent {
    organization_id: AccountAddress,
    account_id: AccountAddress,
    group_id: AccountAddress,
    invitee: AccountAddress,
    role_name: Option<String>,
    permissions_mask: u64,
    granted_mask: u64,
    accepted_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsOrgInvitationDeclinedEvent {
    organization_id: AccountAddress,
    account_id: AccountAddress,
    invitee: AccountAddress,
    declined_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditBalanceCreatedEvent {
    balance_id: AccountAddress,
    memory_account_id: AccountAddress,
    principal_owner: AccountAddress,
    profile_id: AccountAddress,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditDepositedEvent {
    balance_id: AccountAddress,
    amount_mist: u64,
    new_balance_mist: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditWithdrawnEvent {
    balance_id: AccountAddress,
    amount_mist: u64,
    new_balance_mist: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditAccountCapsUpdatedEvent {
    balance_id: AccountAddress,
    daily_cap_mist: Option<u64>,
    monthly_cap_mist: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditAgentBudgetUpdatedEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    budget_mist: Option<u64>,
    daily_cap_mist: Option<u64>,
    monthly_cap_mist: Option<u64>,
    require_approval_above_mist: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditAgentBudgetDisabledEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditAgentBudgetChangedEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    had_previous_entry: bool,
    prev_budget_mist: Option<u64>,
    prev_daily_cap_mist: Option<u64>,
    prev_monthly_cap_mist: Option<u64>,
    prev_require_approval_above_mist: Option<u64>,
    prev_enabled: bool,
    budget_mist: Option<u64>,
    daily_cap_mist: Option<u64>,
    monthly_cap_mist: Option<u64>,
    require_approval_above_mist: Option<u64>,
    enabled: bool,
    set_by: AccountAddress,
    set_by_agent_id: Option<AccountAddress>,
    organization_id: Option<AccountAddress>,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditSpendApprovedEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    approval_nonce: u64,
    max_amount_mist: u64,
    expires_at_ms: u64,
    approved_by: AccountAddress,
    approved_by_agent_id: Option<AccountAddress>,
    organization_id: Option<AccountAddress>,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditSpendApprovalRevokedEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    approval_nonce: u64,
    revoked_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditSpendApprovalConsumedEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    approval_nonce: u64,
    amount_mist: u64,
    approved_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsAiCreditUsageSettledEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    receipt_id: u128,
    amount_mist: u64,
    usage_kind: u8,
    settlement_nonce: u64,
    remaining_mist: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsAiSpendReservedEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    reservation_nonce: u64,
    max_amount_mist: u64,
    provider_envelope_hash: Vec<u8>,
    request_hash: Vec<u8>,
    fx_quote_id: Vec<u8>,
    myso_usd_e8: u64,
    markup_bps: u64,
    capture_deadline_ms: u64,
    hard_expiry_ms: u64,
    available_mist: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsAiSpendCapturedEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    reservation_nonce: u64,
    reserved_mist: u64,
    captured_mist: u64,
    released_mist: u64,
    provider_cost_usd_micros: u64,
    provider_generation_hash: Vec<u8>,
    fx_quote_id: Vec<u8>,
    myso_usd_e8: u64,
    markup_bps: u64,
    captured_at_ms: u64,
    remaining_mist: u64,
    available_mist: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsAiSpendCancelledEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    reservation_nonce: u64,
    released_mist: u64,
    cancelled_at_ms: u64,
    available_mist: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsAiSpendExpiredEvent {
    balance_id: AccountAddress,
    agent_object_id: AccountAddress,
    reservation_nonce: u64,
    released_mist: u64,
    expired_at_ms: u64,
    available_mist: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditBalanceIdEvent {
    balance_id: AccountAddress,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditConfigInitializedEvent {
    oracle_pubkey: Vec<u8>,
    treasury: AccountAddress,
    min_deposit_mist: u64,
    max_single_settlement_mist: u64,
    receipt_ttl_ms: u64,
    oracle_markup_bps: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsAiCreditOraclePubkeyUpdatedEvent {
    updated_by: AccountAddress,
    new_pubkey: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsAiCreditMarkupUpdatedEvent {
    updated_by: AccountAddress,
    oracle_markup_bps: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditMinDepositUpdatedEvent {
    updated_by: AccountAddress,
    min_deposit_mist: u64,
}

#[derive(Debug, Deserialize)]
struct BcsAiCreditSettlementLimitsUpdatedEvent {
    max_single_settlement_mist: u64,
    receipt_ttl_ms: u64,
}

/// Move `ascii::String` BCS (`bytes` field).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BcsMoveAsciiString {
    pub bytes: Vec<u8>,
}

/// Move `std::type_name::TypeName` (fully-qualified type string).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BcsMoveTypeName {
    pub name: BcsMoveAsciiString,
}

fn bcs_move_type_name_display(tn: &BcsMoveTypeName) -> String {
    String::from_utf8_lossy(&tn.name.bytes).into_owned()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsTipEvent {
    object_id: AccountAddress,
    from: AccountAddress,
    to: AccountAddress,
    amount: u64,
    pub coin_type: BcsMoveTypeName,
    is_post: bool,
}

#[derive(Debug, Deserialize)]
pub struct BcsPostParametersUpdatedEvent {
    updated_by: AccountAddress,
    timestamp: u64,
    max_content_length: u64,
    max_media_urls: u64,
    max_mentions: u64,
    max_metadata_size: u64,
    max_description_length: u64,
    max_reaction_length: u64,
    commenter_tip_percentage: u64,
    repost_tip_percentage: u64,
    min_promotion_amount: u64,
    max_promotion_amount: u64,
    min_view_duration_ms: u64,
    platform_fee_bps: u64,
    ecosystem_fee_bps: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPostModerationEvent {
    post_id: AccountAddress,
    platform_id: AccountAddress,
    removed: bool,
    moderated_by: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPostDeletedEvent {
    post_id: AccountAddress,
    owner: AccountAddress,
    profile_id: AccountAddress,
    post_type: String,
    deleted_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsCommentDeletedEvent {
    comment_id: AccountAddress,
    post_id: AccountAddress,
    owner: AccountAddress,
    profile_id: AccountAddress,
    deleted_at: u64,
}

// Promotion event structs - field order matches post.move
#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPromotedPostCreatedEvent {
    post_id: AccountAddress,
    owner: AccountAddress,
    profile_id: AccountAddress,
    payment_per_view: u64,
    total_budget: u64,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPromotedViewConfirmItem {
    post_id: AccountAddress,
    promotion_id: AccountAddress,
    payment_amount: u64,
    platform_fee: u64,
    ecosystem_fee: u64,
    recipient_amount: u64,
    view_duration: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPromotedPostViewsBatchConfirmedEvent {
    viewer: AccountAddress,
    platform_id: AccountAddress,
    timestamp: u64,
    items: Vec<BcsPromotedViewConfirmItem>,
    total_payment_amount: u64,
    total_platform_fee: u64,
    total_ecosystem_fee: u64,
    total_recipient_amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPromotionStatusToggledEvent {
    post_id: AccountAddress, // promotion_id
    toggled_by: AccountAddress,
    new_status: bool,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPromotionFundsWithdrawnEvent {
    post_id: AccountAddress, // promotion_id
    owner: AccountAddress,
    withdrawn_amount: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPostSubscriptionAccessEvent {
    post_id: AccountAddress,
    service_id: AccountAddress,
    subscription_id: AccountAddress,
    subscriber: AccountAddress,
    timestamp: u64,
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
    beneficiary_address: Option<AccountAddress>,
    matched_anchor_id: Option<AccountAddress>,
    media_index: u8,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPocDisputeSubmittedEvent {
    dispute_id: AccountAddress,
    post_id: AccountAddress,
    disputer: AccountAddress,
    dispute_type: u8,
    stake_amount: u64,
    dispute_round: u8,
    effective_fee: u64,
    required_total_stake_quorum: u64,
    post_poc_disputes_submitted_after: u8,
    voting_start_ms: u64,
    voting_end_ms: u64,
    evidence: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPocDisputeResolvedEvent {
    dispute_id: AccountAddress,
    post_id: AccountAddress,
    resolution: u8,
    winning_side: u8,
    total_winning_stake: u64,
    total_losing_stake: u64,
    badge_revoked: bool,
    redirection_removed: bool,
    quorum_met: bool,
    post_poc_disputes_submitted: u8,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPocConfigUpdatedEvent {
    updated_by: AccountAddress,
    oracle_address: AccountAddress,
    image_threshold: u64,
    video_threshold: u64,
    audio_threshold: u64,
    revenue_redirect_percentage: u64,
    dispute_cost: u64,
    min_vote_stake: u64,
    max_vote_stake: u64,
    voting_duration_ms: u64,
    max_reasoning_length: u64,
    max_evidence_urls: u64,
    max_votes_per_dispute: u64,
    dispute_governance_registry_id: AccountAddress,
    claim_treasury_fee_bps: u64,
    max_referral_bps: u64,
    video_embedded_audio_redirect_bps: u64,
    dispute_quorum_base_stake: u64,
    dispute_second_round_fee_multiplier_bps: u64,
    dispute_second_round_quorum_multiplier_bps: u64,
    username_beneficiary_join_referral_bps: u64,
    max_disputes_per_post: u8,
    min_vault_deposit_amount: u64,
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
    insurance_enabled: bool,
    min_coverage_bps: u64,
    max_coverage_bps: u64,
    max_duration_ms: u64,
    fee_bps: u64,
    odds_base_bps: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsRouterConfigUpdatedEvent {
    updated_by: AccountAddress,
    paused: bool,
    max_route_reserve_market: u64,
    max_route_reserve_user: u64,
    max_route_reserve_option: u64,
    max_vault_concentration_bps: u64,
    min_vault_health_factor_bps: u64,
    max_route_legs: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsRiskPricingConfigUpdatedEvent {
    updated_by: AccountAddress,
    min_spot_total_liquidity: u64,
    max_coverage_fraction_of_option_bps: u64,
    max_risk_multiplier_bps: u64,
    min_premium_amount: u64,
    spot_smoothing_per_option: u64,
    implied_prob_floor_bps: u64,
    odds_floor_1x: bool,
    odds_cap_bps: u64,
    liq_cap_bps: u64,
    liq_ref_amount: u64,
    exposure_cap_bps: u64,
    exposure_k_bps: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUnderwriterVaultCreatedEvent {
    vault_id: AccountAddress,
    underwriter: AccountAddress,
    base_rate_bps_per_day: u64,
    utilization_multiplier_bps: u64,
    max_exposure_per_market: u64,
    max_exposure_per_user: u64,
    max_exposure_per_option: u64,
    enabled: bool,
    paused: bool,
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
    vault_id: AccountAddress,
    market_id: AccountAddress,
    insured: AccountAddress,
    option_id: u8,
    covered_amount: u64,
    coverage_bps: u64,
    premium_paid: u64,
    premium_raw: u64,
    reserve_locked: u64,
    expiry_time_ms: u64,
    implied_probability_bps: u64,
    risk_multiplier_bps: u64,
    base_premium: u64,
    market_total_amount: u64,
    option_amount: u64,
    backstop_sweep_amount: u64,
    route_id: Option<AccountAddress>,
    route_leg_index: u8,
}

#[derive(Debug, Deserialize)]
pub struct BcsVaultStatusUpdatedEvent {
    vault_id: AccountAddress,
    enabled: bool,
    paused: bool,
    max_exposure_per_option: u64,
    max_exposure_per_market: u64,
    max_exposure_per_user: u64,
    base_rate_bps_per_day: u64,
    utilization_multiplier_bps: u64,
    updated_by: AccountAddress,
    timestamp_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsCoverageRoutedEvent {
    route_id: AccountAddress,
    insured: AccountAddress,
    market_id: AccountAddress,
    option_id: u8,
    coverage_bps: u64,
    duration_ms: u64,
    total_covered: u64,
    total_premium: u64,
    total_reserve: u64,
    total_backstop_sweep: u64,
    expiry_time_ms: u64,
    policy_ids: Vec<AccountAddress>,
    vault_ids: Vec<AccountAddress>,
}

#[derive(Debug, Deserialize)]
pub struct BcsRouteFillEvent {
    route_id: AccountAddress,
    leg_index: u8,
    vault_id: AccountAddress,
    policy_id: AccountAddress,
    covered_amount: u64,
    premium_paid: u64,
    reserve_locked: u64,
    backstop_sweep_amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsBackstopUsedEvent {
    market_id: AccountAddress,
    recipient: AccountAddress,
    amount: u64,
    total_paid_out_after: u64,
    tail_mode_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct BcsBackstopTreasuryDepositEvent {
    depositor: AccountAddress,
    amount: u64,
    new_balance: u64,
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
    access_configuration_kind: u8,
    created_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPurchaseEvent {
    ip_id: AccountAddress,
    buyer: AccountAddress,
    price: u64,
    purchase_type: String,
    timestamp: u64,
    sub_agent_id: Option<AccountAddress>,
    organization_id: Option<AccountAddress>,
}

#[derive(Debug, Deserialize)]
pub struct BcsPurchaseEventV2 {
    ip_id: AccountAddress,
    buyer: AccountAddress,
    price: u64,
    purchase_type: String,
    timestamp: u64,
    sub_agent_id: Option<AccountAddress>,
    organization_id: Option<AccountAddress>,
    platform_fee: u64,
    ecosystem_fee: u64,
    creator_amount: u64,
    platform_id: Option<AccountAddress>,
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
pub struct BcsAccessRevokedEvent {
    ip_id: AccountAddress,
    user: AccountAddress,
    access_type: String,
    revoked_by: AccountAddress,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMyDataConfigUpdatedEvent {
    updated_by: AccountAddress,
    marketplace_enabled: bool,
    max_tags: u64,
    max_subscription_days: u64,
    max_free_access_grants: u64,
    max_encryption_id_bytes: u64,
    p2p_platform_fee_bps: u64,
    p2p_ecosystem_fee_bps: u64,
    mydata_marketplace_platform_fee_bps: u64,
    mydata_marketplace_ecosystem_fee_bps: u64,
    non_platform_platform_to_creator_bps: u64,
    non_platform_platform_to_treasury_bps: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMyDataConfigUpdatedEventV2 {
    updated_by: AccountAddress,
    marketplace_enabled: bool,
    max_tags: u64,
    max_subscription_days: u64,
    max_free_access_grants: u64,
    max_encryption_id_bytes: u64,
    max_encrypted_data_bytes: u64,
    max_tag_bytes: u64,
    max_metadata_bytes: u64,
    max_payment_reference_bytes: u64,
    max_pool_assignments: u64,
    max_merkle_proof_depth: u64,
    max_paid_access_entries: u64,
    default_claim_window_ms: u64,
    p2p_platform_fee_bps: u64,
    p2p_ecosystem_fee_bps: u64,
    mydata_marketplace_platform_fee_bps: u64,
    mydata_marketplace_ecosystem_fee_bps: u64,
    non_platform_platform_to_creator_bps: u64,
    non_platform_platform_to_treasury_bps: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsBroadPoolCreatedEvent {
    pool_id: AccountAddress,
    name: String,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsBroadPoolCreatedEventV2 {
    pool_id: AccountAddress,
    name: String,
    platform_id: Option<AccountAddress>,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSubPoolCreatedEvent {
    sub_pool_id: AccountAddress,
    broad_pool_id: AccountAddress,
    name: String,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMyDataAssignedToSubPoolEvent {
    ip_id: AccountAddress,
    sub_pool_ids: Vec<AccountAddress>,
    assigned_at: u64,
}

/// Legacy `SnapshotAnchorRecordedEvent` BCS (four fields) from packages before manifest/reference were emitted.
#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSnapshotAnchorRecordedEvent {
    snapshot_id: AccountAddress,
    buyer_address: AccountAddress,
    price_paid: u64,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSnapshotAnchorRecordedEventV2 {
    snapshot_id: AccountAddress,
    buyer_address: AccountAddress,
    price_paid: u64,
    created_at: u64,
    snapshot_manifest_hash: Vec<u8>,
    payment_reference: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSnapshotAnchorRecordedEventV3 {
    snapshot_id: AccountAddress,
    buyer_address: AccountAddress,
    price_paid: u64,
    source_pool_id: AccountAddress,
    source_sub_pool_id: AccountAddress,
    platform_id: Option<AccountAddress>,
    created_at: u64,
    snapshot_manifest_hash: Vec<u8>,
    payment_reference: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMerkleRootPublishedEvent {
    snapshot_id: AccountAddress,
    root_hash: Vec<u8>,
    published_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsClaimExecutedEvent {
    snapshot_id: AccountAddress,
    claimant: AccountAddress,
    amount: u64,
    claimed_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsClaimExecutedEventV2 {
    snapshot_id: AccountAddress,
    claimant: AccountAddress,
    gross_amount: u64,
    platform_fee: u64,
    ecosystem_fee: u64,
    net_amount: u64,
    platform_id: Option<AccountAddress>,
    claimed_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsDistributionRecordedEvent {
    snapshot_id: AccountAddress,
    total_amount: u64,
    contributor_count: u64,
    merkle_root: Vec<u8>,
    published_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsDistributionRecordedEventV2 {
    snapshot_id: AccountAddress,
    total_amount: u64,
    contributor_count: u64,
    merkle_root: Vec<u8>,
    platform_id: Option<AccountAddress>,
    claim_deadline_ms: u64,
    published_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSnapshotEscrowFundedEvent {
    snapshot_id: AccountAddress,
    funder: AccountAddress,
    amount: u64,
    total_funded: u64,
    funded_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSnapshotEscrowReclaimedEvent {
    snapshot_id: AccountAddress,
    buyer_address: AccountAddress,
    amount: u64,
    reclaimed_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMyDataPricingUpdatedEvent {
    ip_id: AccountAddress,
    one_time_price: Option<u64>,
    subscription_price: Option<u64>,
    subscription_duration_days: Option<u64>,
    updated_by: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsMyDataContentUpdatedEvent {
    ip_id: AccountAddress,
    encrypted_data_updated: bool,
    tags_updated: bool,
    updated_by: AccountAddress,
    timestamp: u64,
}

// Social Proof of Truth (SPoT) event structs - field order matches social_proof_of_truth.move
#[derive(Debug, Deserialize)]
pub struct BcsSpotBetPlacedEvent {
    post_id: AccountAddress,
    market_id: AccountAddress,
    user: AccountAddress,
    option_id: u8,
    amount: u64,
    timestamp_ms: u64,
    referrer_post_id: Option<AccountAddress>,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotResolvedEvent {
    post_id: AccountAddress,
    market_id: AccountAddress,
    claim_id: AccountAddress,
    outcome: u8,
    total_escrow: u64,
    fee_taken: u64,
    creator_fee_total: u64,
    reasoning: String,
    evidence_urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotDaoRequiredEvent {
    post_id: AccountAddress,
    spot_record_id: AccountAddress,
    confidence_bps: u64,
    oracle_proposed_outcome: u8,
    dao_escalated_at_ms: u64,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSpotConfigUpdatedEvent {
    updated_by: AccountAddress,
    truth_enabled: bool,
    confidence_threshold_bps: u64,
    resolution_window_ms: u64,
    max_resolution_window_ms: u64,
    payout_delay_ms: u64,
    platform_fee_bps: u64,
    ecosystem_fee_bps: u64,
    creator_fee_bps: u64,
    creator_claim_window_ms: u64,
    expired_creator_ecosystem_bps: u64,
    min_betting_options: u64,
    max_betting_options: u64,
    min_reasoning_length: u64,
    max_reasoning_length: u64,
    max_evidence_urls: u64,
    oracle_address: AccountAddress,
    max_single_bet: u64,
    max_bets_per_record: u64,
    max_claim_per_post: u64,
    spot_governance_registry_id: AccountAddress,
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
    created_at_ms: u64,
    betting_options: Vec<String>,
    resolution_window_ms: Option<u64>,
    max_resolution_window_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotGovernanceProposalLinkedEvent {
    post_id: AccountAddress,
    spot_record_id: AccountAddress,
    proposal_id: AccountAddress,
    proposed_outcome: u8,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotGovernanceProposalClearedEvent {
    post_id: AccountAddress,
    spot_record_id: AccountAddress,
    proposal_id: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotClaimCreatedEvent {
    claim_id: AccountAddress,
    semantic_claim_hash: Vec<u8>,
    created_at_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotMarketCreatedEvent {
    market_id: AccountAddress,
    claim_id: AccountAddress,
    market_key_hash: Vec<u8>,
    primary_post_id: AccountAddress,
    claim_index: u64,
    resolution_policy_hash: Vec<u8>,
    created_at_ms: u64,
    betting_options: Vec<String>,
    resolution_at_ms: u64,
    max_resolution_window_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotPostLinkedEvent {
    post_id: AccountAddress,
    claim_id: AccountAddress,
    market_id: Option<AccountAddress>,
    claim_index: u64,
    policy_hash: Vec<u8>,
}

/// Batch finalize projection for a post's multi-claim analysis (future arrays + past verdicts).
#[derive(Debug, Deserialize)]
pub struct BcsSpotClaimsFinalizedForPost {
    pub post_id: AccountAddress,
    pub status: u8,
    pub detected_claim_count: u64,
    pub rejected_claim_count: u64,
    pub truncated_claim_count: u64,
    pub future_accepted_count: u64,
    pub past_verified_count: u64,
    pub max_claim_per_post_applied: u64,
    pub claim_manifest_hash: Option<Vec<u8>>,
    pub veracity_manifest_hash: Option<Vec<u8>>,
    pub future_claim_indexes: Vec<u64>,
    pub future_claim_ids: Vec<AccountAddress>,
    pub future_market_ids: Vec<AccountAddress>,
    pub past_claim_indexes: Vec<u64>,
    pub past_verdicts: Vec<u8>,
    pub past_related_market_ids: Vec<AccountAddress>,
    pub past_evidence_hashes: Vec<Vec<u8>>,
    pub finalized_at_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotCreatorPayoutAccruedEvent {
    market_id: AccountAddress,
    payout_id: u64,
    creator: AccountAddress,
    referrer_post_id: AccountAddress,
    amount: u64,
    expires_at_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotCreatorPayoutClaimedEvent {
    market_id: AccountAddress,
    payout_id: u64,
    creator: AccountAddress,
    amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsSpotCreatorPayoutReclaimedEvent {
    market_id: AccountAddress,
    payout_id: u64,
    ecosystem_amount: u64,
    platform_amount: u64,
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
    profile_id: AccountAddress,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSubscriptionPlanCreatedEvent {
    service_id: AccountAddress,
    plan_id: AccountAddress,
    title: String,
    description: Option<String>,
    price: u64,
    duration_ms: u64,
    tier_level: Option<u64>,
    platform_id: Option<AccountAddress>,
    created_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSubscriptionPlanUpdatedEvent {
    service_id: AccountAddress,
    plan_id: AccountAddress,
    title: String,
    description: Option<String>,
    price: u64,
    duration_ms: u64,
    tier_level: Option<u64>,
    platform_id: Option<AccountAddress>,
    active: bool,
    updated_by: AccountAddress,
    updated_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSubscriptionPlanDeactivatedEvent {
    service_id: AccountAddress,
    plan_id: AccountAddress,
    deactivated_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileSubscriptionCreatedEvent {
    subscription_id: AccountAddress,
    service_id: AccountAddress,
    plan_id: AccountAddress,
    subscriber: AccountAddress,
    expires_at: u64,
    price: u64,
    duration_ms: u64,
    tier_level: Option<u64>,
    platform_id: Option<AccountAddress>,
    auto_renew: bool,
    platform_fee: u64,
    ecosystem_fee: u64,
    creator_amount: u64,
    payment_platform_id: Option<AccountAddress>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileSubscriptionRenewedEvent {
    subscription_id: AccountAddress,
    subscriber: AccountAddress,
    plan_id: AccountAddress,
    new_expires_at: u64,
    renewal_count: u64,
    auto_renewed: bool,
    price: u64,
    duration_ms: u64,
    tier_level: Option<u64>,
    platform_id: Option<AccountAddress>,
    platform_fee: u64,
    ecosystem_fee: u64,
    creator_amount: u64,
    payment_platform_id: Option<AccountAddress>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsProfileSubscriptionCancelledEvent {
    subscription_id: AccountAddress,
    subscriber: AccountAddress,
    refunded_amount: u64,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsSubscriptionConfigUpdatedEvent {
    updated_by: AccountAddress,
    default_billing_period_ms: u64,
    max_renewal_months: u64,
    platform_fee_bps: u64,
    ecosystem_fee_bps: u64,
    non_platform_platform_to_creator_bps: u64,
    non_platform_platform_to_treasury_bps: u64,
    timestamp: u64,
}

// Social Proof Token (SPT) event structs - field order matches social_proof_tokens.move
#[derive(Debug, Deserialize, Serialize)]
pub struct BcsTokenPoolCreatedEvent {
    id: AccountAddress,
    token_type: u8,
    owner: AccountAddress,
    associated_id: AccountAddress,
    base_price: u64,
    quadratic_coefficient: u64,
    /// Initial nano-SPT circulating supply at pool creation (launch mint; on-chain `(total_reserved * SPT_SCALE) / base_price`).
    circulating_supply: u64,
    /// Net nano-MYSO reserved at launch (denominator for proportional indexer split).
    total_reserved_at_launch: u64,
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

/// Atomic summary of an SPT→SPT swap (emitted after `TokenSoldEvent` + `TokenBoughtEvent`).
/// Fields are in the exact Move struct order for BCS deserialization.
#[derive(Debug, Deserialize, Serialize)]
pub struct BcsTokenSwappedEvent {
    source_pool_id: AccountAddress,
    dest_pool_id: AccountAddress,
    trader: AccountAddress,
    sell_amount: u64,
    dest_amount: u64,
    sell_myso_gross: u64,
    buy_myso_gross: u64,
    sell_fee_amount: u64,
    buy_fee_amount: u64,
    sell_creator_fee: u64,
    sell_platform_fee: u64,
    sell_treasury_fee: u64,
    buy_creator_fee: u64,
    buy_platform_fee: u64,
    buy_treasury_fee: u64,
    leftover_myso: u64,
    source_new_price: u64,
    dest_new_price: u64,
}

/// P2P SPT transfer (`TokenTransferredEvent`). Move field order for BCS.
#[derive(Debug, Deserialize, Serialize)]
pub struct BcsTokenTransferredEvent {
    pool_id: AccountAddress,
    from: AccountAddress,
    to: AccountAddress,
    amount: u64,
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

#[derive(Debug, Deserialize, Serialize)]
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
    non_platform_platform_to_creator_bps: u64,
    non_platform_platform_to_treasury_bps: u64,
    trading_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct BcsTokensAddedEvent {
    owner: AccountAddress,
    pool_id: AccountAddress,
    amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPoCResultAppliedEvent {
    post_id: AccountAddress,
    poc_outcome: u8,
    poc_redirection_kind: u8,
    similarity_detected: bool,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPoCBeneficiaryVaultDepositEvent {
    vault_id: AccountAddress,
    beneficiary: AccountAddress,
    coin_type: BcsMoveTypeName,
    amount: u64,
    source_post_id: Option<AccountAddress>,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPoCBeneficiaryVaultClaimedEvent {
    vault_id: AccountAddress,
    beneficiary: AccountAddress,
    coin_type: BcsMoveTypeName,
    referrer: Option<AccountAddress>,
    treasury_amount: u64,
    referrer_amount: u64,
    beneficiary_amount: u64,
    join_referral_applied: bool,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUsernameBeneficiaryProvisionedEvent {
    beneficiary_id: AccountAddress,
    username: String,
    creator_identity_source: u8,
    creator_identity_hash: Vec<u8>,
    required_x_handle: String,
    beneficiary_address: AccountAddress,
    vault_id: AccountAddress,
    provisioned_by: AccountAddress,
    provisioned_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUsernameBeneficiaryClaimedEvent {
    beneficiary_id: AccountAddress,
    username: String,
    profile_id: AccountAddress,
    claimed_by: AccountAddress,
    wallet: AccountAddress,
    oracle_evidence_hash: Vec<u8>,
    claimed_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUsernameBeneficiaryEndedEvent {
    beneficiary_id: AccountAddress,
    username: String,
    ended_by: AccountAddress,
    end_reason_code: u8,
    swept_mys_amount: u64,
    ended_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUsernameBeneficiaryConflictEvent {
    username: String,
    existing_beneficiary_id: AccountAddress,
    attempted_by: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsCreatorIdentityWalletLinkedEvent {
    creator_identity_source: u8,
    creator_identity_hash: Vec<u8>,
    wallet: AccountAddress,
    beneficiary_id: AccountAddress,
    linked_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsEmergencyKillSwitchEvent {
    admin: AccountAddress,
    trading_enabled: bool,
    timestamp: u64,
    reason: String,
}

#[derive(Debug, Deserialize)]
pub struct BcsPocRedirectionUpdatedEventV1 {
    pool_id: AccountAddress,
    post_id: AccountAddress,
    redirect_to: Option<AccountAddress>,
    redirect_percentage: Option<u64>,
    updated_by: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPocRedirectionUpdatedEvent {
    pool_id: AccountAddress,
    post_id: AccountAddress,
    redirect_to: Option<AccountAddress>,
    redirect_percentage: Option<u64>,
    poc_redirection_kind: u8,
    updated_by: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPlatformStatus {
    status: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPlatformConfigUpdatedEvent {
    updated_by: AccountAddress,
    max_reasoning_length: u64,
    max_cover_photo_url_length: u64,
    max_media_previews: u64,
    max_badge_name_length: u64,
    max_badge_description_length: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
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
    cover_photo: Option<String>,
    media_previews: Option<Vec<String>>,
    primary_category: String,
    secondary_category: Option<String>,
    status: BcsPlatformStatus,
    release_date: String,
    wants_dao_governance: bool,
    governance_registry_id: Option<AccountAddress>,
    delegate_count: Option<u64>,
    delegate_term_epochs: Option<u64>,
    proposal_submission_cost: Option<u64>,
    max_votes_per_user: Option<u64>,
    quadratic_base_cost: Option<u64>,
    voting_period_epochs: Option<u64>,
    quorum_votes: Option<u64>,
    moderators_group_id: BcsMoveObjectId,
    redirect_uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BcsModeratorPermissionsGrantedEvent {
    platform_id: AccountAddress,
    moderators_group_id: BcsMoveObjectId,
    member: AccountAddress,
    permissions: Vec<String>,
    granted_by: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsModeratorPermissionsRevokedEvent {
    platform_id: AccountAddress,
    moderators_group_id: BcsMoveObjectId,
    member: AccountAddress,
    permissions: Vec<String>,
    revoked_by: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct BcsModeratorRemovedEvent {
    platform_id: AccountAddress,
    moderators_group_id: BcsMoveObjectId,
    member: AccountAddress,
    removed_by: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPlatformUpdatedEvent {
    platform_id: AccountAddress,
    name: String,
    tagline: String,
    description: String,
    logo: String,
    terms_of_service: String,
    privacy_policy: String,
    platforms: Vec<String>,
    links: Vec<String>,
    cover_photo: Option<String>,
    media_previews: Option<Vec<String>>,
    primary_category: String,
    secondary_category: Option<String>,
    status: BcsPlatformStatus,
    release_date: String,
    shutdown_date: Option<String>,
    redirect_uri: Option<String>,
    updated_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPlatformApprovalChangedEvent {
    platform_id: AccountAddress,
    approved: bool,
    changed_by: AccountAddress,
    reasoning: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsPlatformDeletedEvent {
    platform_id: AccountAddress,
    name: String,
    developer: AccountAddress,
    deleted_by: AccountAddress,
    timestamp: u64,
    reasoning: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUserJoinedPlatformEvent {
    wallet_address: AccountAddress,
    platform_id: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BcsUserLeftPlatformEvent {
    wallet_address: AccountAddress,
    platform_id: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPlatformTreasuryWithdrawalEvent {
    platform_id: AccountAddress,
    recipient: AccountAddress,
    amount: u64,
    reason_code: u8,
    executed_by: AccountAddress,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct BcsPlatformTreasuryFundedEvent {
    platform_id: AccountAddress,
    amount: u64,
    funded_by: AccountAddress,
    new_balance: u64,
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
    parse_event_contents_inner(module, event_name, contents).map_err(|e| {
        tracing::debug!(%module, %event_name, error = %e.error, hex_preview = %e.contents_hex_preview(32), "event parse failed");
        e
    })
}

fn parse_event_contents_inner(
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
        "poc_vault" => parse_poc_vault_event(event_name, contents),
        "poc_username_beneficiary" => parse_poc_username_beneficiary_event(event_name, contents),
        "mydata" | "my_ip" => parse_mydata_event(event_name, contents),
        "insurance" => parse_insurance_event(event_name, contents),
        "social_proof_of_truth" | "spot" => parse_spot_event(event_name, contents),
        "social_proof_tokens" | "spt" => parse_spt_event(event_name, contents),
        "subscription" | "profile_subscription" => parse_subscription_event(event_name, contents),
        "upgrade" => parse_upgrade_event(event_name, contents),
        "memory" => parse_memory_event(event_name, contents),
        "ai_credit" => parse_ai_credit_event(event_name, contents),
        "paid_messaging_policy" => parse_paid_messaging_policy_event(event_name, contents),
        "messaging_config" | "messaging" => parse_messaging_event(event_name, contents),
        "message_log" => parse_message_log_event(event_name, contents),
        _ => Ok(None),
    };

    if matches!(&result, Ok(None)) {
        tracing::debug!(
            %module,
            %event_name,
            "module event parser returned Ok(None); attempting raw JSON fallback"
        );
    }

    if let Ok(Some(v)) = result {
        return Ok(v);
    }

    match serde_json::from_slice::<serde_json::Value>(contents) {
        Ok(json) => Ok(json),
        Err(json_err) => {
            if let Err(e) = result {
                Err(e)
            } else {
                Err(EventParseError {
                    error: json_err.to_string(),
                    contents: contents.to_vec(),
                })
            }
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
                "bio": ev.bio,
                "profile_picture": ev.profile_picture,
                "cover_photo": ev.cover_photo,
                "updated_at": ev.updated_at,
                "x_username": ev.x_username,
                "website": ev.website,
                "birthdate": ev.birthdate,
                "location": ev.location,
            })))
        }
        "UsernameClaimedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameClaimedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "profile_id": addr_to_string(&ev.profile_id),
            })))
        }
        "UsernameReassignedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameReassignedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "profile_id": addr_to_string(&ev.profile_id),
                "admin": addr_to_string(&ev.admin),
                "reason_code": ev.reason_code,
                "prior_username": ev.prior_username,
            })))
        }
        "UsernameReservedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameReservedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "reason": ev.reason,
                "reserved_by": addr_to_string(&ev.reserved_by),
            })))
        }
        "UsernameReleasedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameReleasedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "reason": ev.reason,
                "released_by": addr_to_string(&ev.released_by),
            })))
        }
        "ProfileXUsernameUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileXUsernameUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "profile_id": addr_to_string(&ev.profile_id),
                "owner_address": addr_to_string(&ev.owner),
                "x_username": ev.x_username,
                "updated_by": addr_to_string(&ev.updated_by),
                "updated_at": ev.updated_at,
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
        "EcosystemBadgeSelectionClearedEvent" => {
            let ev = bcs::from_bytes::<BcsEcosystemBadgeSelectionClearedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "profile_id": addr_to_string(&ev.profile_id),
                "cleared_by": addr_to_string(&ev.cleared_by),
                "cleared_at": ev.cleared_at,
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
        "TokensVestedEvent" => {
            let ev = bcs::from_bytes::<BcsTokensVestedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "wallet_id": addr_to_string(&ev.wallet_id),
                "owner": addr_to_string(&ev.owner),
                "total_amount": ev.total_amount,
                "start_time": ev.start_time,
                "schedule_end": ev.schedule_end,
                "pieces": ev.pieces.iter().map(|p| serde_json::json!({
                    "kind": p.kind,
                    "time_offset": p.time_offset,
                    "duration": p.duration,
                    "amount_bps": p.amount_bps,
                    "curve_factor": p.curve_factor,
                })).collect::<Vec<_>>(),
                "vested_at": ev.vested_at,
            })))
        }
        "TokensClaimedEvent" => {
            let ev = bcs::from_bytes::<BcsTokensClaimedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "wallet_id": addr_to_string(&ev.wallet_id),
                "owner": addr_to_string(&ev.owner),
                "claimed_amount": ev.claimed_amount,
                "remaining_balance": ev.remaining_balance,
                "claimed_at": ev.claimed_at,
            })))
        }
        "VestingWalletDeletedEvent" => {
            let ev = bcs::from_bytes::<BcsVestingWalletDeletedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "wallet_id": addr_to_string(&ev.wallet_id),
                "owner": addr_to_string(&ev.owner),
                "deleted_at": ev.deleted_at,
            })))
        }
        "EcosystemTreasuryUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsEcosystemTreasuryUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "new_treasury_address": addr_to_string(&ev.new_treasury_address),
                "timestamp": ev.timestamp,
            })))
        }
        "ProfileConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "max_vesting_pieces": ev.max_vesting_pieces,
                "curve_factor_min": ev.curve_factor_min,
                "curve_factor_max": ev.curve_factor_max,
                "curve_precision": ev.curve_precision,
                "min_claim_threshold_divisor": ev.min_claim_threshold_divisor,
                "min_username_length": ev.min_username_length,
                "max_username_length": ev.max_username_length,
                "username_sale_fee_bps": ev.username_sale_fee_bps,
                "timestamp": ev.timestamp,
            })))
        }
        "UsernameListingCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameListingCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "seller": addr_to_string(&ev.seller),
                "seller_profile_id": addr_to_string(&ev.seller_profile_id),
                "min_price": ev.min_price,
                "created_at": ev.created_at,
            })))
        }
        "UsernameListingCancelledEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameListingCancelledEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "seller": addr_to_string(&ev.seller),
                "seller_profile_id": addr_to_string(&ev.seller_profile_id),
                "cancelled_at": ev.cancelled_at,
            })))
        }
        "UsernameOfferCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameOfferCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "seller_profile_id": addr_to_string(&ev.seller_profile_id),
                "buyer": addr_to_string(&ev.buyer),
                "buyer_profile_id": addr_to_string(&ev.buyer_profile_id),
                "amount": ev.amount,
                "created_at": ev.created_at,
            })))
        }
        "UsernameOfferAcceptedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameOfferAcceptedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "replacement_username": ev.replacement_username,
                "seller": addr_to_string(&ev.seller),
                "seller_profile_id": addr_to_string(&ev.seller_profile_id),
                "buyer": addr_to_string(&ev.buyer),
                "buyer_profile_id": addr_to_string(&ev.buyer_profile_id),
                "amount": ev.amount,
                "accepted_at": ev.accepted_at,
            })))
        }
        "UsernameOfferRejectedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameOfferRejectedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "seller_profile_id": addr_to_string(&ev.seller_profile_id),
                "buyer": addr_to_string(&ev.buyer),
                "buyer_profile_id": addr_to_string(&ev.buyer_profile_id),
                "rejected_by": addr_to_string(&ev.rejected_by),
                "amount": ev.amount,
                "rejected_at": ev.rejected_at,
                "is_revoked": ev.is_revoked,
            })))
        }
        "UsernameSaleSettledEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameSaleSettledEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "listed_username": ev.listed_username,
                "replacement_username": ev.replacement_username,
                "seller": addr_to_string(&ev.seller),
                "seller_profile_id": addr_to_string(&ev.seller_profile_id),
                "buyer": addr_to_string(&ev.buyer),
                "buyer_profile_id": addr_to_string(&ev.buyer_profile_id),
                "amount": ev.amount,
                "settled_at": ev.settled_at,
                "prior_buyer_username": ev.prior_buyer_username,
            })))
        }
        "UsernameSaleFeeEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameSaleFeeEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "seller": addr_to_string(&ev.seller),
                "seller_profile_id": addr_to_string(&ev.seller_profile_id),
                "buyer": addr_to_string(&ev.buyer),
                "buyer_profile_id": addr_to_string(&ev.buyer_profile_id),
                "sale_amount": ev.sale_amount,
                "fee_amount": ev.fee_amount,
                "fee_recipient": addr_to_string(&ev.fee_recipient),
                "timestamp": ev.timestamp,
            })))
        }
        // PaidMessagingSettingsUpdatedEvent removed from profile module; policy lives in messaging.
        _ => Ok(None),
    }
}

fn parse_paid_messaging_policy_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "PaidMessagingPolicyUpdated" => {
            let ev = bcs::from_bytes::<BcsPaidMessagingPolicyUpdated>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "wallet": ev.wallet(),
                "enabled": ev.enabled,
                "min_cost": ev.min_cost,
            })))
        }
        _ => Ok(None),
    }
}

fn parse_messaging_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "MessagingConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsMessagingConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "timestamp": ev.timestamp,
                "paid_msg_platform_fee_bps": ev.paid_msg_platform_fee_bps,
                "paid_msg_treasury_fee_bps": ev.paid_msg_treasury_fee_bps,
                "payment_expiration_ms": ev.payment_expiration_ms,
                "min_reply_chars": ev.min_reply_chars,
                "max_dedupe_key_bytes": ev.max_dedupe_key_bytes,
            })))
        }
        "AgentGroupCreated" => {
            let ev = bcs::from_bytes::<BcsAgentGroupCreated>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "group_id": move_object_id_to_string(&ev.group_id),
                "creator_actor": addr_to_string(&ev.creator_actor),
                "creator_principal": addr_to_string(&ev.creator_principal),
                "creator_sub_agent_id": optional_move_object_id_json(&ev.creator_sub_agent_id),
                "creator_identity_class": ev.creator_identity_class,
                "organization_id": optional_move_object_id_json(&ev.organization_id),
                "group_name": ev.group_name,
                "group_uuid": ev.group_uuid,
                "created_at": ev.created_at,
            })))
        }
        _ => Ok(None),
    }
}

fn parse_message_log_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "MessageDigestSent" => {
            let ev = bcs::from_bytes::<BcsMessageDigestSent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "group_id": move_object_id_to_string(&ev.group_id),
                "seq": ev.seq,
                "sender": addr_to_string(&ev.sender),
                "recipient": addr_to_string(&ev.recipient),
                "content_digest": hex::encode(ev.content_digest),
                "content_uri": ev.content_uri,
                "created_at_ms": ev.created_at_ms,
            })))
        }
        "PaidMessageSent" => {
            let ev = bcs::from_bytes::<BcsPaidMessageSent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "group_id": move_object_id_to_string(&ev.group_id),
                "seq": ev.seq,
                "payer": addr_to_string(&ev.payer),
                "recipient": addr_to_string(&ev.recipient),
                "amount": ev.amount,
                "created_at_ms": ev.created_at_ms,
            })))
        }
        "PaidMessageReplied" => {
            let ev = bcs::from_bytes::<BcsPaidMessageReplied>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "group_id": move_object_id_to_string(&ev.group_id),
                "paid_msg_seq": ev.paid_msg_seq,
                "recipient": addr_to_string(&ev.recipient),
                "reply_char_count": ev.reply_char_count,
            })))
        }
        "PaymentClaimed" => {
            let ev = bcs::from_bytes::<BcsPaymentClaimed>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "group_id": move_object_id_to_string(&ev.group_id),
                "seq": ev.seq,
                "recipient": addr_to_string(&ev.recipient),
                "amount": ev.amount,
                "claimed_at_ms": ev.claimed_at_ms,
            })))
        }
        "PaymentClaimedSettled" => {
            let ev = bcs::from_bytes::<BcsPaymentClaimedSettled>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "group_id": move_object_id_to_string(&ev.group_id),
                "seq": ev.seq,
                "recipient": addr_to_string(&ev.recipient),
                "total_amount": ev.total_amount,
                "platform_fee": ev.platform_fee,
                "treasury_fee": ev.treasury_fee,
                "net_amount": ev.net_amount,
                "platform_fee_recipient": addr_to_string(&ev.platform_fee_recipient),
                "ecosystem_fee_recipient": addr_to_string(&ev.ecosystem_fee_recipient),
                "claimed_at_ms": ev.claimed_at_ms,
            })))
        }
        "PaymentRefunded" => {
            let ev = bcs::from_bytes::<BcsPaymentRefunded>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "group_id": move_object_id_to_string(&ev.group_id),
                "seq": ev.seq,
                "payer": addr_to_string(&ev.payer),
                "amount": ev.amount,
                "refunded_at_ms": ev.refunded_at_ms,
            })))
        }
        // MessageLogCreated intentionally ignored — handled by messaging stack.
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
            let json = match bcs::from_bytes::<BcsDelegateElectedEvent>(contents) {
                Ok(ev) => serde_json::json!({
                    "delegate_address": addr_to_string(&ev.delegate_address),
                    "registry_type": ev.registry_type,
                    "term_start": ev.term_start,
                    "term_end": ev.term_end,
                    "upvotes": ev.upvotes,
                    "downvotes": ev.downvotes,
                }),
                Err(e1) => {
                    if contents.len() == DELEGATE_ELECTED_BCS_V0_LEN {
                        tracing::warn!(
                            full_format_error = %e1,
                            payload_len = contents.len(),
                            "DelegateElectedEvent: legacy BCS layout (no vote fields); JSON will use 0/0 unless governance handler carries counts from nominated_delegates"
                        );
                        let ev = bcs::from_bytes::<BcsDelegateElectedEventV0>(contents)
                            .map_err(|e2| EventParseError {
                                error: format!(
                                    "DelegateElectedEvent: v1: {e1}; v0 ({DELEGATE_ELECTED_BCS_V0_LEN} bytes): {e2}"
                                ),
                                contents: contents.to_vec(),
                            })?;
                        serde_json::json!({
                            "delegate_address": addr_to_string(&ev.delegate_address),
                            "registry_type": ev.registry_type,
                            "term_start": ev.term_start,
                            "term_end": ev.term_end,
                            "upvotes": 0u64,
                            "downvotes": 0u64,
                        })
                    } else {
                        return Err(bcs_parse_err(e1, contents));
                    }
                }
            };
            Ok(Some(json))
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
        "DelegateVoteClearedEvent" => {
            let ev = bcs::from_bytes::<BcsDelegateVoteClearedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "target_address": addr_to_string(&ev.target_address),
                "voter": addr_to_string(&ev.voter),
                "registry_type": ev.registry_type,
                "is_active_delegate": ev.is_active_delegate,
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
        "ProposalRewardPoolForfeitedToTreasuryEvent" => {
            let ev = bcs::from_bytes::<BcsProposalRewardPoolForfeitedToTreasuryEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "recipient": addr_to_string(&ev.recipient),
                "amount": ev.amount,
                "reason": ev.reason,
                "registry_type": ev.registry_type,
                "treasury_route": ev.treasury_route,
                "forfeited_time": ev.forfeited_time,
            })))
        }
        "ProposalImplementationRewardToSubmitterEvent" => {
            let ev = bcs::from_bytes::<BcsProposalImplementationRewardToSubmitterEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "proposal_id": addr_to_string(&ev.proposal_id),
                "submitter": addr_to_string(&ev.submitter),
                "amount": ev.amount,
                "registry_type": ev.registry_type,
                "sent_time": ev.sent_time,
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
        "DelegatePanelRefreshedEvent" => {
            let ev = bcs::from_bytes::<BcsDelegatePanelRefreshedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "registry_id": addr_to_string(&ev.registry_id),
                "boundary_epoch": ev.boundary_epoch,
                "registry_type": ev.registry_type,
                "executed_at_epoch": ev.executed_at_epoch,
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
            let ev = bcs_post_created_from_bytes(contents)?;
            let access_json = post_access_json_from_bcs(
                &bcs::from_bytes::<BcsPostCreatedEvent>(contents)
                    .map_err(|e| bcs_parse_err(e, contents))?
                    .access,
            );
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "platform_id": addr_to_string(&ev.platform_id),
                "permissions": ev.permissions,
                "content": ev.content,
                "post_type": ev.post_type,
                "parent_post_id": ev.parent_post_id.as_ref().map(addr_to_string),
                "mentions": mentions_to_json(&ev.mentions),
                "media_urls": ev.media_urls,
                "metadata_json": ev.metadata_json,
                "post_access_kind": ev.post_access_kind,
                "mydata_id": ev.mydata_id.as_ref().map(addr_to_string),
                "subscription_service_id": ev.subscription_service_id.as_ref().map(addr_to_string),
                "requires_subscription": ev.requires_subscription,
                "access": access_json["access"],
                "promotion_id": ev.promotion_id.as_ref().map(addr_to_string),
                "revenue_redirect_to": ev.revenue_redirect_to.as_ref().map(addr_to_string),
                "revenue_redirect_percentage": ev.revenue_redirect_percentage,
                "enable_spt": ev.enable_spt,
                "spt_id": ev.spt_id.as_ref().map(addr_to_string),
                "poc_redirection_kind": ev.poc_redirection_kind,
                "actor_address": addr_to_string(&ev.actor_address),
                "sub_agent_id": ev.sub_agent_id,
                "organization_id": ev.organization_id,
                "action_identity_class": ev.action_identity_class,
            })))
        }
        "CommentCreatedEvent" => {
            let ev = bcs_comment_created_from_bytes(contents)?;
            Ok(Some(serde_json::json!({
                "comment_id": addr_to_string(&ev.comment_id),
                "post_id": addr_to_string(&ev.post_id),
                "parent_comment_id": ev.parent_comment_id.as_ref().map(addr_to_string),
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "content": ev.content,
                "mentions": mentions_to_json(&ev.mentions),
                "actor_address": addr_to_string(&ev.actor_address),
                "sub_agent_id": ev.sub_agent_id,
                "organization_id": ev.organization_id,
                "action_identity_class": ev.action_identity_class,
            })))
        }
        "ReactionEvent" | "ReactionAddedEvent" => {
            let ev = bcs_reaction_from_bytes(contents)?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.object_id),
                "user_address": addr_to_string(&ev.actor_address),
                "reaction_text": ev.reaction,
                "is_post": ev.is_post,
                "principal_owner": addr_to_string(&ev.principal_owner),
                "actor_address": addr_to_string(&ev.actor_address),
                "sub_agent_id": ev.sub_agent_id,
                "organization_id": ev.organization_id,
                "action_identity_class": ev.action_identity_class,
            })))
        }
        "RemoveReactionEvent" | "ReactionRemovedEvent" => {
            let ev = bcs_remove_reaction_from_bytes(contents)?;
            Ok(Some(serde_json::json!({
                "object_id": addr_to_string(&ev.object_id),
                "user_address": addr_to_string(&ev.actor_address),
                "reaction_text": ev.reaction,
                "is_post": ev.is_post,
                "principal_owner": addr_to_string(&ev.principal_owner),
                "actor_address": addr_to_string(&ev.actor_address),
                "sub_agent_id": ev.sub_agent_id,
                "organization_id": ev.organization_id,
                "action_identity_class": ev.action_identity_class,
            })))
        }
        "RepostEvent" | "RepostCreatedEvent" => {
            let ev = bcs_repost_from_bytes(contents)?;
            Ok(Some(serde_json::json!({
                "repost_id": addr_to_string(&ev.repost_id),
                "original_id": addr_to_string(&ev.original_id),
                "original_post_id": addr_to_string(&ev.original_id),
                "is_original_post": ev.is_original_post,
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "actor_address": addr_to_string(&ev.actor_address),
                "sub_agent_id": ev.sub_agent_id,
                "organization_id": ev.organization_id,
                "action_identity_class": ev.action_identity_class,
            })))
        }
        "RepostRemovedEvent" => {
            let ev = bcs::from_bytes::<BcsRepostRemovedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "repost_id": addr_to_string(&ev.repost_id),
                "original_id": addr_to_string(&ev.original_id),
                "is_original_post": true,
                "owner": addr_to_string(&ev.owner),
                "actor_address": addr_to_string(&ev.actor_address),
                "sub_agent_id": optional_move_object_id_json(&ev.sub_agent_id),
                "organization_id": optional_move_object_id_json(&ev.organization_id),
                "action_identity_class": ev.action_identity_class,
                "removed_at": ev.removed_at,
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
                "coin_type": bcs_move_type_name_display(&ev.coin_type),
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
                "moderated_at": 0u64,
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
        "PromotedPostCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsPromotedPostCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "payment_per_view": ev.payment_per_view,
                "total_budget": ev.total_budget,
                "created_at": ev.created_at,
            })))
        }
        "PromotedPostViewsBatchConfirmedEvent" => {
            let ev = bcs::from_bytes::<BcsPromotedPostViewsBatchConfirmedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            let items: Vec<serde_json::Value> = ev
                .items
                .iter()
                .map(|item| {
                    serde_json::json!({
                        "post_id": addr_to_string(&item.post_id),
                        "promotion_id": addr_to_string(&item.promotion_id),
                        "payment_amount": item.payment_amount,
                        "platform_fee": item.platform_fee,
                        "ecosystem_fee": item.ecosystem_fee,
                        "recipient_amount": item.recipient_amount,
                        "view_duration": item.view_duration,
                    })
                })
                .collect();
            Ok(Some(serde_json::json!({
                "viewer": addr_to_string(&ev.viewer),
                "platform_id": addr_to_string(&ev.platform_id),
                "timestamp": ev.timestamp,
                "items": items,
                "total_payment_amount": ev.total_payment_amount,
                "total_platform_fee": ev.total_platform_fee,
                "total_ecosystem_fee": ev.total_ecosystem_fee,
                "total_recipient_amount": ev.total_recipient_amount,
            })))
        }
        "PromotionStatusToggledEvent" => {
            let ev = bcs::from_bytes::<BcsPromotionStatusToggledEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "promotion_id": addr_to_string(&ev.post_id),
                "toggled_by": addr_to_string(&ev.toggled_by),
                "new_status": ev.new_status,
                "timestamp": ev.timestamp,
            })))
        }
        "PromotionFundsWithdrawnEvent" => {
            let ev = bcs::from_bytes::<BcsPromotionFundsWithdrawnEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "promotion_id": addr_to_string(&ev.post_id),
                "owner": addr_to_string(&ev.owner),
                "withdrawn_amount": ev.withdrawn_amount,
                "timestamp": ev.timestamp,
            })))
        }
        "PostSubscriptionGateEnabledEvent" => Ok(None),
        "PostSubscriptionAccessEvent" => {
            let ev = bcs::from_bytes::<BcsPostSubscriptionAccessEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "service_id": addr_to_string(&ev.service_id),
                "subscription_id": addr_to_string(&ev.subscription_id),
                "subscriber": addr_to_string(&ev.subscriber),
                "timestamp": ev.timestamp,
            })))
        }
        "PostParametersUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsPostParametersUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "timestamp": ev.timestamp,
                "max_content_length": ev.max_content_length,
                "max_media_urls": ev.max_media_urls,
                "max_mentions": ev.max_mentions,
                "max_metadata_size": ev.max_metadata_size,
                "max_description_length": ev.max_description_length,
                "max_reaction_length": ev.max_reaction_length,
                "commenter_tip_percentage": ev.commenter_tip_percentage,
                "repost_tip_percentage": ev.repost_tip_percentage,
                "min_promotion_amount": ev.min_promotion_amount,
                "max_promotion_amount": ev.max_promotion_amount,
                "min_view_duration_ms": ev.min_view_duration_ms,
                "platform_fee_bps": ev.platform_fee_bps,
                "ecosystem_fee_bps": ev.ecosystem_fee_bps,
            })))
        }
        _ => Ok(None),
    }
}

fn sub_agent_fields_json(ev: &BcsSubAgentRegisteredEvent) -> serde_json::Value {
    serde_json::json!({
        "account_id": addr_to_string(&ev.account_id),
        "principal_owner": addr_to_string(&ev.principal_owner),
        "profile_id": addr_to_string(&ev.profile_id),
        "organization_id": addr_to_string(&ev.organization_id),
        "agent_object_id": addr_to_string(&ev.agent_object_id),
        "derived_address": addr_to_string(&ev.derived_address),
        "label": ev.label,
        "identity_class": ev.identity_class,
        "role_tags": ev.role_tags,
        "capabilities": ev.capabilities,
        "delegatable_caps": ev.delegatable_caps,
        "register_scope": ev.register_scope,
        "approval_required_caps": ev.approval_required_caps,
        "max_action_spend": ev.max_action_spend,
        "platform_scope": optional_addr_json(&ev.platform_scope),
        "parent_object_id": optional_addr_json(&ev.parent_object_id),
        "depth": ev.depth,
        "registered_by": addr_to_string(&ev.registered_by),
        "expires_at": ev.expires_at,
        "active": ev.active,
        "created_at": ev.created_at,
    })
}

fn parse_memory_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "MemoryAccountCreated" => {
            let ev = bcs::from_bytes::<BcsMemoryAccountCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "account_id": addr_to_string(&ev.account_id),
                "owner": addr_to_string(&ev.owner),
                "profile_id": addr_to_string(&ev.profile_id),
            })))
        }
        "SubAgentRegistered" => {
            let ev = bcs::from_bytes::<BcsSubAgentRegisteredEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(sub_agent_fields_json(&ev)))
        }
        "SubAgentUpdated" => {
            let ev = bcs::from_bytes::<BcsSubAgentUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(sub_agent_fields_json(&BcsSubAgentRegisteredEvent {
                account_id: ev.account_id,
                principal_owner: ev.principal_owner,
                profile_id: ev.profile_id,
                organization_id: ev.organization_id,
                agent_object_id: ev.agent_object_id,
                derived_address: ev.derived_address,
                label: ev.label,
                identity_class: ev.identity_class,
                role_tags: ev.role_tags,
                capabilities: ev.capabilities,
                delegatable_caps: ev.delegatable_caps,
                register_scope: ev.register_scope,
                approval_required_caps: ev.approval_required_caps,
                max_action_spend: ev.max_action_spend,
                platform_scope: ev.platform_scope,
                parent_object_id: ev.parent_object_id,
                depth: ev.depth,
                registered_by: ev.registered_by,
                expires_at: ev.expires_at,
                active: ev.active,
                created_at: ev.created_at,
            })))
        }
        "SubAgentDeactivated" => {
            let ev = bcs::from_bytes::<BcsSubAgentDeactivatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "account_id": addr_to_string(&ev.account_id),
                "principal_owner": addr_to_string(&ev.principal_owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "derived_address": addr_to_string(&ev.derived_address),
            })))
        }
        "SubAgentRevoked" => {
            let ev = bcs::from_bytes::<BcsSubAgentRevokedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "account_id": addr_to_string(&ev.account_id),
                "principal_owner": addr_to_string(&ev.principal_owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "derived_address": addr_to_string(&ev.derived_address),
            })))
        }
        "SubAgentsClearedOnTransfer" => {
            let ev = bcs::from_bytes::<BcsSubAgentsClearedOnTransferEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "account_id": addr_to_string(&ev.account_id),
                "principal_owner": addr_to_string(&ev.principal_owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "previous_owner": addr_to_string(&ev.previous_owner),
                "new_owner": addr_to_string(&ev.new_owner),
                "revoked_count": ev.revoked_count,
            })))
        }
        "MemoryAccountDeactivated" => {
            let ev = bcs::from_bytes::<BcsMemoryAccountDeactivatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "account_id": addr_to_string(&ev.account_id),
                "owner": addr_to_string(&ev.owner),
            })))
        }
        "MemoryAccountReactivated" => {
            let ev = bcs::from_bytes::<BcsMemoryAccountReactivatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "account_id": addr_to_string(&ev.account_id),
                "owner": addr_to_string(&ev.owner),
            })))
        }
        "MemoryAccountMigrated" => {
            let ev = bcs::from_bytes::<BcsMemoryAccountMigratedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "account_id": addr_to_string(&ev.account_id),
                "from": ev.from,
                "to": ev.to,
            })))
        }
        "MemoryRegistryMigrated" => {
            let ev = bcs::from_bytes::<BcsMemoryRegistryMigratedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "registry_id": addr_to_string(&ev.registry_id),
                "from": ev.from,
                "to": ev.to,
            })))
        }
        "AgentMemoryVaultCreated" => {
            let ev = bcs::from_bytes::<BcsAgentMemoryVaultCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "vault_id": addr_to_string(&ev.vault_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "memory_account_id": addr_to_string(&ev.memory_account_id),
            })))
        }
        "AgenticOrganizationCreated" => {
            let ev = bcs::from_bytes::<BcsAgenticOrganizationCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "principal_owner": addr_to_string(&ev.principal_owner),
                "profile_id": addr_to_string(&ev.profile_id),
                "name": ev.name,
                "description": ev.description,
                "org_type": ev.org_type,
                "created_at": ev.created_at,
            })))
        }
        "AgenticOrganizationUpdated" => {
            let ev = bcs::from_bytes::<BcsAgenticOrganizationUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "name": ev.name,
                "description": ev.description,
            })))
        }
        "AgenticOrganizationCategoryUpdated" => {
            let ev = bcs::from_bytes::<BcsAgenticOrganizationCategoryUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "org_type": ev.org_type,
                "previous_org_type": ev.previous_org_type,
                "updated_at": ev.updated_at,
            })))
        }
        "AgenticOrganizationDeactivated" => {
            let ev = bcs::from_bytes::<BcsAgenticOrganizationDeactivatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "deactivated_at": ev.deactivated_at,
            })))
        }
        "OrgMemoryGroupCreated" => {
            let ev = bcs::from_bytes::<BcsOrgMemoryGroupCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "group_id": addr_to_string(&ev.group_id),
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "principal_owner": addr_to_string(&ev.principal_owner),
                "created_at": ev.created_at,
            })))
        }
        "OrgMemoryPermissionGranted" => {
            let ev = bcs::from_bytes::<BcsOrgMemoryPermissionGrantedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "group_id": addr_to_string(&ev.group_id),
                "member": addr_to_string(&ev.member),
                "permissions_mask": ev.permissions_mask,
                "granted_by": addr_to_string(&ev.granted_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "OrgMemoryPermissionRevoked" => {
            let ev = bcs::from_bytes::<BcsOrgMemoryPermissionRevokedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "group_id": addr_to_string(&ev.group_id),
                "member": addr_to_string(&ev.member),
                "permissions_mask": ev.permissions_mask,
                "revoked_by": addr_to_string(&ev.revoked_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "OrgRoleDefined" => {
            let ev = bcs::from_bytes::<BcsOrgRoleDefinedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "role_name": ev.role_name,
                "mask": ev.mask,
                "previous_mask": ev.previous_mask,
                "defined_by": addr_to_string(&ev.defined_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "OrgRoleAssigned" => {
            let ev = bcs::from_bytes::<BcsOrgRoleAssignedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "group_id": addr_to_string(&ev.group_id),
                "member": addr_to_string(&ev.member),
                "role_name": ev.role_name,
                "mask": ev.mask,
                "granted_mask": ev.granted_mask,
                "assigned_by": addr_to_string(&ev.assigned_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "OrgRoleRevoked" => {
            let ev = bcs::from_bytes::<BcsOrgRoleRevokedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "group_id": addr_to_string(&ev.group_id),
                "member": addr_to_string(&ev.member),
                "role_name": ev.role_name,
                "revoked_mask": ev.revoked_mask,
                "revoked_by": addr_to_string(&ev.revoked_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "OrgInvitationCreated" => {
            let ev = bcs::from_bytes::<BcsOrgInvitationCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "invitee": addr_to_string(&ev.invitee),
                "role_name": ev.role_name,
                "permissions_mask": ev.permissions_mask,
                "invited_by": addr_to_string(&ev.invited_by),
                "timestamp_ms": ev.timestamp_ms,
                "expires_at_ms": ev.expires_at_ms,
            })))
        }
        "OrgInvitationAccepted" => {
            let ev = bcs::from_bytes::<BcsOrgInvitationAcceptedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "group_id": addr_to_string(&ev.group_id),
                "invitee": addr_to_string(&ev.invitee),
                "role_name": ev.role_name,
                "permissions_mask": ev.permissions_mask,
                "granted_mask": ev.granted_mask,
                "accepted_by": addr_to_string(&ev.accepted_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "OrgInvitationDeclined" => {
            let ev = bcs::from_bytes::<BcsOrgInvitationDeclinedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "organization_id": addr_to_string(&ev.organization_id),
                "account_id": addr_to_string(&ev.account_id),
                "invitee": addr_to_string(&ev.invitee),
                "declined_by": addr_to_string(&ev.declined_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "MemoryConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsMemoryConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "max_organizations_per_user": ev.max_organizations_per_user,
                "org_category_update_cooldown_ms": ev.org_category_update_cooldown_ms,
                "max_agent_depth": ev.max_agent_depth,
                "max_label_length": ev.max_label_length,
                "max_org_name_length": ev.max_org_name_length,
                "max_org_description_length": ev.max_org_description_length,
                "timestamp": ev.timestamp,
            })))
        }
        _ => Ok(None),
    }
}

fn parse_ai_credit_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "AiCreditBalanceCreated" => {
            let ev = bcs::from_bytes::<BcsAiCreditBalanceCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "memory_account_id": addr_to_string(&ev.memory_account_id),
                "principal_owner": addr_to_string(&ev.principal_owner),
                "profile_id": addr_to_string(&ev.profile_id),
            })))
        }
        "AiCreditDeposited" => {
            let ev = bcs::from_bytes::<BcsAiCreditDepositedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "amount_mist": ev.amount_mist,
                "new_balance_mist": ev.new_balance_mist,
            })))
        }
        "AiCreditWithdrawn" => {
            let ev = bcs::from_bytes::<BcsAiCreditWithdrawnEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "amount_mist": ev.amount_mist,
                "new_balance_mist": ev.new_balance_mist,
            })))
        }
        "AiCreditAccountCapsUpdated" => {
            let ev = bcs::from_bytes::<BcsAiCreditAccountCapsUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "daily_cap_mist": ev.daily_cap_mist,
                "monthly_cap_mist": ev.monthly_cap_mist,
            })))
        }
        "AiCreditAgentBudgetUpdated" => {
            let ev = bcs::from_bytes::<BcsAiCreditAgentBudgetUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "budget_mist": ev.budget_mist,
                "daily_cap_mist": ev.daily_cap_mist,
                "monthly_cap_mist": ev.monthly_cap_mist,
                "require_approval_above_mist": ev.require_approval_above_mist,
            })))
        }
        "AiCreditAgentBudgetDisabled" => {
            let ev = bcs::from_bytes::<BcsAiCreditAgentBudgetDisabledEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
            })))
        }
        "AiCreditAgentBudgetChanged" => {
            let ev = bcs::from_bytes::<BcsAiCreditAgentBudgetChangedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "had_previous_entry": ev.had_previous_entry,
                "prev_budget_mist": ev.prev_budget_mist,
                "prev_daily_cap_mist": ev.prev_daily_cap_mist,
                "prev_monthly_cap_mist": ev.prev_monthly_cap_mist,
                "prev_require_approval_above_mist": ev.prev_require_approval_above_mist,
                "prev_enabled": ev.prev_enabled,
                "budget_mist": ev.budget_mist,
                "daily_cap_mist": ev.daily_cap_mist,
                "monthly_cap_mist": ev.monthly_cap_mist,
                "require_approval_above_mist": ev.require_approval_above_mist,
                "enabled": ev.enabled,
                "set_by": addr_to_string(&ev.set_by),
                "set_by_agent_id": optional_addr_json(&ev.set_by_agent_id),
                "organization_id": optional_addr_json(&ev.organization_id),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "AiCreditSpendApproved" => {
            let ev = bcs::from_bytes::<BcsAiCreditSpendApprovedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "approval_nonce": ev.approval_nonce,
                "max_amount_mist": ev.max_amount_mist,
                "expires_at_ms": ev.expires_at_ms,
                "approved_by": addr_to_string(&ev.approved_by),
                "approved_by_agent_id": optional_addr_json(&ev.approved_by_agent_id),
                "organization_id": optional_addr_json(&ev.organization_id),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "AiCreditSpendApprovalRevoked" => {
            let ev = bcs::from_bytes::<BcsAiCreditSpendApprovalRevokedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "approval_nonce": ev.approval_nonce,
                "revoked_by": addr_to_string(&ev.revoked_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "AiCreditSpendApprovalConsumed" => {
            let ev = bcs::from_bytes::<BcsAiCreditSpendApprovalConsumedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "approval_nonce": ev.approval_nonce,
                "amount_mist": ev.amount_mist,
                "approved_by": addr_to_string(&ev.approved_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "AiCreditUsageSettled" => {
            let ev = bcs::from_bytes::<BcsAiCreditUsageSettledEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "receipt_id": ev.receipt_id.to_string(),
                "amount_mist": ev.amount_mist,
                "usage_kind": ev.usage_kind,
                "settlement_nonce": ev.settlement_nonce,
                "remaining_mist": ev.remaining_mist,
            })))
        }
        "AiSpendReserved" => {
            let ev = bcs::from_bytes::<BcsAiSpendReservedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "reservation_nonce": ev.reservation_nonce,
                "max_amount_mist": ev.max_amount_mist,
                "provider_envelope_hash_hex": hex::encode(ev.provider_envelope_hash),
                "request_hash_hex": hex::encode(ev.request_hash),
                "fx_quote_id_hex": hex::encode(ev.fx_quote_id),
                "myso_usd_e8": ev.myso_usd_e8,
                "markup_bps": ev.markup_bps,
                "capture_deadline_ms": ev.capture_deadline_ms,
                "hard_expiry_ms": ev.hard_expiry_ms,
                "available_mist": ev.available_mist,
            })))
        }
        "AiSpendCaptured" => {
            let ev = bcs::from_bytes::<BcsAiSpendCapturedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "reservation_nonce": ev.reservation_nonce,
                "reserved_mist": ev.reserved_mist,
                "captured_mist": ev.captured_mist,
                "released_mist": ev.released_mist,
                "provider_cost_usd_micros": ev.provider_cost_usd_micros,
                "provider_generation_hash_hex": hex::encode(ev.provider_generation_hash),
                "fx_quote_id_hex": hex::encode(ev.fx_quote_id),
                "myso_usd_e8": ev.myso_usd_e8,
                "markup_bps": ev.markup_bps,
                "captured_at_ms": ev.captured_at_ms,
                "remaining_mist": ev.remaining_mist,
                "available_mist": ev.available_mist,
            })))
        }
        "AiSpendCancelled" => {
            let ev = bcs::from_bytes::<BcsAiSpendCancelledEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "reservation_nonce": ev.reservation_nonce,
                "released_mist": ev.released_mist,
                "cancelled_at_ms": ev.cancelled_at_ms,
                "available_mist": ev.available_mist,
            })))
        }
        "AiSpendExpired" => {
            let ev = bcs::from_bytes::<BcsAiSpendExpiredEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
                "agent_object_id": addr_to_string(&ev.agent_object_id),
                "reservation_nonce": ev.reservation_nonce,
                "released_mist": ev.released_mist,
                "expired_at_ms": ev.expired_at_ms,
                "available_mist": ev.available_mist,
            })))
        }
        "AiCreditBalanceDepleted" | "AiCreditBalancePaused" | "AiCreditBalanceReactivated" => {
            let ev = bcs::from_bytes::<BcsAiCreditBalanceIdEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "balance_id": addr_to_string(&ev.balance_id),
            })))
        }
        "AiCreditConfigInitialized" => {
            let ev = bcs::from_bytes::<BcsAiCreditConfigInitializedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "oracle_pubkey_hex": hex::encode(&ev.oracle_pubkey),
                "treasury_address": addr_to_string(&ev.treasury),
                "min_deposit_mist": ev.min_deposit_mist,
                "max_single_settlement_mist": ev.max_single_settlement_mist,
                "receipt_ttl_ms": ev.receipt_ttl_ms,
                "oracle_markup_bps": ev.oracle_markup_bps,
            })))
        }
        "AiCreditOraclePubkeyUpdated" => {
            let ev = bcs::from_bytes::<BcsAiCreditOraclePubkeyUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "new_pubkey_hex": hex::encode(&ev.new_pubkey),
            })))
        }
        "AiCreditMarkupUpdated" => {
            let ev = bcs::from_bytes::<BcsAiCreditMarkupUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "oracle_markup_bps": ev.oracle_markup_bps,
            })))
        }
        "AiCreditMinDepositUpdated" => {
            let ev = bcs::from_bytes::<BcsAiCreditMinDepositUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "min_deposit_mist": ev.min_deposit_mist,
            })))
        }
        "AiCreditSettlementLimitsUpdated" => {
            let ev = bcs::from_bytes::<BcsAiCreditSettlementLimitsUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "max_single_settlement_mist": ev.max_single_settlement_mist,
                "receipt_ttl_ms": ev.receipt_ttl_ms,
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
                "cover_photo": ev.cover_photo,
                "media_previews": ev.media_previews,
                "redirect_uri": ev.redirect_uri,
                "primary_category": ev.primary_category,
                "secondary_category": ev.secondary_category,
                "status": {"status": ev.status.status},
                "release_date": ev.release_date,
                "wants_dao_governance": ev.wants_dao_governance,
                "governance_registry_id": ev.governance_registry_id.as_ref().map(addr_to_string),
                "delegate_count": ev.delegate_count,
                "delegate_term_epochs": ev.delegate_term_epochs,
                "proposal_submission_cost": ev.proposal_submission_cost,
                "max_votes_per_user": ev.max_votes_per_user,
                "quadratic_base_cost": ev.quadratic_base_cost,
                "voting_period_epochs": ev.voting_period_epochs,
                "quorum_votes": ev.quorum_votes,
                "moderators_group_id": move_object_id_to_string(&ev.moderators_group_id),
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
                "logo": ev.logo,
                "terms_of_service": ev.terms_of_service,
                "privacy_policy": ev.privacy_policy,
                "platforms": ev.platforms,
                "links": ev.links,
                "cover_photo": ev.cover_photo,
                "media_previews": ev.media_previews,
                "redirect_uri": ev.redirect_uri,
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
        "ModeratorPermissionsGrantedEvent" => {
            let ev = bcs::from_bytes::<BcsModeratorPermissionsGrantedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "moderators_group_id": move_object_id_to_string(&ev.moderators_group_id),
                "member": addr_to_string(&ev.member),
                "permissions": ev.permissions,
                "granted_by": addr_to_string(&ev.granted_by),
            })))
        }
        "ModeratorPermissionsRevokedEvent" => {
            let ev = bcs::from_bytes::<BcsModeratorPermissionsRevokedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "moderators_group_id": move_object_id_to_string(&ev.moderators_group_id),
                "member": addr_to_string(&ev.member),
                "permissions": ev.permissions,
                "revoked_by": addr_to_string(&ev.revoked_by),
            })))
        }
        "ModeratorRemovedEvent" => {
            let ev = bcs::from_bytes::<BcsModeratorRemovedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "moderators_group_id": move_object_id_to_string(&ev.moderators_group_id),
                "member": addr_to_string(&ev.member),
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
        "PlatformTreasuryWithdrawalEvent" => {
            let ev = bcs::from_bytes::<BcsPlatformTreasuryWithdrawalEvent>(contents)
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
        "PlatformTreasuryFundedEvent" => {
            let ev = bcs::from_bytes::<BcsPlatformTreasuryFundedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "platform_id": addr_to_string(&ev.platform_id),
                "amount": ev.amount,
                "funded_by": addr_to_string(&ev.funded_by),
                "new_balance": ev.new_balance,
                "timestamp": ev.timestamp,
            })))
        }
        "PlatformConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsPlatformConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "max_reasoning_length": ev.max_reasoning_length,
                "max_cover_photo_url_length": ev.max_cover_photo_url_length,
                "max_media_previews": ev.max_media_previews,
                "max_badge_name_length": ev.max_badge_name_length,
                "max_badge_description_length": ev.max_badge_description_length,
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
        "PoCResultAppliedEvent" | "PocResultAppliedEvent" => {
            let ev = bcs::from_bytes::<BcsPoCResultAppliedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "poc_outcome": ev.poc_outcome,
                "poc_redirection_kind": ev.poc_redirection_kind,
                "similarity_detected": ev.similarity_detected,
                "timestamp": ev.timestamp,
            })))
        }
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
                "beneficiary_address": ev.beneficiary_address.as_ref().map(addr_to_string),
                "matched_anchor_id": ev.matched_anchor_id.as_ref().map(addr_to_string),
                "media_index": ev.media_index,
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
                "dispute_round": ev.dispute_round,
                "effective_fee": ev.effective_fee,
                "required_total_stake_quorum": ev.required_total_stake_quorum,
                "post_poc_disputes_submitted_after": ev.post_poc_disputes_submitted_after,
                "voting_start_ms": ev.voting_start_ms,
                "voting_end_ms": ev.voting_end_ms,
                "evidence": ev.evidence,
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
                "quorum_met": ev.quorum_met,
                "post_poc_disputes_submitted": ev.post_poc_disputes_submitted,
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
                "min_vote_stake": ev.min_vote_stake,
                "max_vote_stake": ev.max_vote_stake,
                "voting_duration_ms": ev.voting_duration_ms,
                "max_reasoning_length": ev.max_reasoning_length,
                "max_evidence_urls": ev.max_evidence_urls,
                "max_votes_per_dispute": ev.max_votes_per_dispute,
                "dispute_governance_registry_id": addr_to_string(&ev.dispute_governance_registry_id),
                "claim_treasury_fee_bps": ev.claim_treasury_fee_bps,
                "max_referral_bps": ev.max_referral_bps,
                "video_embedded_audio_redirect_bps": ev.video_embedded_audio_redirect_bps,
                "dispute_quorum_base_stake": ev.dispute_quorum_base_stake,
                "dispute_second_round_fee_multiplier_bps": ev.dispute_second_round_fee_multiplier_bps,
                "dispute_second_round_quorum_multiplier_bps": ev.dispute_second_round_quorum_multiplier_bps,
                "username_beneficiary_join_referral_bps": ev.username_beneficiary_join_referral_bps,
                "max_disputes_per_post": ev.max_disputes_per_post,
                "min_vault_deposit_amount": ev.min_vault_deposit_amount,
                "timestamp": ev.timestamp,
            })))
        }
        "UsernameBeneficiaryProvisionedEvent"
        | "UsernameBeneficiaryClaimedEvent"
        | "UsernameBeneficiaryEndedEvent"
        | "UsernameBeneficiaryConflictEvent"
        | "CreatorIdentityWalletLinkedEvent" => {
            return parse_poc_username_beneficiary_event(event_name, contents);
        }
        _ => Ok(None),
    };
    result
}

fn parse_poc_username_beneficiary_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "UsernameBeneficiaryProvisionedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameBeneficiaryProvisionedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "beneficiary_id": addr_to_string(&ev.beneficiary_id),
                "username": ev.username,
                "creator_identity_source": ev.creator_identity_source,
                "creator_identity_hash": format!("0x{}", hex::encode(&ev.creator_identity_hash)),
                "required_x_handle": ev.required_x_handle,
                "beneficiary_address": addr_to_string(&ev.beneficiary_address),
                "vault_id": addr_to_string(&ev.vault_id),
                "provisioned_by": addr_to_string(&ev.provisioned_by),
                "provisioned_at": ev.provisioned_at,
            })))
        }
        "UsernameBeneficiaryClaimedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameBeneficiaryClaimedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "beneficiary_id": addr_to_string(&ev.beneficiary_id),
                "username": ev.username,
                "profile_id": addr_to_string(&ev.profile_id),
                "claimed_by": addr_to_string(&ev.claimed_by),
                "wallet": addr_to_string(&ev.wallet),
                "oracle_evidence_hash": format!("0x{}", hex::encode(&ev.oracle_evidence_hash)),
                "claimed_at": ev.claimed_at,
            })))
        }
        "UsernameBeneficiaryEndedEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameBeneficiaryEndedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "beneficiary_id": addr_to_string(&ev.beneficiary_id),
                "username": ev.username,
                "ended_by": addr_to_string(&ev.ended_by),
                "end_reason_code": ev.end_reason_code,
                "swept_mys_amount": ev.swept_mys_amount,
                "ended_at": ev.ended_at,
            })))
        }
        "UsernameBeneficiaryConflictEvent" => {
            let ev = bcs::from_bytes::<BcsUsernameBeneficiaryConflictEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "username": ev.username,
                "existing_beneficiary_id": addr_to_string(&ev.existing_beneficiary_id),
                "attempted_by": addr_to_string(&ev.attempted_by),
            })))
        }
        "CreatorIdentityWalletLinkedEvent" => {
            let ev = bcs::from_bytes::<BcsCreatorIdentityWalletLinkedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "creator_identity_source": ev.creator_identity_source,
                "creator_identity_hash": format!("0x{}", hex::encode(&ev.creator_identity_hash)),
                "wallet": addr_to_string(&ev.wallet),
                "beneficiary_id": addr_to_string(&ev.beneficiary_id),
                "linked_at": ev.linked_at,
            })))
        }
        _ => Ok(None),
    }
}

fn parse_poc_vault_event(
    event_name: &str,
    contents: &[u8],
) -> Result<Option<serde_json::Value>, EventParseError> {
    match event_name {
        "PoCBeneficiaryVaultDepositEvent" => {
            let ev = bcs::from_bytes::<BcsPoCBeneficiaryVaultDepositEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "vault_id": addr_to_string(&ev.vault_id),
                "beneficiary": addr_to_string(&ev.beneficiary),
                "coin_type": bcs_move_type_name_display(&ev.coin_type),
                "amount": ev.amount,
                "source_post_id": ev.source_post_id.as_ref().map(addr_to_string),
                "timestamp": ev.timestamp,
            })))
        }
        "PoCBeneficiaryVaultClaimedEvent" => {
            let ev = bcs::from_bytes::<BcsPoCBeneficiaryVaultClaimedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "vault_id": addr_to_string(&ev.vault_id),
                "beneficiary": addr_to_string(&ev.beneficiary),
                "coin_type": bcs_move_type_name_display(&ev.coin_type),
                "referrer": ev.referrer.as_ref().map(addr_to_string),
                "treasury_amount": ev.treasury_amount,
                "referrer_amount": ev.referrer_amount,
                "beneficiary_amount": ev.beneficiary_amount,
                "join_referral_applied": ev.join_referral_applied,
                "timestamp": ev.timestamp,
            })))
        }
        _ => Ok(None),
    }
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
                "access_configuration_kind": ev.access_configuration_kind,
                "created_at": ev.created_at,
            })))
        }
        "PurchaseEvent" | "DataPurchasedEvent" => {
            if let Ok(ev) = bcs::from_bytes::<BcsPurchaseEventV2>(contents) {
                Ok(Some(serde_json::json!({
                    "ip_id": addr_to_string(&ev.ip_id), "buyer": addr_to_string(&ev.buyer),
                    "price": ev.price, "purchase_type": ev.purchase_type, "timestamp": ev.timestamp,
                    "sub_agent_id": optional_addr_json(&ev.sub_agent_id),
                    "organization_id": optional_addr_json(&ev.organization_id),
                    "platform_fee": ev.platform_fee, "ecosystem_fee": ev.ecosystem_fee,
                    "creator_amount": ev.creator_amount,
                    "platform_id": ev.platform_id.as_ref().map(addr_to_string),
                })))
            } else {
                let ev = bcs::from_bytes::<BcsPurchaseEvent>(contents)
                    .map_err(|e| bcs_parse_err(e, contents))?;
                Ok(Some(serde_json::json!({
                    "ip_id": addr_to_string(&ev.ip_id), "buyer": addr_to_string(&ev.buyer),
                    "price": ev.price, "purchase_type": ev.purchase_type, "timestamp": ev.timestamp,
                    "sub_agent_id": optional_addr_json(&ev.sub_agent_id),
                    "organization_id": optional_addr_json(&ev.organization_id),
                })))
            }
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
        "AccessRevokedEvent" | "DataAccessRevokedEvent" => {
            let ev = bcs::from_bytes::<BcsAccessRevokedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "ip_id": addr_to_string(&ev.ip_id),
                "user": addr_to_string(&ev.user),
                "access_type": ev.access_type,
                "revoked_by": addr_to_string(&ev.revoked_by),
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
            if let Ok(ev) = bcs::from_bytes::<BcsMyDataConfigUpdatedEventV2>(contents) {
                Ok(Some(serde_json::json!({
                    "updated_by": addr_to_string(&ev.updated_by), "marketplace_enabled": ev.marketplace_enabled,
                    "max_tags": ev.max_tags, "max_subscription_days": ev.max_subscription_days,
                    "max_free_access_grants": ev.max_free_access_grants,
                    "max_encryption_id_bytes": ev.max_encryption_id_bytes,
                    "max_encrypted_data_bytes": ev.max_encrypted_data_bytes, "max_tag_bytes": ev.max_tag_bytes,
                    "max_metadata_bytes": ev.max_metadata_bytes,
                    "max_payment_reference_bytes": ev.max_payment_reference_bytes,
                    "max_pool_assignments": ev.max_pool_assignments,
                    "max_merkle_proof_depth": ev.max_merkle_proof_depth,
                    "max_paid_access_entries": ev.max_paid_access_entries,
                    "default_claim_window_ms": ev.default_claim_window_ms,
                    "p2p_platform_fee_bps": ev.p2p_platform_fee_bps,
                    "p2p_ecosystem_fee_bps": ev.p2p_ecosystem_fee_bps,
                    "mydata_marketplace_platform_fee_bps": ev.mydata_marketplace_platform_fee_bps,
                    "mydata_marketplace_ecosystem_fee_bps": ev.mydata_marketplace_ecosystem_fee_bps,
                    "non_platform_platform_to_creator_bps": ev.non_platform_platform_to_creator_bps,
                    "non_platform_platform_to_treasury_bps": ev.non_platform_platform_to_treasury_bps,
                    "timestamp": ev.timestamp,
                })))
            } else {
                let ev = bcs::from_bytes::<BcsMyDataConfigUpdatedEvent>(contents)
                    .map_err(|e| bcs_parse_err(e, contents))?;
                Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "marketplace_enabled": ev.marketplace_enabled,
                "max_tags": ev.max_tags,
                "max_subscription_days": ev.max_subscription_days,
                "max_free_access_grants": ev.max_free_access_grants,
                "max_encryption_id_bytes": ev.max_encryption_id_bytes,
                "p2p_platform_fee_bps": ev.p2p_platform_fee_bps,
                "p2p_ecosystem_fee_bps": ev.p2p_ecosystem_fee_bps,
                "mydata_marketplace_platform_fee_bps": ev.mydata_marketplace_platform_fee_bps,
                "mydata_marketplace_ecosystem_fee_bps": ev.mydata_marketplace_ecosystem_fee_bps,
                "non_platform_platform_to_creator_bps": ev.non_platform_platform_to_creator_bps,
                "non_platform_platform_to_treasury_bps": ev.non_platform_platform_to_treasury_bps,
                "timestamp": ev.timestamp,
                })))
            }
        }
        "BroadPoolCreatedEvent" => {
            if let Ok(ev) = bcs::from_bytes::<BcsBroadPoolCreatedEventV2>(contents) {
                Ok(Some(serde_json::json!({
                    "pool_id": addr_to_string(&ev.pool_id), "name": ev.name,
                    "platform_id": ev.platform_id.as_ref().map(addr_to_string), "created_at": ev.created_at,
                })))
            } else {
                let ev = bcs::from_bytes::<BcsBroadPoolCreatedEvent>(contents)
                    .map_err(|e| bcs_parse_err(e, contents))?;
                Ok(Some(serde_json::json!({
                    "pool_id": addr_to_string(&ev.pool_id), "name": ev.name, "created_at": ev.created_at,
                })))
            }
        }
        "SubPoolCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsSubPoolCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "sub_pool_id": addr_to_string(&ev.sub_pool_id),
                "broad_pool_id": addr_to_string(&ev.broad_pool_id),
                "name": ev.name,
                "created_at": ev.created_at,
            })))
        }
        "MyDataAssignedToSubPoolEvent" => {
            let ev = bcs::from_bytes::<BcsMyDataAssignedToSubPoolEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            let sub_pool_ids: Vec<String> = ev.sub_pool_ids.iter().map(addr_to_string).collect();
            Ok(Some(serde_json::json!({
                "ip_id": addr_to_string(&ev.ip_id),
                "sub_pool_ids": sub_pool_ids,
                "assigned_at": ev.assigned_at,
            })))
        }
        "SnapshotAnchorRecordedEvent" => {
            if let Ok(ev) = bcs::from_bytes::<BcsSnapshotAnchorRecordedEventV3>(contents) {
                Ok(Some(serde_json::json!({
                    "snapshot_id": addr_to_string(&ev.snapshot_id),
                    "buyer_address": addr_to_string(&ev.buyer_address), "price_paid": ev.price_paid,
                    "source_pool_id": addr_to_string(&ev.source_pool_id),
                    "source_sub_pool_id": addr_to_string(&ev.source_sub_pool_id),
                    "platform_id": ev.platform_id.as_ref().map(addr_to_string), "created_at": ev.created_at,
                    "manifest_hash": format!("0x{}", hex::encode(&ev.snapshot_manifest_hash)),
                    "payment_reference": format!("0x{}", hex::encode(&ev.payment_reference)),
                })))
            } else if let Ok(ev) = bcs::from_bytes::<BcsSnapshotAnchorRecordedEventV2>(contents) {
                Ok(Some(serde_json::json!({
                    "snapshot_id": addr_to_string(&ev.snapshot_id),
                    "buyer_address": addr_to_string(&ev.buyer_address),
                    "price_paid": ev.price_paid,
                    "created_at": ev.created_at,
                    "manifest_hash": format!("0x{}", hex::encode(&ev.snapshot_manifest_hash)),
                    "payment_reference": format!("0x{}", hex::encode(&ev.payment_reference)),
                })))
            } else {
                let ev = bcs::from_bytes::<BcsSnapshotAnchorRecordedEvent>(contents)
                    .map_err(|e| bcs_parse_err(e, contents))?;
                Ok(Some(serde_json::json!({
                    "snapshot_id": addr_to_string(&ev.snapshot_id),
                    "buyer_address": addr_to_string(&ev.buyer_address),
                    "price_paid": ev.price_paid,
                    "created_at": ev.created_at,
                })))
            }
        }
        "DistributionRecordedEvent" => {
            if let Ok(ev) = bcs::from_bytes::<BcsDistributionRecordedEventV2>(contents) {
                Ok(Some(serde_json::json!({
                    "snapshot_id": addr_to_string(&ev.snapshot_id), "total_amount": ev.total_amount,
                    "contributor_count": ev.contributor_count,
                    "merkle_root": format!("0x{}", hex::encode(&ev.merkle_root)),
                    "platform_id": ev.platform_id.as_ref().map(addr_to_string),
                    "claim_deadline_ms": ev.claim_deadline_ms, "published_at": ev.published_at,
                })))
            } else {
                let ev = bcs::from_bytes::<BcsDistributionRecordedEvent>(contents)
                    .map_err(|e| bcs_parse_err(e, contents))?;
                Ok(Some(serde_json::json!({
                "snapshot_id": addr_to_string(&ev.snapshot_id),
                "total_amount": ev.total_amount,
                "contributor_count": ev.contributor_count,
                "merkle_root": format!("0x{}", hex::encode(&ev.merkle_root)),
                "published_at": ev.published_at,
                })))
            }
        }
        "MerkleRootPublishedEvent" => {
            let ev = bcs::from_bytes::<BcsMerkleRootPublishedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "snapshot_id": addr_to_string(&ev.snapshot_id),
                "root_hash": format!("0x{}", hex::encode(&ev.root_hash)),
                "published_at": ev.published_at,
            })))
        }
        "ClaimExecutedEvent" => {
            if let Ok(ev) = bcs::from_bytes::<BcsClaimExecutedEventV2>(contents) {
                Ok(Some(serde_json::json!({
                    "snapshot_id": addr_to_string(&ev.snapshot_id), "claimant": addr_to_string(&ev.claimant),
                    "gross_amount": ev.gross_amount, "platform_fee": ev.platform_fee,
                    "ecosystem_fee": ev.ecosystem_fee, "net_amount": ev.net_amount,
                    "platform_id": ev.platform_id.as_ref().map(addr_to_string), "claimed_at": ev.claimed_at,
                })))
            } else {
                let ev = bcs::from_bytes::<BcsClaimExecutedEvent>(contents)
                    .map_err(|e| bcs_parse_err(e, contents))?;
                Ok(Some(serde_json::json!({
                "snapshot_id": addr_to_string(&ev.snapshot_id),
                "claimant": addr_to_string(&ev.claimant),
                "amount": ev.amount,
                "claimed_at": ev.claimed_at,
                })))
            }
        }
        "SnapshotEscrowFundedEvent" => {
            let ev = bcs::from_bytes::<BcsSnapshotEscrowFundedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "snapshot_id": addr_to_string(&ev.snapshot_id), "funder": addr_to_string(&ev.funder),
                "amount": ev.amount, "total_funded": ev.total_funded, "funded_at": ev.funded_at,
            })))
        }
        "SnapshotEscrowReclaimedEvent" => {
            let ev = bcs::from_bytes::<BcsSnapshotEscrowReclaimedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "snapshot_id": addr_to_string(&ev.snapshot_id),
                "buyer_address": addr_to_string(&ev.buyer_address), "amount": ev.amount,
                "reclaimed_at": ev.reclaimed_at,
            })))
        }
        "MyDataPricingUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsMyDataPricingUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "ip_id": addr_to_string(&ev.ip_id), "one_time_price": ev.one_time_price,
                "subscription_price": ev.subscription_price,
                "subscription_duration_days": ev.subscription_duration_days,
                "updated_by": addr_to_string(&ev.updated_by), "timestamp": ev.timestamp,
            })))
        }
        "MyDataContentUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsMyDataContentUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "ip_id": addr_to_string(&ev.ip_id),
                "encrypted_data_updated": ev.encrypted_data_updated, "tags_updated": ev.tags_updated,
                "updated_by": addr_to_string(&ev.updated_by), "timestamp": ev.timestamp,
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
                "insurance_enabled": ev.insurance_enabled,
                "min_coverage_bps": ev.min_coverage_bps,
                "max_coverage_bps": ev.max_coverage_bps,
                "max_duration_ms": ev.max_duration_ms,
                "fee_bps": ev.fee_bps,
                "odds_base_bps": ev.odds_base_bps,
                "timestamp": ev.timestamp,
            })))
        }
        "RouterConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsRouterConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "paused": ev.paused,
                "max_route_reserve_market": ev.max_route_reserve_market,
                "max_route_reserve_user": ev.max_route_reserve_user,
                "max_route_reserve_option": ev.max_route_reserve_option,
                "max_vault_concentration_bps": ev.max_vault_concentration_bps,
                "min_vault_health_factor_bps": ev.min_vault_health_factor_bps,
                "max_route_legs": ev.max_route_legs,
                "timestamp": ev.timestamp,
            })))
        }
        "RiskPricingConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsRiskPricingConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "min_spot_total_liquidity": ev.min_spot_total_liquidity,
                "max_coverage_fraction_of_option_bps": ev.max_coverage_fraction_of_option_bps,
                "max_risk_multiplier_bps": ev.max_risk_multiplier_bps,
                "min_premium_amount": ev.min_premium_amount,
                "spot_smoothing_per_option": ev.spot_smoothing_per_option,
                "implied_prob_floor_bps": ev.implied_prob_floor_bps,
                "odds_floor_1x": ev.odds_floor_1x,
                "odds_cap_bps": ev.odds_cap_bps,
                "liq_cap_bps": ev.liq_cap_bps,
                "liq_ref_amount": ev.liq_ref_amount,
                "exposure_cap_bps": ev.exposure_cap_bps,
                "exposure_k_bps": ev.exposure_k_bps,
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
                "max_exposure_per_option": ev.max_exposure_per_option,
                "enabled": ev.enabled,
                "paused": ev.paused,
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
            let route_id = ev
                .route_id
                .as_ref()
                .map(addr_to_string)
                .map(serde_json::Value::from);
            Ok(Some(serde_json::json!({
                "policy_id": addr_to_string(&ev.policy_id),
                "vault_id": addr_to_string(&ev.vault_id),
                "market_id": addr_to_string(&ev.market_id),
                "insured": addr_to_string(&ev.insured),
                "option_id": ev.option_id,
                "covered_amount": ev.covered_amount,
                "coverage_bps": ev.coverage_bps,
                "premium_paid": ev.premium_paid,
                "premium_raw": ev.premium_raw,
                "reserve_locked": ev.reserve_locked,
                "expiry_time_ms": ev.expiry_time_ms,
                "implied_probability_bps": ev.implied_probability_bps,
                "risk_multiplier_bps": ev.risk_multiplier_bps,
                "base_premium": ev.base_premium,
                "market_total_amount": ev.market_total_amount,
                "option_escrow_amount": ev.option_amount,
                "backstop_sweep_amount": ev.backstop_sweep_amount,
                "route_id": route_id,
                "route_leg_index": ev.route_leg_index,
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
        "VaultStatusUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsVaultStatusUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "vault_id": addr_to_string(&ev.vault_id),
                "enabled": ev.enabled,
                "paused": ev.paused,
                "max_exposure_per_option": ev.max_exposure_per_option,
                "max_exposure_per_market": ev.max_exposure_per_market,
                "max_exposure_per_user": ev.max_exposure_per_user,
                "base_rate_bps_per_day": ev.base_rate_bps_per_day,
                "utilization_multiplier_bps": ev.utilization_multiplier_bps,
                "updated_by": addr_to_string(&ev.updated_by),
                "timestamp_ms": ev.timestamp_ms,
            })))
        }
        "CoverageRoutedEvent" => {
            let ev = bcs::from_bytes::<BcsCoverageRoutedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            let policy_ids: Vec<String> = ev.policy_ids.iter().map(addr_to_string).collect();
            let vault_ids: Vec<String> = ev.vault_ids.iter().map(addr_to_string).collect();
            Ok(Some(serde_json::json!({
                "route_id": addr_to_string(&ev.route_id),
                "insured": addr_to_string(&ev.insured),
                "market_id": addr_to_string(&ev.market_id),
                "option_id": ev.option_id,
                "coverage_bps": ev.coverage_bps,
                "duration_ms": ev.duration_ms,
                "total_covered": ev.total_covered,
                "total_premium": ev.total_premium,
                "total_reserve": ev.total_reserve,
                "total_backstop_sweep": ev.total_backstop_sweep,
                "expiry_time_ms": ev.expiry_time_ms,
                "policy_ids": policy_ids,
                "vault_ids": vault_ids,
            })))
        }
        "RouteFillEvent" => {
            let ev = bcs::from_bytes::<BcsRouteFillEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "route_id": addr_to_string(&ev.route_id),
                "leg_index": ev.leg_index,
                "vault_id": addr_to_string(&ev.vault_id),
                "policy_id": addr_to_string(&ev.policy_id),
                "covered_amount": ev.covered_amount,
                "premium_paid": ev.premium_paid,
                "reserve_locked": ev.reserve_locked,
                "backstop_sweep_amount": ev.backstop_sweep_amount,
            })))
        }
        "BackstopUsedEvent" => {
            let ev = bcs::from_bytes::<BcsBackstopUsedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "market_id": addr_to_string(&ev.market_id),
                "recipient": addr_to_string(&ev.recipient),
                "amount": ev.amount,
                "total_paid_out_after": ev.total_paid_out_after,
                "tail_mode_enabled": ev.tail_mode_enabled,
            })))
        }
        "BackstopTreasuryDepositEvent" => {
            let ev = bcs::from_bytes::<BcsBackstopTreasuryDepositEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "depositor": addr_to_string(&ev.depositor),
                "amount": ev.amount,
                "new_balance": ev.new_balance,
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
                "market_id": addr_to_string(&ev.market_id),
                "user": addr_to_string(&ev.user),
                "option_id": ev.option_id,
                "amount": ev.amount,
                "timestamp_ms": ev.timestamp_ms,
                "referrer_post_id": ev.referrer_post_id.as_ref().map(addr_to_string),
            })))
        }
        "SpotResolvedEvent" | "ResolvedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotResolvedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "market_id": addr_to_string(&ev.market_id),
                "claim_id": addr_to_string(&ev.claim_id),
                "outcome": ev.outcome,
                "total_escrow": ev.total_escrow,
                "fee_taken": ev.fee_taken,
                "creator_fee_total": ev.creator_fee_total,
                "reasoning": ev.reasoning,
                "evidence_urls": ev.evidence_urls,
            })))
        }
        "SpotDaoRequiredEvent" | "DaoRequiredEvent" => {
            let ev = bcs::from_bytes::<BcsSpotDaoRequiredEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "spot_record_id": addr_to_string(&ev.spot_record_id),
                "confidence_bps": ev.confidence_bps,
                "oracle_proposed_outcome": ev.oracle_proposed_outcome,
                "dao_escalated_at_ms": ev.dao_escalated_at_ms,
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
                "truth_enabled": ev.truth_enabled,
                "confidence_threshold_bps": ev.confidence_threshold_bps,
                "resolution_window_ms": ev.resolution_window_ms,
                "max_resolution_window_ms": ev.max_resolution_window_ms,
                "payout_delay_ms": ev.payout_delay_ms,
                "platform_fee_bps": ev.platform_fee_bps,
                "ecosystem_fee_bps": ev.ecosystem_fee_bps,
                "creator_fee_bps": ev.creator_fee_bps,
                "creator_claim_window_ms": ev.creator_claim_window_ms,
                "expired_creator_ecosystem_bps": ev.expired_creator_ecosystem_bps,
                "min_betting_options": ev.min_betting_options,
                "max_betting_options": ev.max_betting_options,
                "min_reasoning_length": ev.min_reasoning_length,
                "max_reasoning_length": ev.max_reasoning_length,
                "max_evidence_urls": ev.max_evidence_urls,
                "oracle_address": addr_to_string(&ev.oracle_address),
                "max_single_bet": ev.max_single_bet,
                "max_bets_per_record": ev.max_bets_per_record,
                "max_claim_per_post": ev.max_claim_per_post,
                "spot_governance_registry_id": addr_to_string(&ev.spot_governance_registry_id),
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
                "created_at_ms": ev.created_at_ms,
                "betting_options": ev.betting_options,
                "resolution_window_ms": ev.resolution_window_ms,
                "max_resolution_window_ms": ev.max_resolution_window_ms,
            })))
        }
        "SpotGovernanceProposalLinkedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotGovernanceProposalLinkedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "spot_record_id": addr_to_string(&ev.spot_record_id),
                "proposal_id": addr_to_string(&ev.proposal_id),
                "proposed_outcome": ev.proposed_outcome,
            })))
        }
        "SpotGovernanceProposalClearedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotGovernanceProposalClearedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "spot_record_id": addr_to_string(&ev.spot_record_id),
                "proposal_id": addr_to_string(&ev.proposal_id),
            })))
        }
        "SpotClaimCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotClaimCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "claim_id": addr_to_string(&ev.claim_id),
                "semantic_claim_hash": format!("0x{}", hex::encode(&ev.semantic_claim_hash)),
                "created_at_ms": ev.created_at_ms,
            })))
        }
        "SpotMarketCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotMarketCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "market_id": addr_to_string(&ev.market_id),
                "claim_id": addr_to_string(&ev.claim_id),
                "market_key_hash": format!("0x{}", hex::encode(&ev.market_key_hash)),
                "primary_post_id": addr_to_string(&ev.primary_post_id),
                "claim_index": ev.claim_index,
                "resolution_policy_hash": format!("0x{}", hex::encode(&ev.resolution_policy_hash)),
                "created_at_ms": ev.created_at_ms,
                "betting_options": ev.betting_options,
                "resolution_at_ms": ev.resolution_at_ms,
                "max_resolution_window_ms": ev.max_resolution_window_ms,
            })))
        }
        "SpotPostLinkedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotPostLinkedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "claim_id": addr_to_string(&ev.claim_id),
                "market_id": ev.market_id.as_ref().map(addr_to_string),
                "claim_index": ev.claim_index,
                "policy_hash": format!("0x{}", hex::encode(&ev.policy_hash)),
            })))
        }
        "SpotClaimsFinalizedForPost" => {
            let ev = bcs::from_bytes::<BcsSpotClaimsFinalizedForPost>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "post_id": addr_to_string(&ev.post_id),
                "status": ev.status,
                "detected_claim_count": ev.detected_claim_count,
                "rejected_claim_count": ev.rejected_claim_count,
                "truncated_claim_count": ev.truncated_claim_count,
                "future_accepted_count": ev.future_accepted_count,
                "past_verified_count": ev.past_verified_count,
                "max_claim_per_post_applied": ev.max_claim_per_post_applied,
                "claim_manifest_hash": ev.claim_manifest_hash.as_ref().map(|h| format!("0x{}", hex::encode(h))),
                "veracity_manifest_hash": ev.veracity_manifest_hash.as_ref().map(|h| format!("0x{}", hex::encode(h))),
                "future_claim_indexes": ev.future_claim_indexes,
                "future_claim_ids": ev.future_claim_ids.iter().map(addr_to_string).collect::<Vec<_>>(),
                "future_market_ids": ev.future_market_ids.iter().map(addr_to_string).collect::<Vec<_>>(),
                "past_claim_indexes": ev.past_claim_indexes,
                "past_verdicts": ev.past_verdicts,
                "past_related_market_ids": ev.past_related_market_ids.iter().map(addr_to_string).collect::<Vec<_>>(),
                "past_evidence_hashes": ev.past_evidence_hashes.iter().map(|h| format!("0x{}", hex::encode(h))).collect::<Vec<_>>(),
                "finalized_at_ms": ev.finalized_at_ms,
            })))
        }
        "SpotCreatorPayoutAccruedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotCreatorPayoutAccruedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "market_id": addr_to_string(&ev.market_id),
                "payout_id": ev.payout_id,
                "creator": addr_to_string(&ev.creator),
                "referrer_post_id": addr_to_string(&ev.referrer_post_id),
                "amount": ev.amount,
                "expires_at_ms": ev.expires_at_ms,
            })))
        }
        "SpotCreatorPayoutClaimedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotCreatorPayoutClaimedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "market_id": addr_to_string(&ev.market_id),
                "payout_id": ev.payout_id,
                "creator": addr_to_string(&ev.creator),
                "amount": ev.amount,
            })))
        }
        "SpotCreatorPayoutReclaimedEvent" => {
            let ev = bcs::from_bytes::<BcsSpotCreatorPayoutReclaimedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "market_id": addr_to_string(&ev.market_id),
                "payout_id": ev.payout_id,
                "ecosystem_amount": ev.ecosystem_amount,
                "platform_amount": ev.platform_amount,
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
                "base_price": ev.base_price,
                "quadratic_coefficient": ev.quadratic_coefficient,
                "circulating_supply": ev.circulating_supply,
                "total_reserved_at_launch": ev.total_reserved_at_launch,
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
        "TokenSwappedEvent" | "SwapEvent" => {
            let ev = bcs::from_bytes::<BcsTokenSwappedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "source_pool_id": addr_to_string(&ev.source_pool_id),
                "dest_pool_id": addr_to_string(&ev.dest_pool_id),
                "trader": addr_to_string(&ev.trader),
                "sell_amount": ev.sell_amount,
                "dest_amount": ev.dest_amount,
                "sell_myso_gross": ev.sell_myso_gross,
                "buy_myso_gross": ev.buy_myso_gross,
                "sell_fee_amount": ev.sell_fee_amount,
                "buy_fee_amount": ev.buy_fee_amount,
                "sell_creator_fee": ev.sell_creator_fee,
                "sell_platform_fee": ev.sell_platform_fee,
                "sell_treasury_fee": ev.sell_treasury_fee,
                "buy_creator_fee": ev.buy_creator_fee,
                "buy_platform_fee": ev.buy_platform_fee,
                "buy_treasury_fee": ev.buy_treasury_fee,
                "leftover_myso": ev.leftover_myso,
                "source_new_price": ev.source_new_price,
                "dest_new_price": ev.dest_new_price,
            })))
        }
        "TokenTransferredEvent" | "TransferEvent" => {
            let ev = bcs::from_bytes::<BcsTokenTransferredEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "pool_id": addr_to_string(&ev.pool_id),
                "from": addr_to_string(&ev.from),
                "to": addr_to_string(&ev.to),
                "amount": ev.amount,
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
                "non_platform_platform_to_creator_bps": ev.non_platform_platform_to_creator_bps,
                "non_platform_platform_to_treasury_bps": ev.non_platform_platform_to_treasury_bps,
                "trading_enabled": ev.trading_enabled,
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
            let (
                pool_id,
                post_id,
                redirect_to,
                redirect_percentage,
                poc_redirection_kind,
                updated_by,
                timestamp,
            ) = match bcs::from_bytes::<BcsPocRedirectionUpdatedEvent>(contents) {
                Ok(ev) => (
                    ev.pool_id,
                    ev.post_id,
                    ev.redirect_to,
                    ev.redirect_percentage,
                    ev.poc_redirection_kind,
                    ev.updated_by,
                    ev.timestamp,
                ),
                Err(e_v2) => {
                    let ev = bcs::from_bytes::<BcsPocRedirectionUpdatedEventV1>(contents).map_err(
                        |e_v1| EventParseError {
                            error: format!("PocRedirection BCS v2: {e_v2}; v1: {e_v1}"),
                            contents: contents.to_vec(),
                        },
                    )?;
                    let kind = if ev.redirect_to.is_some() && ev.redirect_percentage.is_some() {
                        1u8
                    } else {
                        0u8
                    };
                    (
                        ev.pool_id,
                        ev.post_id,
                        ev.redirect_to,
                        ev.redirect_percentage,
                        kind,
                        ev.updated_by,
                        ev.timestamp,
                    )
                }
            };
            Ok(Some(serde_json::json!({
                "pool_id": addr_to_string(&pool_id),
                "post_id": addr_to_string(&post_id),
                "redirect_to": redirect_to.as_ref().map(addr_to_string),
                "redirect_percentage": redirect_percentage,
                "poc_redirection_kind": poc_redirection_kind,
                "updated_by": addr_to_string(&updated_by),
                "timestamp": timestamp,
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
                "profile_id": move_object_id_to_string(&BcsMoveObjectId { bytes: ev.profile_id }),
                "created_at": ev.created_at,
            })))
        }
        "SubscriptionPlanCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsSubscriptionPlanCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "service_id": addr_to_string(&ev.service_id),
                "plan_id": move_object_id_to_string(&BcsMoveObjectId { bytes: ev.plan_id }),
                "title": ev.title,
                "description": ev.description,
                "price": ev.price,
                "duration_ms": ev.duration_ms,
                "tier_level": ev.tier_level,
                "platform_id": ev.platform_id.as_ref().map(addr_to_string),
                "created_at": ev.created_at,
            })))
        }
        "SubscriptionPlanUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsSubscriptionPlanUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "service_id": addr_to_string(&ev.service_id),
                "plan_id": move_object_id_to_string(&BcsMoveObjectId { bytes: ev.plan_id }),
                "title": ev.title,
                "description": ev.description,
                "price": ev.price,
                "duration_ms": ev.duration_ms,
                "tier_level": ev.tier_level,
                "platform_id": ev.platform_id.as_ref().map(addr_to_string),
                "active": ev.active,
                "updated_by": addr_to_string(&ev.updated_by),
                "updated_at": ev.updated_at,
            })))
        }
        "SubscriptionPlanDeactivatedEvent" => {
            let ev = bcs::from_bytes::<BcsSubscriptionPlanDeactivatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "service_id": addr_to_string(&ev.service_id),
                "plan_id": move_object_id_to_string(&BcsMoveObjectId { bytes: ev.plan_id }),
                "deactivated_at": ev.deactivated_at,
            })))
        }
        "ProfileSubscriptionCreatedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileSubscriptionCreatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "subscription_id": move_object_id_to_string(&BcsMoveObjectId { bytes: ev.subscription_id }),
                "service_id": addr_to_string(&ev.service_id),
                "plan_id": move_object_id_to_string(&BcsMoveObjectId { bytes: ev.plan_id }),
                "subscriber": addr_to_string(&ev.subscriber),
                "expires_at": ev.expires_at,
                "price": ev.price,
                "duration_ms": ev.duration_ms,
                "tier_level": ev.tier_level,
                "platform_id": ev.platform_id.as_ref().map(addr_to_string),
                "auto_renew": ev.auto_renew,
                "platform_fee": ev.platform_fee,
                "ecosystem_fee": ev.ecosystem_fee,
                "creator_amount": ev.creator_amount,
                "payment_platform_id": ev.payment_platform_id.as_ref().map(addr_to_string),
            })))
        }
        "ProfileSubscriptionRenewedEvent" => {
            let ev = bcs::from_bytes::<BcsProfileSubscriptionRenewedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "subscription_id": addr_to_string(&ev.subscription_id),
                "subscriber": addr_to_string(&ev.subscriber),
                "plan_id": move_object_id_to_string(&BcsMoveObjectId { bytes: ev.plan_id }),
                "new_expires_at": ev.new_expires_at,
                "renewal_count": ev.renewal_count,
                "auto_renewed": ev.auto_renewed,
                "price": ev.price,
                "duration_ms": ev.duration_ms,
                "tier_level": ev.tier_level,
                "platform_id": ev.platform_id.as_ref().map(addr_to_string),
                "platform_fee": ev.platform_fee,
                "ecosystem_fee": ev.ecosystem_fee,
                "creator_amount": ev.creator_amount,
                "payment_platform_id": ev.payment_platform_id.as_ref().map(addr_to_string),
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
        "SubscriptionConfigUpdatedEvent" => {
            let ev = bcs::from_bytes::<BcsSubscriptionConfigUpdatedEvent>(contents)
                .map_err(|e| bcs_parse_err(e, contents))?;
            Ok(Some(serde_json::json!({
                "updated_by": addr_to_string(&ev.updated_by),
                "default_billing_period_ms": ev.default_billing_period_ms,
                "max_renewal_months": ev.max_renewal_months,
                "platform_fee_bps": ev.platform_fee_bps,
                "ecosystem_fee_bps": ev.ecosystem_fee_bps,
                "non_platform_platform_to_creator_bps": ev.non_platform_platform_to_creator_bps,
                "non_platform_platform_to_treasury_bps": ev.non_platform_platform_to_treasury_bps,
                "timestamp": ev.timestamp,
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
pub(crate) fn username_beneficiary_claimed_bcs_fixture() -> Vec<u8> {
    use move_core_types::account_address::AccountAddress;

    let ev = BcsUsernameBeneficiaryClaimedEvent {
        beneficiary_id: AccountAddress::from_hex_literal(
            "0x19e12c82effb103ed5a762f7d5c3daa0d7ed96b1d421ba686734de3e897ce939",
        )
        .unwrap(),
        username: "pocub1782775058".to_string(),
        profile_id: AccountAddress::from_hex_literal(
            "0x3853b739126a0e3773c415eab400b4adc26a4257d58abf08be12992e0d0ee48f",
        )
        .unwrap(),
        claimed_by: AccountAddress::from_hex_literal(
            "0x7eb74c2ca45c41a4c4126f13c2286cbc9ac400c7b5ab5fe38694ecd71161ccaf",
        )
        .unwrap(),
        wallet: AccountAddress::from_hex_literal(
            "0x7e91c216898618e1c5f614a01dde30b5f5d7e1e2fb4fdfe0b4a3423d55202430",
        )
        .unwrap(),
        oracle_evidence_hash: vec![],
        claimed_at: 5_837_000_000,
    };
    bcs::to_bytes(&ev).expect("bcs")
}

#[cfg(test)]
mod tests {
    use super::*;
    use move_core_types::account_address::AccountAddress;

    pub(crate) fn brandon_profile_created_bcs(created_at: u64) -> Vec<u8> {
        let profile_id = AccountAddress::from_hex_literal(
            "0xd988a8c1f1262d0aa7ab581a78b957fa97cbf53db4d27af2ee7006247a",
        )
        .unwrap();
        let owner = AccountAddress::from_hex_literal(
            "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
        )
        .unwrap();
        bcs::to_bytes(&BcsProfileCreatedEvent {
            profile_id,
            display_name: "Brandon Shaw".to_string(),
            bio: "Web8 developer and crypto enthusiast".to_string(),
            profile_picture: Some("https://example.com/profile.jpg".to_string()),
            cover_photo: Some("https://example.com/cover.png".to_string()),
            owner,
            created_at,
        })
        .expect("serialize ProfileCreatedEvent")
    }

    #[test]
    fn ai_credit_usage_settled_bcs_roundtrip() {
        let balance_id = AccountAddress::from_hex_literal(
            "0x2f41b4f43f505d427e8777c511461de8e50eac26558a996627dded27dce50918",
        )
        .unwrap();
        let agent_object_id = AccountAddress::from_hex_literal(
            "0x124043762fbf1db8d8ba247c69a66e71702bebfc4f22ac5663a9b089bde73620",
        )
        .unwrap();
        let ev = BcsAiCreditUsageSettledEvent {
            balance_id,
            agent_object_id,
            receipt_id: 132625655239685005677817396617643760670,
            amount_mist: 222222223,
            usage_kind: 1,
            settlement_nonce: 1,
            remaining_mist: 4_677_777_777,
        };
        let contents = bcs::to_bytes(&ev).expect("serialize AiCreditUsageSettled");
        let json = parse_event_contents("ai_credit", "AiCreditUsageSettled", &contents)
            .expect("parse AiCreditUsageSettled");
        assert_eq!(
            json["receipt_id"],
            "132625655239685005677817396617643760670"
        );
        assert_eq!(json["settlement_nonce"], 1);
        assert_eq!(json["remaining_mist"], 4_677_777_777_i64);
    }

    #[test]
    fn ai_spend_reservation_events_bcs_roundtrip() {
        let balance_id = AccountAddress::from_hex_literal("0x1").unwrap();
        let agent_object_id = AccountAddress::from_hex_literal("0x2").unwrap();
        let reserved = BcsAiSpendReservedEvent {
            balance_id,
            agent_object_id,
            reservation_nonce: 7,
            max_amount_mist: 900,
            provider_envelope_hash: vec![0x11; 32],
            request_hash: vec![0x22; 32],
            fx_quote_id: b"quote-7".to_vec(),
            myso_usd_e8: 450_000,
            markup_bps: 1_500,
            capture_deadline_ms: 10_000,
            hard_expiry_ms: 20_000,
            available_mist: 9_100,
        };
        let json = parse_event_contents(
            "ai_credit",
            "AiSpendReserved",
            &bcs::to_bytes(&reserved).unwrap(),
        )
        .unwrap();
        assert_eq!(json["reservation_nonce"], 7);
        assert_eq!(json["provider_envelope_hash_hex"], "11".repeat(32));
        assert_eq!(json["fx_quote_id_hex"], hex::encode("quote-7"));

        let captured = BcsAiSpendCapturedEvent {
            balance_id,
            agent_object_id,
            reservation_nonce: 7,
            reserved_mist: 900,
            captured_mist: 750,
            released_mist: 150,
            provider_cost_usd_micros: 3_000,
            provider_generation_hash: vec![0x33; 32],
            fx_quote_id: b"quote-7".to_vec(),
            myso_usd_e8: 450_000,
            markup_bps: 1_500,
            captured_at_ms: 14_000,
            remaining_mist: 9_250,
            available_mist: 9_250,
        };
        let json = parse_event_contents(
            "ai_credit",
            "AiSpendCaptured",
            &bcs::to_bytes(&captured).unwrap(),
        )
        .unwrap();
        assert_eq!(json["captured_mist"], 750);
        assert_eq!(json["provider_cost_usd_micros"], 3_000);
        assert_eq!(json["provider_generation_hash_hex"], "33".repeat(32));

        let cancelled = BcsAiSpendCancelledEvent {
            balance_id,
            agent_object_id,
            reservation_nonce: 8,
            released_mist: 400,
            cancelled_at_ms: 15_000,
            available_mist: 9_650,
        };
        let json = parse_event_contents(
            "ai_credit",
            "AiSpendCancelled",
            &bcs::to_bytes(&cancelled).unwrap(),
        )
        .unwrap();
        assert_eq!(json["cancelled_at_ms"], 15_000);

        let expired = BcsAiSpendExpiredEvent {
            balance_id,
            agent_object_id,
            reservation_nonce: 9,
            released_mist: 500,
            expired_at_ms: 25_000,
            available_mist: 10_150,
        };
        let json = parse_event_contents(
            "ai_credit",
            "AiSpendExpired",
            &bcs::to_bytes(&expired).unwrap(),
        )
        .unwrap();
        assert_eq!(json["expired_at_ms"], 25_000);
    }

    #[test]
    fn test_parse_profile_created_event() {
        let contents = brandon_profile_created_bcs(5);
        let result = parse_event_contents("profile", "ProfileCreatedEvent", &contents);
        assert!(
            result.is_ok(),
            "parse_event_contents should succeed for ProfileCreatedEvent"
        );
        let json = result.unwrap();
        assert_eq!(json["display_name"], "Brandon Shaw");
        assert!(json.get("username").is_none());
        assert_eq!(json["bio"], "Web8 developer and crypto enthusiast");
        assert_eq!(json["created_at"], 5);
        assert!(json["owner_address"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn test_parse_profile_created_event_user_bytes() {
        let contents = brandon_profile_created_bcs(8);
        let result = parse_event_contents("profile", "ProfileCreatedEvent", &contents);
        assert!(
            result.is_ok(),
            "parse_event_contents should succeed for user's ProfileCreatedEvent bytes"
        );
        let json = result.unwrap();
        assert_eq!(json["display_name"], "Brandon Shaw");
        assert!(json.get("username").is_none());
        assert_eq!(json["bio"], "Web8 developer and crypto enthusiast");
        assert_eq!(json["created_at"], 8);
        assert!(json["owner_address"].as_str().unwrap().starts_with("0x"));
    }

    /// Layout matches on-chain `UnderwriterVaultCreatedEvent` from `insurance::create_vault`
    /// (BCS: two addresses + five u64 + two bools).
    #[test]
    fn underwriter_vault_created_event_bcs_matches_create_vault_tx_layout() {
        let vault_id = AccountAddress::from_hex_literal(
            "0xdabe953127e770c6abb207607652f5b0fdbba3d93f8c3125ba4c7b80a0d5f399",
        )
        .unwrap();
        let underwriter = AccountAddress::from_hex_literal(
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
        )
        .unwrap();
        let ev = BcsUnderwriterVaultCreatedEvent {
            vault_id,
            underwriter,
            base_rate_bps_per_day: 25,
            utilization_multiplier_bps: 5000,
            max_exposure_per_market: 0,
            max_exposure_per_user: 0,
            max_exposure_per_option: 0,
            enabled: true,
            paused: false,
        };
        let contents = bcs::to_bytes(&ev).expect("BCS serialize UnderwriterVaultCreatedEvent");
        assert_eq!(contents.len(), 106, "expected 32+32+40+2 bytes");
        let json = parse_event_contents("insurance", "UnderwriterVaultCreatedEvent", &contents)
            .expect("parse UnderwriterVaultCreatedEvent");
        assert_eq!(
            json["vault_id"].as_str().unwrap(),
            "0xdabe953127e770c6abb207607652f5b0fdbba3d93f8c3125ba4c7b80a0d5f399"
        );
        assert_eq!(json["base_rate_bps_per_day"], 25);
        assert_eq!(json["utilization_multiplier_bps"], 5000);
        assert_eq!(json["enabled"], true);
        assert_eq!(json["paused"], false);

        let rows = crate::handlers::insurance::handle_insurance_event(
            "UnderwriterVaultCreatedEvent",
            &json,
            "DBxT2TDCTHGmqhE3HT5wBkGWFwnzJM4aRazBywmX1BgQ:0",
            0,
        )
        .expect("handler must accept parsed JSON");
        assert!(
            rows.iter()
                .any(|r| matches!(r, crate::handlers::SocialEventRow::InsuranceVault(_))),
            "expected InsuranceVault row: {:?}",
            rows
        );
    }

    #[test]
    fn router_config_updated_event_bcs_roundtrip_and_handler() {
        let updated_by = AccountAddress::from_hex_literal(
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
        )
        .unwrap();
        let ev = BcsRouterConfigUpdatedEvent {
            updated_by,
            paused: false,
            max_route_reserve_market: 1_000_000_000,
            max_route_reserve_user: 2_000_000_000,
            max_route_reserve_option: 3_000_000_000,
            max_vault_concentration_bps: 10_000,
            min_vault_health_factor_bps: 10_000,
            max_route_legs: 4,
            timestamp: 1_700_000_000_000,
        };
        let contents = bcs::to_bytes(&ev).expect("BCS serialize RouterConfigUpdatedEvent");
        let json = parse_event_contents("insurance", "RouterConfigUpdatedEvent", &contents)
            .expect("parse RouterConfigUpdatedEvent");
        assert_eq!(json["paused"], false);
        assert_eq!(json["max_route_legs"], 4);
        assert!(!json.get("router_enabled").is_some());
        assert!(!json.get("router_paused").is_some());

        let rows = crate::handlers::insurance::handle_insurance_event(
            "RouterConfigUpdatedEvent",
            &json,
            "DBxT2TDCTHGmqhE3HT5wBkGWFwnzJM4aRazBywmX1BgQ:1",
            1_700_000_000_000,
        )
        .expect("handler must accept parsed JSON");
        assert!(
            rows.iter().any(|r| {
                matches!(
                    r,
                    crate::handlers::SocialEventRow::InsuranceRouterConfig(c) if !c.paused
                        && c.max_route_legs == 4
                )
            }),
            "expected InsuranceRouterConfig row: {:?}",
            rows
        );
    }

    #[test]
    fn token_pool_created_extended_bcs_carries_circulating() {
        let id = AccountAddress::from_hex_literal("0x1").unwrap();
        let owner = AccountAddress::from_hex_literal("0x2").unwrap();
        let associated_id = AccountAddress::from_hex_literal("0x3").unwrap();
        let ev = BcsTokenPoolCreatedEvent {
            id,
            token_type: 1,
            owner,
            associated_id,
            base_price: 10,
            quadratic_coefficient: 2,
            circulating_supply: 999,
            total_reserved_at_launch: 5000,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs serialize TokenPoolCreatedEvent");
        let json = parse_spt_event("TokenPoolCreatedEvent", &bytes)
            .expect("parse_spt_event ok")
            .expect("event json");
        assert_eq!(json["circulating_supply"], 999);
        assert_eq!(json["total_reserved_at_launch"], 5000);
    }

    #[test]
    fn test_bcs_option_string_encoding() {
        use move_core_types::account_address::AccountAddress;
        let addr = AccountAddress::from_hex_literal("0x50c1").unwrap();
        let ev = BcsProfileCreatedEvent {
            profile_id: addr,
            display_name: "Test".to_string(),
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
        let contents = brandon_profile_created_bcs(1);
        let result = parse_event_contents("profile", "ProfileCreatedEvent", &contents);
        assert!(
            result.is_ok(),
            "parse_event_contents should succeed for live ProfileCreatedEvent bytes"
        );
        let json = result.unwrap();
        assert_eq!(json["display_name"], "Brandon Shaw");
        assert!(json.get("username").is_none());
        assert_eq!(json["bio"], "Web8 developer and crypto enthusiast");
        assert_eq!(json["profile_picture"], "https://example.com/profile.jpg");
        assert_eq!(json["cover_photo"], "https://example.com/cover.png");
    }

    #[test]
    fn test_bcs_profile_created_event_round_trip() {
        let contents = brandon_profile_created_bcs(5);
        let ev: BcsProfileCreatedEvent =
            bcs::from_bytes(&contents).expect("fixture bytes should deserialize");
        let serialized = bcs::to_bytes(&ev).expect("BcsProfileCreatedEvent should serialize");
        let ev_round_trip: BcsProfileCreatedEvent =
            bcs::from_bytes(&serialized).expect("round-trip bytes should deserialize");
        assert_eq!(ev.created_at, ev_round_trip.created_at);
        assert_eq!(ev.display_name, ev_round_trip.display_name);
        assert_eq!(ev.bio, ev_round_trip.bio);
        assert_eq!(ev.profile_picture, ev_round_trip.profile_picture);
        assert_eq!(ev.cover_photo, ev_round_trip.cover_photo);
    }

    #[test]
    fn test_parse_post_created_event_json_fallback() {
        let json = r#"{"post_id":"0x123","owner":"0x456","profile_id":"0x789","content":"hello","post_type":"post","parent_post_id":null,"mentions":null,"media_urls":null,"metadata_json":null,"mydata_id":null,"promotion_id":null,"revenue_redirect_to":null,"revenue_redirect_percentage":null,"enable_spt":false,"enable_spot":false,"spot_id":null,"spt_id":null}"#;
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
        let json = r#"{"platform_id":"0xabc","name":"Test","tagline":"Tag","description":"Desc","developer":"0xdef","logo":"","terms_of_service":"","privacy_policy":"","platforms":[],"links":[],"cover_photo":null,"media_previews":null,"primary_category":"Social","secondary_category":null,"status":{"status":0},"release_date":"2024-01-01","wants_dao_governance":false,"governance_registry_id":null,"delegate_count":null,"delegate_term_epochs":null,"proposal_submission_cost":null,"max_votes_per_user":null,"quadratic_base_cost":null,"voting_period_epochs":null,"quorum_votes":null,"moderators_group_id":"0x123","redirect_uri":"https://example.com/callback"}"#;
        let result = parse_event_contents("platform", "PlatformCreatedEvent", json.as_bytes());
        assert!(
            result.is_ok(),
            "parse_event_contents should succeed for PlatformCreatedEvent JSON"
        );
        let parsed = result.unwrap();
        assert_eq!(parsed["platform_id"], "0xabc");
        assert_eq!(parsed["name"], "Test");
        assert_eq!(parsed["redirect_uri"], "https://example.com/callback");
    }

    /// BCS field order must match Move `platform::PlatformCreatedEvent` (redirect_uri after moderators_group_id).
    #[test]
    fn platform_created_event_bcs_parse_then_handler_row_shape() {
        use crate::handlers::platform::handle_platform_event;
        use crate::handlers::SocialEventRow;

        let pid = AccountAddress::from_hex_literal(
            "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        )
        .unwrap();
        let dev = AccountAddress::from_hex_literal(
            "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        )
        .unwrap();
        let mods_gid = AccountAddress::from_hex_literal(
            "0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
        )
        .unwrap();
        let ev = BcsPlatformCreatedEvent {
            platform_id: pid,
            name: "Test Platform".into(),
            tagline: "Tag".into(),
            description: "Desc".into(),
            developer: dev,
            logo: String::new(),
            terms_of_service: String::new(),
            privacy_policy: String::new(),
            platforms: vec![],
            links: vec![],
            cover_photo: None,
            media_previews: None,
            primary_category: "Social".into(),
            secondary_category: None,
            status: BcsPlatformStatus { status: 0 },
            release_date: "2024-01-01".into(),
            wants_dao_governance: false,
            governance_registry_id: None,
            delegate_count: None,
            delegate_term_epochs: None,
            proposal_submission_cost: None,
            max_votes_per_user: None,
            quadratic_base_cost: None,
            voting_period_epochs: None,
            quorum_votes: None,
            moderators_group_id: BcsMoveObjectId { bytes: mods_gid },
            redirect_uri: Some("https://example.com/oauth/callback".into()),
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize PlatformCreatedEvent BCS fixture");
        let json = parse_event_contents("platform", "PlatformCreatedEvent", &bytes)
            .expect("parse_event_contents should succeed for PlatformCreatedEvent BCS");
        assert_eq!(
            json["redirect_uri"].as_str(),
            Some("https://example.com/oauth/callback")
        );
        let event_id = "digest:platform-created";
        let rows =
            handle_platform_event("PlatformCreatedEvent", &json, event_id, 1_700_000_000_000)
                .expect("handler should deserialize JSON from BCS path");
        assert!(
            rows.iter()
                .any(|r| matches!(r, SocialEventRow::Platform(_))),
            "expected Platform row from PlatformCreatedEvent"
        );
        let platform_row = rows
            .iter()
            .find_map(|r| match r {
                SocialEventRow::Platform(p) => Some(p),
                _ => None,
            })
            .expect("platform row");
        assert_eq!(
            platform_row.redirect_uri.as_deref(),
            Some("https://example.com/oauth/callback")
        );
    }

    /// BCS → JSON → handler for join: membership upsert + audit event.
    #[test]
    fn user_joined_platform_event_bcs_parse_then_handler_row_shape() {
        use crate::handlers::platform::handle_platform_event;
        use crate::handlers::SocialEventRow;

        let wallet = AccountAddress::from_hex_literal(
            "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        )
        .unwrap();
        let pid = AccountAddress::from_hex_literal(
            "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        )
        .unwrap();
        let timestamp_ms = 1_735_891_200_000u64;
        let ev = BcsUserJoinedPlatformEvent {
            wallet_address: wallet,
            platform_id: pid,
            timestamp: timestamp_ms,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize UserJoinedPlatformEvent BCS fixture");
        let json = parse_event_contents("platform", "UserJoinedPlatformEvent", &bytes)
            .expect("parse_event_contents should succeed for UserJoinedPlatformEvent BCS");
        let event_id = "digest:join-bcs";
        let rows = handle_platform_event("UserJoinedPlatformEvent", &json, event_id, timestamp_ms)
            .expect("handler should deserialize JSON from BCS path");
        assert_eq!(rows.len(), 2);
        match &rows[0] {
            SocialEventRow::PlatformMembership(m) => {
                assert_eq!(m.platform_id, addr_to_string(&pid));
                assert_eq!(m.wallet_address, addr_to_string(&wallet));
                assert!(m.left_at.is_none());
            }
            other => panic!("expected PlatformMembership, got {:?}", other),
        }
        match &rows[1] {
            SocialEventRow::PlatformEvent(row) => {
                assert_eq!(row.event_type, "UserJoinedPlatform");
                assert_eq!(row.event_id.as_deref(), Some(event_id));
            }
            other => panic!("expected PlatformEvent, got {:?}", other),
        }
    }

    /// BCS → JSON → handler for leave: soft left_at stamp + audit event (no delete).
    #[test]
    fn user_left_platform_event_bcs_parse_then_handler_row_shape() {
        use crate::handlers::platform::handle_platform_event;
        use crate::handlers::SocialEventRow;
        use chrono::{TimeZone, Utc};

        let wallet = AccountAddress::from_hex_literal(
            "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        )
        .unwrap();
        let pid = AccountAddress::from_hex_literal(
            "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        )
        .unwrap();
        let timestamp_ms = 1_735_891_200_000u64;
        let ev = BcsUserLeftPlatformEvent {
            wallet_address: wallet,
            platform_id: pid,
            timestamp: timestamp_ms,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize UserLeftPlatformEvent BCS fixture");
        let json = parse_event_contents("platform", "UserLeftPlatformEvent", &bytes)
            .expect("parse_event_contents should succeed for UserLeftPlatformEvent BCS");
        let event_id = "digest:leave-bcs";
        let rows = handle_platform_event("UserLeftPlatformEvent", &json, event_id, timestamp_ms)
            .expect("handler should deserialize JSON from BCS path");
        assert_eq!(rows.len(), 2);
        match &rows[0] {
            SocialEventRow::PlatformMembershipLeave {
                platform_id,
                wallet_address,
                left_at,
            } => {
                assert_eq!(platform_id, &addr_to_string(&pid));
                assert_eq!(wallet_address, &addr_to_string(&wallet));
                let expected_left_at = {
                    let secs = (timestamp_ms / 1000) as i64;
                    let nsecs = ((timestamp_ms % 1000) * 1_000_000) as u32;
                    Utc.timestamp_opt(secs, nsecs)
                        .single()
                        .expect("fixture timestamp fits")
                        .naive_utc()
                };
                assert_eq!(*left_at, expected_left_at);
            }
            other => panic!("expected PlatformMembershipLeave, got {:?}", other),
        }
        match &rows[1] {
            SocialEventRow::PlatformEvent(row) => {
                assert_eq!(row.event_type, "UserLeftPlatform");
                assert_eq!(row.event_id.as_deref(), Some(event_id));
            }
            other => panic!("expected PlatformEvent, got {:?}", other),
        }
    }

    /// BCS serialization matches Move `platform::PlatformDeletedEvent`; handlers produce delete + audit rows.
    #[test]
    fn platform_deleted_event_bcs_parse_then_handler_row_shape() {
        use crate::handlers::platform::handle_platform_event;
        use crate::handlers::SocialEventRow;
        use chrono::{TimeZone, Utc};

        let pid = AccountAddress::from_hex_literal(
            "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        )
        .unwrap();
        let dev = AccountAddress::from_hex_literal(
            "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        )
        .unwrap();
        let by = AccountAddress::from_hex_literal(
            "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        )
        .unwrap();
        let timestamp_ms = 987_654_321_000u64;
        let ev = BcsPlatformDeletedEvent {
            platform_id: pid,
            name: "n".into(),
            developer: dev,
            deleted_by: by,
            timestamp: timestamp_ms,
            reasoning: None,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize PlatformDeletedEvent BCS fixture");
        let json = parse_event_contents("platform", "PlatformDeletedEvent", &bytes)
            .expect("parse_event_contents should succeed for PlatformDeletedEvent BCS");
        let deleted_at_naive = {
            let secs = (timestamp_ms / 1000) as i64;
            let nsecs = ((timestamp_ms % 1000) * 1_000_000) as u32;
            Utc.timestamp_opt(secs, nsecs)
                .single()
                .expect("fixture timestamp fits in naive datetime")
                .naive_utc()
        };
        let event_id = "digest:99";
        let rows = handle_platform_event("PlatformDeletedEvent", &json, event_id, timestamp_ms)
            .expect("handler should deserialize JSON from BCS path");
        assert_eq!(rows.len(), 2);
        match &rows[0] {
            SocialEventRow::PlatformDeleted { deleted_at, .. } => {
                assert_eq!(deleted_at, &deleted_at_naive)
            }
            other => panic!("expected PlatformDeleted, got {:?}", other),
        }
        match &rows[1] {
            SocialEventRow::PlatformEvent(row) => {
                assert_eq!(row.event_type, "PlatformDeleted");
                assert!(row.reasoning.is_none());
                assert_eq!(row.event_id.as_deref(), Some(event_id));
            }
            other => panic!("expected PlatformEvent, got {:?}", other),
        }
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
    fn test_parse_delegate_nominated_event_bcs_round_trip() {
        use move_core_types::account_address::AccountAddress;

        let nominee = AccountAddress::from_hex_literal("0xace").unwrap();
        let ev = BcsDelegateNominatedEvent {
            nominee_address: nominee,
            scheduled_term_start_epoch: 42,
            registry_type: 3,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize");
        let result = parse_event_contents("governance", "DelegateNominatedEvent", &bytes);
        assert!(result.is_ok(), "BCS parse should succeed");
        let json = result.unwrap();
        assert_eq!(json["scheduled_term_start_epoch"], 42);
        assert_eq!(json["registry_type"], 3);
        assert_eq!(json["nominee_address"], addr_to_string(&nominee));
    }

    #[test]
    fn test_parse_delegate_elected_event_v1_carries_vote_counts() {
        use move_core_types::account_address::AccountAddress;

        let a = AccountAddress::from_hex_literal("0xace").unwrap();
        let ev = BcsDelegateElectedEvent {
            delegate_address: a,
            term_start: 10,
            term_end: 100,
            registry_type: 1,
            upvotes: 42,
            downvotes: 5,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize");
        let result = parse_event_contents("governance", "DelegateElectedEvent", &bytes);
        assert!(result.is_ok(), "BCS v1 parse should succeed");
        let json = result.unwrap();
        assert_eq!(json["upvotes"], 42);
        assert_eq!(json["downvotes"], 5);
        assert_eq!(json["term_start"], 10);
        assert_eq!(json["term_end"], 100);
    }

    #[test]
    fn test_parse_delegate_elected_event_v0_legacy() {
        use move_core_types::account_address::AccountAddress;

        let a = AccountAddress::from_hex_literal("0xbeef").unwrap();
        let ev = BcsDelegateElectedEventV0 {
            delegate_address: a,
            term_start: 1,
            term_end: 2,
            registry_type: 0,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize");
        assert_eq!(bytes.len(), super::DELEGATE_ELECTED_BCS_V0_LEN);
        let result = parse_event_contents("governance", "DelegateElectedEvent", &bytes);
        assert!(result.is_ok(), "BCS v0 parse should succeed");
        let json = result.unwrap();
        assert_eq!(json["upvotes"], 0u64);
        assert_eq!(json["downvotes"], 0u64);
    }

    #[test]
    fn test_parse_delegate_elected_event_rejects_malformed_non_v0_len() {
        let bad = vec![0u8; 50];
        let result = parse_event_contents("governance", "DelegateElectedEvent", &bad);
        assert!(result.is_err(), "50-byte payload is neither v1 nor v0");
    }

    #[test]
    fn test_parse_subscription_events_bcs_round_trip() {
        use move_core_types::account_address::AccountAddress;

        let addr1 = AccountAddress::from_hex_literal("0x1").unwrap();
        let addr2 = AccountAddress::from_hex_literal("0x2").unwrap();

        let ev = BcsProfileSubscriptionServiceCreatedEvent {
            service_id: addr1,
            profile_owner: addr2,
            profile_id: addr1,
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
        assert!(json["profile_id"].as_str().unwrap().starts_with("0x"));
        assert_eq!(json["created_at"], 12345);

        let ev2 = BcsProfileSubscriptionCreatedEvent {
            subscription_id: addr1,
            service_id: addr1,
            plan_id: addr2,
            subscriber: addr2,
            expires_at: 99999,
            price: 500,
            duration_ms: 2_592_000_000,
            tier_level: Some(1),
            platform_id: None,
            auto_renew: true,
            platform_fee: 50,
            ecosystem_fee: 25,
            creator_amount: 425,
            payment_platform_id: None,
        };
        let bytes2 = bcs::to_bytes(&ev2).expect("serialize");
        let result2 =
            parse_event_contents("subscription", "ProfileSubscriptionCreatedEvent", &bytes2);
        assert!(result2.is_ok());
        let json2 = result2.unwrap();
        assert_eq!(json2["expires_at"], 99999);
        assert_eq!(json2["price"], 500);
        assert_eq!(json2["auto_renew"], true);
    }

    #[test]
    fn test_parse_subscription_events_json_fallback() {
        let json = r#"{"service_id":"0x123","plan_id":"0xplan","subscriber":"0x456","expires_at":1000,"price":100,"duration_ms":2592000000,"auto_renew":true}"#;
        let result = parse_event_contents(
            "subscription",
            "ProfileSubscriptionCreatedEvent",
            json.as_bytes(),
        );
        assert!(result.is_ok(), "JSON fallback should succeed");
        let parsed = result.unwrap();
        assert_eq!(parsed["service_id"], "0x123");
        assert_eq!(parsed["price"], 100);
        assert_eq!(parsed["auto_renew"], true);
    }

    #[test]
    fn test_mydata_marketplace_events_bcs_roundtrip_and_parse() {
        use move_core_types::account_address::AccountAddress;

        let pool_id = AccountAddress::from_hex_literal("0x1").unwrap();
        let broad = BcsBroadPoolCreatedEvent {
            pool_id,
            name: "pool-a".to_string(),
            created_at: 1_700_000_000_000u64,
        };
        let bytes = bcs::to_bytes(&broad).expect("broad pool bcs");
        let parsed = parse_mydata_event("BroadPoolCreatedEvent", &bytes).expect("parse");
        let json = parsed.expect("some json");
        assert_eq!(json["name"], "pool-a");
        assert_eq!(json["created_at"], 1_700_000_000_000u64);

        let sub_pool_id = AccountAddress::from_hex_literal("0x2").unwrap();
        let sub = BcsSubPoolCreatedEvent {
            sub_pool_id,
            broad_pool_id: pool_id,
            name: "sub-1".to_string(),
            created_at: 2,
        };
        let bytes = bcs::to_bytes(&sub).expect("sub pool bcs");
        let json = parse_mydata_event("SubPoolCreatedEvent", &bytes)
            .expect("parse")
            .expect("json");
        assert_eq!(json["name"], "sub-1");

        let ip = AccountAddress::from_hex_literal("0x3").unwrap();
        let assign = BcsMyDataAssignedToSubPoolEvent {
            ip_id: ip,
            sub_pool_ids: vec![sub_pool_id, pool_id],
            assigned_at: 99,
        };
        let bytes = bcs::to_bytes(&assign).expect("assign bcs");
        let json = parse_mydata_event("MyDataAssignedToSubPoolEvent", &bytes)
            .expect("parse")
            .expect("json");
        assert_eq!(json["assigned_at"], 99);
        let ids = json["sub_pool_ids"].as_array().expect("array");
        assert_eq!(ids.len(), 2);

        let snap_id = AccountAddress::from_hex_literal("0x4").unwrap();
        let buyer = AccountAddress::from_hex_literal("0x5").unwrap();
        let anchor = BcsSnapshotAnchorRecordedEvent {
            snapshot_id: snap_id,
            buyer_address: buyer,
            price_paid: 1_000,
            created_at: 42,
        };
        let bytes = bcs::to_bytes(&anchor).expect("anchor bcs");
        let json = parse_mydata_event("SnapshotAnchorRecordedEvent", &bytes)
            .expect("parse")
            .expect("json");
        assert_eq!(json["price_paid"], 1_000);
        assert!(json.get("manifest_hash").is_none());

        let anchor_v2 = BcsSnapshotAnchorRecordedEventV2 {
            snapshot_id: snap_id,
            buyer_address: buyer,
            price_paid: 2_000,
            created_at: 43,
            snapshot_manifest_hash: vec![1u8, 2u8, 3u8],
            payment_reference: vec![0xabu8; 4],
        };
        let bytes = bcs::to_bytes(&anchor_v2).expect("anchor v2 bcs");
        let json = parse_mydata_event("SnapshotAnchorRecordedEvent", &bytes)
            .expect("parse")
            .expect("json");
        assert_eq!(json["manifest_hash"].as_str().unwrap(), "0x010203");
        assert_eq!(json["payment_reference"].as_str().unwrap(), "0xabababab");

        let root_bytes = [7u8; 32];
        let dist = BcsDistributionRecordedEvent {
            snapshot_id: snap_id,
            total_amount: 9_000,
            contributor_count: 12,
            merkle_root: root_bytes.to_vec(),
            published_at: 88,
        };
        let bytes = bcs::to_bytes(&dist).expect("distribution bcs");
        let json = parse_mydata_event("DistributionRecordedEvent", &bytes)
            .expect("parse")
            .expect("json");
        assert_eq!(json["total_amount"], 9_000);
        assert_eq!(json["contributor_count"], 12);

        let merkle = BcsMerkleRootPublishedEvent {
            snapshot_id: snap_id,
            root_hash: root_bytes.to_vec(),
            published_at: 55,
        };
        let bytes = bcs::to_bytes(&merkle).expect("merkle bcs");
        let json = parse_mydata_event("MerkleRootPublishedEvent", &bytes)
            .expect("parse")
            .expect("json");
        assert_eq!(
            json["root_hash"].as_str().unwrap(),
            "0x0707070707070707070707070707070707070707070707070707070707070707"
        );

        let claimant = AccountAddress::from_hex_literal("0x6").unwrap();
        let claim = BcsClaimExecutedEvent {
            snapshot_id: snap_id,
            claimant,
            amount: 500,
            claimed_at: 77,
        };
        let bytes = bcs::to_bytes(&claim).expect("claim bcs");
        let json = parse_mydata_event("ClaimExecutedEvent", &bytes)
            .expect("parse")
            .expect("json");
        assert_eq!(json["amount"], 500);
    }

    /// `SubPoolCreatedEvent` BCS matches `social_contracts::mydata` (`mydata.move`).
    ///
    /// Fixture addresses mirror a real `create_sub_pool` flow (checkpoint ~77472): dynamic-field
    /// value object id and a pool-registry parent id from transaction effects. When validating
    /// against RPC, replace `contents` with raw bytes from `events[].contents` for that tx.
    #[test]
    fn test_parse_sub_pool_created_event_from_live_transaction() {
        use move_core_types::account_address::AccountAddress;

        use crate::handlers::mydata;
        use crate::handlers::SocialEventRow;

        let sub_pool_id = AccountAddress::from_hex_literal(
            "0x2147facf6a89c71b6fe2144647a0810f9eaf2e755235f61b94bd18f624f85cb1",
        )
        .unwrap();
        let broad_pool_id = AccountAddress::from_hex_literal(
            "0x31c6d92c219148254d4d8646f0fab639e812e19371fcb6d256d4ae138788b76d",
        )
        .unwrap();

        let ev = BcsSubPoolCreatedEvent {
            sub_pool_id,
            broad_pool_id,
            name: "research".to_string(),
            created_at: 1_717_171_717_000,
        };
        let contents = bcs::to_bytes(&ev).expect("SubPoolCreatedEvent BCS");

        let json = parse_event_contents("mydata", "SubPoolCreatedEvent", &contents)
            .expect("parse_event_contents full dispatch");
        assert_eq!(json["name"], "research");
        assert_eq!(
            json["sub_pool_id"].as_str().unwrap(),
            "0x2147facf6a89c71b6fe2144647a0810f9eaf2e755235f61b94bd18f624f85cb1"
        );
        assert_eq!(
            json["broad_pool_id"].as_str().unwrap(),
            "0x31c6d92c219148254d4d8646f0fab639e812e19371fcb6d256d4ae138788b76d"
        );

        let rows = mydata::handle_mydata_event("SubPoolCreatedEvent", &json, "digest:7")
            .expect("handler produces rows");
        assert_eq!(rows.len(), 1);
        match &rows[0] {
            SocialEventRow::MyDataSubPool(sp) => {
                assert_eq!(sp.sub_pool_id, json["sub_pool_id"].as_str().unwrap());
                assert_eq!(sp.broad_pool_id, json["broad_pool_id"].as_str().unwrap());
                assert_eq!(sp.name, "research");
                assert_eq!(sp.created_at_ms, 1_717_171_717_000);
                assert_eq!(sp.event_id, "digest:7");
                assert_eq!(sp.transaction_id, "digest");
            }
            _ => panic!("expected MyDataSubPool row"),
        }
    }

    #[test]
    fn post_reported_event_bcs_round_trip() {
        let post_id = AccountAddress::from_hex_literal(
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55",
        )
        .unwrap();
        let reporter = AccountAddress::from_hex_literal(
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
        )
        .unwrap();
        let ev = BcsPostReportedEvent {
            post_id,
            reporter,
            reason_code: 6,
            description: "Short description of the issue here.".to_string(),
            reported_at: 1_714_113_519_157,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "PostReportedEvent", &bytes).expect("parse");
        assert_eq!(
            json["object_id"].as_str().unwrap(),
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55"
        );
        assert_eq!(json["is_comment"], false);
        assert_eq!(json["reason_code"], 6);
        assert_eq!(
            json["description"].as_str().unwrap(),
            "Short description of the issue here."
        );
        assert_eq!(json["reported_at"], 1_714_113_519_157i64);
    }

    #[test]
    fn promoted_post_created_event_bcs_round_trip() {
        let post_id = AccountAddress::from_hex_literal(
            "0x320c97b64e7228da3b9f8a6adc5401b289bf41cf3f4e3a2e159d5ee939b8cdda",
        )
        .unwrap();
        let owner = AccountAddress::from_hex_literal(
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
        )
        .unwrap();
        let profile_id = AccountAddress::from_hex_literal(
            "0xccf58c286df1ee89368c9b5dfb4f2bc79ca97ce57611df33cc340556a9a260c3",
        )
        .unwrap();
        let ev = BcsPromotedPostCreatedEvent {
            post_id,
            owner,
            profile_id,
            payment_per_view: 1_000_000,
            total_budget: 1_000_000,
            created_at: 1_742_000_000_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let back: BcsPromotedPostCreatedEvent = bcs::from_bytes(&bytes).expect("bcs from_bytes");
        assert_eq!(back.payment_per_view, 1_000_000);
        assert_eq!(back.total_budget, 1_000_000);
        let json = parse_event_contents("post", "PromotedPostCreatedEvent", &bytes).expect("parse");
        assert_eq!(json["payment_per_view"], 1_000_000_i64);
        assert_eq!(json["total_budget"], 1_000_000_i64);
    }

    #[test]
    fn promoted_post_views_batch_confirmed_event_bcs_round_trip_len_one() {
        let viewer = AccountAddress::from_hex_literal(
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
        )
        .unwrap();
        let platform_id = AccountAddress::from_hex_literal(
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
        )
        .unwrap();
        let post_id = AccountAddress::from_hex_literal(
            "0x320c97b64e7228da3b9f8a6adc5401b289bf41cf3f4e3a2e159d5ee939b8cdda",
        )
        .unwrap();
        let promotion_id = AccountAddress::from_hex_literal(
            "0xccf58c286df1ee89368c9b5dfb4f2bc79ca97ce57611df33cc340556a9a260c3",
        )
        .unwrap();
        let ev = BcsPromotedPostViewsBatchConfirmedEvent {
            viewer,
            platform_id,
            timestamp: 1_742_000_000_000,
            items: vec![BcsPromotedViewConfirmItem {
                post_id,
                promotion_id,
                payment_amount: 1_000_000,
                platform_fee: 100_000,
                ecosystem_fee: 100_000,
                recipient_amount: 800_000,
                view_duration: 3_000,
            }],
            total_payment_amount: 1_000_000,
            total_platform_fee: 100_000,
            total_ecosystem_fee: 100_000,
            total_recipient_amount: 800_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let back: BcsPromotedPostViewsBatchConfirmedEvent =
            bcs::from_bytes(&bytes).expect("bcs from_bytes");
        assert_eq!(back.items.len(), 1);
        assert_eq!(back.total_recipient_amount, 800_000);
        let json = parse_event_contents("post", "PromotedPostViewsBatchConfirmedEvent", &bytes)
            .expect("parse");
        assert_eq!(json["items"].as_array().unwrap().len(), 1);
        assert_eq!(json["items"][0]["payment_amount"], 1_000_000_i64);
        assert_eq!(json["items"][0]["recipient_amount"], 800_000_i64);
        assert_eq!(json["total_platform_fee"], 100_000_i64);
    }

    #[test]
    fn promoted_post_views_batch_confirmed_event_bcs_round_trip_len_two() {
        let viewer = AccountAddress::from_hex_literal(
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
        )
        .unwrap();
        let platform_id = AccountAddress::from_hex_literal(
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
        )
        .unwrap();
        let post_a = AccountAddress::from_hex_literal(
            "0x320c97b64e7228da3b9f8a6adc5401b289bf41cf3f4e3a2e159d5ee939b8cdda",
        )
        .unwrap();
        let promo_a = AccountAddress::from_hex_literal(
            "0xccf58c286df1ee89368c9b5dfb4f2bc79ca97ce57611df33cc340556a9a260c3",
        )
        .unwrap();
        let post_b = AccountAddress::from_hex_literal(
            "0xa7953fb1af6d0495b3da10d4d25888158e8dc451fa5354a9723dc70676d38f3d",
        )
        .unwrap();
        let promo_b = AccountAddress::from_hex_literal(
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55",
        )
        .unwrap();
        let ev = BcsPromotedPostViewsBatchConfirmedEvent {
            viewer,
            platform_id,
            timestamp: 1_742_000_000_100,
            items: vec![
                BcsPromotedViewConfirmItem {
                    post_id: post_a,
                    promotion_id: promo_a,
                    payment_amount: 1_000_000,
                    platform_fee: 100_000,
                    ecosystem_fee: 100_000,
                    recipient_amount: 800_000,
                    view_duration: 3_000,
                },
                BcsPromotedViewConfirmItem {
                    post_id: post_b,
                    promotion_id: promo_b,
                    payment_amount: 2_000_000,
                    platform_fee: 200_000,
                    ecosystem_fee: 200_000,
                    recipient_amount: 1_600_000,
                    view_duration: 4_000,
                },
            ],
            total_payment_amount: 3_000_000,
            total_platform_fee: 300_000,
            total_ecosystem_fee: 300_000,
            total_recipient_amount: 2_400_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "PromotedPostViewsBatchConfirmedEvent", &bytes)
            .expect("parse");
        assert_eq!(json["items"].as_array().unwrap().len(), 2);
        assert_eq!(json["total_recipient_amount"], 2_400_000_i64);
        assert_eq!(json["items"][1]["view_duration"], 4_000_i64);
    }

    #[test]
    fn tip_event_bcs_round_trip() {
        let object_id = AccountAddress::from_hex_literal(
            "0xa7953fb1af6d0495b3da10d4d25888158e8dc451fa5354a9723dc70676d38f3d",
        )
        .unwrap();
        let from = AccountAddress::from_hex_literal(
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
        )
        .unwrap();
        let to = AccountAddress::from_hex_literal(
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
        )
        .unwrap();
        let ev = BcsTipEvent {
            object_id,
            from,
            to,
            amount: 5_000_000_000,
            coin_type: BcsMoveTypeName {
                name: BcsMoveAsciiString {
                    bytes: b"0x2::myso::MYSO".to_vec(),
                },
            },
            is_post: true,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let back: BcsTipEvent = bcs::from_bytes(&bytes).expect("bcs from_bytes");
        assert_eq!(back.amount, 5_000_000_000);
        assert_eq!(back.is_post, true);
        let json = parse_event_contents("post", "TipEvent", &bytes).expect("parse");
        assert_eq!(json["amount"], 5_000_000_000_i64);
        assert_eq!(json["is_post"], true);
        assert_eq!(json["coin_type"].as_str().unwrap(), "0x2::myso::MYSO");
    }

    #[test]
    fn poc_beneficiary_vault_deposit_bcs_round_trip() {
        let vault_id = AccountAddress::from_hex_literal("0x1").unwrap();
        let beneficiary = AccountAddress::from_hex_literal("0x2").unwrap();
        let source_post_id = AccountAddress::from_hex_literal("0x3").unwrap();
        let ev = BcsPoCBeneficiaryVaultDepositEvent {
            vault_id,
            beneficiary,
            coin_type: BcsMoveTypeName {
                name: BcsMoveAsciiString {
                    bytes: b"0x2::sui::SUI".to_vec(),
                },
            },
            amount: 1_000,
            source_post_id: Some(source_post_id),
            timestamp: 42,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let back: BcsPoCBeneficiaryVaultDepositEvent = bcs::from_bytes(&bytes).expect("from_bytes");
        assert_eq!(back.amount, 1_000);
        let json =
            parse_event_contents("poc_vault", "PoCBeneficiaryVaultDepositEvent", &bytes).unwrap();
        assert_eq!(json["amount"], 1_000_i64);
        assert_eq!(json["timestamp"], 42_i64);
        assert_eq!(json["coin_type"].as_str().unwrap(), "0x2::sui::SUI");
        assert_eq!(
            json["source_post_id"].as_str().unwrap(),
            "0x0000000000000000000000000000000000000000000000000000000000000003"
        );
    }

    #[test]
    fn poc_beneficiary_vault_claimed_bcs_round_trip() {
        let vault_id = AccountAddress::from_hex_literal("0x10").unwrap();
        let beneficiary = AccountAddress::from_hex_literal("0x20").unwrap();
        let referrer = AccountAddress::from_hex_literal("0x30").unwrap();
        let ev = BcsPoCBeneficiaryVaultClaimedEvent {
            vault_id,
            beneficiary,
            coin_type: BcsMoveTypeName {
                name: BcsMoveAsciiString {
                    bytes: b"0x2::myso::MYSO".to_vec(),
                },
            },
            referrer: Some(referrer),
            treasury_amount: 100,
            referrer_amount: 200,
            beneficiary_amount: 700,
            join_referral_applied: true,
            timestamp: 99,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json =
            parse_event_contents("poc_vault", "PoCBeneficiaryVaultClaimedEvent", &bytes).unwrap();
        assert_eq!(json["treasury_amount"], 100_i64);
        assert_eq!(json["referrer_amount"], 200_i64);
        assert_eq!(json["beneficiary_amount"], 700_i64);
        assert_eq!(json["join_referral_applied"], true);
        assert_eq!(json["timestamp"], 99_i64);
        assert!(json["referrer"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn username_beneficiary_provisioned_bcs_round_trip() {
        let beneficiary_id = AccountAddress::from_hex_literal("0x10").unwrap();
        let beneficiary_address = AccountAddress::from_hex_literal("0x20").unwrap();
        let vault_id = AccountAddress::from_hex_literal("0x30").unwrap();
        let provisioned_by = AccountAddress::from_hex_literal("0x40").unwrap();
        let ev = BcsUsernameBeneficiaryProvisionedEvent {
            beneficiary_id,
            username: "creator".to_string(),
            creator_identity_source: 1,
            creator_identity_hash: vec![1, 2, 3],
            required_x_handle: "creatorx".to_string(),
            beneficiary_address,
            vault_id,
            provisioned_by,
            provisioned_at: 1_700_000_000_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents(
            "poc_username_beneficiary",
            "UsernameBeneficiaryProvisionedEvent",
            &bytes,
        )
        .unwrap();
        assert_eq!(json["username"], "creator");
        assert_eq!(json["creator_identity_source"], 1_i64);
        assert_eq!(json["creator_identity_hash"], "0x010203");
        assert_eq!(json["provisioned_at"], 1_700_000_000_000_i64);
    }

    #[test]
    fn username_beneficiary_claimed_bcs_via_proof_of_creativity_module() {
        let bytes = super::username_beneficiary_claimed_bcs_fixture();
        let json = parse_event_contents(
            "proof_of_creativity",
            "UsernameBeneficiaryClaimedEvent",
            &bytes,
        )
        .expect("parse via proof_of_creativity module tag");
        assert_eq!(json["username"], "pocub1782775058");
    }

    #[test]
    fn creator_identity_wallet_linked_bcs_round_trip() {
        let wallet = AccountAddress::from_hex_literal("0x50").unwrap();
        let beneficiary_id = AccountAddress::from_hex_literal("0x60").unwrap();
        let ev = BcsCreatorIdentityWalletLinkedEvent {
            creator_identity_source: 1,
            creator_identity_hash: vec![4, 5],
            wallet,
            beneficiary_id,
            linked_at: 99,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents(
            "poc_username_beneficiary",
            "CreatorIdentityWalletLinkedEvent",
            &bytes,
        )
        .unwrap();
        assert_eq!(json["creator_identity_hash"], "0x0405");
        assert_eq!(json["linked_at"], 99_i64);
        assert!(json["wallet"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn poc_dispute_resolved_bcs_round_trip() {
        let dispute_id = AccountAddress::from_hex_literal("0xaa").unwrap();
        let post_id = AccountAddress::from_hex_literal("0xbb").unwrap();
        let ev = BcsPocDisputeResolvedEvent {
            dispute_id,
            post_id,
            resolution: 1,
            winning_side: 2,
            total_winning_stake: 10,
            total_losing_stake: 3,
            badge_revoked: true,
            redirection_removed: false,
            quorum_met: true,
            post_poc_disputes_submitted: 4,
            timestamp: 1234,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("poc", "PoCDisputeResolvedEvent", &bytes).expect("parse");
        assert_eq!(json["resolution"], 1_i64);
        assert_eq!(json["winning_side"], 2_i64);
        assert_eq!(json["badge_revoked"], true);
        assert_eq!(json["quorum_met"], true);
        assert_eq!(json["post_poc_disputes_submitted"], 4_i64);
    }

    #[test]
    fn poc_dispute_submitted_bcs_round_trip() {
        let dispute_id = AccountAddress::from_hex_literal("0xaa").unwrap();
        let post_id = AccountAddress::from_hex_literal("0xbb").unwrap();
        let disputer = AccountAddress::from_hex_literal("0xcc").unwrap();
        let ev = BcsPocDisputeSubmittedEvent {
            dispute_id,
            post_id,
            disputer,
            dispute_type: 1,
            stake_amount: 100,
            dispute_round: 1,
            effective_fee: 100,
            required_total_stake_quorum: 500,
            post_poc_disputes_submitted_after: 1,
            voting_start_ms: 10,
            voting_end_ms: 10000,
            evidence: "Derivative claim".to_string(),
            timestamp: 12345,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("poc", "PoCDisputeSubmittedEvent", &bytes).expect("parse");
        assert_eq!(json["dispute_round"], 1_i64);
        assert_eq!(json["evidence"], "Derivative claim");
        assert_eq!(json["effective_fee"], 100_i64);
    }

    #[test]
    fn post_moderation_event_bcs_round_trip() {
        let post_id = AccountAddress::from_hex_literal(
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55",
        )
        .unwrap();
        let platform_id = AccountAddress::from_hex_literal(
            "0x05a761d1fe77ff1006e210727f25a7f3137c6d1e87dc6dab898fd685736cff5a",
        )
        .unwrap();
        let moderated_by = AccountAddress::from_hex_literal(
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
        )
        .unwrap();
        let ev = BcsPostModerationEvent {
            post_id,
            platform_id,
            removed: true,
            moderated_by,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "PostModerationEvent", &bytes).expect("parse");
        assert_eq!(
            json["object_id"].as_str().unwrap(),
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55"
        );
        assert_eq!(
            json["platform_id"].as_str().unwrap(),
            "0x05a761d1fe77ff1006e210727f25a7f3137c6d1e87dc6dab898fd685736cff5a"
        );
        assert_eq!(json["removed"], true);
        assert_eq!(
            json["moderated_by"].as_str().unwrap(),
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8"
        );
        assert_eq!(json["moderated_at"], 0i64);
    }

    #[test]
    fn post_deleted_event_bcs_round_trip() {
        use move_core_types::account_address::AccountAddress;

        let post_id = AccountAddress::from_hex_literal(
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55",
        )
        .unwrap();
        let owner = AccountAddress::from_hex_literal(
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
        )
        .unwrap();
        let profile_id = AccountAddress::from_hex_literal(
            "0x000000000000000000000000000000000000000000000000000000006f199773",
        )
        .unwrap();
        let ev = BcsPostDeletedEvent {
            post_id,
            owner,
            profile_id,
            post_type: "quote_repost".to_string(),
            deleted_at: 1_717_200_000_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "PostDeletedEvent", &bytes).expect("parse");
        assert_eq!(
            json["object_id"].as_str().unwrap(),
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55"
        );
        assert_eq!(json["is_post"], true);
        assert_eq!(json["post_type"].as_str().unwrap(), "quote_repost");
        assert_eq!(json["deleted_at"], 1_717_200_000_000u64);
    }

    #[test]
    fn comment_deleted_event_bcs_round_trip() {
        use move_core_types::account_address::AccountAddress;

        let comment_id = AccountAddress::from_hex_literal(
            "0xcccc00000000000000000000000000000000000000000000000000000000cccc",
        )
        .unwrap();
        let post_id = AccountAddress::from_hex_literal(
            "0xdddd00000000000000000000000000000000000000000000000000000000dddd",
        )
        .unwrap();
        let owner = AccountAddress::from_hex_literal(
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
        )
        .unwrap();
        let profile_id = AccountAddress::from_hex_literal(
            "0x000000000000000000000000000000000000000000000000000000006f199773",
        )
        .unwrap();
        let ev = BcsCommentDeletedEvent {
            comment_id,
            post_id,
            owner,
            profile_id,
            deleted_at: 1_717_201_000_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "CommentDeletedEvent", &bytes).expect("parse");
        assert_eq!(
            json["object_id"].as_str().unwrap(),
            "0xcccc00000000000000000000000000000000000000000000000000000000cccc"
        );
        assert_eq!(json["is_post"], false);
        assert!(json["post_type"].is_null());
    }

    #[test]
    fn comment_created_attribution_bcs_round_trip() {
        let ev = BcsCommentCreatedEventWithAttribution {
            comment_id: AccountAddress::from_hex_literal("0x1").unwrap(),
            post_id: AccountAddress::from_hex_literal("0x2").unwrap(),
            parent_comment_id: None,
            owner: AccountAddress::from_hex_literal("0x3").unwrap(),
            profile_id: AccountAddress::from_hex_literal("0x4").unwrap(),
            content: "hi".to_string(),
            mentions: None,
            actor_address: AccountAddress::from_hex_literal("0x5").unwrap(),
            sub_agent_id: Some(BcsMoveObjectId {
                bytes: AccountAddress::from_hex_literal("0x6").unwrap(),
            }),
            action_identity_class: 2,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "CommentCreatedEvent", &bytes).expect("parse");
        assert_eq!(json["action_identity_class"], 2_i64);
        assert!(json["sub_agent_id"].as_str().is_some());
    }

    #[test]
    fn comment_created_legacy_bcs_still_parses() {
        let ev = BcsCommentCreatedEvent {
            comment_id: AccountAddress::from_hex_literal("0x1").unwrap(),
            post_id: AccountAddress::from_hex_literal("0x2").unwrap(),
            parent_comment_id: None,
            owner: AccountAddress::from_hex_literal("0x3").unwrap(),
            profile_id: AccountAddress::from_hex_literal("0x4").unwrap(),
            content: "legacy".to_string(),
            mentions: None,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "CommentCreatedEvent", &bytes).expect("parse");
        assert_eq!(json["action_identity_class"], 0_i64);
        assert!(json["sub_agent_id"].is_null());
    }

    #[test]
    fn reaction_attribution_bcs_round_trip() {
        let actor_address = AccountAddress::from_hex_literal("0x4").unwrap();
        let ev = BcsReactionEvent {
            object_id: AccountAddress::from_hex_literal("0x1").unwrap(),
            _user: AccountAddress::from_hex_literal("0x2").unwrap(),
            reaction: "👍".to_string(),
            is_post: true,
            principal_owner: AccountAddress::from_hex_literal("0x3").unwrap(),
            actor_address,
            sub_agent_id: Some(BcsMoveObjectId {
                bytes: AccountAddress::from_hex_literal("0x5").unwrap(),
            }),
            action_identity_class: 2,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "ReactionEvent", &bytes).expect("parse");
        assert!(json["object_id"].as_str().unwrap().starts_with("0x"));
        assert_eq!(json["reaction_text"], "👍");
        assert_eq!(json["is_post"], true);
        assert_eq!(
            json["user_address"],
            addr_to_string(&actor_address),
            "user_address must come from actor_address, not legacy user field"
        );
        assert_eq!(json["actor_address"], addr_to_string(&actor_address));
        assert_eq!(
            json["principal_owner"],
            addr_to_string(&AccountAddress::from_hex_literal("0x3").unwrap())
        );
        assert_eq!(json["action_identity_class"], 2_i64);
        assert!(json["sub_agent_id"].as_str().is_some());
    }

    #[test]
    fn remove_reaction_attribution_bcs_round_trip() {
        let actor_address = AccountAddress::from_hex_literal("0x4").unwrap();
        let ev = BcsRemoveReactionEvent {
            object_id: AccountAddress::from_hex_literal("0x1").unwrap(),
            user: AccountAddress::from_hex_literal("0x2").unwrap(),
            reaction: "👍".to_string(),
            is_post: true,
            principal_owner: AccountAddress::from_hex_literal("0x3").unwrap(),
            actor_address,
            sub_agent_id: Some(BcsMoveObjectId {
                bytes: AccountAddress::from_hex_literal("0x5").unwrap(),
            }),
            action_identity_class: 2,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "RemoveReactionEvent", &bytes).expect("parse");
        assert!(json["object_id"].as_str().unwrap().starts_with("0x"));
        assert_eq!(json["reaction_text"], "👍");
        assert_eq!(json["is_post"], true);
        assert_eq!(
            json["user_address"],
            addr_to_string(&actor_address),
            "user_address must come from actor_address, not legacy user field"
        );
        assert_eq!(json["action_identity_class"], 2_i64);
        assert!(json["sub_agent_id"].as_str().is_some());
    }

    #[test]
    fn memory_account_deactivated_bcs_round_trip() {
        let ev = BcsMemoryAccountDeactivatedEvent {
            account_id: AccountAddress::from_hex_literal("0xaa").unwrap(),
            owner: AccountAddress::from_hex_literal("0xbb").unwrap(),
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json =
            parse_event_contents("memory", "MemoryAccountDeactivated", &bytes).expect("parse");
        assert!(json["account_id"].as_str().unwrap().starts_with("0x"));
        assert!(json["owner"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn memory_account_reactivated_bcs_round_trip() {
        let ev = BcsMemoryAccountReactivatedEvent {
            account_id: AccountAddress::from_hex_literal("0xaa").unwrap(),
            owner: AccountAddress::from_hex_literal("0xbb").unwrap(),
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json =
            parse_event_contents("memory", "MemoryAccountReactivated", &bytes).expect("parse");
        assert!(json["account_id"].as_str().unwrap().starts_with("0x"));
        assert!(json["owner"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn memory_account_migrated_bcs_round_trip() {
        let ev = BcsMemoryAccountMigratedEvent {
            account_id: AccountAddress::from_hex_literal("0xaa").unwrap(),
            from: 1,
            to: 2,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("memory", "MemoryAccountMigrated", &bytes).expect("parse");
        assert_eq!(json["from"], 1_i64);
        assert_eq!(json["to"], 2_i64);
    }

    #[test]
    fn memory_registry_migrated_bcs_round_trip() {
        let ev = BcsMemoryRegistryMigratedEvent {
            registry_id: AccountAddress::from_hex_literal("0xcc").unwrap(),
            from: 1,
            to: 2,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("memory", "MemoryRegistryMigrated", &bytes).expect("parse");
        assert!(json["registry_id"].as_str().unwrap().starts_with("0x"));
        assert_eq!(json["from"], 1_i64);
        assert_eq!(json["to"], 2_i64);
    }

    #[test]
    fn agent_memory_vault_created_bcs_round_trip() {
        let ev = BcsAgentMemoryVaultCreatedEvent {
            vault_id: AccountAddress::from_hex_literal("0x11").unwrap(),
            agent_object_id: AccountAddress::from_hex_literal("0x22").unwrap(),
            memory_account_id: AccountAddress::from_hex_literal("0x33").unwrap(),
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json =
            parse_event_contents("memory", "AgentMemoryVaultCreated", &bytes).expect("parse");
        assert!(json["vault_id"].as_str().unwrap().starts_with("0x"));
        assert!(json["agent_object_id"].as_str().unwrap().starts_with("0x"));
        assert!(json["memory_account_id"]
            .as_str()
            .unwrap()
            .starts_with("0x"));
    }

    #[test]
    fn sub_agent_registered_memory_bcs_round_trip() {
        let ev = BcsSubAgentRegisteredEvent {
            account_id: AccountAddress::from_hex_literal("0xaa").unwrap(),
            principal_owner: AccountAddress::from_hex_literal("0xbb").unwrap(),
            profile_id: AccountAddress::from_hex_literal("0xcc").unwrap(),
            organization_id: AccountAddress::from_hex_literal("0x11").unwrap(),
            agent_object_id: AccountAddress::from_hex_literal("0xdd").unwrap(),
            derived_address: AccountAddress::from_hex_literal("0xee").unwrap(),
            label: "bot".to_string(),
            identity_class: 1,
            role_tags: 0,
            capabilities: 512,
            delegatable_caps: 0,
            register_scope: 0,
            approval_required_caps: 0,
            max_action_spend: Some(1_000_000_000),
            platform_scope: None,
            parent_object_id: None,
            depth: 1,
            registered_by: AccountAddress::from_hex_literal("0xbb").unwrap(),
            expires_at: None,
            active: true,
            created_at: 1234,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("memory", "SubAgentRegistered", &bytes).expect("parse");
        assert_eq!(json["label"], "bot");
        assert_eq!(json["capabilities"], 512_i64);
        assert_eq!(json["max_action_spend"], 1_000_000_000_i64);
    }

    fn sample_post_created_with_attribution(
        organization_id: Option<BcsMoveObjectId>,
    ) -> BcsPostCreatedEvent {
        BcsPostCreatedEvent {
            post_id: AccountAddress::from_hex_literal("0x1").unwrap(),
            owner: AccountAddress::from_hex_literal("0x2").unwrap(),
            profile_id: AccountAddress::from_hex_literal("0x3").unwrap(),
            platform_id: AccountAddress::from_hex_literal("0x4").unwrap(),
            permissions: 0,
            content: "hello".into(),
            post_type: "post".into(),
            parent_post_id: None,
            mentions: None,
            media_urls: None,
            metadata_json: None,
            access: BcsPostAccess::Public,
            promotion_id: None,
            revenue_redirect_to: None,
            revenue_redirect_percentage: None,
            enable_spt: false,
            spt_id: None,
            poc_redirection_kind: 1,
            actor_address: AccountAddress::from_hex_literal("0x2").unwrap(),
            sub_agent_id: None,
            organization_id,
            action_identity_class: 0,
        }
    }

    fn org_object_id() -> BcsMoveObjectId {
        BcsMoveObjectId {
            bytes: AccountAddress::from_hex_literal(
                "0x0000000000000000000000000000000000000000000000000000000000000001",
            )
            .unwrap(),
        }
    }

    #[test]
    fn post_created_event_bcs_round_trip_with_organization_id() {
        let ev = sample_post_created_with_attribution(Some(org_object_id()));
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "PostCreatedEvent", &bytes).expect("parse");
        assert_eq!(json["content"], "hello");
        assert!(json["organization_id"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn post_created_event_bcs_round_trip_without_organization_id() {
        let ev = sample_post_created_with_attribution(None);
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "PostCreatedEvent", &bytes).expect("parse");
        assert!(json["organization_id"].is_null());
    }

    #[test]
    fn comment_created_event_bcs_round_trip_with_organization_id() {
        let ev = BcsCommentCreatedEventWithOrganization {
            comment_id: AccountAddress::from_hex_literal("0xc1").unwrap(),
            post_id: AccountAddress::from_hex_literal("0xc2").unwrap(),
            parent_comment_id: None,
            owner: AccountAddress::from_hex_literal("0xc3").unwrap(),
            profile_id: AccountAddress::from_hex_literal("0xc4").unwrap(),
            content: "comment".into(),
            mentions: None,
            actor_address: AccountAddress::from_hex_literal("0xc3").unwrap(),
            sub_agent_id: None,
            organization_id: Some(org_object_id()),
            action_identity_class: 0,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "CommentCreatedEvent", &bytes).expect("parse");
        assert_eq!(json["content"], "comment");
        assert!(json["organization_id"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn repost_event_bcs_round_trip_with_organization_id() {
        let ev = BcsRepostEventWithOrganization {
            repost_id: AccountAddress::from_hex_literal("0x10").unwrap(),
            original_id: AccountAddress::from_hex_literal("0x20").unwrap(),
            is_original_post: true,
            owner: AccountAddress::from_hex_literal("0x30").unwrap(),
            profile_id: AccountAddress::from_hex_literal("0x40").unwrap(),
            actor_address: AccountAddress::from_hex_literal("0x30").unwrap(),
            sub_agent_id: None,
            organization_id: Some(org_object_id()),
            action_identity_class: 0,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "RepostEvent", &bytes).expect("parse");
        assert!(json["organization_id"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn repost_removed_event_bcs_round_trip() {
        let ev = BcsRepostRemovedEvent {
            repost_id: AccountAddress::from_hex_literal("0x10").unwrap(),
            original_id: AccountAddress::from_hex_literal("0x20").unwrap(),
            owner: AccountAddress::from_hex_literal("0x30").unwrap(),
            actor_address: AccountAddress::from_hex_literal("0x40").unwrap(),
            sub_agent_id: None,
            organization_id: Some(org_object_id()),
            action_identity_class: 1,
            removed_at: 123,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "RepostRemovedEvent", &bytes).expect("parse");
        assert_eq!(json["is_original_post"], true);
        assert_eq!(json["removed_at"], 123);
    }

    #[test]
    fn message_digest_sent_bcs_round_trip() {
        let ev = BcsMessageDigestSent {
            group_id: org_object_id(),
            seq: 7,
            sender: AccountAddress::from_hex_literal("0x30").unwrap(),
            recipient: AccountAddress::from_hex_literal("0x40").unwrap(),
            content_digest: vec![0xab; 32],
            content_uri: "wal://encrypted-message".into(),
            created_at_ms: 456,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("message_log", "MessageDigestSent", &bytes).expect("parse");
        assert_eq!(json["seq"], 7);
        assert_eq!(json["content_digest"], "ab".repeat(32));
        assert_eq!(json["content_uri"], "wal://encrypted-message");
    }

    #[test]
    fn reaction_event_bcs_round_trip_with_organization_id() {
        let ev = BcsReactionEventWithOrganization {
            object_id: AccountAddress::from_hex_literal("0x50").unwrap(),
            _user: AccountAddress::from_hex_literal("0x60").unwrap(),
            reaction: "like".into(),
            is_post: true,
            principal_owner: AccountAddress::from_hex_literal("0x70").unwrap(),
            actor_address: AccountAddress::from_hex_literal("0x60").unwrap(),
            sub_agent_id: None,
            organization_id: Some(org_object_id()),
            action_identity_class: 0,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs");
        let json = parse_event_contents("post", "ReactionEvent", &bytes).expect("parse");
        assert_eq!(json["reaction_text"], "like");
        assert!(json["organization_id"].as_str().unwrap().starts_with("0x"));
    }

    /// The Move `AiCreditOraclePubkeyUpdated` event was fixed to carry `new_pubkey`
    /// so the indexer can persist `oracle_pubkey_hex` instead of only `updated_by`.
    #[test]
    fn ai_credit_oracle_pubkey_updated_bcs_roundtrip() {
        let updated_by = AccountAddress::from_hex_literal(
            "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
        )
        .unwrap();
        let new_pubkey = vec![0xde, 0xad, 0xbe, 0xef, 0x01, 0x23, 0x45, 0x67];
        let ev = BcsAiCreditOraclePubkeyUpdatedEvent {
            updated_by,
            new_pubkey: new_pubkey.clone(),
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize AiCreditOraclePubkeyUpdated");
        let json = parse_event_contents("ai_credit", "AiCreditOraclePubkeyUpdated", &bytes)
            .expect("parse AiCreditOraclePubkeyUpdated");
        assert!(json["updated_by"].as_str().unwrap().starts_with("0x"));
        assert_eq!(json["new_pubkey_hex"], hex::encode(&new_pubkey));
    }

    /// New `AiCreditMarkupUpdated` event carries the dynamic `oracle_markup_bps`.
    #[test]
    fn ai_credit_markup_updated_bcs_roundtrip() {
        let updated_by = AccountAddress::from_hex_literal(
            "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
        )
        .unwrap();
        let ev = BcsAiCreditMarkupUpdatedEvent {
            updated_by,
            oracle_markup_bps: 250,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize AiCreditMarkupUpdated");
        let json = parse_event_contents("ai_credit", "AiCreditMarkupUpdated", &bytes)
            .expect("parse AiCreditMarkupUpdated");
        assert!(json["updated_by"].as_str().unwrap().starts_with("0x"));
        assert_eq!(json["oracle_markup_bps"], 250);
    }

    /// SpotConfigUpdatedEvent carries platform/ecosystem fee bps, not legacy
    /// `fee_bps` / `fee_split_bps_platform` fields.
    #[test]
    fn spot_config_updated_bcs_roundtrip_fee_breakout() {
        let updated_by = AccountAddress::from_hex_literal(
            "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
        )
        .unwrap();
        let oracle_address = AccountAddress::from_hex_literal(
            "0x2f41b4f43f505d427e8777c511461de8e50eac26558a996627dded27dce50918",
        )
        .unwrap();
        let ev = BcsSpotConfigUpdatedEvent {
            updated_by,
            truth_enabled: true,
            confidence_threshold_bps: 6500,
            resolution_window_ms: 86_400_000,
            max_resolution_window_ms: 604_800_000,
            payout_delay_ms: 12_000,
            platform_fee_bps: 50,
            ecosystem_fee_bps: 50,
            creator_fee_bps: 100,
            creator_claim_window_ms: 7_776_000_000,
            expired_creator_ecosystem_bps: 10_000,
            min_betting_options: 2,
            max_betting_options: 10,
            min_reasoning_length: 10,
            max_reasoning_length: 5000,
            max_evidence_urls: 10,
            oracle_address,
            max_single_bet: 1_000_000_000,
            max_bets_per_record: 100,
            max_claim_per_post: 10,
            spot_governance_registry_id: oracle_address,
            timestamp: 1_700_000_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize SpotConfigUpdatedEvent");
        let json = parse_event_contents("social_proof_of_truth", "SpotConfigUpdatedEvent", &bytes)
            .expect("parse SpotConfigUpdatedEvent");
        assert_eq!(json["platform_fee_bps"], 50);
        assert_eq!(json["ecosystem_fee_bps"], 50);
        assert_eq!(json["min_betting_options"], 2);
        assert_eq!(json["max_betting_options"], 10);
        assert_eq!(json["max_reasoning_length"], 5000);
        assert_eq!(json["max_evidence_urls"], 10);
        assert_eq!(json["max_bets_per_record"], 100);
        assert_eq!(json["max_claim_per_post"], 10);
        assert_eq!(
            json["spot_governance_registry_id"].as_str().unwrap(),
            format!("0x{}", hex::encode(oracle_address))
        );
        assert_eq!(json["timestamp"], 1_700_000_000);
        assert!(json.get("fee_bps").is_none());
        assert!(json.get("fee_split_bps_platform").is_none());
    }

    #[test]
    fn poc_config_updated_bcs_roundtrip_dispute_governance_registry_id() {
        let updated_by = AccountAddress::from_hex_literal(
            "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
        )
        .unwrap();
        let oracle_address = AccountAddress::from_hex_literal(
            "0x2f41b4f43f505d427e8777c511461de8e50eac26558a996627dded27dce50918",
        )
        .unwrap();
        let dispute_governance_registry_id = AccountAddress::from_hex_literal(
            "0x3f41b4f43f505d427e8777c511461de8e50eac26558a996627dded27dce50919",
        )
        .unwrap();
        let ev = BcsPocConfigUpdatedEvent {
            updated_by,
            oracle_address,
            image_threshold: 85,
            video_threshold: 85,
            audio_threshold: 85,
            revenue_redirect_percentage: 100,
            dispute_cost: 5_000_000_000,
            min_vote_stake: 1_000_000_000,
            max_vote_stake: 100_000_000_000,
            voting_duration_ms: 7 * 24 * 60 * 60 * 1000,
            max_reasoning_length: 5000,
            max_evidence_urls: 10,
            max_votes_per_dispute: 10_000,
            dispute_governance_registry_id,
            claim_treasury_fee_bps: 100,
            max_referral_bps: 500,
            video_embedded_audio_redirect_bps: 3000,
            dispute_quorum_base_stake: 0,
            dispute_second_round_fee_multiplier_bps: 10_000,
            dispute_second_round_quorum_multiplier_bps: 10_000,
            username_beneficiary_join_referral_bps: 500,
            max_disputes_per_post: 2,
            min_vault_deposit_amount: 1,
            timestamp: 1_700_000_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize PoCConfigUpdatedEvent");
        let json = parse_event_contents("proof_of_creativity", "PoCConfigUpdatedEvent", &bytes)
            .expect("parse PoCConfigUpdatedEvent");
        assert!(json["dispute_governance_registry_id"]
            .as_str()
            .unwrap()
            .starts_with("0x"));
    }

    #[test]
    fn subscription_config_updated_bcs_roundtrip() {
        let updated_by = AccountAddress::from_hex_literal(
            "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
        )
        .unwrap();
        let ev = BcsSubscriptionConfigUpdatedEvent {
            updated_by,
            default_billing_period_ms: 2_592_000_000,
            max_renewal_months: 120,
            platform_fee_bps: 250,
            ecosystem_fee_bps: 250,
            non_platform_platform_to_creator_bps: 0,
            non_platform_platform_to_treasury_bps: 10_000,
            timestamp: 1_700_000_000_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize SubscriptionConfigUpdatedEvent");
        let json = parse_event_contents("subscription", "SubscriptionConfigUpdatedEvent", &bytes)
            .expect("parse SubscriptionConfigUpdatedEvent");
        assert_eq!(
            json["default_billing_period_ms"].as_u64(),
            Some(2_592_000_000)
        );
        assert_eq!(json["platform_fee_bps"], 250);
        assert_eq!(json["ecosystem_fee_bps"], 250);
        assert_eq!(json["non_platform_platform_to_treasury_bps"], 10_000);
    }

    #[test]
    fn platform_config_updated_bcs_roundtrip_uses_five_config_values() {
        let updated_by = AccountAddress::from_hex_literal(
            "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
        )
        .unwrap();
        let ev = BcsPlatformConfigUpdatedEvent {
            updated_by,
            max_reasoning_length: 2000,
            max_cover_photo_url_length: 2048,
            max_media_previews: 10,
            max_badge_name_length: 100,
            max_badge_description_length: 500,
            timestamp: 1_700_000_000_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize PlatformConfigUpdatedEvent");
        let json = parse_event_contents("platform", "PlatformConfigUpdatedEvent", &bytes)
            .expect("parse PlatformConfigUpdatedEvent");
        assert_eq!(json["max_reasoning_length"], 2000);
        assert_eq!(json["max_cover_photo_url_length"], 2048);
        assert_eq!(json["max_media_previews"], 10);
        assert_eq!(json["max_badge_name_length"], 100);
        assert_eq!(json["max_badge_description_length"], 500);
        assert!(json.get("max_media_preview_url_length").is_none());
        assert!(json.get("max_badge_media_url_length").is_none());
        assert!(json.get("max_badge_icon_url_length").is_none());
    }

    #[test]
    fn mydata_config_updated_bcs_roundtrip() {
        let updated_by = AccountAddress::from_hex_literal(
            "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
        )
        .unwrap();
        let ev = BcsMyDataConfigUpdatedEvent {
            updated_by,
            marketplace_enabled: true,
            max_tags: 10,
            max_subscription_days: 365,
            max_free_access_grants: 100_000,
            max_encryption_id_bytes: 1024,
            p2p_platform_fee_bps: 250,
            p2p_ecosystem_fee_bps: 250,
            mydata_marketplace_platform_fee_bps: 250,
            mydata_marketplace_ecosystem_fee_bps: 250,
            non_platform_platform_to_creator_bps: 0,
            non_platform_platform_to_treasury_bps: 10_000,
            timestamp: 1_700_000_000_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize MyDataConfigUpdatedEvent");
        let json = parse_event_contents("mydata", "MyDataConfigUpdatedEvent", &bytes)
            .expect("parse MyDataConfigUpdatedEvent");
        assert_eq!(json["marketplace_enabled"], true);
        assert_eq!(json["p2p_platform_fee_bps"], 250);
        assert_eq!(json["mydata_marketplace_ecosystem_fee_bps"], 250);
        assert_eq!(json["non_platform_platform_to_treasury_bps"], 10_000);
    }

    #[test]
    fn spt_config_updated_bcs_roundtrip() {
        let updated_by = AccountAddress::from_hex_literal(
            "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
        )
        .unwrap();
        let ev = BcsSptConfigUpdatedEvent {
            updated_by,
            timestamp: 1_700_000_000_000,
            total_fee_bps: 150,
            trading_creator_fee_bps: 100,
            trading_platform_fee_bps: 25,
            trading_treasury_fee_bps: 25,
            reservation_total_fee_bps: 150,
            reservation_creator_fee_bps: 100,
            reservation_platform_fee_bps: 25,
            reservation_treasury_fee_bps: 25,
            base_price: 100_000_000,
            quadratic_coefficient: 100_000,
            max_hold_percent_bps: 500,
            post_threshold: 1_000_000_000_000,
            profile_threshold: 10_000_000_000_000,
            max_individual_reservation_bps: 2000,
            max_reservers_per_pool: 1000,
            non_platform_platform_to_creator_bps: 5000,
            non_platform_platform_to_treasury_bps: 5000,
            trading_enabled: true,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize ConfigUpdatedEvent");
        let json = parse_event_contents("social_proof_tokens", "ConfigUpdatedEvent", &bytes)
            .expect("parse ConfigUpdatedEvent");
        assert_eq!(json["total_fee_bps"], 150);
        assert_eq!(json["base_price"], 100_000_000);
        assert_eq!(json["trading_enabled"], true);
        assert_eq!(json["timestamp"].as_u64(), Some(1_700_000_000_000));
    }

    #[test]
    fn token_swapped_event_bcs_roundtrip() {
        let source = AccountAddress::from_hex_literal("0x11").unwrap();
        let dest = AccountAddress::from_hex_literal("0x22").unwrap();
        let trader = AccountAddress::from_hex_literal("0x33").unwrap();
        let ev = BcsTokenSwappedEvent {
            source_pool_id: source,
            dest_pool_id: dest,
            trader,
            sell_amount: 1_000,
            dest_amount: 2_000,
            sell_myso_gross: 300,
            buy_myso_gross: 280,
            sell_fee_amount: 15,
            buy_fee_amount: 14,
            sell_creator_fee: 10,
            sell_platform_fee: 3,
            sell_treasury_fee: 2,
            buy_creator_fee: 9,
            buy_platform_fee: 3,
            buy_treasury_fee: 2,
            leftover_myso: 6,
            source_new_price: 100_000,
            dest_new_price: 200_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize TokenSwappedEvent");
        let json = parse_event_contents("social_proof_tokens", "TokenSwappedEvent", &bytes)
            .expect("parse TokenSwappedEvent");
        assert_eq!(json["sell_amount"], 1_000);
        assert_eq!(json["dest_amount"], 2_000);
        assert_eq!(json["leftover_myso"], 6);
        assert_eq!(json["source_new_price"], 100_000);
        assert_eq!(json["dest_new_price"], 200_000);
        assert!(json["source_pool_id"].as_str().unwrap().contains("11"));
        assert!(json["dest_pool_id"].as_str().unwrap().contains("22"));

        let alias = parse_event_contents("social_proof_tokens", "SwapEvent", &bytes)
            .expect("parse SwapEvent alias");
        assert_eq!(alias["sell_amount"], 1_000);
    }

    #[test]
    fn token_transferred_event_bcs_roundtrip() {
        let pool = AccountAddress::from_hex_literal("0xaa").unwrap();
        let from = AccountAddress::from_hex_literal("0xbb").unwrap();
        let to = AccountAddress::from_hex_literal("0xcc").unwrap();
        let ev = BcsTokenTransferredEvent {
            pool_id: pool,
            from,
            to,
            amount: 42_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize TokenTransferredEvent");
        let json = parse_event_contents("social_proof_tokens", "TokenTransferredEvent", &bytes)
            .expect("parse TokenTransferredEvent");
        assert_eq!(json["amount"], 42_000);
        assert!(json["pool_id"].as_str().unwrap().contains("aa"));
        assert!(json["from"].as_str().unwrap().contains("bb"));
        assert!(json["to"].as_str().unwrap().contains("cc"));

        let alias = parse_event_contents("social_proof_tokens", "TransferEvent", &bytes)
            .expect("parse TransferEvent alias");
        assert_eq!(alias["amount"], 42_000);
    }
}
