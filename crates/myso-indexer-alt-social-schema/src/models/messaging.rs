// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    message_digests, messaging_agent_groups, messaging_config, paid_message_escrows,
};

pub const PAID_MESSAGE_STATUS_ESCROWED: &str = "escrowed";
pub const PAID_MESSAGE_STATUS_CLAIMED: &str = "claimed";
pub const PAID_MESSAGE_STATUS_SETTLED: &str = "settled";
pub const PAID_MESSAGE_STATUS_REFUNDED: &str = "refunded";

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = messaging_config)]
pub struct NewMessagingConfig {
    pub updated_by: String,
    pub paid_msg_platform_fee_bps: i64,
    pub paid_msg_treasury_fee_bps: i64,
    pub payment_expiration_ms: i64,
    pub min_reply_chars: i64,
    pub max_dedupe_key_bytes: i64,
    pub version: i64,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewMessagingConfig {
    pub fn from_event(
        updated_by: String,
        paid_msg_platform_fee_bps: u64,
        paid_msg_treasury_fee_bps: u64,
        payment_expiration_ms: u64,
        min_reply_chars: u64,
        max_dedupe_key_bytes: u64,
        version: u64,
        updated_at: u64,
        transaction_id: String,
    ) -> Self {
        let time = chrono::DateTime::<chrono::Utc>::from_timestamp((updated_at / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now);
        Self {
            updated_by,
            paid_msg_platform_fee_bps: paid_msg_platform_fee_bps as i64,
            paid_msg_treasury_fee_bps: paid_msg_treasury_fee_bps as i64,
            payment_expiration_ms: payment_expiration_ms as i64,
            min_reply_chars: min_reply_chars as i64,
            max_dedupe_key_bytes: max_dedupe_key_bytes as i64,
            version: version as i64,
            updated_at: updated_at as i64,
            time,
            transaction_id,
        }
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = paid_message_escrows)]
pub struct NewPaidMessageEscrow {
    pub group_id: String,
    pub seq: i64,
    pub payer: String,
    pub recipient: String,
    pub amount: i64,
    pub status: String,
    pub platform_fee: Option<i64>,
    pub treasury_fee: Option<i64>,
    pub net_amount: Option<i64>,
    pub platform_fee_recipient: Option<String>,
    pub ecosystem_fee_recipient: Option<String>,
    pub reply_char_count: Option<i64>,
    pub created_at_ms: i64,
    pub claimed_at_ms: Option<i64>,
    pub refunded_at_ms: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = message_digests)]
pub struct NewMessageDigest {
    pub group_id: String,
    pub seq: i64,
    pub sender: String,
    pub recipient: String,
    pub content_digest: String,
    pub content_uri: String,
    pub created_at_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = messaging_agent_groups)]
pub struct NewMessagingAgentGroup {
    pub group_id: String,
    pub creator_actor: String,
    pub creator_principal: String,
    pub creator_sub_agent_id: Option<String>,
    pub creator_identity_class: i64,
    pub organization_id: Option<String>,
    pub group_name: String,
    pub group_uuid: String,
    pub created_at_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}
