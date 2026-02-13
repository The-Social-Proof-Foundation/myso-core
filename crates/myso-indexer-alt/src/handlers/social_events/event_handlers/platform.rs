// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;

pub fn handle_platform_event(
    event_name: &str,
    _data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "PlatformCreatedEvent" => process_platform_created_event(),
        "PlatformUpdatedEvent" => process_platform_updated_event(),
        "PlatformApprovalChangedEvent" | "ApprovalChangedEvent" => {
            process_platform_approval_changed_event()
        }
        "ModeratorAddedEvent" => process_moderator_added_event(),
        "ModeratorRemovedEvent" => process_moderator_removed_event(),
        "PlatformBlockedProfileEvent" => process_platform_blocked_profile_event(),
        "PlatformUnblockedProfileEvent" => process_platform_unblocked_profile_event(),
        "UserJoinedPlatformEvent" => process_user_joined_platform_event(),
        "UserLeftPlatformEvent" => process_user_left_platform_event(),
        "TokenAirdropEvent" => process_token_airdrop_event(),
        "PlatformDeletedEvent" => process_platform_deleted_event(),
        _ => None,
    }
}

fn process_platform_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_platform_updated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_platform_approval_changed_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_moderator_added_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_moderator_removed_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_platform_blocked_profile_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_platform_unblocked_profile_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_user_joined_platform_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_user_left_platform_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_token_airdrop_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_platform_deleted_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
