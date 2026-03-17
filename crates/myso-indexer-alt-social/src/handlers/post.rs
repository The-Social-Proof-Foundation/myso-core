// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;
use serde::Deserialize;

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewComment, NewDeletionEvent, NewModerationEvent, NewPost, NewPostTransfer, NewReaction,
    NewReactionCount, NewReport, NewRepost, NewTip, NewUnifiedRevenue,
};
use myso_indexer_alt_social_schema::models::{
    CONTENT_TYPE_COMMENT, CONTENT_TYPE_POST, REVENUE_TYPE_TIPS_COMMENT, REVENUE_TYPE_TIPS_POST,
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
struct ContentUpdateEvent {
    object_id: String,
    #[serde(default)]
    is_post: bool,
    content: String,
    media_urls: Option<serde_json::Value>,
    mentions: Option<serde_json::Value>,
    metadata_json: Option<String>,
    #[serde(default, deserialize_with = "de_u64")]
    updated_at: u64,
}

#[derive(Debug, Deserialize)]
struct PostParametersUpdatedEvent {
    updated_by: String,
    #[serde(default, deserialize_with = "de_u64")]
    timestamp: u64,
    #[serde(default, deserialize_with = "de_u64")]
    max_content_length: u64,
    #[serde(default, deserialize_with = "de_u64")]
    max_media_urls: u64,
    #[serde(default, deserialize_with = "de_u64")]
    max_mentions: u64,
    #[serde(default, deserialize_with = "de_u64")]
    max_metadata_size: u64,
    #[serde(default, deserialize_with = "de_u64")]
    max_description_length: u64,
    #[serde(default, deserialize_with = "de_u64")]
    max_reaction_length: u64,
    #[serde(default, deserialize_with = "de_u64")]
    commenter_tip_percentage: u64,
    #[serde(default, deserialize_with = "de_u64")]
    repost_tip_percentage: u64,
    #[serde(default, deserialize_with = "de_opt_u64")]
    version: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct OwnershipTransferEvent {
    object_id: String,
    previous_owner: String,
    new_owner: String,
    #[serde(default)]
    is_post: bool,
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

#[derive(Debug, Deserialize)]
struct PromotedPostCreatedEvent {
    post_id: String,
    owner: String,
    profile_id: String,
    #[serde(deserialize_with = "de_u64")]
    payment_per_view: u64,
    #[serde(deserialize_with = "de_u64")]
    total_budget: u64,
    #[serde(deserialize_with = "de_u64")]
    created_at: u64,
}

#[derive(Debug, Deserialize)]
struct PromotedPostViewConfirmedEvent {
    promotion_id: String,
    viewer: String,
    #[serde(deserialize_with = "de_u64")]
    payment_amount: u64,
    #[serde(deserialize_with = "de_u64")]
    view_duration: u64,
    platform_id: String,
    #[serde(deserialize_with = "de_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct PromotionStatusToggledEvent {
    promotion_id: String,
    toggled_by: String,
    new_status: bool,
    #[serde(deserialize_with = "de_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct PromotionFundsWithdrawnEvent {
    promotion_id: String,
    owner: String,
    #[serde(deserialize_with = "de_u64")]
    withdrawn_amount: u64,
    #[serde(deserialize_with = "de_u64")]
    timestamp: u64,
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
        "ContentUpdateEvent" | "PostUpdatedEvent" | "CommentUpdatedEvent" => {
            process_content_update_event(data)
        }
        "OwnershipTransferEvent" => process_ownership_transfer_event(data, event_id),
        "PostParametersUpdatedEvent" => process_post_parameters_updated_event(data, event_id),
        "PromotedPostCreatedEvent" => process_promoted_post_created_event(data, event_id),
        "PromotedPostViewConfirmedEvent" => {
            process_promoted_post_view_confirmed_event(data, event_id)
        }
        "PromotionStatusToggledEvent" => process_promotion_status_toggled_event(data, event_id),
        "PromotionFundsWithdrawnEvent" => process_promotion_funds_withdrawn_event(data, event_id),
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
        owner: ev.owner.clone(),
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
    Some(vec![
        SocialEventRow::Post(post),
        SocialEventRow::ProfilePostCountIncrement {
            owner_address: ev.owner.clone(),
        },
    ])
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
        original_id: ev.original_id.clone(),
        original_post_id: ev.original_post_id,
        is_original_post: ev.is_original_post,
        owner: ev.owner,
        profile_id: ev.profile_id,
        created_at,
        time: now,
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::Repost(repost),
        SocialEventRow::PostRepostCountIncrement {
            original_id: ev.original_id.clone(),
            is_original_post: ev.is_original_post,
        },
    ])
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
        tipper: ev.from.clone(),
        recipient: ev.to.clone(),
        object_id: ev.object_id.clone(),
        amount: ev.amount as i64,
        is_post: ev.is_post,
        created_at,
        time: now,
        transaction_id: event_id.to_string(),
    };
    let revenue_type = if ev.is_post {
        REVENUE_TYPE_TIPS_POST.to_string()
    } else {
        REVENUE_TYPE_TIPS_COMMENT.to_string()
    };
    let content_type = if ev.is_post {
        CONTENT_TYPE_POST.to_string()
    } else {
        CONTENT_TYPE_COMMENT.to_string()
    };
    let unified_revenue = NewUnifiedRevenue::from_tip(
        revenue_type,
        ev.to.clone(),
        ev.amount as i64,
        ev.object_id.clone(),
        content_type,
        ev.from.clone(),
        created_at,
        event_id.to_string(),
    );
    Some(vec![
        SocialEventRow::Tip(tip),
        SocialEventRow::PostTipsReceivedIncrement {
            object_id: ev.object_id,
            amount: ev.amount as i64,
            is_post: ev.is_post,
        },
        SocialEventRow::UnifiedRevenue(unified_revenue),
    ])
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
        object_id: ev.object_id.clone(),
        platform_id: ev.platform_id,
        removed: ev.removed,
        moderated_by: ev.moderated_by.clone(),
        moderated_at,
        time: now,
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::ModerationEvent(mod_ev),
        SocialEventRow::PostModerationUpdate {
            object_id: ev.object_id,
            removed: ev.removed,
            moderated_by: ev.moderated_by,
        },
    ])
}

