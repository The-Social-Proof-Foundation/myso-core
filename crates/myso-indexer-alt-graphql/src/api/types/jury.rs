// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! The AI Jury as GraphQL sees it: a proposal's jury records, a registry's jury policy and a case
//! summary. Jurors, votes and evidence are served by the social server's REST routes.

use std::str::FromStr;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Enum;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    JuryCaseSummaryRow, ProposalJuryReferralRow, ProposalJuryReviewRow, RegistryJuryPolicyRow,
    SocialPgReader,
};

use crate::api::scalars::myso_address::MySoAddress;

/// The human step of a proposal that the jury can take over.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum JuryStage {
    /// The delegate council's review.
    Council,
    /// The public community vote.
    Community,
}

/// Where the jury stands on one step.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum JuryReviewState {
    /// Delegates are referring the proposal; no majority yet.
    Referring,
    /// The step earned a jury and anyone can hand the proposal over.
    AwaitingEscalation,
    /// A jury holds the step, so the humans at it can no longer decide.
    WithJury,
    /// The jury decided, or could not and the step's default applied.
    Decided,
}

/// Why a step went to the jury.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum JuryTrigger {
    /// Not recorded yet.
    None,
    /// A majority of delegates referred it.
    ReferralMajority,
    /// The council tied.
    CouncilTie,
    /// The council ran out of time.
    CouncilDeadline,
    /// The community vote missed quorum.
    QuorumNotMet,
    /// The community vote tied.
    VoteTie,
}

/// What the jury decided.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum JuryDecision {
    /// Council step: send to the community vote. Community step: approve.
    Forward,
    /// Reject the proposal.
    Reject,
}

/// Lifecycle of a jury case.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum JuryCaseState {
    Evidence,
    AwaitingDraw,
    Selecting,
    Voting,
    Tallied,
    Final,
    Failed,
    Cancelled,
}

/// What a case is about.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum JurySubjectKind {
    Task,
    GovernanceProposal,
}

impl JuryStage {
    fn from_code(code: i16) -> Self {
        if code == 2 { Self::Community } else { Self::Council }
    }
}

impl JuryReviewState {
    fn from_code(code: i16) -> Self {
        match code {
            2 => Self::AwaitingEscalation,
            3 => Self::WithJury,
            4 => Self::Decided,
            _ => Self::Referring,
        }
    }
}

impl JuryTrigger {
    fn from_code(code: i16) -> Self {
        match code {
            1 => Self::ReferralMajority,
            2 => Self::CouncilTie,
            3 => Self::CouncilDeadline,
            4 => Self::QuorumNotMet,
            5 => Self::VoteTie,
            _ => Self::None,
        }
    }
}

impl JuryDecision {
    fn from_code(code: i16) -> Self {
        if code == 0 { Self::Forward } else { Self::Reject }
    }
}

impl JuryCaseState {
    fn from_code(code: i16) -> Self {
        match code {
            2 => Self::AwaitingDraw,
            3 => Self::Selecting,
            4 => Self::Voting,
            5 => Self::Tallied,
            6 => Self::Final,
            7 => Self::Failed,
            8 => Self::Cancelled,
            _ => Self::Evidence,
        }
    }
}

impl JurySubjectKind {
    fn from_code(code: i16) -> Self {
        if code == 2 { Self::GovernanceProposal } else { Self::Task }
    }
}

/// One step of a proposal that has reached, or may reach, the AI Jury.
#[derive(Clone)]
pub(crate) struct ProposalJuryReview {
    inner: ProposalJuryReviewRow,
}

