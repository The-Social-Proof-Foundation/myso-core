// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Jury pipeline: indexes every `jury` event into `jury_cases`, `jury_rounds`, `jury_slots`,
//! `jury_evidence`, `jury_payouts` and `jury_roster`.
//!
//! Sequential, like the task market: a case's events apply in chain order, and every lifecycle
//! update is guarded by the state it expects so a replayed checkpoint cannot move a case backwards.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    duty_drafted_event, voting_started_event, JurorSeat, NewAutomationEvent, NewJuryCase,
    NewJuryEvidence, NewJuryPayout, NewJuryRosterEntry, NewJuryRound, NewJurySlot,
    NewProposalJuryReferral, NewProposalJuryReview, NewProposalRefund, NewProposalVoteRefund,
    NewRegistryJuryPolicy, CASE_STATE_CANCELLED, CASE_STATE_EVIDENCE, CASE_STATE_FAILED,
    CASE_STATE_FINAL, CASE_STATE_SELECTING, CASE_STATE_TALLIED, CASE_STATE_VOTING, DUTY_NO_REVEAL,
    REVIEW_DECISION_REJECT, REVIEW_STAGE_COMMUNITY, REVIEW_STAGE_COUNCIL, REVIEW_STATE_AWAITING,
    REVIEW_STATE_DECIDED, REVIEW_STATE_ESCALATED, REVIEW_STATE_REFERRING, ROSTER_ACTIVE,
    ROSTER_LEFT, ROSTER_SUSPENDED, SLOT_ACCEPTED, SLOT_COMMITTED, SLOT_DEFAULTED, SLOT_DRAFTED,
    SLOT_REPLACED, SLOT_REVEALED,
};
use myso_indexer_alt_social_schema::schema::{
    automation_event_outbox, jury_cases, jury_evidence, jury_payouts, jury_roster, jury_rounds,
    jury_slots, proposal_jury_referrals, proposal_jury_reviews, proposal_refunds,
    proposal_vote_refunds, registry_jury_policies,
};

use super::common;
use super::events;
use crate::metrics::SocialMetrics;

const JURY_MODULE: &str = "jury";
const GOVERNANCE_MODULE: &str = "governance";

/// The governance events that concern the jury. The proposal status they cause is carried by the
/// existing approve and reject events, except for a refund, whose status change the governance
/// pipeline also indexes (`ProposalRefundedJuryUnavailableEvent`).
pub const GOVERNANCE_JURY_EVENTS: &[&str] = &[
    "JuryPolicyUpdatedEvent",
    "DelegateReferredToJuryEvent",
    "ProposalEscalatedToJuryEvent",
    "CommunityVoteNeedsJuryEvent",
    "JuryDecisionAppliedEvent",
    "ProposalRejectedByJuryEvent",
    "JuryUnavailableEvent",
    "ProposalRefundedJuryUnavailableEvent",
    "VoteRefundClaimedEvent",
    "VoteRefundsSweptEvent",
];

/// Every event `jury.move` emits. A Move event missing here is caught by
/// `every_jury_event_is_handled`.
pub const JURY_EVENTS: &[&str] = &[
    "JuryConfigUpdated",
    "RosterJoined",
    "RosterLeft",
    "RosterSuspended",
    "DutyScoreChanged",
    "CaseOpened",
    "EvidenceSubmitted",
    "JuryDrawn",
    "AssignmentAccepted",
    "JurorReplaced",
    "VotingStarted",
    "VoteCommitted",
    "VoteRevealed",
    "RoundFinalized",
    "CaseAppealed",
    "CaseFinalized",
    "CaseFailed",
    "CaseCancelled",
    "JurorPaid",
];

#[derive(Debug, Clone)]
pub enum JuryRow {
    RosterJoined(NewJuryRosterEntry),
    RosterLeft {
        agent_id: String,
        reason: i16,
        left_at_ms: i64,
    },
    RosterSuspended {
        agent_id: String,
        until_ms: i64,
    },
    Duty {
        agent_id: String,
        case_id: String,
        score: i64,
        strikes: i64,
        reason: i16,
    },
    CaseOpened(NewJuryCase),
    Evidence(NewJuryEvidence),
    Drawn {
        round: NewJuryRound,
        slots: Vec<NewJurySlot>,
        replacement: bool,
        event_id: String,
    },
    Accepted {
        case_id: String,
        round: i64,
        agent_id: String,
        accepted_at_ms: i64,
    },
    Replaced {
        case_id: String,
        round: i64,
        agent_id: String,
    },
    VotingStarted {
        case_id: String,
        round: i64,
        commit_deadline_ms: i64,
        reveal_deadline_ms: i64,
        event_id: String,
    },
    Committed {
        case_id: String,
        round: i64,
        agent_id: String,
        commitment: String,
    },
    Revealed {
        case_id: String,
        round: i64,
        agent_id: String,
        outcome: i16,
        rationale_digest: String,
        evidence_refs: String,
        bundle_digest: String,
        revealed_at_ms: i64,
    },
    RoundFinalized {
        case_id: String,
        round: i64,
        tally: String,
        abstain: i64,
        revealed: i64,
        decisive: bool,
        leading: Option<i16>,
        appeal_deadline_ms: Option<i64>,
    },
    Appealed {
        case_id: String,
        round: i64,
    },
    Finalized {
        case_id: String,
        outcome: i16,
        round: i64,
        appeals_used: i64,
        failed_appeal: bool,
        participants: i64,
        per_juror_fee_mist: i64,
        finalized_at_ms: i64,
    },
    Failed {
        case_id: String,
        reason: i16,
        participants: i64,
        per_juror_fee_mist: i64,
        failed_at_ms: i64,
    },
    Cancelled {
        case_id: String,
        cancelled_at_ms: i64,
    },
    Paid(NewJuryPayout),
    PolicyUpdated(NewRegistryJuryPolicy),
    Referred {
        review: NewProposalJuryReview,
        referral: NewProposalJuryReferral,
    },
    NeedsJury(NewProposalJuryReview),
    Escalated(NewProposalJuryReview),
    StepDecided(NewProposalJuryReview),
    /// No jury could form. `review` is inserted if the step has no row yet; `state` is where the
    /// chain put the step back (referring at the council, awaiting at the community step).
    Unavailable {
        review: NewProposalJuryReview,
        at_ms: i64,
        deferred_since_ms: i64,
        formable_seats: i64,
        required_seats: i64,
    },
    Refunded(NewProposalRefund),
    VoteRefundClaimed(NewProposalVoteRefund),
    VoteRefundsSwept {
        proposal_id: String,
        amount_mist: i64,
        swept_at_ms: i64,
    },
}

impl FieldCount for JuryRow {
    const FIELD_COUNT: usize = 33;
}

fn str_field(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| v.as_str()).map(String::from)
}

fn i64_field(data: &serde_json::Value, key: &str) -> Option<i64> {
    let v = data.get(key)?;
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| i64::try_from(u).ok()))
}

fn bool_field(data: &serde_json::Value, key: &str) -> Option<bool> {
    data.get(key).and_then(|v| v.as_bool())
}

/// A JSON array stored as text (round sizes, tallies, evidence refs).
fn json_text_field(data: &serde_json::Value, key: &str) -> Option<String> {
    let v = data.get(key)?;
    v.is_array().then(|| v.to_string())
}

