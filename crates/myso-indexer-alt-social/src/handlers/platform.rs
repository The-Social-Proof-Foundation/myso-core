// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{TimeZone, Utc};
use serde::Deserialize;

use super::common;
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

fn normalize_date_format(date_str: &str) -> String {
    if date_str.is_empty() {
        return date_str.to_string();
    }
    if date_str.matches('-').count() == 2 {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() == 3 && parts[0].len() == 4 {
            return date_str.to_string();
        }
    }
    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() == 3 {
        if let (Ok(month), Ok(day), year_str) =
            (parts[0].parse::<u32>(), parts[1].parse::<u32>(), parts[2])
        {
            let year = if year_str.len() == 2 {
                year_str
                    .parse::<u32>()
                    .map(|yy| if yy < 50 { 2000 + yy } else { 1900 + yy })
                    .unwrap_or(0)
            } else if year_str.len() == 4 {
                year_str.parse::<u32>().unwrap_or(0)
            } else {
                0
            };
            if (1..=12).contains(&month) && (1..=31).contains(&day) && year > 0 {
                return format!("{:04}-{:02}-{:02}", year, month, day);
            }
        }
    }
    date_str.to_string()
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
    wants_dao_governance: Option<bool>,
    #[serde(default)]
    governance_registry_id: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    delegate_count: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    delegate_term_epochs: Option<u64>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    proposal_submission_cost: Option<u64>,
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

#[derive(Debug, Deserialize)]
struct TreasuryFundedEvent {
    platform_id: String,
    #[serde(deserialize_with = "de_u64")]
    _amount: u64,
    _funded_by: String,
    #[serde(deserialize_with = "de_u64")]
    _new_balance: u64,
    #[serde(deserialize_with = "de_u64")]
    _timestamp: u64,
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
        "ModeratorRemovedEvent" => process_moderator_removed_event(data, event_id),
        "PlatformBlockedProfileEvent" => process_platform_blocked_profile_event(data, event_id),
        "PlatformUnblockedProfileEvent" => process_platform_unblocked_profile_event(data, event_id),
        "UserJoinedPlatformEvent" => process_user_joined_platform_event(data, event_id),
        "UserLeftPlatformEvent" => process_user_left_platform_event(data, event_id),
        "TokenAirdropEvent" => process_token_airdrop_event(data, event_id),
        "PlatformDeletedEvent" => process_platform_deleted_event(data, event_id),
        "TreasuryFundedEvent" => process_treasury_funded_event(data, event_id),
        _ => None,
    }
}

fn normalize_dao_fields(
    ev: &PlatformCreatedEvent,
) -> (
    bool,
    Option<String>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
    Option<i64>,
) {
    let explicit_dao = ev.wants_dao_governance.unwrap_or(false);
    let has_governance_registry = ev.governance_registry_id.is_some();
    let has_dao_fields = ev.delegate_count.is_some()
        || ev.delegate_term_epochs.is_some()
        || ev.max_votes_per_user.is_some()
        || ev.proposal_submission_cost.is_some()
        || ev.quadratic_base_cost.is_some()
        || ev.quorum_votes.is_some()
        || ev.voting_period_epochs.is_some();
    let is_dao = explicit_dao || has_governance_registry || has_dao_fields;
    let wants_dao = is_dao;
    (
        wants_dao,
        ev.governance_registry_id.clone(),
        ev.delegate_count.map(|v| v as i64),
        ev.delegate_term_epochs.map(|v| v as i64),
        ev.max_votes_per_user.map(|v| v as i64),
        ev.proposal_submission_cost.map(|v| v as i64),
        ev.quadratic_base_cost.map(|v| v as i64),
        ev.quorum_votes.map(|v| v as i64),
        ev.voting_period_epochs.map(|v| v as i64),
    )
}

