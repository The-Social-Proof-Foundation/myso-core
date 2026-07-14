// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Pipeline pattern follows myso-indexer-alt.

use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use super::common;
use super::{ProfileUpdate, SocialEventRow};
use myso_indexer_alt_social_schema::models::{
    NewEcosystemTreasury, NewProfile, NewProfileConfig, NewProfileEvent, NewUsernameListing,
    NewUsernameOffer, NewUsernameSaleFee, NewUsernameRegistry, NewUsernameReservation,
    NewVestingEvent, NewVestingWallet, USERNAME_RESERVATION_STATUS_ACTIVE,
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
    created_at: u64,
}

/// Event emitted when a profile is updated. Aligned with Move ProfileUpdatedEvent.
#[derive(Debug, Clone, Deserialize)]
struct ProfileUpdatedEvent {
    #[serde(rename = "profile_id", alias = "id", default)]
    profile_id: String,

    #[serde(rename = "display_name", default)]
    display_name: Option<String>,

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
    bio: String,

    #[serde(default)]
    website: Option<String>,

    #[serde(default)]
    birthdate: Option<String>,

    #[serde(default)]
    location: Option<String>,

    #[serde(default)]
    x_username: Option<String>,
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
    fn into_model(&self, checkpoint_timestamp_ms: u64) -> NewProfile {
        let ms = common::chain_timestamp_ms(Some(self.created_at as i64), checkpoint_timestamp_ms);
        let now = common::chain_time_from_ms(ms).naive_utc();

        NewProfile {
            owner_address: self.owner_address.clone(),
            username: String::new(),
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
            birthdate: None,
            location: None,
            x_username: None,
            social_proof_token_address: None,
            reservation_pool_address: None,
            selected_badge_id: None,
            selected_ecosystem_badge_id: None,
            memory_account_id: None,
            ai_credit_balance_id: None,
        }
    }
}

pub fn enrich_new_profile_bootstrap(
    profile: &mut NewProfile,
    memory_account_id: Option<String>,
    ai_credit_balance_id: Option<String>,
) {
    if profile.memory_account_id.is_none() {
        profile.memory_account_id = memory_account_id;
    }
    if profile.ai_credit_balance_id.is_none() {
        profile.ai_credit_balance_id = ai_credit_balance_id;
    }
}

pub fn handle_profile_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "ProfileCreatedEvent" => {
            process_profile_created_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ProfileUpdatedEvent" => {
            process_profile_updated_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ProfileXUsernameUpdatedEvent" => {
            process_profile_x_username_updated_event(data, event_id, checkpoint_timestamp_ms)
        }
        "UsernameClaimedEvent" => process_username_claimed_event(data, event_id),
        "UsernameReassignedEvent" => {
            process_username_reassigned_event(data, event_id, checkpoint_timestamp_ms)
        }
        "BadgeAssignedEvent" => {
            process_badge_assigned_event(data, event_id, checkpoint_timestamp_ms)
        }
        "BadgeRevokedEvent" => process_badge_revoked_event(data, event_id, checkpoint_timestamp_ms),
        "BadgeRemovedEvent" => process_badge_removed_event(data, event_id, checkpoint_timestamp_ms),
        "BadgeSelectedEvent" => process_badge_selected_event(data, event_id),
        "EcosystemBadgeSelectionClearedEvent" => {
            process_ecosystem_badge_selection_cleared_event(data, event_id)
        }
        "TokensVestedEvent" => process_tokens_vested_event(data, event_id, checkpoint_timestamp_ms),
        "TokensClaimedEvent" => {
            process_tokens_claimed_event(data, event_id, checkpoint_timestamp_ms)
        }
        "VestingWalletDeletedEvent" => {
            process_vesting_wallet_deleted_event(data, event_id, checkpoint_timestamp_ms)
        }
        "EcosystemTreasuryUpdatedEvent" => {
            process_ecosystem_treasury_updated_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ProfileConfigUpdatedEvent" => {
            process_profile_config_updated_event(data, event_id, checkpoint_timestamp_ms)
        }
        "UsernameListingCreatedEvent" => process_username_listing_created_event(data, event_id),
        "UsernameListingCancelledEvent" => process_username_listing_cancelled_event(data, event_id),
        "UsernameOfferCreatedEvent" => process_username_offer_created_event(data, event_id),
        "UsernameOfferAcceptedEvent" => process_username_offer_accepted_event(data, event_id),
        "UsernameOfferRejectedEvent" => process_username_offer_rejected_event(data, event_id),
        "UsernameSaleSettledEvent" => process_username_sale_settled_event(data, event_id),
        "UsernameSaleFeeEvent" => process_username_sale_fee_event(data, event_id),
        "UsernameReservedEvent" => {
            process_username_reserved_event(data, event_id, checkpoint_timestamp_ms)
        }
        "UsernameReleasedEvent" => {
            process_username_released_event(data, event_id, checkpoint_timestamp_ms)
        }
        _ => None,
    }
}

