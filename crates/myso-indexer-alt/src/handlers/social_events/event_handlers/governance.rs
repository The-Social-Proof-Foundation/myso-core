// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    GovernanceRegistryUpdate, NewAnonymousVote, NewCommunityVote, NewDelegate, NewDelegateRating,
    NewDelegateVote, NewGovernanceEvent, NewGovernanceRegistry, NewNominatedDelegate, NewProposal,
    NewRewardDistribution, NewVoteDecryptionFailure, ProposalUpdateSet,
};
use myso_indexer_alt_social_schema::{
    GOVERNANCE_STATUS_APPROVED, GOVERNANCE_STATUS_COMMUNITY_VOTING, GOVERNANCE_STATUS_IMPLEMENTED,
    GOVERNANCE_STATUS_OWNER_RESCINDED, GOVERNANCE_STATUS_REJECTED,
};

fn de_u64<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        I(u64),
        S(String),
    }
    match V::deserialize(d) {
        Ok(V::I(n)) => Ok(n),
        Ok(V::S(s)) => s.parse().map_err(serde::de::Error::custom),
        Err(e) => Err(e),
    }
}

fn de_u8<'de, D>(d: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        I(u8),
        S(String),
    }
    match V::deserialize(d) {
        Ok(V::I(n)) => Ok(n),
        Ok(V::S(s)) => s.parse().map_err(serde::de::Error::custom),
        Err(e) => Err(e),
    }
}

pub fn handle_governance_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let normalized = event_name.strip_suffix("Event").unwrap_or(event_name);
    match normalized {
        "GovernanceRegistryCreated" => process_governance_registry_created_event(data, event_id),
        "DelegateNominated" => process_delegate_nominated_event(data, event_id),
        "DelegateElected" => process_delegate_elected_event(data, event_id),
        "DelegateVoted" => process_delegate_voted_event(data, event_id),
        "ProposalSubmitted" => process_proposal_submitted_event(data, event_id),
        "DelegateVote" => process_delegate_vote_event(data, event_id),
        "CommunityVote" => process_community_vote_event(data, event_id),
        "ProposalApprovedForVoting" => process_proposal_approved_for_voting_event(data, event_id),
        "ProposalRejected" => process_proposal_rejected_event(data, event_id),
        "ProposalRescinded" => process_proposal_rescinded_event(data, event_id),
        "ProposalRejectedByCommunity" => process_proposal_rejected_by_community_event(data, event_id),
        "ProposalApproved" => process_proposal_approved_event(data, event_id),
        "ProposalImplemented" => process_proposal_implemented_event(data, event_id),
        "RewardsDistributed" => process_rewards_distributed_event(data, event_id),
        "AnonymousVote" => process_anonymous_vote_event(data, event_id),
        "VoteDecryptionFailed" => process_vote_decryption_failed_event(data, event_id),
        "GovernanceParametersUpdated" => process_governance_parameters_updated_event(data, event_id),
        _ => None,
    }
}

fn process_governance_registry_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        registry_id: String,
        #[serde(deserialize_with = "de_u8")]
        registry_type: u8,
        #[serde(deserialize_with = "de_u64")]
        delegate_count: u64,
        #[serde(deserialize_with = "de_u64")]
        delegate_term_epochs: u64,
        #[serde(deserialize_with = "de_u64")]
        proposal_submission_cost: u64,
        #[serde(deserialize_with = "de_u64")]
        max_votes_per_user: u64,
        #[serde(deserialize_with = "de_u64")]
        quadratic_base_cost: u64,
        #[serde(deserialize_with = "de_u64")]
        voting_period_ms: u64,
        #[serde(deserialize_with = "de_u64")]
        quorum_votes: u64,
        #[serde(deserialize_with = "de_u64")]
        updated_at: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now();
    let reg = NewGovernanceRegistry {
        registry_type: ev.registry_type as i16,
        registry_id: ev.registry_id,
        delegate_count: ev.delegate_count as i64,
        delegate_term_epochs: ev.delegate_term_epochs as i64,
        proposal_submission_cost: ev.proposal_submission_cost as i64,
        min_on_chain_age_days: 0,
        max_votes_per_user: ev.max_votes_per_user as i64,
        quadratic_base_cost: ev.quadratic_base_cost as i64,
        voting_period_ms: ev.voting_period_ms as i64,
        quorum_votes: ev.quorum_votes as i64,
        updated_at: ev.updated_at as i64,
        transaction_id: event_id.to_string(),
    };
    let gov_ev = NewGovernanceEvent {
        event_type: "GovernanceRegistryCreatedEvent".to_string(),
        registry_type: ev.registry_type as i16,
        event_data: data.clone(),
        event_id: event_id.to_string(),
        created_at: now,
        anonymous_voting_related: None,
    };
    Some(vec![
        SocialEventRow::GovernanceRegistry(reg),
        SocialEventRow::GovernanceEvent(gov_ev),
    ])
}

