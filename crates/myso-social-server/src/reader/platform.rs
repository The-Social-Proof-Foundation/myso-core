// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::Platform;
use myso_indexer_alt_social_schema::schema::{
    platform_blocked_profiles, platform_events, platform_memberships, platforms,
};
use serde_json::Value as JsonValue;

use crate::error::SocialError;
use crate::reader::types::{
    PlatformApprovalRow, PlatformBlockedProfileRow, PlatformConfigInfo, PlatformEventRow,
    PlatformMemberRow, PlatformModeratorRow, PlatformRow, PlatformUserAccessRow,
};
use myso_pg_db::Db;

async fn require_active_platform(db: &Db, platform_id: &str) -> Result<(), SocialError> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    let mut conn = db.connect().await?;
    let visible: i64 = platforms::table
        .filter(platforms::platform_id.eq(platform_id))
        .filter(platforms::deleted_at.is_null())
        .count()
        .get_result(&mut conn)
        .await?;
    if visible > 0 {
        return Ok(());
    }
    Err(SocialError::not_found(format!(
        "Platform '{}'",
        platform_id
    )))
}

pub(crate) async fn list_platforms(
    db: &Db,
    approved_only: bool,
    governance: Option<bool>,
    limit: i64,
    offset: i64,
) -> Result<Vec<PlatformRow>, SocialError> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    let mut conn = db.connect().await?;
    let mut query = platforms::table
        .filter(platforms::deleted_at.is_null())
        .into_boxed();
    if approved_only {
        query = query.filter(platforms::is_approved.eq(true));
    }
    if let Some(wg) = governance {
        if wg {
            query = query.filter(platforms::governance_registry_id.is_not_null());
        } else {
            query = query.filter(platforms::governance_registry_id.is_null());
        }
    }
    let results: Vec<Platform> = query
        .order_by(platforms::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select(Platform::as_select())
        .load(&mut conn)
        .await?;
    Ok(results.into_iter().map(PlatformRow::from).collect())
}

pub(crate) async fn get_platform_by_id(
    db: &Db,
    platform_id: &str,
) -> Result<Option<PlatformRow>, SocialError> {
    use diesel::ExpressionMethods;
    use diesel::OptionalExtension;
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    let mut conn = db.connect().await?;
    let result: Option<Platform> = platforms::table
        .filter(platforms::platform_id.eq(platform_id))
        .filter(platforms::deleted_at.is_null())
        .select(Platform::as_select())
        .first(&mut conn)
        .await
        .optional()?;
    Ok(result.map(PlatformRow::from))
}

pub(crate) async fn get_platform_moderators(
    db: &Db,
    platform_id: &str,
    permission_filter: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<PlatformModeratorRow>, SocialError> {
    if let Some(filter) = permission_filter {
        if !myso_indexer_alt_social_schema::platform_permissions::is_valid_moderator_permission(
            filter,
        ) {
            return Err(SocialError::bad_request(format!(
                "invalid platform moderator permission filter: {filter}"
            )));
        }
    }
    require_active_platform(db, platform_id).await?;
    let mut conn = db.connect().await?;
    #[derive(diesel::QueryableByName)]
    struct Row {
        #[diesel(sql_type = diesel::sql_types::Text)]
        moderator_address: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        added_by: String,
        #[diesel(sql_type = diesel::sql_types::Timestamp)]
        created_at: chrono::NaiveDateTime,
        #[diesel(sql_type = diesel::sql_types::Jsonb)]
        permissions: JsonValue,
    }
    let query = if permission_filter.is_some() {
        "
        SELECT
            m.moderator_address,
            m.added_by,
            m.created_at,
            COALESCE(
                json_agg(p.permission_type ORDER BY p.permission_type)
                    FILTER (WHERE p.revoked_at IS NULL),
                '[]'::json
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
                json_agg(p.permission_type ORDER BY p.permission_type)
                    FILTER (WHERE p.revoked_at IS NULL),
                '[]'::json
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
            .bind::<diesel::sql_types::Text, _>(platform_id)
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .bind::<diesel::sql_types::BigInt, _>(offset)
            .bind::<diesel::sql_types::Text, _>(filter)
            .load::<Row>(&mut conn)
            .await?
    } else {
        diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(platform_id)
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .bind::<diesel::sql_types::BigInt, _>(offset)
            .load::<Row>(&mut conn)
            .await?
    };
    Ok(rows
        .into_iter()
        .map(|r| PlatformModeratorRow {
            moderator_address: r.moderator_address,
            added_by: r.added_by,
            created_at: r.created_at,
            permissions: serde_json::from_value(r.permissions).unwrap_or_default(),
        })
        .collect())
}

pub(crate) async fn get_platform_user_access(
    db: &Db,
    platform_id: &str,
    user_address: &str,
) -> Result<PlatformUserAccessRow, SocialError> {
    require_active_platform(db, platform_id).await?;
    let mut conn = db.connect().await?;
    #[derive(diesel::QueryableByName)]
    struct Row {
        #[diesel(sql_type = diesel::sql_types::Bool)]
        is_member: bool,
        #[diesel(sql_type = diesel::sql_types::Bool)]
        is_blocked: bool,
        #[diesel(sql_type = diesel::sql_types::Bool)]
        is_moderator: bool,
        #[diesel(sql_type = diesel::sql_types::Jsonb)]
        moderator_permissions: JsonValue,
    }
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
                SELECT json_agg(DISTINCT p.permission_type ORDER BY p.permission_type)
                FROM platform_moderator_permissions p
                WHERE p.platform_id = $1
                  AND p.moderator_address = $2
                  AND p.revoked_at IS NULL
            ), '[]'::json) AS moderator_permissions",
    )
    .bind::<diesel::sql_types::Text, _>(platform_id)
    .bind::<diesel::sql_types::Text, _>(user_address)
    .get_result::<Row>(&mut conn)
    .await?;
    Ok(PlatformUserAccessRow::from_db(
        row.is_member,
        row.is_blocked,
        row.is_moderator,
        row.moderator_permissions,
    ))
}

