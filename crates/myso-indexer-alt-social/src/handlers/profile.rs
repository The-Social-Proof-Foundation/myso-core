// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Pipeline pattern follows myso-indexer-alt.

use chrono::{TimeZone, Utc};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use super::common;
use super::{ProfileUpdate, SocialEventRow};
use myso_indexer_alt_social_schema::models::{
    NewEcosystemTreasury, NewProfile, NewProfileEvent, NewProfileOffer, NewProfileSaleFee,
    NewVestingEvent, NewVestingWallet,
};

fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber<T> {
        String(String),
        Number(T),
    }

    match StringOrNumber::<T>::deserialize(deserializer) {
        Ok(StringOrNumber::String(s)) => T::from_str(&s).map_err(serde::de::Error::custom),
        Ok(StringOrNumber::Number(n)) => Ok(n),
        Err(e) => Err(e),
    }
}

fn default_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn deserialize_optional_number_from_string<'de, T, D>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    T: FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumberOrNone<T> {
        String(String),
        Number(T),
        None,
    }

    match StringOrNumberOrNone::<T>::deserialize(deserializer) {
        Ok(StringOrNumberOrNone::String(s)) => {
            if s.is_empty() {
                Ok(None)
            } else {
                T::from_str(&s).map(Some).map_err(serde::de::Error::custom)
            }
        }
        Ok(StringOrNumberOrNone::Number(n)) => Ok(Some(n)),
        Ok(StringOrNumberOrNone::None) => Ok(None),
        Err(_) => Ok(None),
    }
}

/// Event emitted when a profile is created. Aligned with Move ProfileCreatedEvent.
#[derive(Debug, Clone, Deserialize)]
struct ProfileCreatedEvent {
    #[serde(rename = "profile_id", alias = "id")]
    profile_id: String,

    #[serde(rename = "owner_address", alias = "owner")]
    owner_address: String,

    username: String,

    #[serde(rename = "display_name")]
    display_name: String,

    bio: String,

    #[serde(
        rename = "profile_photo",
        alias = "profile_picture",
        alias = "avatar_url"
    )]
    profile_photo: Option<String>,

    #[serde(rename = "cover_photo", alias = "cover_url")]
    cover_photo: Option<String>,

    #[serde(
        rename = "created_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    _created_at: u64,
}

/// Event emitted when a profile is updated. Aligned with Move ProfileUpdatedEvent.
#[derive(Debug, Clone, Deserialize)]
struct ProfileUpdatedEvent {
    #[serde(rename = "profile_id", alias = "id", default)]
    profile_id: String,

    #[serde(rename = "display_name", default)]
    display_name: Option<String>,

    username: String,

    #[serde(rename = "owner_address", alias = "owner", default)]
    owner_address: String,

    #[serde(
        rename = "profile_photo",
        alias = "profile_picture",
        alias = "avatar_url",
        default
    )]
    profile_photo: Option<String>,

    #[serde(rename = "cover_photo", alias = "cover_url", default)]
    cover_photo: Option<String>,

    #[serde(rename = "bio", alias = "description")]
    bio: String,

    #[serde(default)]
    birthdate: Option<String>,

    #[serde(default)]
    current_location: Option<String>,

    #[serde(default)]
    raised_location: Option<String>,

    #[serde(default)]
    phone: Option<String>,

    #[serde(default)]
    email: Option<String>,

    #[serde(default)]
    gender: Option<String>,

    #[serde(default)]
    political_view: Option<String>,

    #[serde(default)]
    religion: Option<String>,

    #[serde(default)]
    education: Option<String>,

    #[serde(default)]
    primary_language: Option<String>,

    #[serde(default)]
    relationship_status: Option<String>,

    #[serde(default)]
    x_username: Option<String>,

    #[serde(default, deserialize_with = "deserialize_optional_number_from_string")]
    min_offer_amount: Option<i64>,
}

/// Emitted when an EcosystemBadgeAdminCap holder sets or clears `x_username` on-chain.
#[derive(Debug, Clone, Deserialize)]
struct ProfileXUsernameUpdatedEvent {
    #[serde(rename = "profile_id", alias = "id", default)]
    profile_id: String,

    #[serde(rename = "owner_address", alias = "owner", default)]
    owner_address: String,

    x_username: Option<String>,

    #[serde(default)]
    updated_by: String,

    #[serde(
        rename = "updated_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    updated_at: u64,
}

