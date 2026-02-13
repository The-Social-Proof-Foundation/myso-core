// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;

use super::{ProfileUpdate, SocialEventRow};
use myso_indexer_alt_social_schema::models::{NewProfile, NewProfileEvent};

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
    #[derive(serde::Deserialize)]
    struct ProfileCreatedJson {
        profile_id: Option<String>,
        display_name: Option<String>,
        username: Option<String>,
        bio: Option<String>,
        profile_picture: Option<String>,
        cover_photo: Option<String>,
        #[serde(alias = "owner", alias = "owner_address")]
        owner_address: String,
    }
    let ev: ProfileCreatedJson = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now().naive_utc();
    let username = ev.username.filter(|s| !s.is_empty()).unwrap_or_else(|| {
        format!(
            "user_{}",
            ev.owner_address.chars().take(10).collect::<String>()
        )
    });
    let profile = NewProfile {
        owner_address: ev.owner_address,
        username,
        display_name: ev.display_name.filter(|s| !s.is_empty()),
        bio: ev.bio.filter(|s| !s.is_empty()),
        profile_photo: ev.profile_picture,
        website: None,
        created_at: now,
        updated_at: now,
        cover_photo: ev.cover_photo,
        profile_id: ev.profile_id,
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
    };
    Some(vec![SocialEventRow::Profile(profile)])
}

fn process_profile_updated_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct ProfileUpdatedJson {
        #[serde(alias = "profile_id", alias = "id")]
        profile_id: String,
        #[serde(alias = "owner", alias = "owner_address")]
        owner_address: String,
        display_name: Option<String>,
        bio: Option<String>,
        #[serde(
            alias = "profile_photo",
            alias = "profile_picture",
            alias = "avatar_url"
        )]
        profile_photo: Option<String>,
        #[serde(alias = "cover_photo", alias = "cover_url")]
        cover_photo: Option<String>,
        birthdate: Option<String>,
        current_location: Option<String>,
        raised_location: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        gender: Option<String>,
        political_view: Option<String>,
        religion: Option<String>,
        education: Option<String>,
        primary_language: Option<String>,
        relationship_status: Option<String>,
        x_username: Option<String>,
        facebook_username: Option<String>,
        reddit_username: Option<String>,
        github_username: Option<String>,
        instagram_username: Option<String>,
        linkedin_username: Option<String>,
        twitch_username: Option<String>,
        #[serde(deserialize_with = "deserialize_optional_i64")]
        min_offer_amount: Option<i64>,
    }
    fn deserialize_optional_i64<'de, D>(d: D) -> Result<Option<i64>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum V {
            I(i64),
            S(String),
            N,
        }
        match V::deserialize(d) {
            Ok(V::I(n)) => Ok(Some(n)),
            Ok(V::S(s)) => s.parse().map(Some).map_err(serde::de::Error::custom),
            Ok(V::N) => Ok(None),
            Err(e) => Err(e),
        }
    }
    let ev: ProfileUpdatedJson = serde_json::from_value(data.clone()).ok()?;
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
        username: None,
        selected_badge_id: None,
        paid_messaging_enabled: None,
        paid_messaging_min_cost: None,
    };
    Some(vec![SocialEventRow::ProfileUpdate(up)])
}

fn process_username_registered_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct UsernameRegisteredJson {
        #[serde(alias = "profile_id")]
        profile_id: String,
        username: String,
        #[serde(alias = "owner", alias = "owner_address")]
        owner_address: String,
    }
    let ev: UsernameRegisteredJson = serde_json::from_value(data.clone()).ok()?;
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

fn process_username_updated_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct UsernameUpdatedJson {
        #[serde(alias = "profile_id")]
        profile_id: String,
        #[serde(alias = "new_username")]
        new_username: String,
        #[serde(alias = "owner", alias = "owner_address")]
        owner_address: String,
    }
    let ev: UsernameUpdatedJson = serde_json::from_value(data.clone()).ok()?;
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

fn process_badge_assigned_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct BadgeAssignedJson {
        #[serde(alias = "profile_id")]
        profile_id: String,
        #[serde(alias = "badge_id")]
        badge_id: String,
        name: String,
        description: Option<String>,
        #[serde(alias = "media_url")]
        media_url: Option<String>,
        #[serde(alias = "icon_url")]
        icon_url: Option<String>,
        #[serde(alias = "platform_id")]
        platform_id: String,
        #[serde(alias = "assigned_by")]
        assigned_by: String,
        #[serde(alias = "assigned_at", default)]
        assigned_at: u64,
        #[serde(alias = "badge_type", default)]
        badge_type: u8,
    }
    let ev: BadgeAssignedJson = serde_json::from_value(data.clone()).ok()?;
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

fn process_badge_revoked_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct BadgeRevokedJson {
        #[serde(alias = "profile_id")]
        profile_id: String,
        #[serde(alias = "badge_id")]
        badge_id: String,
        #[serde(alias = "platform_id")]
        platform_id: String,
        #[serde(alias = "revoked_by")]
        revoked_by: String,
        #[serde(alias = "revoked_at", default)]
        revoked_at: u64,
    }
    let ev: BadgeRevokedJson = serde_json::from_value(data.clone()).ok()?;
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

fn process_badge_selected_event(
    data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct BadgeSelectedJson {
        #[serde(alias = "profile_id")]
        profile_id: String,
        #[serde(alias = "badge_id")]
        badge_id: String,
        #[serde(alias = "owner", alias = "selected_by")]
        selected_by: String,
    }
    let ev: BadgeSelectedJson = serde_json::from_value(data.clone()).ok()?;
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

fn process_paid_messaging_settings_updated_event(
    data: &serde_json::Value,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct PaidMessagingJson {
        #[serde(alias = "profile_id")]
        profile_id: String,
        #[serde(alias = "owner")]
        owner: String,
        enabled: bool,
        #[serde(alias = "min_cost")]
        min_cost: Option<u64>,
    }
    let ev: PaidMessagingJson = serde_json::from_value(data.clone()).ok()?;
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