pub(crate) async fn get_platform_approval(
    db: &Db,
    platform_id: &str,
) -> Result<Option<PlatformApprovalRow>, SocialError> {
    use diesel::ExpressionMethods;
    use diesel::OptionalExtension;
    use diesel::QueryDsl;
    let mut conn = db.connect().await?;
    let row = platforms::table
        .filter(platforms::platform_id.eq(platform_id))
        .filter(platforms::deleted_at.is_null())
        .select((
            platforms::is_approved,
            platforms::approval_changed_at,
            platforms::approved_by,
        ))
        .first::<(bool, Option<chrono::NaiveDateTime>, Option<String>)>(&mut conn)
        .await
        .optional()?;
    Ok(row.map(
        |(is_approved, approval_changed_at, approved_by)| PlatformApprovalRow {
            is_approved,
            approval_changed_at,
            approved_by,
        },
    ))
}

pub(crate) async fn get_platform_blocked_profiles(
    db: &Db,
    platform_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PlatformBlockedProfileRow>, SocialError> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    require_active_platform(db, platform_id).await?;
    let mut conn = db.connect().await?;
    let results = platform_blocked_profiles::table
        .filter(platform_blocked_profiles::platform_id.eq(platform_id))
        .order_by(platform_blocked_profiles::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            platform_blocked_profiles::wallet_address,
            platform_blocked_profiles::blocked_by,
            platform_blocked_profiles::created_at,
        ))
        .load::<(String, String, chrono::NaiveDateTime)>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(wallet_address, blocked_by, created_at)| PlatformBlockedProfileRow {
                wallet_address,
                blocked_by,
                created_at,
            },
        )
        .collect())
}

pub(crate) async fn get_platform_members(
    db: &Db,
    platform_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PlatformMemberRow>, SocialError> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    require_active_platform(db, platform_id).await?;
    let mut conn = db.connect().await?;
    let results = platform_memberships::table
        .filter(platform_memberships::platform_id.eq(platform_id))
        .order_by(platform_memberships::joined_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            platform_memberships::wallet_address,
            platform_memberships::joined_at,
        ))
        .load::<(String, chrono::NaiveDateTime)>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(|(wallet_address, joined_at)| PlatformMemberRow {
            wallet_address,
            joined_at,
        })
        .collect())
}

pub(crate) async fn check_platform_membership(
    db: &Db,
    platform_id: &str,
    profile_address: &str,
) -> Result<bool, SocialError> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    require_active_platform(db, platform_id).await?;
    let mut conn = db.connect().await?;
    let count: i64 = platform_memberships::table
        .filter(platform_memberships::platform_id.eq(platform_id))
        .filter(platform_memberships::wallet_address.eq(profile_address))
        .count()
        .get_result(&mut conn)
        .await?;
    Ok(count > 0)
}

pub(crate) async fn get_platform_events(
    db: &Db,
    platform_id: &str,
    limit: i64,
    offset: i64,
) -> Result<(Vec<PlatformEventRow>, i64), SocialError> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    require_active_platform(db, platform_id).await?;
    let mut conn = db.connect().await?;
    let total: i64 = platform_events::table
        .filter(platform_events::platform_id.eq(platform_id))
        .count()
        .get_result(&mut conn)
        .await?;
    let results = platform_events::table
        .filter(platform_events::platform_id.eq(platform_id))
        .order_by(platform_events::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            platform_events::platform_id,
            platform_events::event_type,
            platform_events::event_data,
            platform_events::event_id,
            platform_events::created_at,
            platform_events::reasoning,
        ))
        .load::<(
            String,
            String,
            JsonValue,
            Option<String>,
            chrono::NaiveDateTime,
            Option<String>,
        )>(&mut conn)
        .await?;
    let events = results
        .into_iter()
        .map(
            |(platform_id, event_type, event_data, event_id, created_at, reasoning)| {
                PlatformEventRow {
                    platform_id,
                    event_type,
                    event_data,
                    event_id,
                    created_at,
                    reasoning,
                }
            },
        )
        .collect();
    Ok((events, total))
}

pub(crate) async fn get_platform_configuration(
    db: &Db,
) -> Result<Option<PlatformConfigInfo>, SocialError> {
    use diesel::OptionalExtension;
    let mut conn = db.connect().await?;
    let query = "
        SELECT updated_by, max_reasoning_length, max_cover_photo_url_length, max_media_previews,
               max_media_preview_url_length, max_badge_name_length, max_badge_description_length,
               max_badge_media_url_length, max_badge_icon_url_length, version, updated_at
        FROM platform_config
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<PlatformConfigInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}
