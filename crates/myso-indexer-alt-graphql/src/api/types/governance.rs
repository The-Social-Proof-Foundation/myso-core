// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    DelegateRow, GovernanceRegistryRow, GovernanceStatsRow, PlatformRevenueSummaryRow, ProposalRow,
    SocialPgReader,
};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::platform::Platform;
use crate::api::types::profile_summary::ProfileSummary;

/// Governance registry config (voting params) for GraphQL.
#[derive(Clone)]
pub(crate) struct GovernanceRegistryConfig {
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub max_votes_per_user: i64,
    pub voting_period_ms: i64,
    pub quorum_votes: i64,
}

#[derive(Clone)]
pub(crate) struct Proposal {
    inner: ProposalRow,
}

impl Proposal {
    pub(crate) fn from_row(inner: ProposalRow) -> Self {
        Self { inner }
    }
}

#[derive(Clone)]
pub(crate) struct ProposalVotes {
    delegate_for: i64,
    delegate_against: i64,
    community_for: i64,
    community_against: i64,
}

#[Object]
impl Proposal {
    /// Unique proposal identifier (object ID on-chain).
    async fn proposal_id(&self) -> &str {
        &self.inner.id
    }

    /// Registry type (0=ecosystem, 1=proof of creativity, 2=platform).
    async fn registry_type(&self) -> i16 {
        self.inner.proposal_type
    }

    /// Proposal status (0=submitted, 1=delegate_review, 2=community_voting, 3=approved, 4=rejected, 5=implemented, 6=rescinded).
    async fn status(&self) -> i16 {
        self.inner.status
    }

    /// Vote counts (delegate and community).
    async fn votes(&self) -> ProposalVotes {
        ProposalVotes {
            delegate_for: self.inner.delegate_approval_count,
            delegate_against: self.inner.delegate_rejection_count,
            community_for: self.inner.community_votes_for,
            community_against: self.inner.community_votes_against,
        }
    }

    /// Proposal title.
    async fn title(&self) -> &str {
        &self.inner.title
    }

    /// Proposal description.
    async fn description(&self) -> &str {
        &self.inner.description
    }

    /// Submitter address.
    async fn submitter(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.submitter)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the proposal submitter.
    async fn submitter_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.submitter).await
    }

    /// Submission time (epoch ms).
    async fn submission_time(&self) -> i64 {
        self.inner.submission_time
    }

    /// Voting start time (epoch ms, when in community voting).
    async fn voting_start_time(&self) -> Option<i64> {
        self.inner.voting_start_time
    }

    /// Voting end time (epoch ms).
    async fn voting_end_time(&self) -> Option<i64> {
        self.inner.voting_end_time
    }

    /// Reward pool amount.
    async fn reward_pool(&self) -> i64 {
        self.inner.reward_pool
    }

    /// Reference ID (e.g. linked post or object).
    async fn reference_id(&self) -> Option<&str> {
        self.inner.reference_id.as_deref()
    }

    /// Implemented description (when status=implemented).
    async fn implemented_description(&self) -> Option<&str> {
        self.inner.implemented_description.as_deref()
    }

    /// Implementation time (epoch ms).
    async fn implementation_time(&self) -> Option<i64> {
        self.inner.implementation_time
    }

    /// Rescind time (epoch ms, when rescinded).
    async fn rescind_time(&self) -> Option<i64> {
        self.inner.rescind_time
    }

    /// Anonymous voters count (when anonymous voting enabled).
    async fn anonymous_voters_count(&self) -> Option<i64> {
        self.inner.anonymous_voters_count
    }
}

#[Object]
impl ProposalVotes {
    /// Delegate votes for (approve).
    async fn delegate_for(&self) -> i64 {
        self.delegate_for
    }

    /// Delegate votes against (reject).
    async fn delegate_against(&self) -> i64 {
        self.delegate_against
    }

    /// Community votes for (approve).
    async fn community_for(&self) -> i64 {
        self.community_for
    }

