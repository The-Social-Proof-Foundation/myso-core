// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Jury events for the automation engine.
//!
//! Two kinds of recipient, two ways events are produced:
//!
//! * **Jurors** are told what to do about their own duty. The indexer queues `duty_drafted` and
//!   `voting_started` in the transaction that applies the chain event, and the social server's scan
//!   adds `duty_poll` (a heartbeat the juror runner answers) and `fee_claimable`.
//! * **Keepers** run the permissionless cranks that move a case along (draw, replace, finalize,
//!   apply the verdict). What needs cranking is a fact about the indexed state, so the server finds
//!   it by scanning and queues one event per keeper account; the indexer never needs to know who
//!   the keepers are.
//!
//! Every row carries a real `account_id` and never an organization, so the engine scopes it to
//! exactly one party. Ids are strings and amounts are decimal strings.

use serde_json::{Value, json};

use super::automation_outbox::NewAutomationEvent;

pub const AUTOMATION_FAMILY_JURY: &str = "jury";

/// Duty polls repeat on this cadence while a duty is open.
pub const DUTY_POLL_BUCKET_MS: i64 = 300_000;
/// A crank or fee claim that is still possible is queued again each hour, so one the engine could
/// not run (relayer down, delegate expired) is retried instead of being deduplicated forever.
pub const CRANK_RETRY_BUCKET_MS: i64 = 3_600_000;

/// Roster refreshes, evictions and fee sweeps: no one waits on them, so a day between repeats is
/// enough and keeps a large roster from flooding the engine.
pub const UPKEEP_RETRY_BUCKET_MS: i64 = 86_400_000;

/// Cranks anyone may run. Each maps to one permissionless entry point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrankKind {
    /// `select_jury`: evidence is over, or a round was inconclusive and a larger jury is next.
    SelectDue,
    /// `replace_no_shows`: the accept window ended with seats still unconfirmed.
    ReplaceDue,
    /// `finalize_round`: the reveal window ended, or every juror revealed.
    RoundFinalizeDue,
    /// `finalize_case`: the appeal window ended.
    CaseFinalizeDue,
    /// `execute_verdict`: a task's case is final.
    ExecuteVerdict,
    /// `start_neutral_resolution`: a task's jury failed.
    StartNeutral,
    /// `execute_neutral_resolution`: the settlement window after a failed jury ended.
    ExecuteNeutral,
    /// `settle_dispute_fees`: the dispute is over and the fee pool is unpaid.
    SettleFees,
    /// `escalate_to_jury`: a proposal's council or community step has earned a jury.
    EscalateProposal,
    /// `finalize_proposal`: a community vote has ended on a registry whose jury is switched on.
    FinalizeProposal,
    /// `apply_jury_decision`: a governance case is finished and its proposal waits.
    ApplyDecision,
    /// `refresh_roster_entry`: a juror's eligibility snapshot is due for a refresh.
    RefreshRoster,
    /// `evict_ineligible`: a juror on the roster can no longer serve.
    EvictIneligible,
    /// `sweep_jury_case`: a governance case has ended and its unclaimed fees can go back.
    SweepCase,
}

impl CrankKind {
    pub const ALL: [CrankKind; 14] = [
        CrankKind::SelectDue,
        CrankKind::ReplaceDue,
        CrankKind::RoundFinalizeDue,
        CrankKind::CaseFinalizeDue,
        CrankKind::ExecuteVerdict,
        CrankKind::StartNeutral,
        CrankKind::ExecuteNeutral,
        CrankKind::SettleFees,
        CrankKind::EscalateProposal,
        CrankKind::FinalizeProposal,
        CrankKind::ApplyDecision,
        CrankKind::RefreshRoster,
        CrankKind::EvictIneligible,
        CrankKind::SweepCase,
    ];