impl ProfileCreatedEvent {
    fn into_model(&self) -> NewProfile {
        let now = Utc::now().naive_utc();

        NewProfile {
            owner_address: self.owner_address.clone(),
            username: self.username.clone(),
            display_name: if self.display_name.is_empty() {
                None
            } else {
                Some(self.display_name.clone())
            },
            bio: if self.bio.is_empty() {
                None
            } else {
                Some(self.bio.clone())
            },
            profile_photo: self.profile_photo.clone(),
            website: None,
            created_at: now,
            updated_at: now,
            cover_photo: self.cover_photo.clone(),
            profile_id: if self.profile_id.is_empty() {
                None
            } else {
                Some(self.profile_id.clone())
            },
            followers_count: 0,
            following_count: 0,
            blocked_count: 0,
            post_count: 0,
            min_offer_amount: None,
            birthdate: None,
            current_location: None,
            raised_location: None,
            phone: None,
            email: None,
            gender: None,
            political_view: None,
            religion: None,
            education: None,
            primary_language: None,
            relationship_status: None,
            x_username: None,
            social_proof_token_address: None,
            reservation_pool_address: None,
            selected_badge_id: None,
            selected_ecosystem_badge_id: None,
            paid_messaging_enabled: false,
            paid_messaging_min_cost: None,
        }
    }
}

pub fn handle_profile_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "ProfileCreatedEvent" => process_profile_created_event(data, event_id),
        "ProfileUpdatedEvent" => process_profile_updated_event(data, event_id),
        "ProfileXUsernameUpdatedEvent" => process_profile_x_username_updated_event(data, event_id),
        "UsernameRegisteredEvent" => process_username_registered_event(data, event_id),
        "UsernameUpdatedEvent" => process_username_updated_event(data, event_id),
        "BadgeAssignedEvent" => process_badge_assigned_event(data, event_id),
        "BadgeRevokedEvent" => process_badge_revoked_event(data, event_id),
        "BadgeRemovedEvent" => process_badge_removed_event(data, event_id),
        "BadgeSelectedEvent" => process_badge_selected_event(data, event_id),
        "EcosystemBadgeSelectionClearedEvent" => {
            process_ecosystem_badge_selection_cleared_event(data, event_id)
        }
        "PaidMessagingSettingsUpdatedEvent" => {
            process_paid_messaging_settings_updated_event(data, event_id)
        }
        "TokensVestedEvent" => process_tokens_vested_event(data, event_id),
        "TokensClaimedEvent" => process_tokens_claimed_event(data, event_id),
        "VestingWalletDeletedEvent" => process_vesting_wallet_deleted_event(data, event_id),
        "EcosystemTreasuryUpdatedEvent" => process_ecosystem_treasury_updated_event(data, event_id),
        "ProfileOfferCreatedEvent" => process_profile_offer_created_event(data, event_id),
        "ProfileOfferAcceptedEvent" => process_profile_offer_accepted_event(data, event_id),
        "ProfileOfferRejectedEvent" => process_profile_offer_rejected_event(data, event_id),
        "ProfileSaleFeeEvent" => process_profile_sale_fee_event(data, event_id),
        _ => None,
    }
}

