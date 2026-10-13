// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Task market events for the automation engine.
//!
//! The indexer turns each task lifecycle step into one outbox row per party that may need to react
//! (`build_task_events`), written in the same transaction as the task update. The social server's
//! forwarder later fills in the counterparty's reputation (`Enrichment`) and posts the row to the
//! engine as a `PlatformEvent` (`platform_event`). Both halves live here so the indexer and the
//! server cannot disagree about what an event looks like.
//!
//! Payloads are flat and carry everything a job's policy needs (amounts, windows, scores), so an
//! unattended agent never has to call back for context.

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::schema::{agent_tasks, automation_event_outbox};

pub const AUTOMATION_FAMILY_TASK: &str = "task";
pub const AUTOMATION_EVENT_SOURCE: &str = "myso_social_indexer";

/// Reputation shown to a job when the chain has no review for the counterparty yet. Matches the
/// on-chain defaults; the forwarder prefers the live `task_config` values.
pub const DEFAULT_PRIOR_SCORE_MILLI: i64 = 3000;
pub const DEFAULT_BASE_CREDIBILITY_BPS: i64 = 2000;

const ROLE_BUYER: i16 = 1;
const ROLE_SELLER: i16 = 2;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, PartialEq)]
#[diesel(table_name = automation_event_outbox)]
pub struct NewAutomationEvent {
    pub dedup_key: String,
    pub event_family: String,
    pub event_type: String,
    pub account_id: String,
    pub agent_object_id: Option<String>,
    pub payload: Value,
    pub occurred_at_ms: i64,
    pub source_event_id: String,
}

/// The slice of `agent_tasks` an event payload is built from.
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, PartialEq)]
#[diesel(table_name = agent_tasks)]
pub struct TaskSnapshot {
    pub task_id: String,
    pub status: i16,
    pub buyer_principal: String,
    pub buyer_agent_id: Option<String>,
    pub buyer_memory_account_id: String,
    pub seller_principal: String,
    pub seller_agent_id: String,
    pub seller_memory_account_id: String,
    pub organization_id: Option<String>,
    pub room_group_id: Option<String>,
    pub platform_id: Option<String>,
    pub title: String,
    pub scope_digest: String,
    pub terms_digest: String,
    pub amount_mist: i64,
    pub accept_window_ms: i64,
    pub delivery_window_ms: i64,
    pub settle_window_ms: i64,
    pub accept_deadline_ms: i64,
    pub accepted_at_ms: Option<i64>,
    pub delivery_deadline_ms: Option<i64>,
    pub delivered_at_ms: Option<i64>,
    pub settle_deadline_ms: Option<i64>,
    pub settled_at_ms: Option<i64>,
    pub review_deadline_ms: Option<i64>,
    pub close_kind: Option<i16>,
    pub settlement_kind: Option<i16>,
    pub refund_kind: Option<i16>,
    pub refund_proposer: i16,
    pub deliverable_digest: Option<String>,
    pub deliverable_uri: Option<String>,
}

/// What just happened to a task. `Reviewed` carries the direction (1: the buyer rated the seller,
/// 2: the seller rated the buyer), matching `task_reviews.rater_role`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskEventKind {
    Offered,
    Accepted,
    Cancelled,
    Delivered,
    RefundProposed,
    RefundCleared,
    Refunded,
    Settled,
    Reviewed { rater_role: i16 },
    /// A party took the delivered work to a jury; the escrow is frozen.
    Disputed,
    /// The jury failed and the settlement window opened.
    NeutralWindowOpened,
    /// The dispute ended (verdict, agreement or the automatic split).
    DisputeResolved,
    /// Time-driven: an unanswered offer is past its accept deadline.
    ExpiredOfferDue,
    /// Time-driven: an accepted task is past its delivery deadline.
    RefundDue,
    /// Time-driven: a delivered task is past its settle deadline plus the release grace.
    ReleaseDue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Party {
    Buyer,
    Seller,
}

impl Party {
    fn role(self) -> &'static str {
        match self {
            Party::Buyer => "buyer",
            Party::Seller => "seller",
        }
    }

    fn other(self) -> Party {
        match self {
            Party::Buyer => Party::Seller,
            Party::Seller => Party::Buyer,
        }
    }
}