    /// Community votes against (reject).
    async fn community_against(&self) -> i64 {
        self.community_against
    }
}

#[derive(Clone)]
pub(crate) struct Delegate {
    inner: DelegateRow,
}

impl Delegate {
    pub(crate) fn from_row(inner: DelegateRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl Delegate {
    /// Delegate address.
    async fn address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Registry type (0=ecosystem, 1=proof of creativity, 2=platform).
    async fn registry_type(&self) -> i16 {
        self.inner.registry_type
    }

    /// Upvotes (delegate ratings).
    async fn upvotes(&self) -> i64 {
        self.inner.upvotes
    }

    /// Downvotes (delegate ratings).
    async fn downvotes(&self) -> i64 {
        self.inner.downvotes
    }

    /// Proposals reviewed.
    async fn proposals_reviewed(&self) -> i64 {
        self.inner.proposals_reviewed
    }

    /// Proposals submitted.
    async fn proposals_submitted(&self) -> i64 {
        self.inner.proposals_submitted
    }

    /// Term start (epoch).
    async fn term_start(&self) -> i64 {
        self.inner.term_start
    }

    /// Term end (epoch).
    async fn term_end(&self) -> i64 {
        self.inner.term_end
    }

    /// Whether the delegate is currently active.
    async fn is_active(&self) -> bool {
        self.inner.is_active
    }
}

#[derive(Clone)]
pub(crate) struct GovernanceRegistry {
    inner: GovernanceRegistryRow,
    platform_id: Option<String>,
}

impl GovernanceRegistry {
    pub(crate) fn from_row(inner: GovernanceRegistryRow) -> Self {
        Self {
            inner,
            platform_id: None,
        }
    }

    pub(crate) fn from_row_with_platform(
        inner: GovernanceRegistryRow,
        platform_id: Option<String>,
    ) -> Self {
        Self {
            inner,
            platform_id,
        }
    }
}

#[derive(Clone)]
pub(crate) struct GovernanceStats {
    inner: GovernanceStatsRow,
}

#[derive(Clone)]
pub(crate) struct PlatformRevenueSummary {
    inner: PlatformRevenueSummaryRow,
}

#[Object]
impl GovernanceRegistry {
    /// Registry type (0=ecosystem, 1=proof of creativity, 2=platform).
    async fn registry_type(&self) -> i16 {
        self.inner.registry_type
    }

    /// Registry object ID on-chain.
    async fn registry_id(&self) -> &str {
        &self.inner.registry_id
    }

    /// Number of delegates.
    async fn delegate_count(&self) -> i64 {
        self.inner.delegate_count
    }

    /// Voting configuration (term, costs, quorum, etc.).
    async fn config(&self) -> GovernanceRegistryConfig {
        GovernanceRegistryConfig {
            delegate_term_epochs: self.inner.delegate_term_epochs,
            proposal_submission_cost: self.inner.proposal_submission_cost,
            max_votes_per_user: self.inner.max_votes_per_user,
            voting_period_ms: self.inner.voting_period_ms,
            quorum_votes: self.inner.quorum_votes,
        }
    }

    /// Governance stats (delegate/proposal counts by registry type).
    async fn stats(&self, ctx: &Context<'_>) -> Option<GovernanceStats> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_governance_stats_by_registry_type(self.inner.registry_type)
            .await
            .ok()??;
        Some(GovernanceStats { inner: row })
    }

    /// Platform revenue summary (12-month metrics). Only when registry fetched via governanceRegistry(platformId).
    async fn revenue(&self, ctx: &Context<'_>) -> Option<PlatformRevenueSummary> {
        let platform_id = self.platform_id.as_ref()?;
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_platform_revenue_summary(platform_id)
            .await
            .ok()??;
        Some(PlatformRevenueSummary { inner: row })
    }

    /// Platform details (when registry_type=2). Resolved by platform_id when from governanceRegistry(platformId), else by registry_id.
    async fn platform(&self, ctx: &Context<'_>) -> Option<Platform> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = if let Some(pid) = &self.platform_id {
            reader.get_platform_by_id(pid).await.ok()?
        } else if self.inner.registry_type == 2 {
            reader
                .get_platform_by_registry_id(&self.inner.registry_id)
                .await
                .ok()?
        } else {
            return None;
        };
        row.map(Platform::from_db)
    }
}

#[Object]
impl GovernanceStats {
    /// Registry type (0=ecosystem, 1=proof of creativity, 2=platform).
    async fn registry_type(&self) -> i16 {
        self.inner.registry_type
    }