fn str_vec_field(data: &serde_json::Value, key: &str) -> Option<Vec<String>> {
    data.get(key)?
        .as_array()?
        .iter()
        .map(|v| v.as_str().map(String::from))
        .collect()
}

/// Pure translation of one parsed event into rows. `None` means the payload did not match;
/// an empty list means the event carries nothing the index keeps.
pub fn jury_rows(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<JuryRow>> {
    let transaction_id = event_id.split(':').next().unwrap_or(event_id).to_string();
    let event_id = event_id.to_string();
    let row = match event_name {
        "JuryConfigUpdated" => return Some(vec![]),
        "RosterJoined" => {
            let joined_at_ms = i64_field(data, "joined_at_ms")?;
            JuryRow::RosterJoined(NewJuryRosterEntry {
                agent_id: str_field(data, "agent_id")?,
                owner: str_field(data, "owner")?,
                memory_account_id: str_field(data, "memory_account_id")?,
                organization_id: str_field(data, "organization_id")?,
                payout_address: str_field(data, "payout_address")?,
                model_family: i64_field(data, "model_family")? as i16,
                exempt: bool_field(data, "exempt")?,
                status: ROSTER_ACTIVE,
                joined_at_ms,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(joined_at_ms),
            })
        }
        "RosterLeft" => JuryRow::RosterLeft {
            agent_id: str_field(data, "agent_id")?,
            reason: i64_field(data, "reason")? as i16,
            left_at_ms: i64_field(data, "left_at_ms")?,
        },
        "RosterSuspended" => JuryRow::RosterSuspended {
            agent_id: str_field(data, "agent_id")?,
            until_ms: i64_field(data, "until_ms")?,
        },
        "DutyScoreChanged" => JuryRow::Duty {
            agent_id: str_field(data, "agent_id")?,
            case_id: str_field(data, "case_id")?,
            score: i64_field(data, "score")?,
            strikes: i64_field(data, "strikes")?,
            reason: i64_field(data, "reason")? as i16,
        },
        "CaseOpened" => {
            let created_at_ms = i64_field(data, "created_at_ms")?;
            JuryRow::CaseOpened(NewJuryCase {
                case_id: str_field(data, "case_id")?,
                subject_kind: i64_field(data, "subject_kind")? as i16,
                subject_id: str_field(data, "subject_id")?,
                stage: i64_field(data, "stage")? as i16,
                menu_len: i64_field(data, "menu_len")? as i16,
                approval_bps: i64_field(data, "approval_bps")?,
                quorum: i64_field(data, "quorum")?,
                round_sizes: json_text_field(data, "round_sizes")?,
                appealable: bool_field(data, "appealable")?,
                failure_path: i64_field(data, "failure_path")? as i16,
                opened_by: str_field(data, "opened_by")?,
                fee_mist: i64_field(data, "fee_mist")?,
                created_at_ms,
                evidence_deadline_ms: i64_field(data, "evidence_deadline_ms")?,
                state: CASE_STATE_EVIDENCE,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(created_at_ms),
            })
        }
        "EvidenceSubmitted" => JuryRow::Evidence(NewJuryEvidence {
            case_id: str_field(data, "case_id")?,
            idx: i64_field(data, "index")?,
            submitter: str_field(data, "submitter")?,
            round: i64_field(data, "round")?,
            kind: i64_field(data, "kind")? as i16,
            digest: str_field(data, "digest")?,
            uri: str_field(data, "uri")?,
            submitted_at_ms: i64_field(data, "submitted_at_ms")?,
        }),
        "JuryDrawn" => {
            let case_id = str_field(data, "case_id")?;
            let round = i64_field(data, "round")?;
            let accept_deadline_ms = i64_field(data, "accept_deadline_ms")?;
            let slots = str_vec_field(data, "jurors")?
                .into_iter()
                .map(|agent_id| NewJurySlot {
                    case_id: case_id.clone(),
                    round,
                    agent_id,
                    status: SLOT_DRAFTED,
                    accept_deadline_ms,
                })
                .collect();
            JuryRow::Drawn {
                round: NewJuryRound {
                    case_id,
                    round,
                    accept_deadline_ms,
                },
                slots,
                replacement: bool_field(data, "replacement")?,
                event_id,
            }
        }
        "AssignmentAccepted" => JuryRow::Accepted {
            case_id: str_field(data, "case_id")?,
            round: i64_field(data, "round")?,
            agent_id: str_field(data, "agent_id")?,
            accepted_at_ms: i64_field(data, "accepted_at_ms")?,
        },
        "JurorReplaced" => JuryRow::Replaced {
            case_id: str_field(data, "case_id")?,
            round: i64_field(data, "round")?,
            agent_id: str_field(data, "agent_id")?,
        },
        "VotingStarted" => JuryRow::VotingStarted {
            case_id: str_field(data, "case_id")?,
            round: i64_field(data, "round")?,
            commit_deadline_ms: i64_field(data, "commit_deadline_ms")?,
            reveal_deadline_ms: i64_field(data, "reveal_deadline_ms")?,
            event_id,
        },
        "VoteCommitted" => JuryRow::Committed {
            case_id: str_field(data, "case_id")?,
            round: i64_field(data, "round")?,
            agent_id: str_field(data, "agent_id")?,
            commitment: str_field(data, "commitment")?,
        },
        "VoteRevealed" => JuryRow::Revealed {
            case_id: str_field(data, "case_id")?,
            round: i64_field(data, "round")?,
            agent_id: str_field(data, "agent_id")?,
            outcome: i64_field(data, "outcome")? as i16,
            rationale_digest: str_field(data, "rationale_digest")?,
            evidence_refs: json_text_field(data, "evidence_refs")?,
            bundle_digest: str_field(data, "bundle_digest")?,
            revealed_at_ms: i64_field(data, "revealed_at_ms")?,
        },
        "RoundFinalized" => JuryRow::RoundFinalized {
            case_id: str_field(data, "case_id")?,
            round: i64_field(data, "round")?,
            tally: json_text_field(data, "tally")?,
            abstain: i64_field(data, "abstain")?,
            revealed: i64_field(data, "revealed")?,
            decisive: bool_field(data, "decisive")?,
            leading: data
                .get("leading")
                .filter(|v| !v.is_null())
                .and_then(|v| v.as_i64())
                .map(|v| v as i16),
            appeal_deadline_ms: i64_field(data, "appeal_deadline_ms").filter(|ms| *ms > 0),
        },
        "CaseAppealed" => JuryRow::Appealed {
            case_id: str_field(data, "case_id")?,
            round: i64_field(data, "round")?,
        },
        "CaseFinalized" => JuryRow::Finalized {
            case_id: str_field(data, "case_id")?,
            outcome: i64_field(data, "outcome")? as i16,
            round: i64_field(data, "round")?,
            appeals_used: i64_field(data, "appeals_used")?,
            failed_appeal: bool_field(data, "failed_appeal")?,
            participants: i64_field(data, "participants")?,
            per_juror_fee_mist: i64_field(data, "per_juror_fee")?,
            finalized_at_ms: i64_field(data, "finalized_at_ms")?,
        },
        "CaseFailed" => JuryRow::Failed {
            case_id: str_field(data, "case_id")?,
            reason: i64_field(data, "reason")? as i16,
            participants: i64_field(data, "participants")?,
            per_juror_fee_mist: i64_field(data, "per_juror_fee")?,
            failed_at_ms: i64_field(data, "failed_at_ms")?,
        },
        "CaseCancelled" => JuryRow::Cancelled {
            case_id: str_field(data, "case_id")?,
            cancelled_at_ms: i64_field(data, "cancelled_at_ms")?,
        },
        "JuryPolicyUpdatedEvent" => {
            let updated_at_ms = i64_field(data, "updated_at_ms")?;
            JuryRow::PolicyUpdated(NewRegistryJuryPolicy {
                registry_id: str_field(data, "registry_id")?,
                council_stage_enabled: bool_field(data, "council_stage_enabled")?,
                community_stage_enabled: bool_field(data, "community_stage_enabled")?,
                council_window_ms: i64_field(data, "council_window_ms")?,
                jury_size: i64_field(data, "jury_size")?,
                round2_size: i64_field(data, "round2_size")?,
                jury_fee_bps: i64_field(data, "jury_fee_bps")?,
                jury_fee_cap_mist: i64_field(data, "jury_fee_cap_mist")?,
                min_pool_for_jury_mist: i64_field(data, "min_pool_for_jury_mist")?,
                quorum: i64_field(data, "quorum")?,
                approval_bps_council: i64_field(data, "approval_bps_council")?,
                approval_bps_community: i64_field(data, "approval_bps_community")?,
                updated_at_ms,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(updated_at_ms),
            })
        }
        "DelegateReferredToJuryEvent" => {
            let referred_at_ms = i64_field(data, "referred_at_ms")?;
            let proposal_id = str_field(data, "proposal_id")?;
            JuryRow::Referred {
                review: NewProposalJuryReview {
                    proposal_id: proposal_id.clone(),
                    stage: REVIEW_STAGE_COUNCIL,
                    state: REVIEW_STATE_REFERRING,
                    trigger: if bool_field(data, "majority_reached")? {
                        1
                    } else {
                        0
                    },
                    case_id: None,
                    referral_count: i64_field(data, "referral_count")?,
                    escalated_at_ms: None,
                    decided_at_ms: None,
                    decision: None,
                    failed: false,
                    event_id: event_id.clone(),
                    transaction_id,
                    time: common::chain_time_from_ms(referred_at_ms),
                },
                referral: NewProposalJuryReferral {
                    proposal_id,
                    delegate_address: str_field(data, "delegate_address")?,
                    reason: data
                        .get("reason")
                        .filter(|v| !v.is_null())
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    referred_at_ms,
                    event_id,
                },
            }
        }
        "CommunityVoteNeedsJuryEvent" => {
            let time_ms = i64_field(data, "time_ms")?;
            JuryRow::NeedsJury(NewProposalJuryReview {
                proposal_id: str_field(data, "proposal_id")?,
                stage: REVIEW_STAGE_COMMUNITY,
                state: REVIEW_STATE_AWAITING,
                trigger: i64_field(data, "trigger")? as i16,
                case_id: None,
                referral_count: 0,
                escalated_at_ms: None,
                decided_at_ms: None,
                decision: None,
                failed: false,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(time_ms),
            })
        }
        "ProposalEscalatedToJuryEvent" => {
            let escalated_at_ms = i64_field(data, "escalated_at_ms")?;
            JuryRow::Escalated(NewProposalJuryReview {
                proposal_id: str_field(data, "proposal_id")?,
                stage: i64_field(data, "stage")? as i16,
                state: REVIEW_STATE_ESCALATED,
                trigger: i64_field(data, "trigger")? as i16,
                case_id: Some(str_field(data, "case_id")?),
                referral_count: 0,
                escalated_at_ms: Some(escalated_at_ms),
                decided_at_ms: None,
                decision: None,
                failed: false,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(escalated_at_ms),
            })
        }
        "JuryDecisionAppliedEvent" => {
            let applied_at_ms = i64_field(data, "applied_at_ms")?;
            JuryRow::StepDecided(NewProposalJuryReview {
                proposal_id: str_field(data, "proposal_id")?,
                stage: i64_field(data, "stage")? as i16,
                state: REVIEW_STATE_DECIDED,
                trigger: 0,
                case_id: data
                    .get("case_id")
                    .filter(|v| !v.is_null())
                    .and_then(|v| v.as_str())
                    .map(String::from),
                referral_count: 0,
                escalated_at_ms: None,
                decided_at_ms: Some(applied_at_ms),
                decision: Some(i64_field(data, "decision")? as i16),
                failed: bool_field(data, "failed")?,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(applied_at_ms),
            })
        }
        // The proposal's status already moved through the existing rejection events.
        "ProposalRejectedByJuryEvent" => return Some(vec![]),
        "JuryUnavailableEvent" => {
            let at_ms = i64_field(data, "at_ms")?;
            let stage = i64_field(data, "stage")? as i16;
            JuryRow::Unavailable {
                review: NewProposalJuryReview {
                    proposal_id: str_field(data, "proposal_id")?,
                    stage,
                    state: if stage == REVIEW_STAGE_COUNCIL {
                        REVIEW_STATE_REFERRING
                    } else {
                        REVIEW_STATE_AWAITING
                    },
                    trigger: 0,
                    case_id: data
                        .get("case_id")
                        .filter(|v| !v.is_null())
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    referral_count: 0,
                    escalated_at_ms: None,
                    decided_at_ms: None,
                    decision: None,
                    failed: false,
                    event_id,
                    transaction_id,
                    time: common::chain_time_from_ms(at_ms),
                },
                at_ms,
                deferred_since_ms: i64_field(data, "deferred_since_ms")?,
                formable_seats: i64_field(data, "formable_seats")?,
                required_seats: i64_field(data, "required_seats")?,
            }
        }
        "ProposalRefundedJuryUnavailableEvent" => {
            let refunded_at_ms = i64_field(data, "refunded_at_ms")?;
            let voter_refund_pool_mist = i64_field(data, "voter_refund_pool_mist")?;
            JuryRow::Refunded(NewProposalRefund {
                proposal_id: str_field(data, "proposal_id")?,
                submitter: str_field(data, "submitter")?,
                submitter_refund_mist: i64_field(data, "submitter_refund_mist")?,
                voter_refund_pool_mist,
                outstanding_mist: voter_refund_pool_mist,
                claim_deadline_ms: i64_field(data, "claim_deadline_ms")?,
                refunded_at_ms,
                event_id,
                transaction_id,
                time: common::chain_time_from_ms(refunded_at_ms),
            })
        }
        "VoteRefundClaimedEvent" => JuryRow::VoteRefundClaimed(NewProposalVoteRefund {
            proposal_id: str_field(data, "proposal_id")?,
            voter: str_field(data, "voter")?,
            amount_mist: i64_field(data, "amount_mist")?,
            claimed_at_ms: i64_field(data, "claimed_at_ms")?,
            event_id,
        }),
        "VoteRefundsSweptEvent" => JuryRow::VoteRefundsSwept {
            proposal_id: str_field(data, "proposal_id")?,
            amount_mist: i64_field(data, "amount_mist")?,
            swept_at_ms: i64_field(data, "swept_at_ms")?,
        },
        "JurorPaid" => JuryRow::Paid(NewJuryPayout {
            case_id: str_field(data, "case_id")?,
            agent_id: str_field(data, "agent_id")?,
            recipient: str_field(data, "recipient")?,
            amount_mist: i64_field(data, "amount_mist")?,
        }),
        _ => return None,
    };
    Some(vec![row])
}

/// Queue automation events in the transaction that applied the chain event. A conflict on the
/// deduplication key means a replayed checkpoint already queued it.
async fn enqueue_events<'a>(
    conn: &mut Connection<'a>,
    events: &[NewAutomationEvent],
) -> Result<usize> {
    if events.is_empty() {
        return Ok(0);
    }
    Ok(diesel::insert_into(automation_event_outbox::table)
        .values(events)
        .on_conflict(automation_event_outbox::dedup_key)
        .do_nothing()
        .execute(conn)
        .await?)
}

