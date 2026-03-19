// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewPocAnalysisResult, NewPocBadge, NewPocConfiguration, NewPocDispute, NewPocDisputeVote,
    NewPocRevenueRedirection, DISPUTE_STATUS_VOTING,
};

fn transaction_id_from_event_id(event_id: &str) -> String {
    event_id.split(':').next().unwrap_or(event_id).to_string()
}

fn extract_u64(v: &serde_json::Value) -> u64 {
    v.as_u64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        .unwrap_or(0)
}

#[derive(Debug, Deserialize)]
struct AnalysisSubmittedEvent {
    post_id: String,
    media_type: u8,
    similarity_detected: bool,
    #[serde(deserialize_with = "deserialize_u64")]
    highest_similarity_score: u64,
    oracle_address: String,
    #[serde(deserialize_with = "deserialize_u64")]
    timestamp: u64,
    #[serde(default)]
    reasoning: Option<String>,
    #[serde(default)]
    evidence_urls: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct PocBadgeIssuedEvent {
    badge_id: String,
    post_id: String,
    media_type: u8,
    issued_by: String,
    #[serde(deserialize_with = "deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct RevenueRedirectionActivatedEvent {
    redirection_id: String,
    accused_post_id: String,
    original_post_id: String,
    #[serde(deserialize_with = "deserialize_u64")]
    redirect_percentage: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    similarity_score: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct PocDisputeSubmittedEvent {
    dispute_id: String,
    post_id: String,
    disputer: String,
    dispute_type: u8,
    #[serde(deserialize_with = "deserialize_u64")]
    stake_amount: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    voting_start_epoch: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    voting_end_epoch: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct DisputeVoteCastEvent {
    dispute_id: String,
    voter: String,
    vote_choice: u8,
    #[serde(deserialize_with = "deserialize_u64")]
    stake_amount: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct PocDisputeResolvedEvent {
    dispute_id: String,
    post_id: String,
    resolution: u8,
    winning_side: u8,
    #[serde(deserialize_with = "deserialize_u64")]
    total_winning_stake: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    total_losing_stake: u64,
    badge_revoked: bool,
    redirection_removed: bool,
    #[serde(deserialize_with = "deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct VotingRewardClaimedEvent {
    dispute_id: String,
    voter: String,
    #[serde(deserialize_with = "deserialize_u64")]
    reward_amount: u64,
}

#[derive(Debug, Deserialize)]
struct PocConfigUpdatedEvent {
    updated_by: String,
    oracle_address: String,
    #[serde(deserialize_with = "deserialize_u64")]
    image_threshold: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    video_threshold: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    audio_threshold: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    revenue_redirect_percentage: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    dispute_cost: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    dispute_protocol_fee: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    min_vote_stake: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    max_vote_stake: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    voting_duration_epochs: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    max_reasoning_length: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    max_evidence_urls: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    max_votes_per_dispute: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    timestamp: u64,
}

fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum U64OrStr {
        U64(u64),
        Str(String),
    }
    match U64OrStr::deserialize(deserializer)? {
        U64OrStr::U64(n) => Ok(n),
        U64OrStr::Str(s) => s.parse().map_err(D::Error::custom),
    }
}

pub fn handle_poc_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let tx_id = transaction_id_from_event_id(event_id);
    match event_name {
        "AnalysisSubmittedEvent" => process_analysis_submitted_event(data, &tx_id),
        "PoCBadgeIssuedEvent" | "PocBadgeIssuedEvent" | "BadgeIssuedEvent" => {
            process_poc_badge_issued_event(data, &tx_id)
        }
        "RevenueRedirectionActivatedEvent" => {
            process_revenue_redirection_activated_event(data, &tx_id)
        }
        "PoCDisputeSubmittedEvent" | "PocDisputeSubmittedEvent" | "DisputeSubmittedEvent" => {
            process_poc_dispute_submitted_event(data, &tx_id)
        }
        "DisputeVoteCastEvent" | "VoteCastEvent" => process_dispute_vote_cast_event(data, &tx_id),
        "PoCDisputeResolvedEvent" | "PocDisputeResolvedEvent" | "DisputeResolvedEvent" => {
            process_poc_dispute_resolved_event(data)
        }
        "VotingRewardClaimedEvent" | "RewardClaimedEvent" => {
            process_voting_reward_claimed_event(data)
        }
        "PoCConfigUpdatedEvent" | "PocConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
            process_poc_config_updated_event(data, &tx_id)
        }
        "TokenPoolSyncNeededEvent" => process_token_pool_sync_needed_event(data),
        _ => None,
    }
}

fn process_analysis_submitted_event(
    data: &serde_json::Value,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: AnalysisSubmittedEvent = serde_json::from_value(data.clone()).ok()?;
    if ev.post_id.is_empty() || ev.oracle_address.is_empty() {
        return None;
    }
    if ev.media_type != 1 && ev.media_type != 2 && ev.media_type != 3 {
        return None;
    }
    let evidence_urls_json = ev
        .evidence_urls
        .as_ref()
        .map(|urls| serde_json::to_value(urls).unwrap_or(serde_json::Value::Null));

    let analysis = NewPocAnalysisResult {
        post_id: ev.post_id.clone(),
        media_type: ev.media_type as i16,
        similarity_detected: ev.similarity_detected,
        highest_similarity_score: ev.highest_similarity_score as i64,
        oracle_address: ev.oracle_address.clone(),
        original_creator: None,
        analysis_timestamp: ev.timestamp as i64,
        transaction_id: tx_id.to_string(),
        reasoning: ev.reasoning.clone(),
        evidence_urls: evidence_urls_json.clone(),
    };

    let post_update = SocialEventRow::PostPocUpdate {
        post_id: ev.post_id,
        poc_reasoning: ev.reasoning,
        poc_evidence_urls: evidence_urls_json,
        poc_similarity_score: Some(ev.highest_similarity_score as i64),
        poc_media_type: Some(ev.media_type as i16),
        poc_oracle_address: Some(ev.oracle_address),
        poc_analyzed_at: Some(ev.timestamp as i64),
    };

    Some(vec![
        SocialEventRow::PocAnalysisResult(analysis),
        post_update,
    ])
}

fn process_poc_badge_issued_event(
    data: &serde_json::Value,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PocBadgeIssuedEvent = serde_json::from_value(data.clone()).ok()?;
    if ev.badge_id.is_empty() || ev.post_id.is_empty() || ev.issued_by.is_empty() {
        return None;
    }
    if ev.media_type != 1 && ev.media_type != 2 && ev.media_type != 3 {
        return None;
    }
    let badge = NewPocBadge {
        badge_id: ev.badge_id,
        post_id: ev.post_id,
        media_type: ev.media_type as i16,
        issued_by: ev.issued_by,
        issued_at: ev.timestamp as i64,
        revoked: false,
        revoked_at: None,
        transaction_id: tx_id.to_string(),
    };
    Some(vec![SocialEventRow::PocBadge(badge)])
}

fn process_revenue_redirection_activated_event(
    data: &serde_json::Value,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: RevenueRedirectionActivatedEvent = serde_json::from_value(data.clone()).ok()?;
    if ev.redirection_id.is_empty()
        || ev.accused_post_id.is_empty()
        || ev.original_post_id.is_empty()
    {
        return None;
    }
    if ev.redirect_percentage > 100 || ev.similarity_score > 100 {
        return None;
    }
    let redirection = NewPocRevenueRedirection {
        redirection_id: ev.redirection_id.clone(),
        accused_post_id: ev.accused_post_id.clone(),
        original_post_id: ev.original_post_id.clone(),
        redirect_percentage: ev.redirect_percentage as i64,
        similarity_score: ev.similarity_score as i64,
        created_at: ev.timestamp as i64,
        removed: false,
        removed_at: None,
        transaction_id: tx_id.to_string(),
    };

    let post_update = SocialEventRow::PostRevenueRedirectUpdate {
        post_id: ev.accused_post_id,
        revenue_redirect_to: ev.original_post_id,
        revenue_redirect_percentage: ev.redirect_percentage as i64,
    };

    Some(vec![
        SocialEventRow::PocRevenueRedirection(redirection),
        post_update,
    ])
}

fn process_poc_dispute_submitted_event(
    data: &serde_json::Value,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PocDisputeSubmittedEvent = serde_json::from_value(data.clone()).ok()?;
    if ev.dispute_id.is_empty() || ev.post_id.is_empty() || ev.disputer.is_empty() {
        return None;
    }
    if ev.voting_start_epoch >= ev.voting_end_epoch {
        return None;
    }
    let evidence = data
        .get("evidence")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let dispute = NewPocDispute {
        dispute_id: ev.dispute_id,
        post_id: ev.post_id,
        disputer: ev.disputer,
        dispute_type: ev.dispute_type as i16,
        evidence,
        status: DISPUTE_STATUS_VOTING,
        stake_amount: ev.stake_amount as i64,
        voting_start_epoch: ev.voting_start_epoch as i64,
        voting_end_epoch: ev.voting_end_epoch as i64,
        resolution: None,
        winning_side: None,
        total_winning_stake: None,
        total_losing_stake: None,
        submitted_at: ev.timestamp as i64,
        resolved_at: None,
        transaction_id: tx_id.to_string(),
    };
    Some(vec![SocialEventRow::PocDispute(dispute)])
}

fn process_dispute_vote_cast_event(
    data: &serde_json::Value,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: DisputeVoteCastEvent = serde_json::from_value(data.clone()).ok()?;
    if ev.dispute_id.is_empty() || ev.voter.is_empty() {
        return None;
    }
    if ev.vote_choice != 1 && ev.vote_choice != 2 {
        return None;
    }
    if ev.stake_amount == 0 {
        return None;
    }
    let vote = NewPocDisputeVote {
        dispute_id: ev.dispute_id,
        voter: ev.voter,
        vote_choice: ev.vote_choice as i16,
        stake_amount: ev.stake_amount as i64,
        voted_at: ev.timestamp as i64,
        reward_claimed: false,
        reward_amount: None,
        transaction_id: tx_id.to_string(),
    };
    Some(vec![SocialEventRow::PocDisputeVote(vote)])
}

fn process_poc_dispute_resolved_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: PocDisputeResolvedEvent = serde_json::from_value(data.clone()).ok()?;
    if ev.dispute_id.is_empty() || ev.post_id.is_empty() {
        return None;
    }
    Some(vec![SocialEventRow::PocDisputeResolved {
        dispute_id: ev.dispute_id,
        post_id: ev.post_id,
        resolution: ev.resolution as i16,
        winning_side: ev.winning_side as i16,
        total_winning_stake: ev.total_winning_stake as i64,
        total_losing_stake: ev.total_losing_stake as i64,
        resolved_at: ev.timestamp as i64,
        badge_revoked: ev.badge_revoked,
        redirection_removed: ev.redirection_removed,
    }])
}

