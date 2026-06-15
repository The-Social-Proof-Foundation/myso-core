// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! `post.move` event handlers. PoC escrow and oracle flows are in [`super::poc`].
//!
//! **Indexed here:** `PostCreatedEvent` (includes `platform_id`, `permissions` when present on-chain),
//! reactions, reposts, tips, moderation, reports, promotion lifecycle.

use chrono::Utc;
use serde::Deserialize;

use super::common;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewComment, NewDeletionEvent, NewModerationEvent, NewPost, NewPostTransfer, NewReaction,
    NewReport, NewRepost, NewTip, NewUnifiedRevenue,
};
use myso_indexer_alt_social_schema::models::{
    CONTENT_TYPE_COMMENT, CONTENT_TYPE_POST, CURRENCY_MYSO, POST_TYPE_QUOTE_REPOST,
    REVENUE_TYPE_TIPS_COMMENT, REVENUE_TYPE_TIPS_POST,
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

/// `Option<u64>` where the inner value may be a JSON number or decimal string (e.g. raw JSON fallback).
fn de_opt_u64<'de, D>(d: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(d)?;
    match opt {
        None => Ok(None),
        Some(v) if v.is_null() => Ok(None),
        Some(serde_json::Value::Number(n)) => {
            let u = n
                .as_u64()
                .ok_or_else(|| serde::de::Error::custom("invalid u64"))?;
            Ok(Some(u))
        }
        Some(serde_json::Value::String(s)) => {
            let u = s.parse::<u64>().map_err(serde::de::Error::custom)?;
            Ok(Some(u))
        }
        Some(_) => Err(serde::de::Error::custom(
            "expected number, string, or null for Option<u64>",
        )),
    }
}

fn de_u8<'de, D>(d: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        U8(u8),
        U64(u64),
        S(String),
    }
    match V::deserialize(d) {
        Ok(V::U8(n)) => Ok(n),
        Ok(V::U64(n)) => u8::try_from(n).map_err(|_| serde::de::Error::custom("u8 out of range")),
        Ok(V::S(s)) => s.parse().map_err(serde::de::Error::custom),
        Err(e) => Err(e),
    }
}

fn de_opt_u8<'de, D>(d: D) -> Result<Option<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(d)?;
    match opt {
        None => Ok(None),
        Some(v) if v.is_null() => Ok(None),
        Some(serde_json::Value::Number(n)) => {
            let u = n
                .as_u64()
                .and_then(|x| u8::try_from(x).ok())
                .ok_or_else(|| serde::de::Error::custom("invalid u8"))?;
            Ok(Some(u))
        }
        Some(serde_json::Value::String(s)) => {
            s.parse::<u8>().map_err(serde::de::Error::custom).map(Some)
        }
        Some(_) => Err(serde::de::Error::custom(
            "expected number, string, or null for Option<u8>",
        )),
    }
}

fn attribution_fields(
    data: &serde_json::Value,
    default_actor: &str,
) -> (Option<String>, Option<String>, Option<i16>) {
    let actor_address = data
        .get("actor_address")
        .and_then(|v| v.as_str())
        .map(String::from)
        .or_else(|| Some(default_actor.to_string()));
    let sub_agent_id = data.get("sub_agent_id").and_then(|v| {
        if v.is_null() {
            None
        } else {
            v.as_str().map(String::from)
        }
    });
    let action_identity_class = data
        .get("action_identity_class")
        .and_then(|v| v.as_u64())
        .and_then(|n| i16::try_from(n).ok());
    (actor_address, sub_agent_id, action_identity_class)
}

