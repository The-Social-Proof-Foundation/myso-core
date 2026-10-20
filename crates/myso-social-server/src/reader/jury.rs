// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Jury reads: cases with their rounds, jurors and evidence, the juror roster, a juror's duties
//! and a task's dispute. Everything here is on-chain fact; votes stay hidden until revealed.

use std::collections::HashMap;

use diesel::sql_types::{BigInt, Nullable, SmallInt, Text};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::{
    governance_registries, jury_cases, jury_evidence, jury_payouts, jury_roster, jury_rounds,
    jury_slots, proposal_jury_referrals, proposal_jury_reviews, proposal_refunds,
    registry_jury_policies, task_disputes,
};
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::{
    JurorAssignmentInfo, JurorRosterInfo, JuryCaseDetailInfo, JuryCaseInfo, JuryCaseRow,
    JuryEvidenceInfo, JuryPayoutInfo, JuryRationaleInfo, JuryRoundInfo, JuryRoundRow, JurySlotInfo,
    JurySlotRow, ProposalJuryInfo, ProposalJuryReferralInfo, ProposalJuryReviewInfo,
    ProposalJuryReviewView, ProposalRefundInfo, ProposalVoteRefundsInfo, RationaleOutcome,
    RegistryJuryPolicyInfo, RegistryJuryPolicyView, StoreRationaleRequest, TaskDisputeDetailInfo,
    TaskDisputeInfo, VoteRefundInfo,
};

const SLOT_DRAFTED: i16 = 1;
const SLOT_ACCEPTED: i16 = 2;
const SLOT_COMMITTED: i16 = 3;
const SLOT_REVEALED: i16 = 4;

/// Which cases `GET /jury/cases` returns. At least one scoping field is required.
#[derive(Debug, Default, Clone)]
pub struct CaseFilter {
    pub subject_kind: Option<i16>,
    pub subject_id: Option<String>,
    /// Case state codes (1 evidence ... 8 cancelled); empty means all.
    pub states: Vec<i16>,
}

impl CaseFilter {
    pub fn is_scoped(&self) -> bool {
        self.subject_id.is_some() || !self.states.is_empty()
    }
}

pub(crate) async fn list_cases(
    db: &Db,
    filter: &CaseFilter,
    limit: i64,
    offset: i64,
) -> Result<Vec<JuryCaseInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = jury_cases::table.into_boxed();
    if let Some(kind) = filter.subject_kind {
        query = query.filter(jury_cases::subject_kind.eq(kind));
    }
    if let Some(id) = &filter.subject_id {
        query = query.filter(jury_cases::subject_id.eq(id));
    }
    if !filter.states.is_empty() {
        query = query.filter(jury_cases::state.eq_any(filter.states.clone()));
    }
    let rows = query
        .order((jury_cases::created_at_ms.desc(), jury_cases::case_id.asc()))
        .limit(limit)
        .offset(offset)
        .select(JuryCaseRow::as_select())
        .load(&mut conn)
        .await?;
    Ok(rows.into_iter().map(JuryCaseInfo::from).collect())
}

async fn case_detail(db: &Db, case_id: &str) -> Result<Option<JuryCaseDetailInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let Some(case) = jury_cases::table
        .filter(jury_cases::case_id.eq(case_id))
        .select(JuryCaseRow::as_select())
        .first(&mut conn)
        .await
        .optional()?
    else {
        return Ok(None);
    };
    let rounds = jury_rounds::table
        .filter(jury_rounds::case_id.eq(case_id))
        .order(jury_rounds::round.asc())
        .select(JuryRoundRow::as_select())
        .load(&mut conn)
        .await?;
    let jurors = jury_slots::table
        .filter(jury_slots::case_id.eq(case_id))
        .order((jury_slots::round.asc(), jury_slots::agent_id.asc()))
        .select(JurySlotRow::as_select())
        .load(&mut conn)
        .await?;
    let evidence = jury_evidence::table
        .filter(jury_evidence::case_id.eq(case_id))
        .order(jury_evidence::idx.asc())
        .select(JuryEvidenceInfo::as_select())
        .load(&mut conn)
        .await?;
    let payouts = jury_payouts::table
        .filter(jury_payouts::case_id.eq(case_id))
        .order(jury_payouts::agent_id.asc())
        .select(JuryPayoutInfo::as_select())
        .load(&mut conn)
        .await?;
    Ok(Some(JuryCaseDetailInfo {
        case: case.into(),
        rounds: rounds.into_iter().map(JuryRoundInfo::from).collect(),
        jurors: jurors.into_iter().map(JurySlotInfo::from).collect(),
        evidence,
        payouts,
    }))
}

