// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::QueryableByName;
use serde::Serialize;

#[derive(Debug, Serialize, QueryableByName)]
#[serde(rename_all = "camelCase")]
pub struct MessagingConfigInfo {
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
    pub updated_at: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
#[serde(rename_all = "camelCase")]
pub struct PaidMessageEscrowInfo {
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

#[derive(Debug, Serialize, QueryableByName)]
#[serde(rename_all = "camelCase")]
pub struct MessagingAgentGroupInfo {
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagingRevenueSummaryInfo {
    pub address: String,
    pub total_messaging_revenue: i64,
}
