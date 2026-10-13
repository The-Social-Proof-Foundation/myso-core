// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    jury_cases, jury_evidence, jury_payouts, jury_roster, jury_rounds, jury_slots,
    proposal_jury_referrals, proposal_jury_reviews, registry_jury_policies,
};

pub const CASE_STATE_EVIDENCE: i16 = 1;
pub const CASE_STATE_AWAITING_DRAW: i16 = 2;
pub const CASE_STATE_SELECTING: i16 = 3;
pub const CASE_STATE_VOTING: i16 = 4;
pub const CASE_STATE_TALLIED: i16 = 5;
pub const CASE_STATE_FINAL: i16 = 6;
pub const CASE_STATE_FAILED: i16 = 7;
pub const CASE_STATE_CANCELLED: i16 = 8;

pub const SLOT_DRAFTED: i16 = 1;
pub const SLOT_ACCEPTED: i16 = 2;
pub const SLOT_COMMITTED: i16 = 3;
pub const SLOT_REVEALED: i16 = 4;
pub const SLOT_REPLACED: i16 = 5;
pub const SLOT_DEFAULTED: i16 = 6;

pub const ROSTER_LEFT: i16 = 0;
pub const ROSTER_ACTIVE: i16 = 1;
pub const ROSTER_SUSPENDED: i16 = 3;

/// `DutyScoreChanged.reason` for a juror who committed but never revealed.
pub const DUTY_NO_REVEAL: i16 = 3;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = jury_cases)]
pub struct NewJuryCase {
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
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = jury_rounds)]
pub struct NewJuryRound {
    pub case_id: String,
    pub round: i64,
    pub accept_deadline_ms: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = jury_slots)]
pub struct NewJurySlot {
    pub case_id: String,
    pub round: i64,
    pub agent_id: String,
    pub status: i16,
    pub accept_deadline_ms: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = jury_evidence)]
pub struct NewJuryEvidence {
    pub case_id: String,
    pub idx: i64,
    pub submitter: String,
    pub round: i64,
    pub kind: i16,
    pub digest: String,
    pub uri: String,
    pub submitted_at_ms: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = jury_payouts)]
pub struct NewJuryPayout {
    pub case_id: String,
    pub agent_id: String,
    pub recipient: String,
    pub amount_mist: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = jury_roster)]
pub struct NewJuryRosterEntry {
    pub agent_id: String,
    pub owner: String,
    pub memory_account_id: String,
    pub organization_id: String,
    pub payout_address: String,
    pub model_family: i16,
    pub exempt: bool,
    pub status: i16,
    pub joined_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

pub const REVIEW_STAGE_COUNCIL: i16 = 1;
pub const REVIEW_STAGE_COMMUNITY: i16 = 2;
pub const REVIEW_STATE_REFERRING: i16 = 1;
pub const REVIEW_STATE_AWAITING: i16 = 2;
pub const REVIEW_STATE_ESCALATED: i16 = 3;
pub const REVIEW_STATE_DECIDED: i16 = 4;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = registry_jury_policies)]
pub struct NewRegistryJuryPolicy {
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
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

/// Inserted the first time a step is mentioned; later events move it forward.
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = proposal_jury_reviews)]
pub struct NewProposalJuryReview {
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
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = proposal_jury_referrals)]
pub struct NewProposalJuryReferral {
    pub proposal_id: String,
    pub delegate_address: String,
    pub reason: Option<String>,
    pub referred_at_ms: i64,
    pub event_id: String,
}
