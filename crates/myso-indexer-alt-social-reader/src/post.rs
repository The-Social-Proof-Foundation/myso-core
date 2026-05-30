// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel::sql_types::{BigInt, Bool, Int4, Nullable, SmallInt, Text};
use diesel_async::RunQueryDsl;
use serde::Serialize;
use serde_json::Value as JsonValue;

use myso_indexer_alt_social_schema::models::POST_TYPE_QUOTE_REPOST;
use myso_indexer_alt_social_schema::models::PostDeletionEventRow;
use myso_indexer_alt_social_schema::models::PostModerationEventRow;
pub use myso_indexer_alt_social_schema::models::{CommentRow, ReactionRow, RepostRow, TipRow};
use myso_indexer_alt_social_schema::schema::posts_deletion_events;
use myso_indexer_alt_social_schema::schema::posts_moderation_events;
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName, Serialize)]
pub struct PostRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = Text)]
    pub profile_id: String,
    #[diesel(sql_type = Text)]
    pub content: String,
    #[diesel(sql_type = Text)]
    pub post_type: String,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub deleted_at: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub reaction_count: i64,
    #[diesel(sql_type = BigInt)]
    pub comment_count: i64,
    #[diesel(sql_type = BigInt)]
    pub repost_count: i64,
    #[diesel(sql_type = BigInt)]
    pub tips_received: i64,
    #[diesel(sql_type = Nullable<diesel::sql_types::Jsonb>)]
    pub media_urls: Option<JsonValue>,
    #[diesel(sql_type = Nullable<diesel::sql_types::Jsonb>)]
    pub mentions: Option<JsonValue>,
    #[diesel(sql_type = Nullable<Text>)]
    pub parent_post_id: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub updated_at: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub poc_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub revenue_redirect_to: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub revenue_redirect_percentage: Option<i64>,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub enable_poc: bool,
    #[diesel(sql_type = Nullable<Text>)]
    pub poc_reasoning: Option<String>,
    #[diesel(sql_type = Nullable<diesel::sql_types::Jsonb>)]
    pub poc_evidence_urls: Option<JsonValue>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub poc_similarity_score: Option<i64>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub poc_media_type: Option<i16>,
    #[diesel(sql_type = Nullable<Text>)]
    pub poc_oracle_address: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub poc_analyzed_at: Option<i64>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub poc_outcome: Option<i16>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub poc_redirection_kind: Option<i16>,
    #[diesel(sql_type = SmallInt)]
    pub poc_disputes_submitted: i16,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub enable_spt: bool,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub enable_spot: bool,
    #[diesel(sql_type = Nullable<Text>)]
    pub spot_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub spt_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub mydata_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub revenue_recipient: Option<String>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub requires_subscription: Option<bool>,
    #[diesel(sql_type = Nullable<Text>)]
    pub subscription_service_id: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub subscription_price: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub encrypted_content_hash: Option<String>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub removed_from_platform: Option<bool>,
    #[diesel(sql_type = Nullable<Text>)]
    pub removed_by: Option<String>,
    #[diesel(sql_type = Nullable<diesel::sql_types::Jsonb>)]
    pub metadata_json: Option<JsonValue>,
    #[diesel(sql_type = Nullable<Text>)]
    pub promotion_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_id: Option<String>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub permissions: Option<i16>,
    #[diesel(sql_type = Nullable<Text>)]
    pub actor_address: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub sub_agent_id: Option<String>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub action_identity_class: Option<i16>,
}

#[derive(Debug, Clone)]
pub struct PostTransferRow {
    pub object_id: String,
    pub previous_owner: String,
    pub new_owner: String,
    pub is_post: bool,
    pub transferred_at: i64,
    pub transaction_id: String,
}

/// One user report against a post or comment, as stored in `posts_reports`.
#[derive(Debug, Clone, QueryableByName)]
pub struct PostReportRow {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub object_id: String,
    #[diesel(sql_type = Bool)]
    pub is_comment: bool,
    #[diesel(sql_type = Text)]
    pub reporter: String,
    #[diesel(sql_type = SmallInt)]
    pub reason_code: i16,
    #[diesel(sql_type = Text)]
    pub description: String,
    #[diesel(sql_type = BigInt)]
    pub reported_at: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct PostConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub max_content_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_media_urls: i64,
    #[diesel(sql_type = BigInt)]
    pub max_mentions: i64,
    #[diesel(sql_type = BigInt)]
    pub max_metadata_size: i64,
    #[diesel(sql_type = BigInt)]
    pub max_description_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_reaction_length: i64,
    #[diesel(sql_type = BigInt)]
    pub commenter_tip_percentage: i64,
    #[diesel(sql_type = BigInt)]
    pub repost_tip_percentage: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
}

