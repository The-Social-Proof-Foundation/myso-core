// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Database side of the automation event forwarder.
//!
//! The indexer queues event-driven rows in `automation_event_outbox`. This module claims them for
//! delivery, attaches the counterparty's current reputation, records the outcome, and queues the
//! time-driven events (`expired_offer_due`, `refund_due`, `release_due`) that no chain event
//! announces.

use diesel::sql_types::{BigInt, Integer, Jsonb, Nullable, Text};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, QueryableByName, SelectableHelper};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    build_task_events, AutomationEventPayload, Enrichment, PartyStats, TaskEventKind, TaskSnapshot,
    DEFAULT_BASE_CREDIBILITY_BPS, DEFAULT_PRIOR_SCORE_MILLI,
};
use myso_indexer_alt_social_schema::schema::{
    agent_tasks, automation_event_outbox, credibility, reputation_aggregates, task_config_history,
};
use myso_pg_db::Db;

use crate::error::SocialError;

const ROLE_SELLER: i16 = 1;
const ROLE_BUYER: i16 = 2;
/// Task statuses that count as the seller having work in flight.
const OPEN_STATUSES: [i16; 2] = [1, 2];

/// A row leased for delivery.
#[derive(Debug, Clone, QueryableByName)]
pub struct ClaimedEvent {
    #[diesel(sql_type = BigInt)]
    pub id: i64,
    #[diesel(sql_type = Text)]
    pub dedup_key: String,
    #[diesel(sql_type = Text)]
    pub event_family: String,
    #[diesel(sql_type = Text)]
    pub event_type: String,
    #[diesel(sql_type = Text)]
    pub account_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub agent_object_id: Option<String>,
    #[diesel(sql_type = Jsonb)]
    pub payload: serde_json::Value,
    #[diesel(sql_type = BigInt)]
    pub occurred_at_ms: i64,
    #[diesel(sql_type = Text)]
    pub source_event_id: String,
    #[diesel(sql_type = Integer)]
    pub attempts: i32,
}

impl ClaimedEvent {
    pub fn identity(&self) -> AutomationEventPayload {
        AutomationEventPayload {
            dedup_key: self.dedup_key.clone(),
            event_family: self.event_family.clone(),
            event_type: self.event_type.clone(),
            account_id: self.account_id.clone(),
            agent_object_id: self.agent_object_id.clone(),
            occurred_at_ms: self.occurred_at_ms,
            source_event_id: self.source_event_id.clone(),
        }
    }
}

/// Lease up to `limit` due rows. The lease pushes `next_attempt_at` forward, so a forwarder that
/// crashes mid-delivery leaves the row to be retried later instead of locking it, and two
/// forwarders never take the same row.
pub(crate) async fn claim_batch(
    db: &Db,
    limit: i64,
    lease_secs: i64,
    include_jury: bool,
) -> Result<Vec<ClaimedEvent>, SocialError> {
    let mut conn = db.connect().await?;
    let rows = diesel::sql_query(
        "UPDATE automation_event_outbox
            SET attempts = attempts + 1,
                next_attempt_at = NOW() + make_interval(secs => $2::double precision)
          WHERE id IN (
                SELECT id FROM automation_event_outbox
                 WHERE sent_at IS NULL AND next_attempt_at <= NOW()
                   AND ($3 OR event_family <> 'jury')
                 ORDER BY id
                 LIMIT $1
                 FOR UPDATE SKIP LOCKED)
      RETURNING id, dedup_key, event_family, event_type, account_id, agent_object_id,
                payload, occurred_at_ms, source_event_id, attempts",
    )
    .bind::<BigInt, _>(limit)
    .bind::<BigInt, _>(lease_secs)
    .bind::<diesel::sql_types::Bool, _>(include_jury)
    .load::<ClaimedEvent>(&mut conn)
    .await?;
    let mut rows = rows;
    // RETURNING does not promise an order.
    rows.sort_by_key(|row| row.id);
    Ok(rows)
}

pub(crate) async fn mark_sent(db: &Db, id: i64) -> Result<(), SocialError> {
    let mut conn = db.connect().await?;
    diesel::sql_query(
        "UPDATE automation_event_outbox SET sent_at = NOW(), last_error = NULL WHERE id = $1",
    )
    .bind::<BigInt, _>(id)
    .execute(&mut conn)
    .await?;
    Ok(())
}

pub(crate) async fn mark_failed(
    db: &Db,
    id: i64,
    error: &str,
    retry_in_secs: i64,
) -> Result<(), SocialError> {
    let mut conn = db.connect().await?;
    diesel::sql_query(
        "UPDATE automation_event_outbox
            SET last_error = $2,
                next_attempt_at = NOW() + make_interval(secs => $3::double precision)
          WHERE id = $1",
    )
    .bind::<BigInt, _>(id)
    .bind::<Text, _>(error.chars().take(500).collect::<String>())
    .bind::<BigInt, _>(retry_in_secs)
    .execute(&mut conn)
    .await?;
    Ok(())
}

