// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::Profile;
use myso_indexer_alt_social_schema::schema::profiles;
use myso_pg_db::{Db, DbArgs};
use url::Url;

#[derive(Clone)]
pub struct Reader {
    db: Db,
}

impl Reader {
    pub async fn new(database_url: Url, db_args: DbArgs) -> Result<Self, anyhow::Error> {
        let db = Db::for_read(database_url, db_args).await?;
        let _ = db.connect().await?;
        Ok(Self { db })
    }

    pub async fn get_profiles(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Profile>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let results = profiles::table
            .order_by(profiles::id.desc())
            .limit(limit)
            .offset(offset)
            .select(Profile::as_select())
            .load::<Profile>(&mut conn)
            .await?;
        Ok(results)
    }

    pub async fn get_profile_count(&self) -> Result<i64, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let count: i64 = profiles::table.count().get_result(&mut conn).await?;
        Ok(count)
    }

    pub async fn get_profile_by_address(
        &self,
        address: &str,
    ) -> Result<Option<Profile>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = profiles::table
            .filter(profiles::owner_address.eq(address))
            .select(Profile::as_select())
            .first::<Profile>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }

    pub async fn get_profile_by_username(
        &self,
        username: &str,
    ) -> Result<Option<Profile>, crate::error::SocialError> {
        let mut conn = self.db.connect().await?;
        let result = profiles::table
            .filter(profiles::username.eq(username))
            .select(Profile::as_select())
            .first::<Profile>(&mut conn)
            .await
            .optional()?;
        Ok(result)
    }
}