pub(crate) async fn get_case(
    db: &Db,
    case_id: &str,
) -> Result<Option<JuryCaseDetailInfo>, SocialError> {
    case_detail(db, case_id).await
}

pub(crate) async fn get_task_dispute(
    db: &Db,
    task_id: &str,
) -> Result<Option<TaskDisputeDetailInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let Some(dispute) = task_disputes::table
        .filter(task_disputes::task_id.eq(task_id))
        .select(TaskDisputeInfo::as_select())
        .first(&mut conn)
        .await
        .optional()?
    else {
        return Ok(None);
    };
    let case = case_detail(db, &dispute.case_id).await?;
    Ok(Some(TaskDisputeDetailInfo { dispute, case }))
}

/// Both human steps of a proposal that the jury has been asked about, and who asked at the council.
pub(crate) async fn get_proposal_jury(
    db: &Db,
    proposal_id: &str,
) -> Result<ProposalJuryInfo, SocialError> {
    let mut conn = db.connect().await?;
    let reviews = proposal_jury_reviews::table
        .filter(proposal_jury_reviews::proposal_id.eq(proposal_id))
        .order(proposal_jury_reviews::stage.asc())
        .select(ProposalJuryReviewInfo::as_select())
        .load(&mut conn)
        .await?;
    let referrals = proposal_jury_referrals::table
        .filter(proposal_jury_referrals::proposal_id.eq(proposal_id))
        .order((
            proposal_jury_referrals::referred_at_ms.asc(),
            proposal_jury_referrals::delegate_address.asc(),
        ))
        .select(ProposalJuryReferralInfo::as_select())
        .load(&mut conn)
        .await?;
    let refund = proposal_refund(&mut conn, proposal_id).await?;
    let reviews = reviews
        .into_iter()
        .map(|review| ProposalJuryReviewView {
            resolved_by_wait: review.resolved_by_wait(),
            review,
        })
        .collect();
    Ok(ProposalJuryInfo {
        reviews,
        referrals,
        refund,
    })
}

async fn proposal_refund(
    conn: &mut diesel_async::AsyncPgConnection,
    proposal_id: &str,
) -> Result<Option<ProposalRefundInfo>, SocialError> {
    Ok(proposal_refunds::table
        .filter(proposal_refunds::proposal_id.eq(proposal_id))
        .select(ProposalRefundInfo::as_select())
        .first(conn)
        .await
        .optional()?)
}

/// Every voter who paid on a refunded proposal, with their pro-rata share of the voter pool (the
/// chain's own formula) and when they claimed it.
const VOTE_REFUNDS_SQL: &str = "SELECT paid.voter,
            COALESCE(c.amount_mist,
                     (paid.paid * rf.voter_refund_pool_mist / NULLIF(totals.total, 0))::bigint, 0)
              AS amount_mist,
            c.claimed_at_ms
       FROM proposal_refunds rf
       JOIN (SELECT proposal_id, voter_address AS voter, SUM(vote_cost)::numeric AS paid
               FROM community_votes WHERE proposal_id = $1 AND vote_cost > 0
              GROUP BY proposal_id, voter_address) paid ON paid.proposal_id = rf.proposal_id
       JOIN (SELECT proposal_id, SUM(vote_cost)::numeric AS total
               FROM community_votes WHERE proposal_id = $1 AND vote_cost > 0
              GROUP BY proposal_id) totals ON totals.proposal_id = rf.proposal_id
       LEFT JOIN proposal_vote_refunds c ON c.proposal_id = rf.proposal_id AND c.voter = paid.voter
      WHERE rf.proposal_id = $1
      ORDER BY paid.voter";

