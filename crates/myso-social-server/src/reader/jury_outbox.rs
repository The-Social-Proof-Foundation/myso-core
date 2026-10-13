// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! The jury's time- and state-driven automation events.
//!
//! No chain event announces that a reveal window ended or that a verdict is waiting to be applied,
//! so these are found by scanning the indexed state. Everything here is a fact about a case that
//! the chain would accept as an action right now; the engine's job runs it, and the chain stays the
//! authority (a crank someone else already ran is an expected abort, not a failure).

use diesel::sql_types::{BigInt, Nullable, SmallInt, Text};
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    duty_poll_event, fee_claimable_event, keeper_event, Crank, CrankKind, CrankRefs, DutyKind,
    JurorSeat, NewAutomationEvent,
};
use myso_indexer_alt_social_schema::schema::automation_event_outbox;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::{JuryHealthInfo, OverdueCrank, RosterHealth};

/// Rows per insert; a scan can produce one row per keeper per crank.
const INSERT_CHUNK: usize = 500;

#[derive(QueryableByName)]
struct CrankRow {
    #[diesel(sql_type = Text)]
    case_id: String,
    #[diesel(sql_type = BigInt)]
    round_no: i64,
    #[diesel(sql_type = SmallInt)]
    subject_kind: i16,
    #[diesel(sql_type = Text)]
    subject_id: String,
    #[diesel(sql_type = BigInt)]
    due_ms: i64,
    #[diesel(sql_type = BigInt)]
    extra: i64,
    #[diesel(sql_type = Nullable<Text>)]
    registry_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    platform_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    buyer_account_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    seller_account_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    memory_account_id: Option<String>,
}

/// The columns every crank query ends with, for the cranks that name no task or registry.
const NO_REFS: &str = "NULL::text AS registry_id, NULL::text AS platform_id,
                    NULL::text AS buyer_account_id, NULL::text AS seller_account_id,
                    NULL::text AS memory_account_id";

/// A task's own accounts and platform, which the keeper job cannot look up.
const TASK_REFS: &str = "NULL::text AS registry_id, t.platform_id AS platform_id,
                    t.buyer_memory_account_id AS buyer_account_id,
                    t.seller_memory_account_id AS seller_account_id,
                    NULL::text AS memory_account_id";

/// The platform behind a proposal's registry, if it has one (global registries have none).
const PROPOSAL_REFS: &str = "p.governance_registry_id AS registry_id,
                    (SELECT pl.platform_id FROM platforms pl
                      WHERE pl.governance_registry_id = p.governance_registry_id
                        AND pl.deleted_at IS NULL LIMIT 1) AS platform_id,
                    NULL::text AS buyer_account_id, NULL::text AS seller_account_id,
                    NULL::text AS memory_account_id";

/// A roster entry's own MemoryAccount: the refresh transaction takes the agent's account, not its owner's.
const ROSTER_REFS: &str = "NULL::text AS registry_id, NULL::text AS platform_id,
                    NULL::text AS buyer_account_id, NULL::text AS seller_account_id,
                    sa.account_id AS memory_account_id";

/// Upkeep cranks that are repeated daily need a pool of work no one is waiting on, so they leave a
/// day for jurors to claim before a governance case's leftover fees are swept, and stop after a month.
const SWEEP_AFTER_MS: i64 = 3 * 86_400_000;
const SWEEP_UNTIL_MS: i64 = 30 * 86_400_000;
/// `CAP_JURY`: an agent without it can no longer serve.
const CAP_JURY: i64 = 262_144;