fn process_content_update_event(data: &serde_json::Value) -> Option<Vec<SocialEventRow>> {
    let ev: ContentUpdateEvent = serde_json::from_value(data.clone()).ok()?;
    let media_urls = ev.media_urls.clone();
    let mentions = ev.mentions.clone();
    let metadata_json = ev
        .metadata_json
        .as_ref()
        .and_then(|s| serde_json::from_str(s).ok());
    Some(vec![SocialEventRow::PostContentUpdate {
        object_id: ev.object_id,
        content: ev.content,
        media_urls,
        mentions,
        metadata_json,
        is_post: ev.is_post,
        updated_at: ev.updated_at as i64,
    }])
}

fn process_post_parameters_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PostParametersUpdatedEvent = serde_json::from_value(data.clone()).ok()?;
    Some(vec![SocialEventRow::PostConfig {
        updated_by: ev.updated_by,
        max_content_length: ev.max_content_length as i64,
        max_media_urls: ev.max_media_urls as i64,
        max_mentions: ev.max_mentions as i64,
        max_metadata_size: ev.max_metadata_size as i64,
        max_description_length: ev.max_description_length as i64,
        max_reaction_length: ev.max_reaction_length as i64,
        commenter_tip_percentage: ev.commenter_tip_percentage as i64,
        repost_tip_percentage: ev.repost_tip_percentage as i64,
        version: ev.version.map(|v| v as i64),
        updated_at: ev.timestamp as i64,
        transaction_id: event_id.to_string(),
    }])
}

