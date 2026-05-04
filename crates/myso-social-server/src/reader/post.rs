// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_query;
use diesel::sql_types::{BigInt, Text};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::{
    comments, post_config, posts, posts_deletion_events, posts_moderation_events, posts_reports,
    posts_transfers, reactions,
};

use crate::error::SocialError;
use crate::reader::types::{CommentRow, PostBasicRow, PostConfigRow, ReactionRow, RepostRow};
use myso_indexer_alt_social_schema::models::{
    PostDeletionEventRow, PostModerationEventRow, PostReport, PostTransfer, POST_TYPE_QUOTE_REPOST,
};
use myso_pg_db::Db;

pub(crate) async fn list_posts(
    db: &Db,
    owner: Option<&str>,
    post_type: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostBasicRow>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = posts::table
        .filter(posts::deleted_at.is_null())
        .into_boxed();
    if let Some(o) = owner {
        query = query.filter(posts::owner.eq(o));
    }
    if let Some(pt) = post_type {
        query = query.filter(posts::post_type.eq(pt));
    }
    let results = query
        .order_by(posts::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            posts::post_id,
            posts::owner,
            posts::profile_id,
            posts::content,
            posts::post_type,
            posts::created_at,
            posts::deleted_at,
            posts::reaction_count,
            posts::comment_count,
            posts::repost_count,
            posts::tips_received,
            posts::mydata_id,
            posts::poc_id,
            posts::revenue_redirect_to,
            posts::revenue_redirect_percentage,
            posts::poc_reasoning,
            posts::poc_evidence_urls,
            posts::poc_similarity_score,
            posts::poc_media_type,
            posts::poc_oracle_address,
            posts::poc_analyzed_at,
            posts::poc_outcome,
            posts::poc_redirection_kind,
            posts::poc_disputes_submitted,
        ))
        .load::<(
            String,
            String,
            String,
            String,
            String,
            i64,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<i64>,
            Option<String>,
            Option<serde_json::Value>,
            Option<i64>,
            Option<i16>,
            Option<String>,
            Option<i64>,
            Option<i16>,
            Option<i16>,
            i16,
        )>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(
                post_id,
                owner,
                profile_id,
                content,
                post_type,
                created_at,
                deleted_at,
                reaction_count,
                comment_count,
                repost_count,
                tips_received,
                mydata_id,
                poc_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
                poc_reasoning,
                poc_evidence_urls,
                poc_similarity_score,
                poc_media_type,
                poc_oracle_address,
                poc_analyzed_at,
                poc_outcome,
                poc_redirection_kind,
                poc_disputes_submitted,
            )| PostBasicRow {
                post_id,
                owner,
                profile_id,
                content,
                post_type,
                created_at,
                deleted_at,
                reaction_count: reaction_count.unwrap_or(0),
                comment_count: comment_count.unwrap_or(0),
                repost_count: repost_count.unwrap_or(0),
                tips_received: tips_received.unwrap_or(0),
                mydata_id,
                poc_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
                poc_reasoning,
                poc_evidence_urls,
                poc_similarity_score,
                poc_media_type,
                poc_oracle_address,
                poc_analyzed_at,
                poc_outcome,
                poc_redirection_kind,
                poc_disputes_submitted,
            },
        )
        .collect())
}

