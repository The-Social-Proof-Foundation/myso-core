// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Domain handler for messaging package events (config, paid messages, agent groups).

use serde::Deserialize;

use super::common;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewMessageDigest, NewMessagingAgentGroup, NewMessagingConfig, NewPaidMessageEscrow, NewUnifiedRevenue,
    NewWalletMessagingPolicy, PAID_MESSAGE_STATUS_CLAIMED, PAID_MESSAGE_STATUS_ESCROWED,
    PAID_MESSAGE_STATUS_REFUNDED, PAID_MESSAGE_STATUS_SETTLED, REVENUE_TYPE_MESSAGING_CLAIM,
    REVENUE_TYPE_MESSAGING_NET, REVENUE_TYPE_MESSAGING_PLATFORM_FEE, REVENUE_TYPE_MESSAGING_REFUND,
    REVENUE_TYPE_MESSAGING_TREASURY_FEE,
};

#[derive(Debug, Deserialize)]
struct MessagingConfigUpdatedEvent {
    updated_by: String,
    timestamp: u64,
    paid_msg_platform_fee_bps: u64,
    paid_msg_treasury_fee_bps: u64,
    payment_expiration_ms: u64,
    min_reply_chars: u32,
    max_dedupe_key_bytes: u64,
}

#[derive(Debug, Deserialize)]
struct PaidMessageSentEvent {
    group_id: String,
    seq: u64,
    payer: String,
    recipient: String,
    amount: u64,
    created_at_ms: u64,
}

#[derive(Debug, Deserialize)]
struct MessageDigestSentEvent {
    group_id: String,
    seq: u64,
    sender: String,
    recipient: String,
    content_digest: String,
    content_uri: String,
    created_at_ms: u64,
}

#[derive(Debug, Deserialize)]
struct PaidMessageRepliedEvent {
    group_id: String,
    paid_msg_seq: u64,
    reply_char_count: u32,
}

#[derive(Debug, Deserialize)]
struct PaymentClaimedEvent {
    group_id: String,
    seq: u64,
    recipient: String,
    amount: u64,
    claimed_at_ms: u64,
}

#[derive(Debug, Deserialize)]
struct PaymentClaimedSettledEvent {
    group_id: String,
    seq: u64,
    recipient: String,
    total_amount: u64,
    platform_fee: u64,
    treasury_fee: u64,
    net_amount: u64,
    platform_fee_recipient: String,
    ecosystem_fee_recipient: String,
    claimed_at_ms: u64,
}

#[derive(Debug, Deserialize)]
struct PaymentRefundedEvent {
    group_id: String,
    seq: u64,
    payer: String,
    amount: u64,
    refunded_at_ms: u64,
}

#[derive(Debug, Deserialize)]
struct AgentGroupCreatedEvent {
    group_id: String,
    creator_actor: String,
    creator_principal: String,
    creator_sub_agent_id: Option<String>,
    creator_identity_class: u64,
    organization_id: Option<String>,
    group_name: String,
    group_uuid: String,
    created_at: u64,
}

fn content_id(group_id: &str, seq: u64) -> String {
    format!("{group_id}:{seq}")
}

pub fn handle_messaging_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "MessagingConfigUpdatedEvent" => {
            process_messaging_config_updated_event(data, event_id, checkpoint_timestamp_ms)
        }
        "AgentGroupCreated" => {
            process_agent_group_created_event(data, event_id, checkpoint_timestamp_ms)
        }
        _ => None,
    }
}

pub fn handle_message_log_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    reply_char_count: Option<u32>,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "MessageDigestSent" => {
            process_message_digest_sent_event(data, event_id, checkpoint_timestamp_ms)
        }
        "PaidMessageSent" => {
            process_paid_message_sent_event(data, event_id, checkpoint_timestamp_ms)
        }
        "PaidMessageReplied" => None,
        "PaymentClaimed" => process_payment_claimed_event(
            data,
            event_id,
            checkpoint_timestamp_ms,
            reply_char_count,
        ),
        "PaymentClaimedSettled" => process_payment_claimed_settled_event(
            data,
            event_id,
            checkpoint_timestamp_ms,
            reply_char_count,
        ),
        "PaymentRefunded" => {
            process_payment_refunded_event(data, event_id, checkpoint_timestamp_ms)
        }
        _ => None,
    }
}

