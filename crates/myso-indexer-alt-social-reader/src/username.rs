// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    UsernameListing, UsernameRegistryRow, USERNAME_BENEFICIARY_STATUS_ACTIVE,
    USERNAME_LISTING_STATUS_ACTIVE,
};
use myso_indexer_alt_social_schema::schema::{profiles, username_listings, username_registry};
use serde::Serialize;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;
use crate::poc_username_beneficiary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidUsername {
    Empty,
    InvalidCharset,
}

impl std::fmt::Display for InvalidUsername {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidUsername::Empty => write!(f, "username is empty"),
            InvalidUsername::InvalidCharset => {
                write!(f, "username contains characters outside a-z, 0-9, '_', '.'")
            }
        }
    }
}

impl std::error::Error for InvalidUsername {}

/// Canonical username key: ASCII lowercase + charset validation (`a-z`, `0-9`, `_`, `.`).
/// Mirrors `profile::normalize_username` on-chain so `Brandon` and `brandon` resolve to the
/// same registry row and disallowed characters are rejected before any DB lookup.
pub fn canonical_username_key(input: &str) -> Result<String, InvalidUsername> {
    if input.is_empty() {
        return Err(InvalidUsername::Empty);
    }
    let mut out = Vec::with_capacity(input.len());
    for &b in input.as_bytes() {
        // `str` is UTF-8; bytes >= 128 belong to multi-byte sequences (Unicode lookalikes/emoji).
        if b >= 128 {
            return Err(InvalidUsername::InvalidCharset);
        }
        let lower = if b >= b'A' && b <= b'Z' { b + 32 } else { b };
        let ok = lower.is_ascii_lowercase()
            || lower.is_ascii_digit()
            || lower == b'_'
            || lower == b'.';
        if !ok {
            return Err(InvalidUsername::InvalidCharset);
        }
        out.push(lower);
    }
    // SAFETY: all bytes are ASCII.
    Ok(unsafe { String::from_utf8_unchecked(out) })
}

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
    // Lookups are case-insensitive over the canonical key; invalid charset cannot be stored.
    let canonical = match canonical_username_key(username) {
        Ok(c) => c,
        Err(_) => return Ok(None),
    };
    let row = username_registry::table
        .filter(username_registry::username.eq(canonical))
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
    pub marketplace_listed: bool,
    pub registry_profile_id: Option<String>,
    pub beneficiary_id: Option<String>,
    pub listing_seller_profile_id: Option<String>,
    /// Active reservation reasons derived from on-chain state (e.g. "beneficiary", "marketplace").
    pub lock_reasons: Vec<String>,
}

/// Latest `username_listings` row for a canonical username, if any.
pub(crate) async fn get_latest_username_listing(
    conn: &mut Connection<'_>,
    username: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<UsernameListing>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let canonical = match canonical_username_key(username) {
        Ok(c) => c,
        Err(_) => return Ok(None),
    };
    let row = username_listings::table
        .filter(username_listings::username.eq(canonical))
        .order(username_listings::time.desc())
        .select(UsernameListing::as_select())
        .first(conn)
        .await
        .optional()?;
    Ok(row)
}

pub(crate) async fn get_username_availability(
    conn: &mut Connection<'_>,
    username: &str,
    exclude_address: Option<&str>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<UsernameAvailabilityDetail> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    // Canonicalize the lookup key (case-insensitive, charset-validated). An invalid username
    // is never available for registration; return a not-available detail without DB lookups.
    let canonical = match canonical_username_key(username) {
        Ok(c) => c,
        Err(_) => {
            metrics.requests_succeeded.inc();
            return Ok(UsernameAvailabilityDetail {
                username: username.to_string(),
                available: false,
                registry_claimed: false,
                beneficiary_provisioned: false,
                marketplace_listed: false,
                registry_profile_id: None,
                beneficiary_id: None,
                listing_seller_profile_id: None,
                lock_reasons: Vec::new(),
            });
        }
    };

    let registry_entry = get_username_registry_entry(conn, &canonical, metrics).await?;
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
        poc_username_beneficiary::get_by_username(conn, &canonical, metrics).await?;
    let beneficiary_provisioned = active_beneficiary.as_ref().is_some_and(|row| {
        row.status == USERNAME_BENEFICIARY_STATUS_ACTIVE
    });
    let beneficiary_id = if beneficiary_provisioned {
        active_beneficiary
            .as_ref()
            .map(|row| row.beneficiary_id.clone())
    } else {
        None
    };

    let latest_listing = get_latest_username_listing(conn, &canonical, metrics).await?;
    let marketplace_listed = latest_listing
        .as_ref()
        .is_some_and(|l| l.status == USERNAME_LISTING_STATUS_ACTIVE);
    let listing_seller_profile_id = if marketplace_listed {
        latest_listing.as_ref().map(|l| l.seller_profile_id.clone())
    } else {
        None
    };

    let mut lock_reasons = Vec::new();
    if beneficiary_provisioned {
        lock_reasons.push("beneficiary".to_string());
    }
    if marketplace_listed {
        lock_reasons.push("marketplace".to_string());
    }

    let available = !registry_claimed && !beneficiary_provisioned && !marketplace_listed;
    metrics.requests_succeeded.inc();
    Ok(UsernameAvailabilityDetail {
        username: canonical,
        available,
        registry_claimed,
        beneficiary_provisioned,
        marketplace_listed,
        registry_profile_id: if registry_claimed {
            registry_entry.map(|entry| entry.profile_id)
        } else {
            None
        },
        beneficiary_id,
        listing_seller_profile_id,
        lock_reasons,
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
