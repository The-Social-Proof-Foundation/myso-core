// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::Profile;
use myso_indexer_alt_social_schema::schema::profiles;

use crate::error::SocialError;
use myso_pg_db::Db;

pub(crate) async fn get_profiles(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<Profile>, SocialError> {
    let mut conn = db.connect().await?;
    let results = profiles::table
        .order_by(profiles::id.desc())
        .limit(limit)
        .offset(offset)
        .select(Profile::as_select())
        .load::<Profile>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_profile_count(db: &Db) -> Result<i64, SocialError> {
    let mut conn = db.connect().await?;
    let count: i64 = profiles::table.count().get_result(&mut conn).await?;
    Ok(count)
}

pub(crate) async fn get_profile_by_address(
    db: &Db,
    address: &str,
) -> Result<Option<Profile>, SocialError> {
    let mut conn = db.connect().await?;
    let result = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(Profile::as_select())
        .first::<Profile>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_profile_by_username(
    db: &Db,
    username: &str,
) -> Result<Option<Profile>, SocialError> {
    let mut conn = db.connect().await?;
    let result = profiles::table
        .filter(profiles::username.eq(username))
        .select(Profile::as_select())
        .first::<Profile>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}
