// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    PocCreatorIdentityLinkRow, PocUsernameBeneficiaryRow, USERNAME_BENEFICIARY_STATUS_ACTIVE,
};
use myso_indexer_alt_social_schema::schema::{
    poc_creator_identity_links, poc_username_beneficiaries,
};

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;
use crate::metrics::standalone_reader_metrics;

pub async fn get_by_username(
    conn: &mut Connection<'_>,
    username: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocUsernameBeneficiaryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = poc_username_beneficiaries::table
        .filter(poc_username_beneficiaries::username.eq(username))
        .select(PocUsernameBeneficiaryRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub async fn get_by_id(
    conn: &mut Connection<'_>,
    beneficiary_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocUsernameBeneficiaryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = poc_username_beneficiaries::table
        .filter(poc_username_beneficiaries::beneficiary_id.eq(beneficiary_id))
        .select(PocUsernameBeneficiaryRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub async fn has_active(
    conn: &mut Connection<'_>,
    username: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<bool> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let count: i64 = poc_username_beneficiaries::table
        .filter(poc_username_beneficiaries::username.eq(username))
        .filter(poc_username_beneficiaries::status.eq(USERNAME_BENEFICIARY_STATUS_ACTIVE))
        .count()
        .get_result(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(count > 0)
}

pub async fn is_username_available_for_registration(
    conn: &mut Connection<'_>,
    username: &str,
    exclude_address: Option<&str>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<bool> {
    let detail =
        crate::username::get_username_availability(conn, username, exclude_address, metrics).await?;
    Ok(detail.available)
}

pub async fn list_username_beneficiaries(
    conn: &mut Connection<'_>,
    status: Option<i16>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocUsernameBeneficiaryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let mut query = poc_username_beneficiaries::table
        .into_boxed()
        .order(poc_username_beneficiaries::provisioned_at_ms.desc());
    if let Some(status) = status {
        query = query.filter(poc_username_beneficiaries::status.eq(status));
    }
    let rows = query
        .select(PocUsernameBeneficiaryRow::as_select())
        .limit(limit)
        .offset(offset)
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub async fn get_creator_identity_link(
    conn: &mut Connection<'_>,
    creator_identity_source: i16,
    creator_identity_hash: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocCreatorIdentityLinkRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = poc_creator_identity_links::table
        .filter(poc_creator_identity_links::creator_identity_source.eq(creator_identity_source))
        .filter(poc_creator_identity_links::creator_identity_hash.eq(creator_identity_hash))
        .select(PocCreatorIdentityLinkRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub async fn get_creator_identity_link_by_wallet(
    conn: &mut Connection<'_>,
    wallet_address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocCreatorIdentityLinkRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = poc_creator_identity_links::table
        .filter(poc_creator_identity_links::wallet_address.eq(wallet_address))
        .order(poc_creator_identity_links::linked_at_ms.desc())
        .select(PocCreatorIdentityLinkRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub async fn get_by_vault_id(
    conn: &mut Connection<'_>,
    vault_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocUsernameBeneficiaryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = poc_username_beneficiaries::table
        .filter(poc_username_beneficiaries::vault_id.eq(vault_id))
        .select(PocUsernameBeneficiaryRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub async fn get_poc_username_beneficiary_by_vault_id_for_conn(
    conn: &mut Connection<'_>,
    vault_id: &str,
) -> anyhow::Result<Option<PocUsernameBeneficiaryRow>> {
    get_by_vault_id(conn, vault_id, standalone_reader_metrics()).await
}

pub async fn get_poc_username_beneficiary_by_username_for_conn(
    conn: &mut Connection<'_>,
    username: &str,
) -> anyhow::Result<Option<PocUsernameBeneficiaryRow>> {
    get_by_username(conn, username, standalone_reader_metrics()).await
}

pub async fn get_poc_username_beneficiary_by_id_for_conn(
    conn: &mut Connection<'_>,
    beneficiary_id: &str,
) -> anyhow::Result<Option<PocUsernameBeneficiaryRow>> {
    get_by_id(conn, beneficiary_id, standalone_reader_metrics()).await
}

pub async fn list_username_beneficiaries_for_conn(
    conn: &mut Connection<'_>,
    status: Option<i16>,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<PocUsernameBeneficiaryRow>> {
    list_username_beneficiaries(conn, status, limit, offset, standalone_reader_metrics()).await
}

pub async fn get_poc_creator_identity_link_for_conn(
    conn: &mut Connection<'_>,
    creator_identity_source: i16,
    creator_identity_hash: &str,
) -> anyhow::Result<Option<PocCreatorIdentityLinkRow>> {
    get_creator_identity_link(
        conn,
        creator_identity_source,
        creator_identity_hash,
        standalone_reader_metrics(),
    )
    .await
}
