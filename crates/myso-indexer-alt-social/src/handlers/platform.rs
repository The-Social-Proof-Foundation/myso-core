// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{TimeZone, Utc};
use serde::Deserialize;

use super::common;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewPlatform, NewPlatformBlockedProfile, NewPlatformEvent, NewPlatformMembership,
    NewPlatformModerator, NewPlatformModeratorPermission, NewPlatformTokenAirdrop,
};
use myso_indexer_alt_social_schema::platform_permissions::ALL_MODERATOR_EXTENSION_PERMISSIONS;

fn normalize_platform_permission(name: &str) -> Option<&'static str> {
    myso_indexer_alt_social_schema::platform_permissions::normalize_platform_permission(name)
}

fn permission_grant_row(
    platform_id: &str,
    moderator_address: &str,
    permission_type: &str,
    granted_by: &str,
    granted_at: chrono::NaiveDateTime,
) -> NewPlatformModeratorPermission {
    NewPlatformModeratorPermission {
        platform_id: platform_id.to_string(),
        moderator_address: moderator_address.to_string(),
        permission_type: permission_type.to_string(),
        granted_by: granted_by.to_string(),
        granted_at,
        revoked_at: None,
    }
}

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

fn ms_to_naive(ms: u64, checkpoint_timestamp_ms: u64) -> chrono::NaiveDateTime {
    common::chain_time_from_ms(common::chain_timestamp_ms(
        if ms > 0 { Some(ms as i64) } else { None },
        checkpoint_timestamp_ms,
    ))
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
    #[serde(default)]
    cover_photo: Option<String>,
    #[serde(default)]
    media_previews: Option<Vec<String>>,
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
    #[serde(default)]
    moderators_group_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PlatformUpdatedEvent {
    platform_id: String,
    name: String,
    tagline: String,
    description: String,
    #[serde(default)]
    logo: String,
    terms_of_service: String,
    privacy_policy: String,
    platforms: Vec<String>,
    links: Vec<String>,
    #[serde(default)]
    cover_photo: Option<String>,
    #[serde(default)]
    media_previews: Option<Vec<String>>,
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
struct ModeratorPermissionsGrantedEvent {
    platform_id: String,
    #[serde(default, rename = "moderators_group_id")]
    _moderators_group_id: Option<String>,
    member: String,
    permissions: Vec<String>,
    granted_by: String,
}

#[derive(Debug, Deserialize)]
struct ModeratorPermissionsRevokedEvent {
    platform_id: String,
    #[serde(default, rename = "moderators_group_id")]
    _moderators_group_id: Option<String>,
    member: String,
    permissions: Vec<String>,
    _revoked_by: String,
}

#[derive(Debug, Deserialize)]
struct ModeratorRemovedEvent {
    platform_id: String,
    #[serde(default, rename = "moderators_group_id")]
    _moderators_group_id: Option<String>,
    member: String,
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
    name: String,
    developer: String,
    deleted_by: String,
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "PlatformCreatedEvent" => {
            process_platform_created_event(data, event_id, checkpoint_timestamp_ms)
        }
        "PlatformUpdatedEvent" => {
            process_platform_updated_event(data, event_id, checkpoint_timestamp_ms)
        }
        "PlatformApprovalChangedEvent" | "ApprovalChangedEvent" => {
            process_platform_approval_changed_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ModeratorPermissionsGrantedEvent" => {
            process_moderator_permissions_granted_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ModeratorPermissionsRevokedEvent" => {
            process_moderator_permissions_revoked_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ModeratorRemovedEvent" => {
            process_moderator_removed_event(data, event_id, checkpoint_timestamp_ms)
        }
        "PlatformBlockedProfileEvent" => {
            process_platform_blocked_profile_event(data, event_id, checkpoint_timestamp_ms)
        }
        "PlatformUnblockedProfileEvent" => {
            process_platform_unblocked_profile_event(data, event_id, checkpoint_timestamp_ms)
        }
        "UserJoinedPlatformEvent" => {
            process_user_joined_platform_event(data, event_id, checkpoint_timestamp_ms)
        }
        "UserLeftPlatformEvent" => {
            process_user_left_platform_event(data, event_id, checkpoint_timestamp_ms)
        }
        "TokenAirdropEvent" => {
            process_token_airdrop_event(data, event_id, checkpoint_timestamp_ms)
        }
        "PlatformDeletedEvent" => {
            process_platform_deleted_event(data, event_id, checkpoint_timestamp_ms)
        }
        "TreasuryFundedEvent" => {
            process_treasury_funded_event(data, event_id, checkpoint_timestamp_ms)
        }
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformCreatedEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformCreatedEvent",
        event_id,
        data,
        "platform PlatformCreatedEvent JSON did not match PlatformCreatedEvent",
    )?;
    let event_ms = common::json_field_as_i64(data.get("created_at"));
    let now = common::chain_time_from_ms(common::chain_timestamp_ms(
        event_ms,
        checkpoint_timestamp_ms,
    ))
    .naive_utc();
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
        cover_photo: ev.cover_photo.filter(|s| !s.is_empty()),
        media_previews: ev
            .media_previews
            .map(|v| serde_json::to_value(&v).unwrap_or_default()),
        developer_address: developer.clone(),
        moderators_group_id: ev.moderators_group_id.clone(),
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
        version: None,
        primary_category: ev.primary_category,
        secondary_category: ev.secondary_category,
        deleted_at: None,
    };

    let platform_event = NewPlatformEvent {
        event_type: "PlatformCreated".to_string(),
        platform_id: ev.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    let mut rows = vec![
        SocialEventRow::Platform(platform),
        SocialEventRow::PlatformModerator(moderator),
    ];
    for permission_type in ALL_MODERATOR_EXTENSION_PERMISSIONS {
        rows.push(SocialEventRow::PlatformModeratorPermissionGrant(
            permission_grant_row(
                &ev.platform_id,
                &developer,
                permission_type,
                &developer,
                now,
            ),
        ));
    }
    rows.push(SocialEventRow::PlatformEvent(platform_event));
    Some(rows)
}

fn process_platform_updated_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformUpdatedEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformUpdatedEvent",
        event_id,
        data,
        "platform PlatformUpdatedEvent JSON did not match PlatformUpdatedEvent",
    )?;
    let updated_at = ms_to_naive(ev.updated_at, checkpoint_timestamp_ms);

    let now = common::chain_time_from_ms(common::chain_timestamp_ms(Some(ev.updated_at as i64), checkpoint_timestamp_ms)).naive_utc();
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
            logo: Some(ev.logo).filter(|s| !s.is_empty()),
            terms_of_service: Some(ev.terms_of_service),
            privacy_policy: Some(ev.privacy_policy),
            platform_names: Some(serde_json::to_value(&ev.platforms).unwrap_or_default()),
            links: Some(serde_json::to_value(&ev.links).unwrap_or_default()),
            cover_photo: ev.cover_photo.filter(|s| !s.is_empty()),
            media_previews: ev
                .media_previews
                .map(|v| serde_json::to_value(&v).unwrap_or_default()),
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformApprovalChangedEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformApprovalChangedEvent",
        event_id,
        data,
        "platform PlatformApprovalChangedEvent JSON did not match PlatformApprovalChangedEvent",
    )?;
    let changed_at = ms_to_naive(ev.changed_at, checkpoint_timestamp_ms);

    let now = common::chain_time_from_ms(common::chain_timestamp_ms(Some(ev.changed_at as i64), checkpoint_timestamp_ms)).naive_utc();
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

