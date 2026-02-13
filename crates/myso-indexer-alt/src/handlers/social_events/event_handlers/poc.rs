// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;

pub fn handle_poc_event(
    event_name: &str,
    _data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "AnalysisSubmittedEvent" => process_analysis_submitted_event(),
        "PocBadgeIssuedEvent" | "BadgeIssuedEvent" => process_poc_badge_issued_event(),
        "RevenueRedirectionActivatedEvent" => process_revenue_redirection_activated_event(),
        "PocDisputeSubmittedEvent" | "DisputeSubmittedEvent" => {
            process_poc_dispute_submitted_event()
        }
        "DisputeVoteCastEvent" | "VoteCastEvent" => process_dispute_vote_cast_event(),
        "PocDisputeResolvedEvent" | "DisputeResolvedEvent" => process_poc_dispute_resolved_event(),
        "VotingRewardClaimedEvent" | "RewardClaimedEvent" => process_voting_reward_claimed_event(),
        "PocConfigUpdatedEvent" | "ConfigUpdatedEvent" => process_poc_config_updated_event(),
        "TokenPoolSyncNeededEvent" => process_token_pool_sync_needed_event(),
        _ => None,
    }
}

fn process_analysis_submitted_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_poc_badge_issued_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_revenue_redirection_activated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_poc_dispute_submitted_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_dispute_vote_cast_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_poc_dispute_resolved_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_voting_reward_claimed_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_poc_config_updated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_token_pool_sync_needed_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