pub(crate) async fn get_post_by_id(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let result = diesel::sql_query(
        "SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
                reaction_count, comment_count, repost_count, tips_received,
                media_urls, mentions, parent_post_id, updated_at,
                poc_id, revenue_redirect_to, revenue_redirect_percentage, enable_poc,
                poc_reasoning, poc_evidence_urls, poc_similarity_score, poc_media_type,
                poc_oracle_address, poc_analyzed_at, poc_outcome, poc_redirection_kind,
                poc_disputes_submitted,
                enable_spt, enable_spot, spot_id, spt_id, mydata_id,
                revenue_recipient, requires_subscription, subscription_service_id, subscription_price,
                encrypted_content_hash, removed_from_platform, removed_by, metadata_json, promotion_id,
                platform_id, permissions
         FROM posts
         WHERE (post_id = $1 OR id = $1) AND deleted_at IS NULL
         ORDER BY created_at DESC
         LIMIT 1",
    )
    .bind::<Text, _>(post_id)
    .get_result::<PostRow>(conn)
    .await
    .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_comment_by_id(
    conn: &mut Connection<'_>,
    comment_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<CommentRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        comment_id: String,
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = Nullable<Text>)]
        parent_comment_id: Option<String>,
        #[diesel(sql_type = Text)]
        owner: String,
        #[diesel(sql_type = Text)]
        profile_id: String,
        #[diesel(sql_type = Text)]
        content: String,
        #[diesel(sql_type = BigInt)]
        created_at: i64,
        #[diesel(sql_type = Nullable<BigInt>)]
        reaction_count: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        comment_count: Option<i64>,
        #[diesel(sql_type = Nullable<Text>)]
        actor_address: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        sub_agent_id: Option<String>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        action_identity_class: Option<i16>,
    }
    let query = "
        SELECT comment_id, post_id, parent_comment_id, owner, profile_id, content, created_at,
               reaction_count, comment_count, actor_address, sub_agent_id, action_identity_class
        FROM comments
        WHERE (comment_id = $1 OR id = $1) AND deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(comment_id)
        .get_result::<Row>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result.map(|r| CommentRow {
        comment_id: r.comment_id,
        post_id: r.post_id,
        parent_comment_id: r.parent_comment_id,
        owner: r.owner,
        profile_id: r.profile_id,
        content: r.content,
        created_at: r.created_at,
        reaction_count: r.reaction_count.unwrap_or(0),
        comment_count: r.comment_count.unwrap_or(0),
        actor_address: r.actor_address,
        sub_agent_id: r.sub_agent_id,
        action_identity_class: r.action_identity_class,
    }))
}

