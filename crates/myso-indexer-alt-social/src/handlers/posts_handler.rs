// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Posts pipeline: indexes post, comment, reaction, repost, tip module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Text};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewComment, NewDeletionEvent, NewModerationEvent, NewPost, NewPostTransfer, NewPromotedPost,
    NewPromotionBudgetEvent, NewPromotionStatusEvent, NewPromotionView, NewReaction,
    NewReactionCount, NewReport, NewRepost, NewTip, NewUnifiedRevenue,
};
use myso_indexer_alt_social_schema::schema::{
    comments, post_config, posts, promoted_posts, promotion_budget_events, promotion_status_events,
    promotion_views, reaction_counts, reactions, reposts, tips,
};
use myso_indexer_alt_social_schema::schema::{
    posts_deletion_events, posts_moderation_events, posts_reports, posts_transfers, profiles,
    unified_revenue,
};

use super::common;
use super::events;
use super::post;

const POST_MODULES: &[&str] = &["post", "comment", "reaction", "repost", "tip"];

#[derive(Debug, Clone)]
pub enum PostRow {
    Post(NewPost),
    Comment(NewComment),
    Reaction(NewReaction),
    ReactionCount(NewReactionCount),
    RemoveReaction {
        object_id: String,
        user_address: String,
        reaction_text: String,
        is_post: bool,
    },
    Repost(NewRepost),
    Tip(NewTip),
    ModerationEvent(NewModerationEvent),
    Report(NewReport),
    DeletionEvent(NewDeletionEvent),
    PostCommentCountIncrement {
        post_id: String,
        delta: i64,
    },
    PostCommentCountDecrementByComment {
        comment_id: String,
        owner: String,
    },
    ProfilePostCountIncrement {
        owner_address: String,
    },
    ProfilePostCountDecrement {
        owner_address: String,
    },
    PostRepostCountIncrement {
        original_id: String,
        is_original_post: bool,
    },
    PostTipsReceivedIncrement {
        object_id: String,
        amount: i64,
        is_post: bool,
    },
    PostModerationUpdate {
        object_id: String,
        removed: bool,
        moderated_by: String,
    },
    PostDeletedAtUpdate {
        object_id: String,
        owner: String,
        deleted_at: i64,
    },
    CommentDeletedAtUpdate {
        object_id: String,
        owner: String,
        deleted_at: i64,
    },
    PostContentUpdate {
        object_id: String,
        content: String,
        media_urls: Option<serde_json::Value>,
        mentions: Option<serde_json::Value>,
        metadata_json: Option<serde_json::Value>,
        is_post: bool,
        updated_at: i64,
    },
    PostOwnerUpdate {
        object_id: String,
        new_owner: String,
        is_post: bool,
    },
    PostTransfer(NewPostTransfer),
    PostConfig {
        updated_by: String,
        max_content_length: i64,
        max_media_urls: i64,
        max_mentions: i64,
        max_metadata_size: i64,
        max_description_length: i64,
        max_reaction_length: i64,
        commenter_tip_percentage: i64,
        repost_tip_percentage: i64,
        version: Option<i64>,
        updated_at: i64,
        transaction_id: String,
    },
    PromotedPost {
        post_id: String,
        owner: String,
        profile_id: String,
        payment_per_view: i64,
        total_budget: i64,
        created_at: i64,
        transaction_id: String,
    },
    PromotionView {
        promotion_id: String,
        viewer: String,
        payment_amount: i64,
        view_duration: i64,
        platform_id: String,
        timestamp: i64,
        transaction_id: String,
    },
    PromotionStatusEvent {
        promotion_id: String,
        toggled_by: String,
        new_status: bool,
        timestamp: i64,
        transaction_id: String,
    },
    PromotionBudgetEvent {
        promotion_id: String,
        owner: String,
        withdrawn_amount: i64,
        timestamp: i64,
        transaction_id: String,
    },
    UnifiedRevenue(NewUnifiedRevenue),
}

