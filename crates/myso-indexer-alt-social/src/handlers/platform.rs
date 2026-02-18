// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{TimeZone, Utc};
use serde::Deserialize;

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewPlatform, NewPlatformBlockedProfile, NewPlatformEvent, NewPlatformMembership,
    NewPlatformModerator, NewPlatformTokenAirdrop,
};

fn de_u64<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
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

fn de_opt_u64<'de, D>(d: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::deserialize(d)
}

fn ms_to_naive(ms: u64) -> chrono::NaiveDateTime {
    if ms == 0 {
        return Utc::now().naive_utc();
    }
    let secs = (ms / 1000) as i64;
    let nsecs = ((ms % 1000) * 1_000_000) as u32;
    Utc.timestamp_opt(secs, nsecs)
        .single()
        .unwrap_or_else(Utc::now)
        .naive_utc()
}

#[derive(Debug, Deserialize)]
struct PlatformStatus {
    status: u8,
}

#[derive(Debug, Deserialize)]
struct PlatformCreatedEvent {
    platform_id: String,
    name: String,
    tagline: String,
    description: String,
    developer: String,
    logo: String,
    terms_of_service: String,
    privacy_policy: String,
    platforms: Vec<String>,
    links: Vec<String>,
    primary_category: String,
    #[serde(default)]
    secondary_category: Option<String>,
    status: PlatformStatus,
    release_date: String,
    #[serde(default)]
    wants_dao_governance: bool,
    #[serde(default)]
    governance_registry_id: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    delegate_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    delegate_term_epochs: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    proposal_submission_cost: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    min_on_chain_age_days: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    max_votes_per_user: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    quadratic_base_cost: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    voting_period_epochs: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    quorum_votes: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct PlatformUpdatedEvent {
    platform_id: String,
    name: String,
    tagline: String,
    description: String,
    terms_of_service: String,
    privacy_policy: String,
    platforms: Vec<String>,
    links: Vec<String>,
    primary_category: String,
    #[serde(default)]
    secondary_category: Option<String>,
    status: PlatformStatus,
    release_date: String,
    #[serde(default)]
    shutdown_date: Option<String>,
    #[serde(deserialize_with = "de_u64")]
    updated_at: u64,
}

#[derive(Debug, Deserialize)]
struct PlatformApprovalChangedEvent {
    platform_id: String,
    #[serde(alias = "approved")]
    is_approved: bool,
    #[serde(alias = "changed_by")]
    approved_by: String,
    #[serde(default, deserialize_with = "de_u64")]
    changed_at: u64,
    #[serde(default)]
    reasoning: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ModeratorAddedEvent {
    platform_id: String,
    moderator_address: String,
    added_by: String,
}

#[derive(Debug, Deserialize)]
struct ModeratorRemovedEvent {
    platform_id: String,
    moderator_address: String,
    _removed_by: String,
}

#[derive(Debug, Deserialize)]
struct PlatformBlockedProfileEvent {
    platform_id: String,
    profile_id: String,
    blocked_by: String,
}

#[derive(Debug, Deserialize)]
struct PlatformUnblockedProfileEvent {
    platform_id: String,
    profile_id: String,
    _unblocked_by: String,
}

#[derive(Debug, Deserialize)]
struct UserJoinedPlatformEvent {
    wallet_address: String,
    platform_id: String,
    #[serde(deserialize_with = "de_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct UserLeftPlatformEvent {
    wallet_address: String,
    platform_id: String,
    #[serde(deserialize_with = "de_u64")]
    _timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct TokenAirdropEvent {
    platform_id: String,
    recipient: String,
    #[serde(deserialize_with = "de_u64")]
    amount: u64,
    reason_code: u8,
    executed_by: String,
    #[serde(deserialize_with = "de_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct PlatformDeletedEvent {
    platform_id: String,
    _name: String,
    _developer: String,
    _deleted_by: String,
    #[serde(deserialize_with = "de_u64")]
    timestamp: u64,
    #[serde(default)]
    reasoning: Option<String>,
}

pub fn handle_platform_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "PlatformCreatedEvent" => process_platform_created_event(data, event_id),
        "PlatformUpdatedEvent" => process_platform_updated_event(data, event_id),
        "PlatformApprovalChangedEvent" | "ApprovalChangedEvent" => {
            process_platform_approval_changed_event(data, event_id)
        }
        "ModeratorAddedEvent" => process_moderator_added_event(data, event_id),
        "ModeratorRemovedEvent" => process_moderator_removed_event(data),
        "PlatformBlockedProfileEvent" => process_platform_blocked_profile_event(data, event_id),
        "PlatformUnblockedProfileEvent" => process_platform_unblocked_profile_event(data),
        "UserJoinedPlatformEvent" => process_user_joined_platform_event(data, event_id),
        "UserLeftPlatformEvent" => process_user_left_platform_event(data),
        "TokenAirdropEvent" => process_token_airdrop_event(data, event_id),
        "PlatformDeletedEvent" => process_platform_deleted_event(data, event_id),
        _ => None,
    }
}

fn process_platform_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformCreatedEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now().naive_utc();