fn process_moderator_permissions_granted_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: ModeratorPermissionsGrantedEvent = common::deserialize_social_event_json(
        "platform",
        "ModeratorPermissionsGrantedEvent",
        event_id,
        data,
        "platform ModeratorPermissionsGrantedEvent JSON did not match ModeratorPermissionsGrantedEvent",
    )?;
    let now = common::chain_time_from_ms(common::chain_timestamp_ms(common::json_field_as_i64(data.get("timestamp")), checkpoint_timestamp_ms)).naive_utc();
    let mut rows = vec![SocialEventRow::PlatformModerator(NewPlatformModerator {
        platform_id: ev.platform_id.clone(),
        moderator_address: ev.member.clone(),
        added_by: ev.granted_by.clone(),
        created_at: now,
    })];
    let mut valid_permissions = Vec::new();
    for permission in &ev.permissions {
        if let Some(permission_type) = normalize_platform_permission(permission) {
            valid_permissions.push(permission_type.to_string());
            rows.push(SocialEventRow::PlatformModeratorPermissionGrant(
                permission_grant_row(
                    &ev.platform_id,
                    &ev.member,
                    permission_type,
                    &ev.granted_by,
                    now,
                ),
            ));
        } else {
            tracing::warn!(
                event_id,
                permission,
                "skipping unknown platform moderator permission"
            );
        }
    }
    if !valid_permissions.is_empty() {
        rows.push(SocialEventRow::PlatformEvent(NewPlatformEvent {
            event_type: "ModeratorPermissionsGranted".to_string(),
            platform_id: ev.platform_id,
            event_data: data.clone(),
            event_id: Some(event_id.to_string()),
            created_at: now,
            reasoning: None,
        }));
    }
    Some(rows)
}