fn process_ownership_transfer_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: OwnershipTransferEvent = serde_json::from_value(data.clone()).ok()?;
    let now = Utc::now();
    let transferred_at = now.timestamp();
    let transfer = NewPostTransfer {
        object_id: ev.object_id.clone(),
        previous_owner: ev.previous_owner.clone(),
        new_owner: ev.new_owner.clone(),
        is_post: ev.is_post,
        transferred_at,
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::PostOwnerUpdate {
            object_id: ev.object_id,
            new_owner: ev.new_owner,
            is_post: ev.is_post,
        },
        SocialEventRow::PostTransfer(transfer),
    ])
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
    let deleted_at = ev.deleted_at as i64;

    let del_ev = NewDeletionEvent {
        object_id: ev.object_id.clone(),
        owner: ev.owner.clone(),
        profile_id: ev.profile_id,
        is_post: ev.is_post,
        post_type: ev.post_type,
        post_id: ev.post_id.clone(),
        deleted_at,
        time: now,
        transaction_id: event_id.to_string(),
    };

    let mut rows = vec![SocialEventRow::DeletionEvent(del_ev)];
    if ev.is_post {
        rows.push(SocialEventRow::ProfilePostCountDecrement {
            owner_address: ev.owner.clone(),
        });
        rows.push(SocialEventRow::PostDeletedAtUpdate {
            object_id: ev.object_id.clone(),
            owner: ev.owner,
            deleted_at,
        });
    } else {
        if let Some(ref post_id) = ev.post_id {
            rows.push(SocialEventRow::PostCommentCountIncrement {
                post_id: post_id.clone(),
                delta: -1,
            });
        } else {
            rows.push(SocialEventRow::PostCommentCountDecrementByComment {
                comment_id: ev.object_id.clone(),
                owner: ev.owner.clone(),
            });
        }
        rows.push(SocialEventRow::CommentDeletedAtUpdate {
            object_id: ev.object_id,
            owner: ev.owner,
            deleted_at,
        });
    }
    Some(rows)
}

fn process_promoted_post_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PromotedPostCreatedEvent = serde_json::from_value(data.clone()).ok()?;
    Some(vec![SocialEventRow::PromotedPost {
        post_id: ev.post_id,
        owner: ev.owner,
        profile_id: ev.profile_id,
        payment_per_view: ev.payment_per_view as i64,
        total_budget: ev.total_budget as i64,
        created_at: ev.created_at as i64,
        transaction_id: event_id.to_string(),
    }])
}

fn process_promoted_post_view_confirmed_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PromotedPostViewConfirmedEvent = serde_json::from_value(data.clone()).ok()?;
    Some(vec![SocialEventRow::PromotionView {
        promotion_id: ev.promotion_id,
        viewer: ev.viewer,
        payment_amount: ev.payment_amount as i64,
        view_duration: ev.view_duration as i64,
        platform_id: ev.platform_id,
        timestamp: ev.timestamp as i64,
        transaction_id: event_id.to_string(),
    }])
}

fn process_promotion_status_toggled_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PromotionStatusToggledEvent = serde_json::from_value(data.clone()).ok()?;
    Some(vec![SocialEventRow::PromotionStatusEvent {
        promotion_id: ev.promotion_id,
        toggled_by: ev.toggled_by,
        new_status: ev.new_status,
        timestamp: ev.timestamp as i64,
        transaction_id: event_id.to_string(),
    }])
}

fn process_promotion_funds_withdrawn_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PromotionFundsWithdrawnEvent = serde_json::from_value(data.clone()).ok()?;
    Some(vec![SocialEventRow::PromotionBudgetEvent {
        promotion_id: ev.promotion_id,
        owner: ev.owner,
        withdrawn_amount: ev.withdrawn_amount as i64,
        timestamp: ev.timestamp as i64,
        transaction_id: event_id.to_string(),
    }])
}
