// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::Bool;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

pub(crate) async fn check_following(
    conn: &mut Connection<'_>,
    follower_address: &str,
    following_address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<bool> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct FollowRow {
        #[diesel(sql_type = Bool)]
        exists: bool,
    }
    let result = diesel::sql_query(
        "SELECT EXISTS(
            SELECT 1 FROM social_graph_relationships
            WHERE follower_address = $1 AND following_address = $2
        ) as exists",
    )
    .bind::<diesel::sql_types::Text, _>(follower_address)
    .bind::<diesel::sql_types::Text, _>(following_address)
    .get_result::<FollowRow>(conn)
    .await?;
    metrics.requests_succeeded.inc();
    Ok(result.exists)
}