/// The cases and proposals a crank of this kind can act on. `$1` is now in ms, `$2` the batch
/// size. Every query is ordered by deadline so the oldest stuck one is served first.
pub(crate) fn crank_sql(kind: CrankKind) -> String {
    match kind {
        // Evidence is over, or a round was inconclusive and a larger jury is next. The round to
        // be drawn is the number of rounds already drawn.
        CrankKind::SelectDue => format!(
            "SELECT c.case_id,
                    (SELECT COUNT(*) FROM jury_rounds r WHERE r.case_id = c.case_id) AS round_no,
                    c.subject_kind, c.subject_id,
                    c.evidence_deadline_ms AS due_ms, 0::bigint AS extra, {NO_REFS}
               FROM jury_cases c
              WHERE (c.state = 1 AND c.evidence_deadline_ms < $1) OR c.state = 2
              ORDER BY c.evidence_deadline_ms, c.case_id LIMIT $2"
        ),
        // The accept window ended with seats still unconfirmed. A replacement draw moves the
        // deadline, which is why it is part of the event key.
        CrankKind::ReplaceDue => format!(
            "SELECT c.case_id, r.round AS round_no, c.subject_kind, c.subject_id,
                    r.accept_deadline_ms AS due_ms, r.accept_deadline_ms AS extra, {NO_REFS}
               FROM jury_cases c
               JOIN jury_rounds r ON r.case_id = c.case_id AND r.round = c.current_round
              WHERE c.state = 3 AND r.accept_deadline_ms < $1
              ORDER BY r.accept_deadline_ms, c.case_id LIMIT $2"
        ),
        // The reveal window ended, or every seat that voted has revealed.
        CrankKind::RoundFinalizeDue => format!(
            "SELECT c.case_id, r.round AS round_no, c.subject_kind, c.subject_id,
                    r.reveal_deadline_ms AS due_ms, 0::bigint AS extra, {NO_REFS}
               FROM jury_cases c
               JOIN jury_rounds r ON r.case_id = c.case_id AND r.round = c.current_round
              WHERE c.state = 4 AND r.reveal_deadline_ms IS NOT NULL
                AND (r.reveal_deadline_ms < $1
                     OR (EXISTS (SELECT 1 FROM jury_slots s
                                  WHERE s.case_id = c.case_id AND s.round = r.round AND s.status = 4)
                         AND NOT EXISTS (SELECT 1 FROM jury_slots s
                                          WHERE s.case_id = c.case_id AND s.round = r.round
                                            AND s.status IN (2, 3))))
              ORDER BY r.reveal_deadline_ms, c.case_id LIMIT $2"
        ),
        // The appeal window ended.
        CrankKind::CaseFinalizeDue => format!(
            "SELECT c.case_id, r.round AS round_no, c.subject_kind, c.subject_id,
                    r.appeal_deadline_ms AS due_ms, 0::bigint AS extra, {NO_REFS}
               FROM jury_cases c
               JOIN jury_rounds r ON r.case_id = c.case_id AND r.round = c.current_round
              WHERE c.state = 5 AND r.appeal_deadline_ms IS NOT NULL AND r.appeal_deadline_ms < $1
              ORDER BY r.appeal_deadline_ms, c.case_id LIMIT $2"
        ),
        CrankKind::ExecuteVerdict => format!(
            "SELECT c.case_id, c.current_round AS round_no, c.subject_kind, c.subject_id,
                    COALESCE(c.finalized_at_ms, 0) AS due_ms, 0::bigint AS extra, {TASK_REFS}
               FROM task_disputes d
               JOIN jury_cases c ON c.case_id = d.case_id
               JOIN agent_tasks t ON t.task_id = c.subject_id
              WHERE c.subject_kind = 1 AND c.state = 6 AND d.state = 1 AND $1 >= 0
              ORDER BY c.finalized_at_ms, c.case_id LIMIT $2"
        ),
        CrankKind::StartNeutral => format!(
            "SELECT c.case_id, c.current_round AS round_no, c.subject_kind, c.subject_id,
                    COALESCE(c.finalized_at_ms, 0) AS due_ms, 0::bigint AS extra, {TASK_REFS}
               FROM task_disputes d
               JOIN jury_cases c ON c.case_id = d.case_id
               JOIN agent_tasks t ON t.task_id = c.subject_id
              WHERE c.state = 7 AND d.state = 1 AND d.neutral_window_ends_ms IS NULL AND $1 >= 0
              ORDER BY c.finalized_at_ms, c.case_id LIMIT $2"
        ),
        CrankKind::ExecuteNeutral => format!(
            "SELECT c.case_id, c.current_round AS round_no, c.subject_kind, c.subject_id,
                    d.neutral_window_ends_ms AS due_ms, 0::bigint AS extra, {TASK_REFS}
               FROM task_disputes d
               JOIN jury_cases c ON c.case_id = d.case_id
               JOIN agent_tasks t ON t.task_id = c.subject_id
              WHERE d.state = 1 AND d.neutral_window_ends_ms IS NOT NULL
                AND d.neutral_window_ends_ms < $1
              ORDER BY d.neutral_window_ends_ms, c.case_id LIMIT $2"
        ),
        // The dispute ended and the fee pool is unpaid. A case still open (an agreement ended the
        // dispute) must be closed first, which `extra = 1` tells the keeper.
        CrankKind::SettleFees => format!(
            "SELECT c.case_id, c.current_round AS round_no, c.subject_kind, c.subject_id,
                    COALESCE(d.resolved_at_ms, d.opened_at_ms) AS due_ms,
                    (CASE WHEN c.state IN (6, 7, 8) THEN 0 ELSE 1 END)::bigint AS extra, {TASK_REFS}
               FROM task_disputes d
               JOIN jury_cases c ON c.case_id = d.case_id
               JOIN agent_tasks t ON t.task_id = c.subject_id
              WHERE d.state IN (2, 3) AND NOT d.fees_settled AND $1 >= 0
              ORDER BY COALESCE(d.resolved_at_ms, d.opened_at_ms), c.case_id LIMIT $2"
        ),
        // A proposal that has earned a jury and has none yet: the community vote ended with no
        // quorum or a tie (the chain recorded it as waiting), or the council step has a referral
        // majority, every current delegate has acted without a majority (a tie), or time ran out. The
        // delegate count comes from the latest indexed row per delegate, so a panel rotation the
        // indexer has not seen yet can make a tie look early: the chain aborts such a crank, which
        // the engine records as a skip. `GREATEST(submission, last policy update)` is
        // never earlier than the chain's own start of the window, so a crank is never early.
        CrankKind::EscalateProposal => format!(
            "SELECT '' AS case_id, 0::bigint AS round_no, 2::smallint AS subject_kind, p.id AS subject_id,
                    COALESCE(p.voting_end_time, 0) AS due_ms, 2::bigint AS extra, {PROPOSAL_REFS}
               FROM proposal_jury_reviews rv
               JOIN proposals p ON p.id = rv.proposal_id
              WHERE rv.stage = 2 AND rv.state = 2 AND p.status = 2 AND $1 >= 0
             UNION ALL
             SELECT '' AS case_id, 0::bigint AS round_no, 2::smallint AS subject_kind, p.id AS subject_id,
                    p.submission_time AS due_ms, 1::bigint AS extra, {PROPOSAL_REFS}
               FROM proposals p
               JOIN registry_jury_policies pol ON pol.registry_id = p.governance_registry_id
                                              AND pol.council_stage_enabled
               LEFT JOIN proposal_jury_reviews rv ON rv.proposal_id = p.id AND rv.stage = 1
               CROSS JOIN LATERAL (
                    SELECT COUNT(*) AS total, COUNT(*) FILTER (WHERE x.acted) AS acted
                      FROM (SELECT cd.address,
                                   (EXISTS (SELECT 1 FROM delegate_votes v
                                             WHERE v.proposal_id = p.id AND v.delegate_address = cd.address)
                                    OR EXISTS (SELECT 1 FROM proposal_jury_referrals r
                                                WHERE r.proposal_id = p.id AND r.delegate_address = cd.address)) AS acted
                              FROM (SELECT DISTINCT ON (address) address, is_active FROM delegates
                                     WHERE governance_registry_id = p.governance_registry_id
                                     ORDER BY address, time DESC) cd
                             WHERE cd.is_active) x
                    ) council
              WHERE p.status = 1 AND (rv.state IS NULL OR rv.state = 1)
                AND p.reward_pool >= pol.min_pool_for_jury_mist
                AND (COALESCE(rv.trigger, 0) = 1
                     OR (council.total > 0 AND council.acted >= council.total)
                     OR (pol.council_window_ms > 0
                         AND $1 > GREATEST(p.submission_time, pol.updated_at_ms) + pol.council_window_ms))
              ORDER BY due_ms, subject_id LIMIT $2"
        ),
        // A community vote has ended on a registry whose jury is on, so ending it is what lets a
        // no-quorum or tied vote reach the jury. Normal outcomes are unchanged.
        CrankKind::FinalizeProposal => format!(
            "SELECT '' AS case_id, 0::bigint AS round_no, 2::smallint AS subject_kind, p.id AS subject_id,
                    COALESCE(p.voting_end_time, 0) AS due_ms, 0::bigint AS extra, {PROPOSAL_REFS}
               FROM proposals p
               JOIN registry_jury_policies pol ON pol.registry_id = p.governance_registry_id
                                              AND pol.community_stage_enabled
               LEFT JOIN proposal_jury_reviews rv ON rv.proposal_id = p.id AND rv.stage = 2
              WHERE p.status = 2 AND p.voting_end_time < $1 AND rv.proposal_id IS NULL
              ORDER BY p.voting_end_time, p.id LIMIT $2"
        ),
        // Every juror on the roster gets its eligibility snapshot refreshed daily, in an order that
        // changes each day so a roster larger than one batch is still covered over time. The chain
        // does not say when a snapshot was last taken, so this does not try to be clever.
        CrankKind::RefreshRoster => format!(
            "SELECT '' AS case_id, 0::bigint AS round_no, 3::smallint AS subject_kind,
                    r.agent_id AS subject_id, r.joined_at_ms AS due_ms, 0::bigint AS extra, {ROSTER_REFS}
               FROM jury_roster r
               JOIN sub_agents sa ON sa.agent_object_id = r.agent_id
              WHERE r.left_at_ms IS NULL AND $1 >= 0
              ORDER BY md5(r.agent_id || (($1 / 86400000)::bigint)::text) LIMIT $2"
        ),
        // A juror whose agent was deactivated, revoked, expired or lost the jury capability, and
        // who holds no open seat. The chain also needs a fresh snapshot saying so, which the daily
        // refresh provides; until then the chain aborts and the engine records a skip.
        CrankKind::EvictIneligible => format!(
            "SELECT '' AS case_id, 0::bigint AS round_no, 3::smallint AS subject_kind,
                    r.agent_id AS subject_id, r.joined_at_ms AS due_ms, 0::bigint AS extra, {ROSTER_REFS}
               FROM jury_roster r
               JOIN sub_agents sa ON sa.agent_object_id = r.agent_id
              WHERE r.left_at_ms IS NULL
                AND (NOT sa.active OR sa.revoked_at_ms IS NOT NULL
                     OR (sa.expires_at_ms IS NOT NULL AND sa.expires_at_ms <= $1)
                     OR (sa.capabilities & {CAP_JURY}) = 0)
                AND NOT EXISTS (SELECT 1 FROM jury_slots s
                                 WHERE s.agent_id = r.agent_id AND s.status IN (1, 2, 3))
              ORDER BY r.joined_at_ms, r.agent_id LIMIT $2"
        ),
        // A governance case that ended: whatever its jurors did not claim goes back to the address the
        // case named. Sweeping is idempotent on chain (an empty remainder is a no-op), so it is simply
        // tried daily within a window after the case ended.
        CrankKind::SweepCase => format!(
            "SELECT c.case_id, c.current_round AS round_no, c.subject_kind, c.subject_id,
                    c.finalized_at_ms AS due_ms, 0::bigint AS extra, {NO_REFS}
               FROM jury_cases c
              WHERE c.subject_kind = 2 AND c.state IN (6, 7)
                AND c.finalized_at_ms IS NOT NULL
                AND c.finalized_at_ms < $1 - {SWEEP_AFTER_MS}
                AND c.finalized_at_ms > $1 - {SWEEP_UNTIL_MS}
              ORDER BY c.finalized_at_ms, c.case_id LIMIT $2"
        ),
        // A governance case finished and its proposal still waits for the decision to be applied.
        CrankKind::ApplyDecision => format!(
            "SELECT c.case_id, c.current_round AS round_no, c.subject_kind, c.subject_id,
                    COALESCE(c.finalized_at_ms, 0) AS due_ms, rv.stage::bigint AS extra, {PROPOSAL_REFS}
               FROM jury_cases c
               JOIN proposal_jury_reviews rv ON rv.case_id = c.case_id AND rv.state = 3
               JOIN proposals p ON p.id = c.subject_id
              WHERE c.subject_kind = 2 AND c.state IN (6, 7) AND $1 >= 0
              ORDER BY c.finalized_at_ms, c.case_id LIMIT $2"
        ),
    }
}