fn process_delegate_nominated_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        nominee_address: String,
        #[serde(deserialize_with = "de_u8")]
        registry_type: u8,
        #[serde(deserialize_with = "de_u64")]
        scheduled_term_start_epoch: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now().timestamp() as i64;
    let nominee = NewNominatedDelegate {
        address: ev.nominee_address,
        registry_type: ev.registry_type as i16,
        upvotes: 0,
        downvotes: 0,
        scheduled_term_start_epoch: ev.scheduled_term_start_epoch as i64,
        nomination_time: now,
        status: 0,
        transaction_id: event_id.to_string(),
    };
    let gov_ev = NewGovernanceEvent {
        event_type: "DelegateNominatedEvent".to_string(),
        registry_type: ev.registry_type as i16,
        event_data: data.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
        anonymous_voting_related: None,
    };
    Some(vec![
        SocialEventRow::NominatedDelegate(nominee),
        SocialEventRow::GovernanceEvent(gov_ev),
    ])
}

fn process_delegate_elected_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        delegate_address: String,
        #[serde(deserialize_with = "de_u8")]
        registry_type: u8,
        #[serde(deserialize_with = "de_u64")]
        term_start: u64,
        #[serde(deserialize_with = "de_u64")]
        term_end: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now().timestamp_millis() as i64;
    let delegate = NewDelegate {
        address: ev.delegate_address,
        registry_type: ev.registry_type as i16,
        upvotes: 0,
        downvotes: 0,
        proposals_reviewed: 0,
        proposals_submitted: 0,
        sided_winning_proposals: 0,
        sided_losing_proposals: 0,
        term_start: ev.term_start as i64,
        term_end: ev.term_end as i64,
        is_active: true,
        created_at: now,
        updated_at: now,
        transaction_id: event_id.to_string(),
    };
    let gov_ev = NewGovernanceEvent {
        event_type: "DelegateElectedEvent".to_string(),
        registry_type: ev.registry_type as i16,
        event_data: data.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
        anonymous_voting_related: None,
    };
    Some(vec![
        SocialEventRow::Delegate(delegate),
        SocialEventRow::GovernanceEvent(gov_ev),
    ])
}

fn process_delegate_voted_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        target_address: String,
        voter: String,
        #[serde(deserialize_with = "de_u8")]
        registry_type: u8,
        is_active_delegate: bool,
        upvote: bool,
        #[serde(rename = "new_upvote_count", deserialize_with = "de_u64")]
        _new_upvote_count: u64,
        #[serde(rename = "new_downvote_count", deserialize_with = "de_u64")]
        _new_downvote_count: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now().timestamp() as i64;
    let rating = NewDelegateRating {
        target_address: ev.target_address,
        voter_address: ev.voter,
        registry_type: ev.registry_type as i16,
        is_active_delegate: ev.is_active_delegate,
        upvote: ev.upvote,
        rated_at: now,
        transaction_id: event_id.to_string(),
    };
    let gov_ev = NewGovernanceEvent {
        event_type: "DelegateVotedEvent".to_string(),
        registry_type: ev.registry_type as i16,
        event_data: data.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
        anonymous_voting_related: None,
    };
    Some(vec![
        SocialEventRow::DelegateRating(rating),
        SocialEventRow::GovernanceEvent(gov_ev),
    ])
}

