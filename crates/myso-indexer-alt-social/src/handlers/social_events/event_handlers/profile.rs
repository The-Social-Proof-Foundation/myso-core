// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Pipeline pattern follows myso-indexer-alt.

use chrono::Utc;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use super::{ProfileUpdate, SocialEventRow};
use myso_indexer_alt_social_schema::models::{NewProfile, NewProfileEvent};

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

/// Event emitted when a profile is created. Ported from mys-indexer.
#[derive(Debug, Clone, Deserialize)]
struct ProfileCreatedEvent {
    #[serde(rename = "profile_id", alias = "id", default)]
    profile_id: String,

    #[serde(rename = "owner_address", alias = "owner", default)]
    owner_address: String,

    #[serde(default)]
    username: Option<String>,

    #[serde(rename = "display_name", default)]
    display_name: String,

    #[serde(
        rename = "profile_photo",
        alias = "profile_picture",
        alias = "avatar_url",
        default
    )]
    profile_photo: Option<String>,

    #[serde(rename = "cover_photo", alias = "cover_url", default)]
    cover_photo: Option<String>,

    #[serde(default)]
    bio: Option<String>,

    #[serde(
        rename = "created_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    _created_at: u64,
}

/// Event emitted when a profile is updated. Ported from mys-indexer.
#[derive(Debug, Clone, Deserialize)]
struct ProfileUpdatedEvent {
    #[serde(rename = "profile_id", alias = "id", default)]
    profile_id: String,

    #[serde(rename = "display_name", default)]
    display_name: Option<String>,

    #[serde(default)]
    username: Option<String>,

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

    #[serde(rename = "bio", alias = "description", default)]
    bio: Option<String>,

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

    #[serde(default)]
    facebook_username: Option<String>,

    #[serde(default)]
    reddit_username: Option<String>,

    #[serde(default)]
    github_username: Option<String>,

    #[serde(default)]
    instagram_username: Option<String>,

    #[serde(default)]
    linkedin_username: Option<String>,

    #[serde(default)]
    twitch_username: Option<String>,

    #[serde(default, deserialize_with = "deserialize_optional_number_from_string")]
    min_offer_amount: Option<i64>,
}

impl ProfileCreatedEvent {
    fn into_model(&self) -> NewProfile {
        let now = Utc::now().naive_utc();

        let username = match &self.username {
            Some(name) if !name.is_empty() => name.clone(),
            _ => format!(
                "user_{}",
                self.owner_address.chars().take(8).collect::<String>()
            ),
        };

        NewProfile {
            owner_address: self.owner_address.clone(),
            username,
            display_name: if self.display_name.is_empty() {
                None
            } else {
                Some(self.display_name.clone())
            },
            bio: self.bio.clone(),
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
            facebook_username: None,
            reddit_username: None,
            github_username: None,
            instagram_username: None,
            linkedin_username: None,
            twitch_username: None,
            social_proof_token_address: None,
            reservation_pool_address: None,
            selected_badge_id: None,
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
        "ProfileCreatedEvent" => process_profile_created_event(data),
        "ProfileUpdatedEvent" => process_profile_updated_event(data),
        "UsernameRegisteredEvent" => process_username_registered_event(data),
        "UsernameUpdatedEvent" => process_username_updated_event(data),
        "BadgeAssignedEvent" => process_badge_assigned_event(data, event_id),
        "BadgeRevokedEvent" => process_badge_revoked_event(data, event_id),
        "BadgeSelectedEvent" => process_badge_selected_event(data, event_id),
        "PaidMessagingSettingsUpdatedEvent" => process_paid_messaging_settings_updated_event(data),
        _ => None,
    }
}

fn process_profile_created_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileCreatedEvent = serde_json::from_value(data.clone()).ok()?;
    let profile = ev.into_model();
    Some(vec![SocialEventRow::Profile(profile)])
}

fn process_profile_updated_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileUpdatedEvent = serde_json::from_value(data.clone()).ok()?;
    let up = ProfileUpdate {
        profile_id: ev.profile_id,
        owner_address: ev.owner_address,
        display_name: ev.display_name,
        bio: ev.bio,
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
        facebook_username: ev.facebook_username,
        reddit_username: ev.reddit_username,
        github_username: ev.github_username,
        instagram_username: ev.instagram_username,
        linkedin_username: ev.linkedin_username,
        twitch_username: ev.twitch_username,
        min_offer_amount: ev.min_offer_amount,
        username: ev.username,
        selected_badge_id: None,
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
    };
    Some(vec![SocialEventRow::ProfileUpdate(up)])
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

fn process_username_registered_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameRegisteredEvent = serde_json::from_value(data.clone()).ok()?;
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
        facebook_username: None,
        reddit_username: None,
        github_username: None,
        instagram_username: None,
        linkedin_username: None,
        twitch_username: None,
        min_offer_amount: None,
        username: Some(ev.username),
        selected_badge_id: None,
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
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

fn process_username_updated_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameUpdatedEvent = serde_json::from_value(data.clone()).ok()?;
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
        facebook_username: None,
        reddit_username: None,
        github_username: None,
        instagram_username: None,
        linkedin_username: None,
        twitch_username: None,
        min_offer_amount: None,
        username: Some(ev.new_username),
        selected_badge_id: None,
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
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
    let ev: BadgeAssignedEvent = serde_json::from_value(data.clone()).ok()?;
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
    let ev: BadgeRevokedEvent = serde_json::from_value(data.clone()).ok()?;
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

fn process_badge_selected_event(
    data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: BadgeSelectedEvent = serde_json::from_value(data.clone()).ok()?;
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
        facebook_username: None,
        reddit_username: None,
        github_username: None,
        instagram_username: None,
        linkedin_username: None,
        twitch_username: None,
        min_offer_amount: None,
        username: None,
        selected_badge_id: Some(ev.badge_id),
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
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
) -> Option<Vec<SocialEventRow>> {
    let ev: PaidMessagingSettingsUpdatedEvent = serde_json::from_value(data.clone()).ok()?;
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
        facebook_username: None,
        reddit_username: None,
        github_username: None,
        instagram_username: None,
        linkedin_username: None,
        twitch_username: None,
        min_offer_amount: None,
        username: None,
        selected_badge_id: None,
        paid_messaging_enabled: Some(ev.enabled),
        paid_messaging_min_cost: ev.min_cost.map(|v| v as i64),
    };
    Some(vec![SocialEventRow::ProfileUpdate(up)])
}
