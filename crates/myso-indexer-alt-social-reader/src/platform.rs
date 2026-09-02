// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{
    Array, BigInt, Bool, Int4, Nullable, SmallInt, Text, Timestamp, Timestamptz,
};
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

/// Membership, block, moderator flags, and active permission list for a wallet on a platform.
#[derive(Debug, Clone, QueryableByName)]
pub struct PlatformUserAccessRow {
    #[diesel(sql_type = Bool)]
    pub is_member: bool,
    #[diesel(sql_type = Bool)]
    pub is_blocked: bool,
    #[diesel(sql_type = Bool)]
    pub is_moderator: bool,
    #[diesel(sql_type = Array<Text>)]
    pub moderator_permissions: Vec<String>,
}

impl PlatformUserAccessRow {
    pub fn permissions(&self) -> Vec<String> {
        self.moderator_permissions.clone()
    }

    pub fn can_block_users(&self) -> bool {
        self.permissions().iter().any(|p| {
            p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_BLOCK_ADMIN
        })
    }

    pub fn can_moderate_content(&self) -> bool {
        self.permissions().iter().any(|p| {
            p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_CONTENT_MODERATOR
        })
    }

    pub fn can_manage_badges(&self) -> bool {
        self.permissions().iter().any(|p| {
            p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_BADGE_ADMIN
        })
    }

    pub fn can_withdraw_from_platform_treasury(&self, is_developer: bool) -> bool {
        is_developer
            || self.permissions().iter().any(|p| {
                p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_TREASURY_ADMIN
            })
    }

    pub fn can_manage_promotions(&self) -> bool {
        self.permissions().iter().any(|p| {
            p == myso_indexer_alt_social_schema::platform_permissions::PLATFORM_PROMOTION_ADMIN
        })
    }
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
    #[diesel(sql_type = Nullable<Text>)]
    pub cover_photo: Option<String>,
    #[diesel(sql_type = Nullable<diesel::sql_types::Jsonb>)]
    pub media_previews: Option<JsonValue>,
    #[diesel(sql_type = Text)]
    pub developer_address: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub moderators_group_id: Option<String>,
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
    #[diesel(sql_type = Nullable<Text>)]
    pub redirect_uri: Option<String>,
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
        "SELECT p.platform_id, p.name, p.tagline, p.description, p.logo, p.cover_photo, p.media_previews,
                p.developer_address, p.moderators_group_id,
                p.status, p.is_approved, p.primary_category, p.secondary_category, p.created_at, p.updated_at,
                p.terms_of_service, p.privacy_policy, p.redirect_uri, p.links, p.platforms AS platform_names, p.release_date, p.shutdown_date,
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

#[derive(Debug, Clone, QueryableByName)]
pub struct PlatformTreasuryBalanceRow {
    #[diesel(sql_type = Text)]
    pub platform_id: String,
    #[diesel(sql_type = Text)]
    pub coin_type: String,
    #[diesel(sql_type = BigInt)]
    pub balance: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub last_funded_at: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub last_withdrawn_at: Option<i64>,
    #[diesel(sql_type = Timestamptz)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct PlatformTreasuryWithdrawalRow {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub platform_id: String,
    #[diesel(sql_type = Text)]
    pub recipient: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = SmallInt)]
    pub reason_code: i16,
    #[diesel(sql_type = Text)]
    pub executed_by: String,
    #[diesel(sql_type = BigInt)]
    pub timestamp: i64,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Nullable<Text>)]
    pub event_id: Option<String>,
    #[diesel(sql_type = Text)]
    pub coin_type: String,
}

