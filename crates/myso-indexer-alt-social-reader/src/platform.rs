// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text, Timestamp};
use diesel_async::RunQueryDsl;
use serde_json::Value as JsonValue;

use myso_indexer_alt_social_schema::models::{PlatformMemberRow, PlatformModeratorRow};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone)]
pub struct PlatformBlockedProfileRow {
    pub wallet_address: String,
    pub blocked_by: String,
    pub created_at: NaiveDateTime,
}

/// Membership, block, and moderator flags for a wallet on a platform (one DB round-trip).
#[derive(Debug, Clone, QueryableByName)]
pub struct PlatformUserAccessRow {
    #[diesel(sql_type = Bool)]
    pub is_member: bool,
    #[diesel(sql_type = Bool)]
    pub is_blocked: bool,
    #[diesel(sql_type = Bool)]
    pub is_moderator: bool,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct PlatformRow {
    #[diesel(sql_type = Text)]
    pub platform_id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub tagline: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub description: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub logo: Option<String>,
    #[diesel(sql_type = Text)]
    pub developer_address: String,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
    #[diesel(sql_type = Bool)]
    pub is_approved: bool,
    #[diesel(sql_type = Text)]
    pub primary_category: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub secondary_category: Option<String>,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,
    #[diesel(sql_type = Nullable<Text>)]
    pub terms_of_service: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub privacy_policy: Option<String>,
    #[diesel(sql_type = Nullable<diesel::sql_types::Jsonb>)]
    pub links: Option<JsonValue>,
    #[diesel(sql_type = Nullable<diesel::sql_types::Jsonb>)]
    pub platform_names: Option<JsonValue>,
    #[diesel(sql_type = Nullable<Text>)]
    pub release_date: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub shutdown_date: Option<String>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub wants_dao_governance: Option<bool>,
    #[diesel(sql_type = Nullable<Text>)]
    pub governance_registry_id: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub delegate_count: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub delegate_term_epochs: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub max_votes_per_user: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub proposal_submission_cost: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub quadratic_base_cost: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub quorum_votes: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub voting_period_epochs: Option<i64>,
}

pub(crate) async fn get_platform_by_id(
    conn: &mut Connection<'_>,
    platform_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PlatformRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let result = diesel::sql_query(
        "SELECT p.platform_id, p.name, p.tagline, p.description, p.logo, p.developer_address,
                p.status, p.is_approved, p.primary_category, p.secondary_category, p.created_at, p.updated_at,
                p.terms_of_service, p.privacy_policy, p.links, p.platforms AS platform_names, p.release_date, p.shutdown_date,
                p.wants_dao_governance, p.governance_registry_id, p.delegate_count,
                p.delegate_term_epochs, p.max_votes_per_user,
                p.proposal_submission_cost, p.quadratic_base_cost, p.quorum_votes, p.voting_period_epochs
         FROM platforms p
         WHERE p.platform_id = $1 AND p.deleted_at IS NULL
         LIMIT 1",
    )
    .bind::<Text, _>(platform_id)
    .get_result::<PlatformRow>(conn)
    .await
    .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_platform_by_registry_id(
    conn: &mut Connection<'_>,
    registry_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PlatformRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let result = diesel::sql_query(
        "SELECT p.platform_id, p.name, p.tagline, p.description, p.logo, p.developer_address,
                p.status, p.is_approved, p.primary_category, p.secondary_category, p.created_at, p.updated_at,
                p.terms_of_service, p.privacy_policy, p.links, p.platforms AS platform_names, p.release_date, p.shutdown_date,
                p.wants_dao_governance, p.governance_registry_id, p.delegate_count,
                p.delegate_term_epochs, p.max_votes_per_user,
                p.proposal_submission_cost, p.quadratic_base_cost, p.quorum_votes, p.voting_period_epochs
         FROM platforms p
         WHERE p.governance_registry_id = $1 AND p.deleted_at IS NULL
         LIMIT 1",
    )
    .bind::<Text, _>(registry_id)
    .get_result::<PlatformRow>(conn)
    .await
    .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_platforms(
    conn: &mut Connection<'_>,
    approved_only: bool,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PlatformRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT p.platform_id, p.name, p.tagline, p.description, p.logo, p.developer_address,
               p.status, p.is_approved, p.primary_category, p.secondary_category, p.created_at, p.updated_at,
               p.terms_of_service, p.privacy_policy, p.links, p.platforms AS platform_names, p.release_date, p.shutdown_date,
               p.wants_dao_governance, p.governance_registry_id, p.delegate_count,
               p.delegate_term_epochs, p.max_votes_per_user,
               p.proposal_submission_cost, p.quadratic_base_cost, p.quorum_votes, p.voting_period_epochs
        FROM platforms p
        WHERE p.deleted_at IS NULL
        AND ($1::BOOL = FALSE OR p.is_approved = TRUE)
        ORDER BY p.created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Bool, _>(approved_only)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PlatformRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_platform_blocked_profiles(
    conn: &mut Connection<'_>,
    platform_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PlatformBlockedProfileRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        wallet_address: String,
        #[diesel(sql_type = Text)]
        blocked_by: String,
        #[diesel(sql_type = Timestamp)]
        created_at: NaiveDateTime,
    }
    let query = "
        SELECT wallet_address, blocked_by, created_at
        FROM platform_blocked_profiles
        WHERE platform_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(platform_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| PlatformBlockedProfileRow {
            wallet_address: r.wallet_address,
            blocked_by: r.blocked_by,
            created_at: r.created_at,
        })
        .collect())
}

pub(crate) async fn get_platform_members(
    conn: &mut Connection<'_>,
    platform_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PlatformMemberRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        wallet_address: String,
        #[diesel(sql_type = Timestamp)]
        joined_at: NaiveDateTime,
    }
    let query = "
        SELECT wallet_address, joined_at
        FROM platform_memberships
        WHERE platform_id = $1
        ORDER BY joined_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(platform_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| PlatformMemberRow {
            wallet_address: r.wallet_address,
            joined_at: r.joined_at,
        })
        .collect())
}

pub(crate) async fn get_platform_moderators(
    conn: &mut Connection<'_>,
    platform_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PlatformModeratorRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        moderator_address: String,
        #[diesel(sql_type = Text)]
        added_by: String,
        #[diesel(sql_type = Timestamp)]
        created_at: NaiveDateTime,
    }
    let query = "
        SELECT moderator_address, added_by, created_at
        FROM platform_moderators
        WHERE platform_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(platform_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| PlatformModeratorRow {
            moderator_address: r.moderator_address,
            added_by: r.added_by,
            created_at: r.created_at,
        })
        .collect())
}

pub(crate) async fn get_platform_user_access(
    conn: &mut Connection<'_>,
    platform_id: &str,
    user_address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<PlatformUserAccessRow> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = diesel::sql_query(
        "SELECT
            EXISTS(
                SELECT 1 FROM platform_memberships
                WHERE platform_id = $1 AND wallet_address = $2
            ) AS is_member,
            EXISTS(
                SELECT 1 FROM platform_blocked_profiles
                WHERE platform_id = $1 AND wallet_address = $2
            ) AS is_blocked,
            EXISTS(
                SELECT 1 FROM platform_moderators
                WHERE platform_id = $1 AND moderator_address = $2
            ) AS is_moderator",
    )
    .bind::<Text, _>(platform_id)
    .bind::<Text, _>(user_address)
    .get_result::<PlatformUserAccessRow>(conn)
    .await?;
    metrics.requests_succeeded.inc();
    Ok(row)
}
