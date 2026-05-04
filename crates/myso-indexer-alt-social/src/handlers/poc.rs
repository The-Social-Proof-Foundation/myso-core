// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! **`poc_vault` module:** `PoCBeneficiaryVaultDepositEvent`, `PoCBeneficiaryVaultClaimedEvent`.
//!
//! **Chain ↔ indexer map (see also `handlers/post.rs` for `post.move` social events):**
//! - `AnalysisSubmittedEvent` → `poc_analysis_results` + `posts` PoC columns
//! - `PoCBadgeIssuedEvent` → `poc_badges`
//! - `RevenueRedirectionActivatedEvent` → `poc_revenue_redirections`
//! - `PoCResultAppliedEvent` → `posts.poc_outcome` / `poc_redirection_kind` (via `posts_handler`)
//! - `PoCDisputeSubmittedEvent` → `poc_disputes`
//! - `DisputeVoteCastEvent` → `poc_dispute_votes`
//! - `PoCDisputeResolvedEvent` → updates `poc_disputes`, may clear `posts` / `poc_badges` / `poc_revenue_redirections`
//! - `VotingRewardClaimedEvent` → `poc_dispute_votes.reward_claimed` / `reward_amount`
//! - `PoCConfigUpdatedEvent` → `poc_configuration`
//! - `PoCBeneficiaryVaultDepositEvent` / `PoCBeneficiaryVaultClaimedEvent` → vault tables + balance materialization
//!
//! **`PostCreatedEvent` omission:** on-chain `Post` stores `platform_id` and `permissions`; the social indexer
//! populates `posts.platform_id` / `posts.permissions` from **`PostCreatedEvent`** after the contract includes those fields.

use serde::Deserialize;

use super::SocialEventRow;
use super::common;
use myso_indexer_alt_social_schema::models::{
    DISPUTE_STATUS_VOTING, NewPocAnalysisResult, NewPocBadge, NewPocConfiguration, NewPocDispute,
    NewPocDisputeVote, NewPocRevenueRedirection,
};

fn transaction_id_from_event_id(event_id: &str) -> String {
    event_id.split(':').next().unwrap_or(event_id).to_string()
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
    #[serde(default)]
    beneficiary_address: Option<String>,
    #[serde(default)]
    matched_anchor_id: Option<String>,
    #[serde(default)]
    media_index: Option<u8>,
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
    #[serde(default)]
    dispute_round: Option<u8>,
    #[serde(default)]
    effective_fee: Option<u64>,
    #[serde(default)]
    required_total_stake_quorum: Option<u64>,
    #[serde(default)]
    post_poc_disputes_submitted_after: Option<u8>,
    #[serde(deserialize_with = "deserialize_u64")]
    voting_start_ms: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    voting_end_ms: u64,
    #[serde(default)]
    evidence: String,
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
    #[serde(default)]
    quorum_met: Option<bool>,
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
    min_vote_stake: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    max_vote_stake: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    voting_duration_ms: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    max_reasoning_length: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    max_evidence_urls: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    max_votes_per_dispute: u64,
    #[serde(default = "default_claim_treasury_fee_bps")]
    claim_treasury_fee_bps: u64,
    #[serde(default = "default_max_referral_bps")]
    max_referral_bps: u64,
    #[serde(default = "default_video_embedded_audio_redirect_bps")]
    video_embedded_audio_redirect_bps: u64,
    #[serde(default)]
    dispute_quorum_base_stake: u64,
    #[serde(default = "default_second_round_mult_bps")]
    dispute_second_round_fee_multiplier_bps: u64,
    #[serde(default = "default_second_round_mult_bps")]
    dispute_second_round_quorum_multiplier_bps: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    timestamp: u64,
}

fn default_second_round_mult_bps() -> u64 {
    10000
}

fn default_video_embedded_audio_redirect_bps() -> u64 {
    3000
}

fn default_claim_treasury_fee_bps() -> u64 {
    100
}

fn default_max_referral_bps() -> u64 {
    500
}