pub(crate) async fn list_platform_treasury_balances(
    conn: &mut Connection<'_>,
    platform_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PlatformTreasuryBalanceRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = diesel::sql_query(
        "SELECT platform_id, coin_type, balance, last_funded_at, last_withdrawn_at, updated_at
         FROM platform_treasury_coin_balances
         WHERE platform_id = $1
         ORDER BY coin_type ASC",
    )
    .bind::<Text, _>(platform_id)
    .load::<PlatformTreasuryBalanceRow>(conn)
    .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn get_platform_treasury_balance(
    conn: &mut Connection<'_>,
    platform_id: &str,
    coin_type: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PlatformTreasuryBalanceRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let result = diesel::sql_query(
        "SELECT platform_id, coin_type, balance, last_funded_at, last_withdrawn_at, updated_at
         FROM platform_treasury_coin_balances
         WHERE platform_id = $1 AND coin_type = $2
         LIMIT 1",
    )
    .bind::<Text, _>(platform_id)
    .bind::<Text, _>(coin_type)
    .get_result::<PlatformTreasuryBalanceRow>(conn)
    .await
    .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_platform_treasury_withdrawals(
    conn: &mut Connection<'_>,
    platform_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PlatformTreasuryWithdrawalRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = diesel::sql_query(
        "SELECT id, platform_id, recipient, amount, reason_code, executed_by, timestamp, created_at, event_id, coin_type
         FROM platform_treasury_withdrawals
         WHERE platform_id = $1
         ORDER BY timestamp DESC
         LIMIT $2 OFFSET $3",
    )
    .bind::<Text, _>(platform_id)
    .bind::<BigInt, _>(limit)
    .bind::<BigInt, _>(offset)
    .load::<PlatformTreasuryWithdrawalRow>(conn)
    .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn get_platform_by_registry_id(
    conn: &mut Connection<'_>,
    registry_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PlatformRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let result = diesel::sql_query(
        "SELECT p.platform_id, p.name, p.tagline, p.description, p.logo, p.cover_photo, p.media_previews,
                p.developer_address, p.moderators_group_id,
                p.status, p.is_approved, p.primary_category, p.secondary_category, p.created_at, p.updated_at,
                p.terms_of_service, p.privacy_policy, p.redirect_uri, p.links, p.platforms AS platform_names, p.release_date, p.shutdown_date,
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
        SELECT p.platform_id, p.name, p.tagline, p.description, p.logo, p.cover_photo, p.media_previews,
               p.developer_address, p.moderators_group_id,
               p.status, p.is_approved, p.primary_category, p.secondary_category, p.created_at, p.updated_at,
               p.terms_of_service, p.privacy_policy, p.redirect_uri, p.links, p.platforms AS platform_names, p.release_date, p.shutdown_date,
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
          AND (left_at IS NULL OR joined_at > left_at)
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
    permission_filter: Option<&str>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PlatformModeratorRow>> {
    if let Some(filter) = permission_filter {
        if !myso_indexer_alt_social_schema::platform_permissions::is_valid_moderator_permission(
            filter,
        ) {
            anyhow::bail!("invalid platform moderator permission filter: {filter}");
        }
    }
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
        #[diesel(sql_type = Array<Text>)]
        permissions: Vec<String>,
    }
    let query = if permission_filter.is_some() {
        "
        SELECT
            m.moderator_address,
            m.added_by,
            m.created_at,
            COALESCE(
                array_agg(p.permission_type ORDER BY p.permission_type)
                    FILTER (WHERE p.revoked_at IS NULL),
                '{}'::text[]
            ) AS permissions
        FROM platform_moderators m
        INNER JOIN platform_moderator_permissions p
            ON p.platform_id = m.platform_id
           AND p.moderator_address = m.moderator_address
           AND p.revoked_at IS NULL
           AND p.permission_type = $4
        WHERE m.platform_id = $1
        GROUP BY m.moderator_address, m.added_by, m.created_at
        ORDER BY m.created_at DESC
        LIMIT $2 OFFSET $3
    "
    } else {
        "
        SELECT
            m.moderator_address,
            m.added_by,
            m.created_at,
            COALESCE(
                array_agg(p.permission_type ORDER BY p.permission_type)
                    FILTER (WHERE p.revoked_at IS NULL),
                '{}'::text[]
            ) AS permissions
        FROM platform_moderators m
        LEFT JOIN platform_moderator_permissions p
            ON p.platform_id = m.platform_id
           AND p.moderator_address = m.moderator_address
        WHERE m.platform_id = $1
        GROUP BY m.moderator_address, m.added_by, m.created_at
        ORDER BY m.created_at DESC
        LIMIT $2 OFFSET $3
    "
    };
    let rows = if let Some(filter) = permission_filter {
        diesel::sql_query(query)
            .bind::<Text, _>(platform_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .bind::<Text, _>(filter)
            .load::<Row>(conn)
            .await?
    } else {
        diesel::sql_query(query)
            .bind::<Text, _>(platform_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<Row>(conn)
            .await?
    };
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| PlatformModeratorRow {
            moderator_address: r.moderator_address,
            added_by: r.added_by,
            created_at: r.created_at,
            permissions: r.permissions,
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
                  AND (left_at IS NULL OR joined_at > left_at)
            ) AS is_member,
            EXISTS(
                SELECT 1 FROM platform_blocked_profiles
                WHERE platform_id = $1 AND wallet_address = $2
            ) AS is_blocked,
            (
                EXISTS(
                    SELECT 1 FROM platform_moderators
                    WHERE platform_id = $1 AND moderator_address = $2
                )
                OR EXISTS(
                    SELECT 1 FROM platforms
                    WHERE platform_id = $1 AND developer_address = $2
                )
            ) AS is_moderator,
            COALESCE((
                SELECT array_agg(DISTINCT p.permission_type ORDER BY p.permission_type)
                FROM platform_moderator_permissions p
                WHERE p.platform_id = $1
                  AND p.moderator_address = $2
                  AND p.revoked_at IS NULL
            ), '{}'::text[]) AS moderator_permissions",
    )
    .bind::<Text, _>(platform_id)
    .bind::<Text, _>(user_address)
    .get_result::<PlatformUserAccessRow>(conn)
    .await?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

#[derive(Debug, Clone, QueryableByName)]
pub struct PlatformConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub max_reasoning_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_cover_photo_url_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_media_previews: i64,
    #[diesel(sql_type = BigInt)]
    pub max_badge_name_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_badge_description_length: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Latest platform configuration.
pub(crate) async fn get_platform_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PlatformConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, max_reasoning_length, max_cover_photo_url_length, max_media_previews,
               max_badge_name_length, max_badge_description_length, version, updated_at, time,
               transaction_id
        FROM platform_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<PlatformConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}
