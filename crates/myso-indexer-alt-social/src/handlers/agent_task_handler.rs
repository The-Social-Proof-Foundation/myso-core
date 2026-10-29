// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Task market pipeline: indexes every `agent_task` event into `agent_tasks`, `task_receipts`,
//! `task_reviews` and `task_config_history`.
//!
//! This is a **sequential** pipeline so a task's events apply in chain order: each lifecycle
//! event is a plain `UPDATE` guarded by the status it expects, and a replayed checkpoint cannot
//! move a task backwards.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    build_task_events, NewAgentTask, NewAutomationEvent, NewTaskConfig, NewTaskDispute,
    NewTaskReceipt, NewTaskReview, TaskEventKind, TaskSnapshot, DISPUTE_STATE_CANCELLED,
    DISPUTE_STATE_OPEN, DISPUTE_STATE_RESOLVED, TASK_STATUS_ACCEPTED, TASK_STATUS_CANCELLED,
    TASK_STATUS_DELIVERED, TASK_STATUS_DISPUTED, TASK_STATUS_OFFERED, TASK_STATUS_REFUNDED,
    TASK_STATUS_SETTLED,
};
use myso_indexer_alt_social_schema::schema::{
    agent_tasks, automation_event_outbox, task_config_history, task_disputes, task_receipts,
    task_reviews,
};

use super::common;
use super::events;
use crate::metrics::SocialMetrics;

const AGENT_TASK_MODULE: &str = "agent_task";

/// Every event `agent_task.move` emits. A Move event missing here is caught by
/// `every_agent_task_event_is_handled`.
pub const AGENT_TASK_EVENTS: &[&str] = &[
    "TaskOffered",
    "TaskAccepted",
    "TaskCancelled",
    "TaskDelivered",
    "TaskRefundProposed",
    "TaskRefundProposalCleared",
    "TaskRefunded",
    "TaskReceiptSettled",
    "TaskReviewSubmitted",
    "TaskConfigUpdated",
    "TaskFeeRecipientChanged",
    "TaskDisputed",
    "TaskDisputeFeePaid",
    "TaskDisputeCancelled",
    "TaskNeutralWindowOpened",
    "TaskDisputeSettlementProposed",
    "TaskDisputeSettlementCleared",
    "TaskDisputeResolved",
    "TaskDisputeFeesSettled",
];

const REVIEW_ROLE_BUYER: i16 = 1;

/// Copies the latest config with the new recipient. Binds: updated_by, recipient, timestamp_ms,
/// event_id, transaction_id, time.
const FEE_RECIPIENT_CHANGED_SQL: &str = "
    INSERT INTO task_config_history (
        updated_by, platform_fee_bps, ecosystem_fee_bps, ecosystem_fee_recipient, min_escrow_mist,
        accept_window_min_ms, accept_window_default_ms, accept_window_max_ms,
        delivery_window_min_ms, delivery_window_max_ms, settle_window_min_ms, settle_window_max_ms,
        release_grace_ms, review_window_ms, max_title_bytes, max_uri_bytes, active,
        prior_mean_milli, prior_weight_bps, value_ref_mist, min_value_weight_bps,
        max_value_weight_bps, cred_base_bps, cred_step_bps, cred_cap_n, cred_max_bps,
        overall_seller_weight_bps, overall_buyer_weight_bps, timestamp_ms, event_id,
        transaction_id, time
    )
    SELECT
        $1, platform_fee_bps, ecosystem_fee_bps, $2, min_escrow_mist,
        accept_window_min_ms, accept_window_default_ms, accept_window_max_ms,
        delivery_window_min_ms, delivery_window_max_ms, settle_window_min_ms, settle_window_max_ms,
        release_grace_ms, review_window_ms, max_title_bytes, max_uri_bytes, active,
        prior_mean_milli, prior_weight_bps, value_ref_mist, min_value_weight_bps,
        max_value_weight_bps, cred_base_bps, cred_step_bps, cred_cap_n, cred_max_bps,
        overall_seller_weight_bps, overall_buyer_weight_bps, $3, $4, $5, $6
    FROM task_config_history
    ORDER BY timestamp_ms DESC, id DESC
    LIMIT 1
    ON CONFLICT (event_id) DO NOTHING";