fn process_profile_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileCreatedEvent = match serde_json::from_value(data.clone()) {
        Ok(e) => e,
        Err(e) => {
            let keys: Vec<String> = data
                .as_object()
                .map(|o| o.keys().cloned().collect())
                .unwrap_or_default();
            tracing::warn!(
                event_id = %event_id,
                error = %e,
                json_keys = ?keys,
                "ProfileCreatedEvent JSON deserialization failed"
            );
            return None;
        }
    };
    tracing::debug!(
        event_id = %event_id,
        owner_address = %ev.owner_address,
        username = ?ev.username,
        display_name = ?ev.display_name,
        bio_len = ev.bio.len(),
        profile_photo = ev.profile_photo.as_ref().map(|_| "set").unwrap_or("none"),
        cover_photo = ev.cover_photo.as_ref().map(|_| "set").unwrap_or("none"),
        "ProfileCreatedEvent parsed successfully, indexing"
    );
    let now = Utc::now().naive_utc();
    let profile_id = ev.profile_id.clone();
    let owner_address = ev.owner_address.clone();
    let profile = ev.into_model();

    let audit_event = NewProfileEvent {
        event_type: "ProfileCreated".to_string(),
        profile_id: if profile_id.is_empty() {
            owner_address.clone()
        } else {
            profile_id
        },
        event_data: serde_json::json!({
            "owner_address": owner_address,
            "username": profile.username,
            "display_name": profile.display_name,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };

    Some(vec![
        SocialEventRow::Profile(profile),
        SocialEventRow::ProfileEvent(audit_event),
    ])
}

fn process_profile_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileUpdatedEvent = common::deserialize_social_event_json(
        "profile",
        "ProfileUpdatedEvent",
        event_id,
        data,
        "profile ProfileUpdatedEvent JSON did not match ProfileUpdatedEvent",
    )?;
    let now = Utc::now().naive_utc();
    let profile_id = ev.profile_id.clone();
    let owner_address = ev.owner_address.clone();

    let audit_event = NewProfileEvent {
        event_type: "ProfileUpdated".to_string(),
        profile_id: profile_id.clone(),
        event_data: serde_json::json!({
            "owner_address": owner_address,
            "display_name": ev.display_name,
            "username": ev.username,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };

    let up = ProfileUpdate {
        profile_id: ev.profile_id,
        owner_address: ev.owner_address,
        display_name: ev.display_name,
        bio: if ev.bio.is_empty() {
            None
        } else {
            Some(ev.bio)
        },
        profile_photo: ev.profile_photo,
        cover_photo: ev.cover_photo,
        birthdate: ev.birthdate,
        current_location: ev.current_location,
        raised_location: ev.raised_location,
        phone: ev.phone,
        email: ev.email,
        gender: ev.gender,
        political_view: ev.political_view,
        religion: ev.religion,
        education: ev.education,
        primary_language: ev.primary_language,
        relationship_status: ev.relationship_status,
        x_username: ev.x_username,
        min_offer_amount: ev.min_offer_amount,
        username: Some(ev.username),
        selected_badge_id: None,
        selected_ecosystem_badge_id: None,
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
        reservation_pool_address: None,
        social_proof_token_address: None,
    };
    Some(vec![
        SocialEventRow::ProfileUpdate(up),
        SocialEventRow::ProfileEvent(audit_event),
    ])
}

fn process_profile_x_username_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileXUsernameUpdatedEvent = common::deserialize_social_event_json(
        "profile",
        "ProfileXUsernameUpdatedEvent",
        event_id,
        data,
        "profile ProfileXUsernameUpdatedEvent JSON did not match ProfileXUsernameUpdatedEvent",
    )?;
    let now = Utc::now().naive_utc();
    let profile_id = ev.profile_id.clone();
    let owner_address = ev.owner_address.clone();

    let audit_event = NewProfileEvent {
        event_type: "ProfileXUsernameUpdated".to_string(),
        profile_id: profile_id.clone(),
        event_data: serde_json::json!({
            "owner_address": owner_address,
            "x_username": ev.x_username,
            "updated_by": ev.updated_by,
            "updated_at": ev.updated_at,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };

    Some(vec![
        SocialEventRow::ProfileXUsernameUpdate {
            profile_id: ev.profile_id,
            owner_address: ev.owner_address,
            x_username: ev.x_username,
        },
        SocialEventRow::ProfileEvent(audit_event),
    ])
}

fn process_ecosystem_treasury_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let updated_by = data.get("updated_by")?.as_str()?.to_string();
    let new_treasury_address = data.get("new_treasury_address")?.as_str()?.to_string();
    let timestamp_ms = data.get("timestamp")?.as_u64().unwrap_or(0) as i64;
    let time = if timestamp_ms > 0 {
        Utc.timestamp_millis_opt(timestamp_ms)
            .single()
            .unwrap_or_else(Utc::now)
    } else {
        Utc::now()
    };
    let row = NewEcosystemTreasury {
        treasury_address: new_treasury_address,
        updated_by,
        timestamp_ms,
        time,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::EcosystemTreasury(row)])
}

/// Event emitted when a username is registered. Ported from mys-indexer.
#[derive(Debug, Clone, Deserialize)]
struct UsernameRegisteredEvent {
    #[serde(rename = "profile_id", default)]
    profile_id: String,

    #[serde(default)]
    username: String,

    #[serde(rename = "owner_address", alias = "owner", default)]
    owner_address: String,
}

fn process_username_registered_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameRegisteredEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameRegisteredEvent",
        event_id,
        data,
        "profile UsernameRegisteredEvent JSON did not match UsernameRegisteredEvent",
    )?;
    let up = ProfileUpdate {
        profile_id: ev.profile_id,
        owner_address: ev.owner_address,
        display_name: None,
        bio: None,
        profile_photo: None,
        cover_photo: None,
        birthdate: None,
        current_location: None,
        raised_location: None,
        phone: None,
        email: None,
        gender: None,
        political_view: None,
        religion: None,
        education: None,
        primary_language: None,
        relationship_status: None,
        x_username: None,
        min_offer_amount: None,
        username: Some(ev.username),
        selected_badge_id: None,
        selected_ecosystem_badge_id: None,
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
        reservation_pool_address: None,
        social_proof_token_address: None,
    };
    Some(vec![SocialEventRow::ProfileUpdate(up)])
}

/// Event emitted when a username is updated. Ported from mys-indexer.
#[derive(Debug, Clone, Deserialize)]
struct UsernameUpdatedEvent {
    #[serde(rename = "profile_id", default)]
    profile_id: String,

    #[serde(rename = "new_username", default)]
    new_username: String,

    #[serde(rename = "owner_address", alias = "owner", default)]
    owner_address: String,
}

fn process_username_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameUpdatedEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameUpdatedEvent",
        event_id,
        data,
        "profile UsernameUpdatedEvent JSON did not match UsernameUpdatedEvent",
    )?;
    let up = ProfileUpdate {
        profile_id: ev.profile_id,
        owner_address: ev.owner_address,
        display_name: None,
        bio: None,
        profile_photo: None,
        cover_photo: None,
        birthdate: None,
        current_location: None,
        raised_location: None,
        phone: None,
        email: None,
        gender: None,
        political_view: None,
        religion: None,
        education: None,
        primary_language: None,
        relationship_status: None,
        x_username: None,
        min_offer_amount: None,
        username: Some(ev.new_username),
        selected_badge_id: None,
        selected_ecosystem_badge_id: None,
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
        reservation_pool_address: None,
        social_proof_token_address: None,
    };
    Some(vec![SocialEventRow::ProfileUpdate(up)])
}