fn process_voting_reward_claimed_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: VotingRewardClaimedEvent = serde_json::from_value(data.clone()).ok()?;
    if ev.dispute_id.is_empty() || ev.voter.is_empty() {
        return None;
    }
    Some(vec![SocialEventRow::PocVoteRewardClaimed {
        dispute_id: ev.dispute_id,
        voter: ev.voter,
        reward_amount: ev.reward_amount as i64,
    }])
}

fn process_poc_config_updated_event(
    data: &serde_json::Value,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PocConfigUpdatedEvent = serde_json::from_value(data.clone()).ok()?;
    if ev.updated_by.is_empty() {
        return None;
    }
    if ev.min_vote_stake > ev.max_vote_stake {
        return None;
    }
    let config = NewPocConfiguration {
        image_threshold: ev.image_threshold as i64,
        video_threshold: ev.video_threshold as i64,
        audio_threshold: ev.audio_threshold as i64,
        revenue_redirect_percentage: ev.revenue_redirect_percentage as i64,
        dispute_cost: ev.dispute_cost as i64,
        dispute_protocol_fee: ev.dispute_protocol_fee as i64,
        min_vote_stake: ev.min_vote_stake as i64,
        max_vote_stake: ev.max_vote_stake as i64,
        voting_duration_epochs: ev.voting_duration_epochs as i64,
        max_reasoning_length: ev.max_reasoning_length as i64,
        max_evidence_urls: ev.max_evidence_urls as i64,
        max_votes_per_dispute: ev.max_votes_per_dispute as i64,
        oracle_address: Some(ev.oracle_address),
        updated_by: ev.updated_by,
        updated_at: ev.timestamp as i64,
        transaction_id: tx_id.to_string(),
    };
    Some(vec![SocialEventRow::PocConfiguration(config)])
}

fn process_token_pool_sync_needed_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?;
    let timestamp = extract_u64(data.get("timestamp")?);
    tracing::debug!(
        "TokenPoolSyncNeededEvent for post_id={} timestamp={}",
        post_id,
        timestamp
    );
    None
}