    pub fn event_type(self) -> &'static str {
        match self {
            CrankKind::SelectDue => "select_due",
            CrankKind::ReplaceDue => "replace_due",
            CrankKind::RoundFinalizeDue => "round_finalize_due",
            CrankKind::CaseFinalizeDue => "case_finalize_due",
            CrankKind::ExecuteVerdict => "execute_ready",
            CrankKind::StartNeutral => "neutral_start_due",
            CrankKind::ExecuteNeutral => "neutral_due",
            CrankKind::SettleFees => "fees_due",
            CrankKind::EscalateProposal => "escalate_due",
            CrankKind::FinalizeProposal => "proposal_finalize_due",
            CrankKind::ApplyDecision => "apply_ready",
            CrankKind::RefreshRoster => "roster_refresh_due",
            CrankKind::EvictIneligible => "evict_due",
            CrankKind::SweepCase => "sweep_due",
        }
    }

    /// How long a crank of this kind waits before it is queued again. A deadline crank is retried
    /// hourly because someone is waiting on it; upkeep that nobody waits on is retried daily.
    pub fn retry_bucket_ms(self) -> i64 {
        match self {
            CrankKind::RefreshRoster | CrankKind::EvictIneligible | CrankKind::SweepCase => {
                UPKEEP_RETRY_BUCKET_MS
            }
            _ => CRANK_RETRY_BUCKET_MS,
        }
    }

    /// Cranks that act on a roster entry rather than a case, task or proposal.
    pub fn is_roster_crank(self) -> bool {
        matches!(self, CrankKind::RefreshRoster | CrankKind::EvictIneligible)
    }

    /// Cranks that act on a proposal rather than a task.
    pub fn is_governance_crank(self) -> bool {
        matches!(
            self,
            CrankKind::EscalateProposal | CrankKind::FinalizeProposal | CrankKind::ApplyDecision
        )
    }

    /// Cranks that act on a task escrow, so their payload names the task.
    pub fn is_task_crank(self) -> bool {
        matches!(
            self,
            CrankKind::ExecuteVerdict
                | CrankKind::StartNeutral
                | CrankKind::ExecuteNeutral
                | CrankKind::SettleFees
        )
    }
}

/// The objects a keeper job needs to fill its action's parameters, which an event can name but
/// a registered action cannot look up.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CrankRefs {
    pub buyer_account_id: Option<String>,
    pub seller_account_id: Option<String>,
    /// The task's own platform, or the platform behind a proposal's registry; `None` for neither.
    pub platform_id: Option<String>,
    pub registry_id: Option<String>,
    /// A roster crank's agent's own MemoryAccount, which the refresh transaction takes.
    pub memory_account_id: Option<String>,
}

/// A case or proposal that can be cranked right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crank {
    pub kind: CrankKind,
    /// Empty for a crank that has no case yet (escalating or finalizing a proposal).
    pub case_id: String,
    /// The round the crank applies to; for `SelectDue` the round about to be drawn.
    pub round: i64,
    pub subject_kind: i16,
    pub subject_id: String,
    /// The deadline that made it crankable.
    pub due_ms: i64,
    /// Kind-specific: the accept deadline for `ReplaceDue` (a new draw changes it, so it keeps
    /// the key unique), 1 for `SettleFees` when the case must be closed first, and the stage
    /// (1 council, 2 community) for `EscalateProposal` and `ApplyDecision`.
    pub extra: i64,
    pub refs: CrankRefs,
}

