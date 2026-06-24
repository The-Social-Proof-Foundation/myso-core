// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::UsernameRegistryRow;
use myso_indexer_alt_social_schema::schema::username_registry;
use serde::Serialize;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, Serialize)]
pub struct UsernameRegistryEntry {
    pub username: String,
    pub profile_id: String,
    pub transaction_id: String,
}

impl From<UsernameRegistryRow> for UsernameRegistryEntry {
    fn from(row: UsernameRegistryRow) -> Self {
        Self {
            username: row.username,
            profile_id: row.profile_id,
            transaction_id: row.transaction_id,
        }
    }
}

pub(crate) async fn get_username_registry_entry(
    conn: &mut Connection<'_>,
    username: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<UsernameRegistryEntry>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = username_registry::table
        .filter(username_registry::username.eq(username))
        .select(UsernameRegistryRow::as_select())
        .first(conn)
        .await
        .optional()?;
    Ok(row.map(UsernameRegistryEntry::from))
}

pub(crate) async fn is_username_available(
    conn: &mut Connection<'_>,
    username: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<bool> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let count: i64 = username_registry::table
        .filter(username_registry::username.eq(username))
        .count()
        .get_result(conn)
        .await?;
    Ok(count == 0)
}