struct Target {
    event_type: &'static str,
    party: Party,
    /// A proposal can be made, cleared and made again, so its key includes the source event.
    repeatable: bool,
}

const fn once(event_type: &'static str, party: Party) -> Target {
    Target {
        event_type,
        party,
        repeatable: false,
    }
}

const fn again(event_type: &'static str, party: Party) -> Target {
    Target {
        event_type,
        party,
        repeatable: true,
    }
}

fn targets(kind: TaskEventKind, task: &TaskSnapshot) -> Vec<Target> {
    use Party::{Buyer, Seller};
    match kind {
        TaskEventKind::Offered => vec![once("offered", Seller)],
        // The seller starts work on acceptance; the buyer learns it began.
        TaskEventKind::Accepted => vec![once("accepted", Seller), once("accepted", Buyer)],
        TaskEventKind::Cancelled => vec![once("cancelled", Buyer), once("cancelled", Seller)],
        TaskEventKind::Delivered => vec![once("delivered", Buyer)],
        TaskEventKind::RefundProposed => match task.refund_proposer {
            1 => vec![again("refund_proposed", Seller)],
            2 => vec![again("refund_proposed", Buyer)],
            _ => Vec::new(),
        },
        TaskEventKind::RefundCleared => {
            vec![again("refund_cleared", Buyer), again("refund_cleared", Seller)]
        }
        TaskEventKind::Refunded => vec![once("refunded", Buyer), once("refunded", Seller)],
        TaskEventKind::Settled => vec![
            once("settled", Seller),
            once("settled", Buyer),
            once("review_open", Buyer),
            once("review_open", Seller),
        ],
        TaskEventKind::Reviewed { rater_role } => match rater_role {
            ROLE_BUYER => vec![once("reviewed", Seller)],
            ROLE_SELLER => vec![once("reviewed", Buyer)],
            _ => Vec::new(),
        },
        TaskEventKind::Disputed => vec![once("disputed", Buyer), once("disputed", Seller)],
        TaskEventKind::NeutralWindowOpened => {
            vec![once("neutral_window_open", Buyer), once("neutral_window_open", Seller)]
        }
        TaskEventKind::DisputeResolved => {
            vec![once("dispute_resolved", Buyer), once("dispute_resolved", Seller)]
        }
        TaskEventKind::ExpiredOfferDue => vec![once("expired_offer_due", Buyer)],
        TaskEventKind::RefundDue => vec![once("refund_due", Buyer)],
        TaskEventKind::ReleaseDue => vec![once("release_due", Seller)],
    }
}

/// The deadline of the step an event asks for. Acting after it can only abort on chain, so the
/// engine skips the run instead.
fn expires_at_ms(event_type: &str, task: &TaskSnapshot) -> Option<i64> {
    match event_type {
        "offered" => Some(task.accept_deadline_ms),
        "accepted" => task.delivery_deadline_ms,
        "review_open" => task.review_deadline_ms,
        _ => None,
    }
}

fn base_payload(task: &TaskSnapshot, event_type: &str, party: Party) -> serde_json::Map<String, Value> {
    let mut payload = serde_json::Map::new();
    let mut put = |key: &str, value: Value| {
        payload.insert(key.to_string(), value);
    };
    put("task_id", json!(task.task_id));
    put("status", json!(task.status));
    put("party_role", json!(party.role()));
    put("counterparty_role", json!(party.other().role()));
    put("buyer_principal", json!(task.buyer_principal));
    put("buyer_account_id", json!(task.buyer_memory_account_id));
    put("buyer_agent_id", json!(task.buyer_agent_id));
    put("seller_principal", json!(task.seller_principal));
    put("seller_account_id", json!(task.seller_memory_account_id));
    put("seller_agent_id", json!(task.seller_agent_id));
    put("organization_id", json!(task.organization_id));
    put("room_group_id", json!(task.room_group_id));
    put("platform_id", json!(task.platform_id));
    put("title", json!(task.title));
    put("scope_digest", json!(task.scope_digest));
    put("terms_digest", json!(task.terms_digest));
    // Amounts and windows are decimal strings so a u64 never passes through a JSON double.
    put("amount_mist", json!(task.amount_mist.to_string()));
    put("accept_window_ms", json!(task.accept_window_ms.to_string()));
    put("delivery_window_ms", json!(task.delivery_window_ms.to_string()));
    put("settle_window_ms", json!(task.settle_window_ms.to_string()));
    put("accept_deadline_ms", json!(task.accept_deadline_ms));
    put("accepted_at_ms", json!(task.accepted_at_ms));
    put("delivery_deadline_ms", json!(task.delivery_deadline_ms));
    put("delivered_at_ms", json!(task.delivered_at_ms));
    put("settle_deadline_ms", json!(task.settle_deadline_ms));
    put("settled_at_ms", json!(task.settled_at_ms));
    put("review_deadline_ms", json!(task.review_deadline_ms));
    put("deliverable_digest", json!(task.deliverable_digest));
    put("deliverable_uri", json!(task.deliverable_uri));
    put("close_kind", json!(task.close_kind));
    put("settlement_kind", json!(task.settlement_kind));
    put("refund_kind", json!(task.refund_kind));
    if task.refund_proposer != 0 {
        let proposer = if task.refund_proposer == 1 { "buyer" } else { "seller" };
        put("refund_proposer_role", json!(proposer));
    }
    if let Some(expires) = expires_at_ms(event_type, task) {
        put("expires_at_ms", json!(expires));
    }
    payload
}