    let platform = NewPlatform {
        platform_id: ev.platform_id.clone(),
        name: ev.name,
        tagline: ev.tagline,
        description: Some(ev.description).filter(|s| !s.is_empty()),
        logo: Some(ev.logo).filter(|s| !s.is_empty()),
        developer_address: ev.developer,
        terms_of_service: Some(ev.terms_of_service),
        privacy_policy: Some(ev.privacy_policy),
        platform_names: Some(serde_json::to_value(&ev.platforms).unwrap_or_default()),
        links: Some(serde_json::to_value(&ev.links).unwrap_or_default()),
        status: ev.status.status as i16,
        release_date: Some(ev.release_date),
        shutdown_date: None,
        created_at: now,
        updated_at: now,
        is_approved: false,
        approval_changed_at: None,
        approved_by: None,
        wants_dao_governance: Some(ev.wants_dao_governance),
        governance_registry_id: ev.governance_registry_id,
        delegate_count: ev.delegate_count.map(|v| v as i64),
        delegate_term_epochs: ev.delegate_term_epochs.map(|v| v as i64),
        max_votes_per_user: ev.max_votes_per_user.map(|v| v as i64),
        min_on_chain_age_days: ev.min_on_chain_age_days.map(|v| v as i64),
        proposal_submission_cost: ev.proposal_submission_cost.map(|v| v as i64),
        quadratic_base_cost: ev.quadratic_base_cost.map(|v| v as i64),
        quorum_votes: ev.quorum_votes.map(|v| v as i64),
        voting_period_epochs: ev.voting_period_epochs.map(|v| v as i64),
        treasury: None,
        version: None,
        primary_category: ev.primary_category,
        secondary_category: ev.secondary_category,
        deleted_at: None,
    };