fn process_profile_created_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
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
        display_name = ?ev.display_name,
        bio_len = ev.bio.len(),
        profile_photo = ev.profile_photo.as_ref().map(|_| "set").unwrap_or("none"),
        cover_photo = ev.cover_photo.as_ref().map(|_| "set").unwrap_or("none"),
        "ProfileCreatedEvent parsed successfully, indexing"
    );
    let ms = common::chain_timestamp_ms(Some(ev.created_at as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms).naive_utc();
    let profile_id = ev.profile_id.clone();
    let owner_address = ev.owner_address.clone();
    let profile = ev.into_model(checkpoint_timestamp_ms);

    let audit_event = NewProfileEvent {
        event_type: "ProfileCreated".to_string(),
        profile_id: if profile_id.is_empty() {
            owner_address.clone()
        } else {
            profile_id
        },
        event_data: serde_json::json!({
            "owner_address": owner_address,
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileUpdatedEvent = common::deserialize_social_event_json(
        "profile",
        "ProfileUpdatedEvent",
        event_id,
        data,
        "profile ProfileUpdatedEvent JSON did not match ProfileUpdatedEvent",
    )?;
    let event_ms = common::json_field_as_i64(data.get("updated_at"));
    let ms = common::chain_timestamp_ms(event_ms, checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms).naive_utc();
    let profile_id = ev.profile_id.clone();
    let owner_address = ev.owner_address.clone();

    let audit_event = NewProfileEvent {
        event_type: "ProfileUpdated".to_string(),
        profile_id: profile_id.clone(),
        event_data: serde_json::json!({
            "owner_address": owner_address,
            "display_name": ev.display_name,
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
        website: ev.website,
        birthdate: ev.birthdate,
        location: ev.location,
        x_username: ev.x_username,
        username: None,
        selected_badge_id: None,
        selected_ecosystem_badge_id: None,
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileXUsernameUpdatedEvent = common::deserialize_social_event_json(
        "profile",
        "ProfileXUsernameUpdatedEvent",
        event_id,
        data,
        "profile ProfileXUsernameUpdatedEvent JSON did not match ProfileXUsernameUpdatedEvent",
    )?;
    let ms = common::chain_timestamp_ms(Some(ev.updated_at as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms).naive_utc();
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let updated_by = data.get("updated_by")?.as_str()?.to_string();
    let new_treasury_address = data.get("new_treasury_address")?.as_str()?.to_string();
    let event_ms = common::json_field_as_i64(data.get("timestamp"));
    let timestamp_ms = common::chain_timestamp_ms(event_ms, checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(timestamp_ms);
    let version = common::json_field_as_i64(data.get("version")).unwrap_or(0);
    let transaction_id = event_id.to_string();
    let treasury = NewEcosystemTreasury {
        treasury_address: new_treasury_address,
        updated_by,
        updated_at: timestamp_ms,
        time,
        transaction_id,
        version,
    };
    Some(vec![SocialEventRow::EcosystemTreasury(treasury)])
}

#[derive(Debug, Deserialize)]
struct ProfileConfigUpdatedEvent {
    updated_by: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    max_vesting_pieces: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    curve_factor_min: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    curve_factor_max: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    curve_precision: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    min_claim_threshold_divisor: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    min_username_length: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    max_username_length: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    username_sale_fee_bps: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    timestamp: u64,
}

fn process_profile_config_updated_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: ProfileConfigUpdatedEvent = common::deserialize_social_event_json(
        "profile",
        "ProfileConfigUpdatedEvent",
        event_id,
        data,
        "profile ProfileConfigUpdatedEvent JSON did not match ProfileConfigUpdatedEvent",
    )?;
    let event_ms = Some(ev.timestamp as i64);
    let timestamp_ms = common::chain_timestamp_ms(event_ms, checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(timestamp_ms);
    let row = NewProfileConfig {
        updated_by: ev.updated_by,
        max_vesting_pieces: ev.max_vesting_pieces as i64,
        curve_factor_min: ev.curve_factor_min as i64,
        curve_factor_max: ev.curve_factor_max as i64,
        curve_precision: ev.curve_precision as i64,
        min_claim_threshold_divisor: ev.min_claim_threshold_divisor as i64,
        min_username_length: ev.min_username_length as i64,
        max_username_length: ev.max_username_length as i64,
        username_sale_fee_bps: ev.username_sale_fee_bps as i64,
        version: 0,
        updated_at: timestamp_ms,
        time,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::ProfileConfig(row)])
}

/// Emitted when a username is claimed at profile creation.
#[derive(Debug, Clone, Deserialize)]
struct UsernameClaimedEvent {
    username: String,
    #[serde(rename = "profile_id", default)]
    profile_id: String,
}

fn process_username_claimed_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameClaimedEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameClaimedEvent",
        event_id,
        data,
        "profile UsernameClaimedEvent JSON did not match UsernameClaimedEvent",
    )?;
    let tx_id = event_id.to_string();
    let now = chrono::Utc::now().naive_utc();
    let registry = NewUsernameRegistry {
        username: ev.username.clone(),
        profile_id: ev.profile_id.clone(),
        transaction_id: tx_id,
    };
    let audit_event = NewProfileEvent {
        event_type: "UsernameClaimed".to_string(),
        profile_id: ev.profile_id.clone(),
        event_data: serde_json::json!({
            "username": ev.username,
            "profile_id": ev.profile_id,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };
    Some(vec![
        SocialEventRow::UsernameRegistryUpsert(registry),
        SocialEventRow::ProfileUsernameSet {
            profile_id: ev.profile_id,
            username: ev.username,
            owner_address: None,
        },
        SocialEventRow::ProfileEvent(audit_event),
    ])
}

/// Emitted when an admin assigns an unclaimed username to a single profile.
#[derive(Debug, Clone, Deserialize)]
struct UsernameReassignedEvent {
    username: String,
    #[serde(rename = "profile_id", default)]
    profile_id: String,
    #[serde(default)]
    admin: String,
    #[serde(default)]
    reason_code: u8,
    /// Freed when the target profile already owned a username.
    #[serde(default)]
    prior_username: Option<String>,
}

fn process_username_reassigned_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameReassignedEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameReassignedEvent",
        event_id,
        data,
        "profile UsernameReassignedEvent JSON did not match UsernameReassignedEvent",
    )?;
    let tx_id = event_id.to_string();
    let now = common::chain_time_from_ms(checkpoint_timestamp_ms as i64).naive_utc();
    let audit_event = NewProfileEvent {
        event_type: "UsernameReassigned".to_string(),
        profile_id: ev.profile_id.clone(),
        event_data: serde_json::json!({
            "username": ev.username,
            "profile_id": ev.profile_id,
            "admin": ev.admin,
            "reason_code": ev.reason_code,
            "prior_username": ev.prior_username,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };
    let mut rows = Vec::new();
    if let Some(prior) = ev.prior_username.clone() {
        if !prior.is_empty() {
            rows.push(SocialEventRow::UsernameRegistryDelete { username: prior });
        }
    }
    rows.push(SocialEventRow::UsernameRegistryUpsert(NewUsernameRegistry {
        username: ev.username.clone(),
        profile_id: ev.profile_id.clone(),
        transaction_id: tx_id,
    }));
    rows.push(SocialEventRow::ProfileUsernameSet {
        profile_id: ev.profile_id,
        username: ev.username,
        owner_address: None,
    });
    rows.push(SocialEventRow::ProfileEvent(audit_event));
    Some(rows)
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: BadgeAssignedEvent = common::deserialize_social_event_json(
        "profile",
        "BadgeAssignedEvent",
        event_id,
        data,
        "profile BadgeAssignedEvent JSON did not match BadgeAssignedEvent",
    )?;
    let ms = common::chain_timestamp_ms(Some(ev.assigned_at as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms);
    let naive = now.naive_utc();
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
        created_at: naive,
        updated_at: naive,
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: BadgeRevokedEvent = common::deserialize_social_event_json(
        "profile",
        "BadgeRevokedEvent",
        event_id,
        data,
        "profile BadgeRevokedEvent JSON did not match BadgeRevokedEvent",
    )?;
    let ms = common::chain_timestamp_ms(Some(ev.revoked_at as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms).naive_utc();
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
        created_at: now,
        updated_at: now,
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
        website: None,
        birthdate: None,
        location: None,
        x_username: None,
        username: None,
        selected_badge_id,
        selected_ecosystem_badge_id,
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
        website: None,
        birthdate: None,
        location: None,
        x_username: None,
        username: None,
        selected_badge_id: None,
        selected_ecosystem_badge_id: Some(None),
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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: BadgeRemovedEvent = common::deserialize_social_event_json(
        "profile",
        "BadgeRemovedEvent",
        event_id,
        data,
        "profile BadgeRemovedEvent JSON did not match BadgeRemovedEvent",
    )?;
    let ms = common::chain_timestamp_ms(Some(ev.removed_at as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms).naive_utc();
    let event = NewProfileEvent {
        event_type: "BadgeRemoved".to_string(),
        profile_id: ev.profile_id.clone(),
        event_data: serde_json::json!({
            "badge_id": ev.badge_id,
            "removed_by": ev.removed_by,
            "removed_at": ev.removed_at,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
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
struct VestingPieceEvent {
    #[serde(default, deserialize_with = "deserialize_optional_number_from_string")]
    kind: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_number_from_string")]
    time_offset: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_number_from_string")]
    duration: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_number_from_string")]
    amount_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_number_from_string")]
    curve_factor: Option<u64>,
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
        rename = "schedule_end",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    schedule_end: Option<u64>,
    #[serde(default)]
    pieces: Vec<VestingPieceEvent>,
    #[serde(
        rename = "vested_at",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    vested_at: Option<u64>,
}

fn pieces_to_json(pieces: &[VestingPieceEvent]) -> serde_json::Value {
    let json_pieces: Vec<serde_json::Value> = pieces
        .iter()
        .map(|p| {
            serde_json::json!({
                "kind": p.kind.unwrap_or(0),
                "time_offset": p.time_offset.unwrap_or(0),
                "duration": p.duration.unwrap_or(0),
                "amount_bps": p.amount_bps.unwrap_or(0),
                "curve_factor": p.curve_factor.unwrap_or(0),
            })
        })
        .collect();
    serde_json::Value::Array(json_pieces)
}

fn process_tokens_vested_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
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
    let schedule_end = ev.schedule_end.unwrap_or(0) as i64;
    let pieces_json = pieces_to_json(&ev.pieces);
    let ms = common::chain_timestamp_ms(ev.vested_at.map(|t| t as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms);
    let naive = now.naive_utc();
    let wallet = NewVestingWallet {
        wallet_id: ev.wallet_id.clone(),
        owner_address: ev.owner.clone(),
        total_amount,
        start_time,
        schedule_end,
        pieces: pieces_json.clone(),
        claimed_amount: 0,
        remaining_balance: total_amount,
        created_at: naive,
        updated_at: naive,
        transaction_id: event_id.to_string(),
    };
    let vest_event = NewVestingEvent {
        wallet_id: ev.wallet_id,
        event_type: "vested".to_string(),
        owner_address: ev.owner,
        amount: total_amount,
        remaining_balance: Some(total_amount),
        start_time: Some(start_time),
        schedule_end: Some(schedule_end),
        pieces: Some(pieces_json),
        event_time: ev.vested_at.unwrap_or(0) as i64,
        time: now,
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
    checkpoint_timestamp_ms: u64,
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
    let ms = common::chain_timestamp_ms(ev.claimed_at.map(|t| t as i64), checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(ms);
    let vest_event = NewVestingEvent {
        wallet_id: ev.wallet_id.clone(),
        event_type: "claimed".to_string(),
        owner_address: ev.owner.clone(),
        amount: claimed_amount,
        remaining_balance: Some(remaining_balance),
        start_time: None,
        schedule_end: None,
        pieces: None,
        event_time: ev.claimed_at.unwrap_or(0) as i64,
        time,
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
struct UsernameListingCreatedEvent {
    username: String,
    seller: String,
    seller_profile_id: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    min_price: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    created_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct UsernameListingCancelledEvent {
    username: String,
    #[serde(rename = "seller")]
    _seller: String,
    #[serde(rename = "seller_profile_id")]
    _seller_profile_id: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    cancelled_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct UsernameOfferCreatedEvent {
    username: String,
    seller_profile_id: String,
    buyer: String,
    buyer_profile_id: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    amount: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    created_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct UsernameOfferAcceptedEvent {
    username: String,
    replacement_username: String,
    seller: String,
    seller_profile_id: String,
    buyer: String,
    buyer_profile_id: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    amount: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    accepted_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct UsernameOfferRejectedEvent {
    username: String,
    seller_profile_id: String,
    buyer: String,
    buyer_profile_id: String,
    #[serde(
        rename = "amount",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    amount: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    rejected_at: u64,
    #[serde(default)]
    is_revoked: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct UsernameSaleFeeEvent {
    username: String,
    seller: String,
    seller_profile_id: String,
    buyer: String,
    buyer_profile_id: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    sale_amount: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
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

fn process_username_listing_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameListingCreatedEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameListingCreatedEvent",
        event_id,
        data,
        "profile UsernameListingCreatedEvent JSON did not match UsernameListingCreatedEvent",
    )?;
    let listing = NewUsernameListing {
        username: ev.username,
        seller_address: ev.seller,
        seller_profile_id: ev.seller_profile_id,
        min_price: ev.min_price as i64,
        status: "active".to_string(),
        created_at: ev.created_at as i64,
        cancelled_at: None,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::UsernameListing(listing)])
}

fn process_username_listing_cancelled_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameListingCancelledEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameListingCancelledEvent",
        event_id,
        data,
        "profile UsernameListingCancelledEvent JSON did not match UsernameListingCancelledEvent",
    )?;
    Some(vec![SocialEventRow::UsernameListingStatusUpdate {
        username: ev.username,
        status: "cancelled".to_string(),
        cancelled_at: Some(ev.cancelled_at as i64),
        transaction_id: event_id.to_string(),
    }])
}

fn process_username_offer_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameOfferCreatedEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameOfferCreatedEvent",
        event_id,
        data,
        "profile UsernameOfferCreatedEvent JSON did not match UsernameOfferCreatedEvent",
    )?;
    let created_at = ev.created_at as i64;
    let offer = NewUsernameOffer {
        username: ev.username,
        seller_profile_id: ev.seller_profile_id,
        buyer_address: ev.buyer,
        buyer_profile_id: ev.buyer_profile_id,
        amount: ev.amount as i64,
        status: "pending".to_string(),
        created_at,
        updated_at: created_at,
        resolved_at: None,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::UsernameOffer(offer)])
}

fn process_username_offer_accepted_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameOfferAcceptedEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameOfferAcceptedEvent",
        event_id,
        data,
        "profile UsernameOfferAcceptedEvent JSON did not match UsernameOfferAcceptedEvent",
    )?;
    let accepted_at = ev.accepted_at as i64;
    let seller_profile_id = ev.seller_profile_id.clone();
    let buyer_profile_id = ev.buyer_profile_id.clone();
    let seller = ev.seller.clone();
    let buyer = ev.buyer.clone();
    let listed_username = ev.username.clone();
    let replacement_username = ev.replacement_username.clone();
    // Hypertable rows are append-only; insert a resolved snapshot instead of UPDATE pending.
    let offer = NewUsernameOffer {
        username: listed_username.clone(),
        seller_profile_id: seller_profile_id.clone(),
        buyer_address: buyer.clone(),
        buyer_profile_id: buyer_profile_id.clone(),
        amount: ev.amount as i64,
        status: "accepted".to_string(),
        created_at: accepted_at,
        updated_at: accepted_at,
        resolved_at: Some(accepted_at),
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::UsernameOffer(offer),
        SocialEventRow::UsernameListingStatusUpdate {
            username: listed_username.clone(),
            status: "sold".to_string(),
            cancelled_at: None,
            transaction_id: event_id.to_string(),
        },
        SocialEventRow::ProfileUsernameSet {
            profile_id: seller_profile_id,
            username: replacement_username,
            owner_address: Some(seller),
        },
        SocialEventRow::ProfileUsernameSet {
            profile_id: buyer_profile_id,
            username: listed_username,
            owner_address: Some(buyer),
        },
    ])
}

fn process_username_offer_rejected_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameOfferRejectedEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameOfferRejectedEvent",
        event_id,
        data,
        "profile UsernameOfferRejectedEvent JSON did not match UsernameOfferRejectedEvent",
    )?;
    let rejected_at = ev.rejected_at as i64;
    let status = if ev.is_revoked {
        "revoked".to_string()
    } else {
        "rejected".to_string()
    };
    // Hypertable rows are append-only; insert a resolved snapshot from on-chain fields.
    let offer = NewUsernameOffer {
        username: ev.username,
        seller_profile_id: ev.seller_profile_id,
        buyer_address: ev.buyer,
        buyer_profile_id: ev.buyer_profile_id,
        amount: ev.amount as i64,
        status,
        created_at: rejected_at,
        updated_at: rejected_at,
        resolved_at: Some(rejected_at),
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::UsernameOffer(offer)])
}

#[derive(Debug, Clone, Deserialize)]
struct UsernameSaleSettledEvent {
    listed_username: String,
    replacement_username: String,
    seller: String,
    seller_profile_id: String,
    buyer: String,
    buyer_profile_id: String,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    amount: u64,
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    settled_at: u64,
    /// Freed when the buyer already owned a username before settlement.
    #[serde(default)]
    prior_buyer_username: Option<String>,
}

fn process_username_sale_settled_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameSaleSettledEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameSaleSettledEvent",
        event_id,
        data,
        "profile UsernameSaleSettledEvent JSON did not match UsernameSaleSettledEvent",
    )?;
    let tx_id = event_id.to_string();
    let now = chrono::Utc::now().naive_utc();
    let audit_event = NewProfileEvent {
        event_type: "UsernameSaleSettled".to_string(),
        profile_id: ev.seller_profile_id.clone(),
        event_data: serde_json::json!({
            "listed_username": ev.listed_username,
            "replacement_username": ev.replacement_username,
            "seller_profile_id": ev.seller_profile_id,
            "buyer_profile_id": ev.buyer_profile_id,
            "amount": ev.amount,
            "settled_at": ev.settled_at,
            "prior_buyer_username": ev.prior_buyer_username,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };
    let mut rows = Vec::new();
    if let Some(prior) = ev.prior_buyer_username {
        if !prior.is_empty() {
            rows.push(SocialEventRow::UsernameRegistryDelete { username: prior });
        }
    }
    rows.extend([
        SocialEventRow::UsernameRegistryReassign {
            username: ev.listed_username.clone(),
            new_profile_id: ev.buyer_profile_id.clone(),
            transaction_id: tx_id.clone(),
        },
        SocialEventRow::UsernameRegistryUpsert(NewUsernameRegistry {
            username: ev.replacement_username.clone(),
            profile_id: ev.seller_profile_id.clone(),
            transaction_id: tx_id.clone(),
        }),
        SocialEventRow::ProfileUsernameSet {
            profile_id: ev.seller_profile_id,
            username: ev.replacement_username,
            owner_address: Some(ev.seller),
        },
        SocialEventRow::ProfileUsernameSet {
            profile_id: ev.buyer_profile_id,
            username: ev.listed_username,
            owner_address: Some(ev.buyer),
        },
        SocialEventRow::ProfileEvent(audit_event),
    ]);
    Some(rows)
}

fn process_username_sale_fee_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameSaleFeeEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameSaleFeeEvent",
        event_id,
        data,
        "profile UsernameSaleFeeEvent JSON did not match UsernameSaleFeeEvent",
    )?;
    let fee = NewUsernameSaleFee {
        username: ev.username,
        seller_address: ev.seller,
        seller_profile_id: ev.seller_profile_id,
        buyer_address: ev.buyer,
        buyer_profile_id: ev.buyer_profile_id,
        sale_amount: ev.sale_amount as i64,
        fee_amount: ev.fee_amount as i64,
        fee_recipient_address: ev.fee_recipient,
        timestamp: ev.timestamp as i64,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::UsernameSaleFee(fee)])
}

fn process_vesting_wallet_deleted_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: VestingWalletDeletedEvent = common::deserialize_social_event_json(
        "profile",
        "VestingWalletDeletedEvent",
        event_id,
        data,
        "profile VestingWalletDeletedEvent JSON did not match VestingWalletDeletedEvent",
    )?;
    let ms = common::chain_timestamp_ms(ev.deleted_at.map(|t| t as i64), checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(ms);
    let vest_event = NewVestingEvent {
        wallet_id: ev.wallet_id.clone(),
        event_type: "deleted".to_string(),
        owner_address: ev._owner.clone(),
        amount: 0,
        remaining_balance: None,
        start_time: None,
        schedule_end: None,
        pieces: None,
        event_time: ev.deleted_at.unwrap_or(0) as i64,
        time,
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::VestingEvent(vest_event),
        SocialEventRow::VestingWalletDelete {
            wallet_id: ev.wallet_id,
        },
    ])
}

/// Emitted when a username is reserved (PoC beneficiary or marketplace listing escrow).
#[derive(Debug, Clone, Deserialize)]
struct UsernameReservedEvent {
    username: String,
    reason: u8,
    #[serde(rename = "reserved_by", default)]
    reserved_by: String,
}

fn process_username_reserved_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameReservedEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameReservedEvent",
        event_id,
        data,
        "profile UsernameReservedEvent JSON did not match UsernameReservedEvent",
    )?;
    let reserved_at = checkpoint_timestamp_ms as i64;
    let reserved_by = common::normalize_hex_address(&ev.reserved_by);
    let now = common::chain_time_from_ms(reserved_at).naive_utc();
    let audit_event = NewProfileEvent {
        event_type: "UsernameReserved".to_string(),
        profile_id: reserved_by.clone(),
        event_data: serde_json::json!({
            "username": ev.username,
            "reason": ev.reason,
            "reserved_by": reserved_by,
            "reserved_at": reserved_at,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };
    let reservation = NewUsernameReservation {
        username: ev.username,
        reason: ev.reason as i16,
        reserved_by,
        reserved_at,
        released_by: None,
        released_at: None,
        status: USERNAME_RESERVATION_STATUS_ACTIVE.to_string(),
        reserve_transaction_id: event_id.to_string(),
        release_transaction_id: None,
        time: common::chain_time_from_ms(reserved_at),
    };
    Some(vec![
        SocialEventRow::UsernameReservation(reservation),
        SocialEventRow::ProfileEvent(audit_event),
    ])
}

/// Emitted when a username reservation is released.
#[derive(Debug, Clone, Deserialize)]
struct UsernameReleasedEvent {
    username: String,
    reason: u8,
    #[serde(rename = "released_by", default)]
    released_by: String,
}

fn process_username_released_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: UsernameReleasedEvent = common::deserialize_social_event_json(
        "profile",
        "UsernameReleasedEvent",
        event_id,
        data,
        "profile UsernameReleasedEvent JSON did not match UsernameReleasedEvent",
    )?;
    let released_at = checkpoint_timestamp_ms as i64;
    let released_by = common::normalize_hex_address(&ev.released_by);
    let now = common::chain_time_from_ms(released_at).naive_utc();
    let audit_event = NewProfileEvent {
        event_type: "UsernameReleased".to_string(),
        profile_id: released_by.clone(),
        event_data: serde_json::json!({
            "username": ev.username,
            "reason": ev.reason,
            "released_by": released_by,
            "released_at": released_at,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };
    Some(vec![
        SocialEventRow::UsernameReservationRelease {
            username: ev.username,
            reason: ev.reason as i16,
            released_by,
            released_at,
            release_transaction_id: event_id.to_string(),
        },
        SocialEventRow::ProfileEvent(audit_event),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::events::parse_event_contents;

    const CK_MS: u64 = 1_700_000_000_000;

    #[test]
    fn test_profile_created_bcs_to_handler_to_new_profile() {
        let data = serde_json::json!({
            "profile_id": "0xd988a8c1f1262d0aa7ab581a78b957fa97cbf53db4d27af2ee7006247a",
            "owner_address": "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
            "display_name": "Brandon Shaw",
            "bio": "Web8 developer and crypto enthusiast",
            "profile_picture": "https://example.com/profile.jpg",
            "cover_photo": "https://example.com/cover.png",
            "created_at": 5,
        });
        let rows = handle_profile_event("ProfileCreatedEvent", &data, "test-event-id", CK_MS)
            .expect("handle_profile_event should return Some");
        assert_eq!(rows.len(), 2, "expect Profile + ProfileEvent");
        let (profile_row, event_row) = match (&rows[0], &rows[1]) {
            (SocialEventRow::Profile(p), SocialEventRow::ProfileEvent(e)) => (p, e),
            (SocialEventRow::ProfileEvent(e), SocialEventRow::Profile(p)) => (p, e),
            _ => panic!("expected Profile and ProfileEvent rows"),
        };
        assert_eq!(profile_row.username, "");
        assert_eq!(profile_row.display_name, Some("Brandon Shaw".to_string()));
        assert_eq!(
            profile_row.bio,
            Some("Web8 developer and crypto enthusiast".to_string())
        );
        assert!(profile_row.owner_address.starts_with("0x"));
        assert_eq!(event_row.event_type, "ProfileCreated");
        let expected = common::chain_time_from_ms(5).naive_utc();
        assert_eq!(profile_row.created_at, expected);
        assert_eq!(event_row.created_at, expected);
    }

    #[test]
    fn test_username_claimed_json_dual_write() {
        let data = serde_json::json!({
            "username": "brandon",
            "profile_id": "0xprofile",
        });
        let rows = handle_profile_event("UsernameClaimedEvent", &data, "tx:1", CK_MS)
            .expect("handle_profile_event should return Some");
        assert_eq!(rows.len(), 3);
        assert!(rows
            .iter()
            .any(|r| matches!(r, SocialEventRow::UsernameRegistryUpsert(_))));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileUsernameSet {
                username,
                ..
            } if username == "brandon"
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileEvent(e) if e.event_type == "UsernameClaimed"
                && e.event_data.get("username").and_then(|v| v.as_str()) == Some("brandon")
        )));
    }

    #[test]
    fn test_username_claimed_before_profile_created_event_order() {
        let username_data = serde_json::json!({
            "username": "pocub1782751083",
            "profile_id": "0x79e88bde",
        });
        let profile_data = serde_json::json!({
            "profile_id": "0x79e88bde",
            "owner_address": "0x24589501",
            "display_name": "Creator",
            "bio": "bio",
            "created_at": 1000,
        });
        let username_rows =
            handle_profile_event("UsernameClaimedEvent", &username_data, "tx:0", CK_MS)
                .expect("UsernameClaimedEvent");
        let profile_rows =
            handle_profile_event("ProfileCreatedEvent", &profile_data, "tx:1", CK_MS)
                .expect("ProfileCreatedEvent");
        assert_eq!(username_rows.len(), 3);
        assert_eq!(profile_rows.len(), 2);
        assert!(username_rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileUsernameSet { username, .. } if username == "pocub1782751083"
        )));
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
        let rows = handle_profile_event("ProfileXUsernameUpdatedEvent", &data, "e-x-1", CK_MS)
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
        let expected = common::chain_time_from_ms(1700000000000).naive_utc();
        assert_eq!(audit.created_at, expected);
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
        let rows = handle_profile_event("ProfileXUsernameUpdatedEvent", &data, "e-x-2", CK_MS)
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
        let rows = handle_profile_event("ProfileXUsernameUpdatedEvent", &json, "e-bcs", CK_MS)
            .expect("handler");
        let up_row = rows.iter().find_map(|r| match r {
            SocialEventRow::ProfileXUsernameUpdate { x_username, .. } => x_username.clone(),
            _ => None,
        });
        assert_eq!(up_row.as_deref(), Some("from_bcs"));
    }

    #[test]
    fn test_enrich_new_profile_bootstrap_sets_linked_ids() {
        let mut profile = ProfileCreatedEvent {
            profile_id: "0xprofile".to_string(),
            owner_address: "0xowner".to_string(),
            display_name: "Test".to_string(),
            bio: String::new(),
            profile_photo: None,
            cover_photo: None,
            created_at: 1000,
        }
        .into_model(CK_MS);
        assert!(profile.memory_account_id.is_none());
        assert!(profile.ai_credit_balance_id.is_none());
        enrich_new_profile_bootstrap(
            &mut profile,
            Some("0xmemory".to_string()),
            Some("0xbalance".to_string()),
        );
        assert_eq!(profile.memory_account_id.as_deref(), Some("0xmemory"));
        assert_eq!(profile.ai_credit_balance_id.as_deref(), Some("0xbalance"));
    }

    #[test]
    fn test_create_profile_bootstrap_event_order_attaches_ids() {
        use super::super::ai_credit;
        use super::super::memory;
        use std::collections::HashMap;

        let profile_id = "0x0ea44175";
        let memory_data = serde_json::json!({
            "account_id": "0x45351697",
            "owner": "0x24589501",
            "profile_id": profile_id,
        });
        let balance_data = serde_json::json!({
            "balance_id": "0xb472ecb4",
            "memory_account_id": "0x45351697",
            "principal_owner": "0x24589501",
            "profile_id": profile_id,
        });
        let profile_data = serde_json::json!({
            "profile_id": profile_id,
            "owner_address": "0x24589501",
            "display_name": "Creator",
            "bio": "bio",
            "created_at": 1000,
        });

        let mut memory_by_profile: HashMap<String, String> = HashMap::new();
        let mut balance_by_profile: HashMap<String, String> = HashMap::new();

        if let Some(rows) =
            memory::handle_memory_event("MemoryAccountCreated", &memory_data, "tx:0")
        {
            for row in rows {
                if let SocialEventRow::MemoryAccount(a) = row {
                    memory_by_profile.insert(a.profile_id.clone(), a.account_id.clone());
                }
            }
        }
        if let Some(rows) =
            ai_credit::handle_ai_credit_event("AiCreditBalanceCreated", &balance_data, "tx:1")
        {
            for row in rows {
                if let SocialEventRow::AiCreditBalanceUpsert(b) = row {
                    balance_by_profile.insert(b.profile_id.clone(), b.balance_id.clone());
                }
            }
        }

        let profile_rows =
            handle_profile_event("ProfileCreatedEvent", &profile_data, "tx:2", CK_MS)
                .expect("ProfileCreatedEvent");
        let mut profile = profile_rows
            .iter()
            .find_map(|r| match r {
                SocialEventRow::Profile(p) => Some(p.clone()),
                _ => None,
            })
            .expect("profile row");
        enrich_new_profile_bootstrap(
            &mut profile,
            memory_by_profile.get(profile_id).cloned(),
            balance_by_profile.get(profile_id).cloned(),
        );
        assert_eq!(profile.memory_account_id.as_deref(), Some("0x45351697"));
        assert_eq!(profile.ai_credit_balance_id.as_deref(), Some("0xb472ecb4"));
    }

    #[test]
    fn test_username_offer_accepted_inserts_resolved_offer_row() {
        let seller_profile_id = move_core_types::account_address::AccountAddress::random();
        let buyer = move_core_types::account_address::AccountAddress::random();
        let buyer_profile_id = move_core_types::account_address::AccountAddress::random();
        let ev = crate::handlers::events::BcsUsernameOfferAcceptedEvent {
            username: "premium1".to_string(),
            replacement_username: "seller1".to_string(),
            seller: seller_profile_id,
            seller_profile_id,
            buyer,
            buyer_profile_id,
            amount: 5_000_000_000,
            accepted_at: 1_783_236_200_000,
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs serialize");
        let json = parse_event_contents("profile", "UsernameOfferAcceptedEvent", &bytes)
            .expect("parse_event_contents");
        let rows = handle_profile_event("UsernameOfferAcceptedEvent", &json, "tx:accept:0", CK_MS)
            .expect("handler");
        let offer = rows.iter().find_map(|r| match r {
            SocialEventRow::UsernameOffer(o) => Some(o.clone()),
            _ => None,
        });
        assert!(offer.is_some(), "expected UsernameOffer insert");
        let offer = offer.unwrap();
        assert_eq!(offer.status, "accepted");
        assert_eq!(offer.amount, 5_000_000_000);
        assert_eq!(offer.resolved_at, Some(1_783_236_200_000));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::UsernameListingStatusUpdate { status, .. } if status == "sold"
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileUsernameSet { profile_id, username, .. }
                if *profile_id == seller_profile_id.to_canonical_string(true) && username == "seller1"
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileUsernameSet { profile_id, username, .. }
                if *profile_id == buyer_profile_id.to_canonical_string(true) && username == "premium1"
        )));
    }

    #[test]
    fn test_username_sale_settled_updates_registry_and_profile_usernames() {
        let seller_profile_id = move_core_types::account_address::AccountAddress::random();
        let buyer = move_core_types::account_address::AccountAddress::random();
        let buyer_profile_id = move_core_types::account_address::AccountAddress::random();
        let ev = crate::handlers::events::BcsUsernameSaleSettledEvent {
            listed_username: "premium1".to_string(),
            replacement_username: "seller1".to_string(),
            seller: seller_profile_id,
            seller_profile_id,
            buyer,
            buyer_profile_id,
            amount: 5_000_000_000,
            settled_at: 1_783_237_181_000,
            prior_buyer_username: Some("buyer1".to_string()),
        };
        let bytes = bcs::to_bytes(&ev).expect("bcs serialize");
        let json = parse_event_contents("profile", "UsernameSaleSettledEvent", &bytes)
            .expect("parse_event_contents");
        let rows = handle_profile_event("UsernameSaleSettledEvent", &json, "tx:settle:0", CK_MS)
            .expect("handler");
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::UsernameRegistryDelete { username } if username == "buyer1"
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::UsernameRegistryReassign { username, new_profile_id, .. }
                if username == "premium1"
                    && *new_profile_id == buyer_profile_id.to_canonical_string(true)
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::UsernameRegistryUpsert(reg)
                if reg.username == "seller1"
                    && reg.profile_id == seller_profile_id.to_canonical_string(true)
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileUsernameSet { profile_id, username, .. }
                if *profile_id == seller_profile_id.to_canonical_string(true) && username == "seller1"
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileUsernameSet { profile_id, username, .. }
                if *profile_id == buyer_profile_id.to_canonical_string(true) && username == "premium1"
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileEvent(e)
                if e.event_type == "UsernameSaleSettled"
                    && e.event_data.get("listed_username").and_then(|v| v.as_str()) == Some("premium1")
                    && e.event_data.get("replacement_username").and_then(|v| v.as_str()) == Some("seller1")
                    && e.event_data.get("prior_buyer_username").and_then(|v| v.as_str()) == Some("buyer1")
        )));
    }

    /// Single-profile rename: delete prior registry row, upsert new name, set profiles.username.
    #[test]
    fn test_username_reassigned_sets_target_and_deletes_prior() {
        let profile_id = move_core_types::account_address::AccountAddress::random();
        let profile_id_string = profile_id.to_canonical_string(true);
        let json = serde_json::json!({
            "username": "brandnew",
            "profile_id": profile_id_string,
            "admin": "0x1",
            "reason_code": 2,
            "prior_username": "oldtarget",
        });
        let rows = handle_profile_event("UsernameReassignedEvent", &json, "tx:reassign:0", CK_MS)
            .expect("handler");
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::UsernameRegistryDelete { username } if username == "oldtarget"
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::UsernameRegistryUpsert(reg)
                if reg.username == "brandnew"
                    && reg.profile_id == profile_id_string
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileUsernameSet { profile_id: pid, username, .. }
                if *pid == profile_id_string && username == "brandnew"
        )));
        assert!(!rows.iter().any(|r| matches!(
            r,
            SocialEventRow::UsernameRegistryReassign { .. }
        )));
        let audit_event = rows
            .iter()
            .find_map(|row| match row {
                SocialEventRow::ProfileEvent(event) => Some(event),
                _ => None,
            })
            .expect("UsernameReassigned profile event");
        let expected_time = common::chain_time_from_ms(CK_MS as i64).naive_utc();
        assert_eq!(audit_event.event_type, "UsernameReassigned");
        assert_eq!(audit_event.profile_id, profile_id_string);
        assert_eq!(
            audit_event.event_data,
            serde_json::json!({
                "username": "brandnew",
                "profile_id": profile_id_string,
                "admin": "0x1",
                "reason_code": 2,
                "prior_username": "oldtarget",
            })
        );
        assert_eq!(audit_event.event_id.as_deref(), Some("tx:reassign:0"));
        assert_eq!(audit_event.created_at, expected_time);
        assert_eq!(audit_event.updated_at, expected_time);
    }

    #[test]
    fn test_username_reserved_event_handler() {
        let reserved_by = move_core_types::account_address::AccountAddress::random();
        let json = serde_json::json!({
            "username": "locked_name",
            "reason": 1,
            "reserved_by": reserved_by.to_canonical_string(true),
        });
        let rows =
            handle_profile_event("UsernameReservedEvent", &json, "tx:reserve:0", CK_MS).expect("handler");
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::UsernameReservation(reservation)
                if reservation.username == "locked_name"
                    && reservation.reason == 1
                    && reservation.status == USERNAME_RESERVATION_STATUS_ACTIVE
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileEvent(e)
                if e.event_type == "UsernameReserved"
                    && e.event_data.get("username").and_then(|v| v.as_str()) == Some("locked_name")
        )));
    }

    #[test]
    fn test_username_released_event_handler() {
        let released_by = move_core_types::account_address::AccountAddress::random();
        let json = serde_json::json!({
            "username": "locked_name",
            "reason": 1,
            "released_by": released_by.to_canonical_string(true),
        });
        let rows =
            handle_profile_event("UsernameReleasedEvent", &json, "tx:release:0", CK_MS).expect("handler");
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::UsernameReservationRelease {
                username,
                reason,
                ..
            } if username == "locked_name" && *reason == 1
        )));
        assert!(rows.iter().any(|r| matches!(
            r,
            SocialEventRow::ProfileEvent(e)
                if e.event_type == "UsernameReleased"
                    && e.event_data.get("username").and_then(|v| v.as_str()) == Some("locked_name")
        )));
    }

    #[test]
    fn test_username_reserved_bcs_parse() {
        let reserved_by = move_core_types::account_address::AccountAddress::random();
        let ev = crate::handlers::events::BcsUsernameReservedEvent {
            username: "locked_name".to_string(),
            reason: 2,
            reserved_by,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize");
        let json = parse_event_contents("profile", "UsernameReservedEvent", &bytes).expect("parse");
        assert_eq!(json.get("username").and_then(|v| v.as_str()), Some("locked_name"));
        assert_eq!(json.get("reason").and_then(|v| v.as_u64()), Some(2));
        assert_eq!(
            json.get("reserved_by").and_then(|v| v.as_str()),
            Some(reserved_by.to_canonical_string(true).as_str())
        );
    }

    #[test]
    fn test_username_released_bcs_parse() {
        let released_by = move_core_types::account_address::AccountAddress::random();
        let ev = crate::handlers::events::BcsUsernameReleasedEvent {
            username: "locked_name".to_string(),
            reason: 3,
            released_by,
        };
        let bytes = bcs::to_bytes(&ev).expect("serialize");
        let json = parse_event_contents("profile", "UsernameReleasedEvent", &bytes).expect("parse");
        assert_eq!(json.get("username").and_then(|v| v.as_str()), Some("locked_name"));
        assert_eq!(json.get("reason").and_then(|v| v.as_u64()), Some(3));
        assert_eq!(
            json.get("released_by").and_then(|v| v.as_str()),
            Some(released_by.to_canonical_string(true).as_str())
        );
    }
}