pub(crate) async fn list_posts(
    conn: &mut Connection<'_>,
    owner: Option<&str>,
    post_type: Option<&str>,
    sub_agent_id: Option<&str>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
               reaction_count, comment_count, repost_count, tips_received,
               media_urls, mentions, parent_post_id, updated_at,
               poc_id, revenue_redirect_to, revenue_redirect_percentage, enable_poc,
               poc_reasoning, poc_evidence_urls, poc_similarity_score, poc_media_type,
               poc_oracle_address, poc_analyzed_at, poc_outcome, poc_redirection_kind,
               poc_disputes_submitted,
               enable_spt, enable_spot, spot_id, spt_id, mydata_id,
               revenue_recipient, requires_subscription, subscription_service_id, subscription_price,
               encrypted_content_hash, removed_from_platform, removed_by, metadata_json, promotion_id,
               platform_id, permissions, actor_address, sub_agent_id, action_identity_class
        FROM posts
        WHERE deleted_at IS NULL
        AND ($1::TEXT IS NULL OR owner = $1)
        AND ($2::TEXT IS NULL OR post_type = $2)
        AND ($5::TEXT IS NULL OR sub_agent_id = $5)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
    ";
    let results = diesel::sql_query(query)
        .bind::<Nullable<Text>, _>(owner)
        .bind::<Nullable<Text>, _>(post_type)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .bind::<Nullable<Text>, _>(sub_agent_id)
        .load::<PostRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

/// Posts for a profile overview: `owner = profile owner` **or** `profile_id = linked object id`
/// (matches REST `/profiles/:address/posts` scope).
pub(crate) async fn list_posts_for_profile(
    conn: &mut Connection<'_>,
    owner_address: &str,
    profile_id: Option<&str>,
    post_type: Option<&str>,
    enable_poc: Option<bool>,
    poc_outcomes: Option<&[i16]>,
    include_removed: bool,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let deleted_sql = if include_removed {
        ""
    } else {
        "AND deleted_at IS NULL"
    };
    let enable_sql = match enable_poc {
        Some(true) => "AND enable_poc = TRUE",
        Some(false) => "AND enable_poc = FALSE",
        None => "",
    };
    let outcomes_sql = match poc_outcomes {
        Some(v) if !v.is_empty() => {
            let elems: Vec<String> = v.iter().map(i16::to_string).collect();
            format!(
                "AND poc_outcome = ANY(ARRAY[{}]::smallint[])",
                elems.join(",")
            )
        }
        _ => String::new(),
    };
    let query = format!(
        "
        SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
               reaction_count, comment_count, repost_count, tips_received,
               media_urls, mentions, parent_post_id, updated_at,
               poc_id, revenue_redirect_to, revenue_redirect_percentage, enable_poc,
               poc_reasoning, poc_evidence_urls, poc_similarity_score, poc_media_type,
               poc_oracle_address, poc_analyzed_at, poc_outcome, poc_redirection_kind,
               poc_disputes_submitted,
               enable_spt, enable_spot, spot_id, spt_id, mydata_id,
               revenue_recipient, requires_subscription, subscription_service_id, subscription_price,
               encrypted_content_hash, removed_from_platform, removed_by, metadata_json, promotion_id,
               platform_id, permissions, actor_address, sub_agent_id, action_identity_class
        FROM (
            SELECT DISTINCT ON (post_id) *
            FROM posts
            WHERE (owner = $1 OR ($2::TEXT IS NOT NULL AND profile_id = $2))
            AND ($3::TEXT IS NULL OR post_type = $3)
            {deleted_sql}
            {enable_sql}
            {outcomes_sql}
            ORDER BY post_id, time DESC
        ) sub
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        ",
        deleted_sql = deleted_sql,
        enable_sql = enable_sql,
        outcomes_sql = outcomes_sql,
    );
    let results = diesel::sql_query(query)
        .bind::<Text, _>(owner_address)
        .bind::<Nullable<Text>, _>(profile_id)
        .bind::<Nullable<Text>, _>(post_type)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PostRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn count_posts_for_profile(
    conn: &mut Connection<'_>,
    owner_address: &str,
    profile_id: Option<&str>,
    post_type: Option<&str>,
    enable_poc: Option<bool>,
    poc_outcomes: Option<&[i16]>,
    include_removed: bool,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<i64> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let deleted_sql = if include_removed {
        ""
    } else {
        "AND deleted_at IS NULL"
    };
    let enable_sql = match enable_poc {
        Some(true) => "AND enable_poc = TRUE",
        Some(false) => "AND enable_poc = FALSE",
        None => "",
    };
    let outcomes_sql = match poc_outcomes {
        Some(v) if !v.is_empty() => {
            let elems: Vec<String> = v.iter().map(i16::to_string).collect();
            format!(
                "AND poc_outcome = ANY(ARRAY[{}]::smallint[])",
                elems.join(",")
            )
        }
        _ => String::new(),
    };
    let query = format!(
        "
        SELECT COUNT(DISTINCT post_id)::bigint AS cnt
        FROM posts
        WHERE (owner = $1 OR ($2::TEXT IS NOT NULL AND profile_id = $2))
        AND ($3::TEXT IS NULL OR post_type = $3)
        {deleted_sql}
        {enable_sql}
        {outcomes_sql}
        ",
        deleted_sql = deleted_sql,
        enable_sql = enable_sql,
        outcomes_sql = outcomes_sql,
    );
    #[derive(QueryableByName)]
    struct Cnt {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
    }
    let row = diesel::sql_query(query)
        .bind::<Text, _>(owner_address)
        .bind::<Nullable<Text>, _>(profile_id)
        .bind::<Nullable<Text>, _>(post_type)
        .get_result::<Cnt>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(row.cnt)
}

pub(crate) async fn get_post_comments(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<CommentRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        comment_id: String,
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = Nullable<Text>)]
        parent_comment_id: Option<String>,
        #[diesel(sql_type = Text)]
        owner: String,
        #[diesel(sql_type = Text)]
        profile_id: String,
        #[diesel(sql_type = Text)]
        content: String,
        #[diesel(sql_type = BigInt)]
        created_at: i64,
        #[diesel(sql_type = Nullable<BigInt>)]
        reaction_count: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        comment_count: Option<i64>,
        #[diesel(sql_type = Nullable<Text>)]
        actor_address: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        sub_agent_id: Option<String>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        action_identity_class: Option<i16>,
    }
    let query = "
        SELECT comment_id, post_id, parent_comment_id, owner, profile_id, content, created_at,
               reaction_count, comment_count, actor_address, sub_agent_id, action_identity_class
        FROM comments
        WHERE post_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| CommentRow {
            comment_id: r.comment_id,
            post_id: r.post_id,
            parent_comment_id: r.parent_comment_id,
            owner: r.owner,
            profile_id: r.profile_id,
            content: r.content,
            created_at: r.created_at,
            reaction_count: r.reaction_count.unwrap_or(0),
            comment_count: r.comment_count.unwrap_or(0),
            actor_address: r.actor_address,
            sub_agent_id: r.sub_agent_id,
            action_identity_class: r.action_identity_class,
        })
        .collect())
}