fn default_zero_u8() -> u8 {
    0
}

/// Event emitted when a badge is assigned. Ported from mys-indexer.
#[derive(Debug, Clone, Deserialize)]
struct BadgeAssignedEvent {
    #[serde(rename = "profile_id", default)]
    profile_id: String,

    #[serde(rename = "badge_id", default)]
    badge_id: String,

    #[serde(rename = "name", default)]
    name: String,

    #[serde(rename = "description", default)]
    description: Option<String>,

    #[serde(rename = "media_url", default)]
    media_url: Option<String>,

    #[serde(rename = "icon_url", default)]
    icon_url: Option<String>,

    #[serde(rename = "platform_id", default)]
    platform_id: String,

    #[serde(rename = "assigned_by", default)]
    assigned_by: String,

    #[serde(
        rename = "assigned_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    assigned_at: u64,

    #[serde(
        rename = "badge_type",
        default = "default_zero_u8",
        deserialize_with = "deserialize_number_from_string"
    )]
    badge_type: u8,
}

fn process_badge_assigned_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: BadgeAssignedEvent = common::deserialize_social_event_json(
        "profile",
        "BadgeAssignedEvent",
        event_id,
        data,
        "profile BadgeAssignedEvent JSON did not match BadgeAssignedEvent",
    )?;
    let now = chrono::Utc::now();
    let profile_id = ev.profile_id.clone();
    let badge_id = ev.badge_id.clone();
    let name = ev.name.clone();
    let platform_id = ev.platform_id.clone();
    let assigned_by = ev.assigned_by.clone();
    let assigned_at = ev.assigned_at;
    let badge = myso_indexer_alt_social_schema::models::NewProfileBadge {
        profile_id: profile_id.clone(),
        badge_id: badge_id.clone(),
        badge_name: name.clone(),
        badge_description: ev.description,
        badge_media_url: ev.media_url,
        badge_icon_url: ev.icon_url,
        platform_id: platform_id.clone(),
        assigned_by: assigned_by.clone(),
        assigned_at: assigned_at as i64,
        revoked: false,
        revoked_at: None,
        revoked_by: None,
        badge_type: ev.badge_type as i16,
        transaction_id: event_id.to_string(),
        time: now,
    };
    let event = NewProfileEvent {
        event_type: "BadgeAssigned".to_string(),
        profile_id,
        event_data: serde_json::json!({
            "badge_id": badge_id,
            "name": name,
            "platform_id": platform_id,
            "assigned_by": assigned_by,
            "assigned_at": assigned_at,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now.naive_utc(),
        updated_at: now.naive_utc(),
    };
    Some(vec![
        SocialEventRow::ProfileBadge(badge),
        SocialEventRow::ProfileEvent(event),
    ])
}

/// Event emitted when a badge is revoked. Ported from mys-indexer.
#[derive(Debug, Clone, Deserialize)]
struct BadgeRevokedEvent {
    #[serde(rename = "profile_id", default)]
    profile_id: String,

    #[serde(rename = "badge_id", default)]
    badge_id: String,

    #[serde(rename = "platform_id", default)]
    platform_id: String,

    #[serde(rename = "revoked_by", default)]
    revoked_by: String,

    #[serde(
        rename = "revoked_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    revoked_at: u64,
}

fn process_badge_revoked_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: BadgeRevokedEvent = common::deserialize_social_event_json(
        "profile",
        "BadgeRevokedEvent",
        event_id,
        data,
        "profile BadgeRevokedEvent JSON did not match BadgeRevokedEvent",
    )?;
    let event = NewProfileEvent {
        event_type: "BadgeRevoked".to_string(),
        profile_id: ev.profile_id.clone(),
        event_data: serde_json::json!({
            "badge_id": ev.badge_id,
            "platform_id": ev.platform_id,
            "revoked_by": ev.revoked_by,
            "revoked_at": ev.revoked_at,
        }),
        event_id: Some(event_id.to_string()),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };
    Some(vec![
        SocialEventRow::ProfileBadgeRevoke {
            profile_id: ev.profile_id,
            badge_id: ev.badge_id,
            revoked_at: ev.revoked_at as i64,
            revoked_by: ev.revoked_by,
        },
        SocialEventRow::ProfileEvent(event),
    ])
}

const ECOSYSTEM_BADGE_PREFIX: &str = "ecosystem_badge_";

/// Event emitted when a badge is selected. Ported from mys-indexer.
#[derive(Debug, Clone, Deserialize)]
struct BadgeSelectedEvent {
    #[serde(rename = "profile_id", default)]
    profile_id: String,

