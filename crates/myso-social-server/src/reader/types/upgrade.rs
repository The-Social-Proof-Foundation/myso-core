// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Integer, Text, Timestamptz};
use serde::Serialize;

#[derive(Debug, Serialize, QueryableByName)]
pub struct UpgradeEventRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub package_id: String,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct ObjectMigratedEventRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub object_id: String,
    #[diesel(sql_type = Text)]
    pub object_type: String,
    #[diesel(sql_type = BigInt)]
    pub old_version: i64,
    #[diesel(sql_type = BigInt)]
    pub new_version: i64,
    #[diesel(sql_type = Text)]
    pub migrated_by: String,
    #[diesel(sql_type = Text)]
    pub event_id: String,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