/// One outbox row per party that may need to react to `kind`.
///
/// `source_event_id` (`<tx digest>:<event seq>` for chain events) makes repeatable events unique and
/// keeps the engine's deduplication key stable if the same row is built twice.
pub fn build_task_events(
    kind: TaskEventKind,
    task: &TaskSnapshot,
    occurred_at_ms: i64,
    source_event_id: &str,
) -> Vec<NewAutomationEvent> {
    targets(kind, task)
        .into_iter()
        .map(|target| {
            let (account_id, agent_object_id) = match target.party {
                Party::Buyer => (
                    task.buyer_memory_account_id.clone(),
                    task.buyer_agent_id.clone(),
                ),
                Party::Seller => (
                    task.seller_memory_account_id.clone(),
                    Some(task.seller_agent_id.clone()),
                ),
            };
            let mut dedup_key = format!(
                "task:{}:{}:{}",
                task.task_id,
                target.event_type,
                target.party.role()
            );
            if target.repeatable {
                dedup_key.push(':');
                dedup_key.push_str(source_event_id);
            }
            NewAutomationEvent {
                dedup_key,
                event_family: AUTOMATION_FAMILY_TASK.to_string(),
                event_type: target.event_type.to_string(),
                account_id,
                agent_object_id,
                payload: Value::Object(base_payload(task, target.event_type, target.party)),
                occurred_at_ms,
                source_event_id: source_event_id.to_string(),
            }
        })
        .collect()
}

/// Counterparty reputation, attached by the forwarder just before delivery so it reflects what the
/// reputation pipeline has indexed by then.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartyStats {
    /// The party's score in the role it plays in this task (buyer or seller), in milli-stars.
    pub score_milli: i64,
    pub credibility_bps: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Enrichment {
    pub buyer: PartyStats,
    pub seller: PartyStats,
    /// The seller agent's tasks that are accepted or delivered and not yet settled.
    pub seller_open_tasks: i64,
}

impl Enrichment {
    pub fn apply(&self, payload: &mut Value) {
        let Some(map) = payload.as_object_mut() else {
            return;
        };
        map.insert("buyer_score_milli".into(), json!(self.buyer.score_milli));
        map.insert("buyer_credibility_bps".into(), json!(self.buyer.credibility_bps));
        map.insert("seller_score_milli".into(), json!(self.seller.score_milli));
        map.insert("seller_credibility_bps".into(), json!(self.seller.credibility_bps));
        map.insert("seller_open_tasks".into(), json!(self.seller_open_tasks));
    }
}

/// The engine's `PlatformEvent` envelope for an outbox row. No `organization_id` is set on purpose:
/// the engine scopes an event that carries one by organization, and task parties are scoped by
/// account instead.
pub fn platform_event(row: &AutomationEventPayload, payload: &Value) -> Value {
    let mut event = serde_json::Map::new();
    event.insert("event_version".into(), json!(1));
    event.insert("event_family".into(), json!(row.event_family));
    event.insert("event_type".into(), json!(row.event_type));
    event.insert("account_id".into(), json!(row.account_id));
    if let Some(agent) = &row.agent_object_id {
        event.insert("agent_object_id".into(), json!(agent));
    }
    event.insert("payload".into(), payload.clone());
    event.insert("occurred_at_ms".into(), json!(row.occurred_at_ms));
    event.insert("source_event_id".into(), json!(row.source_event_id));
    event.insert("deduplication_key".into(), json!(row.dedup_key));
    event.insert("source_service".into(), json!(AUTOMATION_EVENT_SOURCE));
    Value::Object(event)
}

