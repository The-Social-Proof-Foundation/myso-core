// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use myso_indexer_alt_social_schema::schema::{
    jury_cases, jury_evidence, jury_payouts, jury_roster, jury_rounds, jury_slots,
    proposal_jury_referrals, proposal_jury_reviews, registry_jury_policies, task_disputes,
};
use serde::Serialize;

/// Arrays are stored as JSON text by the indexer; a malformed value reads as empty.
fn parse_u64_list(raw: &str) -> Vec<i64> {
    serde_json::from_str(raw).unwrap_or_default()
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = jury_cases)]
pub struct JuryCaseRow {
    pub case_id: String,
    pub subject_kind: i16,
    pub subject_id: String,
    pub stage: i16,
    pub menu_len: i16,
    pub approval_bps: i64,
    pub quorum: i64,
    pub round_sizes: String,
    pub appealable: bool,
    pub failure_path: i16,
    pub opened_by: String,
    pub fee_mist: i64,
    pub created_at_ms: i64,
    pub evidence_deadline_ms: i64,
    pub state: i16,
    pub current_round: i64,
    pub final_outcome: Option<i16>,
    pub appeals_used: i64,
    pub failed_appeal: bool,
    pub participants: Option<i64>,
    pub per_juror_fee_mist: Option<i64>,
    pub fail_reason: Option<i16>,
    pub finalized_at_ms: Option<i64>,
    pub cancelled_at_ms: Option<i64>,
    pub transaction_id: String,
}

/// `state`: 1 evidence, 2 awaiting draw, 3 selecting, 4 voting, 5 tallied, 6 final, 7 failed,
/// 8 cancelled. `subjectKind`: 1 task, 2 governance proposal.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JuryCaseInfo {
    pub case_id: String,
    pub subject_kind: i16,
    pub subject_id: String,
    pub stage: i16,
    pub menu_len: i16,
    pub approval_bps: i64,
    pub quorum: i64,
    pub round_sizes: Vec<i64>,
    pub appealable: bool,
    pub failure_path: i16,
    pub opened_by: String,
    pub fee_mist: i64,
    pub created_at_ms: i64,
    pub evidence_deadline_ms: i64,
    pub state: i16,
    pub current_round: i64,
    pub final_outcome: Option<i16>,
    pub appeals_used: i64,
    pub failed_appeal: bool,
    pub participants: Option<i64>,
    pub per_juror_fee_mist: Option<i64>,
    pub fail_reason: Option<i16>,
    pub finalized_at_ms: Option<i64>,
    pub cancelled_at_ms: Option<i64>,
    pub transaction_id: String,
}

impl From<JuryCaseRow> for JuryCaseInfo {
    fn from(r: JuryCaseRow) -> Self {
        Self {
            round_sizes: parse_u64_list(&r.round_sizes),
            case_id: r.case_id,
            subject_kind: r.subject_kind,
            subject_id: r.subject_id,
            stage: r.stage,
            menu_len: r.menu_len,
            approval_bps: r.approval_bps,
            quorum: r.quorum,
            appealable: r.appealable,
            failure_path: r.failure_path,
            opened_by: r.opened_by,
            fee_mist: r.fee_mist,
            created_at_ms: r.created_at_ms,
            evidence_deadline_ms: r.evidence_deadline_ms,
            state: r.state,
            current_round: r.current_round,
            final_outcome: r.final_outcome,
            appeals_used: r.appeals_used,
            failed_appeal: r.failed_appeal,
            participants: r.participants,
            per_juror_fee_mist: r.per_juror_fee_mist,
            fail_reason: r.fail_reason,
            finalized_at_ms: r.finalized_at_ms,
            cancelled_at_ms: r.cancelled_at_ms,
            transaction_id: r.transaction_id,
        }
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = jury_rounds)]
pub struct JuryRoundRow {
    pub case_id: String,
    pub round: i64,
    pub accept_deadline_ms: i64,
    pub commit_deadline_ms: Option<i64>,
    pub reveal_deadline_ms: Option<i64>,
    pub tally: Option<String>,
    pub abstain: Option<i64>,
    pub revealed: Option<i64>,
    pub decisive: Option<bool>,
    pub leading: Option<i16>,
    pub appeal_deadline_ms: Option<i64>,
}