fn process_message_digest_sent_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: MessageDigestSentEvent = common::deserialize_social_event_json(
        "message_log",
        "MessageDigestSent",
        event_id,
        data,
        "message_log MessageDigestSent JSON did not match MessageDigestSentEvent",
    )?;
    let timestamp_ms = common::chain_timestamp_ms(
        Some(ev.created_at_ms as i64),
        checkpoint_timestamp_ms,
    );
    Some(vec![SocialEventRow::MessageDigest(NewMessageDigest {
        group_id: ev.group_id,
        seq: ev.seq as i64,
        sender: ev.sender,
        recipient: ev.recipient,
        content_digest: ev.content_digest,
        content_uri: ev.content_uri,
        created_at_ms: timestamp_ms,
        time: common::chain_time_from_ms(timestamp_ms),
        transaction_id: event_id.to_string(),
    })])
}

fn process_messaging_config_updated_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: MessagingConfigUpdatedEvent = common::deserialize_social_event_json(
        "messaging_config",
        "MessagingConfigUpdatedEvent",
        event_id,
        data,
        "messaging MessagingConfigUpdatedEvent JSON did not match MessagingConfigUpdatedEvent",
    )?;
    let event_ms = Some(ev.timestamp as i64);
    let timestamp_ms = common::chain_timestamp_ms(event_ms, checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(timestamp_ms);
    let row = NewMessagingConfig {
        updated_by: ev.updated_by,
        paid_msg_platform_fee_bps: ev.paid_msg_platform_fee_bps as i64,
        paid_msg_treasury_fee_bps: ev.paid_msg_treasury_fee_bps as i64,
        payment_expiration_ms: ev.payment_expiration_ms as i64,
        min_reply_chars: ev.min_reply_chars as i64,
        max_dedupe_key_bytes: ev.max_dedupe_key_bytes as i64,
        version: 0,
        updated_at: timestamp_ms,
        time,
        transaction_id: event_id.to_string(),
    };
    Some(vec![SocialEventRow::MessagingConfig(row)])
}

fn process_agent_group_created_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: AgentGroupCreatedEvent = common::deserialize_social_event_json(
        "messaging",
        "AgentGroupCreated",
        event_id,
        data,
        "messaging AgentGroupCreated JSON did not match AgentGroupCreatedEvent",
    )?;
    let timestamp_ms =
        common::chain_timestamp_ms(Some(ev.created_at as i64), checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(timestamp_ms);
    let organization_id = ev.organization_id.clone();
    let row = NewMessagingAgentGroup {
        group_id: ev.group_id,
        creator_actor: ev.creator_actor,
        creator_principal: ev.creator_principal,
        creator_sub_agent_id: ev.creator_sub_agent_id,
        creator_identity_class: ev.creator_identity_class as i64,
        organization_id: organization_id.clone(),
        group_name: ev.group_name,
        group_uuid: ev.group_uuid,
        created_at_ms: timestamp_ms,
        time,
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::MessagingAgentGroup(row),
        SocialEventRow::MessagingAgentGroupOrgActivity {
            organization_id,
            activity_at_ms: timestamp_ms,
        },
    ])
}

fn process_paid_message_sent_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: PaidMessageSentEvent = common::deserialize_social_event_json(
        "message_log",
        "PaidMessageSent",
        event_id,
        data,
        "message_log PaidMessageSent JSON did not match PaidMessageSentEvent",
    )?;
    let timestamp_ms =
        common::chain_timestamp_ms(Some(ev.created_at_ms as i64), checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(timestamp_ms);
    let escrow = NewPaidMessageEscrow {
        group_id: ev.group_id.clone(),
        seq: ev.seq as i64,
        payer: ev.payer.clone(),
        recipient: ev.recipient.clone(),
        amount: ev.amount as i64,
        status: PAID_MESSAGE_STATUS_ESCROWED.to_string(),
        platform_fee: None,
        treasury_fee: None,
        net_amount: None,
        platform_fee_recipient: None,
        ecosystem_fee_recipient: None,
        reply_char_count: None,
        created_at_ms: timestamp_ms,
        claimed_at_ms: None,
        refunded_at_ms: None,
        time,
        transaction_id: event_id.to_string(),
    };
    Some(vec![
        SocialEventRow::PaidMessageEscrow(escrow),
        SocialEventRow::MessagingOrgOutboundSpend {
            payer: ev.payer,
            amount: ev.amount as i64,
            counterparty: Some(ev.recipient),
            activity_at_ms: timestamp_ms,
        },
    ])
}