impl Crank {
    pub fn payload(&self) -> Value {
        let mut payload = serde_json::Map::new();
        if !self.case_id.is_empty() {
            payload.insert("case_id".into(), json!(self.case_id));
        }
        payload.insert("round".into(), json!(self.round));
        payload.insert("subject_kind".into(), json!(self.subject_kind));
        payload.insert("subject_id".into(), json!(self.subject_id));
        payload.insert("due_ms".into(), json!(self.due_ms));
        if self.kind.is_task_crank() {
            payload.insert("task_id".into(), json!(self.subject_id));
            payload.insert("buyer_account_id".into(), json!(self.refs.buyer_account_id));
            payload.insert("seller_account_id".into(), json!(self.refs.seller_account_id));
            // `null` is dropped from a job's parameters, so the SDK falls back to its default platform.
            payload.insert("platform_object_id".into(), json!(self.refs.platform_id));
        }
        if self.kind.is_governance_crank() {
            payload.insert("proposal_id".into(), json!(self.subject_id));
            payload.insert("registry_id".into(), json!(self.refs.registry_id));
            payload.insert("platform_id".into(), json!(self.refs.platform_id));
            payload.insert("stage".into(), json!(self.extra));
        }
        if self.kind.is_roster_crank() {
            payload.insert("agent_id".into(), json!(self.subject_id));
            payload.insert("memory_account_id".into(), json!(self.refs.memory_account_id));
        }
        if self.kind == CrankKind::SettleFees {
            payload.insert("close_case_first".into(), json!(self.extra == 1));
        }
        Value::Object(payload)
    }

    /// The key a keeper's copy of this crank is queued under. `bucket` is the retry bucket.
    pub fn dedup_key(&self, keeper_account: &str, bucket: i64) -> String {
        // A replacement draw moves the accept deadline, and a proposal can reach the jury at two
        // steps, so each of those is a different crank with a different key.
        let discriminator = if matches!(self.kind, CrankKind::ReplaceDue | CrankKind::EscalateProposal) {
            format!(":{}", self.extra)
        } else {
            String::new()
        };
        let subject = if self.case_id.is_empty() { &self.subject_id } else { &self.case_id };
        format!(
            "jury:{}:r{}:{}{}:{}:b{}",
            subject,
            self.round,
            self.kind.event_type(),
            discriminator,
            keeper_account,
            bucket
        )
    }
}

/// One event for one keeper account.
pub fn keeper_event(crank: &Crank, keeper_account: &str, now_ms: i64) -> NewAutomationEvent {
    let bucket = now_ms / crank.kind.retry_bucket_ms();
    NewAutomationEvent {
        dedup_key: crank.dedup_key(keeper_account, bucket),
        event_family: AUTOMATION_FAMILY_JURY.to_string(),
        event_type: crank.kind.event_type().to_string(),
        account_id: keeper_account.to_string(),
        agent_object_id: None,
        payload: crank.payload(),
        occurred_at_ms: now_ms,
        source_event_id: format!(
            "crank:{}:{}",
            crank.kind.event_type(),
            if crank.case_id.is_empty() { &crank.subject_id } else { &crank.case_id }
        ),
    }
}

/// What a juror has to do next about one seat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DutyKind {
    Accept,
    Commit,
    Reveal,
}

impl DutyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            DutyKind::Accept => "accept",
            DutyKind::Commit => "commit",
            DutyKind::Reveal => "reveal",
        }
    }
}

/// A juror's seat in a case, with the account that acts for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JurorSeat {
    pub case_id: String,
    pub round: i64,
    pub agent_id: String,
    pub account_id: String,
    pub subject_kind: i16,
    pub subject_id: String,
}

impl JurorSeat {
    fn payload(&self, duty: DutyKind, deadline_ms: i64) -> serde_json::Map<String, Value> {
        let mut payload = serde_json::Map::new();
        payload.insert("case_id".into(), json!(self.case_id));
        payload.insert("round".into(), json!(self.round));
        payload.insert("agent_id".into(), json!(self.agent_id));
        payload.insert("subject_kind".into(), json!(self.subject_kind));
        payload.insert("subject_id".into(), json!(self.subject_id));
        payload.insert("duty_kind".into(), json!(duty.as_str()));
        payload.insert("deadline_ms".into(), json!(deadline_ms));
        // Acting after the deadline can only abort on chain, so the engine skips the run instead.
        payload.insert("expires_at_ms".into(), json!(deadline_ms));
        payload
    }

