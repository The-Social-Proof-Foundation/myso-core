// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;

use super::common;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{NewSocialGraphEvent, NewSocialGraphRelationship};

pub fn handle_social_graph_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "FollowEvent" | "UserFollowedEvent" => process_follow_event(data, event_id, event_name),
        "UnfollowEvent" | "UserUnfollowedEvent" => {
            process_unfollow_event(data, event_id, event_name)
        }
        _ => None,
    }
}

fn process_follow_event(
    data: &serde_json::Value,
    event_id: &str,
    event_type: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct FollowEventJson {
        follower: String,
        following: String,
    }
    let ev: FollowEventJson = common::deserialize_social_event_json(
        "social_graph",
        event_type,
        event_id,
        data,
        "social_graph follow event JSON did not match FollowEventJson",
    )?;
    let now = Utc::now().naive_utc();
    let rel = NewSocialGraphRelationship {
        follower_address: ev.follower.clone(),
        following_address: ev.following.clone(),
        created_at: now,
    };
    let graph_ev = NewSocialGraphEvent {
        event_type: "follow".to_string(),
        follower_address: ev.follower,
        following_address: ev.following,
        created_at: rel.created_at,
        event_id: Some(event_id.to_string()),
        raw_event_data: Some(data.clone()),
    };
    Some(vec![
        SocialEventRow::SocialGraphRelationship(rel),
        SocialEventRow::SocialGraphEvent(graph_ev),
    ])
}

fn process_unfollow_event(
    data: &serde_json::Value,
    event_id: &str,
    event_type: &str,
) -> Option<Vec<SocialEventRow>> {
    #[derive(serde::Deserialize)]
    struct UnfollowEventJson {
        follower: String,
        unfollowed: String,
    }
    let ev: UnfollowEventJson = common::deserialize_social_event_json(
        "social_graph",
        event_type,
        event_id,
        data,
        "social_graph unfollow event JSON did not match UnfollowEventJson",
    )?;
    let now = Utc::now().naive_utc();
    let graph_ev = NewSocialGraphEvent {
        event_type: "unfollow".to_string(),
        follower_address: ev.follower.clone(),
        following_address: ev.unfollowed.clone(),
        created_at: now,
        event_id: Some(event_id.to_string()),
        raw_event_data: Some(data.clone()),
    };
    Some(vec![
        SocialEventRow::SocialGraphUnfollow {
            follower_address: ev.follower,
            following_address: ev.unfollowed,
        },
        SocialEventRow::SocialGraphEvent(graph_ev),
    ])
}