fn text(payload: &serde_json::Value, key: &str) -> Option<String> {
    payload
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

/// The counterparty reputation a job's policy reads. Unknown parties get the on-chain prior, so a
/// policy such as "buyer score at least 3.0" admits a buyer with no history instead of skipping it.
pub(crate) async fn enrichment_for(
    db: &Db,
    payload: &serde_json::Value,
) -> Result<Enrichment, SocialError> {
    let mut conn = db.connect().await?;

    let (prior, base_credibility) = task_config_history::table
        .order(task_config_history::id.desc())
        .select((
            task_config_history::prior_mean_milli,
            task_config_history::cred_base_bps,
        ))
        .first::<(i64, i64)>(&mut conn)
        .await
        .optional()?
        .unwrap_or((DEFAULT_PRIOR_SCORE_MILLI, DEFAULT_BASE_CREDIBILITY_BPS));

    let buyer_account = text(payload, "buyer_account_id").unwrap_or_default();
    let seller_account = text(payload, "seller_account_id").unwrap_or_default();
    let seller_agent = text(payload, "seller_agent_id").unwrap_or_default();
    // An agent buyer builds its own buyer score; a principal buying directly is scored on its account.
    let buyer_subject = text(payload, "buyer_agent_id").unwrap_or_else(|| buyer_account.clone());

    let score = |subject: String, role: i16| {
        reputation_aggregates::table
            .filter(reputation_aggregates::subject_id.eq(subject))
            .filter(reputation_aggregates::role.eq(role))
            .select(reputation_aggregates::bayesian_score_milli)
    };
    let buyer_score = score(buyer_subject, ROLE_BUYER)
        .first::<i64>(&mut conn)
        .await
        .optional()?
        .unwrap_or(prior);
    let seller_score = score(seller_agent.clone(), ROLE_SELLER)
        .first::<i64>(&mut conn)
        .await
        .optional()?
        .unwrap_or(prior);

    let credibility_of = |account: String| {
        credibility::table
            .filter(credibility::account_id.eq(account))
            .select(credibility::credibility_bps)
    };
    let buyer_credibility = credibility_of(buyer_account)
        .first::<i64>(&mut conn)
        .await
        .optional()?
        .unwrap_or(base_credibility);
    let seller_credibility = credibility_of(seller_account)
        .first::<i64>(&mut conn)
        .await
        .optional()?
        .unwrap_or(base_credibility);

    let seller_open_tasks = agent_tasks::table
        .filter(agent_tasks::seller_agent_id.eq(&seller_agent))
        .filter(agent_tasks::status.eq_any(OPEN_STATUSES))
        .count()
        .get_result::<i64>(&mut conn)
        .await?;

    Ok(Enrichment {
        buyer: PartyStats {
            score_milli: buyer_score,
            credibility_bps: buyer_credibility,
        },
        seller: PartyStats {
            score_milli: seller_score,
            credibility_bps: seller_credibility,
        },
        seller_open_tasks,
    })
}

/// A class of time-driven event and the query that finds tasks whose deadline has passed without
/// a queued event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DueKind {
    ExpiredOffer,
    Refund,
    Release,
}

impl DueKind {
    pub(crate) const ALL: [DueKind; 3] = [DueKind::ExpiredOffer, DueKind::Refund, DueKind::Release];

    fn event_kind(self) -> TaskEventKind {
        match self {
            DueKind::ExpiredOffer => TaskEventKind::ExpiredOfferDue,
            DueKind::Refund => TaskEventKind::RefundDue,
            DueKind::Release => TaskEventKind::ReleaseDue,
        }
    }

    /// Finds due task ids that have no outbox row yet. `$1` is now in ms, `$2` the batch size.
    /// The dedup key built here must match `build_task_events`.
    fn sql(self) -> &'static str {
        match self {
            DueKind::ExpiredOffer => {
                "SELECT t.task_id FROM agent_tasks t
                  WHERE t.status = 0 AND t.accept_deadline_ms < $1
                    AND NOT EXISTS (SELECT 1 FROM automation_event_outbox o
                                     WHERE o.dedup_key = 'task:' || t.task_id || ':expired_offer_due:buyer')
                  ORDER BY t.accept_deadline_ms LIMIT $2"
            }
            DueKind::Refund => {
                "SELECT t.task_id FROM agent_tasks t
                  WHERE t.status = 1 AND t.delivery_deadline_ms < $1
                    AND NOT EXISTS (SELECT 1 FROM automation_event_outbox o
                                     WHERE o.dedup_key = 'task:' || t.task_id || ':refund_due:buyer')
                  ORDER BY t.delivery_deadline_ms LIMIT $2"
            }
            DueKind::Release => {
                "SELECT t.task_id FROM agent_tasks t
                  WHERE t.status = 2
                    AND t.settle_deadline_ms
                        + COALESCE((SELECT release_grace_ms FROM task_config_history ORDER BY id DESC LIMIT 1),
                                   259200000) < $1
                    AND NOT EXISTS (SELECT 1 FROM automation_event_outbox o
                                     WHERE o.dedup_key = 'task:' || t.task_id || ':release_due:seller')
                  ORDER BY t.settle_deadline_ms LIMIT $2"
            }
        }
    }
}