    fn event(
        &self,
        event_type: &str,
        dedup_key: String,
        payload: serde_json::Map<String, Value>,
        occurred_at_ms: i64,
        source_event_id: &str,
    ) -> NewAutomationEvent {
        NewAutomationEvent {
            dedup_key,
            event_family: AUTOMATION_FAMILY_JURY.to_string(),
            event_type: event_type.to_string(),
            account_id: self.account_id.clone(),
            agent_object_id: Some(self.agent_id.clone()),
            payload: Value::Object(payload),
            occurred_at_ms,
            source_event_id: source_event_id.to_string(),
        }
    }
}

/// The seat was just drawn: accept it before the accept deadline.
pub fn duty_drafted_event(
    seat: &JurorSeat,
    accept_deadline_ms: i64,
    occurred_at_ms: i64,
    source_event_id: &str,
) -> NewAutomationEvent {
    seat.event(
        "duty_drafted",
        format!("jury:{}:r{}:duty_drafted:{}", seat.case_id, seat.round, seat.agent_id),
        seat.payload(DutyKind::Accept, accept_deadline_ms),
        occurred_at_ms,
        source_event_id,
    )
}

/// Voting opened for a juror who accepted: commit before the commit deadline, reveal before the
/// reveal deadline.
pub fn voting_started_event(
    seat: &JurorSeat,
    commit_deadline_ms: i64,
    reveal_deadline_ms: i64,
    occurred_at_ms: i64,
    source_event_id: &str,
) -> NewAutomationEvent {
    let mut payload = seat.payload(DutyKind::Commit, commit_deadline_ms);
    payload.insert("commit_deadline_ms".into(), json!(commit_deadline_ms));
    payload.insert("reveal_deadline_ms".into(), json!(reveal_deadline_ms));
    seat.event(
        "voting_started",
        format!("jury:{}:r{}:voting_started:{}", seat.case_id, seat.round, seat.agent_id),
        payload,
        occurred_at_ms,
        source_event_id,
    )
}

/// A heartbeat for an open duty. The juror runner cannot finish judging inside one webhook, so it
/// starts on the first poll and answers with the vote on a later one.
pub fn duty_poll_event(
    seat: &JurorSeat,
    duty: DutyKind,
    deadline_ms: i64,
    now_ms: i64,
) -> NewAutomationEvent {
    let bucket = now_ms / DUTY_POLL_BUCKET_MS;
    seat.event(
        "duty_poll",
        format!(
            "jury:{}:r{}:duty_poll:{}:{}:b{}",
            seat.case_id,
            seat.round,
            seat.agent_id,
            duty.as_str(),
            bucket
        ),
        seat.payload(duty, deadline_ms),
        now_ms,
        &format!("poll:{}:{}", seat.case_id, seat.agent_id),
    )
}