    let platform_event = NewPlatformEvent {
        event_type: "PlatformCreated".to_string(),
        platform_id: ev.platform_id,
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    Some(vec![
        SocialEventRow::Platform(platform),
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_platform_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformUpdatedEvent = serde_json::from_value(data.clone()).ok()?;
    let updated_at = ms_to_naive(ev.updated_at);

    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "PlatformUpdated".to_string(),
        platform_id: ev.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    Some(vec![
        SocialEventRow::PlatformUpdate {
            platform_id: ev.platform_id,
            name: ev.name,
            tagline: ev.tagline,
            description: Some(ev.description),
            terms_of_service: Some(ev.terms_of_service),
            privacy_policy: Some(ev.privacy_policy),
            platform_names: Some(serde_json::to_value(&ev.platforms).unwrap_or_default()),
            links: Some(serde_json::to_value(&ev.links).unwrap_or_default()),
            status: ev.status.status as i16,
            release_date: Some(ev.release_date),
            shutdown_date: ev.shutdown_date,
            updated_at,
            primary_category: ev.primary_category,
            secondary_category: ev.secondary_category,
        },
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_platform_approval_changed_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformApprovalChangedEvent = serde_json::from_value(data.clone()).ok()?;
    let changed_at = ms_to_naive(ev.changed_at);

    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "ApprovalChanged".to_string(),
        platform_id: ev.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: ev.reasoning,
    };

    Some(vec![
        SocialEventRow::PlatformApprovalChange {
            platform_id: ev.platform_id,
            is_approved: ev.is_approved,
            approved_by: ev.approved_by,
            changed_at,
        },
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_moderator_added_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ModeratorAddedEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now().naive_utc();

    let moderator = NewPlatformModerator {
        platform_id: ev.platform_id.clone(),
        moderator_address: ev.moderator_address,
        added_by: ev.added_by,
        created_at: now,
    };

    let platform_event = NewPlatformEvent {
        event_type: "ModeratorAdded".to_string(),
        platform_id: ev.platform_id,
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    Some(vec![
        SocialEventRow::PlatformModerator(moderator),
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_moderator_removed_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: ModeratorRemovedEvent = serde_json::from_value(data.clone()).ok()?;
    Some(vec![SocialEventRow::PlatformModeratorRemove {
        platform_id: ev.platform_id,
        moderator_address: ev.moderator_address,
    }])
}

fn process_platform_blocked_profile_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformBlockedProfileEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now().naive_utc();

    let blocked = NewPlatformBlockedProfile {
        platform_id: ev.platform_id.clone(),
        wallet_address: ev.profile_id,
        blocked_by: ev.blocked_by,
        created_at: now,
    };

    let platform_event = NewPlatformEvent {
        event_type: "PlatformBlockedProfile".to_string(),
        platform_id: ev.platform_id,
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    Some(vec![
        SocialEventRow::PlatformBlockedProfile(blocked),
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_platform_unblocked_profile_event(
    data: &serde_json::Value,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformUnblockedProfileEvent = serde_json::from_value(data.clone()).ok()?;
    Some(vec![SocialEventRow::PlatformBlockedProfileRemove {
        platform_id: ev.platform_id,
        wallet_address: ev.profile_id,
    }])
}

fn process_user_joined_platform_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UserJoinedPlatformEvent = serde_json::from_value(data.clone()).ok()?;
    let joined_at = ms_to_naive(ev.timestamp);

    let membership = NewPlatformMembership {
        platform_id: ev.platform_id.clone(),
        wallet_address: ev.wallet_address,
        joined_at,
    };

    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "UserJoinedPlatform".to_string(),
        platform_id: ev.platform_id,
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    Some(vec![
        SocialEventRow::PlatformMembership(membership),
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_user_left_platform_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: UserLeftPlatformEvent = serde_json::from_value(data.clone()).ok()?;
    Some(vec![SocialEventRow::PlatformMembershipRemove {
        platform_id: ev.platform_id,
        wallet_address: ev.wallet_address,
    }])
}

fn process_token_airdrop_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: TokenAirdropEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now().naive_utc();

    let airdrop = NewPlatformTokenAirdrop {
        platform_id: ev.platform_id.clone(),
        recipient: ev.recipient,
        amount: ev.amount as i64,
        reason_code: ev.reason_code as i16,
        executed_by: ev.executed_by,
        timestamp: ev.timestamp as i64,
        created_at: now,
        event_id: Some(event_id.to_string()),
    };

    let platform_event = NewPlatformEvent {
        event_type: "TokenAirdrop".to_string(),
        platform_id: ev.platform_id,
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    Some(vec![
        SocialEventRow::PlatformTokenAirdrop(airdrop),
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_platform_deleted_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformDeletedEvent = serde_json::from_value(data.clone()).ok()?;
    let deleted_at = ms_to_naive(ev.timestamp);

    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "PlatformDeleted".to_string(),
        platform_id: ev.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: ev.reasoning,
    };

    Some(vec![
        SocialEventRow::PlatformDeleted {
            platform_id: ev.platform_id,
            deleted_at,
        },
        SocialEventRow::PlatformEvent(platform_event),
    ])
}