fn process_payment_claimed_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    reply_char_count: Option<u32>,
) -> Option<Vec<SocialEventRow>> {
    let ev: PaymentClaimedEvent = common::deserialize_social_event_json(
        "message_log",
        "PaymentClaimed",
        event_id,
        data,
        "message_log PaymentClaimed JSON did not match PaymentClaimedEvent",
    )?;
    let timestamp_ms =
        common::chain_timestamp_ms(Some(ev.claimed_at_ms as i64), checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(timestamp_ms);
    let msg_content_id = content_id(&ev.group_id, ev.seq);
    let escrow = NewPaidMessageEscrow {
        group_id: ev.group_id.clone(),
        seq: ev.seq as i64,
        payer: String::new(),
        recipient: ev.recipient.clone(),
        amount: ev.amount as i64,
        status: PAID_MESSAGE_STATUS_CLAIMED.to_string(),
        platform_fee: None,
        treasury_fee: None,
        net_amount: Some(ev.amount as i64),
        platform_fee_recipient: None,
        ecosystem_fee_recipient: None,
        reply_char_count: reply_char_count.map(i64::from),
        created_at_ms: timestamp_ms,
        claimed_at_ms: Some(timestamp_ms),
        refunded_at_ms: None,
        time,
        transaction_id: event_id.to_string(),
    };
    let revenue = NewUnifiedRevenue::from_messaging_at_time(
        REVENUE_TYPE_MESSAGING_CLAIM.to_string(),
        ev.recipient.clone(),
        None,
        ev.amount as i64,
        msg_content_id,
        String::new(),
        ev.recipient.clone(),
        timestamp_ms,
        event_id.to_string(),
        time,
        None,
    );
    Some(vec![
        SocialEventRow::PaidMessageEscrow(escrow),
        SocialEventRow::UnifiedRevenue(revenue),
        SocialEventRow::MessagingOrgInboundRevenue {
            recipient: ev.recipient,
            amount: ev.amount as i64,
            counterparty: None,
            activity_at_ms: timestamp_ms,
        },
    ])
}

fn process_payment_claimed_settled_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    reply_char_count: Option<u32>,
) -> Option<Vec<SocialEventRow>> {
    let ev: PaymentClaimedSettledEvent = common::deserialize_social_event_json(
        "message_log",
        "PaymentClaimedSettled",
        event_id,
        data,
        "message_log PaymentClaimedSettled JSON did not match PaymentClaimedSettledEvent",
    )?;
    let timestamp_ms =
        common::chain_timestamp_ms(Some(ev.claimed_at_ms as i64), checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(timestamp_ms);
    let msg_content_id = content_id(&ev.group_id, ev.seq);
    let escrow = NewPaidMessageEscrow {
        group_id: ev.group_id.clone(),
        seq: ev.seq as i64,
        payer: String::new(),
        recipient: ev.recipient.clone(),
        amount: ev.total_amount as i64,
        status: PAID_MESSAGE_STATUS_SETTLED.to_string(),
        platform_fee: Some(ev.platform_fee as i64),
        treasury_fee: Some(ev.treasury_fee as i64),
        net_amount: Some(ev.net_amount as i64),
        platform_fee_recipient: Some(ev.platform_fee_recipient.clone()),
        ecosystem_fee_recipient: Some(ev.ecosystem_fee_recipient.clone()),
        reply_char_count: reply_char_count.map(i64::from),
        created_at_ms: timestamp_ms,
        claimed_at_ms: Some(timestamp_ms),
        refunded_at_ms: None,
        time,
        transaction_id: event_id.to_string(),
    };
    let mut rows = vec![SocialEventRow::PaidMessageEscrow(escrow)];
    let mut ur_time = time;
    if ev.net_amount != 0 {
        rows.push(SocialEventRow::UnifiedRevenue(
            NewUnifiedRevenue::from_messaging_at_time(
                REVENUE_TYPE_MESSAGING_NET.to_string(),
                ev.recipient.clone(),
                None,
                ev.net_amount as i64,
                msg_content_id.clone(),
                String::new(),
                ev.recipient.clone(),
                timestamp_ms,
                event_id.to_string(),
                ur_time,
                None,
            ),
        ));
        rows.push(SocialEventRow::MessagingOrgInboundRevenue {
            recipient: ev.recipient.clone(),
            amount: ev.net_amount as i64,
            counterparty: None,
            activity_at_ms: timestamp_ms,
        });
        ur_time += chrono::Duration::microseconds(1);
    }
    if ev.platform_fee != 0 {
        rows.push(SocialEventRow::UnifiedRevenue(
            NewUnifiedRevenue::from_messaging_at_time(
                REVENUE_TYPE_MESSAGING_PLATFORM_FEE.to_string(),
                ev.recipient.clone(),
                Some(ev.platform_fee_recipient.clone()),
                ev.platform_fee as i64,
                msg_content_id.clone(),
                String::new(),
                ev.platform_fee_recipient.clone(),
                timestamp_ms,
                event_id.to_string(),
                ur_time,
                None,
            ),
        ));
        ur_time += chrono::Duration::microseconds(1);
    }
    if ev.treasury_fee != 0 {
        rows.push(SocialEventRow::UnifiedRevenue(
            NewUnifiedRevenue::from_messaging_at_time(
                REVENUE_TYPE_MESSAGING_TREASURY_FEE.to_string(),
                ev.recipient.clone(),
                None,
                ev.treasury_fee as i64,
                msg_content_id,
                String::new(),
                ev.ecosystem_fee_recipient.clone(),
                timestamp_ms,
                event_id.to_string(),
                ur_time,
                None,
            ),
        ));
    }
    Some(rows)
}

fn process_payment_refunded_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: PaymentRefundedEvent = common::deserialize_social_event_json(
        "message_log",
        "PaymentRefunded",
        event_id,
        data,
        "message_log PaymentRefunded JSON did not match PaymentRefundedEvent",
    )?;
    let timestamp_ms =
        common::chain_timestamp_ms(Some(ev.refunded_at_ms as i64), checkpoint_timestamp_ms);
    let time = common::chain_time_from_ms(timestamp_ms);
    let msg_content_id = content_id(&ev.group_id, ev.seq);
    let escrow = NewPaidMessageEscrow {
        group_id: ev.group_id.clone(),
        seq: ev.seq as i64,
        payer: ev.payer.clone(),
        recipient: String::new(),
        amount: ev.amount as i64,
        status: PAID_MESSAGE_STATUS_REFUNDED.to_string(),
        platform_fee: None,
        treasury_fee: None,
        net_amount: None,
        platform_fee_recipient: None,
        ecosystem_fee_recipient: None,
        reply_char_count: None,
        created_at_ms: timestamp_ms,
        claimed_at_ms: None,
        refunded_at_ms: Some(timestamp_ms),
        time,
        transaction_id: event_id.to_string(),
    };
    let revenue = NewUnifiedRevenue::from_messaging_at_time(
        REVENUE_TYPE_MESSAGING_REFUND.to_string(),
        ev.payer.clone(),
        None,
        -(ev.amount as i64),
        msg_content_id,
        ev.payer.clone(),
        ev.payer.clone(),
        timestamp_ms,
        event_id.to_string(),
        time,
        None,
    );
    Some(vec![
        SocialEventRow::PaidMessageEscrow(escrow),
        SocialEventRow::UnifiedRevenue(revenue),
        SocialEventRow::MessagingOrgOutboundSpend {
            payer: ev.payer,
            amount: -(ev.amount as i64),
            counterparty: None,
            activity_at_ms: timestamp_ms,
        },
    ])
}