#[derive(Debug, Deserialize)]
struct PoCResultAppliedEventJson {
    post_id: String,
    poc_outcome: u8,
    poc_redirection_kind: u8,
    similarity_detected: bool,
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
        "PoCResultAppliedEvent" | "PocResultAppliedEvent" => {
            process_poc_result_applied_event(data, event_id)
        }
        "AnalysisSubmittedEvent" => process_analysis_submitted_event(data, event_id, &tx_id),
        "PoCBadgeIssuedEvent" | "PocBadgeIssuedEvent" | "BadgeIssuedEvent" => {
            process_poc_badge_issued_event(data, event_id, &tx_id)
        }
        "RevenueRedirectionActivatedEvent" => {
            process_revenue_redirection_activated_event(data, event_id, &tx_id)
        }
        "PoCDisputeSubmittedEvent" | "PocDisputeSubmittedEvent" | "DisputeSubmittedEvent" => {
            process_poc_dispute_submitted_event(data, event_id, &tx_id)
        }
        "DisputeVoteCastEvent" | "VoteCastEvent" => {
            process_dispute_vote_cast_event(data, event_id, &tx_id)
        }
        "PoCDisputeResolvedEvent" | "PocDisputeResolvedEvent" | "DisputeResolvedEvent" => {
            process_poc_dispute_resolved_event(data, event_id)
        }
        "VotingRewardClaimedEvent" | "RewardClaimedEvent" => {
            process_voting_reward_claimed_event(data, event_id)
        }
        "PoCConfigUpdatedEvent" | "PocConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
            process_poc_config_updated_event(data, event_id, &tx_id)
        }
        "PoCBeneficiaryVaultDepositEvent" => {
            process_poc_vault_deposit_event(data, event_id, &tx_id)
        }
        "PoCBeneficiaryVaultClaimedEvent" => process_poc_vault_claim_event(data, event_id, &tx_id),
        _ => None,
    }
}

fn process_poc_result_applied_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PoCResultAppliedEventJson = common::deserialize_social_event_json(
        "poc",
        "PoCResultAppliedEvent",
        event_id,
        data,
        "poc PoCResultAppliedEvent JSON did not match PoCResultAppliedEvent",
    )?;
    if ev.post_id.is_empty() {
        return None;
    }
    Some(vec![SocialEventRow::PostPocResultApplied {
        post_id: ev.post_id,
        poc_outcome: i16::from(ev.poc_outcome),
        poc_redirection_kind: i16::from(ev.poc_redirection_kind),
        similarity_detected: ev.similarity_detected,
        timestamp_ms: ev.timestamp as i64,
    }])
}

fn process_analysis_submitted_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: AnalysisSubmittedEvent = common::deserialize_social_event_json(
        "poc",
        "AnalysisSubmittedEvent",
        event_id,
        data,
        "poc AnalysisSubmittedEvent JSON did not match AnalysisSubmittedEvent",
    )?;
    if ev.post_id.is_empty() || ev.oracle_address.is_empty() {
        return None;
    }
    if ev.media_type != 1 && ev.media_type != 2 && ev.media_type != 3 {
        tracing::warn!(
            post_id = %ev.post_id,
            media_type = ev.media_type,
            "AnalysisSubmittedEvent ignored: media_type must be 1 (image), 2 (video), or 3 (audio)"
        );
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
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PocBadgeIssuedEvent = common::deserialize_social_event_json(
        "poc",
        "PoCBadgeIssuedEvent",
        event_id,
        data,
        "poc PoCBadgeIssuedEvent JSON did not match PoCBadgeIssuedEvent",
    )?;
    if ev.badge_id.is_empty() || ev.post_id.is_empty() || ev.issued_by.is_empty() {
        return None;
    }
    if ev.media_type != 1 && ev.media_type != 2 && ev.media_type != 3 {
        tracing::warn!(
            post_id = %ev.post_id,
            media_type = ev.media_type,
            "PoCBadgeIssuedEvent ignored: media_type must be 1, 2, or 3"
        );
        return None;
    }
    let badge_id = ev.badge_id.clone();
    let post_id = ev.post_id.clone();
    let beneficiary_address = ev
        .beneficiary_address
        .as_ref()
        .filter(|s| !s.is_empty())
        .cloned();
    let matched_anchor_id = ev
        .matched_anchor_id
        .as_ref()
        .filter(|s| !s.is_empty())
        .cloned();
    let media_index = ev.media_index.map(i16::from);
    let badge = NewPocBadge {
        badge_id: ev.badge_id,
        post_id: ev.post_id,
        media_type: ev.media_type as i16,
        issued_by: ev.issued_by,
        beneficiary_address,
        matched_anchor_id,
        media_index,
        issued_at: ev.timestamp as i64,
        revoked: false,
        revoked_at: None,
        transaction_id: tx_id.to_string(),
    };
    Some(vec![
        SocialEventRow::PocBadge(badge),
        SocialEventRow::PostPocBadgePointer {
            post_id,
            poc_badge_object_id: badge_id,
        },
    ])
}