fn process_proposal_submitted_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        title: String,
        description: String,
        #[serde(deserialize_with = "de_u8")]
        proposal_type: u8,
        reference_id: Option<String>,
        metadata_json: Option<String>,
        submitter: String,
        #[serde(deserialize_with = "de_u64")]
        reward_amount: u64,
        #[serde(deserialize_with = "de_u64")]
        submission_time: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let metadata_json = ev
        .metadata_json
        .and_then(|s| serde_json::from_str(&s).ok());
    let proposal = NewProposal {
        id: ev.proposal_id.clone(),
        title: ev.title,
        description: ev.description,
        proposal_type: ev.proposal_type as i16,
        reference_id: ev.reference_id,
        metadata_json,
        submitter: ev.submitter,
        submission_time: ev.submission_time as i64,
        delegate_approval_count: 0,
        delegate_rejection_count: 0,
        community_votes_for: 0,
        community_votes_against: 0,
        status: 0,
        voting_start_time: None,
        voting_end_time: None,
        reward_pool: ev.reward_amount as i64,
        transaction_id: event_id.to_string(),
    };
    let gov_ev = NewGovernanceEvent {
        event_type: "ProposalSubmittedEvent".to_string(),
        registry_type: ev.proposal_type as i16,
        event_data: data.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
        anonymous_voting_related: None,
    };
    Some(vec![
        SocialEventRow::Proposal(proposal),
        SocialEventRow::GovernanceEvent(gov_ev),
    ])
}

fn process_delegate_vote_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        delegate_address: String,
        approve: bool,
        #[serde(deserialize_with = "de_u64")]
        vote_time: u64,
        reason: Option<String>,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let vote = NewDelegateVote {
        proposal_id: ev.proposal_id.clone(),
        delegate_address: ev.delegate_address,
        approve: ev.approve,
        vote_time: ev.vote_time as i64,
        reason: ev.reason,
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::DelegateVote(vote),
        SocialEventRow::GovernanceEventFromProposal {
            proposal_id: ev.proposal_id,
            event_type: "DelegateVoteEvent".to_string(),
            event_data: data.clone(),
            event_id: event_id.to_string(),
            anonymous_voting_related: None,
        },
    ])
}

fn process_community_vote_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        voter: String,
        #[serde(deserialize_with = "de_u64")]
        vote_weight: u64,
        approve: bool,
        #[serde(deserialize_with = "de_u64")]
        vote_time: u64,
        #[serde(deserialize_with = "de_u64")]
        vote_cost: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let vote = NewCommunityVote {
        proposal_id: ev.proposal_id.clone(),
        voter_address: ev.voter,
        vote_weight: ev.vote_weight as i64,
        approve: ev.approve,
        vote_time: ev.vote_time as i64,
        vote_cost: ev.vote_cost as i64,
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::CommunityVote(vote),
        SocialEventRow::GovernanceEventFromProposal {
            proposal_id: ev.proposal_id,
            event_type: "CommunityVoteEvent".to_string(),
            event_data: data.clone(),
            event_id: event_id.to_string(),
            anonymous_voting_related: None,
        },
    ])
}

fn process_proposal_approved_for_voting_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        #[serde(deserialize_with = "de_u64")]
        voting_start_time: u64,
        #[serde(deserialize_with = "de_u64")]
        voting_end_time: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let set = ProposalUpdateSet {
        status: Some(GOVERNANCE_STATUS_COMMUNITY_VOTING),
        voting_start_time: Some(ev.voting_start_time as i64),
        voting_end_time: Some(ev.voting_end_time as i64),
        reward_pool: None,
        community_votes_for: None,
        community_votes_against: None,
        rescind_time: None,
        implementation_time: None,
        implemented_description: None,
    };
    Some(vec![SocialEventRow::ProposalUpdate {
        proposal_id: ev.proposal_id,
        set,
        governance_event: Some((
            "ProposalApprovedForVotingEvent".to_string(),
            data.clone(),
            event_id.to_string(),
        )),
    }])
}

fn process_proposal_rejected_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let set = ProposalUpdateSet {
        status: Some(GOVERNANCE_STATUS_REJECTED),
        voting_start_time: None,
        voting_end_time: None,
        reward_pool: Some(0),
        community_votes_for: None,
        community_votes_against: None,
        rescind_time: None,
        implementation_time: None,
        implemented_description: None,
    };
    Some(vec![SocialEventRow::ProposalUpdate {
        proposal_id: ev.proposal_id,
        set,
        governance_event: Some((
            "ProposalRejectedEvent".to_string(),
            data.clone(),
            event_id.to_string(),
        )),
    }])
}