#[derive(Debug, Clone)]
pub enum TaskMarketRow {
    Offered(NewAgentTask),
    Accepted {
        task_id: String,
        accepted_at_ms: i64,
        delivery_deadline_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Delivered {
        task_id: String,
        deliverable_digest: String,
        deliverable_uri: String,
        delivered_at_ms: i64,
        settle_deadline_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    RefundProposed {
        task_id: String,
        proposer_role: i16,
        proposed_at_ms: i64,
        event_id: String,
    },
    ProposalCleared {
        task_id: String,
        cleared_at_ms: i64,
        event_id: String,
    },
    Settled {
        task_id: String,
        settlement_kind: i16,
        platform_fee_mist: i64,
        ecosystem_fee_mist: i64,
        net_amount_mist: i64,
        buyer_credibility_bps: i64,
        seller_credibility_bps: i64,
        settle_deadline_ms: i64,
        review_deadline_ms: i64,
        settled_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Receipt(NewTaskReceipt),
    Refunded {
        task_id: String,
        refund_kind: i16,
        refunded_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Cancelled {
        task_id: String,
        close_kind: i16,
        cancelled_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Review(NewTaskReview),
    Config(NewTaskConfig),
    /// Only the ecosystem fee recipient changed. Committed as a copy of the latest config row so
    /// `task_config_history` keeps one full snapshot per change.
    FeeRecipientChanged {
        new_recipient: String,
        updated_by: String,
        timestamp_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Disputed(NewTaskDispute),
    DisputeFeePaid {
        task_id: String,
        buyer_paid_mist: i64,
        seller_paid_mist: i64,
    },
    DisputeCancelled {
        task_id: String,
        settle_deadline_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    NeutralWindowOpened {
        task_id: String,
        window_ends_ms: i64,
        opened_at_ms: i64,
        event_id: String,
    },
    DisputeSettlementProposed {
        task_id: String,
        outcome: i16,
        proposer_role: i16,
    },
    DisputeSettlementCleared {
        task_id: String,
    },
    DisputeResolved {
        task_id: String,
        outcome: i16,
        source: i16,
        resolved_at_ms: i64,
        event_id: String,
    },
    DisputeFeesSettled {
        task_id: String,
        buyer_refund_mist: i64,
        seller_refund_mist: i64,
    },
}

impl FieldCount for TaskMarketRow {
    const FIELD_COUNT: usize = 33;
}

fn str_field(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| v.as_str()).map(String::from)
}

fn opt_str_field(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key)
        .filter(|v| !v.is_null())
        .and_then(|v| v.as_str())
        .map(String::from)
}

fn i64_field(data: &serde_json::Value, key: &str) -> Option<i64> {
    let v = data.get(key)?;
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| i64::try_from(u).ok()))
}

fn bool_field(data: &serde_json::Value, key: &str) -> Option<bool> {
    data.get(key).and_then(|v| v.as_bool())
}

/// Pure translation of one parsed event into rows. `None` means the payload did not match.
pub fn task_market_rows(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<TaskMarketRow>> {
    let transaction_id = event_id.split(':').next().unwrap_or(event_id).to_string();
    let event_id = event_id.to_string();
    match event_name {
        "TaskOffered" => {
            let created_at_ms = i64_field(data, "created_at_ms")?;
            Some(vec![TaskMarketRow::Offered(NewAgentTask {
                task_id: str_field(data, "task_id")?,
                buyer_principal: str_field(data, "buyer_principal")?,
                buyer_agent_id: opt_str_field(data, "buyer_agent_id"),
                buyer_memory_account_id: str_field(data, "buyer_memory_account_id")?,
                seller_principal: str_field(data, "seller_principal")?,
                seller_agent_id: str_field(data, "seller_agent_id")?,
                seller_memory_account_id: str_field(data, "seller_memory_account_id")?,
                organization_id: opt_str_field(data, "organization_id"),
                room_group_id: opt_str_field(data, "room_group_id"),
                platform_id: opt_str_field(data, "platform_id"),
                title: str_field(data, "title")?,
                scope_digest: str_field(data, "scope_digest")?,
                amount_mist: i64_field(data, "amount_mist")?,
                accept_window_ms: i64_field(data, "accept_window_ms")?,
                delivery_window_ms: i64_field(data, "delivery_window_ms")?,
                settle_window_ms: i64_field(data, "settle_window_ms")?,
                platform_fee_bps: i64_field(data, "platform_fee_bps")?,
                ecosystem_fee_bps: i64_field(data, "ecosystem_fee_bps")?,
                terms_digest: str_field(data, "terms_digest")?,
                status: TASK_STATUS_OFFERED,
                created_at_ms,
                accept_deadline_ms: i64_field(data, "accept_deadline_ms")?,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(created_at_ms),
            })])
        }
        "TaskAccepted" => Some(vec![TaskMarketRow::Accepted {
            task_id: str_field(data, "task_id")?,
            accepted_at_ms: i64_field(data, "accepted_at_ms")?,
            delivery_deadline_ms: i64_field(data, "delivery_deadline_ms")?,
            event_id,
            transaction_id,
        }]),
        "TaskDelivered" => Some(vec![TaskMarketRow::Delivered {
            task_id: str_field(data, "task_id")?,
            deliverable_digest: str_field(data, "deliverable_digest")?,
            deliverable_uri: str_field(data, "deliverable_uri")?,
            delivered_at_ms: i64_field(data, "delivered_at_ms")?,
            settle_deadline_ms: i64_field(data, "settle_deadline_ms")?,
            event_id,
            transaction_id,
        }]),
        "TaskRefundProposed" => Some(vec![TaskMarketRow::RefundProposed {
            task_id: str_field(data, "task_id")?,
            proposer_role: i64_field(data, "proposer_role")? as i16,
            proposed_at_ms: i64_field(data, "proposed_at_ms")?,
            event_id,
        }]),
        "TaskRefundProposalCleared" => Some(vec![TaskMarketRow::ProposalCleared {
            task_id: str_field(data, "task_id")?,
            cleared_at_ms: i64_field(data, "cleared_at_ms")?,
            event_id,
        }]),
        "TaskRefunded" => Some(vec![TaskMarketRow::Refunded {
            task_id: str_field(data, "task_id")?,
            refund_kind: i64_field(data, "refund_kind")? as i16,
            refunded_at_ms: i64_field(data, "refunded_at_ms")?,
            event_id,
            transaction_id,
        }]),
        "TaskCancelled" => Some(vec![TaskMarketRow::Cancelled {
            task_id: str_field(data, "task_id")?,
            close_kind: i64_field(data, "close_kind")? as i16,
            cancelled_at_ms: i64_field(data, "cancelled_at_ms")?,
            event_id,
            transaction_id,
        }]),
        "TaskReceiptSettled" => {
            let task_id = str_field(data, "task_id")?;
            let settled_at_ms = i64_field(data, "settled_at_ms")?;
            let settlement_kind = i64_field(data, "settlement_kind")? as i16;
            let platform_fee_mist = i64_field(data, "platform_fee_mist")?;
            let ecosystem_fee_mist = i64_field(data, "ecosystem_fee_mist")?;
            let net_amount_mist = i64_field(data, "net_amount_mist")?;
            let buyer_credibility_bps = i64_field(data, "buyer_credibility_bps")?;
            let seller_credibility_bps = i64_field(data, "seller_credibility_bps")?;
            let settle_deadline_ms = i64_field(data, "settle_deadline_ms")?;
            let review_deadline_ms = i64_field(data, "review_deadline_ms")?;
            let receipt = NewTaskReceipt {
                task_id: task_id.clone(),
                receipt_id: str_field(data, "receipt_id")?,
                buyer_principal: str_field(data, "buyer_principal")?,
                buyer_agent_id: opt_str_field(data, "buyer_agent_id"),
                seller_principal: str_field(data, "seller_principal")?,
                seller_agent_id: str_field(data, "seller_agent_id")?,
                organization_id: opt_str_field(data, "organization_id"),
                room_group_id: opt_str_field(data, "room_group_id"),
                settlement_kind,
                amount_mist: i64_field(data, "amount_mist")?,
                platform_fee_mist,
                ecosystem_fee_mist,
                net_amount_mist,
                deliverable_digest: opt_str_field(data, "deliverable_digest"),
                deliverable_uri: opt_str_field(data, "deliverable_uri"),
                buyer_credibility_bps,
                seller_credibility_bps,
                settle_deadline_ms,
                review_deadline_ms,
                settled_at_ms,
                event_id: event_id.clone(),
                transaction_id: transaction_id.clone(),
                time: common::chain_time_from_ms(settled_at_ms),
            };
            Some(vec![
                TaskMarketRow::Settled {
                    task_id,
                    settlement_kind,
                    platform_fee_mist,
                    ecosystem_fee_mist,
                    net_amount_mist,
                    buyer_credibility_bps,
                    seller_credibility_bps,
                    settle_deadline_ms,
                    review_deadline_ms,
                    settled_at_ms,
                    event_id,
                    transaction_id,
                },
                TaskMarketRow::Receipt(receipt),
            ])
        }
        "TaskReviewSubmitted" => {
            let reviewed_at_ms = i64_field(data, "reviewed_at_ms")?;
            Some(vec![TaskMarketRow::Review(NewTaskReview {
                task_id: str_field(data, "task_id")?,
                rater_role: i64_field(data, "rater_role")? as i16,
                receipt_id: str_field(data, "receipt_id")?,
                rater_principal: str_field(data, "rater_principal")?,
                rater_agent_id: opt_str_field(data, "rater_agent_id"),
                reviewee_principal: str_field(data, "reviewee_principal")?,
                reviewee_agent_id: opt_str_field(data, "reviewee_agent_id"),
                stars: i64_field(data, "stars")? as i16,
                rater_credibility_bps: i64_field(data, "rater_credibility_bps")?,
                value_weight_bps: i64_field(data, "value_weight_bps")?,
                base_weight_bps: i64_field(data, "base_weight_bps")?,
                prior_pair_reviews: i64_field(data, "prior_pair_reviews")?,
                weight_bps: i64_field(data, "weight_bps")?,
                reviewed_at_ms,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(reviewed_at_ms),
            })])
        }
        "TaskConfigUpdated" => {
            let timestamp_ms = i64_field(data, "timestamp_ms")?;
            Some(vec![TaskMarketRow::Config(NewTaskConfig {
                updated_by: str_field(data, "updated_by")?,
                platform_fee_bps: i64_field(data, "platform_fee_bps")?,
                ecosystem_fee_bps: i64_field(data, "ecosystem_fee_bps")?,
                ecosystem_fee_recipient: str_field(data, "ecosystem_fee_recipient")?,
                min_escrow_mist: i64_field(data, "min_escrow_mist")?,
                accept_window_min_ms: i64_field(data, "accept_window_min_ms")?,
                accept_window_default_ms: i64_field(data, "accept_window_default_ms")?,
                accept_window_max_ms: i64_field(data, "accept_window_max_ms")?,
                delivery_window_min_ms: i64_field(data, "delivery_window_min_ms")?,
                delivery_window_max_ms: i64_field(data, "delivery_window_max_ms")?,
                settle_window_min_ms: i64_field(data, "settle_window_min_ms")?,
                settle_window_max_ms: i64_field(data, "settle_window_max_ms")?,
                release_grace_ms: i64_field(data, "release_grace_ms")?,
                review_window_ms: i64_field(data, "review_window_ms")?,
                max_title_bytes: i64_field(data, "max_title_bytes")?,
                max_uri_bytes: i64_field(data, "max_uri_bytes")?,
                active: bool_field(data, "active")?,
                prior_mean_milli: i64_field(data, "prior_mean_milli")?,
                prior_weight_bps: i64_field(data, "prior_weight_bps")?,
                value_ref_mist: i64_field(data, "value_ref_mist")?,
                min_value_weight_bps: i64_field(data, "min_value_weight_bps")?,
                max_value_weight_bps: i64_field(data, "max_value_weight_bps")?,
                cred_base_bps: i64_field(data, "cred_base_bps")?,
                cred_step_bps: i64_field(data, "cred_step_bps")?,
                cred_cap_n: i64_field(data, "cred_cap_n")?,
                cred_max_bps: i64_field(data, "cred_max_bps")?,
                overall_seller_weight_bps: i64_field(data, "overall_seller_weight_bps")?,
                overall_buyer_weight_bps: i64_field(data, "overall_buyer_weight_bps")?,
                timestamp_ms,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(timestamp_ms),
            })])
        }
        "TaskFeeRecipientChanged" => Some(vec![TaskMarketRow::FeeRecipientChanged {
            new_recipient: str_field(data, "new_recipient")?,
            updated_by: str_field(data, "updated_by")?,
            timestamp_ms: i64_field(data, "updated_at_ms")?,
            event_id,
            transaction_id,
        }]),
        "TaskDisputed" => {
            let opened_at_ms = i64_field(data, "opened_at_ms")?;
            let opened_by_role = i64_field(data, "opened_by_role")? as i16;
            let opener_paid_mist = i64_field(data, "opener_paid_mist")?;
            // The opener's payment is the only one on record until the other side matches it.
            let (buyer_paid_mist, seller_paid_mist) = if opened_by_role == REVIEW_ROLE_BUYER {
                (opener_paid_mist, 0)
            } else {
                (0, opener_paid_mist)
            };
            Some(vec![TaskMarketRow::Disputed(NewTaskDispute {
                task_id: str_field(data, "task_id")?,
                case_id: str_field(data, "case_id")?,
                opened_by_role,
                state: DISPUTE_STATE_OPEN,
                fee_each_mist: i64_field(data, "fee_each_mist")?,
                buyer_paid_mist,
                seller_paid_mist,
                opened_at_ms,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(opened_at_ms),
            })])
        }
        "TaskDisputeFeePaid" => Some(vec![TaskMarketRow::DisputeFeePaid {
            task_id: str_field(data, "task_id")?,
            buyer_paid_mist: i64_field(data, "buyer_paid_mist")?,
            seller_paid_mist: i64_field(data, "seller_paid_mist")?,
        }]),
        "TaskDisputeCancelled" => Some(vec![TaskMarketRow::DisputeCancelled {
            task_id: str_field(data, "task_id")?,
            settle_deadline_ms: i64_field(data, "settle_deadline_ms")?,
            event_id,
            transaction_id,
        }]),
        "TaskNeutralWindowOpened" => Some(vec![TaskMarketRow::NeutralWindowOpened {
            task_id: str_field(data, "task_id")?,
            window_ends_ms: i64_field(data, "window_ends_ms")?,
            opened_at_ms: i64_field(data, "opened_at_ms")?,
            event_id,
        }]),
        "TaskDisputeSettlementProposed" => Some(vec![TaskMarketRow::DisputeSettlementProposed {
            task_id: str_field(data, "task_id")?,
            outcome: i64_field(data, "outcome")? as i16,
            proposer_role: i64_field(data, "proposer_role")? as i16,
        }]),
        "TaskDisputeSettlementCleared" => Some(vec![TaskMarketRow::DisputeSettlementCleared {
            task_id: str_field(data, "task_id")?,
        }]),
        "TaskDisputeResolved" => Some(vec![TaskMarketRow::DisputeResolved {
            task_id: str_field(data, "task_id")?,
            outcome: i64_field(data, "outcome")? as i16,
            source: i64_field(data, "source")? as i16,
            resolved_at_ms: i64_field(data, "resolved_at_ms")?,
            event_id,
        }]),
        "TaskDisputeFeesSettled" => Some(vec![TaskMarketRow::DisputeFeesSettled {
            task_id: str_field(data, "task_id")?,
            buyer_refund_mist: i64_field(data, "buyer_refund_mist")?,
            seller_refund_mist: i64_field(data, "seller_refund_mist")?,
        }]),
        _ => None,
    }
}

pub struct TaskMarketHandler;

#[async_trait]
impl Processor for TaskMarketHandler {
    const NAME: &'static str = "agent_task_market";

    type Value = TaskMarketRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let mut values = Vec::new();
        for tx in &checkpoint.transactions {
            let tx_digest = tx.transaction.digest().to_string();
            let Some(events) = &tx.events else {
                continue;
            };
            for (event_seq, ev) in events.data.iter().enumerate() {
                if !common::is_social_package_event(&ev.package_id, &ev.type_.address) {
                    continue;
                }
                if ev.type_.module.as_str() != AGENT_TASK_MODULE {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                if !AGENT_TASK_EVENTS.contains(&event_name) {
                    continue;
                }
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let data =
                    match events::parse_event_contents(AGENT_TASK_MODULE, event_name, &ev.contents)
                    {
                        Ok(v) => v,
                        Err(e) => {
                            tracing::warn!(
                                tx_digest = %tx_digest,
                                event_name,
                                error = %e,
                                "agent_task: event parse failed; skipping"
                            );
                            SocialMetrics::record_event_bcs_parse_failed(
                                AGENT_TASK_MODULE,
                                event_name,
                            );
                            continue;
                        }
                    };
                match task_market_rows(event_name, &data, &event_id) {
                    Some(rows) => values.extend(rows),
                    None => tracing::warn!(
                        tx_digest = %tx_digest,
                        event_name,
                        "agent_task: event JSON did not match its row builder"
                    ),
                }
            }
        }
        Ok(values)
    }
}

/// Queue the automation events a task update produced, in the transaction that applied it.
///
/// The snapshot is read back after the update, so the payload reflects the task as the chain
/// now has it. A conflict on the deduplication key means a replayed checkpoint already queued it.
async fn enqueue_events<'a>(
    conn: &mut Connection<'a>,
    task_id: &str,
    kind: TaskEventKind,
    occurred_at_ms: i64,
    source_event_id: &str,
) -> Result<usize> {
    let Some(task) = agent_tasks::table
        .filter(agent_tasks::task_id.eq(task_id))
        .select(TaskSnapshot::as_select())
        .first::<TaskSnapshot>(conn)
        .await
        .optional()?
    else {
        return Ok(0);
    };
    let events = build_task_events(kind, &task, occurred_at_ms, source_event_id);
    if events.is_empty() {
        return Ok(0);
    }
    Ok(diesel::insert_into(automation_event_outbox::table)
        .values(&events)
        .on_conflict(automation_event_outbox::dedup_key)
        .do_nothing()
        .execute(conn)
        .await?)
}

/// Like `enqueue_events`, with `extra` merged into each payload for facts the task row does not
/// hold (the dispute's case and who opened it).
async fn enqueue_events_with<'a>(
    conn: &mut Connection<'a>,
    task_id: &str,
    kind: TaskEventKind,
    occurred_at_ms: i64,
    source_event_id: &str,
    extra: serde_json::Value,
) -> Result<usize> {
    let Some(task) = agent_tasks::table
        .filter(agent_tasks::task_id.eq(task_id))
        .select(TaskSnapshot::as_select())
        .first::<TaskSnapshot>(conn)
        .await
        .optional()?
    else {
        return Ok(0);
    };
    let mut events: Vec<NewAutomationEvent> =
        build_task_events(kind, &task, occurred_at_ms, source_event_id);
    if let serde_json::Value::Object(extra) = extra {
        for event in &mut events {
            if let Some(payload) = event.payload.as_object_mut() {
                payload.extend(extra.clone());
            }
        }
    }
    if events.is_empty() {
        return Ok(0);
    }
    Ok(diesel::insert_into(automation_event_outbox::table)
        .values(&events)
        .on_conflict(automation_event_outbox::dedup_key)
        .do_nothing()
        .execute(conn)
        .await?)
}

#[async_trait]
impl Handler for TaskMarketHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                TaskMarketRow::Offered(task) => {
                    let inserted = diesel::insert_into(agent_tasks::table)
                        .values(task)
                        .on_conflict(agent_tasks::task_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    total += inserted;
                    if inserted > 0 {
                        total += enqueue_events(
                            conn,
                            &task.task_id,
                            TaskEventKind::Offered,
                            task.created_at_ms,
                            &task.event_id,
                        )
                        .await?;
                    }
                }
                TaskMarketRow::Accepted {
                    task_id,
                    accepted_at_ms,
                    delivery_deadline_ms,
                    event_id,
                    transaction_id,
                } => {
                    let changed = diesel::update(
                        agent_tasks::table
                            .filter(agent_tasks::task_id.eq(task_id))
                            .filter(agent_tasks::status.eq(TASK_STATUS_OFFERED)),
                    )
                    .set((
                        agent_tasks::status.eq(TASK_STATUS_ACCEPTED),
                        agent_tasks::accepted_at_ms.eq(*accepted_at_ms),
                        agent_tasks::delivery_deadline_ms.eq(*delivery_deadline_ms),
                        agent_tasks::event_id.eq(event_id),
                        agent_tasks::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events(
                            conn,
                            task_id,
                            TaskEventKind::Accepted,
                            *accepted_at_ms,
                            event_id,
                        )
                        .await?;
                    }
                }
                TaskMarketRow::Delivered {
                    task_id,
                    deliverable_digest,
                    deliverable_uri,
                    delivered_at_ms,
                    settle_deadline_ms,
                    event_id,
                    transaction_id,
                } => {
                    let changed = diesel::update(
                        agent_tasks::table
                            .filter(agent_tasks::task_id.eq(task_id))
                            .filter(agent_tasks::status.eq(TASK_STATUS_ACCEPTED)),
                    )
                    .set((
                        agent_tasks::status.eq(TASK_STATUS_DELIVERED),
                        agent_tasks::deliverable_digest.eq(deliverable_digest),
                        agent_tasks::deliverable_uri.eq(deliverable_uri),
                        agent_tasks::delivered_at_ms.eq(*delivered_at_ms),
                        agent_tasks::settle_deadline_ms.eq(*settle_deadline_ms),
                        agent_tasks::event_id.eq(event_id),
                        agent_tasks::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events(
                            conn,
                            task_id,
                            TaskEventKind::Delivered,
                            *delivered_at_ms,
                            event_id,
                        )
                        .await?;
                    }
                }
                TaskMarketRow::RefundProposed {
                    task_id,
                    proposer_role,
                    proposed_at_ms,
                    event_id,
                } => {
                    let changed = diesel::update(
                        agent_tasks::table
                            .filter(agent_tasks::task_id.eq(task_id))
                            .filter(agent_tasks::status.eq(TASK_STATUS_DELIVERED)),
                    )
                    .set((
                        agent_tasks::refund_proposer.eq(*proposer_role),
                        agent_tasks::refund_proposed_at_ms.eq(*proposed_at_ms),
                    ))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events(
                            conn,
                            task_id,
                            TaskEventKind::RefundProposed,
                            *proposed_at_ms,
                            event_id,
                        )
                        .await?;
                    }
                }
                TaskMarketRow::ProposalCleared {
                    task_id,
                    cleared_at_ms,
                    event_id,
                } => {
                    let changed = diesel::update(
                        agent_tasks::table
                            .filter(agent_tasks::task_id.eq(task_id))
                            .filter(agent_tasks::status.eq(TASK_STATUS_DELIVERED)),
                    )
                    .set((
                        agent_tasks::refund_proposer.eq(0i16),
                        agent_tasks::refund_proposed_at_ms.eq(None::<i64>),
                    ))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events(
                            conn,
                            task_id,
                            TaskEventKind::RefundCleared,
                            *cleared_at_ms,
                            event_id,
                        )
                        .await?;
                    }
                }
                TaskMarketRow::Settled {
                    task_id,
                    settlement_kind,
                    platform_fee_mist,
                    ecosystem_fee_mist,
                    net_amount_mist,
                    buyer_credibility_bps,
                    seller_credibility_bps,
                    settle_deadline_ms,
                    review_deadline_ms,
                    settled_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    let changed = diesel::update(
                        agent_tasks::table
                            .filter(agent_tasks::task_id.eq(task_id))
                            // A dispute pays out from DISPUTED without passing through DELIVERED.
                            .filter(
                                agent_tasks::status
                                    .eq(TASK_STATUS_DELIVERED)
                                    .or(agent_tasks::status.eq(TASK_STATUS_DISPUTED)),
                            ),
                    )
                    .set((
                        agent_tasks::status.eq(TASK_STATUS_SETTLED),
                        agent_tasks::settlement_kind.eq(*settlement_kind),
                        agent_tasks::platform_fee_mist.eq(*platform_fee_mist),
                        agent_tasks::ecosystem_fee_mist.eq(*ecosystem_fee_mist),
                        agent_tasks::net_amount_mist.eq(*net_amount_mist),
                        agent_tasks::buyer_credibility_bps.eq(*buyer_credibility_bps),
                        agent_tasks::seller_credibility_bps.eq(*seller_credibility_bps),
                        agent_tasks::settle_deadline_ms.eq(*settle_deadline_ms),
                        agent_tasks::review_deadline_ms.eq(*review_deadline_ms),
                        agent_tasks::settled_at_ms.eq(*settled_at_ms),
                        // A pending refund proposal is void once the task is paid.
                        agent_tasks::refund_proposer.eq(0i16),
                        agent_tasks::refund_proposed_at_ms.eq(None::<i64>),
                        agent_tasks::event_id.eq(event_id),
                        agent_tasks::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events(
                            conn,
                            task_id,
                            TaskEventKind::Settled,
                            *settled_at_ms,
                            event_id,
                        )
                        .await?;
                    }
                }
                TaskMarketRow::Receipt(receipt) => {
                    total += diesel::insert_into(task_receipts::table)
                        .values(receipt)
                        .on_conflict(task_receipts::task_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                TaskMarketRow::Refunded {
                    task_id,
                    refund_kind,
                    refunded_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    let changed = diesel::update(
                        agent_tasks::table
                            .filter(agent_tasks::task_id.eq(task_id))
                            .filter(
                                agent_tasks::status
                                    .lt(TASK_STATUS_SETTLED)
                                    .or(agent_tasks::status.eq(TASK_STATUS_DISPUTED)),
                            ),
                    )
                    .set((
                        agent_tasks::status.eq(TASK_STATUS_REFUNDED),
                        agent_tasks::refund_kind.eq(*refund_kind),
                        agent_tasks::refunded_at_ms.eq(*refunded_at_ms),
                        agent_tasks::refund_proposer.eq(0i16),
                        agent_tasks::refund_proposed_at_ms.eq(None::<i64>),
                        agent_tasks::event_id.eq(event_id),
                        agent_tasks::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events(
                            conn,
                            task_id,
                            TaskEventKind::Refunded,
                            *refunded_at_ms,
                            event_id,
                        )
                        .await?;
                    }
                }
                TaskMarketRow::Cancelled {
                    task_id,
                    close_kind,
                    cancelled_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    let changed = diesel::update(
                        agent_tasks::table
                            .filter(agent_tasks::task_id.eq(task_id))
                            .filter(agent_tasks::status.eq(TASK_STATUS_OFFERED)),
                    )
                    .set((
                        agent_tasks::status.eq(TASK_STATUS_CANCELLED),
                        agent_tasks::close_kind.eq(*close_kind),
                        agent_tasks::cancelled_at_ms.eq(*cancelled_at_ms),
                        agent_tasks::event_id.eq(event_id),
                        agent_tasks::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events(
                            conn,
                            task_id,
                            TaskEventKind::Cancelled,
                            *cancelled_at_ms,
                            event_id,
                        )
                        .await?;
                    }
                }
                TaskMarketRow::Review(review) => {
                    let inserted = diesel::insert_into(task_reviews::table)
                        .values(review)
                        .on_conflict((task_reviews::task_id, task_reviews::rater_role))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    total += inserted;
                    let target =
                        agent_tasks::table.filter(agent_tasks::task_id.eq(&review.task_id));
                    if review.rater_role == REVIEW_ROLE_BUYER {
                        total += diesel::update(target)
                            .set(agent_tasks::buyer_review_stars.eq(review.stars))
                            .execute(conn)
                            .await?;
                    } else {
                        total += diesel::update(target)
                            .set(agent_tasks::seller_review_stars.eq(review.stars))
                            .execute(conn)
                            .await?;
                    }
                    if inserted > 0 {
                        total += enqueue_events(
                            conn,
                            &review.task_id,
                            TaskEventKind::Reviewed {
                                rater_role: review.rater_role,
                            },
                            review.reviewed_at_ms,
                            &review.event_id,
                        )
                        .await?;
                    }
                }
                TaskMarketRow::Disputed(dispute) => {
                    total += diesel::insert_into(task_disputes::table)
                        .values(dispute)
                        .on_conflict(task_disputes::task_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    let changed = diesel::update(
                        agent_tasks::table
                            .filter(agent_tasks::task_id.eq(&dispute.task_id))
                            .filter(agent_tasks::status.eq(TASK_STATUS_DELIVERED)),
                    )
                    .set((
                        agent_tasks::status.eq(TASK_STATUS_DISPUTED),
                        // The contract voids a pending refund proposal when a jury takes the task.
                        agent_tasks::refund_proposer.eq(0i16),
                        agent_tasks::refund_proposed_at_ms.eq(None::<i64>),
                        agent_tasks::event_id.eq(&dispute.event_id),
                        agent_tasks::transaction_id.eq(&dispute.transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events_with(
                            conn,
                            &dispute.task_id,
                            TaskEventKind::Disputed,
                            dispute.opened_at_ms,
                            &dispute.event_id,
                            serde_json::json!({
                                "case_id": dispute.case_id,
                                "opened_by_role": if dispute.opened_by_role == REVIEW_ROLE_BUYER { "buyer" } else { "seller" },
                                "fee_each_mist": dispute.fee_each_mist.to_string(),
                            }),
                        )
                        .await?;
                    }
                }
                TaskMarketRow::DisputeFeePaid {
                    task_id,
                    buyer_paid_mist,
                    seller_paid_mist,
                } => {
                    total += diesel::update(task_disputes::table.find(task_id))
                        .set((
                            task_disputes::buyer_paid_mist.eq(*buyer_paid_mist),
                            task_disputes::seller_paid_mist.eq(*seller_paid_mist),
                        ))
                        .execute(conn)
                        .await?;
                }
                TaskMarketRow::DisputeCancelled {
                    task_id,
                    settle_deadline_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        task_disputes::table
                            .find(task_id)
                            .filter(task_disputes::state.eq(DISPUTE_STATE_OPEN)),
                    )
                    .set((
                        task_disputes::state.eq(DISPUTE_STATE_CANCELLED),
                        task_disputes::agreement_proposer.eq(0i16),
                        task_disputes::agreement_outcome.eq(None::<i16>),
                    ))
                    .execute(conn)
                    .await?;
                    // The release clock was re-based on chain; the task is payable the usual way again.
                    total += diesel::update(
                        agent_tasks::table
                            .filter(agent_tasks::task_id.eq(task_id))
                            .filter(agent_tasks::status.eq(TASK_STATUS_DISPUTED)),
                    )
                    .set((
                        agent_tasks::status.eq(TASK_STATUS_DELIVERED),
                        agent_tasks::settle_deadline_ms.eq(*settle_deadline_ms),
                        agent_tasks::event_id.eq(event_id),
                        agent_tasks::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                TaskMarketRow::NeutralWindowOpened {
                    task_id,
                    window_ends_ms,
                    opened_at_ms,
                    event_id,
                } => {
                    let changed = diesel::update(
                        task_disputes::table
                            .find(task_id)
                            .filter(task_disputes::neutral_window_ends_ms.is_null()),
                    )
                    .set(task_disputes::neutral_window_ends_ms.eq(*window_ends_ms))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events_with(
                            conn,
                            task_id,
                            TaskEventKind::NeutralWindowOpened,
                            *opened_at_ms,
                            event_id,
                            serde_json::json!({"window_ends_ms": window_ends_ms}),
                        )
                        .await?;
                    }
                }
                TaskMarketRow::DisputeSettlementProposed {
                    task_id,
                    outcome,
                    proposer_role,
                } => {
                    total += diesel::update(task_disputes::table.find(task_id))
                        .set((
                            task_disputes::agreement_proposer.eq(*proposer_role),
                            task_disputes::agreement_outcome.eq(*outcome),
                        ))
                        .execute(conn)
                        .await?;
                }
                TaskMarketRow::DisputeSettlementCleared { task_id } => {
                    total += diesel::update(task_disputes::table.find(task_id))
                        .set((
                            task_disputes::agreement_proposer.eq(0i16),
                            task_disputes::agreement_outcome.eq(None::<i16>),
                        ))
                        .execute(conn)
                        .await?;
                }
                TaskMarketRow::DisputeResolved {
                    task_id,
                    outcome,
                    source,
                    resolved_at_ms,
                    event_id,
                } => {
                    let changed = diesel::update(
                        task_disputes::table
                            .find(task_id)
                            .filter(task_disputes::state.ne(DISPUTE_STATE_RESOLVED)),
                    )
                    .set((
                        task_disputes::state.eq(DISPUTE_STATE_RESOLVED),
                        task_disputes::resolved_outcome.eq(*outcome),
                        task_disputes::resolved_source.eq(*source),
                        task_disputes::resolved_at_ms.eq(*resolved_at_ms),
                        task_disputes::agreement_proposer.eq(0i16),
                        task_disputes::agreement_outcome.eq(None::<i16>),
                    ))
                    .execute(conn)
                    .await?;
                    total += changed;
                    if changed > 0 {
                        total += enqueue_events_with(
                            conn,
                            task_id,
                            TaskEventKind::DisputeResolved,
                            *resolved_at_ms,
                            event_id,
                            serde_json::json!({"outcome": outcome, "source": source}),
                        )
                        .await?;
                    }
                }
                TaskMarketRow::DisputeFeesSettled {
                    task_id,
                    buyer_refund_mist,
                    seller_refund_mist,
                } => {
                    total += diesel::update(task_disputes::table.find(task_id))
                        .set((
                            task_disputes::fees_settled.eq(true),
                            task_disputes::buyer_fee_refund_mist.eq(*buyer_refund_mist),
                            task_disputes::seller_fee_refund_mist.eq(*seller_refund_mist),
                        ))
                        .execute(conn)
                        .await?;
                }
                TaskMarketRow::Config(config) => {
                    total += diesel::insert_into(task_config_history::table)
                        .values(config)
                        .on_conflict(task_config_history::event_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                TaskMarketRow::FeeRecipientChanged {
                    new_recipient,
                    updated_by,
                    timestamp_ms,
                    event_id,
                    transaction_id,
                } => {
                    let inserted = diesel::sql_query(FEE_RECIPIENT_CHANGED_SQL)
                        .bind::<diesel::sql_types::Text, _>(updated_by)
                        .bind::<diesel::sql_types::Text, _>(new_recipient)
                        .bind::<diesel::sql_types::Int8, _>(*timestamp_ms)
                        .bind::<diesel::sql_types::Text, _>(event_id)
                        .bind::<diesel::sql_types::Text, _>(transaction_id)
                        .bind::<diesel::sql_types::Timestamptz, _>(common::chain_time_from_ms(
                            *timestamp_ms,
                        ))
                        .execute(conn)
                        .await?;
                    if inserted == 0 {
                        tracing::warn!(
                            event_id = %event_id,
                            "TaskFeeRecipientChanged: no earlier task_config_history row to copy"
                        );
                    }
                    total += inserted;
                }
            }
        }
        Ok(total)
    }
}

#[async_trait]
impl myso_indexer_alt_framework::pipeline::sequential::Handler for TaskMarketHandler {
    type Store = myso_indexer_alt_framework::postgres::Db;
    type Batch = Vec<TaskMarketRow>;

    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(&self, batch: &Self::Batch, conn: &mut Connection<'a>) -> Result<usize> {
        <Self as Handler>::commit(batch, conn).await
    }
}

#[cfg(test)]
mod tests {
    use move_core_types::account_address::AccountAddress;

    use super::super::events::{
        parse_event_contents, BcsMoveObjectId, BcsTaskAccepted, BcsTaskCancelled,
        BcsTaskConfigUpdated, BcsTaskDelivered, BcsTaskDisputeCancelled, BcsTaskDisputeFeePaid,
        BcsTaskDisputeFeesSettled, BcsTaskDisputeResolved, BcsTaskDisputeSettlementCleared,
        BcsTaskDisputeSettlementProposed, BcsTaskDisputed, BcsTaskNeutralWindowOpened,
        BcsTaskOffered, BcsTaskReceiptSettled, BcsTaskRefundProposalCleared, BcsTaskRefundProposed,
        BcsTaskRefunded, BcsTaskReviewSubmitted,
    };
    use super::*;

    fn addr(hex: &str) -> AccountAddress {
        AccountAddress::from_hex_literal(hex).unwrap()
    }

    fn id(hex: &str) -> BcsMoveObjectId {
        BcsMoveObjectId::from_address(addr(hex))
    }

    fn rows(event_name: &str, bytes: &[u8]) -> Vec<TaskMarketRow> {
        let json = parse_event_contents(AGENT_TASK_MODULE, event_name, bytes)
            .expect("BCS parse should succeed");
        task_market_rows(event_name, &json, "digest:2").expect("row builder should accept")
    }

    #[test]
    fn every_agent_task_event_is_handled() {
        // Mirrors the event structs in agent_task.move. Adding an event there without a parser
        // and row builder must fail here.
        let move_source =
            include_str!("../../../myso-framework/packages/myso-social/sources/agent_task.move");
        let mut declared: Vec<&str> = move_source
            .lines()
            .filter_map(|l| l.trim().strip_prefix("public struct "))
            .filter(|rest| rest.contains("has copy, drop"))
            .filter_map(|rest| rest.split_whitespace().next())
            .collect();
        declared.sort_unstable();
        let mut handled: Vec<&str> = AGENT_TASK_EVENTS.to_vec();
        handled.sort_unstable();
        assert_eq!(declared, handled);
    }

    #[test]
    fn offered_inserts_a_pending_task() {
        let ev = BcsTaskOffered {
            task_id: id("0x7a5c"),
            buyer_principal: addr("0xb0b"),
            buyer_agent_id: Some(id("0xb0a")),
            buyer_memory_account_id: id("0xb0c"),
            seller_principal: addr("0x5e"),
            seller_agent_id: id("0x5ea"),
            seller_memory_account_id: id("0x5ec"),
            organization_id: None,
            room_group_id: Some(id("0x1111")),
            platform_id: None,
            title: "write the report".into(),
            scope_digest: vec![0x11; 32],
            amount_mist: 1_000_000_000,
            accept_window_ms: 86_400_000,
            delivery_window_ms: 86_400_000,
            settle_window_ms: 86_400_000,
            platform_fee_bps: 200,
            ecosystem_fee_bps: 50,
            terms_digest: vec![0x22; 32],
            accept_deadline_ms: 1_700_086_400_000,
            created_at_ms: 1_700_000_000_000,
        };
        let out = rows("TaskOffered", &bcs::to_bytes(&ev).unwrap());
        let [TaskMarketRow::Offered(task)] = out.as_slice() else {
            panic!("expected one Offered row");
        };
        assert_eq!(task.status, TASK_STATUS_OFFERED);
        assert_eq!(task.amount_mist, 1_000_000_000);
        assert!(task.task_id.ends_with("7a5c"));
        assert!(task.buyer_agent_id.as_deref().unwrap().ends_with("b0a"));
        assert!(task.organization_id.is_none());
        assert!(task.platform_id.is_none());
        assert_eq!(task.scope_digest, "11".repeat(32));
        assert_eq!(task.terms_digest, "22".repeat(32));
        assert_eq!(task.transaction_id, "digest");
    }

    #[test]
    fn lifecycle_events_map_to_their_updates() {
        let accepted = rows(
            "TaskAccepted",
            &bcs::to_bytes(&BcsTaskAccepted {
                task_id: id("0x7a5c"),
                seller_agent_id: id("0x5ea"),
                seller_principal: addr("0x5e"),
                accepted_at_ms: 10,
                delivery_deadline_ms: 20,
            })
            .unwrap(),
        );
        assert!(matches!(
            accepted.as_slice(),
            [TaskMarketRow::Accepted {
                accepted_at_ms: 10,
                delivery_deadline_ms: 20,
                ..
            }]
        ));

        let delivered = rows(
            "TaskDelivered",
            &bcs::to_bytes(&BcsTaskDelivered {
                task_id: id("0x7a5c"),
                seller_agent_id: id("0x5ea"),
                deliverable_digest: vec![0x33; 32],
                deliverable_uri: "walrus://x".into(),
                delivered_at_ms: 30,
                settle_deadline_ms: 40,
            })
            .unwrap(),
        );
        let [TaskMarketRow::Delivered {
            deliverable_digest,
            settle_deadline_ms,
            ..
        }] = delivered.as_slice()
        else {
            panic!("expected Delivered");
        };
        assert_eq!(deliverable_digest, &"33".repeat(32));
        assert_eq!(*settle_deadline_ms, 40);

        let proposed = rows(
            "TaskRefundProposed",
            &bcs::to_bytes(&BcsTaskRefundProposed {
                task_id: id("0x7a5c"),
                proposer_role: 2,
                proposed_at_ms: 50,
            })
            .unwrap(),
        );
        assert!(matches!(
            proposed.as_slice(),
            [TaskMarketRow::RefundProposed {
                proposer_role: 2,
                proposed_at_ms: 50,
                ..
            }]
        ));

        let cleared = rows(
            "TaskRefundProposalCleared",
            &bcs::to_bytes(&BcsTaskRefundProposalCleared {
                task_id: id("0x7a5c"),
                cleared_by_role: 1,
                reason: 1,
                cleared_at_ms: 60,
            })
            .unwrap(),
        );
        assert!(matches!(
            cleared.as_slice(),
            [TaskMarketRow::ProposalCleared { .. }]
        ));

        let refunded = rows(
            "TaskRefunded",
            &bcs::to_bytes(&BcsTaskRefunded {
                task_id: id("0x7a5c"),
                refund_kind: 1,
                buyer_principal: addr("0xb0b"),
                amount_mist: 5,
                refunded_at_ms: 70,
            })
            .unwrap(),
        );
        assert!(matches!(
            refunded.as_slice(),
            [TaskMarketRow::Refunded {
                refund_kind: 1,
                refunded_at_ms: 70,
                ..
            }]
        ));

        let cancelled = rows(
            "TaskCancelled",
            &bcs::to_bytes(&BcsTaskCancelled {
                task_id: id("0x7a5c"),
                close_kind: 2,
                buyer_principal: addr("0xb0b"),
                amount_mist: 5,
                cancelled_at_ms: 80,
            })
            .unwrap(),
        );
        assert!(matches!(
            cancelled.as_slice(),
            [TaskMarketRow::Cancelled {
                close_kind: 2,
                cancelled_at_ms: 80,
                ..
            }]
        ));
    }

    #[test]
    fn a_dispute_opens_with_only_the_openers_fee_on_record() {
        let out = rows(
            "TaskDisputed",
            &bcs::to_bytes(&BcsTaskDisputed {
                task_id: id("0x7a5c"),
                case_id: id("0xca5e"),
                opened_by_role: 2,
                fee_each_mist: 30_000_000,
                opener_paid_mist: 30_000_000,
                opened_at_ms: 500,
            })
            .unwrap(),
        );
        let [TaskMarketRow::Disputed(d)] = out.as_slice() else {
            panic!("expected Disputed");
        };
        assert_eq!((d.state, d.opened_by_role), (DISPUTE_STATE_OPEN, 2));
        assert_eq!((d.buyer_paid_mist, d.seller_paid_mist), (0, 30_000_000));
        assert!(d.case_id.ends_with("ca5e"));
        assert_eq!(d.transaction_id, "digest");

        let by_buyer = rows(
            "TaskDisputed",
            &bcs::to_bytes(&BcsTaskDisputed {
                task_id: id("0x7a5c"),
                case_id: id("0xca5e"),
                opened_by_role: 1,
                fee_each_mist: 30_000_000,
                opener_paid_mist: 30_000_000,
                opened_at_ms: 500,
            })
            .unwrap(),
        );
        let [TaskMarketRow::Disputed(d)] = by_buyer.as_slice() else {
            panic!("expected Disputed");
        };
        assert_eq!((d.buyer_paid_mist, d.seller_paid_mist), (30_000_000, 0));
    }

    #[test]
    fn dispute_follow_up_events_map_to_their_updates() {
        let paid = rows(
            "TaskDisputeFeePaid",
            &bcs::to_bytes(&BcsTaskDisputeFeePaid {
                task_id: id("0x7a5c"),
                role: 1,
                amount_mist: 30,
                kind: 1,
                buyer_paid_mist: 30,
                seller_paid_mist: 30,
                paid_at_ms: 510,
            })
            .unwrap(),
        );
        assert!(matches!(
            paid.as_slice(),
            [TaskMarketRow::DisputeFeePaid {
                buyer_paid_mist: 30,
                seller_paid_mist: 30,
                ..
            }]
        ));

        let cancelled = rows(
            "TaskDisputeCancelled",
            &bcs::to_bytes(&BcsTaskDisputeCancelled {
                task_id: id("0x7a5c"),
                case_id: id("0xca5e"),
                settle_deadline_ms: 900,
                cancelled_at_ms: 520,
            })
            .unwrap(),
        );
        assert!(matches!(
            cancelled.as_slice(),
            [TaskMarketRow::DisputeCancelled {
                settle_deadline_ms: 900,
                ..
            }]
        ));

        let window = rows(
            "TaskNeutralWindowOpened",
            &bcs::to_bytes(&BcsTaskNeutralWindowOpened {
                task_id: id("0x7a5c"),
                case_id: id("0xca5e"),
                window_ends_ms: 600,
                opened_at_ms: 530,
            })
            .unwrap(),
        );
        assert!(matches!(
            window.as_slice(),
            [TaskMarketRow::NeutralWindowOpened {
                window_ends_ms: 600,
                opened_at_ms: 530,
                ..
            }]
        ));

        let proposed = rows(
            "TaskDisputeSettlementProposed",
            &bcs::to_bytes(&BcsTaskDisputeSettlementProposed {
                task_id: id("0x7a5c"),
                proposer_role: 2,
                outcome: 1,
                proposed_at_ms: 540,
            })
            .unwrap(),
        );
        assert!(matches!(
            proposed.as_slice(),
            [TaskMarketRow::DisputeSettlementProposed {
                outcome: 1,
                proposer_role: 2,
                ..
            }]
        ));

        let cleared = rows(
            "TaskDisputeSettlementCleared",
            &bcs::to_bytes(&BcsTaskDisputeSettlementCleared {
                task_id: id("0x7a5c"),
                cleared_by_role: 1,
                cleared_at_ms: 550,
            })
            .unwrap(),
        );
        assert!(matches!(
            cleared.as_slice(),
            [TaskMarketRow::DisputeSettlementCleared { .. }]
        ));

        let resolved = rows(
            "TaskDisputeResolved",
            &bcs::to_bytes(&BcsTaskDisputeResolved {
                task_id: id("0x7a5c"),
                case_id: id("0xca5e"),
                outcome: 2,
                source: 3,
                resolved_at_ms: 560,
            })
            .unwrap(),
        );
        assert!(matches!(
            resolved.as_slice(),
            [TaskMarketRow::DisputeResolved {
                outcome: 2,
                source: 3,
                resolved_at_ms: 560,
                ..
            }]
        ));

        let settled = rows(
            "TaskDisputeFeesSettled",
            &bcs::to_bytes(&BcsTaskDisputeFeesSettled {
                task_id: id("0x7a5c"),
                case_id: id("0xca5e"),
                buyer_refund_mist: 10,
                seller_refund_mist: 20,
                settled_at_ms: 570,
            })
            .unwrap(),
        );
        assert!(matches!(
            settled.as_slice(),
            [TaskMarketRow::DisputeFeesSettled {
                buyer_refund_mist: 10,
                seller_refund_mist: 20,
                ..
            }]
        ));
    }

    #[test]
    fn settlement_updates_the_task_and_writes_a_receipt() {
        let ev = BcsTaskReceiptSettled {
            task_id: id("0x7a5c"),
            receipt_id: id("0x7a5c"),
            buyer_principal: addr("0xb0b"),
            buyer_agent_id: None,
            seller_principal: addr("0x5e"),
            seller_agent_id: id("0x5ea"),
            organization_id: None,
            room_group_id: None,
            settlement_kind: 2,
            amount_mist: 1_000_000_000,
            platform_fee_mist: 0,
            ecosystem_fee_mist: 25_000_000,
            net_amount_mist: 975_000_000,
            deliverable_digest: Some(vec![0x33; 32]),
            deliverable_uri: Some("walrus://x".into()),
            buyer_credibility_bps: 2000,
            seller_credibility_bps: 2500,
            settle_deadline_ms: 90,
            review_deadline_ms: 100,
            settled_at_ms: 95,
        };
        let out = rows("TaskReceiptSettled", &bcs::to_bytes(&ev).unwrap());
        let [TaskMarketRow::Settled {
            settlement_kind,
            net_amount_mist,
            buyer_credibility_bps,
            seller_credibility_bps,
            review_deadline_ms,
            ..
        }, TaskMarketRow::Receipt(receipt)] = out.as_slice()
        else {
            panic!("expected Settled then Receipt");
        };
        assert_eq!(*settlement_kind, 2);
        assert_eq!(*net_amount_mist, 975_000_000);
        assert_eq!(
            (*buyer_credibility_bps, *seller_credibility_bps),
            (2000, 2500)
        );
        assert_eq!(*review_deadline_ms, 100);
        assert_eq!(receipt.ecosystem_fee_mist, 25_000_000);
        assert_eq!(receipt.deliverable_uri.as_deref(), Some("walrus://x"));
        assert!(receipt.buyer_agent_id.is_none());
    }

    #[test]
    fn a_review_row_keys_on_task_and_direction() {
        let ev = BcsTaskReviewSubmitted {
            task_id: id("0x7a5c"),
            receipt_id: id("0x7a5c"),
            rater_role: 1,
            rater_principal: addr("0xb0b"),
            rater_agent_id: None,
            reviewee_principal: addr("0x5e"),
            reviewee_agent_id: Some(id("0x5ea")),
            stars: 5,
            rater_credibility_bps: 2000,
            value_weight_bps: 10000,
            base_weight_bps: 2000,
            prior_pair_reviews: 0,
            weight_bps: 2000,
            reviewed_at_ms: 120,
        };
        let out = rows("TaskReviewSubmitted", &bcs::to_bytes(&ev).unwrap());
        let [TaskMarketRow::Review(review)] = out.as_slice() else {
            panic!("expected Review");
        };
        assert_eq!((review.rater_role, review.stars), (1, 5));
        assert_eq!(review.weight_bps, 2000);
        assert!(review.reviewee_agent_id.is_some());
        assert!(review.rater_agent_id.is_none());
    }

    #[test]
    fn config_update_carries_every_field() {
        let ev = BcsTaskConfigUpdated {
            updated_by: addr("0xad"),
            platform_fee_bps: 200,
            ecosystem_fee_bps: 50,
            ecosystem_fee_recipient: addr("0xad"),
            min_escrow_mist: 10_000_000,
            accept_window_min_ms: 300_000,
            accept_window_default_ms: 86_400_000,
            accept_window_max_ms: 1_209_600_000,
            delivery_window_min_ms: 3_600_000,
            delivery_window_max_ms: 7_776_000_000,
            settle_window_min_ms: 3_600_000,
            settle_window_max_ms: 5_184_000_000,
            release_grace_ms: 259_200_000,
            review_window_ms: 2_592_000_000,
            max_title_bytes: 128,
            max_uri_bytes: 2048,
            active: true,
            prior_mean_milli: 3000,
            prior_weight_bps: 50000,
            value_ref_mist: 1_000_000_000,
            min_value_weight_bps: 500,
            max_value_weight_bps: 20000,
            cred_base_bps: 2000,
            cred_step_bps: 500,
            cred_cap_n: 16,
            cred_max_bps: 10000,
            overall_seller_weight_bps: 5000,
            overall_buyer_weight_bps: 5000,
            timestamp_ms: 1_700_000_000_000,
        };
        let out = rows("TaskConfigUpdated", &bcs::to_bytes(&ev).unwrap());
        let [TaskMarketRow::Config(config)] = out.as_slice() else {
            panic!("expected Config");
        };
        assert!(config.active);
        assert_eq!(config.release_grace_ms, 259_200_000);
        assert_eq!(
            config.overall_seller_weight_bps + config.overall_buyer_weight_bps,
            10_000
        );
        assert_eq!(config.event_id, "digest:2");
    }

    #[test]
    fn an_unrelated_event_name_builds_no_rows() {
        assert!(task_market_rows("SomethingElse", &serde_json::json!({}), "d:0").is_none());
    }
}
