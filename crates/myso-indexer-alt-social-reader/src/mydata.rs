// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{
    MyDataAccessAnalyticsRow, MyDataAccessLogRow, MyDataBroadPoolRow, MyDataClaimRow,
    MyDataDailyRevenueRow, MyDataDistributionRoundRow, MyDataListingSubPoolRow,
    MyDataMerkleRootRow, MyDataPurchaseRow, MyDataRecordRow, MyDataRevenueRow,
    MyDataSnapshotAnchorRow, MyDataSnapshotEscrowRow, MyDataStatsRow, MyDataSubPoolRow, MyDataSubscriptionRow,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct MyDataConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    /// Whether buyers may start new broad-pool/snapshot marketplace rounds.
    pub marketplace_enabled: bool,
    #[diesel(sql_type = BigInt)]
    pub max_tags: i64,
    #[diesel(sql_type = BigInt)]
    pub max_subscription_days: i64,
    #[diesel(sql_type = BigInt)]
    pub max_free_access_grants: i64,
    #[diesel(sql_type = BigInt)]
    pub max_encryption_id_bytes: i64,
    #[diesel(sql_type = BigInt)]
    pub max_encrypted_data_bytes: i64,
    #[diesel(sql_type = BigInt)]
    pub max_tag_bytes: i64,
    #[diesel(sql_type = BigInt)]
    pub max_metadata_bytes: i64,
    #[diesel(sql_type = BigInt)]
    pub max_payment_reference_bytes: i64,
    #[diesel(sql_type = BigInt)]
    pub max_pool_assignments: i64,
    #[diesel(sql_type = BigInt)]
    pub max_merkle_proof_depth: i64,
    #[diesel(sql_type = BigInt)]
    pub max_paid_access_entries: i64,
    #[diesel(sql_type = BigInt)]
    pub default_claim_window_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub p2p_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub p2p_ecosystem_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub mydata_marketplace_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub mydata_marketplace_ecosystem_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_creator_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_treasury_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