pub(crate) async fn get_proposal_vote_refunds(
    db: &Db,
    proposal_id: &str,
) -> Result<ProposalVoteRefundsInfo, SocialError> {
    let mut conn = db.connect().await?;
    let refund = proposal_refund(&mut conn, proposal_id).await?;
    let voters = if refund.is_some() {
        diesel::sql_query(VOTE_REFUNDS_SQL)
            .bind::<Text, _>(proposal_id)
            .load::<VoteRefundInfo>(&mut conn)
            .await?
    } else {
        Vec::new()
    };
    Ok(ProposalVoteRefundsInfo { refund, voters })
}

pub(crate) async fn get_registry_jury_policy(
    db: &Db,
    registry_id: &str,
) -> Result<Option<RegistryJuryPolicyView>, SocialError> {
    let mut conn = db.connect().await?;
    let policy = registry_jury_policies::table
        .filter(registry_jury_policies::registry_id.eq(registry_id))
        .select(RegistryJuryPolicyInfo::as_select())
        .first(&mut conn)
        .await
        .optional()?;
    let Some(policy) = policy else {
        return Ok(None);
    };
    let community_quorum_votes = governance_registries::table
        .filter(governance_registries::registry_id.eq(registry_id))
        .select(governance_registries::quorum_votes)
        .first::<i64>(&mut conn)
        .await
        .optional()?;
    Ok(Some(RegistryJuryPolicyView {
        policy,
        community_quorum_votes,
    }))
}

pub(crate) async fn list_roster(
    db: &Db,
    owner: Option<&str>,
    statuses: &[i16],
    limit: i64,
    offset: i64,
) -> Result<Vec<JurorRosterInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = jury_roster::table.into_boxed();
    if let Some(owner) = owner {
        query = query.filter(jury_roster::owner.eq(owner));
    }
    if !statuses.is_empty() {
        query = query.filter(jury_roster::status.eq_any(statuses.to_vec()));
    }
    Ok(query
        .order((
            jury_roster::joined_at_ms.desc(),
            jury_roster::agent_id.asc(),
        ))
        .limit(limit)
        .offset(offset)
        .select(JurorRosterInfo::as_select())
        .load(&mut conn)
        .await?)
}

pub(crate) async fn get_juror(
    db: &Db,
    agent_id: &str,
) -> Result<Option<JurorRosterInfo>, SocialError> {
    let mut conn = db.connect().await?;
    Ok(jury_roster::table
        .filter(jury_roster::agent_id.eq(agent_id))
        .select(JurorRosterInfo::as_select())
        .first(&mut conn)
        .await
        .optional()?)
}

/// Slots with nothing left to do are history; the rest are duties to act on.
pub(crate) fn is_active_status(status: i16) -> bool {
    matches!(status, SLOT_DRAFTED | SLOT_ACCEPTED | SLOT_COMMITTED)
}

/// What the juror must do for this slot and when it closes. A juror who accepted has nothing to
/// commit until the round's voting window opens.
pub(crate) fn duty_for(
    slot: &JurySlotRow,
    round: Option<&JuryRoundRow>,
) -> (Option<&'static str>, Option<i64>) {
    match slot.status {
        SLOT_DRAFTED => (Some("accept"), Some(slot.accept_deadline_ms)),
        SLOT_ACCEPTED => match round.and_then(|r| r.commit_deadline_ms) {
            Some(deadline) => (Some("commit"), Some(deadline)),
            None => (None, None),
        },
        SLOT_COMMITTED => match round.and_then(|r| r.reveal_deadline_ms) {
            Some(deadline) => (Some("reveal"), Some(deadline)),
            None => (None, None),
        },
        _ => (None, None),
    }
}

