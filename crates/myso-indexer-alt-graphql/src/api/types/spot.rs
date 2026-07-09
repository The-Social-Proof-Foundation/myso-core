// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    SocialPgReader, SpotBetRow, SpotBetWithdrawalRow, SpotClaimEarningsRow, SpotClaimRow,
    SpotCreatorStatsRow, SpotMarketEarningsRow, SpotMarketRow, SpotPayoutRow, SpotPendingCreatorPayoutRow,
    SpotPostEarningsRow, SpotRecordRow, SpotRefundRow, SpotResolutionRow, SpotRouteRow,
};

use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::governance::Proposal;
use crate::api::types::profile_summary::ProfileSummary;

fn parse_betting_options(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_option_escrow(value: &serde_json::Value) -> Vec<(i16, i64)> {
    value
        .as_object()
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| {
                    let option_id: i16 = k.parse().ok()?;
                    let amount: i64 = v.as_i64()?;
                    Some((option_id, amount))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn total_escrow_from_option_escrow(value: &serde_json::Value) -> i64 {
    parse_option_escrow(value)
        .into_iter()
        .map(|(_, amt)| amt)
        .sum()
}

#[derive(Clone)]
pub(crate) struct SpotBet {
    inner: SpotBetRow,
}

impl SpotBet {
    pub(crate) fn from_row(inner: SpotBetRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotBet {
    /// Unique bet identifier.
    async fn bet_id(&self) -> String {
        self.inner.id.to_string()
    }

    /// Post ID this bet is for.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Address of the bettor.
    async fn better(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.user_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the bettor.
    async fn profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.user_address).await
    }

    /// Total amount staked (escrow + AMM).
    async fn amount(&self) -> i64 {
        self.inner.escrow_amount + self.inner.amm_amount
    }

    /// Option ID the bettor chose (outcome bet on).
    async fn outcome(&self) -> i16 {
        self.inner.option_id
    }

    /// Human-readable option label (e.g. "Yes", "No").
    async fn option_label(&self) -> Option<&str> {
        self.inner.option_label.as_deref()
    }

    /// Unix milliseconds when the bet was placed (`Clock` on-chain when available).
    async fn placed_at(&self) -> i64 {
        self.inner.timestamp_ms
    }

    /// Escrow amount.
    async fn escrow_amount(&self) -> i64 {
        self.inner.escrow_amount
    }

    /// AMM amount.
    async fn amm_amount(&self) -> i64 {
        self.inner.amm_amount
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct SpotRecord {
    inner: SpotRecordRow,
}

impl SpotRecord {
    pub(crate) fn from_row(inner: SpotRecordRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotRecord {
    /// Unique record identifier.
    async fn record_id(&self) -> String {
        self.inner.id.to_string()
    }

    /// Post ID this record is for.
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    /// Resolved outcome (when resolved).
    async fn resolution(&self) -> Option<i16> {
        self.inner.outcome
    }

    /// Record status (1=open, 2=dao_required, 3=resolved, 4=refundable).
    async fn status(&self) -> i16 {
        self.inner.status
    }

    /// Betting option labels (e.g. ["Yes", "No"]).
    async fn betting_options(&self) -> Vec<String> {
        parse_betting_options(&self.inner.betting_options)
    }

    /// Escrow amount per option (option_id -> amount).
    async fn option_escrow(&self) -> Vec<SpotOptionEscrow> {
        parse_option_escrow(&self.inner.option_escrow)
            .into_iter()
            .map(|(option_id, amount)| SpotOptionEscrow { option_id, amount })
            .collect()
    }

    /// Unix milliseconds when the record was created (`Clock`).
    async fn created_at_ms(&self) -> i64 {
        self.inner.created_at_ms
    }

    /// Resolution window duration in milliseconds.
    async fn resolution_window_ms(&self) -> Option<i64> {
        self.inner.resolution_window_ms
    }

    /// Max resolution window duration in milliseconds.
    async fn max_resolution_window_ms(&self) -> Option<i64> {
        self.inner.max_resolution_window_ms
    }

    /// Last resolution instant (Unix ms), if resolved.
    async fn last_resolution_at_ms(&self) -> Option<i64> {
        self.inner.last_resolution_at_ms
    }

    /// Total escrow across all options.
    async fn total_escrow(&self) -> i64 {
        total_escrow_from_option_escrow(&self.inner.option_escrow)
    }

    /// Transaction ID of the record creation/update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// On-chain SpotRecord object id.
    async fn record_object_id(&self) -> Option<&str> {
        self.inner.record_object_id.as_deref()
    }

    /// Active SPoT governance proposal id while debate is open.
    async fn active_proposal_id(&self) -> Option<&str> {
        self.inner.active_proposal_id.as_deref()
    }

    /// Oracle-suggested outcome when escalated to DAO_REQUIRED.
    async fn oracle_proposed_outcome(&self) -> Option<i16> {
        self.inner.oracle_proposed_outcome
    }

    /// Outcome under community ratification in the active proposal.
    async fn proposed_outcome(&self) -> Option<i16> {
        self.inner.proposed_outcome
    }

    /// Wall-clock ms when oracle escalated to DAO_REQUIRED.
    async fn dao_escalated_at_ms(&self) -> Option<i64> {
        self.inner.dao_escalated_at_ms
    }

    /// Linked governance proposal (when active_proposal_id is set).
    async fn proposal(&self, ctx: &Context<'_>) -> Option<Proposal> {
        let proposal_id = self.inner.active_proposal_id.as_deref()?;
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_proposal_by_id(proposal_id)
            .await
            .ok()?
            .map(Proposal::from_row)
    }
}

#[derive(Clone)]
pub(crate) struct SpotPayout {
    inner: SpotPayoutRow,
}

impl SpotPayout {
    pub(crate) fn from_row(inner: SpotPayoutRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotPayout {
    /// Address of the payout recipient.
    async fn recipient(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.user_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the recipient.
    async fn recipient_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.user_address).await
    }

    /// Payout amount.
    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    /// Unix milliseconds when the payout was recorded (checkpoint / chain time).
    async fn paid_at(&self) -> i64 {
        self.inner.timestamp_ms
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct SpotRefund {
    inner: SpotRefundRow,
}

impl SpotRefund {
    pub(crate) fn from_row(inner: SpotRefundRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotRefund {
    /// Address of the refund recipient.
    async fn recipient(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.user_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the recipient.
    async fn recipient_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.user_address).await
    }

    /// Refund amount.
    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    /// Unix milliseconds when the refund was recorded (checkpoint / chain time).
    async fn refunded_at(&self) -> i64 {
        self.inner.timestamp_ms
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

fn parse_evidence_urls(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

#[derive(Clone)]
pub(crate) struct SpotResolution {
    inner: SpotResolutionRow,
}

impl SpotResolution {
    pub(crate) fn from_row(inner: SpotResolutionRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotResolution {
    /// Resolved outcome (winning option_id).
    async fn outcome(&self) -> i16 {
        self.inner.outcome
    }

    /// Total escrow at resolution.
    async fn total_escrow(&self) -> i64 {
        self.inner.total_escrow
    }

    /// Fee taken at resolution.
    async fn fee_taken(&self) -> i64 {
        self.inner.fee_taken
    }

    /// Unix milliseconds when the record was resolved (checkpoint / chain time).
    async fn resolved_at_ms(&self) -> i64 {
        self.inner.resolved_at_ms
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// Oracle reasoning for the resolution.
    async fn reasoning(&self) -> &str {
        &self.inner.reasoning
    }

    /// Evidence URLs (required for resolution).
    async fn evidence_urls(&self) -> Vec<String> {
        parse_evidence_urls(&self.inner.evidence_urls)
    }
}

#[derive(Clone)]
pub(crate) struct SpotBetWithdrawal {
    inner: SpotBetWithdrawalRow,
}

impl SpotBetWithdrawal {
    pub(crate) fn from_row(inner: SpotBetWithdrawalRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotBetWithdrawal {
    /// Address of the withdrawer.
    async fn withdrawer(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.user_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Profile of the withdrawer.
    async fn withdrawer_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        resolve_profile_summary(ctx, &self.inner.user_address).await
    }

    /// Option ID the bet was on.
    async fn option_id(&self) -> i16 {
        self.inner.option_id
    }

    /// Withdrawn amount.
    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    /// Fee taken on withdrawal.
    async fn fee_taken(&self) -> i64 {
        self.inner.fee_taken
    }

    /// Unix milliseconds when the withdrawal was recorded (checkpoint / chain time).
    async fn withdrawn_at(&self) -> i64 {
        self.inner.timestamp_ms
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct SpotOptionEscrow {
    option_id: i16,
    amount: i64,
}

#[Object]
impl SpotOptionEscrow {
    /// Option ID.
    async fn option_id(&self) -> i16 {
        self.option_id
    }

    /// Escrow amount for this option.
    async fn amount(&self) -> i64 {
        self.amount
    }
}

#[derive(Clone)]
pub(crate) struct SpotClaim {
    inner: SpotClaimRow,
}

impl SpotClaim {
    pub(crate) fn from_row(inner: SpotClaimRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotClaim {
    async fn claim_id(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.claim_object_id)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn semantic_claim_hash(&self) -> Option<&str> {
        self.inner.semantic_claim_hash.as_deref()
    }

    async fn created_at_ms(&self) -> Option<i64> {
        self.inner.created_at_ms
    }
}

#[derive(Clone)]
pub(crate) struct SpotMarket {
    inner: SpotMarketRow,
}

impl SpotMarket {
    pub(crate) fn from_row(inner: SpotMarketRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotMarket {
    async fn market_id(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.market_object_id)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn claim_id(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.claim_object_id)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn market_key_hash(&self) -> Option<&str> {
        self.inner.market_key_hash.as_deref()
    }

    async fn primary_post_id(&self) -> Option<&str> {
        self.inner.primary_post_id.as_deref()
    }

    async fn status(&self) -> i16 {
        self.inner.status
    }

    async fn deadline_ms(&self) -> Option<i64> {
        self.inner.deadline_ms
    }

    async fn betting_options(&self) -> Vec<String> {
        parse_betting_options(&self.inner.betting_options)
    }

    async fn creator_fee_total(&self) -> Option<i64> {
        self.inner.creator_fee_total
    }

    async fn winner_pool(&self) -> Option<i64> {
        self.inner.winner_pool
    }

    async fn resolution_timestamp_ms(&self) -> Option<i64> {
        self.inner.resolution_timestamp_ms
    }

    async fn created_at_ms(&self) -> Option<i64> {
        self.inner.created_at_ms
    }

    async fn claim(&self, ctx: &Context<'_>) -> Option<SpotClaim> {
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_spot_claim(&self.inner.claim_object_id)
            .await
            .ok()?
            .map(SpotClaim::from_row)
    }
}

#[derive(Clone)]
pub(crate) struct SpotRoute {
    inner: SpotRouteRow,
}

impl SpotRoute {
    pub(crate) fn from_row(inner: SpotRouteRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotRoute {
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    async fn claim_id(&self) -> Option<MySoAddress> {
        self.inner.claim_object_id.as_ref().and_then(|id| {
            MySoAddress::from_str(id)
                .ok()
                .map(MySoAddress::from)
        })
    }

    async fn target_market_id(&self) -> Option<MySoAddress> {
        self.inner.target_market_id.as_ref().and_then(|id| {
            MySoAddress::from_str(id)
                .ok()
                .map(MySoAddress::from)
        })
    }

    async fn link_kind(&self) -> Option<&str> {
        self.inner.link_kind.as_deref()
    }

    async fn routing_reason(&self) -> &str {
        &self.inner.routing_reason
    }

    async fn spot_claim(&self, ctx: &Context<'_>) -> Option<SpotClaim> {
        let claim_id = self.inner.claim_object_id.as_deref()?;
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader.get_spot_claim(claim_id).await.ok()?.map(SpotClaim::from_row)
    }

    async fn spot_market(&self, ctx: &Context<'_>) -> Option<SpotMarket> {
        let market_id = self.inner.target_market_id.as_deref()?;
        let reader_opt = ctx.data_opt::<Arc<Option<SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_spot_market(market_id)
            .await
            .ok()?
            .map(SpotMarket::from_row)
    }
}

#[derive(Clone)]
pub(crate) struct SpotPendingCreatorPayout {
    inner: SpotPendingCreatorPayoutRow,
}

impl SpotPendingCreatorPayout {
    pub(crate) fn from_row(inner: SpotPendingCreatorPayoutRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotPendingCreatorPayout {
    async fn payout_id(&self) -> i64 {
        self.inner.payout_id
    }

    async fn market_id(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.market_object_id)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn creator(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.creator)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn referrer_post_id(&self) -> &str {
        &self.inner.referrer_post_id
    }

    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    async fn expires_at_ms(&self) -> i64 {
        self.inner.expires_at_ms
    }

    async fn is_expired(&self) -> bool {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        now_ms >= self.inner.expires_at_ms
    }

    async fn is_claimable(&self) -> bool {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        now_ms < self.inner.expires_at_ms
    }
}

#[derive(Clone)]
pub(crate) struct SpotClaimEarnings {
    inner: SpotClaimEarningsRow,
}

impl SpotClaimEarnings {
    pub(crate) fn from_row(inner: SpotClaimEarningsRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotClaimEarnings {
    async fn claim_id(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.claim_object_id)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn total_amount(&self) -> i64 {
        self.inner.total_amount
    }
}

#[derive(Clone)]
pub(crate) struct SpotPostEarnings {
    inner: SpotPostEarningsRow,
}

impl SpotPostEarnings {
    pub(crate) fn from_row(inner: SpotPostEarningsRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotPostEarnings {
    async fn referrer_post_id(&self) -> &str {
        &self.inner.referrer_post_id
    }

    async fn total_amount(&self) -> i64 {
        self.inner.total_amount
    }
}

#[derive(Clone)]
pub(crate) struct SpotMarketEarnings {
    inner: SpotMarketEarningsRow,
}

impl SpotMarketEarnings {
    pub(crate) fn from_row(inner: SpotMarketEarningsRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotMarketEarnings {
    async fn market_id(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.market_object_id)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn total_amount(&self) -> i64 {
        self.inner.total_amount
    }
}

#[derive(Clone)]
pub(crate) struct SpotCreatorStats {
    inner: SpotCreatorStatsRow,
    top_claims: Vec<SpotClaimEarningsRow>,
    earnings_by_post: Vec<SpotPostEarningsRow>,
    earnings_by_market: Vec<SpotMarketEarningsRow>,
}

impl SpotCreatorStats {
    pub(crate) fn from_parts(
        inner: SpotCreatorStatsRow,
        top_claims: Vec<SpotClaimEarningsRow>,
        earnings_by_post: Vec<SpotPostEarningsRow>,
        earnings_by_market: Vec<SpotMarketEarningsRow>,
    ) -> Self {
        Self {
            inner,
            top_claims,
            earnings_by_post,
            earnings_by_market,
        }
    }
}

#[Object]
impl SpotCreatorStats {
    async fn creator(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.creator)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn lifetime_earnings(&self) -> i64 {
        self.inner.lifetime_earnings
    }

    async fn earnings_last30d(&self) -> i64 {
        self.inner.earnings_last_30d
    }

    async fn pending_earnings(&self) -> i64 {
        self.inner.pending_earnings
    }

    async fn top_claims(&self) -> Vec<SpotClaimEarnings> {
        self.top_claims
            .iter()
            .cloned()
            .map(SpotClaimEarnings::from_row)
            .collect()
    }

    async fn earnings_by_post(&self) -> Vec<SpotPostEarnings> {
        self.earnings_by_post
            .iter()
            .cloned()
            .map(SpotPostEarnings::from_row)
            .collect()
    }

    async fn earnings_by_market(&self) -> Vec<SpotMarketEarnings> {
        self.earnings_by_market
            .iter()
            .cloned()
            .map(SpotMarketEarnings::from_row)
            .collect()
    }
}
