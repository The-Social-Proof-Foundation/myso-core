// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;
use serde::Deserialize;

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewComment, NewDeletionEvent, NewModerationEvent, NewPost, NewReaction, NewReactionCount,
    NewReport, NewRepost, NewTip,
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

#[derive(Debug, Deserialize)]
struct PostCreatedEvent {
    post_id: String,
    owner: String,
    profile_id: String,
    content: String,
    post_type: String,
    parent_post_id: Option<String>,
    mentions: Option<serde_json::Value>,
    media_urls: Option<serde_json::Value>,
    metadata_json: Option<String>,
    mydata_id: Option<String>,
    promotion_id: Option<String>,
    revenue_redirect_to: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u64")]
    revenue_redirect_percentage: Option<u64>,
    #[serde(default)]
    enable_spt: bool,
    #[serde(default)]
    enable_poc: bool,
    #[serde(default)]
    enable_spot: bool,
    spot_id: Option<String>,
    spt_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CommentCreatedEvent {
    comment_id: String,
    post_id: String,
    parent_comment_id: Option<String>,
    owner: String,
    profile_id: String,
    content: String,
    mentions: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct ReactionEvent {
    object_id: String,
    user_address: String,
    reaction_text: String,
    is_post: bool,
    #[serde(default, deserialize_with = "de_u64")]
    created_at: u64,
}

#[derive(Debug, Deserialize)]
struct RemoveReactionEvent {
    object_id: String,
    user_address: String,
    reaction_text: String,
    is_post: bool,
}

#[derive(Debug, Deserialize)]
struct RepostEvent {
    repost_id: String,
    original_id: String,
    original_post_id: String,
    is_original_post: bool,
    owner: String,
    profile_id: String,
    #[serde(default, deserialize_with = "de_u64")]
    created_at: u64,
}

#[derive(Debug, Deserialize)]
struct TipEvent {
    object_id: String,
    from: String,
    to: String,
    #[serde(deserialize_with = "de_u64")]
    amount: u64,
    is_post: bool,
    #[serde(default, deserialize_with = "de_u64")]
    tip_time: u64,
}

#[derive(Debug, Deserialize)]
struct ModerationEvent {
    object_id: String,
    platform_id: String,
    removed: bool,
    moderated_by: String,
    #[serde(default, deserialize_with = "de_u64")]
    moderated_at: u64,
}

#[derive(Debug, Deserialize)]
struct ReportEvent {
    object_id: String,
    is_comment: bool,
    reporter: String,
    reason_code: u8,
    description: String,
    #[serde(deserialize_with = "de_u64")]
    reported_at: u64,
}

#[derive(Debug, Deserialize)]
struct DeletionEvent {
    object_id: String,
    owner: String,
    profile_id: String,
    is_post: bool,
    post_type: Option<String>,
    post_id: Option<String>,
    #[serde(deserialize_with = "de_u64")]
    deleted_at: u64,
}

pub fn handle_post_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "PostCreatedEvent" => process_post_created_event(data, event_id),
        "CommentCreatedEvent" => process_comment_created_event(data, event_id),
        "ReactionEvent" | "ReactionAddedEvent" => process_reaction_event(data, event_id),
        "ReactionRemovedEvent" | "RemoveReactionEvent" => process_remove_reaction_event(data),
        "RepostEvent" | "RepostCreatedEvent" => process_repost_event(data, event_id),
        "TipEvent" | "TipSentEvent" => process_tip_event(data, event_id),
        "ModerationEvent" | "ContentModerationEvent" | "PostModerationEvent" => {
            process_moderation_event(data, event_id)
        }
        "ReportEvent" | "ContentReportEvent" | "PostReportedEvent" | "CommentReportedEvent" => {
            process_report_event(data, event_id)
        }
        "DeletionEvent" | "ContentDeletedEvent" | "PostDeletedEvent" | "CommentDeletedEvent" => {
            process_deletion_event(data, event_id)
        }
        "ContentUpdateEvent" | "PostUpdatedEvent" | "CommentUpdatedEvent" => None,
        "OwnershipTransferEvent" => None,
        "PostParametersUpdatedEvent" => None,
        "PromotedPostCreatedEvent" => None,
        "PromotedPostViewConfirmedEvent" => None,
        "PromotionStatusToggledEvent" => None,
        "PromotionFundsWithdrawnEvent" => None,
        _ => None,
    }
}