/// Votes per menu option, known once the round is tallied.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JuryRoundInfo {
    pub round: i64,
    pub accept_deadline_ms: i64,
    pub commit_deadline_ms: Option<i64>,
    pub reveal_deadline_ms: Option<i64>,
    pub tally: Option<Vec<i64>>,
    pub abstain: Option<i64>,
    pub revealed: Option<i64>,
    pub decisive: Option<bool>,
    pub leading: Option<i16>,
    /// When the appeal window closes and anyone may finalize the case.
    pub appeal_deadline_ms: Option<i64>,
}

impl From<JuryRoundRow> for JuryRoundInfo {
    fn from(r: JuryRoundRow) -> Self {
        Self {
            round: r.round,
            accept_deadline_ms: r.accept_deadline_ms,
            commit_deadline_ms: r.commit_deadline_ms,
            reveal_deadline_ms: r.reveal_deadline_ms,
            tally: r.tally.as_deref().map(parse_u64_list),
            abstain: r.abstain,
            revealed: r.revealed,
            decisive: r.decisive,
            leading: r.leading,
            appeal_deadline_ms: r.appeal_deadline_ms,
        }
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = jury_slots)]
pub struct JurySlotRow {
    pub case_id: String,
    pub round: i64,
    pub agent_id: String,
    pub status: i16,
    pub accept_deadline_ms: i64,
    pub accepted_at_ms: Option<i64>,
    pub commitment: Option<String>,
    pub outcome: Option<i16>,
    pub rationale_digest: Option<String>,
    pub evidence_refs: Option<String>,
    pub bundle_digest: Option<String>,
    pub revealed_at_ms: Option<i64>,
}

/// `status`: 1 drafted, 2 accepted, 3 committed, 4 revealed, 5 replaced, 6 defaulted. A vote's
/// outcome and rationale digest are public only after the reveal.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JurySlotInfo {
    pub round: i64,
    pub agent_id: String,
    pub status: i16,
    pub accept_deadline_ms: i64,
    pub accepted_at_ms: Option<i64>,
    pub commitment: Option<String>,
    pub outcome: Option<i16>,
    pub rationale_digest: Option<String>,
    pub evidence_refs: Option<Vec<i64>>,
    pub bundle_digest: Option<String>,
    pub revealed_at_ms: Option<i64>,
}

impl From<JurySlotRow> for JurySlotInfo {
    fn from(r: JurySlotRow) -> Self {
        Self {
            round: r.round,
            agent_id: r.agent_id,
            status: r.status,
            accept_deadline_ms: r.accept_deadline_ms,
            accepted_at_ms: r.accepted_at_ms,
            commitment: r.commitment,
            outcome: r.outcome,
            rationale_digest: r.rationale_digest,
            evidence_refs: r.evidence_refs.as_deref().map(parse_u64_list),
            bundle_digest: r.bundle_digest,
            revealed_at_ms: r.revealed_at_ms,
        }
    }
}

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = jury_evidence)]
#[serde(rename_all = "camelCase")]
pub struct JuryEvidenceInfo {
    #[serde(rename = "index")]
    pub idx: i64,
    pub submitter: String,
    pub round: i64,
    pub kind: i16,
    pub digest: String,
    pub uri: String,
    pub submitted_at_ms: i64,
}

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = jury_payouts)]
#[serde(rename_all = "camelCase")]
pub struct JuryPayoutInfo {
    pub agent_id: String,
    pub recipient: String,
    pub amount_mist: i64,
}