    #[serde(rename = "badge_id", default)]
    badge_id: String,

    #[serde(rename = "selected_by", alias = "owner", default)]
    selected_by: String,
}

#[derive(Debug, Clone, Deserialize)]
struct EcosystemBadgeSelectionClearedEvent {
    #[serde(rename = "profile_id", default)]
    profile_id: String,

    #[serde(rename = "cleared_by", default)]
    cleared_by: String,
}

fn process_badge_selected_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: BadgeSelectedEvent = common::deserialize_social_event_json(
        "profile",
        "BadgeSelectedEvent",
        event_id,
        data,
        "profile BadgeSelectedEvent JSON did not match BadgeSelectedEvent",
    )?;
    let (selected_badge_id, selected_ecosystem_badge_id) = if ev.badge_id.is_empty() {
        (Some(None), Some(None))
    } else if ev.badge_id.starts_with(ECOSYSTEM_BADGE_PREFIX) {
        (None, Some(Some(ev.badge_id)))
    } else {
        (Some(Some(ev.badge_id)), None)
    };
    let up = ProfileUpdate {
        profile_id: ev.profile_id,
        owner_address: ev.selected_by,
        display_name: None,
        bio: None,
        profile_photo: None,
        cover_photo: None,
        birthdate: None,
        current_location: None,
        raised_location: None,
        phone: None,
        email: None,
        gender: None,
        political_view: None,
        religion: None,
        education: None,
        primary_language: None,
        relationship_status: None,
        x_username: None,
        min_offer_amount: None,
        username: None,
        selected_badge_id,
        selected_ecosystem_badge_id,
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
        reservation_pool_address: None,
        social_proof_token_address: None,
    };
    Some(vec![SocialEventRow::ProfileUpdate(up)])
}

fn process_ecosystem_badge_selection_cleared_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: EcosystemBadgeSelectionClearedEvent = common::deserialize_social_event_json(
        "profile",
        "EcosystemBadgeSelectionClearedEvent",
        event_id,
        data,
        "profile EcosystemBadgeSelectionClearedEvent JSON did not match",
    )?;
    let up = ProfileUpdate {
        profile_id: ev.profile_id,
        owner_address: ev.cleared_by,
        display_name: None,
        bio: None,
        profile_photo: None,
        cover_photo: None,
        birthdate: None,
        current_location: None,
        raised_location: None,
        phone: None,
        email: None,
        gender: None,
        political_view: None,
        religion: None,
        education: None,
        primary_language: None,
        relationship_status: None,
        x_username: None,
        min_offer_amount: None,
        username: None,
        selected_badge_id: None,
        selected_ecosystem_badge_id: Some(None),
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
        reservation_pool_address: None,
        social_proof_token_address: None,
    };
    Some(vec![SocialEventRow::ProfileUpdate(up)])
}

/// Event emitted when paid messaging settings are updated. Ported from mys-indexer.
#[derive(Debug, Clone, Deserialize)]
struct PaidMessagingSettingsUpdatedEvent {
    #[serde(rename = "profile_id", default)]
    profile_id: String,

    #[serde(rename = "owner", default)]
    owner: String,

    #[serde(default)]
    enabled: bool,

    #[serde(
        rename = "min_cost",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    min_cost: Option<u64>,
}

fn process_paid_messaging_settings_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PaidMessagingSettingsUpdatedEvent = common::deserialize_social_event_json(
        "profile",
        "PaidMessagingSettingsUpdatedEvent",
        event_id,
        data,
        "profile PaidMessagingSettingsUpdatedEvent JSON did not match PaidMessagingSettingsUpdatedEvent",
    )?;
    let up = ProfileUpdate {
        profile_id: ev.profile_id.clone(),
        owner_address: ev.owner,
        display_name: None,
        bio: None,
        profile_photo: None,
        cover_photo: None,
        birthdate: None,
        current_location: None,
        raised_location: None,
        phone: None,
        email: None,
        gender: None,
        political_view: None,
        religion: None,
        education: None,
        primary_language: None,
        relationship_status: None,
        x_username: None,
        min_offer_amount: None,
        username: None,
        selected_badge_id: None,
        selected_ecosystem_badge_id: None,
        paid_messaging_enabled: Some(ev.enabled),
        paid_messaging_min_cost: ev.min_cost.map(|v| v as i64),
        reservation_pool_address: None,
        social_proof_token_address: None,
    };
    Some(vec![SocialEventRow::ProfileUpdate(up)])
}

#[derive(Debug, Clone, Deserialize)]
struct BadgeRemovedEvent {
    #[serde(rename = "profile_id", default)]
    profile_id: String,
    #[serde(rename = "badge_id", default)]
    badge_id: String,
    #[serde(rename = "platform_id", default)]
    _platform_id: String,
    #[serde(rename = "removed_by", default)]
    removed_by: String,
    #[serde(
        rename = "removed_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    removed_at: u64,
}