/// A juror's slots. With only active statuses requested (the default) the result is every open
/// duty ordered by the next deadline; otherwise it is history, newest case first.
pub(crate) async fn list_assignments(
    db: &Db,
    agent_id: &str,
    statuses: &[i16],
    limit: i64,
    offset: i64,
) -> Result<Vec<JurorAssignmentInfo>, SocialError> {
    let active_only = statuses.iter().all(|s| is_active_status(*s));
    let mut conn = db.connect().await?;
    let mut query = jury_slots::table
        .inner_join(jury_cases::table)
        .filter(jury_slots::agent_id.eq(agent_id))
        .into_boxed();
    if statuses.is_empty() {
        query = query.filter(jury_slots::status.eq_any(vec![
            SLOT_DRAFTED,
            SLOT_ACCEPTED,
            SLOT_COMMITTED,
        ]));
    } else {
        query = query.filter(jury_slots::status.eq_any(statuses.to_vec()));
    }
    let query = query.select((JurySlotRow::as_select(), JuryCaseRow::as_select()));
    let pairs: Vec<(JurySlotRow, JuryCaseRow)> = if active_only {
        query.load(&mut conn).await?
    } else {
        query
            .order((jury_cases::created_at_ms.desc(), jury_slots::case_id.asc()))
            .limit(limit)
            .offset(offset)
            .load(&mut conn)
            .await?
    };
    let case_ids: Vec<String> = pairs.iter().map(|(s, _)| s.case_id.clone()).collect();
    let rounds: HashMap<(String, i64), JuryRoundRow> = if case_ids.is_empty() {
        HashMap::new()
    } else {
        jury_rounds::table
            .filter(jury_rounds::case_id.eq_any(case_ids))
            .select(JuryRoundRow::as_select())
            .load::<JuryRoundRow>(&mut conn)
            .await?
            .into_iter()
            .map(|r| ((r.case_id.clone(), r.round), r))
            .collect()
    };
    let mut out: Vec<JurorAssignmentInfo> = pairs
        .into_iter()
        .map(|(slot, case)| {
            let round = rounds.get(&(slot.case_id.clone(), slot.round));
            let (duty_kind, deadline_ms) = duty_for(&slot, round);
            JurorAssignmentInfo {
                revealed: slot.status == SLOT_REVEALED,
                case_id: slot.case_id,
                round: slot.round,
                agent_id: slot.agent_id,
                status: slot.status,
                duty_kind,
                deadline_ms,
                commitment: slot.commitment,
                case_state: case.state,
                subject_kind: case.subject_kind,
                subject_id: case.subject_id,
            }
        })
        .collect();
    if active_only {
        out.sort_by(|a, b| {
            let key = |x: &JurorAssignmentInfo| {
                (x.deadline_ms.is_none(), x.deadline_ms, x.case_id.clone())
            };
            key(a).cmp(&key(b))
        });
        let start = usize::try_from(offset).unwrap_or(0).min(out.len());
        let end = start
            .saturating_add(usize::try_from(limit).unwrap_or(0))
            .min(out.len());
        out = out.drain(start..end).collect();
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(status: i16) -> JurySlotRow {
        JurySlotRow {
            case_id: "0xca5e".into(),
            round: 0,
            agent_id: "0xa1".into(),
            status,
            accept_deadline_ms: 100,
            accepted_at_ms: None,
            commitment: None,
            outcome: None,
            rationale_digest: None,
            evidence_refs: None,
            bundle_digest: None,
            revealed_at_ms: None,
        }
    }

    fn round(commit: Option<i64>, reveal: Option<i64>) -> JuryRoundRow {
        JuryRoundRow {
            case_id: "0xca5e".into(),
            round: 0,
            accept_deadline_ms: 100,
            commit_deadline_ms: commit,
            reveal_deadline_ms: reveal,
            tally: None,
            abstain: None,
            revealed: None,
            decisive: None,
            leading: None,
            appeal_deadline_ms: None,
        }
    }

    #[test]
    fn a_drafted_juror_must_accept_before_the_accept_deadline() {
        assert_eq!(
            duty_for(&slot(SLOT_DRAFTED), None),
            (Some("accept"), Some(100))
        );
    }

    #[test]
    fn an_accepted_juror_waits_until_voting_opens_then_commits() {
        assert_eq!(
            duty_for(&slot(SLOT_ACCEPTED), Some(&round(None, None))),
            (None, None)
        );
        assert_eq!(
            duty_for(&slot(SLOT_ACCEPTED), Some(&round(Some(500), Some(900)))),
            (Some("commit"), Some(500))
        );
    }

    #[test]
    fn a_committed_juror_reveals_before_the_reveal_deadline() {
        assert_eq!(
            duty_for(&slot(SLOT_COMMITTED), Some(&round(Some(500), Some(900)))),
            (Some("reveal"), Some(900))
        );
    }

    #[test]
    fn finished_slots_have_no_duty() {
        assert_eq!(
            duty_for(&slot(SLOT_REVEALED), Some(&round(Some(1), Some(2)))),
            (None, None)
        );
        assert_eq!(duty_for(&slot(5), None), (None, None));
    }

    #[test]
    fn only_open_statuses_count_as_active() {
        assert!(is_active_status(1) && is_active_status(2) && is_active_status(3));
        assert!(!is_active_status(4) && !is_active_status(5) && !is_active_status(6));
    }

    #[test]
    fn a_case_listing_needs_a_subject_or_a_state() {
        assert!(!CaseFilter::default().is_scoped());
        assert!(CaseFilter {
            subject_id: Some("0x1".into()),
            ..Default::default()
        }
        .is_scoped());
        assert!(CaseFilter {
            states: vec![3],
            ..Default::default()
        }
        .is_scoped());
    }
}

#[derive(diesel::QueryableByName)]
struct RationaleCounts {
    #[diesel(sql_type = BigInt)]
    stored: i64,
    #[diesel(sql_type = BigInt)]
    revealed: i64,
}

/// Save a juror's published rationale, but only if it is the text behind the vote it revealed: the
/// check is done in the database against the digest the chain recorded for that seat, so a runner
/// (or anyone holding the secret) cannot attach a different explanation to someone's vote.
pub(crate) async fn store_rationale(
    db: &Db,
    req: &StoreRationaleRequest,
    now_ms: i64,
) -> Result<RationaleOutcome, SocialError> {
    use diesel::sql_query;
    let mut conn = db.connect().await?;
    let case_id = req.case_id.to_lowercase();
    let agent_id = req.agent_id.to_lowercase();
    let inserted = sql_query(
        "INSERT INTO jury_rationales (case_id, round, agent_id, rationale, rationale_digest, stored_at_ms)
         SELECT s.case_id, s.round, s.agent_id, $4, s.rationale_digest, $5
           FROM jury_slots s
          WHERE lower(s.case_id) = $1 AND s.round = $2 AND lower(s.agent_id) = $3 AND s.status = 4
            AND s.rationale_digest = encode(sha256(convert_to($4, 'UTF8')), 'hex')
         ON CONFLICT (case_id, round, agent_id) DO NOTHING",
    )
    .bind::<Text, _>(&case_id)
    .bind::<BigInt, _>(req.round)
    .bind::<Text, _>(&agent_id)
    .bind::<Text, _>(&req.rationale)
    .bind::<BigInt, _>(now_ms)
    .execute(&mut conn)
    .await?;
    if inserted > 0 {
        return Ok(RationaleOutcome::Stored);
    }
    let counts = sql_query(
        "SELECT (SELECT COUNT(*) FROM jury_rationales r
                  WHERE lower(r.case_id) = $1 AND r.round = $2 AND lower(r.agent_id) = $3) AS stored,
                (SELECT COUNT(*) FROM jury_slots s
                  WHERE lower(s.case_id) = $1 AND s.round = $2 AND lower(s.agent_id) = $3 AND s.status = 4) AS revealed",
    )
    .bind::<Text, _>(&case_id)
    .bind::<BigInt, _>(req.round)
    .bind::<Text, _>(&agent_id)
    .get_result::<RationaleCounts>(&mut conn)
    .await?;
    Ok(rationale_outcome_when_not_inserted(
        counts.stored,
        counts.revealed,
    ))
}