impl PostRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        use crate::handlers::SocialEventRow;
        match row {
            SocialEventRow::Post(p) => Some(PostRow::Post(p)),
            SocialEventRow::Comment(c) => Some(PostRow::Comment(c)),
            SocialEventRow::Reaction(r) => Some(PostRow::Reaction(r)),
            SocialEventRow::ReactionCount(rc) => Some(PostRow::ReactionCount(rc)),
            SocialEventRow::RemoveReaction {
                object_id,
                user_address,
                reaction_text,
                is_post,
            } => Some(PostRow::RemoveReaction {
                object_id,
                user_address,
                reaction_text,
                is_post,
            }),
            SocialEventRow::Repost(r) => Some(PostRow::Repost(r)),
            SocialEventRow::Tip(t) => Some(PostRow::Tip(t)),
            SocialEventRow::ModerationEvent(m) => Some(PostRow::ModerationEvent(m)),
            SocialEventRow::Report(r) => Some(PostRow::Report(r)),
            SocialEventRow::DeletionEvent(d) => Some(PostRow::DeletionEvent(d)),
            SocialEventRow::PostCommentCountIncrement { post_id, delta } => {
                Some(PostRow::PostCommentCountIncrement { post_id, delta })
            }
            SocialEventRow::PostCommentCountDecrementByComment { comment_id, owner } => {
                Some(PostRow::PostCommentCountDecrementByComment { comment_id, owner })
            }
            SocialEventRow::ProfilePostCountIncrement { owner_address } => {
                Some(PostRow::ProfilePostCountIncrement { owner_address })
            }
            SocialEventRow::ProfilePostCountDecrement { owner_address } => {
                Some(PostRow::ProfilePostCountDecrement { owner_address })
            }
            SocialEventRow::PostRepostCountIncrement {
                original_id,
                is_original_post,
            } => Some(PostRow::PostRepostCountIncrement {
                original_id,
                is_original_post,
            }),
            SocialEventRow::PostTipsReceivedIncrement {
                object_id,
                amount,
                is_post,
            } => Some(PostRow::PostTipsReceivedIncrement {
                object_id,
                amount,
                is_post,
            }),
            SocialEventRow::PostModerationUpdate {
                object_id,
                removed,
                moderated_by,
            } => Some(PostRow::PostModerationUpdate {
                object_id,
                removed,
                moderated_by,
            }),
            SocialEventRow::PostDeletedAtUpdate {
                object_id,
                owner,
                deleted_at,
            } => Some(PostRow::PostDeletedAtUpdate {
                object_id,
                owner,
                deleted_at,
            }),
            SocialEventRow::CommentDeletedAtUpdate {
                object_id,
                owner,
                deleted_at,
            } => Some(PostRow::CommentDeletedAtUpdate {
                object_id,
                owner,
                deleted_at,
            }),
            SocialEventRow::PostContentUpdate {
                object_id,
                content,
                media_urls,
                mentions,
                metadata_json,
                is_post,
                updated_at,
            } => Some(PostRow::PostContentUpdate {
                object_id,
                content,
                media_urls,
                mentions,
                metadata_json,
                is_post,
                updated_at,
            }),
            SocialEventRow::PostOwnerUpdate {
                object_id,
                new_owner,
                is_post,
            } => Some(PostRow::PostOwnerUpdate {
                object_id,
                new_owner,
                is_post,
            }),
            SocialEventRow::PostTransfer(t) => Some(PostRow::PostTransfer(t)),
            SocialEventRow::PostConfig {
                updated_by,
                max_content_length,
                max_media_urls,
                max_mentions,
                max_metadata_size,
                max_description_length,
                max_reaction_length,
                commenter_tip_percentage,
                repost_tip_percentage,
                version,
                updated_at,
                transaction_id,
            } => Some(PostRow::PostConfig {
                updated_by,
                max_content_length,
                max_media_urls,
                max_mentions,
                max_metadata_size,
                max_description_length,
                max_reaction_length,
                commenter_tip_percentage,
                repost_tip_percentage,
                version,
                updated_at,
                transaction_id,
            }),
            SocialEventRow::PromotedPost {
                post_id,
                owner,
                profile_id,
                payment_per_view,
                total_budget,
                created_at,
                transaction_id,
            } => Some(PostRow::PromotedPost {
                post_id,
                owner,
                profile_id,
                payment_per_view,
                total_budget,
                created_at,
                transaction_id,
            }),
            SocialEventRow::PromotionView {
                promotion_id,
                viewer,
                payment_amount,
                view_duration,
                platform_id,
                timestamp,
                transaction_id,
            } => Some(PostRow::PromotionView {
                promotion_id,
                viewer,
                payment_amount,
                view_duration,
                platform_id,
                timestamp,
                transaction_id,
            }),
            SocialEventRow::PromotionStatusEvent {
                promotion_id,
                toggled_by,
                new_status,
                timestamp,
                transaction_id,
            } => Some(PostRow::PromotionStatusEvent {
                promotion_id,
                toggled_by,
                new_status,
                timestamp,
                transaction_id,
            }),
            SocialEventRow::PromotionBudgetEvent {
                promotion_id,
                owner,
                withdrawn_amount,
                timestamp,
                transaction_id,
            } => Some(PostRow::PromotionBudgetEvent {
                promotion_id,
                owner,
                withdrawn_amount,
                timestamp,
                transaction_id,
            }),
            SocialEventRow::UnifiedRevenue(r) => Some(PostRow::UnifiedRevenue(r)),
            _ => None,
        }
    }
}