fn process_badge_removed_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: BadgeRemovedEvent = common::deserialize_social_event_json(
        "profile",
        "BadgeRemovedEvent",
        event_id,
        data,
        "profile BadgeRemovedEvent JSON did not match BadgeRemovedEvent",
    )?;
    let event = NewProfileEvent {
        event_type: "BadgeRemoved".to_string(),
        profile_id: ev.profile_id.clone(),
        event_data: serde_json::json!({
            "badge_id": ev.badge_id,
            "removed_by": ev.removed_by,
            "removed_at": ev.removed_at,
        }),
        event_id: Some(event_id.to_string()),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };
    Some(vec![
        SocialEventRow::ProfileBadgeRevoke {
            profile_id: ev.profile_id,
            badge_id: ev.badge_id,
            revoked_at: ev.removed_at as i64,
            revoked_by: ev.removed_by,
        },
        SocialEventRow::ProfileEvent(event),
    ])
}

#[derive(Debug, Clone, Deserialize)]
struct TokensVestedEvent {
    #[serde(rename = "wallet_id", default)]
    wallet_id: String,
    #[serde(rename = "owner", default)]
    owner: String,
    #[serde(
        rename = "total_amount",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    total_amount: Option<u64>,
    #[serde(
        rename = "start_time",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    start_time: Option<u64>,
    #[serde(
        rename = "duration",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    duration: Option<u64>,
    #[serde(
        rename = "curve_factor",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    curve_factor: Option<u64>,
    #[serde(
        rename = "vested_at",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    vested_at: Option<u64>,
}

fn process_tokens_vested_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: TokensVestedEvent = common::deserialize_social_event_json(
        "profile",
        "TokensVestedEvent",
        event_id,
        data,
        "profile TokensVestedEvent JSON did not match TokensVestedEvent",
    )?;
    let total_amount = ev.total_amount.unwrap_or(0) as i64;
    let start_time = ev.start_time.unwrap_or(0) as i64;
    let duration = ev.duration.unwrap_or(0) as i64;
    let curve_factor = ev.curve_factor.unwrap_or(1000) as i64;
    let now = Utc::now().naive_utc();
    let wallet = NewVestingWallet {
        wallet_id: ev.wallet_id.clone(),
        owner_address: ev.owner.clone(),
        total_amount,
        start_time,
        duration,
        curve_factor,
        claimed_amount: 0,
        remaining_balance: total_amount,
        created_at: now,
        updated_at: now,
        transaction_id: event_id.to_string(),
    };
    let vest_event = NewVestingEvent {
        wallet_id: ev.wallet_id,
        event_type: "vested".to_string(),
        owner_address: ev.owner,
        amount: total_amount,
        remaining_balance: Some(total_amount),
        start_time: Some(start_time),
        duration: Some(duration),
        curve_factor: Some(curve_factor),
        event_time: ev.vested_at.unwrap_or(0) as i64,
        time: Utc::now(),
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::VestingWallet(wallet),
        SocialEventRow::VestingEvent(vest_event),
    ])
}

#[derive(Debug, Clone, Deserialize)]
struct TokensClaimedEvent {
    #[serde(rename = "wallet_id", default)]
    wallet_id: String,
    #[serde(rename = "owner", default)]
    owner: String,
    #[serde(
        rename = "claimed_amount",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    claimed_amount: Option<u64>,
    #[serde(
        rename = "remaining_balance",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    remaining_balance: Option<u64>,
    #[serde(
        rename = "claimed_at",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    claimed_at: Option<u64>,
}

fn process_tokens_claimed_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: TokensClaimedEvent = common::deserialize_social_event_json(
        "profile",
        "TokensClaimedEvent",
        event_id,
        data,
        "profile TokensClaimedEvent JSON did not match TokensClaimedEvent",
    )?;
    let claimed_amount = ev.claimed_amount.unwrap_or(0) as i64;
    let remaining_balance = ev.remaining_balance.unwrap_or(0) as i64;
    let vest_event = NewVestingEvent {
        wallet_id: ev.wallet_id.clone(),
        event_type: "claimed".to_string(),
        owner_address: ev.owner.clone(),
        amount: claimed_amount,
        remaining_balance: Some(remaining_balance),
        start_time: None,
        duration: None,
        curve_factor: None,
        event_time: ev.claimed_at.unwrap_or(0) as i64,
        time: Utc::now(),
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::VestingEvent(vest_event),
        SocialEventRow::VestingWalletClaimUpdate {
            wallet_id: ev.wallet_id,
            claimed_amount,
            remaining_balance,
        },
    ])
}

