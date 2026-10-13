// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! AI Jury reads for GraphQL: a proposal's jury records, a registry's jury policy and a case
//! summary. Everything here is on-chain fact; votes stay hidden until revealed and are not read.

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::Selectable;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::{
    jury_cases, proposal_jury_referrals, proposal_jury_reviews, registry_jury_policies,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

/// One human step of a proposal that has reached, or may reach, the jury.
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = proposal_jury_reviews)]
pub struct ProposalJuryReviewRow {
    pub proposal_id: String,
    /// 1 council, 2 community.
    pub stage: i16,
    /// 1 referring, 2 awaiting escalation, 3 with the jury, 4 decided.
    pub state: i16,
    /// 1 referral majority, 2 council tie, 3 council deadline, 4 quorum not met, 5 tied vote.
    pub trigger: i16,
    pub case_id: Option<String>,
    pub referral_count: i64,
    pub escalated_at_ms: Option<i64>,
    pub decided_at_ms: Option<i64>,
    /// 0 forward (advance or approve), 1 reject.
    pub decision: Option<i16>,
    /// The jury could not decide and the step's default applied.
    pub failed: bool,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = proposal_jury_referrals)]
pub struct ProposalJuryReferralRow {
    pub delegate_address: String,
    pub reason: Option<String>,
    pub referred_at_ms: i64,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = registry_jury_policies)]
pub struct RegistryJuryPolicyRow {
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

/// A case without its jurors, rounds or evidence.
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = jury_cases)]
pub struct JuryCaseSummaryRow {
    pub case_id: String,
    /// 1 task, 2 governance proposal.
    pub subject_kind: i16,
    pub subject_id: String,
    pub stage: i16,
    pub menu_len: i16,
    /// 1 evidence, 2 awaiting draw, 3 selecting, 4 voting, 5 tallied, 6 final, 7 failed, 8 cancelled.
    pub state: i16,
    pub current_round: i64,
    pub final_outcome: Option<i16>,
    pub appeals_used: i64,
    pub fee_mist: i64,
    pub per_juror_fee_mist: Option<i64>,
    pub created_at_ms: i64,
    pub finalized_at_ms: Option<i64>,
}

pub(crate) async fn get_proposal_jury_reviews(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ProposalJuryReviewRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = proposal_jury_reviews::table
        .filter(proposal_jury_reviews::proposal_id.eq(proposal_id))
        .order(proposal_jury_reviews::stage.asc())
        .select(ProposalJuryReviewRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn get_proposal_jury_referrals(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ProposalJuryReferralRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let rows = proposal_jury_referrals::table
        .filter(proposal_jury_referrals::proposal_id.eq(proposal_id))
        .order(proposal_jury_referrals::referred_at_ms.asc())
        .select(ProposalJuryReferralRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn get_registry_jury_policy(
    conn: &mut Connection<'_>,
    registry_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<RegistryJuryPolicyRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = registry_jury_policies::table
        .filter(registry_jury_policies::registry_id.eq(registry_id))
        .select(RegistryJuryPolicyRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub(crate) async fn get_jury_case(
    conn: &mut Connection<'_>,
    case_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<JuryCaseSummaryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = jury_cases::table
        .filter(jury_cases::case_id.eq(case_id))
        .select(JuryCaseSummaryRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}