impl FieldCount for PostRow {
    const FIELD_COUNT: usize = 50;
}

pub struct PostsHandler;

#[async_trait]
impl Processor for PostsHandler {
    const NAME: &'static str = "posts";

    type Value = PostRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let mut values = Vec::new();
        for tx in &checkpoint.transactions {
            let tx_digest = tx.transaction.digest().to_string();
            let Some(events) = &tx.events else {
                continue;
            };
            for (event_seq, ev) in events.data.iter().enumerate() {
                if !common::is_social_package_event(&ev.package_id, &ev.type_.address) {
                    continue;
                }
                let module = ev.type_.module.as_str();
                if !POST_MODULES.contains(&module) {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(module, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                if let Some(rows) = post::handle_post_event(event_name, &event_data, &event_id) {
                    for row in rows {
                        if let Some(r) = PostRow::from_social(row) {
                            values.push(r);
                        }
                    }
                }
            }
        }
        Ok(values)
    }
}

#[async_trait]
impl Handler for PostsHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        use diesel::sql_query;

        let mut total = 0;
        for row in values {
            match row {
                PostRow::Post(p) => {
                    total += diesel::insert_into(posts::table)
                        .values(p)
                        .on_conflict((posts::id, posts::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                PostRow::Comment(c) => {
                    total += diesel::insert_into(comments::table)
                        .values(c)
                        .on_conflict((comments::id, comments::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                PostRow::Reaction(r) => {
                    total += diesel::insert_into(reactions::table)
                        .values(r)
                        .on_conflict_do_nothing()
                        .execute(conn)
                        .await?;
                }
                PostRow::ReactionCount(rc) => {
                    total += diesel::insert_into(reaction_counts::table)
                        .values(rc)
                        .on_conflict((reaction_counts::object_id, reaction_counts::reaction_text))
                        .do_update()
                        .set(reaction_counts::count.eq(reaction_counts::count + 1))
                        .execute(conn)
                        .await?;
                }
                PostRow::RemoveReaction {
                    object_id,
                    user_address,
                    reaction_text,
                    is_post: _,
                } => {
                    let _ = diesel::delete(reactions::table)
                        .filter(reactions::object_id.eq(object_id))
                        .filter(reactions::user_address.eq(user_address))
                        .filter(reactions::reaction_text.eq(reaction_text))
                        .execute(conn)
                        .await;
                    let _ = diesel::update(reaction_counts::table)
                        .filter(reaction_counts::object_id.eq(object_id))
                        .filter(reaction_counts::reaction_text.eq(reaction_text))
                        .set(reaction_counts::count.eq(reaction_counts::count - 1))
                        .execute(conn)
                        .await;
                }
                PostRow::Repost(r) => {
                    total += diesel::insert_into(reposts::table)
                        .values(r)
                        .on_conflict(reposts::id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                PostRow::Tip(t) => {
                    total += diesel::insert_into(tips::table)
                        .values(t)
                        .execute(conn)
                        .await?;
                }
                PostRow::ModerationEvent(m) => {
                    total += diesel::insert_into(posts_moderation_events::table)
                        .values(m)
                        .execute(conn)
                        .await?;
                }
                PostRow::Report(r) => {
                    total += diesel::insert_into(posts_reports::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                PostRow::DeletionEvent(d) => {
                    total += diesel::insert_into(posts_deletion_events::table)
                        .values(d)
                        .execute(conn)
                        .await?;
                }
                PostRow::PostCommentCountIncrement { post_id, delta } => {
                    let _ = diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set(posts::comment_count.eq(posts::comment_count + delta))
                        .execute(conn)
                        .await;
                }
                PostRow::PostCommentCountDecrementByComment { comment_id, owner } => {
                    let _ = sql_query(
                        "UPDATE posts SET comment_count = comment_count - 1 WHERE post_id = (SELECT post_id FROM comments WHERE comment_id = $1 AND owner = $2 LIMIT 1)",
                    )
                    .bind::<Text, _>(comment_id)
                    .bind::<Text, _>(owner)
                    .execute(conn)
                    .await;
                }
                PostRow::ProfilePostCountIncrement { owner_address } => {
                    let _ = diesel::update(profiles::table)
                        .filter(profiles::owner_address.eq(owner_address))
                        .set(profiles::post_count.eq(profiles::post_count + 1))
                        .execute(conn)
                        .await;
                }
                PostRow::ProfilePostCountDecrement { owner_address } => {
                    let _ = diesel::update(profiles::table)
                        .filter(profiles::owner_address.eq(owner_address))
                        .set(profiles::post_count.eq(profiles::post_count - 1))
                        .execute(conn)
                        .await;
                }
                PostRow::PostRepostCountIncrement {
                    original_id,
                    is_original_post,
                } => {
                    if *is_original_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(original_id))
                            .set(posts::repost_count.eq(posts::repost_count + 1))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(original_id))
                            .set(comments::repost_count.eq(comments::repost_count + 1))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::PostTipsReceivedIncrement {
                    object_id,
                    amount,
                    is_post,
                } => {
                    if *is_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(object_id))
                            .set(posts::tips_received.eq(posts::tips_received + amount))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set(comments::tips_received.eq(comments::tips_received + amount))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::PostModerationUpdate {
                    object_id,
                    removed,
                    moderated_by,
                } => {
                    let post_updated = diesel::update(posts::table)
                        .filter(posts::post_id.eq(object_id))
                        .set((
                            posts::removed_from_platform.eq(*removed),
                            posts::removed_by.eq(Some(moderated_by.clone())),
                        ))
                        .execute(conn)
                        .await
                        .unwrap_or(0);
                    if post_updated == 0 {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set((
                                comments::removed_from_platform.eq(*removed),
                                comments::removed_by.eq(Some(moderated_by.clone())),
                            ))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::PostDeletedAtUpdate {
                    object_id,
                    owner,
                    deleted_at,
                } => {
                    let _ = diesel::update(posts::table)
                        .filter(posts::post_id.eq(object_id))
                        .filter(posts::owner.eq(owner))
                        .set(posts::deleted_at.eq(Some(*deleted_at)))
                        .execute(conn)
                        .await;
                }
                PostRow::CommentDeletedAtUpdate {
                    object_id,
                    owner,
                    deleted_at,
                } => {
                    let _ = diesel::update(comments::table)
                        .filter(comments::comment_id.eq(object_id))
                        .filter(comments::owner.eq(owner))
                        .set(comments::deleted_at.eq(Some(*deleted_at)))
                        .execute(conn)
                        .await;
                }
                PostRow::PostContentUpdate {
                    object_id,
                    content,
                    media_urls,
                    mentions,
                    metadata_json,
                    is_post,
                    updated_at,
                } => {
                    if *is_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(object_id))
                            .set((
                                posts::content.eq(content),
                                posts::media_urls.eq(media_urls),
                                posts::mentions.eq(mentions),
                                posts::metadata_json.eq(metadata_json),
                                posts::updated_at.eq(Some(*updated_at)),
                            ))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set((
                                comments::content.eq(content),
                                comments::media_urls.eq(media_urls),
                                comments::mentions.eq(mentions),
                                comments::metadata_json.eq(metadata_json),
                                comments::updated_at.eq(Some(*updated_at)),
                            ))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::PostOwnerUpdate {
                    object_id,
                    new_owner,
                    is_post,
                } => {
                    if *is_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(object_id))
                            .set(posts::owner.eq(new_owner))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set(comments::owner.eq(new_owner))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::PostTransfer(transfer) => {
                    total += diesel::insert_into(posts_transfers::table)
                        .values(transfer)
                        .execute(conn)
                        .await?;
                }
                PostRow::PostConfig {
                    updated_by,
                    max_content_length,
                    max_media_urls,
                    max_mentions,
                    max_metadata_size,
                    max_description_length,
                    max_reaction_length,
                    commenter_tip_percentage,
                    repost_tip_percentage,
                    version,
                    updated_at,
                    transaction_id,
                } => {
                    let version_val = version.unwrap_or(-1);
                    if version_val >= 0 {
                        let _ = diesel::insert_into(post_config::table)
                            .values((
                                post_config::updated_by.eq(updated_by),
                                post_config::max_content_length.eq(max_content_length),
                                post_config::max_media_urls.eq(max_media_urls),
                                post_config::max_mentions.eq(max_mentions),
                                post_config::max_metadata_size.eq(max_metadata_size),
                                post_config::max_description_length.eq(max_description_length),
                                post_config::max_reaction_length.eq(max_reaction_length),
                                post_config::commenter_tip_percentage.eq(commenter_tip_percentage),
                                post_config::repost_tip_percentage.eq(repost_tip_percentage),
                                post_config::version.eq(version_val),
                                post_config::updated_at.eq(updated_at),
                                post_config::transaction_id.eq(transaction_id),
                            ))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = sql_query(
                            r#"INSERT INTO post_config (updated_by, max_content_length, max_media_urls, max_mentions, max_metadata_size, max_description_length, max_reaction_length, commenter_tip_percentage, repost_tip_percentage, version, updated_at, transaction_id)
                               SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, COALESCE((SELECT MAX(version) FROM post_config), 0) + 1, $10, $11"#,
                        )
                        .bind::<Text, _>(updated_by)
                        .bind::<BigInt, _>(max_content_length)
                        .bind::<BigInt, _>(max_media_urls)
                        .bind::<BigInt, _>(max_mentions)
                        .bind::<BigInt, _>(max_metadata_size)
                        .bind::<BigInt, _>(max_description_length)
                        .bind::<BigInt, _>(max_reaction_length)
                        .bind::<BigInt, _>(commenter_tip_percentage)
                        .bind::<BigInt, _>(repost_tip_percentage)
                        .bind::<BigInt, _>(updated_at)
                        .bind::<Text, _>(transaction_id)
                        .execute(conn)
                        .await;
                    }
                }
                PostRow::PromotedPost {
                    post_id,
                    owner,
                    profile_id,
                    payment_per_view,
                    total_budget,
                    created_at,
                    transaction_id,
                } => {
                    let promotion_id_opt: Option<String> = posts::table
                        .filter(posts::post_id.eq(&post_id))
                        .order(posts::time.desc())
                        .select(posts::promotion_id)
                        .first::<Option<String>>(conn)
                        .await
                        .ok()
                        .flatten();
                    if let Some(promotion_id) = promotion_id_opt {
                        let time = chrono::DateTime::from_timestamp(created_at / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotedPost {
                            promotion_id,
                            post_id: post_id.clone(),
                            owner: owner.clone(),
                            profile_id: profile_id.clone(),
                            payment_per_view: *payment_per_view,
                            total_budget: *total_budget,
                            remaining_budget: *total_budget,
                            active: false,
                            created_at: *created_at,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promoted_posts::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                    }
                }
                PostRow::PromotionView {
                    promotion_id,
                    viewer,
                    payment_amount,
                    view_duration,
                    platform_id,
                    timestamp,
                    transaction_id,
                } => {
                    let post_id_opt: Option<String> = promoted_posts::table
                        .filter(promoted_posts::promotion_id.eq(promotion_id))
                        .order(promoted_posts::time.desc())
                        .select(promoted_posts::post_id)
                        .first::<String>(conn)
                        .await
                        .ok();
                    if let Some(post_id) = post_id_opt {
                        let time = chrono::DateTime::from_timestamp(*timestamp / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotionView {
                            post_id,
                            promotion_id: promotion_id.clone(),
                            viewer: viewer.clone(),
                            payment_amount: *payment_amount,
                            view_duration: *view_duration,
                            platform_id: platform_id.clone(),
                            timestamp: *timestamp,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promotion_views::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                    }
                }
                PostRow::PromotionStatusEvent {
                    promotion_id,
                    toggled_by,
                    new_status,
                    timestamp,
                    transaction_id,
                } => {
                    let post_id_opt: Option<String> = promoted_posts::table
                        .filter(promoted_posts::promotion_id.eq(promotion_id))
                        .order(promoted_posts::time.desc())
                        .select(promoted_posts::post_id)
                        .first::<String>(conn)
                        .await
                        .ok();
                    if let Some(post_id) = post_id_opt {
                        let time = chrono::DateTime::from_timestamp(*timestamp / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotionStatusEvent {
                            post_id,
                            promotion_id: promotion_id.clone(),
                            event_type: "status_toggled".to_string(),
                            triggered_by: toggled_by.clone(),
                            new_status: Some(*new_status),
                            amount: None,
                            timestamp: *timestamp,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promotion_status_events::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                        total += diesel::update(promoted_posts::table)
                            .filter(promoted_posts::promotion_id.eq(promotion_id))
                            .set(promoted_posts::active.eq(*new_status))
                            .execute(conn)
                            .await?;
                    }
                }
                PostRow::PromotionBudgetEvent {
                    promotion_id,
                    owner: _,
                    withdrawn_amount,
                    timestamp,
                    transaction_id,
                } => {
                    let post_id_opt: Option<String> = promoted_posts::table
                        .filter(promoted_posts::promotion_id.eq(promotion_id))
                        .order(promoted_posts::time.desc())
                        .select(promoted_posts::post_id)
                        .first::<String>(conn)
                        .await
                        .ok();
                    if let Some(post_id) = post_id_opt {
                        let time = chrono::DateTime::from_timestamp(*timestamp / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotionBudgetEvent {
                            promotion_id: promotion_id.clone(),
                            post_id,
                            event_type: "withdrawal".to_string(),
                            amount: *withdrawn_amount,
                            remaining_budget: 0,
                            timestamp: *timestamp,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promotion_budget_events::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                        total += diesel::update(promoted_posts::table)
                            .filter(promoted_posts::promotion_id.eq(promotion_id))
                            .set((
                                promoted_posts::remaining_budget.eq(0),
                                promoted_posts::active.eq(false),
                            ))
                            .execute(conn)
                            .await?;
                    }
                }
                PostRow::UnifiedRevenue(r) => {
                    total += diesel::insert_into(unified_revenue::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