fn process_revenue_redirection_activated_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: RevenueRedirectionActivatedEvent = common::deserialize_social_event_json(
        "poc",
        "RevenueRedirectionActivatedEvent",
        event_id,
        data,
        "poc RevenueRedirectionActivatedEvent JSON did not match RevenueRedirectionActivatedEvent",
    )?;
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
        poc_redirection_kind: 1,
    };

    Some(vec![
        SocialEventRow::PocRevenueRedirection(redirection),
        post_update,
    ])
}

fn process_poc_dispute_submitted_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PocDisputeSubmittedEvent = common::deserialize_social_event_json(
        "poc",
        "PoCDisputeSubmittedEvent",
        event_id,
        data,
        "poc PoCDisputeSubmittedEvent JSON did not match PocDisputeSubmittedEvent",
    )?;
    if ev.dispute_id.is_empty() || ev.post_id.is_empty() || ev.disputer.is_empty() {
        return None;
    }
    if ev.voting_start_ms >= ev.voting_end_ms {
        return None;
    }

    let dispute_round = i16::from(ev.dispute_round.unwrap_or(1));
    let effective_fee_u64 = ev.effective_fee.unwrap_or(ev.stake_amount);
    let effective_fee = i64::try_from(effective_fee_u64).ok()?;
    let required_quorum = i64::try_from(ev.required_total_stake_quorum.unwrap_or(0)).ok()?;
    let stake_amount = i64::try_from(ev.stake_amount).ok()?;

    let post_after = ev
        .post_poc_disputes_submitted_after
        .unwrap_or(ev.dispute_round.unwrap_or(1));
    let post_after_i16 = i16::from(post_after);

    let dispute = NewPocDispute {
        dispute_id: ev.dispute_id.clone(),
        post_id: ev.post_id.clone(),
        disputer: ev.disputer,
        dispute_type: ev.dispute_type as i16,
        evidence: ev.evidence,
        status: DISPUTE_STATUS_VOTING,
        stake_amount,
        voting_start_ms: ev.voting_start_ms as i64,
        voting_end_ms: ev.voting_end_ms as i64,
        resolution: None,
        winning_side: None,
        total_winning_stake: None,
        total_losing_stake: None,
        submitted_at: ev.timestamp as i64,
        resolved_at: None,
        transaction_id: tx_id.to_string(),
        dispute_round,
        effective_dispute_fee: effective_fee,
        required_total_stake_quorum: required_quorum,
        quorum_met: None,
    };
    Some(vec![
        SocialEventRow::PocDispute(dispute),
        SocialEventRow::PostPocDisputesSubmitted {
            post_id: ev.post_id,
            poc_disputes_submitted: post_after_i16,
        },
    ])
}

fn process_dispute_vote_cast_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: DisputeVoteCastEvent = common::deserialize_social_event_json(
        "poc",
        "DisputeVoteCastEvent",
        event_id,
        data,
        "poc DisputeVoteCastEvent JSON did not match DisputeVoteCastEvent",
    )?;
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

fn process_poc_dispute_resolved_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PocDisputeResolvedEvent = common::deserialize_social_event_json(
        "poc",
        "PoCDisputeResolvedEvent",
        event_id,
        data,
        "poc PoCDisputeResolvedEvent JSON did not match PocDisputeResolvedEvent",
    )?;
    if ev.dispute_id.is_empty() || ev.post_id.is_empty() {
        return None;
    }
    let quorum_met = ev.quorum_met.unwrap_or(true);
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
        quorum_met,
    }])
}