fn process_platform_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformCreatedEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformCreatedEvent",
        event_id,
        data,
        "platform PlatformCreatedEvent JSON did not match PlatformCreatedEvent",
    )?;
    let now = Utc::now().naive_utc();
    let (
        wants_dao,
        governance_registry_id,
        delegate_count,
        delegate_term_epochs,
        max_votes_per_user,
        proposal_submission_cost,
        quadratic_base_cost,
        quorum_votes,
        voting_period_epochs,
    ) = normalize_dao_fields(&ev);
    let release_date = normalize_date_format(&ev.release_date);

    let developer = ev.developer;
    let moderator = NewPlatformModerator {
        platform_id: ev.platform_id.clone(),
        moderator_address: developer.clone(),
        added_by: developer.clone(),
        created_at: now,
    };

    let platform = NewPlatform {
        platform_id: ev.platform_id.clone(),
        name: ev.name,
        tagline: ev.tagline,
        description: Some(ev.description).filter(|s| !s.is_empty()),
        logo: Some(ev.logo).filter(|s| !s.is_empty()),
        developer_address: developer,
        terms_of_service: Some(ev.terms_of_service),
        privacy_policy: Some(ev.privacy_policy),
        platform_names: Some(serde_json::to_value(&ev.platforms).unwrap_or_default()),
        links: Some(serde_json::to_value(&ev.links).unwrap_or_default()),
        status: ev.status.status as i16,
        release_date: Some(release_date),
        shutdown_date: None,
        created_at: now,
        updated_at: now,
        is_approved: false,
        approval_changed_at: None,
        approved_by: None,
        wants_dao_governance: Some(wants_dao),
        governance_registry_id,
        delegate_count,
        delegate_term_epochs,
        max_votes_per_user,
        proposal_submission_cost,
        quadratic_base_cost,
        quorum_votes,
        voting_period_epochs,
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
        SocialEventRow::PlatformModerator(moderator),
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_platform_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformUpdatedEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformUpdatedEvent",
        event_id,
        data,
        "platform PlatformUpdatedEvent JSON did not match PlatformUpdatedEvent",
    )?;
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

    let release_date = normalize_date_format(&ev.release_date);
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
            release_date: Some(release_date),
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
    let ev: PlatformApprovalChangedEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformApprovalChangedEvent",
        event_id,
        data,
        "platform PlatformApprovalChangedEvent JSON did not match PlatformApprovalChangedEvent",
    )?;
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
    let ev: ModeratorAddedEvent = common::deserialize_social_event_json(
        "platform",
        "ModeratorAddedEvent",
        event_id,
        data,
        "platform ModeratorAddedEvent JSON did not match ModeratorAddedEvent",
    )?;
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

fn process_moderator_removed_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ModeratorRemovedEvent = common::deserialize_social_event_json(
        "platform",
        "ModeratorRemovedEvent",
        event_id,
        data,
        "platform ModeratorRemovedEvent JSON did not match ModeratorRemovedEvent",
    )?;
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "ModeratorRemoved".to_string(),
        platform_id: ev.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };
    Some(vec![
        SocialEventRow::PlatformEvent(platform_event),
        SocialEventRow::PlatformModeratorRemove {
            platform_id: ev.platform_id,
            moderator_address: ev.moderator_address,
        },
    ])
}

fn process_platform_blocked_profile_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformBlockedProfileEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformBlockedProfileEvent",
        event_id,
        data,
        "platform PlatformBlockedProfileEvent JSON did not match PlatformBlockedProfileEvent",
    )?;
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
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformUnblockedProfileEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformUnblockedProfileEvent",
        event_id,
        data,
        "platform PlatformUnblockedProfileEvent JSON did not match PlatformUnblockedProfileEvent",
    )?;
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "PlatformUnblockedProfile".to_string(),
        platform_id: ev.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };
    Some(vec![
        SocialEventRow::PlatformEvent(platform_event),
        SocialEventRow::PlatformBlockedProfileRemove {
            platform_id: ev.platform_id,
            wallet_address: ev.profile_id,
        },
    ])
}

fn process_user_joined_platform_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UserJoinedPlatformEvent = common::deserialize_social_event_json(
        "platform",
        "UserJoinedPlatformEvent",
        event_id,
        data,
        "platform UserJoinedPlatformEvent JSON did not match UserJoinedPlatformEvent",
    )?;
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

fn process_user_left_platform_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UserLeftPlatformEvent = common::deserialize_social_event_json(
        "platform",
        "UserLeftPlatformEvent",
        event_id,
        data,
        "platform UserLeftPlatformEvent JSON did not match UserLeftPlatformEvent",
    )?;
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "UserLeftPlatform".to_string(),
        platform_id: ev.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };
    Some(vec![
        SocialEventRow::PlatformEvent(platform_event),
        SocialEventRow::PlatformMembershipRemove {
            platform_id: ev.platform_id,
            wallet_address: ev.wallet_address,
        },
    ])
}

fn process_token_airdrop_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: TokenAirdropEvent = common::deserialize_social_event_json(
        "platform",
        "TokenAirdropEvent",
        event_id,
        data,
        "platform TokenAirdropEvent JSON did not match TokenAirdropEvent",
    )?;
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
    let ev: PlatformDeletedEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformDeletedEvent",
        event_id,
        data,
        "platform PlatformDeletedEvent JSON did not match PlatformDeletedEvent",
    )?;
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

fn process_treasury_funded_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: TreasuryFundedEvent = common::deserialize_social_event_json(
        "platform",
        "TreasuryFundedEvent",
        event_id,
        data,
        "platform TreasuryFundedEvent JSON did not match TreasuryFundedEvent",
    )?;
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "TreasuryFunded".to_string(),
        platform_id: ev.platform_id,
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };
    Some(vec![SocialEventRow::PlatformEvent(platform_event)])
}
