// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Unified messaging reference-layer indexer: wallet policy, config, paid messages, agent groups.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewMessageDigest, NewMessagingAgentGroup, NewMessagingConfig, NewPaidMessageEscrow, NewUnifiedRevenue,
    NewWalletMessagingPolicy,
};
use myso_indexer_alt_social_schema::schema::{
    message_digests, messaging_agent_groups, messaging_config, paid_message_escrows, unified_revenue,
    wallet_messaging_policies,
};

use super::common;
use super::events;
use super::messaging;
use super::organization_stats::{
    apply_org_outbound_spend, apply_org_revenue, increment_org_stat, init_org_stats,
    resolve_organization_id_for_derived_address, OrgStatColumn,
};
use super::SocialEventRow;

const PAID_MESSAGING_POLICY_MODULE: &str = "paid_messaging_policy";
const MESSAGING_CONFIG_MODULE: &str = "messaging_config";
const MESSAGE_LOG_MODULE: &str = "message_log";
const MESSAGING_MODULE: &str = "messaging";

#[derive(Debug, Clone)]
pub enum MessagingRow {
    WalletMessagingPolicy(NewWalletMessagingPolicy),
    MessagingConfig(NewMessagingConfig),
    PaidMessageEscrow(NewPaidMessageEscrow),
    MessageDigest(NewMessageDigest),
    MessagingAgentGroup(NewMessagingAgentGroup),
    UnifiedRevenue(NewUnifiedRevenue),
    OrgOutboundSpend {
        payer: String,
        amount: i64,
        counterparty: Option<String>,
        activity_at_ms: i64,
    },
    OrgInboundRevenue {
        recipient: String,
        amount: i64,
        counterparty: Option<String>,
        activity_at_ms: i64,
    },
    AgentGroupOrgActivity {
        organization_id: Option<String>,
        activity_at_ms: i64,
    },
}

impl FieldCount for MessagingRow {
    const FIELD_COUNT: usize = 22;
}

pub struct MessagingHandler;

#[async_trait]
impl Processor for MessagingHandler {
    const NAME: &'static str = "messaging";

    type Value = MessagingRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_timestamp_ms = checkpoint.summary.timestamp_ms;
        let checkpoint_timestamp_ms_i64 = checkpoint_timestamp_ms as i64;
        let mut values = Vec::new();
        for tx in &checkpoint.transactions {
            let tx_digest = tx.transaction.digest().to_string();
            let Some(events) = &tx.events else {
                continue;
            };
            let mut reply_char_counts: HashMap<String, u32> = HashMap::new();
            for (event_seq, ev) in events.data.iter().enumerate() {
                if !common::is_messaging_package_event(&ev.package_id, &ev.type_.address) {
                    continue;
                }
                let module = ev.type_.module.as_str();
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);

                if module == PAID_MESSAGING_POLICY_MODULE {
                    if event_name != "PaidMessagingPolicyUpdated" {
                        continue;
                    }
                    let event_data = match events::parse_event_contents(
                        PAID_MESSAGING_POLICY_MODULE,
                        "PaidMessagingPolicyUpdated",
                        &ev.contents,
                    ) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                    if let Some(row) = messaging::handle_paid_messaging_policy_event(
                        &event_data,
                        checkpoint_timestamp_ms_i64,
                    ) {
                        values.push(MessagingRow::WalletMessagingPolicy(row));
                    }
                    continue;
                }

                if module == MESSAGING_CONFIG_MODULE {
                    let event_data = match events::parse_event_contents(
                        module,
                        event_name,
                        &ev.contents,
                    ) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                    if let Some(rows) = messaging::handle_messaging_event(
                        event_name,
                        &event_data,
                        &event_id,
                        checkpoint_timestamp_ms,
                    ) {
                        for row in rows {
                            if let SocialEventRow::MessagingConfig(c) = row {
                                values.push(MessagingRow::MessagingConfig(c));
                            }
                        }
                    }
                    continue;
                }

                if module != MESSAGE_LOG_MODULE && module != MESSAGING_MODULE {
                    continue;
                }

                let event_data =
                    match events::parse_event_contents(module, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };

                if module == MESSAGE_LOG_MODULE && event_name == "PaidMessageReplied" {
                    if let Some((key, count)) =
                        messaging::stash_paid_message_reply(&event_data, &event_id)
                    {
                        reply_char_counts.insert(key, count);
                    }
                    continue;
                }

                let reply_char_count = if module == MESSAGE_LOG_MODULE
                    && matches!(event_name, "PaymentClaimed" | "PaymentClaimedSettled")
                {
                    event_data
                        .get("group_id")
                        .and_then(|v| v.as_str())
                        .zip(event_data.get("seq").and_then(|v| v.as_u64()))
                        .and_then(|(group_id, seq)| {
                            reply_char_counts.remove(&format!("{group_id}:{seq}"))
                        })
                } else {
                    None
                };

                let rows = if module == MESSAGE_LOG_MODULE {
                    messaging::handle_message_log_event(
                        event_name,
                        &event_data,
                        &event_id,
                        checkpoint_timestamp_ms,
                        reply_char_count,
                    )
                } else {
                    messaging::handle_messaging_event(
                        event_name,
                        &event_data,
                        &event_id,
                        checkpoint_timestamp_ms,
                    )
                };
                if let Some(rows) = rows {
                    for row in rows {
                        values.extend(convert_social_event_row(row));
                    }
                }
            }
        }
        Ok(values)
    }
}