pub(crate) async fn get_post_reactions(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ReactionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        user_address: String,
        #[diesel(sql_type = Text)]
        reaction_text: String,
        #[diesel(sql_type = BigInt)]
        created_at: i64,
        #[diesel(sql_type = Nullable<Text>)]
        principal_owner: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        actor_address: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        sub_agent_id: Option<String>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        action_identity_class: Option<i16>,
    }
    let query = "
        SELECT user_address, reaction_text, created_at, principal_owner, actor_address,
               sub_agent_id, action_identity_class
        FROM reactions
        WHERE object_id = $1 AND is_post = true
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| ReactionRow {
            user_address: r.user_address,
            reaction_text: r.reaction_text,
            created_at: r.created_at,
            principal_owner: r.principal_owner,
            actor_address: r.actor_address,
            sub_agent_id: r.sub_agent_id,
            action_identity_class: r.action_identity_class,
        })
        .collect())
}

pub(crate) async fn get_post_reposts(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<RepostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
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
        #[diesel(sql_type = Nullable<Text>)]
        actor_address: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        sub_agent_id: Option<String>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        action_identity_class: Option<i16>,
    }
    let query = format!(
        "
        SELECT repost_id, original_post_id, owner, profile_id, created_at,
               actor_address, sub_agent_id, action_identity_class
        FROM (
            SELECT repost_id, original_post_id, owner, profile_id, created_at,
                   actor_address, sub_agent_id, action_identity_class
            FROM reposts
            WHERE original_post_id = $1 AND is_original_post = true
            UNION ALL
            SELECT post_id AS repost_id, parent_post_id AS original_post_id, owner, profile_id, created_at,
                   actor_address, sub_agent_id, action_identity_class
            FROM posts
            WHERE parent_post_id = $1 AND post_type = '{}' AND deleted_at IS NULL
        ) AS combined
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ",
        POST_TYPE_QUOTE_REPOST
    );
    let rows = diesel::sql_query(&query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| RepostRow {
            repost_id: r.repost_id,
            original_post_id: r.original_post_id,
            owner: r.owner,
            profile_id: r.profile_id,
            created_at: r.created_at,
            actor_address: r.actor_address,
            sub_agent_id: r.sub_agent_id,
            action_identity_class: r.action_identity_class,
        })
        .collect())
}

pub(crate) async fn get_post_tips(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<TipRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        tipper: String,
        #[diesel(sql_type = Text)]
        recipient: String,
        #[diesel(sql_type = BigInt)]
        amount: i64,
        #[diesel(sql_type = BigInt)]
        created_at: i64,
    }
    let query = "
        SELECT tipper, recipient, amount, created_at
        FROM tips
        WHERE object_id = $1 AND is_post = true
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| TipRow {
            tipper: r.tipper,
            recipient: r.recipient,
            amount: r.amount,
            created_at: r.created_at,
        })
        .collect())
}

