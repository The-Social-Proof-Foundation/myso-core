// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    PocAnalysisResultRow, PocBadgeRow, PocBeneficiaryVaultRow, PocDisputeRow, PocDisputeVoteRow,
    PocRevenueRedirectionRow, PocVaultClaimRow, PocVaultCoinBalanceRow, PocVaultDepositRow,
};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::json::Json;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::profile_summary::ProfileSummary;

#[derive(Clone)]
pub(crate) struct PocBadge {
    inner: PocBadgeRow,
}

impl PocBadge {
    pub(crate) fn from_row(inner: PocBadgeRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocBadge {
    /// Badge ID.
    async fn badge_id(&self) -> &str {
        &self.inner.badge_id
    }

    /// Post ID this badge was issued for.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Media type (1=image, 2=video, 3=audio).
    async fn media_type(&self) -> i16 {
        self.inner.media_type
    }

    /// Address of the oracle that issued the badge.
    async fn issued_by(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.issued_by)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// When the badge was issued (epoch milliseconds).
    async fn issued_at(&self) -> i64 {
        self.inner.issued_at
    }

    /// Whether the badge has been revoked.
    async fn revoked(&self) -> bool {
        self.inner.revoked
    }

    async fn beneficiary_address(&self) -> Option<&str> {
        self.inner.beneficiary_address.as_deref()
    }

    async fn matched_anchor_id(&self) -> Option<&str> {
        self.inner.matched_anchor_id.as_deref()
    }

    async fn media_index(&self) -> Option<i16> {
        self.inner.media_index
    }

    /// Indexed beneficiary vault for this badge's `beneficiary_address`, when present and materialized.
    async fn poc_beneficiary_vault(&self, ctx: &Context<'_>) -> Option<PocBeneficiaryVault> {
        let addr = self.inner.beneficiary_address.as_deref()?;
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_poc_beneficiary_vault_by_beneficiary_address(addr)
            .await
            .ok()??;
        Some(PocBeneficiaryVault::from_row(row))
    }
}

#[derive(Clone)]
pub(crate) struct PocRevenueRedirection {
    inner: PocRevenueRedirectionRow,
}

impl PocRevenueRedirection {
    pub(crate) fn from_row(inner: PocRevenueRedirectionRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocRevenueRedirection {
    /// Redirection ID.
    async fn redirection_id(&self) -> &str {
        &self.inner.redirection_id
    }

    /// Post ID accused of being derivative (receiving redirected revenue).
    async fn accused_post_id(&self) -> &str {
        &self.inner.accused_post_id
    }

    /// Original post ID that receives the redirected revenue.
    async fn original_post_id(&self) -> &str {
        &self.inner.original_post_id
    }

    /// Percentage of revenue redirected to the original creator.
    async fn redirect_percentage(&self) -> i64 {
        self.inner.redirect_percentage
    }

    /// Similarity score from the analysis (0-100).
    async fn similarity_score(&self) -> i64 {
        self.inner.similarity_score
    }

    /// When the redirection was created (epoch milliseconds).
    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }
}

#[derive(Clone)]
pub(crate) struct PocAnalysisResult {
    inner: PocAnalysisResultRow,
}

impl PocAnalysisResult {
    pub(crate) fn from_row(inner: PocAnalysisResultRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocAnalysisResult {
    /// Post ID that was analyzed.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Whether similarity to existing content was detected.
    async fn similarity_detected(&self) -> bool {
        self.inner.similarity_detected
    }

    /// Highest similarity score from the analysis (0-100).
    async fn highest_similarity_score(&self) -> i64 {
        self.inner.highest_similarity_score
    }

    /// Media type analyzed (1=image, 2=video, 3=audio).
    async fn media_type(&self) -> i16 {
        self.inner.media_type
    }

    /// Address of the oracle that performed the analysis.
    async fn oracle_address(&self) -> &str {
        &self.inner.oracle_address
    }

    /// Address of the original creator if similarity was detected.
    async fn original_creator(&self) -> Option<&str> {
        self.inner.original_creator.as_deref()
    }

    /// Oracle reasoning captured on submission (indexed row — same provenance as `Post.pocReasoning`).
    async fn reasoning(&self) -> Option<&str> {
        self.inner.reasoning.as_deref()
    }

    /// Structured evidence URLs from analysis (indexed row — parallels `Post.pocEvidenceUrls`).
    async fn evidence_urls(&self) -> Option<Json> {
        self.inner
            .evidence_urls
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    /// When the analysis was performed (epoch milliseconds).
    async fn analysis_timestamp(&self) -> i64 {
        self.inner.analysis_timestamp
    }
}

#[derive(Clone)]
pub(crate) struct PocDispute {
    inner: PocDisputeRow,
}

impl PocDispute {
    pub(crate) fn from_row(inner: PocDisputeRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocDispute {
    /// Dispute ID.
    async fn dispute_id(&self) -> &str {
        &self.inner.dispute_id
    }

    /// Post ID being disputed.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Address of the disputer.
    async fn disputer(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.disputer)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the disputer.
    async fn disputer_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.disputer).await
    }

    /// Dispute type (1=challenge badge, 2=challenge redirection).
    async fn dispute_type(&self) -> i16 {
        self.inner.dispute_type
    }

    /// Round when opened (`1` or `2`).
    async fn dispute_round(&self) -> i16 {
        self.inner.dispute_round
    }

    /// Evidence submitted by the disputer.
    async fn evidence(&self) -> &str {
        &self.inner.evidence
    }

    /// Status (1=voting, 2=resolved upheld, 3=resolved overturned).
    async fn status(&self) -> i16 {
        self.inner.status
    }

    /// Resolution outcome code when resolved (from `PoCDisputeResolvedEvent`).
    async fn resolution(&self) -> Option<i32> {
        self.inner.resolution.map(i32::from)
    }

    /// Winning vote side when resolved (`VOTE_UPHOLD` / `VOTE_OVERTURN`).
    async fn winning_side(&self) -> Option<i32> {
        self.inner.winning_side.map(i32::from)
    }

    /// Aggregate stake on the winning side when resolved.
    async fn total_winning_stake(&self) -> Option<i64> {
        self.inner.total_winning_stake
    }

    /// Aggregate stake on the losing side when resolved.
    async fn total_losing_stake(&self) -> Option<i64> {
        self.inner.total_losing_stake
    }

    /// Stake amount (dispute fee) paid when opening the dispute.
    async fn stake_amount(&self) -> i64 {
        self.inner.stake_amount
    }

    /// Effective dispute fee charged for this round (MYSO base units).
    async fn effective_dispute_fee(&self) -> i64 {
        self.inner.effective_dispute_fee
    }

    /// Minimum total voting stake required for resolution when the dispute opened (MYSO base units).
    async fn required_total_stake_quorum(&self) -> i64 {
        self.inner.required_total_stake_quorum
    }

    /// Voting window start (milliseconds).
    async fn voting_start_ms(&self) -> i64 {
        self.inner.voting_start_ms
    }

    /// Voting window end (milliseconds).
    async fn voting_end_ms(&self) -> i64 {
        self.inner.voting_end_ms
    }

    /// When the dispute was submitted (epoch milliseconds).
    async fn submitted_at(&self) -> i64 {
        self.inner.submitted_at
    }

    /// When the dispute was resolved (epoch milliseconds), if resolved.
    async fn resolved_at(&self) -> Option<i64> {
        self.inner.resolved_at
    }

    /// Whether total vote stake met the quorum threshold when resolved; absent until resolved.
    async fn quorum_met(&self) -> Option<bool> {
        self.inner.quorum_met
    }

    /// Votes cast on this dispute (latest row per voter).
    async fn votes(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PocDisputeVote>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(200) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .get_poc_dispute_votes(&self.inner.dispute_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PocDisputeVote::from_row).collect())
    }
}

#[derive(Clone)]
pub(crate) struct PocDisputeVote {
    inner: PocDisputeVoteRow,
}

impl PocDisputeVote {
    pub(crate) fn from_row(inner: PocDisputeVoteRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocDisputeVote {
    async fn dispute_id(&self) -> &str {
        &self.inner.dispute_id
    }

    async fn voter(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.voter)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn voter_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.voter).await
    }

    /// Vote choice (`VOTE_UPHOLD` / `VOTE_OVERTURN`).
    async fn vote_choice(&self) -> i32 {
        i32::from(self.inner.vote_choice)
    }

    async fn stake_amount(&self) -> i64 {
        self.inner.stake_amount
    }

    async fn voted_at(&self) -> i64 {
        self.inner.voted_at
    }

    async fn reward_claimed(&self) -> Option<bool> {
        self.inner.reward_claimed
    }

    async fn reward_amount(&self) -> Option<i64> {
        self.inner.reward_amount
    }
}

#[derive(Clone)]
pub(crate) struct PocBeneficiaryVault {
    inner: PocBeneficiaryVaultRow,
}

impl PocBeneficiaryVault {
    pub(crate) fn from_row(inner: PocBeneficiaryVaultRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocBeneficiaryVault {
    /// On-chain vault object address.
    async fn vault_id(&self) -> &str {
        &self.inner.vault_id
    }

    /// Beneficiary wallet for this vault.
    async fn beneficiary(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.beneficiary_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Last indexer touch (epoch milliseconds).
    async fn updated_at_ms(&self) -> i64 {
        self.inner.updated_at_ms
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// Non-zero per-coin-type balances (excludes legacy aggregate sentinel).
    async fn coin_balances(&self, ctx: &Context<'_>) -> Option<Vec<PocVaultCoinBalance>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let rows = reader
            .list_poc_beneficiary_vault_coin_balances(&self.inner.vault_id)
            .await
            .ok()?;
        Some(
            rows.into_iter()
                .map(PocVaultCoinBalance::from_row)
                .collect(),
        )
    }

    async fn deposits(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PocVaultDeposit>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(200) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_poc_vault_deposits_for_vault(&self.inner.vault_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PocVaultDeposit::from_row).collect())
    }

    async fn claims(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<PocVaultClaim>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(200) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let rows = reader
            .list_poc_vault_claims_for_vault(&self.inner.vault_id, limit, offset)
            .await
            .ok()?;
        Some(rows.into_iter().map(PocVaultClaim::from_row).collect())
    }
}

#[derive(Clone)]
pub(crate) struct PocVaultCoinBalance {
    inner: PocVaultCoinBalanceRow,
}

impl PocVaultCoinBalance {
    fn from_row(inner: PocVaultCoinBalanceRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocVaultCoinBalance {
    async fn vault_id(&self) -> &str {
        &self.inner.vault_id
    }

    async fn coin_type(&self) -> &str {
        &self.inner.coin_type
    }

    async fn balance(&self) -> i64 {
        self.inner.balance
    }

    async fn updated_at_ms(&self) -> i64 {
        self.inner.updated_at_ms
    }
}

#[derive(Clone)]
pub(crate) struct PocVaultDeposit {
    inner: PocVaultDepositRow,
}

impl PocVaultDeposit {
    fn from_row(inner: PocVaultDepositRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocVaultDeposit {
    async fn id(&self) -> i64 {
        self.inner.id
    }

    async fn vault_id(&self) -> &str {
        &self.inner.vault_id
    }

    async fn beneficiary(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.beneficiary_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    async fn coin_type(&self) -> &str {
        &self.inner.coin_type
    }

    async fn source_post_id(&self) -> Option<&str> {
        self.inner.source_post_id.as_deref()
    }

    async fn occurred_at_ms(&self) -> i64 {
        self.inner.occurred_at_ms
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct PocVaultClaim {
    inner: PocVaultClaimRow,
}

impl PocVaultClaim {
    fn from_row(inner: PocVaultClaimRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocVaultClaim {
    async fn id(&self) -> i64 {
        self.inner.id
    }

    async fn vault_id(&self) -> &str {
        &self.inner.vault_id
    }

    async fn beneficiary(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.beneficiary_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn coin_type(&self) -> &str {
        &self.inner.coin_type
    }

    async fn referrer(&self) -> Option<MySoAddress> {
        self.inner.referrer_address.as_ref().map(|s| {
            MySoAddress::from_str(s)
                .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
        })
    }

    async fn treasury_amount(&self) -> i64 {
        self.inner.treasury_amount
    }

    async fn referrer_amount(&self) -> i64 {
        self.inner.referrer_amount
    }

    async fn beneficiary_amount(&self) -> i64 {
        self.inner.beneficiary_amount
    }

    async fn occurred_at_ms(&self) -> i64 {
        self.inner.occurred_at_ms
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}
