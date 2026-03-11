// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::{ObjectMigratedEventRow, UpgradeEventRow};

pub(crate) async fn get_upgrade_events(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<UpgradeEventRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT id, package_id, version, event_id, transaction_id, created_at
        FROM upgrade_events
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<UpgradeEventRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_object_migrated_events(
    db: &Db,
    limit: i64,
    offset: i64,
    object_id_filter: Option<&str>,
) -> Result<Vec<ObjectMigratedEventRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = if let Some(object_id) = object_id_filter {
        let query = "
            SELECT id, object_id, object_type, old_version, new_version, migrated_by,
                   event_id, transaction_id, created_at
            FROM object_migrated_events
            WHERE object_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        ";
        diesel::sql_query(query)
            .bind::<Text, _>(object_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ObjectMigratedEventRow>(&mut conn)
    } else {
        let query = "
            SELECT id, object_id, object_type, old_version, new_version, migrated_by,
                   event_id, transaction_id, created_at
            FROM object_migrated_events
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        ";
        diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ObjectMigratedEventRow>(&mut conn)
    }
    .await?;
    Ok(results)
}
