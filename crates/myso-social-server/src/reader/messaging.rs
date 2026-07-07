// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Text};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::WalletMessagingPolicy;
use myso_indexer_alt_social_schema::schema::wallet_messaging_policies;

use crate::error::SocialError;
use crate::reader::types::{
    MessagingAgentGroupInfo, MessagingConfigInfo, MessagingRevenueSummaryInfo,
    PaidMessageEscrowInfo, WalletMessagingPolicyResponse,
};
use myso_pg_db::Db;

pub(crate) async fn get_wallet_messaging_policy(
    db: &Db,
    address: &str,
) -> Result<Option<WalletMessagingPolicyResponse>, SocialError> {
    let mut conn = db.connect().await?;
    let policy = wallet_messaging_policies::table
        .filter(wallet_messaging_policies::wallet_address.eq(address))
        .select(WalletMessagingPolicy::as_select())
        .first(&mut conn)
        .await
        .optional()?;
    Ok(policy.map(WalletMessagingPolicyResponse::from))
}

pub(crate) async fn get_messaging_configuration(
    db: &Db,
) -> Result<Option<MessagingConfigInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT updated_by, paid_msg_platform_fee_bps, paid_msg_treasury_fee_bps,
               payment_expiration_ms, min_reply_chars, max_dedupe_key_bytes, updated_at
        FROM messaging_config
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<MessagingConfigInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_paid_message_escrows(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PaidMessageEscrowInfo>, SocialError> {
    let mut conn = db.connect().await?;
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
        .load::<PaidMessageEscrowInfo>(&mut conn)
        .await?;
    Ok(rows)
}

pub(crate) async fn get_messaging_agent_groups(
    db: &Db,
    organization_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<MessagingAgentGroupInfo>, SocialError> {
    let mut conn = db.connect().await?;
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
        .load::<MessagingAgentGroupInfo>(&mut conn)
        .await?;
    Ok(rows)
}

pub(crate) async fn get_messaging_revenue_summary(
    db: &Db,
    address: &str,
) -> Result<MessagingRevenueSummaryInfo, SocialError> {
    let mut conn = db.connect().await?;
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
        .get_result::<SumRow>(&mut conn)
        .await?;
    Ok(MessagingRevenueSummaryInfo {
        address: address.to_string(),
        total_messaging_revenue: row.total,
    })
}