/// The seats `agent_ids` hold in a round, each with the account that acts for it. A juror who has
/// left the roster since being drawn has no account to address, so it gets no event.
async fn seats_of<'a>(
    conn: &mut Connection<'a>,
    case_id: &str,
    round: i64,
    agent_ids: Vec<String>,
) -> Result<Vec<JurorSeat>> {
    if agent_ids.is_empty() {
        return Ok(Vec::new());
    }
    let Some((subject_kind, subject_id)) = jury_cases::table
        .find(case_id)
        .select((jury_cases::subject_kind, jury_cases::subject_id))
        .first::<(i16, String)>(conn)
        .await
        .optional()?
    else {
        return Ok(Vec::new());
    };
    let accounts: Vec<(String, String)> = jury_roster::table
        .filter(jury_roster::agent_id.eq_any(&agent_ids))
        .select((jury_roster::agent_id, jury_roster::memory_account_id))
        .load(conn)
        .await?;
    Ok(accounts
        .into_iter()
        .map(|(agent_id, account_id)| JurorSeat {
            case_id: case_id.to_string(),
            round,
            agent_id,
            account_id,
            subject_kind,
            subject_id: subject_id.clone(),
        })
        .collect())
}

pub struct JuryHandler;

#[async_trait]
impl Processor for JuryHandler {
    const NAME: &'static str = "jury";