fn process_proposal_rescinded_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        submitter: String,
        #[serde(deserialize_with = "de_u64")]
        rescind_time: u64,
        #[serde(deserialize_with = "de_u64")]
        refund_amount: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let set = ProposalUpdateSet {
        status: Some(GOVERNANCE_STATUS_OWNER_RESCINDED),
        voting_start_time: None,
        voting_end_time: None,
        reward_pool: Some(0),
        community_votes_for: None,
        community_votes_against: None,
        rescind_time: Some(ev.rescind_time as i64),
        implementation_time: None,
        implemented_description: None,
    };
    let rows = vec![
        SocialEventRow::ProposalUpdate {
            proposal_id: ev.proposal_id.clone(),
            set,
            governance_event: Some((
                "ProposalRescindedEvent".to_string(),
                data.clone(),
                event_id.to_string(),
            )),
        },
        SocialEventRow::RewardDistribution(NewRewardDistribution {
            proposal_id: ev.proposal_id,
            recipient_address: ev.submitter,
            amount: ev.refund_amount as i64,
            distribution_time: ev.rescind_time as i64,
            distribution_type: Some("rescind_refund".to_string()),
            transaction_id: event_id.to_string(),
        }),
    ];
    Some(rows)
}

fn process_proposal_rejected_by_community_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        #[serde(deserialize_with = "de_u64")]
        votes_for: u64,
        #[serde(deserialize_with = "de_u64")]
        votes_against: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let set = ProposalUpdateSet {
        status: Some(GOVERNANCE_STATUS_REJECTED),
        voting_start_time: None,
        voting_end_time: None,
        reward_pool: None,
        community_votes_for: Some(ev.votes_for as i64),
        community_votes_against: Some(ev.votes_against as i64),
        rescind_time: None,
        implementation_time: None,
        implemented_description: None,
    };
    Some(vec![SocialEventRow::ProposalUpdate {
        proposal_id: ev.proposal_id,
        set,
        governance_event: Some((
            "ProposalRejectedByCommunityEvent".to_string(),
            data.clone(),
            event_id.to_string(),
        )),
    }])
}

fn process_proposal_approved_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        #[serde(deserialize_with = "de_u64")]
        votes_for: u64,
        #[serde(deserialize_with = "de_u64")]
        votes_against: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let set = ProposalUpdateSet {
        status: Some(GOVERNANCE_STATUS_APPROVED),
        voting_start_time: None,
        voting_end_time: None,
        reward_pool: None,
        community_votes_for: Some(ev.votes_for as i64),
        community_votes_against: Some(ev.votes_against as i64),
        rescind_time: None,
        implementation_time: None,
        implemented_description: None,
    };
    Some(vec![SocialEventRow::ProposalUpdate {
        proposal_id: ev.proposal_id,
        set,
        governance_event: Some((
            "ProposalApprovedEvent".to_string(),
            data.clone(),
            event_id.to_string(),
        )),
    }])
}

fn process_proposal_implemented_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        #[serde(deserialize_with = "de_u64")]
        implementation_time: u64,
        description: Option<String>,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let set = ProposalUpdateSet {
        status: Some(GOVERNANCE_STATUS_IMPLEMENTED),
        voting_start_time: None,
        voting_end_time: None,
        reward_pool: None,
        community_votes_for: None,
        community_votes_against: None,
        rescind_time: None,
        implementation_time: Some(ev.implementation_time as i64),
        implemented_description: ev.description,
    };
    Some(vec![SocialEventRow::ProposalUpdate {
        proposal_id: ev.proposal_id,
        set,
        governance_event: Some((
            "ProposalImplementedEvent".to_string(),
            data.clone(),
            event_id.to_string(),
        )),
    }])
}

