// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::UsernameRegistryRow;
use myso_indexer_alt_social_schema::schema::{profiles, username_registry};
use serde::Serialize;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;
use crate::poc_username_beneficiary;

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

#[derive(Debug, Clone, Serialize)]
pub struct UsernameAvailabilityDetail {
    pub username: String,
    pub available: bool,
    pub registry_claimed: bool,
    pub beneficiary_provisioned: bool,
    pub registry_profile_id: Option<String>,
    pub beneficiary_id: Option<String>,
}

pub(crate) async fn get_username_availability(
    conn: &mut Connection<'_>,
    username: &str,
    exclude_address: Option<&str>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<UsernameAvailabilityDetail> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let registry_entry = get_username_registry_entry(conn, username, metrics).await?;
    let registry_claimed = match (&registry_entry, exclude_address) {
        (None, _) => false,
        (Some(_), None) => true,
        (Some(entry), Some(addr)) => {
            let owner: Option<String> = profiles::table
                .filter(profiles::profile_id.eq(entry.profile_id.clone()))
                .select(profiles::owner_address)
                .first(conn)
                .await
                .optional()?;
            owner.as_deref() != Some(addr)
        }
    };

    let active_beneficiary =
        poc_username_beneficiary::get_by_username(conn, username, metrics).await?;
    let beneficiary_provisioned = active_beneficiary
        .as_ref()
        .is_some_and(|row| row.status == myso_indexer_alt_social_schema::models::USERNAME_BENEFICIARY_STATUS_ACTIVE);
    let beneficiary_id = if beneficiary_provisioned {
        active_beneficiary.as_ref().map(|row| row.beneficiary_id.clone())
    } else {
        None
    };

    let available = !registry_claimed && !beneficiary_provisioned;
    metrics.requests_succeeded.inc();
    Ok(UsernameAvailabilityDetail {
        username: username.to_string(),
        available,
        registry_claimed,
        beneficiary_provisioned,
        registry_profile_id: if registry_claimed {
            registry_entry.map(|entry| entry.profile_id)
        } else {
            None
        },
        beneficiary_id,
    })
}

pub(crate) async fn is_username_available(
    conn: &mut Connection<'_>,
    username: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<bool> {
    get_username_availability(conn, username, None, metrics)
        .await
        .map(|detail| detail.available)
}