pub(crate) async fn get_mydata_record(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MyDataRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT mydata_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
               created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
               geographic_region, data_quality, sample_size, collection_method, is_updating, update_frequency,
               access_configuration_kind
        FROM mydata_data
        WHERE mydata_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .get_result::<MyDataRecordRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_mydata_records_by_owner(
    conn: &mut Connection<'_>,
    owner: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT mydata_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
               created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
               geographic_region, data_quality, sample_size, collection_method, is_updating, update_frequency,
               access_configuration_kind
        FROM mydata_data
        WHERE owner = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(owner)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataRecordRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_mydata_purchases_by_buyer(
    conn: &mut Connection<'_>,
    buyer: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataPurchaseRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, mydata_id, buyer, price, platform_fee, ecosystem_fee, creator_amount,
               platform_address, purchase_type, purchase_time, time, transaction_id,
               revoked, revoked_at, revoked_by
        FROM mydata_purchases
        WHERE buyer = $1
        ORDER BY purchase_time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(buyer)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataPurchaseRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MyDataConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, marketplace_enabled, max_tags, max_subscription_days, max_free_access_grants,
               max_encryption_id_bytes, max_encrypted_data_bytes, max_tag_bytes, max_metadata_bytes,
               max_payment_reference_bytes, max_pool_assignments, max_merkle_proof_depth,
               max_paid_access_entries, default_claim_window_ms,
               p2p_platform_fee_bps, p2p_ecosystem_fee_bps,
               mydata_marketplace_platform_fee_bps, mydata_marketplace_ecosystem_fee_bps,
               non_platform_platform_to_creator_bps, non_platform_platform_to_treasury_bps,
               version, updated_at, time, transaction_id
        FROM mydata_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<MyDataConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_mydata(
    conn: &mut Connection<'_>,
    creator: Option<&str>,
    media_type: Option<&str>,
    platform_id: Option<&str>,
    sort_by: Option<&str>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let sort_clause = match sort_by {
        Some("price") => " ORDER BY COALESCE(one_time_price, subscription_price) DESC",
        Some("updated") => " ORDER BY last_updated DESC",
        _ => " ORDER BY created_at DESC",
    };

    let query = format!(
        "
        SELECT mydata_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
               created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
               geographic_region, data_quality, sample_size, collection_method, is_updating, update_frequency,
               access_configuration_kind
        FROM mydata_data
        WHERE ($1::text IS NULL OR owner = $1)
          AND ($2::text IS NULL OR media_type = $2)
          AND ($3::text IS NULL OR platform_id = $3)
        {}
        LIMIT $4 OFFSET $5
        ",
        sort_clause
    );

    let results = diesel::sql_query(&query)
        .bind::<Nullable<Text>, _>(creator)
        .bind::<Nullable<Text>, _>(media_type)
        .bind::<Nullable<Text>, _>(platform_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataRecordRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_popular_mydata(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT
            d.mydata_id, d.owner, d.media_type, d.tags, d.platform_id, d.timestamp_start, d.timestamp_end,
            d.created_at, d.last_updated, d.one_time_price, d.subscription_price, d.subscription_duration_days,
            d.geographic_region, d.data_quality, d.sample_size, d.collection_method, d.is_updating, d.update_frequency,
            access_configuration_kind
        FROM mydata_popular_30_days p
        INNER JOIN mydata_data d ON d.mydata_id = p.mydata_id
        ORDER BY p.unique_purchasers DESC, p.total_revenue DESC NULLS LAST, d.created_at DESC
        LIMIT $1 OFFSET $2
    ";

    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataRecordRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_purchases(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataPurchaseRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, mydata_id, buyer, price, platform_fee, ecosystem_fee, creator_amount,
               platform_address, purchase_type, purchase_time, time, transaction_id,
               revoked, revoked_at, revoked_by
        FROM mydata_purchases
        WHERE mydata_id = $1
        ORDER BY purchase_time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataPurchaseRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_subscriptions(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataSubscriptionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, mydata_id, subscriber, subscription_start, subscription_end, price, time, transaction_id,
               revoked, revoked_at, revoked_by
        FROM mydata_subscriptions
        WHERE mydata_id = $1
        ORDER BY subscription_start DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataSubscriptionRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_revenue(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataRevenueRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, mydata_id, from_address, to_address, amount, platform_fee, ecosystem_fee,
               creator_amount, platform_address, revenue_type, revenue_time, time, transaction_id
        FROM mydata_revenue
        WHERE mydata_id = $1
        ORDER BY revenue_time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataRevenueRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_access_logs(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataAccessLogRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, mydata_id, user_address, access_type, access_time, time, transaction_id
        FROM mydata_access_logs
        WHERE mydata_id = $1
        ORDER BY access_time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataAccessLogRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_stats(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MyDataStatsRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT
            d.mydata_id, d.owner, d.media_type,
            COALESCE((SELECT SUM(amount)::bigint FROM mydata_revenue WHERE mydata_id = $1), 0) as total_revenue,
            (SELECT COUNT(*) FROM mydata_purchases WHERE mydata_id = $1 AND revoked = FALSE) as purchase_count,
            (SELECT COUNT(*) FROM mydata_subscriptions WHERE mydata_id = $1 AND revoked = FALSE AND subscription_end >= (EXTRACT(EPOCH FROM NOW()) * 1000)::bigint) as subscription_count,
            (SELECT COUNT(*) FROM mydata_access_logs WHERE mydata_id = $1) as access_count,
            d.one_time_price, d.subscription_price, d.created_at, d.last_updated
        FROM mydata_data d
        WHERE d.mydata_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .get_result::<MyDataStatsRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_mydata_revenue_timeline(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataDailyRevenueRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT
            time_bucket('1 day', to_timestamp(revenue_time / 1000.0))::date as day,
            SUM(amount)::bigint as daily_revenue,
            COUNT(*) as daily_transactions
        FROM mydata_revenue
        WHERE mydata_id = $1
        GROUP BY time_bucket('1 day', to_timestamp(revenue_time / 1000.0))
        ORDER BY day DESC
        LIMIT 30
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .load::<MyDataDailyRevenueRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_access_analytics(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataAccessAnalyticsRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT
            time_bucket('1 day', to_timestamp(access_time / 1000.0))::date as day,
            access_type,
            COUNT(DISTINCT user_address) as unique_users,
            COUNT(*) as total_accesses
        FROM mydata_access_logs
        WHERE mydata_id = $1
        GROUP BY time_bucket('1 day', to_timestamp(access_time / 1000.0)), access_type
        ORDER BY day DESC, access_type
        LIMIT 100
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .load::<MyDataAccessAnalyticsRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_mydata_broad_pools(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataBroadPoolRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT pool_id, name, platform_address, created_at_ms, event_id, transaction_id, time
        FROM mydata_broad_pools
        ORDER BY created_at_ms DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataBroadPoolRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_mydata_sub_pools_for_broad_pool(
    conn: &mut Connection<'_>,
    broad_pool_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataSubPoolRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT sub_pool_id, broad_pool_id, name, created_at_ms, event_id, transaction_id, time
        FROM mydata_sub_pools
        WHERE broad_pool_id = $1
        ORDER BY created_at_ms DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(broad_pool_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataSubPoolRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_mydata_sub_pools_for_listing(
    conn: &mut Connection<'_>,
    listing_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataSubPoolRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT s.sub_pool_id, s.broad_pool_id, s.name, s.created_at_ms, j.event_id, j.transaction_id, j.time
        FROM mydata_listing_sub_pools j
        INNER JOIN mydata_sub_pools s ON s.sub_pool_id = j.sub_pool_id
        WHERE j.listing_id = $1
        ORDER BY j.assigned_at_ms DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(listing_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataSubPoolRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_mydata_listings_for_sub_pool(
    conn: &mut Connection<'_>,
    sub_pool_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataListingSubPoolRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT listing_id, sub_pool_id, assigned_at_ms, event_id, transaction_id, time
        FROM mydata_listing_sub_pools
        WHERE sub_pool_id = $1
        ORDER BY assigned_at_ms DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(sub_pool_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataListingSubPoolRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_snapshot_anchor(
    conn: &mut Connection<'_>,
    snapshot_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MyDataSnapshotAnchorRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT id, snapshot_id, buyer_address, price_paid, source_pool_id, source_sub_pool_id,
               platform_address, initial_escrow, created_at_ms, event_id, transaction_id, time,
               manifest_hash, payment_reference
        FROM mydata_snapshot_anchors
        WHERE snapshot_id = $1
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(snapshot_id)
        .get_result::<MyDataSnapshotAnchorRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_mydata_snapshot_anchors(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataSnapshotAnchorRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT id, snapshot_id, buyer_address, price_paid, source_pool_id, source_sub_pool_id,
               platform_address, initial_escrow, created_at_ms, event_id, transaction_id, time,
               manifest_hash, payment_reference
        FROM mydata_snapshot_anchors
        ORDER BY time DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataSnapshotAnchorRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_distribution_round(
    conn: &mut Connection<'_>,
    snapshot_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MyDataDistributionRoundRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT snapshot_id, total_amount, contributor_count, merkle_root, platform_address,
               claim_deadline_ms, published_at_ms,
               event_id, transaction_id, time
        FROM mydata_distribution_rounds
        WHERE snapshot_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(snapshot_id)
        .get_result::<MyDataDistributionRoundRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_mydata_distribution_rounds(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataDistributionRoundRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT snapshot_id, total_amount, contributor_count, merkle_root, platform_address,
               claim_deadline_ms, published_at_ms,
               event_id, transaction_id, time
        FROM mydata_distribution_rounds
        ORDER BY time DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataDistributionRoundRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_merkle_root(
    conn: &mut Connection<'_>,
    snapshot_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MyDataMerkleRootRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT snapshot_id, root_hash, published_at_ms, event_id, transaction_id, time
        FROM mydata_merkle_roots
        WHERE snapshot_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(snapshot_id)
        .get_result::<MyDataMerkleRootRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_mydata_snapshot_escrow(
    conn: &mut Connection<'_>,
    snapshot_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MyDataSnapshotEscrowRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT snapshot_id, total_funded, total_claimed, remaining_amount, claim_deadline_ms,
               reclaimed_at_ms, status, updated_at_ms, transaction_id, time
        FROM mydata_snapshot_escrow
        WHERE snapshot_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(snapshot_id)
        .get_result::<MyDataSnapshotEscrowRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_mydata_claims_for_snapshot(
    conn: &mut Connection<'_>,
    snapshot_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataClaimRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT id, snapshot_id, claimant, amount, gross_amount, platform_fee, ecosystem_fee,
               net_amount, platform_address, claimed_at_ms, event_id, transaction_id, time
        FROM mydata_claims
        WHERE snapshot_id = $1
        ORDER BY claimed_at_ms DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(snapshot_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataClaimRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

#[derive(Debug, Clone, QueryableByName)]
pub struct MyDataHasAccessRow {
    #[diesel(sql_type = Bool)]
    pub has_access: bool,
}

pub(crate) async fn check_mydata_has_access(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    user_address: &str,
    at_ms: Option<i64>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<bool> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let at_ms = at_ms.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    });

    let query = "SELECT user_has_access($1, $2, $3) AS has_access";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .bind::<Text, _>(user_address)
        .bind::<BigInt, _>(at_ms)
        .get_result::<MyDataHasAccessRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(result.has_access)
}
