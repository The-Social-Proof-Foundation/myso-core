// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Organization treasury balances and withdrawal access (`OrgTreasury*` events).

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::Selectable;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::{org_treasury_access, org_treasury_balances};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = org_treasury_balances)]
pub struct OrgTreasuryBalanceRow {
    pub organization_id: String,
    pub coin_type: String,
    pub balance: i64,
    pub updated_at_ms: i64,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = org_treasury_access)]
pub struct OrgTreasuryAccessRow {
    pub organization_id: String,
    pub recipient: String,
    pub has_access: bool,
    pub updated_by: String,
    pub updated_at_ms: i64,
}

pub(crate) async fn list_org_treasury_balances(
    conn: &mut Connection<'_>,
    organization_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<OrgTreasuryBalanceRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = org_treasury_balances::table
        .filter(org_treasury_balances::organization_id.eq(organization_id))
        .order(org_treasury_balances::coin_type.asc())
        .select(OrgTreasuryBalanceRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

/// Recipients who may currently withdraw.
pub(crate) async fn list_org_treasury_access(
    conn: &mut Connection<'_>,
    organization_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<OrgTreasuryAccessRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = org_treasury_access::table
        .filter(org_treasury_access::organization_id.eq(organization_id))
        .filter(org_treasury_access::has_access.eq(true))
        .order(org_treasury_access::updated_at_ms.desc())
        .select(OrgTreasuryAccessRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}