/// Every crank that can run now, oldest first within each kind.
pub(crate) async fn list_cranks(db: &Db, now_ms: i64, limit: i64) -> Result<Vec<Crank>, SocialError> {
    let mut conn = db.connect().await?;
    let mut out = Vec::new();
    for kind in CrankKind::ALL {
        let rows = diesel::sql_query(crank_sql(kind))
            .bind::<BigInt, _>(now_ms)
            .bind::<BigInt, _>(limit)
            .load::<CrankRow>(&mut conn)
            .await?;
        out.extend(rows.into_iter().map(|row| Crank {
            kind,
            case_id: row.case_id,
            round: row.round_no,
            subject_kind: row.subject_kind,
            subject_id: row.subject_id,
            due_ms: row.due_ms,
            extra: row.extra,
            refs: CrankRefs {
                buyer_account_id: row.buyer_account_id,
                seller_account_id: row.seller_account_id,
                platform_id: row.platform_id,
                registry_id: row.registry_id,
                memory_account_id: row.memory_account_id,
            },
        }));
    }
    Ok(out)
}

#[derive(QueryableByName)]
struct DutyRow {
    #[diesel(sql_type = Text)]
    case_id: String,
    #[diesel(sql_type = BigInt)]
    round_no: i64,
    #[diesel(sql_type = Text)]
    agent_id: String,
    #[diesel(sql_type = Text)]
    account_id: String,
    #[diesel(sql_type = SmallInt)]
    subject_kind: i16,
    #[diesel(sql_type = Text)]
    subject_id: String,
    #[diesel(sql_type = SmallInt)]
    slot_status: i16,
    #[diesel(sql_type = BigInt)]
    deadline_ms: i64,
}