    type Value = JuryRow;

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
                let module = ev.type_.module.as_str();
                let event_name = ev.type_.name.as_str();
                let wanted = match module {
                    JURY_MODULE => JURY_EVENTS.contains(&event_name),
                    GOVERNANCE_MODULE => GOVERNANCE_JURY_EVENTS.contains(&event_name),
                    _ => false,
                };
                if !wanted {
                    continue;
                }
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let data = match events::parse_event_contents(module, event_name, &ev.contents) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::warn!(
                            tx_digest = %tx_digest,
                            event_name,
                            error = %e,
                            "jury: event parse failed; skipping"
                        );
                        SocialMetrics::record_event_bcs_parse_failed(module, event_name);
                        continue;
                    }
                };
                match jury_rows(event_name, &data, &event_id) {
                    Some(rows) => values.extend(rows),
                    None => tracing::warn!(
                        tx_digest = %tx_digest,
                        event_name,
                        "jury: event JSON did not match its row builder"
                    ),
                }
            }
        }
        Ok(values)
    }
}

#[async_trait]
impl Handler for JuryHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                JuryRow::RosterJoined(entry) => {
                    // A juror can leave and join again; the latest join wins and clears the exit.
                    total += diesel::insert_into(jury_roster::table)
                        .values(entry)
                        .on_conflict(jury_roster::agent_id)
                        .do_update()
                        .set((
                            jury_roster::owner.eq(&entry.owner),
                            jury_roster::memory_account_id.eq(&entry.memory_account_id),
                            jury_roster::organization_id.eq(&entry.organization_id),
                            jury_roster::payout_address.eq(&entry.payout_address),
                            jury_roster::model_family.eq(entry.model_family),
                            jury_roster::exempt.eq(entry.exempt),
                            jury_roster::status.eq(ROSTER_ACTIVE),
                            jury_roster::joined_at_ms.eq(entry.joined_at_ms),
                            jury_roster::left_at_ms.eq(None::<i64>),
                            jury_roster::left_reason.eq(None::<i16>),
                            jury_roster::event_id.eq(&entry.event_id),
                            jury_roster::transaction_id.eq(&entry.transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                JuryRow::RosterLeft {
                    agent_id,
                    reason,
                    left_at_ms,
                } => {
                    total += diesel::update(jury_roster::table.find(agent_id))
                        .set((
                            jury_roster::status.eq(ROSTER_LEFT),
                            jury_roster::left_at_ms.eq(*left_at_ms),
                            jury_roster::left_reason.eq(*reason),
                        ))
                        .execute(conn)
                        .await?;
                }
                JuryRow::RosterSuspended { agent_id, until_ms } => {
                    total += diesel::update(jury_roster::table.find(agent_id))
                        .set((
                            jury_roster::status.eq(ROSTER_SUSPENDED),
                            jury_roster::suspended_until_ms.eq(*until_ms),
                        ))
                        .execute(conn)
                        .await?;
                }
                JuryRow::Duty {
                    agent_id,
                    case_id,
                    score,
                    strikes,
                    reason,
                } => {
                    total += diesel::update(jury_roster::table.find(agent_id))
                        .set((
                            jury_roster::duty_score.eq(*score),
                            jury_roster::strikes.eq(*strikes),
                        ))
                        .execute(conn)
                        .await?;
                    if *reason == DUTY_NO_REVEAL {
                        // The slot has no event of its own when a juror defaults on the reveal.
                        total += diesel::update(
                            jury_slots::table
                                .filter(jury_slots::case_id.eq(case_id))
                                .filter(jury_slots::agent_id.eq(agent_id))
                                .filter(
                                    jury_slots::status
                                        .eq(SLOT_ACCEPTED)
                                        .or(jury_slots::status.eq(SLOT_COMMITTED)),
                                ),
                        )
                        .set(jury_slots::status.eq(SLOT_DEFAULTED))
                        .execute(conn)
                        .await?;
                    }
                }
                JuryRow::CaseOpened(case) => {
                    total += diesel::insert_into(jury_cases::table)
                        .values(case)
                        .on_conflict(jury_cases::case_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                JuryRow::Evidence(item) => {
                    total += diesel::insert_into(jury_evidence::table)
                        .values(item)
                        .on_conflict((jury_evidence::case_id, jury_evidence::idx))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                JuryRow::Drawn {
                    round,
                    slots,
                    replacement,
                    event_id,
                } => {
                    // A replacement draw keeps the round and only extends the accept window.
                    total += diesel::insert_into(jury_rounds::table)
                        .values(round)
                        .on_conflict((jury_rounds::case_id, jury_rounds::round))
                        .do_update()
                        .set(jury_rounds::accept_deadline_ms.eq(round.accept_deadline_ms))
                        .execute(conn)
                        .await?;
                    if !slots.is_empty() {
                        total += diesel::insert_into(jury_slots::table)
                            .values(slots)
                            .on_conflict((
                                jury_slots::case_id,
                                jury_slots::round,
                                jury_slots::agent_id,
                            ))
                            .do_nothing()
                            .execute(conn)
                            .await?;
                    }
                    if !*replacement {
                        total += diesel::update(
                            jury_cases::table
                                .find(&round.case_id)
                                .filter(jury_cases::state.lt(CASE_STATE_FINAL)),
                        )
                        .set((
                            jury_cases::state.eq(CASE_STATE_SELECTING),
                            jury_cases::current_round.eq(round.round),
                        ))
                        .execute(conn)
                        .await?;
                    }
                    let seats = seats_of(
                        conn,
                        &round.case_id,
                        round.round,
                        slots.iter().map(|s| s.agent_id.clone()).collect(),
                    )
                    .await?;
                    let now_ms = chrono::Utc::now().timestamp_millis();
                    let events: Vec<_> = seats
                        .iter()
                        .map(|seat| {
                            duty_drafted_event(seat, round.accept_deadline_ms, now_ms, event_id)
                        })
                        .collect();
                    total += enqueue_events(conn, &events).await?;
                }
                JuryRow::Accepted {
                    case_id,
                    round,
                    agent_id,
                    accepted_at_ms,
                } => {
                    total += diesel::update(
                        jury_slots::table
                            .find((case_id, *round, agent_id))
                            .filter(jury_slots::status.eq(SLOT_DRAFTED)),
                    )
                    .set((
                        jury_slots::status.eq(SLOT_ACCEPTED),
                        jury_slots::accepted_at_ms.eq(*accepted_at_ms),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::Replaced {
                    case_id,
                    round,
                    agent_id,
                } => {
                    total += diesel::update(
                        jury_slots::table
                            .find((case_id, *round, agent_id))
                            .filter(jury_slots::status.eq(SLOT_DRAFTED)),
                    )
                    .set(jury_slots::status.eq(SLOT_REPLACED))
                    .execute(conn)
                    .await?;
                }
                JuryRow::VotingStarted {
                    case_id,
                    round,
                    commit_deadline_ms,
                    reveal_deadline_ms,
                    event_id,
                } => {
                    total += diesel::update(jury_rounds::table.find((case_id, *round)))
                        .set((
                            jury_rounds::commit_deadline_ms.eq(*commit_deadline_ms),
                            jury_rounds::reveal_deadline_ms.eq(*reveal_deadline_ms),
                        ))
                        .execute(conn)
                        .await?;
                    total += diesel::update(
                        jury_cases::table
                            .find(case_id)
                            .filter(jury_cases::state.lt(CASE_STATE_FINAL)),
                    )
                    .set((
                        jury_cases::state.eq(CASE_STATE_VOTING),
                        jury_cases::current_round.eq(*round),
                    ))
                    .execute(conn)
                    .await?;
                    // Only jurors who accepted vote; the rest were replaced or defaulted.
                    let accepted: Vec<String> = jury_slots::table
                        .filter(jury_slots::case_id.eq(case_id))
                        .filter(jury_slots::round.eq(*round))
                        .filter(jury_slots::status.eq(SLOT_ACCEPTED))
                        .select(jury_slots::agent_id)
                        .load(conn)
                        .await?;
                    let seats = seats_of(conn, case_id, *round, accepted).await?;
                    let now_ms = chrono::Utc::now().timestamp_millis();
                    let events: Vec<_> = seats
                        .iter()
                        .map(|seat| {
                            voting_started_event(
                                seat,
                                *commit_deadline_ms,
                                *reveal_deadline_ms,
                                now_ms,
                                event_id,
                            )
                        })
                        .collect();
                    total += enqueue_events(conn, &events).await?;
                }
                JuryRow::Committed {
                    case_id,
                    round,
                    agent_id,
                    commitment,
                } => {
                    total += diesel::update(
                        jury_slots::table
                            .find((case_id, *round, agent_id))
                            .filter(jury_slots::status.lt(SLOT_COMMITTED)),
                    )
                    .set((
                        jury_slots::status.eq(SLOT_COMMITTED),
                        jury_slots::commitment.eq(commitment),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::Revealed {
                    case_id,
                    round,
                    agent_id,
                    outcome,
                    rationale_digest,
                    evidence_refs,
                    bundle_digest,
                    revealed_at_ms,
                } => {
                    total += diesel::update(
                        jury_slots::table
                            .find((case_id, *round, agent_id))
                            .filter(jury_slots::status.eq(SLOT_COMMITTED)),
                    )
                    .set((
                        jury_slots::status.eq(SLOT_REVEALED),
                        jury_slots::outcome.eq(*outcome),
                        jury_slots::rationale_digest.eq(rationale_digest),
                        jury_slots::evidence_refs.eq(evidence_refs),
                        jury_slots::bundle_digest.eq(bundle_digest),
                        jury_slots::revealed_at_ms.eq(*revealed_at_ms),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::RoundFinalized {
                    case_id,
                    round,
                    tally,
                    abstain,
                    revealed,
                    decisive,
                    leading,
                    appeal_deadline_ms,
                } => {
                    total += diesel::update(jury_rounds::table.find((case_id, *round)))
                        .set((
                            jury_rounds::tally.eq(tally),
                            jury_rounds::abstain.eq(*abstain),
                            jury_rounds::revealed.eq(*revealed),
                            jury_rounds::decisive.eq(*decisive),
                            jury_rounds::leading.eq(*leading),
                            jury_rounds::appeal_deadline_ms.eq(*appeal_deadline_ms),
                        ))
                        .execute(conn)
                        .await?;
                    total += diesel::update(
                        jury_cases::table
                            .find(case_id)
                            .filter(jury_cases::state.lt(CASE_STATE_FINAL)),
                    )
                    .set(jury_cases::state.eq(CASE_STATE_TALLIED))
                    .execute(conn)
                    .await?;
                }
                JuryRow::Appealed { case_id, round } => {
                    total += diesel::update(
                        jury_cases::table
                            .find(case_id)
                            .filter(jury_cases::state.lt(CASE_STATE_FINAL)),
                    )
                    .set((
                        jury_cases::state.eq(CASE_STATE_SELECTING),
                        jury_cases::current_round.eq(*round),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::Finalized {
                    case_id,
                    outcome,
                    round,
                    appeals_used,
                    failed_appeal,
                    participants,
                    per_juror_fee_mist,
                    finalized_at_ms,
                } => {
                    total += diesel::update(
                        jury_cases::table
                            .find(case_id)
                            .filter(jury_cases::state.lt(CASE_STATE_FINAL)),
                    )
                    .set((
                        jury_cases::state.eq(CASE_STATE_FINAL),
                        jury_cases::final_outcome.eq(*outcome),
                        jury_cases::current_round.eq(*round),
                        jury_cases::appeals_used.eq(*appeals_used),
                        jury_cases::failed_appeal.eq(*failed_appeal),
                        jury_cases::participants.eq(*participants),
                        jury_cases::per_juror_fee_mist.eq(*per_juror_fee_mist),
                        jury_cases::finalized_at_ms.eq(*finalized_at_ms),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::Failed {
                    case_id,
                    reason,
                    participants,
                    per_juror_fee_mist,
                    failed_at_ms,
                } => {
                    total += diesel::update(
                        jury_cases::table
                            .find(case_id)
                            .filter(jury_cases::state.lt(CASE_STATE_FINAL)),
                    )
                    .set((
                        jury_cases::state.eq(CASE_STATE_FAILED),
                        jury_cases::fail_reason.eq(*reason),
                        jury_cases::participants.eq(*participants),
                        jury_cases::per_juror_fee_mist.eq(*per_juror_fee_mist),
                        jury_cases::finalized_at_ms.eq(*failed_at_ms),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::Cancelled {
                    case_id,
                    cancelled_at_ms,
                } => {
                    total += diesel::update(
                        jury_cases::table
                            .find(case_id)
                            .filter(jury_cases::state.lt(CASE_STATE_FINAL)),
                    )
                    .set((
                        jury_cases::state.eq(CASE_STATE_CANCELLED),
                        jury_cases::cancelled_at_ms.eq(*cancelled_at_ms),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::PolicyUpdated(policy) => {
                    total += diesel::insert_into(registry_jury_policies::table)
                        .values(policy)
                        .on_conflict(registry_jury_policies::registry_id)
                        .do_update()
                        .set((
                            registry_jury_policies::council_stage_enabled
                                .eq(policy.council_stage_enabled),
                            registry_jury_policies::community_stage_enabled
                                .eq(policy.community_stage_enabled),
                            registry_jury_policies::council_window_ms.eq(policy.council_window_ms),
                            registry_jury_policies::jury_size.eq(policy.jury_size),
                            registry_jury_policies::round2_size.eq(policy.round2_size),
                            registry_jury_policies::jury_fee_bps.eq(policy.jury_fee_bps),
                            registry_jury_policies::jury_fee_cap_mist.eq(policy.jury_fee_cap_mist),
                            registry_jury_policies::min_pool_for_jury_mist
                                .eq(policy.min_pool_for_jury_mist),
                            registry_jury_policies::quorum.eq(policy.quorum),
                            registry_jury_policies::approval_bps_council
                                .eq(policy.approval_bps_council),
                            registry_jury_policies::approval_bps_community
                                .eq(policy.approval_bps_community),
                            registry_jury_policies::updated_at_ms.eq(policy.updated_at_ms),
                        ))
                        .execute(conn)
                        .await?;
                }
                JuryRow::Referred { review, referral } => {
                    // A referral only ever raises the count, and never moves a step the jury has taken.
                    total += diesel::insert_into(proposal_jury_reviews::table)
                        .values(review)
                        .on_conflict((
                            proposal_jury_reviews::proposal_id,
                            proposal_jury_reviews::stage,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    total += diesel::update(
                        proposal_jury_reviews::table
                            .filter(proposal_jury_reviews::proposal_id.eq(&review.proposal_id))
                            .filter(proposal_jury_reviews::stage.eq(review.stage))
                            .filter(proposal_jury_reviews::state.eq(REVIEW_STATE_REFERRING)),
                    )
                    .set((
                        proposal_jury_reviews::referral_count.eq(review.referral_count),
                        proposal_jury_reviews::trigger.eq(review.trigger),
                    ))
                    .execute(conn)
                    .await?;
                    total += diesel::insert_into(proposal_jury_referrals::table)
                        .values(referral)
                        .on_conflict((
                            proposal_jury_referrals::proposal_id,
                            proposal_jury_referrals::delegate_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                JuryRow::NeedsJury(review) => {
                    total += diesel::insert_into(proposal_jury_reviews::table)
                        .values(review)
                        .on_conflict((
                            proposal_jury_reviews::proposal_id,
                            proposal_jury_reviews::stage,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                JuryRow::Escalated(review) => {
                    total += diesel::insert_into(proposal_jury_reviews::table)
                        .values(review)
                        .on_conflict((
                            proposal_jury_reviews::proposal_id,
                            proposal_jury_reviews::stage,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    total += diesel::update(
                        proposal_jury_reviews::table
                            .filter(proposal_jury_reviews::proposal_id.eq(&review.proposal_id))
                            .filter(proposal_jury_reviews::stage.eq(review.stage))
                            .filter(proposal_jury_reviews::state.lt(REVIEW_STATE_ESCALATED)),
                    )
                    .set((
                        proposal_jury_reviews::state.eq(REVIEW_STATE_ESCALATED),
                        proposal_jury_reviews::trigger.eq(review.trigger),
                        proposal_jury_reviews::case_id.eq(&review.case_id),
                        proposal_jury_reviews::escalated_at_ms.eq(review.escalated_at_ms),
                        proposal_jury_reviews::event_id.eq(&review.event_id),
                        proposal_jury_reviews::transaction_id.eq(&review.transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::StepDecided(review) => {
                    total += diesel::insert_into(proposal_jury_reviews::table)
                        .values(review)
                        .on_conflict((
                            proposal_jury_reviews::proposal_id,
                            proposal_jury_reviews::stage,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    total += diesel::update(
                        proposal_jury_reviews::table
                            .filter(proposal_jury_reviews::proposal_id.eq(&review.proposal_id))
                            .filter(proposal_jury_reviews::stage.eq(review.stage))
                            .filter(proposal_jury_reviews::state.lt(REVIEW_STATE_DECIDED)),
                    )
                    .set((
                        proposal_jury_reviews::state.eq(REVIEW_STATE_DECIDED),
                        proposal_jury_reviews::decision.eq(review.decision),
                        proposal_jury_reviews::failed.eq(review.failed),
                        proposal_jury_reviews::case_id.eq(&review.case_id),
                        proposal_jury_reviews::decided_at_ms.eq(review.decided_at_ms),
                        proposal_jury_reviews::event_id.eq(&review.event_id),
                        proposal_jury_reviews::transaction_id.eq(&review.transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::Unavailable {
                    review,
                    at_ms,
                    deferred_since_ms,
                    formable_seats,
                    required_seats,
                } => {
                    total += diesel::insert_into(proposal_jury_reviews::table)
                        .values(review)
                        .on_conflict((
                            proposal_jury_reviews::proposal_id,
                            proposal_jury_reviews::stage,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    // A case that could not seat its jurors reports no seat counts; the counts from
                    // the last check before it are kept.
                    let (formable, required) = if review.case_id.is_some() {
                        (None, None)
                    } else {
                        (Some(*formable_seats), Some(*required_seats))
                    };
                    total += diesel::update(
                        proposal_jury_reviews::table
                            .filter(proposal_jury_reviews::proposal_id.eq(&review.proposal_id))
                            .filter(proposal_jury_reviews::stage.eq(review.stage))
                            .filter(proposal_jury_reviews::state.lt(REVIEW_STATE_DECIDED)),
                    )
                    .set((
                        proposal_jury_reviews::state.eq(review.state),
                        proposal_jury_reviews::jury_unavailable_at_ms.eq(Some(*at_ms)),
                        proposal_jury_reviews::jury_deferred_since_ms.eq(Some(*deferred_since_ms)),
                        proposal_jury_reviews::event_id.eq(&review.event_id),
                        proposal_jury_reviews::transaction_id.eq(&review.transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    if let (Some(formable), Some(required)) = (formable, required) {
                        total += diesel::update(
                            proposal_jury_reviews::table
                                .filter(proposal_jury_reviews::proposal_id.eq(&review.proposal_id))
                                .filter(proposal_jury_reviews::stage.eq(review.stage)),
                        )
                        .set((
                            proposal_jury_reviews::jury_formable_seats.eq(Some(formable)),
                            proposal_jury_reviews::jury_required_seats.eq(Some(required)),
                        ))
                        .execute(conn)
                        .await?;
                    }
                }
                JuryRow::Refunded(refund) => {
                    total += diesel::insert_into(proposal_refunds::table)
                        .values(refund)
                        .on_conflict(proposal_refunds::proposal_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    // The chain decides the community step in the same call and emits no decision
                    // event for it: the step is rejected, and no case decided it.
                    total += diesel::update(
                        proposal_jury_reviews::table
                            .filter(proposal_jury_reviews::proposal_id.eq(&refund.proposal_id))
                            .filter(proposal_jury_reviews::stage.eq(REVIEW_STAGE_COMMUNITY))
                            .filter(proposal_jury_reviews::state.lt(REVIEW_STATE_DECIDED)),
                    )
                    .set((
                        proposal_jury_reviews::state.eq(REVIEW_STATE_DECIDED),
                        proposal_jury_reviews::decision.eq(Some(REVIEW_DECISION_REJECT)),
                        proposal_jury_reviews::failed.eq(true),
                        proposal_jury_reviews::case_id.eq(None::<String>),
                        proposal_jury_reviews::decided_at_ms.eq(Some(refund.refunded_at_ms)),
                        proposal_jury_reviews::event_id.eq(&refund.event_id),
                        proposal_jury_reviews::transaction_id.eq(&refund.transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::VoteRefundClaimed(claim) => {
                    let inserted = diesel::insert_into(proposal_vote_refunds::table)
                        .values(claim)
                        .on_conflict((
                            proposal_vote_refunds::proposal_id,
                            proposal_vote_refunds::voter,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    total += inserted;
                    // Only a first sighting counts against what is outstanding, so a replay is a no-op.
                    if inserted > 0 {
                        total += diesel::update(
                            proposal_refunds::table
                                .filter(proposal_refunds::proposal_id.eq(&claim.proposal_id)),
                        )
                        .set(
                            proposal_refunds::outstanding_mist
                                .eq(proposal_refunds::outstanding_mist - claim.amount_mist),
                        )
                        .execute(conn)
                        .await?;
                    }
                }
                JuryRow::VoteRefundsSwept {
                    proposal_id,
                    amount_mist,
                    swept_at_ms,
                } => {
                    total += diesel::update(
                        proposal_refunds::table
                            .filter(proposal_refunds::proposal_id.eq(proposal_id))
                            .filter(proposal_refunds::swept_at_ms.is_null()),
                    )
                    .set((
                        proposal_refunds::swept_at_ms.eq(Some(*swept_at_ms)),
                        proposal_refunds::swept_mist.eq(Some(*amount_mist)),
                        proposal_refunds::outstanding_mist.eq(0),
                    ))
                    .execute(conn)
                    .await?;
                }
                JuryRow::Paid(payout) => {
                    total += diesel::insert_into(jury_payouts::table)
                        .values(payout)
                        .on_conflict((jury_payouts::case_id, jury_payouts::agent_id))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}

#[async_trait]
impl myso_indexer_alt_framework::pipeline::sequential::Handler for JuryHandler {
    type Store = myso_indexer_alt_framework::postgres::Db;
    type Batch = Vec<JuryRow>;

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
        parse_event_contents, BcsCaseFinalized, BcsCaseOpened, BcsCommunityVoteNeedsJuryEvent,
        BcsDelegateReferredToJuryEvent, BcsJuryDecisionAppliedEvent, BcsJuryDrawn,
        BcsJuryPolicyUpdatedEvent, BcsMoveObjectId, BcsProposalEscalatedToJuryEvent,
        BcsProposalRejectedByJuryEvent, BcsRoundFinalized, BcsVoteRevealed,
    };
    use super::*;

    fn addr(hex: &str) -> AccountAddress {
        AccountAddress::from_hex_literal(hex).unwrap()
    }

    fn id(hex: &str) -> BcsMoveObjectId {
        BcsMoveObjectId::from_address(addr(hex))
    }

    fn rows(event_name: &str, bytes: &[u8]) -> Vec<JuryRow> {
        let json =
            parse_event_contents(JURY_MODULE, event_name, bytes).expect("BCS parse should succeed");
        jury_rows(event_name, &json, "digest:1").expect("row builder should accept")
    }

    #[test]
    fn every_jury_event_is_handled() {
        // An event is a struct jury.move emits; helper structs that are never emitted (the
        // eligibility snapshot) are not events.
        let move_source =
            include_str!("../../../myso-framework/packages/myso-social/sources/jury.move");
        let emitted: Vec<&str> = move_source
            .lines()
            .filter_map(|l| l.trim().strip_prefix("event::emit("))
            .filter_map(|rest| rest.split(|c: char| !c.is_alphanumeric()).next())
            .collect();
        let mut declared: Vec<&str> = move_source
            .lines()
            .filter_map(|l| l.trim().strip_prefix("public struct "))
            .filter(|rest| rest.contains("has copy, drop"))
            .filter_map(|rest| rest.split_whitespace().next())
            .filter(|name| emitted.contains(name))
            .collect();
        declared.sort_unstable();
        declared.dedup();
        let mut handled: Vec<&str> = JURY_EVENTS.to_vec();
        handled.sort_unstable();
        assert_eq!(declared, handled);
    }

    #[test]
    fn a_case_opens_in_the_evidence_state() {
        let out = rows(
            "CaseOpened",
            &bcs::to_bytes(&BcsCaseOpened {
                case_id: id("0xca5e"),
                subject_kind: 1,
                subject_id: id("0x7a5c"),
                stage: 0,
                menu_len: 3,
                approval_bps: 5000,
                quorum: 2,
                round_sizes: vec![3, 5],
                appealable: true,
                failure_path: 1,
                opened_by: addr("0xb0b"),
                fee_mist: 60,
                created_at_ms: 100,
                evidence_deadline_ms: 200,
            })
            .unwrap(),
        );
        let [JuryRow::CaseOpened(case)] = out.as_slice() else {
            panic!("expected CaseOpened");
        };
        assert_eq!(case.state, CASE_STATE_EVIDENCE);
        assert_eq!(case.round_sizes, "[3,5]");
        assert!(case.subject_id.ends_with("7a5c"));
        assert_eq!(case.transaction_id, "digest");
    }

    #[test]
    fn a_draw_creates_one_drafted_slot_per_juror() {
        let out = rows(
            "JuryDrawn",
            &bcs::to_bytes(&BcsJuryDrawn {
                case_id: id("0xca5e"),
                round: 0,
                jurors: vec![id("0xa1"), id("0xa2"), id("0xa3")],
                accept_deadline_ms: 900,
                replacement: false,
            })
            .unwrap(),
        );
        let [JuryRow::Drawn {
            round,
            slots,
            replacement,
            ..
        }] = out.as_slice()
        else {
            panic!("expected Drawn");
        };
        assert!(!*replacement);
        assert_eq!(round.accept_deadline_ms, 900);
        assert_eq!(slots.len(), 3);
        assert!(slots
            .iter()
            .all(|s| s.status == SLOT_DRAFTED && s.round == 0));
    }

    #[test]
    fn a_reveal_keeps_the_digests_and_refs() {
        let out = rows(
            "VoteRevealed",
            &bcs::to_bytes(&BcsVoteRevealed {
                case_id: id("0xca5e"),
                round: 1,
                agent_id: id("0xa1"),
                outcome: 2,
                rationale_digest: vec![0xaa; 32],
                evidence_refs: vec![0, 3],
                bundle_digest: vec![0xbb; 32],
                revealed_at_ms: 777,
            })
            .unwrap(),
        );
        let [JuryRow::Revealed {
            outcome,
            rationale_digest,
            evidence_refs,
            bundle_digest,
            ..
        }] = out.as_slice()
        else {
            panic!("expected Revealed");
        };
        assert_eq!(*outcome, 2);
        assert_eq!(rationale_digest, &"aa".repeat(32));
        assert_eq!(evidence_refs, "[0,3]");
        assert_eq!(bundle_digest, &"bb".repeat(32));
    }

    #[test]
    fn a_tally_may_have_no_leader() {
        let out = rows(
            "RoundFinalized",
            &bcs::to_bytes(&BcsRoundFinalized {
                case_id: id("0xca5e"),
                round: 0,
                tally: vec![1, 1, 0],
                abstain: 0,
                revealed: 2,
                decisive: false,
                leading: None,
                appeal_deadline_ms: 0,
            })
            .unwrap(),
        );
        let [JuryRow::RoundFinalized {
            tally,
            decisive,
            leading,
            appeal_deadline_ms,
            ..
        }] = out.as_slice()
        else {
            panic!("expected RoundFinalized");
        };
        assert_eq!(tally, "[1,1,0]");
        assert!(!*decisive);
        assert!(leading.is_none());
        assert!(appeal_deadline_ms.is_none());
    }

    #[test]
    fn a_finalized_case_carries_its_outcome_and_pay() {
        let out = rows(
            "CaseFinalized",
            &bcs::to_bytes(&BcsCaseFinalized {
                case_id: id("0xca5e"),
                outcome: 1,
                round: 1,
                appeals_used: 1,
                failed_appeal: false,
                participants: 5,
                per_juror_fee: 20,
                finalized_at_ms: 999,
            })
            .unwrap(),
        );
        assert!(matches!(
            out.as_slice(),
            [JuryRow::Finalized {
                outcome: 1,
                appeals_used: 1,
                participants: 5,
                per_juror_fee_mist: 20,
                ..
            }]
        ));
    }

    #[test]
    fn every_governance_jury_event_is_declared_and_handled() {
        let move_source =
            include_str!("../../../myso-framework/packages/myso-social/sources/governance.move");
        let declared: Vec<&str> = move_source
            .lines()
            .filter_map(|l| l.trim().strip_prefix("public struct "))
            .filter(|rest| rest.contains("has copy, drop"))
            .filter_map(|rest| rest.split_whitespace().next())
            .collect();
        for name in GOVERNANCE_JURY_EVENTS {
            assert!(
                declared.contains(name),
                "{name} is not an event of governance.move"
            );
        }
        // Every governance event that mentions the jury must be in the list.
        for name in &declared {
            if name.contains("Jury") {
                assert!(
                    GOVERNANCE_JURY_EVENTS.contains(name),
                    "{name} is not indexed"
                );
            }
        }
    }

    fn gov_rows(event_name: &str, bytes: &[u8]) -> Vec<JuryRow> {
        let json = parse_event_contents(GOVERNANCE_MODULE, event_name, bytes)
            .expect("BCS parse should succeed");
        jury_rows(event_name, &json, "digest:3").expect("row builder should accept")
    }

    #[test]
    fn a_referral_records_the_delegate_and_the_running_count() {
        let out = gov_rows(
            "DelegateReferredToJuryEvent",
            &bcs::to_bytes(&BcsDelegateReferredToJuryEvent {
                proposal_id: id("0x9a"),
                delegate_address: addr("0xd2"),
                reason: Some("too big for us".into()),
                referral_count: 2,
                majority_reached: true,
                referred_at_ms: 40,
            })
            .unwrap(),
        );
        let [JuryRow::Referred { review, referral }] = out.as_slice() else {
            panic!("expected Referred");
        };
        assert_eq!(
            (
                review.stage,
                review.state,
                review.referral_count,
                review.trigger
            ),
            (1, 1, 2, 1)
        );
        assert_eq!(referral.reason.as_deref(), Some("too big for us"));
        assert!(referral.delegate_address.ends_with("d2"));
    }

    #[test]
    fn escalation_names_the_case_and_the_step() {
        let out = gov_rows(
            "ProposalEscalatedToJuryEvent",
            &bcs::to_bytes(&BcsProposalEscalatedToJuryEvent {
                proposal_id: id("0x9a"),
                stage: 2,
                trigger: 4,
                case_id: id("0xca5e"),
                jury_fee_mist: 2500,
                pool_remaining_mist: 7500,
                escalated_at_ms: 50,
            })
            .unwrap(),
        );
        let [JuryRow::Escalated(review)] = out.as_slice() else {
            panic!("expected Escalated");
        };
        assert_eq!((review.stage, review.state, review.trigger), (2, 3, 4));
        assert!(review.case_id.as_deref().unwrap().ends_with("ca5e"));
        assert_eq!(review.escalated_at_ms, Some(50));
    }

    #[test]
    fn a_vote_waiting_for_the_jury_is_recorded_before_any_case_exists() {
        let out = gov_rows(
            "CommunityVoteNeedsJuryEvent",
            &bcs::to_bytes(&BcsCommunityVoteNeedsJuryEvent {
                proposal_id: id("0x9a"),
                trigger: 4,
                votes_for: 0,
                votes_against: 0,
                quorum_votes: 2,
                time_ms: 60,
            })
            .unwrap(),
        );
        let [JuryRow::NeedsJury(review)] = out.as_slice() else {
            panic!("expected NeedsJury");
        };
        assert_eq!((review.stage, review.state, review.trigger), (2, 2, 4));
        assert!(review.case_id.is_none());
    }

    #[test]
    fn a_decision_without_a_case_is_the_paused_jury_default() {
        let out = gov_rows(
            "JuryDecisionAppliedEvent",
            &bcs::to_bytes(&BcsJuryDecisionAppliedEvent {
                proposal_id: id("0x9a"),
                stage: 2,
                decision: 1,
                failed: true,
                case_id: None,
                applied_at_ms: 70,
            })
            .unwrap(),
        );
        let [JuryRow::StepDecided(review)] = out.as_slice() else {
            panic!("expected StepDecided");
        };
        assert_eq!(
            (review.state, review.decision, review.failed),
            (4, Some(1), true)
        );
        assert!(review.case_id.is_none());
    }

    #[test]
    fn a_policy_change_carries_every_parameter() {
        let out = gov_rows(
            "JuryPolicyUpdatedEvent",
            &bcs::to_bytes(&BcsJuryPolicyUpdatedEvent {
                registry_id: id("0x7e6"),
                council_stage_enabled: true,
                community_stage_enabled: false,
                council_window_ms: 259_200_000,
                jury_size: 5,
                round2_size: 11,
                jury_fee_bps: 2500,
                jury_fee_cap_mist: 0,
                min_pool_for_jury_mist: 1_000,
                quorum: 3,
                approval_bps_council: 5000,
                approval_bps_community: 6667,
                updated_at_ms: 80,
            })
            .unwrap(),
        );
        let [JuryRow::PolicyUpdated(policy)] = out.as_slice() else {
            panic!("expected PolicyUpdated");
        };
        assert!(policy.council_stage_enabled && !policy.community_stage_enabled);
        assert_eq!(
            (policy.jury_size, policy.round2_size, policy.jury_fee_bps),
            (5, 11, 2500)
        );
    }

    #[test]
    fn a_rejection_by_the_jury_adds_nothing_the_status_events_do_not() {
        let out = gov_rows(
            "ProposalRejectedByJuryEvent",
            &bcs::to_bytes(&BcsProposalRejectedByJuryEvent {
                proposal_id: id("0x9a"),
                stage: 1,
                rejection_time: 90,
            })
            .unwrap(),
        );
        assert!(out.is_empty());
    }

    #[test]
    fn a_config_change_keeps_nothing() {
        let json = serde_json::json!({ "updated_by": "0xad", "section": 1, "timestamp_ms": 5 });
        assert!(jury_rows("JuryConfigUpdated", &json, "d:0")
            .unwrap()
            .is_empty());
    }

    #[test]
    fn an_unrelated_event_name_builds_no_rows() {
        assert!(jury_rows("SomethingElse", &serde_json::json!({}), "d:0").is_none());
    }
}