#[derive(Debug, Deserialize)]
struct PostCreatedEvent {
    post_id: String,
    owner: String,
    profile_id: String,
    #[serde(default)]
    platform_id: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u8")]
    permissions: Option<u8>,
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
    #[serde(default, deserialize_with = "de_u8")]
    poc_redirection_kind: u8,
    #[serde(default)]
    actor_address: Option<String>,
    #[serde(default)]
    sub_agent_id: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u8")]
    action_identity_class: Option<u8>,
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
    #[serde(default)]
    actor_address: Option<String>,
    #[serde(default)]
    sub_agent_id: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u8")]
    action_identity_class: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct ReactionEvent {
    object_id: String,
    user_address: String,
    reaction_text: String,
    is_post: bool,
    #[serde(default, deserialize_with = "de_u64")]
    created_at: u64,
    #[serde(default)]
    principal_owner: Option<String>,
    #[serde(default)]
    actor_address: Option<String>,
    #[serde(default)]
    sub_agent_id: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u8")]
    action_identity_class: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct RemoveReactionEvent {
    object_id: String,
    user_address: String,
    reaction_text: String,
    is_post: bool,
    #[serde(default)]
    actor_address: Option<String>,
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
    #[serde(default)]
    actor_address: Option<String>,
    #[serde(default)]
    sub_agent_id: Option<String>,
    #[serde(default, deserialize_with = "de_opt_u8")]
    action_identity_class: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct TipEvent {
    object_id: String,
    from: String,
    to: String,
    #[serde(deserialize_with = "de_u64")]
    amount: u64,
    is_post: bool,
    #[serde(default)]
    coin_type: String,
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
        "ReactionRemovedEvent" | "RemoveReactionEvent" => {
            process_remove_reaction_event(data, event_id)
        }
        "RepostEvent" | "RepostCreatedEvent" => process_repost_event(data, event_id),
        "TipEvent" | "TipSentEvent" | "TipCreatedEvent" => process_tip_event(data, event_id),
        "ModerationEvent" | "ContentModerationEvent" | "PostModerationEvent" => {
            process_moderation_event(data, event_id)
        }
        "ReportEvent" | "ContentReportEvent" | "PostReportedEvent" | "CommentReportedEvent" => {
            process_report_event(data, event_id)
        }
        "DeletionEvent" | "ContentDeletedEvent" | "PostDeletedEvent" | "CommentDeletedEvent" => {
            process_deletion_event(data, event_id, event_name)
        }
        "ContentUpdateEvent" | "PostUpdatedEvent" | "CommentUpdatedEvent" => {
            process_content_update_event(data, event_id)
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
    let ev: PostCreatedEvent = common::deserialize_social_event_json(
        "post",
        "PostCreatedEvent",
        event_id,
        data,
        "post PostCreatedEvent JSON did not match PostCreatedEvent",
    )?;
    let is_quote_repost = ev.post_type == POST_TYPE_QUOTE_REPOST;
    let parent_for_repost_count = if is_quote_repost {
        ev.parent_post_id.clone()
    } else {
        None
    };
    let now = Utc::now();
    let created_at = now.timestamp_millis() as i64;
    let (actor_address, sub_agent_id, action_identity_class) =
        attribution_fields(data, &ev.owner);

    let post = NewPost {
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
        poc_outcome: None,
        poc_redirection_kind: match ev.poc_redirection_kind {
            0 => None,
            k => Some(i16::from(k)),
        },
        poc_disputes_submitted: 0,
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
        platform_id: ev.platform_id,
        permissions: ev.permissions.map(|p| i16::from(p)),
        actor_address: ev.actor_address.or(actor_address),
        sub_agent_id: ev.sub_agent_id.or(sub_agent_id),
        action_identity_class: ev
            .action_identity_class
            .map(i16::from)
            .or(action_identity_class),
    };
    let mut out = vec![
        SocialEventRow::Post(post),
        SocialEventRow::ProfilePostCountIncrement {
            owner_address: ev.owner.clone(),
        },
    ];
    if let Some(parent_id) = parent_for_repost_count {
        out.push(SocialEventRow::PostRepostCountIncrement {
            original_id: parent_id,
            is_original_post: true,
        });
    }
    Some(out)
}

fn process_comment_created_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: CommentCreatedEvent = common::deserialize_social_event_json(
        "post",
        "CommentCreatedEvent",
        event_id,
        data,
        "post CommentCreatedEvent JSON did not match CommentCreatedEvent",
    )?;
    let post_id = ev.post_id.clone();
    let now = Utc::now();
    let created_at = now.timestamp_millis() as i64;
    let id = format!("{}:{}", ev.comment_id, created_at);
    let (actor_address, sub_agent_id, action_identity_class) =
        attribution_fields(data, &ev.owner);

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
        actor_address: ev.actor_address.or(actor_address),
        sub_agent_id: ev.sub_agent_id.or(sub_agent_id),
        action_identity_class: ev
            .action_identity_class
            .map(i16::from)
            .or(action_identity_class),
    };
    Some(vec![
        SocialEventRow::Comment(comment),
        SocialEventRow::PostCommentCountIncrement { post_id, delta: 1 },
    ])
}

fn process_reaction_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let ev: ReactionEvent = common::deserialize_social_event_json(
        "post",
        "ReactionEvent",
        event_id,
        data,
        "post reaction event JSON did not match ReactionEvent",
    )?;
    let now = Utc::now();
    let created_at = if ev.created_at > 0 {
        ev.created_at as i64
    } else {
        now.timestamp_millis() as i64
    };

    let reaction = NewReaction {
        object_id: ev.object_id.clone(),
        user_address: ev
            .actor_address
            .clone()
            .unwrap_or_else(|| ev.user_address.clone()),
        reaction_text: ev.reaction_text.clone(),
        is_post: ev.is_post,
        created_at,
        time: now,
        transaction_id: event_id.to_string(),
        principal_owner: ev
            .principal_owner
            .or_else(|| Some(ev.user_address.clone())),
        actor_address: ev
            .actor_address
            .or_else(|| Some(ev.user_address.clone())),
        sub_agent_id: ev.sub_agent_id,
        action_identity_class: ev.action_identity_class.map(i16::from),
    };
    Some(vec![SocialEventRow::Reaction(reaction)])
}

fn process_remove_reaction_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: RemoveReactionEvent = common::deserialize_social_event_json(
        "post",
        "RemoveReactionEvent",
        event_id,
        data,
        "post remove-reaction event JSON did not match RemoveReactionEvent",
    )?;
    let user_address = ev
        .actor_address
        .unwrap_or(ev.user_address);
    Some(vec![SocialEventRow::RemoveReaction {
        object_id: ev.object_id,
        user_address,
        reaction_text: ev.reaction_text,
        is_post: ev.is_post,
    }])
}