pub fn stash_paid_message_reply(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<(String, u32)> {
    let ev: PaidMessageRepliedEvent = common::deserialize_social_event_json(
        "message_log",
        "PaidMessageReplied",
        event_id,
        data,
        "message_log PaidMessageReplied JSON did not match PaidMessageRepliedEvent",
    )?;
    Some((content_id(&ev.group_id, ev.paid_msg_seq), ev.reply_char_count))
}

pub fn handle_paid_messaging_policy_event(
    data: &serde_json::Value,
    updated_at: i64,
) -> Option<NewWalletMessagingPolicy> {
    let wallet = data.get("wallet")?.as_str()?.to_string();
    let enabled = data.get("enabled")?.as_bool()?;
    let min_cost = data.get("min_cost").and_then(|v| {
        if v.is_null() {
            None
        } else {
            v.as_u64().map(|n| n as i64)
        }
    });
    Some(NewWalletMessagingPolicy {
        wallet_address: wallet,
        enabled,
        min_cost,
        updated_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paid_messaging_policy_updated_bcs_roundtrip() {
        use move_core_types::account_address::AccountAddress;

        let wallet = AccountAddress::from_hex_literal(
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
        )
        .unwrap();
        let ev = super::super::events::BcsPaidMessagingPolicyUpdated {
            wallet,
            enabled: true,
            min_cost: Some(1_000_000),
        };
        let bytes = bcs::to_bytes(&ev).unwrap();
        let json = super::super::events::parse_event_contents(
            "paid_messaging_policy",
            "PaidMessagingPolicyUpdated",
            &bytes,
        )
        .expect("BCS parse should succeed");
        let row = handle_paid_messaging_policy_event(&json, 42).expect("handler should accept");
        assert!(row.enabled);
        assert_eq!(row.min_cost, Some(1_000_000));
        assert_eq!(row.updated_at, 42);
        assert!(row.wallet_address.starts_with("0x"));
    }
}