impl DutyRow {
    fn seat(&self) -> JurorSeat {
        JurorSeat {
            case_id: self.case_id.clone(),
            round: self.round_no,
            agent_id: self.agent_id.clone(),
            account_id: self.account_id.clone(),
            subject_kind: self.subject_kind,
            subject_id: self.subject_id.clone(),
        }
    }
}

/// What a seat in this status has to do next.
pub(crate) fn duty_of(slot_status: i16) -> Option<DutyKind> {
    match slot_status {
        1 => Some(DutyKind::Accept),
        2 => Some(DutyKind::Commit),
        3 => Some(DutyKind::Reveal),
        _ => None,
    }
}

/// Seats with a duty whose deadline has not passed. A seat waiting on the next phase has no
/// deadline yet and is left out. `$1` is now in ms, `$2` the batch size.
pub(crate) const OPEN_DUTIES_SQL: &str =
    "SELECT s.case_id, s.round AS round_no, s.agent_id, ro.memory_account_id AS account_id,
            c.subject_kind, c.subject_id, s.status AS slot_status,
            (CASE s.status WHEN 1 THEN s.accept_deadline_ms
                           WHEN 2 THEN r.commit_deadline_ms
                           ELSE r.reveal_deadline_ms END) AS deadline_ms
       FROM jury_slots s
       JOIN jury_cases c ON c.case_id = s.case_id AND c.current_round = s.round
       JOIN jury_rounds r ON r.case_id = s.case_id AND r.round = s.round
       JOIN jury_roster ro ON ro.agent_id = s.agent_id
      WHERE s.status IN (1, 2, 3) AND c.state IN (3, 4)
        AND (CASE s.status WHEN 1 THEN s.accept_deadline_ms
                           WHEN 2 THEN r.commit_deadline_ms
                           ELSE r.reveal_deadline_ms END) >= $1
      ORDER BY deadline_ms, s.case_id LIMIT $2";