/// `GET /jury/cases/:case_id`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JuryCaseDetailInfo {
    #[serde(flatten)]
    pub case: JuryCaseInfo,
    pub rounds: Vec<JuryRoundInfo>,
    pub jurors: Vec<JurySlotInfo>,
    pub evidence: Vec<JuryEvidenceInfo>,
    pub payouts: Vec<JuryPayoutInfo>,
}

/// `status`: 0 left, 1 active, 3 suspended (over once `suspendedUntilMs` is in the past).
#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = jury_roster)]
#[serde(rename_all = "camelCase")]
pub struct JurorRosterInfo {
    pub agent_id: String,
    pub owner: String,
    pub memory_account_id: String,
    pub organization_id: String,
    pub payout_address: String,
    pub model_family: i16,
    pub exempt: bool,
    pub status: i16,
    pub suspended_until_ms: Option<i64>,
    pub duty_score: Option<i64>,
    pub strikes: i64,
    pub joined_at_ms: i64,
    pub left_at_ms: Option<i64>,
}

/// One duty a juror holds. `dutyKind` is what the juror must do next (`accept`, `commit` or
/// `reveal`) and `deadlineMs` when it closes; both are null once the juror has nothing to do.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JurorAssignmentInfo {
    pub case_id: String,
    pub round: i64,
    pub agent_id: String,
    pub status: i16,
    pub duty_kind: Option<&'static str>,
    pub deadline_ms: Option<i64>,
    pub commitment: Option<String>,
    pub revealed: bool,
    pub case_state: i16,
    pub subject_kind: i16,
    pub subject_id: String,
}

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = task_disputes)]
#[serde(rename_all = "camelCase")]
pub struct TaskDisputeInfo {
    pub task_id: String,
    pub case_id: String,
    pub opened_by_role: i16,
    pub state: i16,
    pub fee_each_mist: i64,
    pub buyer_paid_mist: i64,
    pub seller_paid_mist: i64,
    pub opened_at_ms: i64,
    pub neutral_window_ends_ms: Option<i64>,
    pub agreement_proposer: i16,
    pub agreement_outcome: Option<i16>,
    pub resolved_outcome: Option<i16>,
    pub resolved_source: Option<i16>,
    pub resolved_at_ms: Option<i64>,
    pub fees_settled: bool,
    pub buyer_fee_refund_mist: Option<i64>,
    pub seller_fee_refund_mist: Option<i64>,
}

/// `GET /tasks/:task_id/dispute`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDisputeDetailInfo {
    #[serde(flatten)]
    pub dispute: TaskDisputeInfo,
    pub case: Option<JuryCaseDetailInfo>,
}

/// One human governance step the jury has been asked to take over. `stage`: 1 council, 2
/// community. `state`: 1 referring, 2 awaiting escalation, 3 with the jury, 4 decided. `decision`:
/// 0 forward (advance or approve), 1 reject; `failed` means the jury could not decide and the
/// step's default applied.
#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = proposal_jury_reviews)]
#[serde(rename_all = "camelCase")]
pub struct ProposalJuryReviewInfo {
    pub proposal_id: String,
    pub stage: i16,
    pub state: i16,
    pub trigger: i16,
    pub case_id: Option<String>,
    pub referral_count: i64,
    pub escalated_at_ms: Option<i64>,
    pub decided_at_ms: Option<i64>,
    pub decision: Option<i16>,
    pub failed: bool,
}

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = proposal_jury_referrals)]
#[serde(rename_all = "camelCase")]
pub struct ProposalJuryReferralInfo {
    pub delegate_address: String,
    pub reason: Option<String>,
    pub referred_at_ms: i64,
}

/// `GET /governance/proposals/:id/jury`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalJuryInfo {
    pub reviews: Vec<ProposalJuryReviewInfo>,
    pub referrals: Vec<ProposalJuryReferralInfo>,
}