#[derive(Debug, Clone, Deserialize)]
struct ProfileOfferCreatedEvent {
    #[serde(rename = "profile_id")]
    profile_id: String,
    #[serde(rename = "offeror")]
    offeror: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    amount: u64,
    #[serde(
        rename = "created_at",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    created_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct ProfileOfferAcceptedEvent {
    #[serde(rename = "profile_id")]
    profile_id: String,
    #[serde(rename = "offeror")]
    offeror: String,
    #[serde(rename = "previous_owner")]
    _previous_owner: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    _amount: u64,
    #[serde(
        rename = "accepted_at",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    accepted_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct ProfileOfferRejectedEvent {
    #[serde(rename = "profile_id")]
    profile_id: String,
    #[serde(rename = "offeror")]
    offeror: String,
    #[serde(rename = "rejected_by")]
    _rejected_by: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    _amount: u64,
    #[serde(
        rename = "rejected_at",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    rejected_at: u64,
    #[serde(rename = "is_revoked", default)]
    is_revoked: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ProfileSaleFeeEvent {
    #[serde(rename = "profile_id")]
    profile_id: String,
    #[serde(rename = "offeror")]
    offeror: String,
    #[serde(rename = "previous_owner")]
    previous_owner: String,
    #[serde(
        rename = "sale_amount",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    sale_amount: u64,
    #[serde(
        rename = "fee_amount",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    fee_amount: u64,
    #[serde(rename = "fee_recipient")]
    fee_recipient: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    timestamp: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct VestingWalletDeletedEvent {
    #[serde(rename = "wallet_id", default)]
    wallet_id: String,
    #[serde(rename = "owner", default)]
    _owner: String,
    #[serde(
        rename = "deleted_at",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    deleted_at: Option<u64>,
}

fn process_profile_offer_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileOfferCreatedEvent = common::deserialize_social_event_json(
        "profile",
        "ProfileOfferCreatedEvent",
        event_id,
        data,
        "profile ProfileOfferCreatedEvent JSON did not match ProfileOfferCreatedEvent",
    )?;
    let created_at = ev.created_at as i64;
    let offer = NewProfileOffer {
        profile_id: ev.profile_id,
        offeror_address: ev.offeror,
        amount: ev.amount as i64,
        status: "pending".to_string(),
        created_at,
        updated_at: created_at,
        resolved_at: None,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::ProfileOffer(offer)])
}

fn process_profile_offer_accepted_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileOfferAcceptedEvent = common::deserialize_social_event_json(
        "profile",
        "ProfileOfferAcceptedEvent",
        event_id,
        data,
        "profile ProfileOfferAcceptedEvent JSON did not match ProfileOfferAcceptedEvent",
    )?;
    let accepted_at = ev.accepted_at as i64;
    Some(vec![SocialEventRow::ProfileOfferStatusUpdate {
        profile_id: ev.profile_id,
        offeror_address: ev.offeror,
        status: "accepted".to_string(),
        resolved_at: accepted_at,
        updated_at: accepted_at,
        transaction_id: event_id.to_string(),
    }])
}

fn process_profile_offer_rejected_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileOfferRejectedEvent = common::deserialize_social_event_json(
        "profile",
        "ProfileOfferRejectedEvent",
        event_id,
        data,
        "profile ProfileOfferRejectedEvent JSON did not match ProfileOfferRejectedEvent",
    )?;
    let rejected_at = ev.rejected_at as i64;
    let status = if ev.is_revoked {
        "revoked".to_string()
    } else {
        "rejected".to_string()
    };
    Some(vec![SocialEventRow::ProfileOfferStatusUpdate {
        profile_id: ev.profile_id,
        offeror_address: ev.offeror,
        status,
        resolved_at: rejected_at,
        updated_at: rejected_at,
        transaction_id: event_id.to_string(),
    }])
}

fn process_profile_sale_fee_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileSaleFeeEvent = common::deserialize_social_event_json(
        "profile",
        "ProfileSaleFeeEvent",
        event_id,
        data,
        "profile ProfileSaleFeeEvent JSON did not match ProfileSaleFeeEvent",
    )?;
    let fee = NewProfileSaleFee {
        profile_id: ev.profile_id,
        offeror_address: ev.offeror,
        previous_owner_address: ev.previous_owner,
        sale_amount: ev.sale_amount as i64,
        fee_amount: ev.fee_amount as i64,
        fee_recipient_address: ev.fee_recipient,
        timestamp: ev.timestamp as i64,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::ProfileSaleFee(fee)])
}