fn convert_social_event_row(row: SocialEventRow) -> Vec<MessagingRow> {
    match row {
        SocialEventRow::PaidMessageEscrow(e) => vec![MessagingRow::PaidMessageEscrow(e)],
        SocialEventRow::MessageDigest(e) => vec![MessagingRow::MessageDigest(e)],
        SocialEventRow::MessagingAgentGroup(g) => vec![MessagingRow::MessagingAgentGroup(g)],
        SocialEventRow::UnifiedRevenue(r) => vec![MessagingRow::UnifiedRevenue(r)],
        SocialEventRow::MessagingOrgOutboundSpend {
            payer,
            amount,
            counterparty,
            activity_at_ms,
        } => vec![MessagingRow::OrgOutboundSpend {
            payer,
            amount,
            counterparty,
            activity_at_ms,
        }],
        SocialEventRow::MessagingOrgInboundRevenue {
            recipient,
            amount,
            counterparty,
            activity_at_ms,
        } => vec![MessagingRow::OrgInboundRevenue {
            recipient,
            amount,
            counterparty,
            activity_at_ms,
        }],
        SocialEventRow::MessagingAgentGroupOrgActivity {
            organization_id,
            activity_at_ms,
        } => vec![MessagingRow::AgentGroupOrgActivity {
            organization_id,
            activity_at_ms,
        }],
        _ => Vec::new(),
    }
}

#[async_trait]
impl Handler for MessagingHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                MessagingRow::WalletMessagingPolicy(policy) => {
                    let enabled = policy.enabled;
                    let min_cost = policy.min_cost;
                    let updated_at = policy.updated_at;
                    total += diesel::insert_into(wallet_messaging_policies::table)
                        .values(policy)
                        .on_conflict(wallet_messaging_policies::wallet_address)
                        .do_update()
                        .set((
                            wallet_messaging_policies::enabled.eq(enabled),
                            wallet_messaging_policies::min_cost.eq(min_cost),
                            wallet_messaging_policies::updated_at.eq(updated_at),
                        ))
                        .execute(conn)
                        .await?;
                }
                MessagingRow::MessagingConfig(c) => {
                    total += diesel::insert_into(messaging_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                MessagingRow::PaidMessageEscrow(e) => {
                    total += diesel::insert_into(paid_message_escrows::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                MessagingRow::MessageDigest(e) => {
                    total += diesel::insert_into(message_digests::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                MessagingRow::MessagingAgentGroup(g) => {
                    total += diesel::insert_into(messaging_agent_groups::table)
                        .values(g)
                        .execute(conn)
                        .await?;
                }
                MessagingRow::UnifiedRevenue(r) => {
                    total += diesel::insert_into(unified_revenue::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                MessagingRow::OrgOutboundSpend {
                    payer,
                    amount,
                    counterparty,
                    activity_at_ms,
                } => {
                    let org_id = resolve_organization_id_for_derived_address(conn, payer).await?;
                    apply_org_outbound_spend(
                        conn,
                        org_id.as_deref(),
                        *amount,
                        counterparty.as_deref(),
                        *activity_at_ms,
                    )
                    .await?;
                }
                MessagingRow::OrgInboundRevenue {
                    recipient,
                    amount,
                    counterparty,
                    activity_at_ms,
                } => {
                    let org_id =
                        resolve_organization_id_for_derived_address(conn, recipient).await?;
                    apply_org_revenue(
                        conn,
                        org_id.as_deref(),
                        *amount,
                        counterparty.as_deref(),
                        *activity_at_ms,
                    )
                    .await?;
                }
                MessagingRow::AgentGroupOrgActivity {
                    organization_id,
                    activity_at_ms,
                } => {
                    if let Some(org_id) = organization_id {
                        init_org_stats(conn, org_id, *activity_at_ms).await?;
                        increment_org_stat(
                            conn,
                            org_id,
                            OrgStatColumn::TotalActionsExecuted,
                            1,
                            *activity_at_ms,
                        )
                        .await?;
                    }
                }
            }
        }
        Ok(total)
    }
}