/// What a rationale that was not inserted means: saved before, not yet revealed, or not the right text.
pub(crate) fn rationale_outcome_when_not_inserted(stored: i64, revealed: i64) -> RationaleOutcome {
    if stored > 0 {
        RationaleOutcome::AlreadyStored
    } else if revealed == 0 {
        RationaleOutcome::NotRevealed
    } else {
        RationaleOutcome::Mismatch
    }
}

#[derive(diesel::QueryableByName)]
struct RationaleRow {
    #[diesel(sql_type = BigInt)]
    round_no: i64,
    #[diesel(sql_type = Text)]
    agent_id: String,
    #[diesel(sql_type = Nullable<SmallInt>)]
    outcome: Option<i16>,
    #[diesel(sql_type = Nullable<Text>)]
    rationale_digest: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    revealed_at_ms: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    rationale: Option<String>,
}

/// Every revealed vote of a case with its published reason, oldest reveal first.
pub(crate) async fn list_rationales(
    db: &Db,
    case_id: &str,
) -> Result<Vec<JuryRationaleInfo>, SocialError> {
    use diesel::sql_query;
    let mut conn = db.connect().await?;
    let rows = sql_query(
        "SELECT s.round AS round_no, s.agent_id, s.outcome, s.rationale_digest, s.revealed_at_ms, r.rationale
           FROM jury_slots s
           LEFT JOIN jury_rationales r
                  ON r.case_id = s.case_id AND r.round = s.round AND r.agent_id = s.agent_id
          WHERE lower(s.case_id) = $1 AND s.status = 4
          ORDER BY s.round, s.revealed_at_ms, s.agent_id",
    )
    .bind::<Text, _>(case_id.to_lowercase())
    .load::<RationaleRow>(&mut conn)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| JuryRationaleInfo {
            round: row.round_no,
            agent_id: row.agent_id,
            outcome: row.outcome,
            rationale_digest: row.rationale_digest,
            revealed_at_ms: row.revealed_at_ms,
            rationale: row.rationale,
        })
        .collect())
}

#[cfg(test)]
mod rationale_tests {
    use super::*;

    #[test]
    fn a_rationale_that_was_not_inserted_says_why() {
        assert_eq!(
            rationale_outcome_when_not_inserted(1, 1),
            RationaleOutcome::AlreadyStored
        );
        assert_eq!(
            rationale_outcome_when_not_inserted(0, 0),
            RationaleOutcome::NotRevealed
        );
        assert_eq!(
            rationale_outcome_when_not_inserted(0, 1),
            RationaleOutcome::Mismatch
        );
    }

    #[test]
    fn the_outcome_names_serialize_in_snake_case_for_the_runner() {
        assert_eq!(
            serde_json::to_value(RationaleOutcome::NotRevealed).unwrap(),
            "not_revealed"
        );
        assert_eq!(
            serde_json::to_value(RationaleOutcome::AlreadyStored).unwrap(),
            "already_stored"
        );
    }
}