/// `GET /governance/registries/:id/jury-policy`.
#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = registry_jury_policies)]
#[serde(rename_all = "camelCase")]
pub struct RegistryJuryPolicyInfo {
    pub registry_id: String,
    pub council_stage_enabled: bool,
    pub community_stage_enabled: bool,
    pub council_window_ms: i64,
    pub jury_size: i64,
    pub round2_size: i64,
    pub jury_fee_bps: i64,
    pub jury_fee_cap_mist: i64,
    pub min_pool_for_jury_mist: i64,
    pub quorum: i64,
    pub approval_bps_council: i64,
    pub approval_bps_community: i64,
    pub updated_at_ms: i64,
}

/// Deadline-driven cranks that nobody has run, by kind.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverdueCrank {
    pub kind: &'static str,
    pub count: i64,
    /// How long the oldest of them has been waiting past its deadline.
    pub oldest_overdue_ms: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterHealth {
    /// Jurors who can be drafted now.
    pub active: i64,
    /// Jurors serving a suspension that has not ended.
    pub suspended: i64,
    /// The floor below which task disputes cannot open (the jury config's `min_eligible_roster`).
    pub minimum: i64,
}

/// `GET /jury/health`: whether the jury is keeping up, in numbers an alert can watch.
/// `status` is `ok`, `warn` or `critical`; `reasons` says why in words.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JuryHealthInfo {
    pub status: &'static str,
    pub reasons: Vec<String>,
    /// Jury events queued for the automation engine and not yet delivered.
    pub unsent_events: i64,
    /// Age of the oldest of them; `None` when nothing is waiting.
    pub oldest_unsent_event_ms: Option<i64>,
    pub overdue_cranks: Vec<OverdueCrank>,
    pub roster: RosterHealth,
}

/// One revealed vote with the reason its juror gave. `rationale` is absent until the juror's runner
/// has published it; the digest is what the chain recorded, so the text can be checked against it.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JuryRationaleInfo {
    pub round: i64,
    pub agent_id: String,
    pub outcome: Option<i16>,
    pub rationale_digest: Option<String>,
    pub revealed_at_ms: Option<i64>,
    pub rationale: Option<String>,
}

/// `POST /internal/jury/rationales`: a juror's runner publishing the reason behind a revealed vote.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreRationaleRequest {
    pub case_id: String,
    pub round: i64,
    pub agent_id: String,
    pub rationale: String,
}

/// What became of a published rationale. The runner retries only `NotRevealed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RationaleOutcome {
    /// Saved: the text hashes to the digest the chain recorded.
    Stored,
    /// Already saved earlier.
    AlreadyStored,
    /// The seat has not revealed on chain yet; try again later.
    NotRevealed,
    /// The text does not hash to the revealed digest, so it is not this vote's rationale.
    Mismatch,
}

/// The policy plus the registry's own community quorum, which a page needs to tell whether an
/// ended vote will go to the jury (quorum missed) or be decided as usual.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryJuryPolicyView {
    #[serde(flatten)]
    pub policy: RegistryJuryPolicyInfo,
    pub community_quorum_votes: Option<i64>,
}

/// One thing a keeper can do right now (`GET /jury/cranks`). `kind` is the crank's event type:
/// `select_due`, `replace_due`, `round_finalize_due`, `case_finalize_due`, `execute_ready`,
/// `neutral_start_due`, `neutral_due` or `fees_due`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrankInfo {
    pub kind: &'static str,
    pub case_id: String,
    pub round: i64,
    pub subject_kind: i16,
    pub subject_id: String,
    pub due_ms: i64,
    /// Only for `fees_due`: whether the case must be closed before the fees are settled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_case_first: Option<bool>,
    /// Only for proposal cranks: the registry and (for platform registries) the platform, and the
    /// step (1 council, 2 community) the proposal is at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<i64>,
    /// Only for task cranks: the accounts and platform the action's transaction needs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_account_id: Option<String>,
    /// Only for roster cranks (`subjectId` is the juror's agent): the agent's own MemoryAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_account_id: Option<String>,
}