#[derive(QueryableByName)]
struct DueTaskId {
    #[diesel(sql_type = Text)]
    task_id: String,
}

/// Queue the time-driven events for every task whose deadline has passed. Returns the number of
/// rows queued. Safe to run on every tick and from several forwarders: the dedup key makes a
/// repeat a no-op.
pub(crate) async fn enqueue_due_events(
    db: &Db,
    now_ms: i64,
    batch: i64,
) -> Result<usize, SocialError> {
    let mut conn = db.connect().await?;
    let mut queued = 0;
    for kind in DueKind::ALL {
        let ids = diesel::sql_query(kind.sql())
            .bind::<BigInt, _>(now_ms)
            .bind::<BigInt, _>(batch)
            .load::<DueTaskId>(&mut conn)
            .await?;
        for DueTaskId { task_id } in ids {
            let Some(task) = agent_tasks::table
                .filter(agent_tasks::task_id.eq(&task_id))
                .select(TaskSnapshot::as_select())
                .first::<TaskSnapshot>(&mut conn)
                .await
                .optional()?
            else {
                continue;
            };
            let source = format!("due:{kind:?}:{task_id}");
            let events = build_task_events(kind.event_kind(), &task, now_ms, &source);
            if events.is_empty() {
                continue;
            }
            queued += diesel::insert_into(automation_event_outbox::table)
                .values(&events)
                .on_conflict(automation_event_outbox::dedup_key)
                .do_nothing()
                .execute(&mut conn)
                .await?;
        }
    }
    Ok(queued)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn due_queries_build_the_same_dedup_keys_as_the_event_builder() {
        // The NOT EXISTS filter in each query only works if its key matches the row the builder
        // writes, so derive both from the same task and compare.
        let task = TaskSnapshot {
            task_id: "0xt".into(),
            status: 0,
            buyer_principal: "b".into(),
            buyer_agent_id: None,
            buyer_memory_account_id: "ba".into(),
            seller_principal: "s".into(),
            seller_agent_id: "sa".into(),
            seller_memory_account_id: "sac".into(),
            organization_id: None,
            room_group_id: None,
            platform_id: None,
            title: "t".into(),
            scope_digest: "d".into(),
            terms_digest: "d".into(),
            amount_mist: 1,
            accept_window_ms: 1,
            delivery_window_ms: 1,
            settle_window_ms: 1,
            accept_deadline_ms: 1,
            accepted_at_ms: None,
            delivery_deadline_ms: None,
            delivered_at_ms: None,
            settle_deadline_ms: None,
            settled_at_ms: None,
            review_deadline_ms: None,
            close_kind: None,
            settlement_kind: None,
            refund_kind: None,
            refund_proposer: 0,
            deliverable_digest: None,
            deliverable_uri: None,
        };
        for (kind, suffix) in [
            (DueKind::ExpiredOffer, ":expired_offer_due:buyer"),
            (DueKind::Refund, ":refund_due:buyer"),
            (DueKind::Release, ":release_due:seller"),
        ] {
            let events = build_task_events(kind.event_kind(), &task, 0, "x");
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].dedup_key, format!("task:0xt{suffix}"));
            assert!(
                kind.sql().contains(&format!("'{suffix}'")),
                "{kind:?} query must look for the key the builder writes"
            );
        }
    }

    #[test]
    fn every_due_query_is_bounded_and_ordered() {
        for kind in DueKind::ALL {
            let sql = kind.sql();
            assert!(sql.contains("LIMIT $2"), "{kind:?} must be bounded");
            assert!(sql.contains("ORDER BY"), "{kind:?} must be ordered");
            assert!(sql.contains("NOT EXISTS"), "{kind:?} must skip already queued tasks");
        }
    }

    #[test]
    fn the_release_query_defaults_to_the_on_chain_three_day_grace() {
        // Used until a TaskConfigUpdated has been indexed.
        assert!(DueKind::Release.sql().contains("259200000"));
    }
}