/// The identifying fields of a queued event, independent of how the row was loaded.
#[derive(Debug, Clone, PartialEq)]
pub struct AutomationEventPayload {
    pub dedup_key: String,
    pub event_family: String,
    pub event_type: String,
    pub account_id: String,
    pub agent_object_id: Option<String>,
    pub occurred_at_ms: i64,
    pub source_event_id: String,
}

impl From<&NewAutomationEvent> for AutomationEventPayload {
    fn from(event: &NewAutomationEvent) -> Self {
        Self {
            dedup_key: event.dedup_key.clone(),
            event_family: event.event_family.clone(),
            event_type: event.event_type.clone(),
            account_id: event.account_id.clone(),
            agent_object_id: event.agent_object_id.clone(),
            occurred_at_ms: event.occurred_at_ms,
            source_event_id: event.source_event_id.clone(),
        }
    }
}

/// Retry delay after a failed delivery: 5 s doubling to a 10 minute ceiling.
pub fn retry_delay_secs(attempts: i32) -> i64 {
    let shift = attempts.clamp(1, 8) - 1;
    (5i64 << shift).min(600)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task() -> TaskSnapshot {
        TaskSnapshot {
            task_id: "0xtask".into(),
            status: 0,
            buyer_principal: "0xbuyer".into(),
            buyer_agent_id: Some("0xbuyer_agent".into()),
            buyer_memory_account_id: "0xbuyer_account".into(),
            seller_principal: "0xseller".into(),
            seller_agent_id: "0xseller_agent".into(),
            seller_memory_account_id: "0xseller_account".into(),
            organization_id: Some("0xorg".into()),
            room_group_id: Some("0xroom".into()),
            platform_id: None,
            title: "Translate".into(),
            scope_digest: "ab".repeat(32),
            terms_digest: "cd".repeat(32),
            amount_mist: 5_000_000_000,
            accept_window_ms: 3_600_000,
            delivery_window_ms: 86_400_000,
            settle_window_ms: 86_400_000,
            accept_deadline_ms: 1_000,
            accepted_at_ms: Some(2_000),
            delivery_deadline_ms: Some(3_000),
            delivered_at_ms: Some(4_000),
            settle_deadline_ms: Some(5_000),
            settled_at_ms: Some(6_000),
            review_deadline_ms: Some(7_000),
            close_kind: None,
            settlement_kind: Some(1),
            refund_kind: None,
            refund_proposer: 0,
            deliverable_digest: Some("ef".repeat(32)),
            deliverable_uri: Some("wal://out".into()),
        }
    }

    fn rows(kind: TaskEventKind, task: &TaskSnapshot) -> Vec<NewAutomationEvent> {
        build_task_events(kind, task, 10, "txdigest:0")
    }

    fn summary(events: &[NewAutomationEvent]) -> Vec<(String, String)> {
        events
            .iter()
            .map(|e| (e.event_type.clone(), e.account_id.clone()))
            .collect()
    }

    #[test]
    fn a_dispute_reaches_both_parties_and_each_outcome_is_announced_once() {
        let t = task();
        for (kind, name) in [
            (TaskEventKind::Disputed, "disputed"),
            (TaskEventKind::NeutralWindowOpened, "neutral_window_open"),
            (TaskEventKind::DisputeResolved, "dispute_resolved"),
        ] {
            let events = rows(kind, &t);
            assert_eq!(
                summary(&events),
                vec![
                    (name.to_string(), "0xbuyer_account".to_string()),
                    (name.to_string(), "0xseller_account".to_string()),
                ]
            );
            assert!(events.iter().all(|e| e.event_family == AUTOMATION_FAMILY_TASK));
            assert_eq!(
                events.iter().map(|e| e.dedup_key.clone()).collect::<Vec<_>>(),
                vec![format!("task:0xtask:{name}:buyer"), format!("task:0xtask:{name}:seller")]
            );
        }
    }

    #[test]
    fn an_offer_reaches_only_the_seller_agent() {
        let events = rows(TaskEventKind::Offered, &task());
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.event_family, "task");
        assert_eq!(event.event_type, "offered");
        assert_eq!(event.account_id, "0xseller_account");
        assert_eq!(event.agent_object_id.as_deref(), Some("0xseller_agent"));
        assert_eq!(event.dedup_key, "task:0xtask:offered:seller");
        assert_eq!(event.payload["party_role"], "seller");
        assert_eq!(event.payload["counterparty_role"], "buyer");
        assert_eq!(event.payload["expires_at_ms"], 1_000);
    }

    #[test]
    fn payload_amounts_are_decimal_strings() {
        let payload = &rows(TaskEventKind::Offered, &task())[0].payload;
        assert_eq!(payload["amount_mist"], "5000000000");
        assert_eq!(payload["delivery_window_ms"], "86400000");
        assert_eq!(payload["terms_digest"], "cd".repeat(32));
        assert_eq!(payload["buyer_principal"], "0xbuyer");
        assert_eq!(payload["buyer_agent_id"], "0xbuyer_agent");
    }

    #[test]
    fn acceptance_starts_the_sellers_work_and_informs_the_buyer() {
        let events = rows(TaskEventKind::Accepted, &task());
        assert_eq!(
            summary(&events),
            [
                ("accepted".into(), "0xseller_account".into()),
                ("accepted".into(), "0xbuyer_account".into())
            ]
        );
        assert!(events.iter().all(|e| e.payload["expires_at_ms"] == 3_000));
    }

    #[test]
    fn a_human_buyer_has_no_agent_to_scope_to() {
        let mut human = task();
        human.buyer_agent_id = None;
        let events = rows(TaskEventKind::Delivered, &human);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].account_id, "0xbuyer_account");
        assert_eq!(events[0].agent_object_id, None);
        assert_eq!(events[0].payload["deliverable_uri"], "wal://out");
        assert!(events[0].payload.get("expires_at_ms").is_none());
    }

    #[test]
    fn settlement_notifies_both_parties_and_opens_a_review_for_each() {
        let events = rows(TaskEventKind::Settled, &task());
        let kinds: Vec<_> = events.iter().map(|e| (e.event_type.as_str(), e.payload["party_role"].as_str().unwrap())).collect();
        assert_eq!(
            kinds,
            [("settled", "seller"), ("settled", "buyer"), ("review_open", "buyer"), ("review_open", "seller")]
        );
        for event in events.iter().filter(|e| e.event_type == "review_open") {
            assert_eq!(event.payload["expires_at_ms"], 7_000);
        }
        let keys: std::collections::HashSet<_> = events.iter().map(|e| &e.dedup_key).collect();
        assert_eq!(keys.len(), 4, "every row needs its own deduplication key");
    }

    #[test]
    fn a_review_is_delivered_to_the_reviewee() {
        let by_buyer = rows(TaskEventKind::Reviewed { rater_role: 1 }, &task());
        assert_eq!(summary(&by_buyer), [("reviewed".into(), "0xseller_account".into())]);
        let by_seller = rows(TaskEventKind::Reviewed { rater_role: 2 }, &task());
        assert_eq!(summary(&by_seller), [("reviewed".into(), "0xbuyer_account".into())]);
        assert!(rows(TaskEventKind::Reviewed { rater_role: 3 }, &task()).is_empty());
    }

    #[test]
    fn a_refund_proposal_goes_to_the_other_party() {
        let mut proposed = task();
        proposed.refund_proposer = 1;
        let events = rows(TaskEventKind::RefundProposed, &proposed);
        assert_eq!(summary(&events), [("refund_proposed".into(), "0xseller_account".into())]);
        assert_eq!(events[0].payload["refund_proposer_role"], "buyer");

        proposed.refund_proposer = 2;
        assert_eq!(
            summary(&rows(TaskEventKind::RefundProposed, &proposed)),
            [("refund_proposed".into(), "0xbuyer_account".into())]
        );
        proposed.refund_proposer = 0;
        assert!(rows(TaskEventKind::RefundProposed, &proposed).is_empty());
    }

    #[test]
    fn a_repeatable_event_gets_a_key_per_source_event() {
        let mut proposed = task();
        proposed.refund_proposer = 1;
        let first = build_task_events(TaskEventKind::RefundProposed, &proposed, 1, "tx1:0");
        let second = build_task_events(TaskEventKind::RefundProposed, &proposed, 2, "tx2:0");
        assert_ne!(first[0].dedup_key, second[0].dedup_key);
        // Rebuilding the same event is stable, which is what makes a replayed checkpoint harmless.
        assert_eq!(
            first[0].dedup_key,
            build_task_events(TaskEventKind::RefundProposed, &proposed, 1, "tx1:0")[0].dedup_key
        );
        // A one-shot event ignores the source event id.
        let a = build_task_events(TaskEventKind::Delivered, &task(), 1, "tx1:0");
        let b = build_task_events(TaskEventKind::Delivered, &task(), 1, "tx9:3");
        assert_eq!(a[0].dedup_key, b[0].dedup_key);
    }

    #[test]
    fn time_driven_events_go_to_whoever_benefits() {
        assert_eq!(
            summary(&rows(TaskEventKind::ExpiredOfferDue, &task())),
            [("expired_offer_due".into(), "0xbuyer_account".into())]
        );
        assert_eq!(
            summary(&rows(TaskEventKind::RefundDue, &task())),
            [("refund_due".into(), "0xbuyer_account".into())]
        );
        assert_eq!(
            summary(&rows(TaskEventKind::ReleaseDue, &task())),
            [("release_due".into(), "0xseller_account".into())]
        );
    }

    #[test]
    fn closing_events_reach_both_parties() {
        for kind in [TaskEventKind::Cancelled, TaskEventKind::Refunded, TaskEventKind::RefundCleared] {
            let events = rows(kind, &task());
            assert_eq!(events.len(), 2, "{kind:?}");
            assert_ne!(events[0].account_id, events[1].account_id);
        }
    }

    #[test]
    fn enrichment_adds_counterparty_reputation_and_open_work() {
        let mut payload = rows(TaskEventKind::Offered, &task())[0].payload.clone();
        Enrichment {
            buyer: PartyStats { score_milli: 3800, credibility_bps: 4000 },
            seller: PartyStats { score_milli: 4100, credibility_bps: 6000 },
            seller_open_tasks: 2,
        }
        .apply(&mut payload);
        assert_eq!(payload["buyer_score_milli"], 3800);
        assert_eq!(payload["buyer_credibility_bps"], 4000);
        assert_eq!(payload["seller_score_milli"], 4100);
        assert_eq!(payload["seller_credibility_bps"], 6000);
        assert_eq!(payload["seller_open_tasks"], 2);
        // Existing fields survive.
        assert_eq!(payload["task_id"], "0xtask");
    }

    #[test]
    fn the_platform_event_is_account_scoped_not_organization_scoped() {
        let event = &rows(TaskEventKind::Offered, &task())[0];
        let envelope = platform_event(&AutomationEventPayload::from(event), &event.payload);
        assert_eq!(envelope["event_version"], 1);
        assert_eq!(envelope["event_family"], "task");
        assert_eq!(envelope["event_type"], "offered");
        assert_eq!(envelope["account_id"], "0xseller_account");
        assert_eq!(envelope["agent_object_id"], "0xseller_agent");
        assert_eq!(envelope["deduplication_key"], "task:0xtask:offered:seller");
        assert_eq!(envelope["source_service"], AUTOMATION_EVENT_SOURCE);
        assert!(envelope.get("organization_id").is_none());

        let mut human = task();
        human.buyer_agent_id = None;
        let event = &rows(TaskEventKind::Delivered, &human)[0];
        let envelope = platform_event(&AutomationEventPayload::from(event), &event.payload);
        assert!(envelope.get("agent_object_id").is_none());
    }

    #[test]
    fn redelivery_backs_off_and_is_capped() {
        assert_eq!(retry_delay_secs(1), 5);
        assert_eq!(retry_delay_secs(2), 10);
        assert_eq!(retry_delay_secs(5), 80);
        assert_eq!(retry_delay_secs(8), 600);
        assert_eq!(retry_delay_secs(500), 600);
        assert_eq!(retry_delay_secs(0), 5);
    }
}