pub(crate) async fn get_post_transfers(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostTransferRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        object_id: String,
        #[diesel(sql_type = Text)]
        previous_owner: String,
        #[diesel(sql_type = Text)]
        new_owner: String,
        #[diesel(sql_type = diesel::sql_types::Bool)]
        is_post: bool,
        #[diesel(sql_type = BigInt)]
        transferred_at: i64,
        #[diesel(sql_type = Text)]
        transaction_id: String,
    }
    let query = "
        SELECT object_id, previous_owner, new_owner, is_post, transferred_at, transaction_id
        FROM posts_transfers
        WHERE object_id = $1
        ORDER BY transferred_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| PostTransferRow {
            object_id: r.object_id,
            previous_owner: r.previous_owner,
            new_owner: r.new_owner,
            is_post: r.is_post,
            transferred_at: r.transferred_at,
            transaction_id: r.transaction_id,
        })
        .collect())
}

/// User reports for a post (`is_comment = false`); `object_id` is the post ID.
pub(crate) async fn list_post_reports(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostReportRow>> {
    list_object_reports(conn, post_id, false, limit, offset, metrics).await
}

/// User reports for a comment (`is_comment = true`); `object_id` is the comment ID.
pub(crate) async fn list_comment_reports(
    conn: &mut Connection<'_>,
    comment_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostReportRow>> {
    list_object_reports(conn, comment_id, true, limit, offset, metrics).await
}

async fn list_object_reports(
    conn: &mut Connection<'_>,
    object_id: &str,
    is_comment: bool,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostReportRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT id, object_id, is_comment, reporter, reason_code, description, reported_at, transaction_id
        FROM posts_reports
        WHERE object_id = $1 AND is_comment = $2
        ORDER BY time DESC
        LIMIT $3 OFFSET $4
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(object_id)
        .bind::<Bool, _>(is_comment)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PostReportRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

/// Platform moderation events for a post (`posts_moderation_events.object_id` = post id).
pub(crate) async fn list_post_moderation_events(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostModerationEventRow>> {
    list_moderation_events_by_object_id(conn, post_id, limit, offset, metrics).await
}

/// Platform moderation events for a comment (same table; `object_id` = comment id).
pub(crate) async fn list_comment_moderation_events(
    conn: &mut Connection<'_>,
    comment_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostModerationEventRow>> {
    list_moderation_events_by_object_id(conn, comment_id, limit, offset, metrics).await
}

async fn list_moderation_events_by_object_id(
    conn: &mut Connection<'_>,
    object_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostModerationEventRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let results = posts_moderation_events::table
        .filter(posts_moderation_events::object_id.eq(object_id))
        .order_by(posts_moderation_events::time.desc())
        .limit(limit)
        .offset(offset)
        .select(PostModerationEventRow::as_select())
        .load::<PostModerationEventRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

/// Deletion events for a post (`posts_deletion_events.object_id` = post id).
pub(crate) async fn list_post_deletion_events(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostDeletionEventRow>> {
    list_deletion_events_by_object_id(conn, post_id, limit, offset, metrics).await
}

/// Deletion events for a comment (same table; `object_id` = comment id).
pub(crate) async fn list_comment_deletion_events(
    conn: &mut Connection<'_>,
    comment_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostDeletionEventRow>> {
    list_deletion_events_by_object_id(conn, comment_id, limit, offset, metrics).await
}

async fn list_deletion_events_by_object_id(
    conn: &mut Connection<'_>,
    object_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostDeletionEventRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let results = posts_deletion_events::table
        .filter(posts_deletion_events::object_id.eq(object_id))
        .order_by(posts_deletion_events::time.desc())
        .limit(limit)
        .offset(offset)
        .select(PostDeletionEventRow::as_select())
        .load::<PostDeletionEventRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_post_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PostConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, max_content_length, max_media_urls, max_mentions, max_metadata_size,
               max_description_length, max_reaction_length, commenter_tip_percentage,
               repost_tip_percentage, version, updated_at
        FROM post_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<PostConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}