fn process_voting_reward_claimed_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: VotingRewardClaimedEvent = common::deserialize_social_event_json(
        "poc",
        "VotingRewardClaimedEvent",
        event_id,
        data,
        "poc VotingRewardClaimedEvent JSON did not match VotingRewardClaimedEvent",
    )?;
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
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PocConfigUpdatedEvent = common::deserialize_social_event_json(
        "poc",
        "PoCConfigUpdatedEvent",
        event_id,
        data,
        "poc PoCConfigUpdatedEvent JSON did not match PocConfigUpdatedEvent",
    )?;
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
        min_vote_stake: ev.min_vote_stake as i64,
        max_vote_stake: ev.max_vote_stake as i64,
        voting_duration_ms: ev.voting_duration_ms as i64,
        max_reasoning_length: ev.max_reasoning_length as i64,
        max_evidence_urls: ev.max_evidence_urls as i64,
        max_votes_per_dispute: ev.max_votes_per_dispute as i64,
        oracle_address: Some(ev.oracle_address),
        claim_treasury_fee_bps: ev.claim_treasury_fee_bps as i64,
        max_referral_bps: ev.max_referral_bps as i64,
        video_embedded_audio_redirect_bps: ev.video_embedded_audio_redirect_bps as i64,
        dispute_quorum_base_stake: ev.dispute_quorum_base_stake as i64,
        dispute_second_round_fee_multiplier_bps: ev.dispute_second_round_fee_multiplier_bps as i64,
        dispute_second_round_quorum_multiplier_bps: ev.dispute_second_round_quorum_multiplier_bps
            as i64,
        updated_by: ev.updated_by,
        updated_at: ev.timestamp as i64,
        transaction_id: tx_id.to_string(),
    };
    Some(vec![SocialEventRow::PocConfiguration(config)])
}

fn process_poc_vault_deposit_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(Deserialize)]
    struct VaultDepositJson {
        vault_id: String,
        beneficiary: String,
        coin_type: String,
        #[serde(deserialize_with = "deserialize_u64")]
        amount: u64,
        #[serde(default)]
        source_post_id: Option<String>,
        #[serde(deserialize_with = "deserialize_u64")]
        timestamp: u64,
    }
    let ev: VaultDepositJson = common::deserialize_social_event_json(
        "poc_vault",
        "PoCBeneficiaryVaultDepositEvent",
        event_id,
        data,
        "poc_vault PoCBeneficiaryVaultDepositEvent JSON did not match VaultDepositJson",
    )?;
    if ev.vault_id.is_empty() || ev.beneficiary.is_empty() {
        return None;
    }
    let amount = i64::try_from(ev.amount).ok()?;
    Some(vec![SocialEventRow::PocBeneficiaryVaultDeposit {
        vault_id: ev.vault_id,
        beneficiary_address: ev.beneficiary,
        coin_type: ev.coin_type,
        amount,
        source_post_id: ev.source_post_id.filter(|s| !s.is_empty()),
        timestamp_ms: ev.timestamp as i64,
        transaction_id: tx_id.to_string(),
    }])
}

fn process_poc_vault_claim_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(Deserialize)]
    struct VaultClaimJson {
        vault_id: String,
        beneficiary: String,
        coin_type: String,
        #[serde(default)]
        referrer: Option<String>,
        #[serde(deserialize_with = "deserialize_u64")]
        treasury_amount: u64,
        #[serde(deserialize_with = "deserialize_u64")]
        referrer_amount: u64,
        #[serde(deserialize_with = "deserialize_u64")]
        beneficiary_amount: u64,
        #[serde(deserialize_with = "deserialize_u64")]
        timestamp: u64,
    }
    let ev: VaultClaimJson = common::deserialize_social_event_json(
        "poc_vault",
        "PoCBeneficiaryVaultClaimedEvent",
        event_id,
        data,
        "poc_vault PoCBeneficiaryVaultClaimedEvent JSON did not match VaultClaimJson",
    )?;
    if ev.vault_id.is_empty() || ev.beneficiary.is_empty() {
        return None;
    }
    Some(vec![SocialEventRow::PocBeneficiaryVaultClaimed {
        vault_id: ev.vault_id,
        beneficiary_address: ev.beneficiary,
        coin_type: ev.coin_type,
        referrer_address: ev.referrer.filter(|s| !s.is_empty()),
        treasury_amount: ev.treasury_amount as i64,
        referrer_amount: ev.referrer_amount as i64,
        beneficiary_amount: ev.beneficiary_amount as i64,
        timestamp_ms: ev.timestamp as i64,
        transaction_id: tx_id.to_string(),
    }])
}
