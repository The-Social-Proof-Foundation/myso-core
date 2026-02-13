// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! BCS-compatible event structs and parsing for myso-social Move events.
//! Field order must match the Move struct definitions exactly.

use move_core_types::account_address::AccountAddress;
use serde::Deserialize;

fn addr_to_string(addr: &AccountAddress) -> String {
    format!("0x{}", hex::encode(addr))
}

#[derive(Debug, Deserialize)]
pub struct BcsProfileCreatedEvent {
    profile_id: AccountAddress,
    display_name: String,
    username: String,
    bio: String,
    profile_picture: Option<String>,
    cover_photo: Option<String>,
    owner: AccountAddress,
    #[allow(dead_code)]
    created_at: u64,
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

pub fn parse_event_contents(contents: &[u8]) -> Option<serde_json::Value> {
    if let Ok(ev) = bcs::from_bytes::<BcsProfileCreatedEvent>(contents) {
        return Some(serde_json::json!({
            "profile_id": addr_to_string(&ev.profile_id),
            "display_name": ev.display_name,
            "username": ev.username,
            "bio": ev.bio,
            "profile_picture": ev.profile_picture,
            "cover_photo": ev.cover_photo,
            "owner": addr_to_string(&ev.owner),
            "owner_address": addr_to_string(&ev.owner),
            "created_at": ev.created_at,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsFollowEvent>(contents) {
        return serde_json::json!({
            "follower": ev.follower(),
            "following": ev.following(),
        })
        .into();
    }

    if let Ok(ev) = bcs::from_bytes::<BcsUnfollowEvent>(contents) {
        return serde_json::json!({
            "follower": ev.follower(),
            "unfollowed": ev.unfollowed(),
        })
        .into();
    }

    if let Ok(ev) = bcs::from_bytes::<BcsUserBlockEvent>(contents) {
        return serde_json::json!({
            "blocker": ev.blocker(),
            "blocked": ev.blocked(),
        })
        .into();
    }

    if let Ok(ev) = bcs::from_bytes::<BcsUserUnblockEvent>(contents) {
        return serde_json::json!({
            "blocker": ev.blocker(),
            "unblocked": ev.unblocked(),
        })
        .into();
    }

    if let Ok(ev) = bcs::from_bytes::<BcsGovernanceRegistryCreatedEvent>(contents) {
        return Some(serde_json::json!({
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
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsDelegateNominatedEvent>(contents) {
        return Some(serde_json::json!({
            "nominee_address": addr_to_string(&ev.nominee_address),
            "registry_type": ev.registry_type,
            "scheduled_term_start_epoch": ev.scheduled_term_start_epoch,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsDelegateElectedEvent>(contents) {
        return Some(serde_json::json!({
            "delegate_address": addr_to_string(&ev.delegate_address),
            "registry_type": ev.registry_type,
            "term_start": ev.term_start,
            "term_end": ev.term_end,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsDelegateVotedEvent>(contents) {
        return Some(serde_json::json!({
            "target_address": addr_to_string(&ev.target_address),
            "voter": addr_to_string(&ev.voter),
            "registry_type": ev.registry_type,
            "is_active_delegate": ev.is_active_delegate,
            "upvote": ev.upvote,
            "new_upvote_count": ev.new_upvote_count,
            "new_downvote_count": ev.new_downvote_count,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsProposalSubmittedEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "title": ev.title,
            "description": ev.description,
            "proposal_type": ev.proposal_type,
            "reference_id": ev.reference_id.as_ref().map(addr_to_string),
            "metadata_json": ev.metadata_json,
            "submitter": addr_to_string(&ev.submitter),
            "reward_amount": ev.reward_amount,
            "submission_time": ev.submission_time,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsDelegateVoteEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "delegate_address": addr_to_string(&ev.delegate_address),
            "approve": ev.approve,
            "vote_time": ev.vote_time,
            "reason": ev.reason,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsCommunityVoteEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "voter": addr_to_string(&ev.voter),
            "vote_weight": ev.vote_weight,
            "approve": ev.approve,
            "vote_time": ev.vote_time,
            "vote_cost": ev.vote_cost,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsAnonymousVoteEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "voter": addr_to_string(&ev.voter),
            "vote_time": ev.vote_time,
            "encrypted_vote_data": ev.encrypted_vote_data,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsProposalApprovedForVotingEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "voting_start_time": ev.voting_start_time,
            "voting_end_time": ev.voting_end_time,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsProposalRejectedEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "rejection_time": ev.rejection_time,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsProposalApprovedEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "approval_time": ev.approval_time,
            "votes_for": ev.votes_for,
            "votes_against": ev.votes_against,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsProposalRejectedByCommunityEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "rejection_time": ev.rejection_time,
            "votes_for": ev.votes_for,
            "votes_against": ev.votes_against,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsProposalImplementedEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "implementation_time": ev.implementation_time,
            "description": ev.description,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsRewardsDistributedEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "total_reward": ev.total_reward,
            "recipient_count": ev.recipient_count,
            "distribution_time": ev.distribution_time,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsVoteDecryptionFailedEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "voter": addr_to_string(&ev.voter),
            "failure_reason": ev.failure_reason,
            "timestamp": ev.timestamp,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsProposalRescindedEvent>(contents) {
        return Some(serde_json::json!({
            "proposal_id": addr_to_string(&ev.proposal_id),
            "submitter": addr_to_string(&ev.submitter),
            "rescind_time": ev.rescind_time,
            "refund_amount": ev.refund_amount,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsGovernanceParametersUpdatedEvent>(contents) {
        return Some(serde_json::json!({
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
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsProfileUpdatedEvent>(contents) {
        return Some(serde_json::json!({
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
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsBadgeAssignedEvent>(contents) {
        return Some(serde_json::json!({
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
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsBadgeRevokedEvent>(contents) {
        return Some(serde_json::json!({
            "profile_id": addr_to_string(&ev.profile_id),
            "badge_id": ev.badge_id,
            "platform_id": addr_to_string(&ev.platform_id),
            "revoked_by": addr_to_string(&ev.revoked_by),
            "revoked_at": ev.revoked_at,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsBadgeSelectedEvent>(contents) {
        return Some(serde_json::json!({
            "profile_id": addr_to_string(&ev.profile_id),
            "badge_id": ev.badge_id,
            "selected_by": addr_to_string(&ev.selected_by),
            "selected_at": ev.selected_at,
        }));
    }

    if let Ok(ev) = bcs::from_bytes::<BcsBadgeRemovedEvent>(contents) {
        return Some(serde_json::json!({
            "profile_id": addr_to_string(&ev.profile_id),
            "badge_id": ev.badge_id,
            "platform_id": addr_to_string(&ev.platform_id),
            "removed_by": addr_to_string(&ev.removed_by),
            "removed_at": ev.removed_at,
        }));
    }

    if let Ok(v) = serde_json::from_slice(contents) {
        return Some(v);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_profile_created_event() {
        let contents: Vec<u8> = vec![
            217, 0, 116, 168, 192, 241, 38, 98, 208, 170, 122, 181, 129, 167, 139, 149, 123, 82,
            250, 151, 203, 248, 61, 180, 210, 122, 244, 238, 112, 6, 36, 122, 12, 66, 114, 97, 110,
            100, 111, 110, 32, 83, 104, 97, 119, 7, 98, 114, 97, 110, 100, 111, 110, 36, 87, 101, 98,
            56, 32, 100, 101, 118, 101, 108, 111, 112, 101, 114, 32, 97, 110, 100, 32, 99, 114, 121,
            112, 116, 111, 32, 101, 110, 116, 104, 117, 115, 105, 97, 115, 116, 1, 31, 104, 116, 116,
            112, 115, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47, 112, 114,
            111, 102, 105, 108, 101, 46, 106, 112, 103, 1, 29, 104, 116, 116, 112, 115, 58, 47, 47,
            101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 47, 99, 111, 118, 101, 114, 46, 112,
            110, 103, 156, 200, 104, 135, 157, 106, 255, 74, 171, 250, 160, 27, 141, 86, 246, 73, 253,
            178, 164, 32, 199, 252, 9, 96, 225, 249, 235, 52, 206, 192, 0, 124, 5, 0, 0, 0, 0, 0, 0,
            0,
        ];
        let result = parse_event_contents(&contents);
        assert!(result.is_some(), "parse_event_contents should succeed for ProfileCreatedEvent");
        let json = result.unwrap();
        assert_eq!(json["display_name"], "Brandon Shaw");
        assert_eq!(json["username"], "brandon");
        assert_eq!(json["bio"], "Web8 developer and crypto enthusiast");
        assert_eq!(json["created_at"], 5);
        assert!(json["owner_address"].as_str().unwrap().starts_with("0x"));
    }

    #[test]
    fn test_parse_governance_registry_created_event() {
        let contents: Vec<u8> = vec![
            100, 127, 122, 106, 253, 121, 113, 7, 80, 14, 70, 113, 185, 17, 144, 225, 233, 109, 169,
            54, 79, 166, 56, 144, 151, 211, 15, 115, 163, 148, 80, 156, 0, 3, 0, 0, 0, 0, 0, 0, 0,
            90, 0, 0, 0, 0, 0, 0, 0, 0, 225, 245, 5, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 128, 150,
            152, 0, 0, 0, 0, 0, 0, 132, 12, 36, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 1, 205, 99, 86,
            156, 1, 0, 0,
        ];
        let result = parse_event_contents(&contents);
        assert!(result.is_some(), "parse_event_contents should succeed");
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
}
