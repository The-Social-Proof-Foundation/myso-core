// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{NewBlockedEvent, NewBlockedProfile, NewProfileEvent};

pub fn handle_blocking_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "UserBlockEvent" | "ProfileBlockEvent" => process_profile_block_event(data, event_id),
        "UserUnblockEvent" | "ProfileUnblockEvent" => process_profile_unblock_event(data, event_id),
        _ => None,
    }
}

fn process_profile_block_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct BlockJson {
        blocker: String,
        blocked: String,
    }
    let ev: BlockJson = serde_json::from_value(data.clone()).ok()?;
    if ev.blocker.is_empty() || ev.blocked.is_empty() {
        return None;
    }
    let now = Utc::now().naive_utc();
    let blocked_event = NewBlockedEvent {
        event_id: Some(event_id.to_string()),
        event_type: "block".to_string(),
        blocker_address: ev.blocker.clone(),
        blocked_address: Some(ev.blocked.clone()),
        raw_event_data: Some(data.clone()),
        processed_at: now,
        created_at: now,
    };
    let blocked_profile = NewBlockedProfile {
        blocker_address: ev.blocker.clone(),
        blocked_address: ev.blocked.clone(),
        blocked_profile_id: None,
        blocked_username: ev.blocked.clone(),
        blocked_display_name: None,
        blocked_profile_photo: None,
        first_blocked_at: now,
        last_blocked_at: now,
        total_block_count: 1,
    };
    let blocker = ev.blocker.clone();
    let blocked = ev.blocked.clone();
    let profile_event = NewProfileEvent {
        event_type: "BlockAdded".to_string(),
        profile_id: blocker.clone(),
        event_data: serde_json::json!({
            "blocker_profile_id": blocker.clone(),
            "blocked_profile_id": blocked.clone(),
            "timestamp": now.and_utc().timestamp() as u64,
            "is_platform_block": false,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };
    Some(vec![
        SocialEventRow::BlockedEvent(blocked_event),
        SocialEventRow::BlockedProfile(blocked_profile),
        SocialEventRow::ProfileEvent(profile_event),
        SocialEventRow::SocialGraphUnfollow {
            follower_address: blocker,
            following_address: blocked.clone(),
        },
        SocialEventRow::SocialGraphUnfollow {
            follower_address: blocked,
            following_address: ev.blocker,
        },
    ])
}

fn process_profile_unblock_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct UnblockJson {
        blocker: String,
        unblocked: String,
    }
    let ev: UnblockJson = serde_json::from_value(data.clone()).ok()?;
    if ev.blocker.is_empty() || ev.unblocked.is_empty() {
        return None;
    }
    let now = Utc::now().naive_utc();
    let blocked_event = NewBlockedEvent {
        event_id: Some(event_id.to_string()),
        event_type: "unblock".to_string(),
        blocker_address: ev.blocker.clone(),
        blocked_address: Some(ev.unblocked.clone()),
        raw_event_data: Some(data.clone()),
        processed_at: now,
        created_at: now,
    };
    let profile_event = NewProfileEvent {
        event_type: "BlockRemoved".to_string(),
        profile_id: ev.blocker.clone(),
        event_data: serde_json::json!({
            "blocker_profile_id": ev.blocker,
            "blocked_profile_id": ev.unblocked,
            "timestamp": now.and_utc().timestamp() as u64,
            "is_platform_block": false,
        }),
        event_id: Some(event_id.to_string()),
        created_at: now,
        updated_at: now,
    };
    Some(vec![
        SocialEventRow::BlockedEvent(blocked_event),
        SocialEventRow::BlockedProfileDelete {
            blocker_address: ev.blocker,
            blocked_address: ev.unblocked,
        },
        SocialEventRow::ProfileEvent(profile_event),
    ])
}