#[derive(QueryableByName)]
struct FeeRow {
    #[diesel(sql_type = Text)]
    case_id: String,
    #[diesel(sql_type = BigInt)]
    round_no: i64,
    #[diesel(sql_type = Text)]
    agent_id: String,
    #[diesel(sql_type = Text)]
    account_id: String,
    #[diesel(sql_type = SmallInt)]
    subject_kind: i16,
    #[diesel(sql_type = Text)]
    subject_id: String,
}

/// Jurors who revealed in a case that has ended and have not been paid. One row per juror per
/// case, even if they sat in more than one round. `$1` is unused padding so every scan binds the
/// same two parameters.
pub(crate) const UNPAID_FEES_SQL: &str =
    "SELECT * FROM (
       SELECT DISTINCT ON (s.case_id, s.agent_id)
              s.case_id, s.round AS round_no, s.agent_id, ro.memory_account_id AS account_id,
              c.subject_kind, c.subject_id
         FROM jury_slots s
         JOIN jury_cases c ON c.case_id = s.case_id
         JOIN jury_roster ro ON ro.agent_id = s.agent_id
        WHERE s.status = 4 AND c.state IN (6, 7) AND $1 >= 0
          AND NOT EXISTS (SELECT 1 FROM jury_payouts p
                           WHERE p.case_id = s.case_id AND p.agent_id = s.agent_id)
        ORDER BY s.case_id, s.agent_id, s.round DESC) unpaid
      ORDER BY case_id, agent_id LIMIT $2";

async fn insert_events(
    conn: &mut diesel_async::AsyncPgConnection,
    events: &[NewAutomationEvent],
) -> Result<usize, SocialError> {
    let mut queued = 0;
    for chunk in events.chunks(INSERT_CHUNK) {
        queued += diesel::insert_into(automation_event_outbox::table)
            .values(chunk)
            .on_conflict(automation_event_outbox::dedup_key)
            .do_nothing()
            .execute(conn)
            .await?;
    }
    Ok(queued)
}

/// Queue the jury's scan-driven events: one crank event per keeper account, a heartbeat for every
/// open juror duty, and a claim reminder for every unpaid juror. Safe to run on every tick and from
/// several forwarders: the dedup keys make a repeat within a bucket a no-op.
pub(crate) async fn enqueue_jury_events(
    db: &Db,
    now_ms: i64,
    batch: i64,
    keeper_accounts: &[String],
) -> Result<usize, SocialError> {
    let mut events: Vec<NewAutomationEvent> = Vec::new();

    for crank in list_cranks(db, now_ms, batch).await? {
        events.extend(keeper_accounts.iter().map(|keeper| keeper_event(&crank, keeper, now_ms)));
    }

    let mut conn = db.connect().await?;
    let duties = diesel::sql_query(OPEN_DUTIES_SQL)
        .bind::<BigInt, _>(now_ms)
        .bind::<BigInt, _>(batch)
        .load::<DutyRow>(&mut conn)
        .await?;
    for row in &duties {
        if let Some(duty) = duty_of(row.slot_status) {
            events.push(duty_poll_event(&row.seat(), duty, row.deadline_ms, now_ms));
        }
    }

    let unpaid = diesel::sql_query(UNPAID_FEES_SQL)
        .bind::<BigInt, _>(now_ms)
        .bind::<BigInt, _>(batch)
        .load::<FeeRow>(&mut conn)
        .await?;
    for row in &unpaid {
        let seat = JurorSeat {
            case_id: row.case_id.clone(),
            round: row.round_no,
            agent_id: row.agent_id.clone(),
            account_id: row.account_id.clone(),
            subject_kind: row.subject_kind,
            subject_id: row.subject_id.clone(),
        };
        events.push(fee_claimable_event(&seat, now_ms));
    }

    insert_events(&mut conn, &events).await
}

