// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;

pub fn handle_post_event(
    event_name: &str,
    _data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "PostCreatedEvent" => process_post_created_event(),
        "CommentCreatedEvent" => process_comment_created_event(),
        "ReactionEvent" | "ReactionAddedEvent" => process_reaction_event(),
        "ReactionRemovedEvent" | "RemoveReactionEvent" => process_remove_reaction_event(),
        "RepostEvent" | "RepostCreatedEvent" => process_repost_event(),
        "TipEvent" | "TipSentEvent" => process_tip_event(),
        "ModerationEvent" | "ContentModerationEvent" | "PostModerationEvent" => {
            process_moderation_event()
        }
        "ReportEvent" | "ContentReportEvent" | "PostReportedEvent" | "CommentReportedEvent" => {
            process_report_event()
        }
        "DeletionEvent" | "ContentDeletedEvent" | "PostDeletedEvent" | "CommentDeletedEvent" => {
            process_deletion_event()
        }
        "ContentUpdateEvent" | "PostUpdatedEvent" | "CommentUpdatedEvent" => {
            process_content_update_event()
        }
        "OwnershipTransferEvent" => process_ownership_transfer_event(),
        "PostParametersUpdatedEvent" => process_post_parameters_updated_event(),
        "PromotedPostCreatedEvent" => process_promoted_post_created_event(),
        "PromotedPostViewConfirmedEvent" => process_promoted_post_view_confirmed_event(),
        "PromotionStatusToggledEvent" => process_promotion_status_toggled_event(),
        "PromotionFundsWithdrawnEvent" => process_promotion_funds_withdrawn_event(),
        _ => None,
    }
}

fn process_post_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_comment_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_reaction_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_remove_reaction_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_repost_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_tip_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_moderation_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_report_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_deletion_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_content_update_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_ownership_transfer_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_post_parameters_updated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_promoted_post_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_promoted_post_view_confirmed_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_promotion_status_toggled_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_promotion_funds_withdrawn_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