impl ProposalJuryReview {
    pub(crate) fn from_row(inner: ProposalJuryReviewRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ProposalJuryReview {
    async fn proposal_id(&self) -> &str {
        &self.inner.proposal_id
    }

    async fn stage(&self) -> JuryStage {
        JuryStage::from_code(self.inner.stage)
    }

    async fn state(&self) -> JuryReviewState {
        JuryReviewState::from_code(self.inner.state)
    }

    async fn trigger(&self) -> JuryTrigger {
        JuryTrigger::from_code(self.inner.trigger)
    }

    /// The jury case, once the step has been handed over.
    async fn case_id(&self) -> Option<&str> {
        self.inner.case_id.as_deref()
    }

    /// How many delegates referred the proposal at the council step.
    async fn referral_count(&self) -> i64 {
        self.inner.referral_count
    }

    async fn escalated_at_ms(&self) -> Option<i64> {
        self.inner.escalated_at_ms
    }

    async fn decided_at_ms(&self) -> Option<i64> {
        self.inner.decided_at_ms
    }

    /// Null until the jury has decided or its default has applied.
    async fn decision(&self) -> Option<JuryDecision> {
        self.inner.decision.map(JuryDecision::from_code)
    }

    /// The jury could not decide, so the step's default applied.
    async fn failed(&self) -> bool {
        self.inner.failed
    }

    /// The case behind this step. Jurors, votes and evidence are on the REST routes.
    async fn case(&self, ctx: &Context<'_>) -> Option<JuryCase> {
        let case_id = self.inner.case_id.as_deref()?;
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader.get_jury_case(case_id).await.ok()??;
        Some(JuryCase::from_row(row))
    }
}

/// A delegate's referral of a proposal to the AI Jury.
#[derive(Clone)]
pub(crate) struct ProposalJuryReferral {
    inner: ProposalJuryReferralRow,
}

impl ProposalJuryReferral {
    pub(crate) fn from_row(inner: ProposalJuryReferralRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ProposalJuryReferral {
    async fn delegate_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.delegate_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn reason(&self) -> Option<&str> {
        self.inner.reason.as_deref()
    }

    async fn referred_at_ms(&self) -> i64 {
        self.inner.referred_at_ms
    }
}

/// A registry's switch for the AI Jury at each human step.
#[derive(Clone)]
pub(crate) struct RegistryJuryPolicy {
    inner: RegistryJuryPolicyRow,
}

impl RegistryJuryPolicy {
    pub(crate) fn from_row(inner: RegistryJuryPolicyRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl RegistryJuryPolicy {
    async fn registry_id(&self) -> &str {
        &self.inner.registry_id
    }

    /// The jury can take over the delegate council's review.
    async fn council_stage_enabled(&self) -> bool {
        self.inner.council_stage_enabled
    }

    /// The jury can decide a community vote that missed quorum or tied.
    async fn community_stage_enabled(&self) -> bool {
        self.inner.community_stage_enabled
    }

    /// How long the council has before the jury can take over; 0 means no deadline.
    async fn council_window_ms(&self) -> i64 {
        self.inner.council_window_ms
    }

    async fn jury_size(&self) -> i64 {
        self.inner.jury_size
    }

    /// Size of the second, larger jury when the first cannot decide.
    async fn round2_size(&self) -> i64 {
        self.inner.round2_size
    }

    /// Share of the proposal's reward pool paid to jurors, in basis points.
    async fn jury_fee_bps(&self) -> i64 {
        self.inner.jury_fee_bps
    }

    async fn jury_fee_cap_mist(&self) -> i64 {
        self.inner.jury_fee_cap_mist
    }

    /// Proposals with a smaller reward pool skip the jury and behave as before.
    async fn min_pool_for_jury_mist(&self) -> i64 {
        self.inner.min_pool_for_jury_mist
    }

    /// Reveals a round needs before it can decide.
    async fn quorum(&self) -> i64 {
        self.inner.quorum
    }

    async fn approval_bps_council(&self) -> i64 {
        self.inner.approval_bps_council
    }

    async fn approval_bps_community(&self) -> i64 {
        self.inner.approval_bps_community
    }

    async fn updated_at_ms(&self) -> i64 {
        self.inner.updated_at_ms
    }
}

/// A jury case without its jurors, rounds or evidence.
#[derive(Clone)]
pub(crate) struct JuryCase {
    inner: JuryCaseSummaryRow,
}

impl JuryCase {
    pub(crate) fn from_row(inner: JuryCaseSummaryRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl JuryCase {
    async fn case_id(&self) -> &str {
        &self.inner.case_id
    }

    async fn subject_kind(&self) -> JurySubjectKind {
        JurySubjectKind::from_code(self.inner.subject_kind)
    }

    /// The task or proposal the case decides.
    async fn subject_id(&self) -> &str {
        &self.inner.subject_id
    }

    /// For a proposal, the step the case decides.
    async fn stage(&self) -> JuryStage {
        JuryStage::from_code(self.inner.stage)
    }

    async fn state(&self) -> JuryCaseState {
        JuryCaseState::from_code(self.inner.state)
    }

    async fn current_round(&self) -> i64 {
        self.inner.current_round
    }

    /// Index into the case's fixed menu of outcomes, once the case is final.
    async fn final_outcome(&self) -> Option<i16> {
        self.inner.final_outcome
    }

    async fn appeals_used(&self) -> i64 {
        self.inner.appeals_used
    }

    async fn fee_mist(&self) -> i64 {
        self.inner.fee_mist
    }

    /// What each juror who voted is paid, once the case has ended.
    async fn per_juror_fee_mist(&self) -> Option<i64> {
        self.inner.per_juror_fee_mist
    }

    async fn created_at_ms(&self) -> i64 {
        self.inner.created_at_ms
    }

    async fn finalized_at_ms(&self) -> Option<i64> {
        self.inner.finalized_at_ms
    }
}
