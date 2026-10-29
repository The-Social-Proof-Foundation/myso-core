// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Purchases that unlocked a gated post (`PostAccessPurchaseEvent`).

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::Selectable;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::post_access_purchases;
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = post_access_purchases)]
pub struct PostAccessPurchaseRow {
    pub post_id: String,
    pub mydata_id: String,
    pub buyer: String,
    pub seller: String,
    pub purchased_at_ms: i64,
}

/// Newest first. `buyer` narrows to one wallet (has this viewer bought access?).
pub(crate) async fn list_post_access_purchases(
    conn: &mut Connection<'_>,
    post_id: &str,
    buyer: Option<&str>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostAccessPurchaseRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let mut query = post_access_purchases::table
        .filter(post_access_purchases::post_id.eq(post_id))
        .into_boxed();
    if let Some(buyer) = buyer {
        query = query.filter(post_access_purchases::buyer.eq(buyer));
    }
    let rows = query
        .order(post_access_purchases::purchased_at_ms.desc())
        .limit(limit)
        .offset(offset)
        .select(PostAccessPurchaseRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn count_post_access_purchases(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<i64> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let count = post_access_purchases::table
        .filter(post_access_purchases::post_id.eq(post_id))
        .count()
        .get_result(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(count)
}