fn process_post_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PostCreatedEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now();
    let created_at = now.timestamp_millis() as i64;
    let id = format!("{}:{}", ev.post_id, created_at);

    let post = NewPost {
        id: id.clone(),
        post_id: ev.post_id,
        owner: ev.owner,
        profile_id: ev.profile_id,
        content: ev.content,
        media_urls: ev.media_urls,
        mentions: ev.mentions,
        metadata_json: ev.metadata_json.and_then(|s| serde_json::from_str(&s).ok()),
        post_type: ev.post_type,
        parent_post_id: ev.parent_post_id,
        created_at,
        updated_at: None,
        deleted_at: None,
        reaction_count: 0,
        comment_count: 0,
        repost_count: 0,
        tips_received: 0,
        removed_from_platform: false,
        removed_by: None,
        transaction_id: event_id.to_string(),
        time: now,
        mydata_id: ev.mydata_id,
        revenue_recipient: None,
        poc_id: None,
        poc_reasoning: None,
        poc_evidence_urls: None,
        poc_similarity_score: None,
        poc_media_type: None,
        poc_oracle_address: None,
        poc_analyzed_at: None,
        revenue_redirect_to: ev.revenue_redirect_to,
        revenue_redirect_percentage: ev.revenue_redirect_percentage.map(|p| p as i64),
        requires_subscription: None,
        subscription_service_id: None,
        subscription_price: None,
        encrypted_content_hash: None,
        promotion_id: ev.promotion_id,
        enable_spt: ev.enable_spt,
        enable_poc: ev.enable_poc,
        enable_spot: ev.enable_spot,
        spot_id: ev.spot_id,
        spt_id: ev.spt_id,
    };
    Some(vec![SocialEventRow::Post(post)])
}

fn process_comment_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: CommentCreatedEvent = serde_json::from_value(data.clone()).ok()?;
    let post_id = ev.post_id.clone();
    let now = Utc::now();
    let created_at = now.timestamp_millis() as i64;
    let id = format!("{}:{}", ev.comment_id, created_at);

    let comment = NewComment {
        id,
        comment_id: ev.comment_id,
        post_id: ev.post_id,
        parent_comment_id: ev.parent_comment_id,
        owner: ev.owner,
        profile_id: ev.profile_id,
        content: ev.content,
        media_urls: None,
        mentions: ev.mentions,
        metadata_json: None,
        created_at,
        updated_at: None,
        deleted_at: None,
        reaction_count: 0,
        comment_count: 0,
        repost_count: 0,
        tips_received: 0,
        removed_from_platform: false,
        removed_by: None,
        transaction_id: event_id.to_string(),
        time: now,
    };
    Some(vec![
        SocialEventRow::Comment(comment),
        SocialEventRow::PostCommentCountIncrement { post_id, delta: 1 },
    ])
}

fn process_reaction_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let ev: ReactionEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now();
    let created_at = if ev.created_at > 0 {
        ev.created_at as i64
    } else {
        now.timestamp_millis() as i64
    };

    let reaction = NewReaction {
        object_id: ev.object_id.clone(),
        user_address: ev.user_address,
        reaction_text: ev.reaction_text.clone(),
        is_post: ev.is_post,
        created_at,
        time: now,
        transaction_id: event_id.to_string(),
    };
    let count = NewReactionCount {
        object_id: ev.object_id,
        reaction_text: ev.reaction_text,
        count: 1,
    };
    Some(vec![
        SocialEventRow::Reaction(reaction),
        SocialEventRow::ReactionCount(count),
    ])
}

fn process_remove_reaction_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: RemoveReactionEvent = serde_json::from_value(data.clone()).ok()?;
    Some(vec![SocialEventRow::RemoveReaction {
        object_id: ev.object_id,
        user_address: ev.user_address,
        reaction_text: ev.reaction_text,
        is_post: ev.is_post,
    }])
}

fn process_repost_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let ev: RepostEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now();
    let created_at = if ev.created_at > 0 {
        ev.created_at as i64
    } else {
        now.timestamp_millis() as i64
    };
    let id = format!("{}:{}", ev.repost_id, created_at);

    let repost = NewRepost {
        id,
        repost_id: ev.repost_id,
        original_id: ev.original_id,
        original_post_id: ev.original_post_id,
        is_original_post: ev.is_original_post,
        owner: ev.owner,
        profile_id: ev.profile_id,
        created_at,
        time: now,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::Repost(repost)])
}

fn process_tip_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let ev: TipEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now();
    let created_at = if ev.tip_time > 0 {
        ev.tip_time as i64
    } else {
        now.timestamp_millis() as i64
    };

    let tip = NewTip {
        tipper: ev.from,
        recipient: ev.to,
        object_id: ev.object_id,
        amount: ev.amount as i64,
        is_post: ev.is_post,
        created_at,
        time: now,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::Tip(tip)])
}

fn process_moderation_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ModerationEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now();
    let moderated_at = if ev.moderated_at > 0 {
        ev.moderated_at as i64
    } else {
        now.timestamp_millis() as i64
    };

    let mod_ev = NewModerationEvent {
        object_id: ev.object_id,
        platform_id: ev.platform_id,
        removed: ev.removed,
        moderated_by: ev.moderated_by,
        moderated_at,
        time: now,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::ModerationEvent(mod_ev)])
}

fn process_report_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let ev: ReportEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now();

    let report = NewReport {
        object_id: ev.object_id,
        is_comment: ev.is_comment,
        reporter: ev.reporter,
        reason_code: ev.reason_code as i16,
        description: ev.description,
        reported_at: ev.reported_at as i64,
        time: now,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::Report(report)])
}

fn process_deletion_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let ev: DeletionEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now();

    let del_ev = NewDeletionEvent {
        object_id: ev.object_id,
        owner: ev.owner,
        profile_id: ev.profile_id,
        is_post: ev.is_post,
        post_type: ev.post_type,
        post_id: ev.post_id,
        deleted_at: ev.deleted_at as i64,
        time: now,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::DeletionEvent(del_ev)])
}