fn process_moderator_permissions_revoked_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: ModeratorPermissionsRevokedEvent = common::deserialize_social_event_json(
        "platform",
        "ModeratorPermissionsRevokedEvent",
        event_id,
        data,
        "platform ModeratorPermissionsRevokedEvent JSON did not match ModeratorPermissionsRevokedEvent",
    )?;
    let now = common::chain_time_from_ms(common::chain_timestamp_ms(common::json_field_as_i64(data.get("timestamp")), checkpoint_timestamp_ms)).naive_utc();
    let mut rows = Vec::new();
    for permission in &ev.permissions {
        if let Some(permission_type) = normalize_platform_permission(permission) {
            rows.push(SocialEventRow::PlatformModeratorPermissionRevoke {
                platform_id: ev.platform_id.clone(),
                moderator_address: ev.member.clone(),
                permission_type: permission_type.to_string(),
                revoked_at: now,
            });
        } else {
            tracing::warn!(
                event_id,
                permission,
                "skipping unknown platform moderator permission revoke"
            );
        }
    }
    if !rows.is_empty() {
        rows.push(SocialEventRow::PlatformEvent(NewPlatformEvent {
            event_type: "ModeratorPermissionsRevoked".to_string(),
            platform_id: ev.platform_id,
            event_data: data.clone(),
            event_id: Some(event_id.to_string()),
            created_at: now,
            reasoning: None,
        }));
    }
    Some(rows)
}

fn process_moderator_removed_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: ModeratorRemovedEvent = common::deserialize_social_event_json(
        "platform",
        "ModeratorRemovedEvent",
        event_id,
        data,
        "platform ModeratorRemovedEvent JSON did not match ModeratorRemovedEvent",
    )?;
    let now = common::chain_time_from_ms(common::chain_timestamp_ms(common::json_field_as_i64(data.get("timestamp")), checkpoint_timestamp_ms)).naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "ModeratorRemoved".to_string(),
        platform_id: ev.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };
    Some(vec![
        SocialEventRow::PlatformModeratorPermissionRevokeAll {
            platform_id: ev.platform_id.clone(),
            moderator_address: ev.member.clone(),
            revoked_at: now,
        },
        SocialEventRow::PlatformModeratorRemove {
            platform_id: ev.platform_id,
            moderator_address: ev.member,
        },
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_platform_blocked_profile_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformBlockedProfileEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformBlockedProfileEvent",
        event_id,
        data,
        "platform PlatformBlockedProfileEvent JSON did not match PlatformBlockedProfileEvent",
    )?;
    let now = common::chain_time_from_ms(common::chain_timestamp_ms(common::json_field_as_i64(data.get("timestamp")), checkpoint_timestamp_ms)).naive_utc();

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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformUnblockedProfileEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformUnblockedProfileEvent",
        event_id,
        data,
        "platform PlatformUnblockedProfileEvent JSON did not match PlatformUnblockedProfileEvent",
    )?;
    let now = common::chain_time_from_ms(common::chain_timestamp_ms(common::json_field_as_i64(data.get("timestamp")), checkpoint_timestamp_ms)).naive_utc();
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: UserJoinedPlatformEvent = common::deserialize_social_event_json(
        "platform",
        "UserJoinedPlatformEvent",
        event_id,
        data,
        "platform UserJoinedPlatformEvent JSON did not match UserJoinedPlatformEvent",
    )?;
    let joined_at = ms_to_naive(ev.timestamp, checkpoint_timestamp_ms);

    let membership = NewPlatformMembership {
        platform_id: ev.platform_id.clone(),
        wallet_address: ev.wallet_address,
        joined_at,
    };

    let now = common::chain_time_from_ms(common::chain_timestamp_ms(Some(ev.timestamp as i64), checkpoint_timestamp_ms)).naive_utc();
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: UserLeftPlatformEvent = common::deserialize_social_event_json(
        "platform",
        "UserLeftPlatformEvent",
        event_id,
        data,
        "platform UserLeftPlatformEvent JSON did not match UserLeftPlatformEvent",
    )?;
    let now = common::chain_time_from_ms(common::chain_timestamp_ms(common::json_field_as_i64(data.get("timestamp")), checkpoint_timestamp_ms)).naive_utc();
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: TokenAirdropEvent = common::deserialize_social_event_json(
        "platform",
        "TokenAirdropEvent",
        event_id,
        data,
        "platform TokenAirdropEvent JSON did not match TokenAirdropEvent",
    )?;
    let now = common::chain_time_from_ms(common::chain_timestamp_ms(Some(ev.timestamp as i64), checkpoint_timestamp_ms)).naive_utc();

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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: PlatformDeletedEvent = common::deserialize_social_event_json(
        "platform",
        "PlatformDeletedEvent",
        event_id,
        data,
        "platform PlatformDeletedEvent JSON did not match PlatformDeletedEvent",
    )?;
    let PlatformDeletedEvent {
        platform_id,
        name,
        developer,
        deleted_by,
        timestamp,
        reasoning,
    } = ev;
    std::mem::drop((name, developer, deleted_by));
    let deleted_at = ms_to_naive(timestamp, checkpoint_timestamp_ms);

    let now = common::chain_time_from_ms(common::chain_timestamp_ms(
        Some(timestamp as i64),
        checkpoint_timestamp_ms,
    ))
    .naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "PlatformDeleted".to_string(),
        platform_id: platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning,
    };

    Some(vec![
        SocialEventRow::PlatformDeleted {
            platform_id,
            deleted_at,
        },
        SocialEventRow::PlatformEvent(platform_event),
    ])
}

