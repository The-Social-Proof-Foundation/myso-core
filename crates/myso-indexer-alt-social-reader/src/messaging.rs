// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel_async::RunQueryDsl;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct MessagingConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub paid_msg_platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub paid_msg_treasury_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub payment_expiration_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub min_reply_chars: i64,
    #[diesel(sql_type = BigInt)]
    pub max_dedupe_key_bytes: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct PaidMessageEscrowRow {
    #[diesel(sql_type = Text)]
    pub group_id: String,
    #[diesel(sql_type = BigInt)]
    pub seq: i64,
    #[diesel(sql_type = Text)]
    pub payer: String,
    #[diesel(sql_type = Text)]
    pub recipient: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = Text)]
    pub status: String,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub platform_fee: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub treasury_fee: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub net_amount: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub reply_char_count: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub created_at_ms: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub claimed_at_ms: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub refunded_at_ms: Option<i64>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct MessagingAgentGroupRow {
    #[diesel(sql_type = Text)]
    pub group_id: String,
    #[diesel(sql_type = Text)]
    pub creator_actor: String,
    #[diesel(sql_type = Text)]
    pub creator_principal: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub creator_sub_agent_id: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub creator_identity_class: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub organization_id: Option<String>,
    #[diesel(sql_type = Text)]
    pub group_name: String,
    #[diesel(sql_type = Text)]
    pub group_uuid: String,
    #[diesel(sql_type = BigInt)]
    pub created_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Latest paid-messaging configuration.
pub(crate) async fn get_messaging_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MessagingConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, paid_msg_platform_fee_bps, paid_msg_treasury_fee_bps,
               payment_expiration_ms, min_reply_chars, max_dedupe_key_bytes, version,
               updated_at, time, transaction_id
        FROM messaging_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<MessagingConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_paid_message_escrows_by_wallet(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    address: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<PaidMessageEscrowRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT DISTINCT ON (group_id, seq)
               group_id, seq, payer, recipient, amount, status,
               platform_fee, treasury_fee, net_amount, reply_char_count,
               created_at_ms, claimed_at_ms, refunded_at_ms, transaction_id
        FROM paid_message_escrows
        WHERE payer = $1 OR recipient = $1
        ORDER BY group_id, seq, time DESC
        LIMIT $2 OFFSET $3
    ";

    let rows = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PaidMessageEscrowRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn get_messaging_agent_groups_by_org(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    organization_id: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<MessagingAgentGroupRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT group_id, creator_actor, creator_principal, creator_sub_agent_id,
               creator_identity_class, organization_id, group_name, group_uuid,
               created_at_ms, transaction_id
        FROM messaging_agent_groups
        WHERE organization_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";

    let rows = diesel::sql_query(query)
        .bind::<Text, _>(organization_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MessagingAgentGroupRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn get_messaging_revenue_summary(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    address: &str,
) -> anyhow::Result<i64> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    #[derive(QueryableByName)]
    struct SumRow {
        #[diesel(sql_type = BigInt)]
        total: i64,
    }

    let query = "
        SELECT COALESCE(SUM(amount), 0)::bigint AS total
        FROM unified_revenue
        WHERE recipient_address = $1
          AND revenue_source = 'messaging'
    ";

    let row = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .get_result::<SumRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(row.total)
}