fn process_vesting_wallet_deleted_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: VestingWalletDeletedEvent = common::deserialize_social_event_json(
        "profile",
        "VestingWalletDeletedEvent",
        event_id,
        data,
        "profile VestingWalletDeletedEvent JSON did not match VestingWalletDeletedEvent",
    )?;
    let vest_event = NewVestingEvent {
        wallet_id: ev.wallet_id.clone(),
        event_type: "deleted".to_string(),
        owner_address: ev._owner.clone(),
        amount: 0,
        remaining_balance: None,
        start_time: None,
        duration: None,
        curve_factor: None,
        event_time: ev.deleted_at.unwrap_or(0) as i64,
        time: Utc::now(),
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::VestingEvent(vest_event),
        SocialEventRow::VestingWalletDelete {
            wallet_id: ev.wallet_id,
        },
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::events::parse_event_contents;

    #[test]
    fn test_profile_created_bcs_to_handler_to_new_profile() {
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
        let json = parse_event_contents("profile", "ProfileCreatedEvent", &contents)
            .expect("parse_event_contents should succeed");
        let rows = handle_profile_event("ProfileCreatedEvent", &json, "test-event-id")
            .expect("handle_profile_event should return Some");
        assert_eq!(rows.len(), 2, "expect Profile + ProfileEvent");
        let (profile_row, event_row) = match (&rows[0], &rows[1]) {
            (SocialEventRow::Profile(p), SocialEventRow::ProfileEvent(e)) => (p, e),
            (SocialEventRow::ProfileEvent(e), SocialEventRow::Profile(p)) => (p, e),
            _ => panic!("expected Profile and ProfileEvent rows"),
        };
        assert_eq!(profile_row.username, "brandon");
        assert_eq!(profile_row.display_name, Some("Brandon Shaw".to_string()));
        assert_eq!(
            profile_row.bio,
            Some("Web8 developer and crypto enthusiast".to_string())
        );
        assert!(profile_row.owner_address.starts_with("0x"));
        assert_eq!(event_row.event_type, "ProfileCreated");
    }

    #[test]
    fn test_profile_x_username_updated_json_sets_handle() {
        let data = serde_json::json!({
            "profile_id": "0xaa",
            "owner_address": "0xbb",
            "x_username": "verified",
            "updated_by": "0xcc",
            "updated_at": 1700000000000u64,
        });
        let rows = handle_profile_event("ProfileXUsernameUpdatedEvent", &data, "e-x-1")
            .expect("handle_profile_event should return Some");
        assert_eq!(
            rows.len(),
            2,
            "expect ProfileXUsernameUpdate + ProfileEvent"
        );
        let (up_row, audit) = match (&rows[0], &rows[1]) {
            (SocialEventRow::ProfileXUsernameUpdate { .. }, SocialEventRow::ProfileEvent(e)) => {
                (&rows[0], e)
            }
            (SocialEventRow::ProfileEvent(e), SocialEventRow::ProfileXUsernameUpdate { .. }) => {
                (&rows[1], e)
            }
            _ => panic!("expected ProfileXUsernameUpdate and ProfileEvent"),
        };
        let SocialEventRow::ProfileXUsernameUpdate {
            profile_id,
            owner_address,
            x_username,
        } = up_row
        else {
            unreachable!()
        };
        assert_eq!(profile_id, "0xaa");
        assert_eq!(owner_address, "0xbb");
        assert_eq!(x_username.as_deref(), Some("verified"));
        assert_eq!(audit.event_type, "ProfileXUsernameUpdated");
        assert_eq!(
            audit.event_data.get("x_username").and_then(|v| v.as_str()),
            Some("verified")
        );
    }

    #[test]
    fn test_profile_x_username_updated_json_clear_handle() {
        let data = serde_json::json!({
            "profile_id": "0xaa",
            "owner_address": "0xbb",
            "x_username": serde_json::Value::Null,
            "updated_by": "0xcc",
            "updated_at": 1700000000001u64,
        });
        let rows = handle_profile_event("ProfileXUsernameUpdatedEvent", &data, "e-x-2")
            .expect("handle_profile_event should return Some");
        let up_row = rows.iter().find_map(|r| match r {
            SocialEventRow::ProfileXUsernameUpdate {
                profile_id,
                owner_address,
                x_username,
            } => Some((
                profile_id.clone(),
                owner_address.clone(),
                x_username.clone(),
            )),
            _ => None,
        });
        let Some((pid, owner, x)) = up_row else {
            panic!("expected ProfileXUsernameUpdate row");
        };
        assert_eq!(pid, "0xaa");
        assert_eq!(owner, "0xbb");
        assert_eq!(x, None);
    }

    #[test]
    fn test_profile_x_username_updated_bcs_roundtrip() {
        let pid = move_core_types::account_address::AccountAddress::random();
        let owner = move_core_types::account_address::AccountAddress::random();
        let upd = move_core_types::account_address::AccountAddress::random();
        let ev = crate::handlers::events::BcsProfileXUsernameUpdatedEvent {
            profile_id: pid,
            owner,
            x_username: Some("from_bcs".to_string()),
            updated_by: upd,
            updated_at: 42,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs serialize");
        let json = parse_event_contents("profile", "ProfileXUsernameUpdatedEvent", &bytes)
            .expect("parse_event_contents");
        let rows =
            handle_profile_event("ProfileXUsernameUpdatedEvent", &json, "e-bcs").expect("handler");
        let up_row = rows.iter().find_map(|r| match r {
            SocialEventRow::ProfileXUsernameUpdate { x_username, .. } => x_username.clone(),
            _ => None,
        });
        assert_eq!(up_row.as_deref(), Some("from_bcs"));
    }
}
