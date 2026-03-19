// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::Platform;
use myso_indexer_alt_social_schema::schema::{
    platform_blocked_profiles, platform_events, platform_memberships, platform_moderators,
    platforms,
};

use crate::error::SocialError;
use crate::reader::types::{
    PlatformApprovalRow, PlatformBlockedProfileRow, PlatformEventRow, PlatformMemberRow,
    PlatformModeratorRow, PlatformRow,
};
use myso_pg_db::Db;

pub(crate) async fn list_platforms(
    db: &Db,
    approved_only: bool,
    governance: Option<bool>,
    limit: i64,
    offset: i64,
) -> Result<Vec<PlatformRow>, SocialError> {
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
    limit: i64,
    offset: i64,
) -> Result<Vec<PlatformModeratorRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = platform_moderators::table
        .filter(platform_moderators::platform_id.eq(platform_id))
        .order_by(platform_moderators::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            platform_moderators::moderator_address,
            platform_moderators::added_by,
            platform_moderators::created_at,
        ))
        .load::<(String, String, chrono::NaiveDateTime)>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(moderator_address, added_by, created_at)| PlatformModeratorRow {
                moderator_address,
                added_by,
                created_at,
            },
        )
        .collect())
}

pub(crate) async fn get_platform_approval(
    db: &Db,
    platform_id: &str,
) -> Result<Option<PlatformApprovalRow>, SocialError> {
    let mut conn = db.connect().await?;
    let result = platforms::table
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
    Ok(result.map(
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
            serde_json::Value,
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