fn process_rewards_distributed_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        #[serde(deserialize_with = "de_u64")]
        total_reward: u64,
        #[serde(deserialize_with = "de_u64")]
        recipient_count: u64,
        #[serde(deserialize_with = "de_u64")]
        distribution_time: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let dist = NewRewardDistribution {
        proposal_id: ev.proposal_id.clone(),
        recipient_address: format!("aggregate_{}", ev.proposal_id),
        amount: ev.total_reward as i64,
        distribution_time: ev.distribution_time as i64,
        distribution_type: Some(format!("aggregate_{}_recipients", ev.recipient_count)),
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::RewardDistribution(dist),
        SocialEventRow::GovernanceEventFromProposal {
            proposal_id: ev.proposal_id,
            event_type: "RewardsDistributedEvent".to_string(),
            event_data: data.clone(),
            event_id: event_id.to_string(),
            anonymous_voting_related: None,
        },
    ])
}

fn process_anonymous_vote_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        voter: String,
        #[serde(deserialize_with = "de_u64")]
        vote_time: u64,
        #[serde(default)]
        encrypted_vote_data: Option<Vec<u8>>,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let vote = NewAnonymousVote {
        proposal_id: ev.proposal_id.clone(),
        voter_address: ev.voter,
        encrypted_vote_data: ev.encrypted_vote_data,
        submitted_at: ev.vote_time as i64,
        decryption_status: 0,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };
    Some(vec![
        SocialEventRow::AnonymousVote(vote),
        SocialEventRow::GovernanceEventFromProposal {
            proposal_id: ev.proposal_id,
            event_type: "AnonymousVoteEvent".to_string(),
            event_data: data.clone(),
            event_id: event_id.to_string(),
            anonymous_voting_related: Some(true),
        },
    ])
}

fn process_vote_decryption_failed_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        proposal_id: String,
        voter: String,
        failure_reason: String,
        #[serde(deserialize_with = "de_u64")]
        timestamp: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let failure = NewVoteDecryptionFailure {
        proposal_id: ev.proposal_id.clone(),
        voter_address: ev.voter,
        failure_reason: ev.failure_reason,
        attempted_at: ev.timestamp as i64,
        encrypted_vote_length: None,
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::VoteDecryptionFailure(failure),
        SocialEventRow::GovernanceEventFromProposal {
            proposal_id: ev.proposal_id,
            event_type: "VoteDecryptionFailedEvent".to_string(),
            event_data: data.clone(),
            event_id: event_id.to_string(),
            anonymous_voting_related: Some(true),
        },
    ])
}

fn process_governance_parameters_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct Ev {
        #[serde(deserialize_with = "de_u8")]
        registry_type: u8,
        #[serde(deserialize_with = "de_u64")]
        delegate_count: u64,
        #[serde(deserialize_with = "de_u64")]
        delegate_term_epochs: u64,
        #[serde(deserialize_with = "de_u64")]
        proposal_submission_cost: u64,
        #[serde(deserialize_with = "de_u64")]
        max_votes_per_user: u64,
        #[serde(deserialize_with = "de_u64")]
        quadratic_base_cost: u64,
        #[serde(deserialize_with = "de_u64")]
        voting_period_ms: u64,
        #[serde(deserialize_with = "de_u64")]
        quorum_votes: u64,
        #[serde(deserialize_with = "de_u64")]
        timestamp: u64,
    }
    let ev: Ev = serde_json::from_value(data.clone()).ok()?;
    let update = GovernanceRegistryUpdate {
        registry_type: ev.registry_type as i16,
        delegate_count: ev.delegate_count as i64,
        delegate_term_epochs: ev.delegate_term_epochs as i64,
        proposal_submission_cost: ev.proposal_submission_cost as i64,
        max_votes_per_user: ev.max_votes_per_user as i64,
        quadratic_base_cost: ev.quadratic_base_cost as i64,
        voting_period_ms: ev.voting_period_ms as i64,
        quorum_votes: ev.quorum_votes as i64,
        updated_at: ev.timestamp as i64,
        transaction_id: event_id.to_string(),
    };
    let gov_ev = NewGovernanceEvent {
        event_type: "GovernanceParametersUpdatedEvent".to_string(),
        registry_type: ev.registry_type as i16,
        event_data: data.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
        anonymous_voting_related: None,
    };
    Some(vec![
        SocialEventRow::GovernanceRegistryUpdate(update),
        SocialEventRow::GovernanceEvent(gov_ev),
    ])
}