/// Events waiting longer than this mean the engine is not receiving them (warn), or has not for a
/// long while (critical). The forwarder retries every 30 seconds, so minutes are already a problem.
const UNSENT_WARN_MS: i64 = 5 * 60_000;
const UNSENT_CRITICAL_MS: i64 = 30 * 60_000;
/// A crank this far past its deadline has outlived the keeper's hourly retries.
const OVERDUE_WARN_MS: i64 = 60 * 60_000;
const OVERDUE_CRITICAL_MS: i64 = 6 * 60 * 60_000;
/// How many due cranks to sample per kind; enough to see a backlog without reading all of it.
const HEALTH_SAMPLE: i64 = 200;

#[derive(QueryableByName)]
struct OutboxHealthRow {
    #[diesel(sql_type = BigInt)]
    unsent: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    oldest_ms: Option<i64>,
}

#[derive(QueryableByName)]
struct RosterHealthRow {
    #[diesel(sql_type = BigInt)]
    active: i64,
    #[diesel(sql_type = BigInt)]
    suspended: i64,
}

/// Deadline-driven cranks. Roster refreshes and fee sweeps are routine upkeep with no deadline
/// anyone is waiting on, so they are not counted as overdue.
fn is_deadline_crank(kind: CrankKind) -> bool {
    !matches!(kind, CrankKind::RefreshRoster | CrankKind::EvictIneligible | CrankKind::SweepCase)
}

/// Turn the measurements into a verdict and the reasons for it. Pure, so the thresholds are tested.
pub(crate) fn assess_health(
    oldest_unsent_ms: Option<i64>,
    overdue: &[OverdueCrank],
    roster: &RosterHealth,
) -> (&'static str, Vec<String>) {
    let mut critical = false;
    let mut warn = false;
    let mut reasons = Vec::new();
    if let Some(age) = oldest_unsent_ms {
        if age > UNSENT_CRITICAL_MS {
            critical = true;
            reasons.push(format!("jury events have been undelivered for {} minutes", age / 60_000));
        } else if age > UNSENT_WARN_MS {
            warn = true;
            reasons.push(format!("jury events have been undelivered for {} minutes", age / 60_000));
        }
    }
    for crank in overdue {
        if crank.oldest_overdue_ms > OVERDUE_CRITICAL_MS {
            critical = true;
            reasons.push(format!(
                "{} {} crank(s) are over {} hours late",
                crank.count,
                crank.kind,
                crank.oldest_overdue_ms / 3_600_000
            ));
        } else if crank.oldest_overdue_ms > OVERDUE_WARN_MS {
            warn = true;
            reasons.push(format!("{} {} crank(s) are over an hour late", crank.count, crank.kind));
        }
    }
    if roster.active < roster.minimum {
        warn = true;
        reasons.push(format!(
            "only {} jurors are available and task disputes need {}",
            roster.active, roster.minimum
        ));
    }
    let status = if critical { "critical" } else if warn { "warn" } else { "ok" };
    (status, reasons)
}