    /// Active delegates count.
    async fn active_delegates(&self) -> i64 {
        self.inner.active_delegates
    }

    /// Pending nominees count.
    async fn pending_nominees(&self) -> i64 {
        self.inner.pending_nominees
    }

    /// Submitted proposals count.
    async fn submitted_proposals(&self) -> i64 {
        self.inner.submitted_proposals
    }

    /// In-review proposals count.
    async fn in_review_proposals(&self) -> i64 {
        self.inner.in_review_proposals
    }

    /// Voting proposals count.
    async fn voting_proposals(&self) -> i64 {
        self.inner.voting_proposals
    }

    /// Approved proposals count.
    async fn approved_proposals(&self) -> i64 {
        self.inner.approved_proposals
    }

    /// Rejected proposals count.
    async fn rejected_proposals(&self) -> i64 {
        self.inner.rejected_proposals
    }

    /// Implemented proposals count.
    async fn implemented_proposals(&self) -> i64 {
        self.inner.implemented_proposals
    }

    /// Rescinded proposals count.
    async fn rescinded_proposals(&self) -> i64 {
        self.inner.rescinded_proposals
    }
}

#[Object]
impl PlatformRevenueSummary {
    /// Platform address (object ID).
    async fn platform_address(&self) -> &str {
        &self.inner.platform_address
    }

    /// Total revenue (12-month).
    async fn total_revenue(&self) -> i64 {
        self.inner.total_revenue
    }

    /// Subscription revenue (12-month).
    async fn total_subscription_revenue(&self) -> i64 {
        self.inner.total_subscription_revenue
    }

    /// MyData revenue (12-month).
    async fn total_mydata_revenue(&self) -> i64 {
        self.inner.total_mydata_revenue
    }

    /// SPT revenue (12-month).
    async fn total_spt_revenue(&self) -> i64 {
        self.inner.total_spt_revenue
    }

    /// Total transactions (12-month).
    async fn total_transactions(&self) -> i64 {
        self.inner.total_transactions
    }

    /// Total creators count.
    async fn total_creators(&self) -> i64 {
        self.inner.total_creators
    }

    /// Total payers count.
    async fn total_payers(&self) -> i64 {
        self.inner.total_payers
    }

    /// Average transaction amount.
    async fn avg_transaction_amount(&self) -> f64 {
        self.inner.avg_transaction_amount
    }

    /// Active months count.
    async fn active_months(&self) -> i64 {
        self.inner.active_months
    }

    /// Last active month (ISO date string).
    async fn last_active_month(&self) -> Option<String> {
        self.inner
            .last_active_month
            .map(|d| d.format("%Y-%m-%d").to_string())
    }
}

#[Object]
impl GovernanceRegistryConfig {
    /// Delegate term length in epochs.
    async fn delegate_term_epochs(&self) -> i64 {
        self.delegate_term_epochs
    }

    /// Cost to submit a proposal.
    async fn proposal_submission_cost(&self) -> i64 {
        self.proposal_submission_cost
    }

    /// Max votes per user in community voting.
    async fn max_votes_per_user(&self) -> i64 {
        self.max_votes_per_user
    }

    /// Voting period in milliseconds.
    async fn voting_period_ms(&self) -> i64 {
        self.voting_period_ms
    }

    /// Quorum votes required.
    async fn quorum_votes(&self) -> i64 {
        self.quorum_votes
    }
}