fn process_treasury_funded_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: TreasuryFundedEvent = common::deserialize_social_event_json(
        "platform",
        "TreasuryFundedEvent",
        event_id,
        data,
        "platform TreasuryFundedEvent JSON did not match TreasuryFundedEvent",
    )?;
    let now = common::chain_time_from_ms(common::chain_timestamp_ms(Some(ev._timestamp as i64), checkpoint_timestamp_ms)).naive_utc();
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

#[cfg(test)]
mod platform_deleted_tests {
    use chrono::{TimeZone, Utc};

    use super::handle_platform_event;
    use crate::handlers::common;
    use crate::handlers::SocialEventRow;

    const CK_MS: u64 = 1_700_000_000_000;

    fn naive_from_chain_ms(ms: u64) -> chrono::NaiveDateTime {
        common::chain_time_from_ms(common::chain_timestamp_ms(
            if ms > 0 { Some(ms as i64) } else { None },
            CK_MS,
        ))
        .naive_utc()
    }

    #[test]
    fn platform_deleted_event_json_through_handler() {
        let ts_ms = 1_735_891_200_000u64;
        let json = serde_json::json!({
            "platform_id": "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
            "name": "Removed",
            "developer": "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "deleted_by": "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
            "timestamp": ts_ms,
            "reasoning": "cleanup",
        });
        let event_id = "digest:42";
        let rows = handle_platform_event("PlatformDeletedEvent", &json, event_id, CK_MS)
            .expect("handler should recognize PlatformDeletedEvent");
        assert_eq!(rows.len(), 2);

        let SocialEventRow::PlatformDeleted {
            platform_id,
            deleted_at,
        } = &rows[0]
        else {
            panic!("expected PlatformDeleted row first, got {:?}", rows[0]);
        };
        assert_eq!(
            platform_id.as_str(),
            "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
        );
        assert_eq!(deleted_at, &naive_from_chain_ms(ts_ms));

        let SocialEventRow::PlatformEvent(ev) = &rows[1] else {
            panic!("expected PlatformEvent row second, got {:?}", rows[1]);
        };
        assert_eq!(ev.event_type, "PlatformDeleted");
        assert_eq!(ev.event_id.as_deref(), Some(event_id));
        assert_eq!(ev.platform_id.as_str(), platform_id.as_str());
        assert_eq!(ev.reasoning.as_deref(), Some("cleanup"));
        assert_eq!(&ev.event_data, &json);
    }
}