/// Whether the jury is keeping up: undelivered events, cranks nobody ran, and the size of the roster.
pub(crate) async fn health(db: &Db, now_ms: i64, min_roster: i64) -> Result<JuryHealthInfo, SocialError> {
    let mut conn = db.connect().await?;
    let outbox = diesel::sql_query(
        "SELECT COUNT(*) FILTER (WHERE sent_at IS NULL) AS unsent,
                MIN(occurred_at_ms) FILTER (WHERE sent_at IS NULL) AS oldest_ms
           FROM automation_event_outbox WHERE event_family = 'jury'",
    )
    .get_result::<OutboxHealthRow>(&mut conn)
    .await?;
    let roster_row = diesel::sql_query(
        "SELECT COUNT(*) FILTER (WHERE left_at_ms IS NULL AND status = 1) AS active,
                COUNT(*) FILTER (WHERE left_at_ms IS NULL AND status = 3
                                   AND COALESCE(suspended_until_ms, 0) > $1) AS suspended
           FROM jury_roster",
    )
    .bind::<BigInt, _>(now_ms)
    .get_result::<RosterHealthRow>(&mut conn)
    .await?;
    drop(conn);

    let mut overdue: Vec<OverdueCrank> = Vec::new();
    for crank in list_cranks(db, now_ms, HEALTH_SAMPLE).await? {
        if !is_deadline_crank(crank.kind) || crank.due_ms <= 0 {
            continue;
        }
        let late = now_ms - crank.due_ms;
        if late <= 0 {
            continue;
        }
        let kind = crank.kind.event_type();
        match overdue.iter_mut().find(|o| o.kind == kind) {
            Some(entry) => {
                entry.count += 1;
                entry.oldest_overdue_ms = entry.oldest_overdue_ms.max(late);
            }
            None => overdue.push(OverdueCrank { kind, count: 1, oldest_overdue_ms: late }),
        }
    }

    let oldest_unsent_ms = outbox.oldest_ms.map(|oldest| (now_ms - oldest).max(0));
    let roster = RosterHealth { active: roster_row.active, suspended: roster_row.suspended, minimum: min_roster };
    let (status, reasons) = assess_health(oldest_unsent_ms, &overdue, &roster);
    Ok(JuryHealthInfo {
        status,
        reasons,
        unsent_events: outbox.unsent,
        oldest_unsent_event_ms: oldest_unsent_ms,
        overdue_cranks: overdue,
        roster,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_crank_query_is_bounded_ordered_and_takes_now_and_a_limit() {
        for kind in CrankKind::ALL {
            let sql = crank_sql(kind);
            assert!(sql.contains("LIMIT $2"), "{kind:?} must be bounded");
            assert!(sql.contains("ORDER BY"), "{kind:?} must be ordered");
            assert!(sql.contains("$1"), "{kind:?} must bind the clock");
            for column in ["case_id", "round_no", "subject_kind", "subject_id", "due_ms", "extra"] {
                assert!(sql.contains(column), "{kind:?} must select {column}");
            }
        }
    }

    #[test]
    fn the_scans_read_only_live_case_states() {
        // States: 1 evidence, 2 awaiting draw, 3 selecting, 4 voting, 5 tallied, 6 final,
        // 7 failed, 8 cancelled.
        assert!(crank_sql(CrankKind::SelectDue).contains("c.state = 1"));
        assert!(crank_sql(CrankKind::SelectDue).contains("c.state = 2"));
        assert!(crank_sql(CrankKind::ReplaceDue).contains("c.state = 3"));
        assert!(crank_sql(CrankKind::RoundFinalizeDue).contains("c.state = 4"));
        assert!(crank_sql(CrankKind::CaseFinalizeDue).contains("c.state = 5"));
        assert!(crank_sql(CrankKind::ExecuteVerdict).contains("c.state = 6"));
        assert!(crank_sql(CrankKind::StartNeutral).contains("c.state = 7"));
    }

    #[test]
    fn a_verdict_is_applied_only_while_the_dispute_is_open() {
        for kind in [CrankKind::ExecuteVerdict, CrankKind::StartNeutral, CrankKind::ExecuteNeutral] {
            assert!(crank_sql(kind).contains("d.state = 1"), "{kind:?}");
        }
        assert!(crank_sql(CrankKind::SettleFees).contains("d.state IN (2, 3)"));
        assert!(crank_sql(CrankKind::SettleFees).contains("NOT d.fees_settled"));
    }

    #[test]
    fn a_round_is_finalized_early_only_when_every_seat_that_voted_has_revealed() {
        let sql = crank_sql(CrankKind::RoundFinalizeDue);
        assert!(sql.contains("s.status = 4"));
        assert!(sql.contains("s.status IN (2, 3)"));
    }

    #[test]
    fn a_proposal_is_cranked_only_while_a_human_step_still_holds_it() {
        let escalate = crank_sql(CrankKind::EscalateProposal);
        // Community step: waiting on the jury after a quorum miss or tie, proposal still voting.
        assert!(escalate.contains("rv.stage = 2 AND rv.state = 2 AND p.status = 2"));
        // Council step: still in review, no case yet, pool large enough, and a trigger holds.
        assert!(escalate.contains("p.status = 1"));
        assert!(escalate.contains("rv.state IS NULL OR rv.state = 1"));
        assert!(escalate.contains("min_pool_for_jury_mist"));
        // A council that has fully acted without a majority is a tie.
        assert!(escalate.contains("council.acted >= council.total"));
        let finalize = crank_sql(CrankKind::FinalizeProposal);
        assert!(finalize.contains("community_stage_enabled"));
        assert!(finalize.contains("rv.proposal_id IS NULL"));
        let apply = crank_sql(CrankKind::ApplyDecision);
        assert!(apply.contains("rv.state = 3"));
        assert!(apply.contains("c.state IN (6, 7)"));
    }

    #[test]
    fn roster_upkeep_reads_only_live_roster_entries_and_the_agents_own_account() {
        let refresh = crank_sql(CrankKind::RefreshRoster);
        assert!(refresh.contains("r.left_at_ms IS NULL"));
        assert!(refresh.contains("sa.account_id AS memory_account_id"));
        // The order changes with the day, so a large roster is covered over successive days.
        assert!(refresh.contains("md5("));
        let evict = crank_sql(CrankKind::EvictIneligible);
        assert!(evict.contains("NOT sa.active"));
        assert!(evict.contains("sa.revoked_at_ms IS NOT NULL"));
        assert!(evict.contains("sa.expires_at_ms <= $1"));
        assert!(evict.contains("(sa.capabilities & 262144) = 0"));
        // An agent with an open seat cannot be evicted, so it is not offered.
        assert!(evict.contains("s.status IN (1, 2, 3)"));
    }

    #[test]
    fn a_sweep_waits_for_jurors_to_claim_and_stops_after_a_month() {
        let sweep = crank_sql(CrankKind::SweepCase);
        assert!(sweep.contains("c.subject_kind = 2"));
        assert!(sweep.contains("c.state IN (6, 7)"));
        assert!(sweep.contains(&format!("$1 - {SWEEP_AFTER_MS}")));
        assert!(sweep.contains(&format!("$1 - {SWEEP_UNTIL_MS}")));
    }

    #[test]
    fn duties_map_to_what_the_juror_does_next() {
        assert_eq!(duty_of(1), Some(DutyKind::Accept));
        assert_eq!(duty_of(2), Some(DutyKind::Commit));
        assert_eq!(duty_of(3), Some(DutyKind::Reveal));
        assert_eq!(duty_of(4), None);
        assert_eq!(duty_of(5), None);
        assert_eq!(duty_of(6), None);
    }

    #[test]
    fn open_duties_leave_out_seats_with_no_deadline_yet() {
        assert!(OPEN_DUTIES_SQL.contains(">= $1"));
        assert!(OPEN_DUTIES_SQL.contains("LIMIT $2"));
        assert!(OPEN_DUTIES_SQL.contains("c.current_round = s.round"));
    }

    #[test]
    fn unpaid_fees_are_one_row_per_juror_per_case() {
        assert!(UNPAID_FEES_SQL.contains("DISTINCT ON (s.case_id, s.agent_id)"));
        assert!(UNPAID_FEES_SQL.contains("jury_payouts"));
        assert!(UNPAID_FEES_SQL.contains("LIMIT $2"));
    }

    #[test]
    fn health_is_ok_until_something_is_late() {
        let roster = RosterHealth { active: 25, suspended: 0, minimum: 20 };
        assert_eq!(assess_health(None, &[], &roster), ("ok", vec![]));
        assert_eq!(assess_health(Some(60_000), &[], &roster).0, "ok");
    }

    #[test]
    fn undelivered_events_warn_then_turn_critical() {
        let roster = RosterHealth { active: 25, suspended: 0, minimum: 20 };
        assert_eq!(assess_health(Some(6 * 60_000), &[], &roster).0, "warn");
        let (status, reasons) = assess_health(Some(31 * 60_000), &[], &roster);
        assert_eq!(status, "critical");
        assert!(reasons[0].contains("31 minutes"));
    }

    #[test]
    fn a_crank_nobody_ran_is_a_warning_and_then_an_emergency() {
        let roster = RosterHealth { active: 25, suspended: 0, minimum: 20 };
        let late = |ms| vec![OverdueCrank { kind: "select_due", count: 2, oldest_overdue_ms: ms }];
        assert_eq!(assess_health(None, &late(30 * 60_000), &roster).0, "ok");
        assert_eq!(assess_health(None, &late(2 * 3_600_000), &roster).0, "warn");
        let (status, reasons) = assess_health(None, &late(7 * 3_600_000), &roster);
        assert_eq!(status, "critical");
        assert!(reasons[0].contains("select_due") && reasons[0].contains("7 hours"));
    }

    #[test]
    fn a_roster_below_the_floor_warns_that_disputes_cannot_open() {
        let (status, reasons) = assess_health(None, &[], &RosterHealth { active: 12, suspended: 3, minimum: 20 });
        assert_eq!(status, "warn");
        assert!(reasons[0].contains("12") && reasons[0].contains("20"));
    }

    #[test]
    fn routine_upkeep_is_never_counted_as_late() {
        assert!(!is_deadline_crank(CrankKind::RefreshRoster));
        assert!(!is_deadline_crank(CrankKind::EvictIneligible));
        assert!(!is_deadline_crank(CrankKind::SweepCase));
        assert!(is_deadline_crank(CrankKind::SelectDue));
        assert!(is_deadline_crank(CrankKind::ApplyDecision));
    }
}