fn process_repost_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let ev: RepostEvent = common::deserialize_social_event_json(
        "post",
        "RepostEvent",
        event_id,
        data,
        "post repost event JSON did not match RepostEvent",
    )?;
    let now = Utc::now();
    let created_at = if ev.created_at > 0 {
        ev.created_at as i64
    } else {
        now.timestamp_millis() as i64
    };
    let id = format!("{}:{}", ev.repost_id, created_at);
    let (actor_address, sub_agent_id, action_identity_class) =
        attribution_fields(data, &ev.owner);

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
        actor_address: ev.actor_address.or(actor_address),
        sub_agent_id: ev.sub_agent_id.or(sub_agent_id),
        action_identity_class: ev
            .action_identity_class
            .map(i16::from)
            .or(action_identity_class),
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
    let ev: TipEvent = common::deserialize_social_event_json(
        "post",
        "TipEvent",
        event_id,
        data,
        "post tip event JSON did not match TipEvent",
    )?;
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
        coin_type: if ev.coin_type.is_empty() {
            CURRENCY_MYSO.to_string()
        } else {
            ev.coin_type.clone()
        },
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
    let currency = if ev.coin_type.is_empty() {
        CURRENCY_MYSO.to_string()
    } else {
        ev.coin_type.clone()
    };
    let unified_revenue = NewUnifiedRevenue::from_tip(
        revenue_type,
        ev.to.clone(),
        ev.amount as i64,
        currency,
        ev.object_id.clone(),
        content_type,
        ev.from.clone(),
        created_at,
        event_id.to_string(),
    );
    let object_id = ev.object_id.clone();
    let recipient = ev.to.clone();
    Some(vec![
        SocialEventRow::Tip(tip),
        SocialEventRow::PostTipsReceivedIncrement {
            object_id,
            recipient,
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
    let ev: ModerationEvent = common::deserialize_social_event_json(
        "post",
        "PostModerationEvent",
        event_id,
        data,
        "post moderation event JSON did not match ModerationEvent",
    )?;
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

fn process_content_update_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: ContentUpdateEvent = common::deserialize_social_event_json(
        "post",
        "ContentUpdateEvent",
        event_id,
        data,
        "post content update event JSON did not match ContentUpdateEvent",
    )?;
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
    let ev: PostParametersUpdatedEvent = common::deserialize_social_event_json(
        "post",
        "PostParametersUpdatedEvent",
        event_id,
        data,
        "post PostParametersUpdatedEvent JSON did not match PostParametersUpdatedEvent",
    )?;
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
    let ev: OwnershipTransferEvent = common::deserialize_social_event_json(
        "post",
        "OwnershipTransferEvent",
        event_id,
        data,
        "post OwnershipTransferEvent JSON did not match OwnershipTransferEvent",
    )?;
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
    let ev: ReportEvent = common::deserialize_social_event_json(
        "post",
        "ReportEvent",
        event_id,
        data,
        "post report event JSON did not match ReportEvent",
    )?;
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

fn process_deletion_event(
    data: &serde_json::Value,
    event_id: &str,
    event_type_for_metrics: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: DeletionEvent = common::deserialize_social_event_json(
        "post",
        event_type_for_metrics,
        event_id,
        data,
        "post deletion event JSON did not match DeletionEvent",
    )?;
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
    let ev: PromotedPostCreatedEvent = match serde_json::from_value(data.clone()) {
        Ok(v) => v,
        Err(e) => {
            crate::metrics::SocialMetrics::record_event_json_deserialize_failed(
                "post",
                "PromotedPostCreatedEvent",
            );
            let keys: Vec<String> = data
                .as_object()
                .map(|m| m.keys().cloned().collect())
                .unwrap_or_default();
            tracing::warn!(
                event_id = %event_id,
                error = %e,
                json_keys = ?keys,
                "post PromotedPostCreatedEvent JSON did not match PromotedPostCreatedEvent"
            );
            return None;
        }
    };
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
    let ev: PromotedPostViewConfirmedEvent = match serde_json::from_value(data.clone()) {
        Ok(v) => v,
        Err(e) => {
            crate::metrics::SocialMetrics::record_event_json_deserialize_failed(
                "post",
                "PromotedPostViewConfirmedEvent",
            );
            let keys: Vec<String> = data
                .as_object()
                .map(|m| m.keys().cloned().collect())
                .unwrap_or_default();
            tracing::warn!(
                event_id = %event_id,
                error = %e,
                json_keys = ?keys,
                "post PromotedPostViewConfirmedEvent JSON did not match struct"
            );
            return None;
        }
    };
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
    let ev: PromotionStatusToggledEvent = match serde_json::from_value(data.clone()) {
        Ok(v) => v,
        Err(e) => {
            crate::metrics::SocialMetrics::record_event_json_deserialize_failed(
                "post",
                "PromotionStatusToggledEvent",
            );
            let keys: Vec<String> = data
                .as_object()
                .map(|m| m.keys().cloned().collect())
                .unwrap_or_default();
            tracing::warn!(
                event_id = %event_id,
                error = %e,
                json_keys = ?keys,
                "post PromotionStatusToggledEvent JSON did not match struct"
            );
            return None;
        }
    };
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
    let ev: PromotionFundsWithdrawnEvent = match serde_json::from_value(data.clone()) {
        Ok(v) => v,
        Err(e) => {
            crate::metrics::SocialMetrics::record_event_json_deserialize_failed(
                "post",
                "PromotionFundsWithdrawnEvent",
            );
            let keys: Vec<String> = data
                .as_object()
                .map(|m| m.keys().cloned().collect())
                .unwrap_or_default();
            tracing::warn!(
                event_id = %event_id,
                error = %e,
                json_keys = ?keys,
                "post PromotionFundsWithdrawnEvent JSON did not match struct"
            );
            return None;
        }
    };
    Some(vec![SocialEventRow::PromotionBudgetEvent {
        promotion_id: ev.promotion_id,
        owner: ev.owner,
        withdrawn_amount: ev.withdrawn_amount as i64,
        timestamp: ev.timestamp as i64,
        transaction_id: event_id.to_string(),
    }])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::SocialEventRow;
    use myso_indexer_alt_social_schema::models::CURRENCY_MYSO;

    #[test]
    fn reaction_event_produces_single_reaction_row() {
        let data = serde_json::json!({
            "object_id": "0xpost123",
            "user_address": "0xuser456",
            "reaction_text": "👍",
            "is_post": true,
            "principal_owner": "0xowner789",
            "actor_address": "0xactorabc",
            "sub_agent_id": null,
            "action_identity_class": 0,
        });
        let rows = handle_post_event("ReactionEvent", &data, "tx:rx1").expect("rows");
        assert_eq!(rows.len(), 1);
        assert!(
            rows.iter().all(|r| matches!(r, SocialEventRow::Reaction(_))),
            "expected only Reaction row, no ReactionCount"
        );
    }

    #[test]
    fn remove_reaction_event_produces_remove_row() {
        let data = serde_json::json!({
            "object_id": "0xpost123",
            "user_address": "0xuser456",
            "reaction_text": "👍",
            "is_post": true,
            "principal_owner": "0xowner789",
            "actor_address": "0xactorabc",
            "sub_agent_id": null,
            "action_identity_class": 0,
        });
        let rows = handle_post_event("RemoveReactionEvent", &data, "tx:rm1").expect("rows");
        assert_eq!(rows.len(), 1);
        assert!(
            rows.iter().any(|r| matches!(
                r,
                SocialEventRow::RemoveReaction {
                    object_id,
                    user_address,
                    reaction_text,
                    is_post,
                } if object_id == "0xpost123"
                    && user_address == "0xactorabc"
                    && reaction_text == "👍"
                    && *is_post
            )),
            "expected RemoveReaction row with actor_address as user_address"
        );
    }

    #[test]
    fn post_created_quote_repost_increments_parent_repost_count() {
        let parent = "0xaaaabbbbaaaabbbbaaaabbbbaaaabbbb";
        let data = serde_json::json!({
            "post_id": "0xccc",
            "owner": "0xddd",
            "profile_id": "0xeee",
            "content": "quote body",
            "post_type": POST_TYPE_QUOTE_REPOST,
            "parent_post_id": parent,
            "mentions": null,
            "media_urls": null,
            "metadata_json": null,
            "mydata_id": null,
            "promotion_id": null,
            "revenue_redirect_to": null,
            "revenue_redirect_percentage": null,
            "enable_spt": false,
            "enable_poc": false,
            "enable_spot": false,
            "spot_id": null,
            "spt_id": null,
        });
        let rows = handle_post_event("PostCreatedEvent", &data, "digest:0").expect("rows");
        assert!(
            rows.iter().any(|r| matches!(
                r,
                SocialEventRow::PostRepostCountIncrement {
                    original_id,
                    is_original_post: true,
                } if original_id == parent
            )),
            "expected PostRepostCountIncrement for quote repost parent"
        );
    }

    #[test]
    fn post_created_standard_repost_does_not_duplicate_repost_count_from_quote_path() {
        let data = serde_json::json!({
            "post_id": "0xccc",
            "owner": "0xddd",
            "profile_id": "0xeee",
            "content": "",
            "post_type": "repost",
            "parent_post_id": "0xparent",
            "mentions": null,
            "media_urls": null,
            "metadata_json": null,
            "mydata_id": null,
            "promotion_id": null,
            "revenue_redirect_to": null,
            "revenue_redirect_percentage": null,
            "enable_spt": false,
            "enable_poc": false,
            "enable_spot": false,
            "spot_id": null,
            "spt_id": null,
        });
        let rows = handle_post_event("PostCreatedEvent", &data, "digest:0").expect("rows");
        assert!(!rows
            .iter()
            .any(|r| matches!(r, SocialEventRow::PostRepostCountIncrement { .. })));
    }

    #[test]
    fn post_reported_event_yields_report_row() {
        let data = serde_json::json!({
            "object_id": "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55",
            "is_comment": false,
            "reporter": "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
            "reason_code": 6,
            "description": "Short description of the issue here.",
            "reported_at": 1714113519157_u64,
        });
        let rows = handle_post_event("PostReportedEvent", &data, "digest:7").expect("rows");
        let SocialEventRow::Report(r) = &rows[0] else {
            panic!("expected Report row");
        };
        assert_eq!(
            r.object_id,
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55"
        );
        assert!(!r.is_comment);
        assert_eq!(r.reason_code, 6);
        assert_eq!(r.description, "Short description of the issue here.");
        assert_eq!(r.reported_at, 1_714_113_519_157);
        assert_eq!(r.transaction_id, "digest:7");
    }

    #[test]
    fn post_moderation_event_yields_moderation_rows() {
        let data = serde_json::json!({
            "object_id": "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55",
            "platform_id": "0x05a761d1fe77ff1006e210727f25a7f3137c6d1e87dc6dab898fd685736cff5a",
            "removed": true,
            "moderated_by": "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
            "moderated_at": 0u64,
        });
        let rows = handle_post_event("PostModerationEvent", &data, "tx:1").expect("rows");
        assert_eq!(rows.len(), 2);
        let SocialEventRow::ModerationEvent(m) = &rows[0] else {
            panic!("expected ModerationEvent");
        };
        assert_eq!(
            m.object_id,
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55"
        );
        assert_eq!(
            m.platform_id,
            "0x05a761d1fe77ff1006e210727f25a7f3137c6d1e87dc6dab898fd685736cff5a"
        );
        assert!(m.removed);
        assert_eq!(
            m.moderated_by,
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8"
        );
        assert_eq!(m.transaction_id, "tx:1");
        let SocialEventRow::PostModerationUpdate {
            object_id,
            removed,
            moderated_by,
        } = &rows[1]
        else {
            panic!("expected PostModerationUpdate");
        };
        assert_eq!(
            object_id,
            "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55"
        );
        assert!(*removed);
        assert_eq!(
            moderated_by,
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8"
        );
    }

    #[test]
    fn post_deleted_event_yields_deletion_rows() {
        let post_oid = "0x9c5f189cdf741b0cf724297a5aee8536a0ef41ad356bed6070cc6703ec949c55";
        let data = serde_json::json!({
            "object_id": post_oid,
            "owner": "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
            "profile_id": "0x000000000000000000000000000000000000000000000000000000006f199773",
            "is_post": true,
            "post_type": "quote_repost",
            "post_id": post_oid,
            "deleted_at": 1_717_200_000_000_u64,
        });
        let rows = handle_post_event("PostDeletedEvent", &data, "tx:del1").expect("rows");
        assert_eq!(rows.len(), 3);
        let SocialEventRow::DeletionEvent(d) = &rows[0] else {
            panic!("expected DeletionEvent");
        };
        assert_eq!(d.object_id, post_oid);
        assert!(d.is_post);
        assert_eq!(d.post_type.as_deref(), Some("quote_repost"));
        assert_eq!(d.deleted_at, 1_717_200_000_000);
        assert_eq!(d.transaction_id, "tx:del1");
        assert!(matches!(
            &rows[1],
            SocialEventRow::ProfilePostCountDecrement { .. }
        ));
        assert!(matches!(
            &rows[2],
            SocialEventRow::PostDeletedAtUpdate { .. }
        ));
    }

    #[test]
    fn promoted_post_created_event_yields_row() {
        let data = serde_json::json!({
            "post_id": "0x320c97b64e7228da3b9f8a6adc5401b289bf41cf3f4e3a2e159d5ee939b8cdda",
            "owner": "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
            "profile_id": "0xccf58c286df1ee89368c9b5dfb4f2bc79ca97ce57611df33cc340556a9a260c3",
            "payment_per_view": 1000000_u64,
            "total_budget": 1000000_u64,
            "created_at": 1_742_000_000_000_u64,
        });
        let rows = handle_post_event("PromotedPostCreatedEvent", &data, "tx:promo1").expect("rows");
        assert_eq!(rows.len(), 1);
        let SocialEventRow::PromotedPost {
            post_id,
            owner,
            profile_id,
            payment_per_view,
            total_budget,
            transaction_id,
            ..
        } = &rows[0]
        else {
            panic!("expected PromotedPost");
        };
        assert_eq!(
            post_id,
            "0x320c97b64e7228da3b9f8a6adc5401b289bf41cf3f4e3a2e159d5ee939b8cdda"
        );
        assert_eq!(
            owner,
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72"
        );
        assert_eq!(
            profile_id,
            "0xccf58c286df1ee89368c9b5dfb4f2bc79ca97ce57611df33cc340556a9a260c3"
        );
        assert_eq!(*payment_per_view, 1_000_000);
        assert_eq!(*total_budget, 1_000_000);
        assert_eq!(transaction_id, "tx:promo1");
    }

    #[test]
    fn tip_event_comment_uses_recipient_for_owner_match() {
        let data = serde_json::json!({
            "object_id": "0xccc00000000000000000000000000000000000000000000000000000cccc",
            "from": "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
            "to": "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
            "amount": 800_u64,
            "is_post": false,
            "tip_time": 0u64,
        });
        let rows = handle_post_event("TipEvent", &data, "tx:tip2").expect("rows");
        let SocialEventRow::Tip(tip_row) = &rows[0] else {
            panic!("expected Tip");
        };
        assert_eq!(tip_row.coin_type, CURRENCY_MYSO);
        let SocialEventRow::PostTipsReceivedIncrement {
            recipient, is_post, ..
        } = &rows[1]
        else {
            panic!("expected PostTipsReceivedIncrement");
        };
        assert!(!*is_post);
        assert_eq!(
            recipient,
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72"
        );
    }

    #[test]
    fn tip_event_yields_tip_revenue_and_increment() {
        let data = serde_json::json!({
            "object_id": "0xa7953fb1af6d0495b3da10d4d25888158e8dc451fa5354a9723dc70676d38f3d",
            "from": "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
            "to": "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
            "amount": 5000000000_u64,
            "coin_type": "0x0000000000000000000000000000000000000000000000000000000000000002::myso::MYSO",
            "is_post": true,
            "tip_time": 0u64,
        });
        let rows = handle_post_event("TipEvent", &data, "tx:tip1").expect("rows");
        assert_eq!(rows.len(), 3);
        let SocialEventRow::Tip(t) = &rows[0] else {
            panic!("expected Tip");
        };
        assert_eq!(t.amount, 5_000_000_000);
        assert_eq!(
            t.tipper,
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8"
        );
        assert_eq!(
            t.recipient,
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72"
        );
        assert_eq!(t.transaction_id, "tx:tip1");
        assert_eq!(
            t.coin_type,
            "0x0000000000000000000000000000000000000000000000000000000000000002::myso::MYSO"
        );
        let SocialEventRow::PostTipsReceivedIncrement {
            object_id,
            recipient,
            amount,
            is_post,
        } = &rows[1]
        else {
            panic!("expected PostTipsReceivedIncrement");
        };
        assert_eq!(*amount, 5_000_000_000);
        assert!(*is_post);
        assert_eq!(
            recipient,
            "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72"
        );
        assert_eq!(
            object_id,
            "0xa7953fb1af6d0495b3da10d4d25888158e8dc451fa5354a9723dc70676d38f3d"
        );
    }

    #[test]
    fn comment_deleted_event_yields_deletion_rows() {
        let comment_id = "0xcccc00000000000000000000000000000000000000000000000000000000cccc";
        let post_id = "0xdddd00000000000000000000000000000000000000000000000000000000dddd";
        let data = serde_json::json!({
            "object_id": comment_id,
            "owner": "0x8a8d7490ab0dee5e6a0092a463ade496a1352d89b5091e96e3d356d4f8577f72",
            "profile_id": "0x000000000000000000000000000000000000000000000000000000006f199774",
            "is_post": false,
            "post_type": null,
            "post_id": post_id,
            "deleted_at": 1_717_201_000_000_u64,
        });
        let rows = handle_post_event("CommentDeletedEvent", &data, "tx:del2").expect("rows");
        assert_eq!(rows.len(), 3);
        let SocialEventRow::DeletionEvent(d) = &rows[0] else {
            panic!("expected DeletionEvent");
        };
        assert_eq!(d.object_id, comment_id);
        assert!(!d.is_post);
        assert_eq!(d.transaction_id, "tx:del2");
        assert!(matches!(
            &rows[1],
            SocialEventRow::PostCommentCountIncrement { post_id: pid, delta: -1 } if pid == post_id
        ));
        assert!(matches!(
            &rows[2],
            SocialEventRow::CommentDeletedAtUpdate { .. }
        ));
    }
}