pub(crate) async fn get_post_config(db: &Db) -> Result<Option<PostConfigRow>, SocialError> {
    let mut conn = db.connect().await?;
    let result = post_config::table
        .order_by(post_config::time.desc())
        .limit(1)
        .select((
            post_config::updated_by,
            post_config::max_content_length,
            post_config::max_media_urls,
            post_config::max_mentions,
            post_config::max_metadata_size,
            post_config::max_description_length,
            post_config::max_reaction_length,
            post_config::commenter_tip_percentage,
            post_config::repost_tip_percentage,
            post_config::version,
            post_config::updated_at,
        ))
        .first::<(String, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>(&mut conn)
        .await
        .optional()?;
    Ok(result.map(
        |(
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
        )| PostConfigRow {
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
        },
    ))
}

pub(crate) async fn get_trending_posts(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostBasicRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
               reaction_count, comment_count, repost_count, tips_received, mydata_id,
               poc_id, revenue_redirect_to, revenue_redirect_percentage,
               poc_reasoning, poc_evidence_urls, poc_similarity_score, poc_media_type,
               poc_oracle_address, poc_analyzed_at,
               poc_outcome, poc_redirection_kind, poc_disputes_submitted
        FROM posts
        WHERE deleted_at IS NULL
        ORDER BY (reaction_count + comment_count + repost_count) DESC, created_at DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PostBasicRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_post_by_id(
    db: &Db,
    post_id: &str,
) -> Result<Option<PostBasicRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
               reaction_count, comment_count, repost_count, tips_received, mydata_id,
               poc_id, revenue_redirect_to, revenue_redirect_percentage,
               poc_reasoning, poc_evidence_urls, poc_similarity_score, poc_media_type,
               poc_oracle_address, poc_analyzed_at,
               poc_outcome, poc_redirection_kind, poc_disputes_submitted
        FROM posts
        WHERE (post_id = $1 OR id = $1) AND deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<PostBasicRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_post_comments(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<CommentRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = comments::table
        .filter(comments::post_id.eq(post_id))
        .filter(comments::deleted_at.is_null())
        .order_by(comments::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            comments::comment_id,
            comments::post_id,
            comments::parent_comment_id,
            comments::owner,
            comments::profile_id,
            comments::content,
            comments::created_at,
            comments::reaction_count,
            comments::comment_count,
        ))
        .load::<(
            String,
            String,
            Option<String>,
            String,
            String,
            String,
            i64,
            Option<i64>,
            Option<i64>,
        )>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(
                comment_id,
                post_id,
                parent_comment_id,
                owner,
                profile_id,
                content,
                created_at,
                reaction_count,
                comment_count,
            )| CommentRow {
                comment_id,
                post_id,
                parent_comment_id,
                owner,
                profile_id,
                content,
                created_at,
                reaction_count: reaction_count.unwrap_or(0),
                comment_count: comment_count.unwrap_or(0),
            },
        )
        .collect())
}

pub(crate) async fn get_post_reactions(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ReactionRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = reactions::table
        .filter(reactions::object_id.eq(post_id))
        .filter(reactions::is_post.eq(true))
        .order_by(reactions::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            reactions::user_address,
            reactions::reaction_text,
            reactions::created_at,
        ))
        .load::<(String, String, i64)>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(|(user_address, reaction_text, created_at)| ReactionRow {
            user_address,
            reaction_text,
            created_at,
        })
        .collect())
}

pub(crate) async fn get_post_reposts(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<RepostRow>, SocialError> {
    let mut conn = db.connect().await?;
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        repost_id: String,
        #[diesel(sql_type = Text)]
        original_post_id: String,
        #[diesel(sql_type = Text)]
        owner: String,
        #[diesel(sql_type = Text)]
        profile_id: String,
        #[diesel(sql_type = BigInt)]
        created_at: i64,
    }
    let query = format!(
        "
        SELECT repost_id, original_post_id, owner, profile_id, created_at
        FROM (
            SELECT repost_id, original_post_id, owner, profile_id, created_at
            FROM reposts
            WHERE original_post_id = $1 AND is_original_post = true
            UNION ALL
            SELECT post_id AS repost_id, parent_post_id AS original_post_id, owner, profile_id, created_at
            FROM posts
            WHERE parent_post_id = $1 AND post_type = '{}' AND deleted_at IS NULL
        ) AS combined
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ",
        POST_TYPE_QUOTE_REPOST
    );
    let results = sql_query(&query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(|r| RepostRow {
            repost_id: r.repost_id,
            original_post_id: r.original_post_id,
            owner: r.owner,
            profile_id: r.profile_id,
            created_at: r.created_at,
        })
        .collect())
}

pub(crate) async fn list_post_transfers(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostTransfer>, SocialError> {
    let mut conn = db.connect().await?;
    let results = posts_transfers::table
        .filter(posts_transfers::object_id.eq(post_id))
        .order_by(posts_transfers::transferred_at.desc())
        .limit(limit)
        .offset(offset)
        .select(PostTransfer::as_select())
        .load::<PostTransfer>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_post_reports(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostReport>, SocialError> {
    let mut conn = db.connect().await?;
    let results = posts_reports::table
        .filter(posts_reports::object_id.eq(post_id))
        .filter(posts_reports::is_comment.eq(false))
        .order_by(posts_reports::time.desc())
        .limit(limit)
        .offset(offset)
        .select(PostReport::as_select())
        .load::<PostReport>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_post_moderation_events(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostModerationEventRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = posts_moderation_events::table
        .filter(posts_moderation_events::object_id.eq(post_id))
        .order_by(posts_moderation_events::time.desc())
        .limit(limit)
        .offset(offset)
        .select(PostModerationEventRow::as_select())
        .load::<PostModerationEventRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_post_deletion_events(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostDeletionEventRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = posts_deletion_events::table
        .filter(posts_deletion_events::object_id.eq(post_id))
        .order_by(posts_deletion_events::time.desc())
        .limit(limit)
        .offset(offset)
        .select(PostDeletionEventRow::as_select())
        .load::<PostDeletionEventRow>(&mut conn)
        .await?;
    Ok(results)
}