impl From<myso_indexer_alt_social_schema::models::Crank> for CrankInfo {
    fn from(crank: myso_indexer_alt_social_schema::models::Crank) -> Self {
        use myso_indexer_alt_social_schema::models::CrankKind;
        let governance = crank.kind.is_governance_crank();
        Self {
            kind: crank.kind.event_type(),
            close_case_first: (crank.kind == CrankKind::SettleFees).then_some(crank.extra == 1),
            registry_id: crank.refs.registry_id,
            platform_id: crank.refs.platform_id,
            buyer_account_id: crank.refs.buyer_account_id,
            seller_account_id: crank.refs.seller_account_id,
            memory_account_id: crank.refs.memory_account_id,
            stage: governance.then_some(crank.extra),
            case_id: crank.case_id,
            round: crank.round,
            subject_kind: crank.subject_kind,
            subject_id: crank.subject_id,
            due_ms: crank.due_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myso_indexer_alt_social_schema::models::{Crank, CrankKind, CrankRefs};

    fn crank(kind: CrankKind, extra: i64) -> Crank {
        Crank {
            kind,
            case_id: "0xc".into(),
            round: 1,
            subject_kind: 1,
            subject_id: "0xt".into(),
            due_ms: 9,
            extra,
            refs: CrankRefs::default(),
        }
    }

    #[test]
    fn a_proposal_crank_reports_its_registry_and_step() {
        let mut escalate = crank(CrankKind::EscalateProposal, 2);
        escalate.refs.registry_id = Some("0xreg".into());
        let info = CrankInfo::from(escalate);
        assert_eq!(info.registry_id.as_deref(), Some("0xreg"));
        assert_eq!(info.stage, Some(2));
        assert_eq!(CrankInfo::from(crank(CrankKind::ExecuteVerdict, 0)).stage, None);
    }

    #[test]
    fn a_task_crank_reports_the_accounts_its_transaction_needs() {
        let mut verdict = crank(CrankKind::ExecuteVerdict, 0);
        verdict.refs.buyer_account_id = Some("0xb".into());
        verdict.refs.seller_account_id = Some("0xs".into());
        let json = serde_json::to_value(CrankInfo::from(verdict)).unwrap();
        assert_eq!(json["buyerAccountId"], "0xb");
        assert_eq!(json["sellerAccountId"], "0xs");
        let bare = serde_json::to_value(CrankInfo::from(crank(CrankKind::SelectDue, 0))).unwrap();
        assert!(bare.get("buyerAccountId").is_none());
    }

    #[test]
    fn a_crank_is_reported_by_its_event_type() {
        let info = CrankInfo::from(crank(CrankKind::ExecuteVerdict, 0));
        assert_eq!(info.kind, "execute_ready");
        assert_eq!(info.close_case_first, None);
        let json = serde_json::to_value(&info).unwrap();
        assert!(json.get("closeCaseFirst").is_none());
        assert_eq!(json["caseId"], "0xc");
    }

    #[test]
    fn a_fee_crank_says_whether_the_case_must_be_closed_first() {
        assert_eq!(CrankInfo::from(crank(CrankKind::SettleFees, 1)).close_case_first, Some(true));
        assert_eq!(CrankInfo::from(crank(CrankKind::SettleFees, 0)).close_case_first, Some(false));
    }

    #[test]
    fn a_roster_entry_serializes_in_camel_case() {
        let json = serde_json::to_value(JurorAssignmentInfo {
            case_id: "0xc".into(),
            round: 0,
            agent_id: "0xa".into(),
            status: 1,
            duty_kind: Some("accept"),
            deadline_ms: Some(5),
            commitment: None,
            revealed: false,
            case_state: 3,
            subject_kind: 1,
            subject_id: "0xt".into(),
        })
        .unwrap();
        assert_eq!(json["dutyKind"], "accept");
        assert_eq!(json["deadlineMs"], 5);
    }
}