/// The case ended and this juror took part, so its share is waiting to be claimed.
pub fn fee_claimable_event(seat: &JurorSeat, now_ms: i64) -> NewAutomationEvent {
    let bucket = now_ms / CRANK_RETRY_BUCKET_MS;
    let mut payload = serde_json::Map::new();
    payload.insert("case_id".into(), json!(seat.case_id));
    payload.insert("round".into(), json!(seat.round));
    payload.insert("agent_id".into(), json!(seat.agent_id));
    payload.insert("subject_kind".into(), json!(seat.subject_kind));
    payload.insert("subject_id".into(), json!(seat.subject_id));
    seat.event(
        "fee_claimable",
        format!("jury:{}:fee_claimable:{}:b{}", seat.case_id, seat.agent_id, bucket),
        payload,
        now_ms,
        &format!("fee:{}:{}", seat.case_id, seat.agent_id),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seat() -> JurorSeat {
        JurorSeat {
            case_id: "0xcase".into(),
            round: 1,
            agent_id: "0xagent".into(),
            account_id: "0xaccount".into(),
            subject_kind: 1,
            subject_id: "0xtask".into(),
        }
    }

    fn crank(kind: CrankKind) -> Crank {
        Crank {
            kind,
            case_id: "0xcase".into(),
            round: 2,
            subject_kind: 1,
            subject_id: "0xtask".into(),
            due_ms: 500,
            extra: 900,
            refs: CrankRefs {
                buyer_account_id: Some("0xbuyer".into()),
                seller_account_id: Some("0xseller".into()),
                platform_id: None,
                registry_id: None,
                memory_account_id: None,
            },
        }
    }

    #[test]
    fn a_drafted_juror_is_told_to_accept_before_the_deadline() {
        let e = duty_drafted_event(&seat(), 1_000, 10, "tx:0");
        assert_eq!(e.dedup_key, "jury:0xcase:r1:duty_drafted:0xagent");
        assert_eq!(e.event_family, "jury");
        assert_eq!(e.account_id, "0xaccount");
        assert_eq!(e.agent_object_id.as_deref(), Some("0xagent"));
        assert_eq!(e.payload["duty_kind"], "accept");
        assert_eq!(e.payload["expires_at_ms"], 1_000);
    }

    #[test]
    fn voting_expires_with_the_commit_window_and_names_the_reveal_deadline() {
        let e = voting_started_event(&seat(), 2_000, 3_000, 10, "tx:1");
        assert_eq!(e.payload["duty_kind"], "commit");
        assert_eq!(e.payload["expires_at_ms"], 2_000);
        assert_eq!(e.payload["reveal_deadline_ms"], 3_000);
    }

    #[test]
    fn a_poll_repeats_once_per_bucket_and_never_within_one() {
        let a = duty_poll_event(&seat(), DutyKind::Commit, 9_000_000, DUTY_POLL_BUCKET_MS * 4 + 10);
        let b = duty_poll_event(&seat(), DutyKind::Commit, 9_000_000, DUTY_POLL_BUCKET_MS * 5 - 1);
        let c = duty_poll_event(&seat(), DutyKind::Commit, 9_000_000, DUTY_POLL_BUCKET_MS * 5);
        assert_eq!(a.dedup_key, b.dedup_key);
        assert_ne!(a.dedup_key, c.dedup_key);
        assert!(a.dedup_key.contains(":commit:"));
    }

    #[test]
    fn a_reveal_poll_is_a_different_event_from_a_commit_poll() {
        let commit = duty_poll_event(&seat(), DutyKind::Commit, 9_000_000, 0);
        let reveal = duty_poll_event(&seat(), DutyKind::Reveal, 9_000_000, 0);
        assert_ne!(commit.dedup_key, reveal.dedup_key);
        assert_eq!(reveal.payload["duty_kind"], "reveal");
    }

    #[test]
    fn a_keepers_crank_is_scoped_to_that_keeper_and_retried_hourly() {
        let c = crank(CrankKind::RoundFinalizeDue);
        let a = keeper_event(&c, "0xkeeper_a", 10);
        let b = keeper_event(&c, "0xkeeper_b", 10);
        assert_ne!(a.dedup_key, b.dedup_key);
        assert_eq!(a.account_id, "0xkeeper_a");
        assert!(a.agent_object_id.is_none());
        let later = keeper_event(&c, "0xkeeper_a", CRANK_RETRY_BUCKET_MS + 1);
        assert_ne!(a.dedup_key, later.dedup_key);
        assert_eq!(a.event_type, "round_finalize_due");
    }

    #[test]
    fn a_replacement_draw_changes_the_key_through_the_new_accept_deadline() {
        let mut first = crank(CrankKind::ReplaceDue);
        first.extra = 1_000;
        let mut second = first.clone();
        second.extra = 2_000;
        assert_ne!(first.dedup_key("0xk", 0), second.dedup_key("0xk", 0));
        assert_eq!(first.dedup_key("0xk", 0), first.dedup_key("0xk", 0));
    }

    #[test]
    fn only_task_cranks_name_a_task_and_a_fee_payout_says_whether_to_close_first() {
        let verdict = crank(CrankKind::ExecuteVerdict).payload();
        assert_eq!(verdict["task_id"], "0xtask");
        let select = crank(CrankKind::SelectDue).payload();
        assert!(select.get("task_id").is_none());
        let mut fees = crank(CrankKind::SettleFees);
        fees.extra = 1;
        assert_eq!(fees.payload()["close_case_first"], true);
        fees.extra = 0;
        assert_eq!(fees.payload()["close_case_first"], false);
    }

    #[test]
    fn a_task_crank_carries_the_accounts_a_job_cannot_look_up() {
        let payload = crank(CrankKind::ExecuteVerdict).payload();
        assert_eq!(payload["buyer_account_id"], "0xbuyer");
        assert_eq!(payload["seller_account_id"], "0xseller");
        assert!(payload["platform_object_id"].is_null());
        assert!(crank(CrankKind::SelectDue).payload().get("buyer_account_id").is_none());
    }

    #[test]
    fn a_proposal_crank_names_the_proposal_and_has_no_case_yet() {
        let mut escalate = crank(CrankKind::EscalateProposal);
        escalate.case_id = String::new();
        escalate.subject_kind = 2;
        escalate.subject_id = "0xprop".into();
        escalate.extra = 2;
        escalate.refs = CrankRefs {
            registry_id: Some("0xreg".into()),
            platform_id: Some("0xplat".into()),
            ..Default::default()
        };
        let payload = escalate.payload();
        assert!(payload.get("case_id").is_none());
        assert_eq!(payload["proposal_id"], "0xprop");
        assert_eq!(payload["registry_id"], "0xreg");
        assert_eq!(payload["platform_id"], "0xplat");
        assert_eq!(payload["stage"], 2);
        assert!(payload.get("task_id").is_none());
        // Keyed by the proposal, and a different step is a different crank.
        assert!(escalate.dedup_key("0xk", 0).starts_with("jury:0xprop:"));
        let mut council = escalate.clone();
        council.extra = 1;
        assert_ne!(escalate.dedup_key("0xk", 0), council.dedup_key("0xk", 0));
    }

    #[test]
    fn roster_upkeep_names_the_agent_and_its_own_account_and_repeats_daily() {
        let mut refresh = crank(CrankKind::RefreshRoster);
        refresh.case_id = String::new();
        refresh.round = 0;
        refresh.subject_kind = 3;
        refresh.subject_id = "0xagent".into();
        refresh.refs = CrankRefs { memory_account_id: Some("0xacct".into()), ..Default::default() };
        let payload = refresh.payload();
        assert_eq!(payload["agent_id"], "0xagent");
        assert_eq!(payload["memory_account_id"], "0xacct");
        assert!(payload.get("case_id").is_none());
        assert!(payload.get("task_id").is_none());
        assert!(refresh.dedup_key("0xk", 5).starts_with("jury:0xagent:"));
        // Not re-queued inside a day, queued again the next day; a deadline crank still repeats hourly.
        let at = |kind: CrankKind, ms: i64| keeper_event(&Crank { kind, ..refresh.clone() }, "0xk", ms).dedup_key;
        let day = UPKEEP_RETRY_BUCKET_MS;
        assert_eq!(at(CrankKind::RefreshRoster, day + 1), at(CrankKind::RefreshRoster, day + 3_600_000));
        assert_ne!(at(CrankKind::RefreshRoster, day + 1), at(CrankKind::RefreshRoster, 2 * day + 1));
        assert_ne!(at(CrankKind::SelectDue, day + 1), at(CrankKind::SelectDue, day + 3_600_001));
        assert_eq!(CrankKind::EvictIneligible.event_type(), "evict_due");
        assert_eq!(CrankKind::SweepCase.event_type(), "sweep_due");
    }

    #[test]
    fn every_crank_kind_has_a_distinct_event_type() {
        let mut names: Vec<&str> = CrankKind::ALL.iter().map(|k| k.event_type()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), CrankKind::ALL.len());
    }

    #[test]
    fn a_fee_claim_is_retried_hourly_until_paid() {
        let a = fee_claimable_event(&seat(), 10);
        let b = fee_claimable_event(&seat(), CRANK_RETRY_BUCKET_MS * 2);
        assert_ne!(a.dedup_key, b.dedup_key);
        assert_eq!(a.event_type, "fee_claimable");
    }
}
