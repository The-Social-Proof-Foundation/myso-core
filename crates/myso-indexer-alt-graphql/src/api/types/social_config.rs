// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;

use crate::api::scalars::date_time::DateTime;
use myso_indexer_alt_social_reader::ai_credit::AiCreditConfigRow;
use myso_indexer_alt_social_reader::insurance::{InsuranceConfigRow, InsuranceRouterConfigRow};
use myso_indexer_alt_social_reader::memory::MemoryConfigRow;
use myso_indexer_alt_social_reader::messaging::MessagingConfigRow;
use myso_indexer_alt_social_reader::mydata::MyDataConfigRow;
use myso_indexer_alt_social_reader::platform::PlatformConfigRow;
use myso_indexer_alt_social_reader::post::PostConfigRow;
use myso_indexer_alt_social_reader::profile::{EcosystemTreasuryRow, ProfileConfigRow};
use myso_indexer_alt_social_reader::spot::SpotConfigRow;
use myso_indexer_alt_social_reader::spt::SptExchangeConfigRow;
use myso_indexer_alt_social_reader::subscription::SubscriptionConfigRow;
use myso_indexer_alt_social_schema::models::PocConfigRow;

#[derive(Clone)]
pub(crate) struct SptExchangeConfig {
    inner: SptExchangeConfigRow,
}

impl SptExchangeConfig {
    pub(crate) fn from_row(inner: SptExchangeConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SptExchangeConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Total MYSO required to reserve toward a post before the pool can graduate (global config).
    async fn post_threshold(&self) -> i64 {
        self.inner.post_threshold
    }

    /// Total MYSO required to reserve toward a profile before the pool can graduate (global config).
    async fn profile_threshold(&self) -> i64 {
        self.inner.profile_threshold
    }

    /// Max share of the pool threshold any single wallet may reserve, in basis points (10000 = 100%).
    async fn max_individual_reservation_bps(&self) -> i64 {
        self.inner.max_individual_reservation_bps
    }

    /// Per-wallet reservation cap for **post** pools: `(postThreshold * maxIndividualReservationBps) / 10000` in MYSO base units (matches on-chain `social_proof_tokens`).
    async fn max_individual_reservation_amount_post(&self) -> i64 {
        (self.inner.post_threshold as i128 * self.inner.max_individual_reservation_bps as i128
            / 10_000) as i64
    }

    /// Per-wallet reservation cap for **profile** pools: `(profileThreshold * maxIndividualReservationBps) / 10000` in MYSO base units.
    async fn max_individual_reservation_amount_profile(&self) -> i64 {
        (self.inner.profile_threshold as i128 * self.inner.max_individual_reservation_bps as i128
            / 10_000) as i64
    }

    /// Total fee in basis points.
    async fn total_fee_bps(&self) -> i64 {
        self.inner.total_fee_bps
    }

    /// Creator fee in basis points.
    async fn creator_fee_bps(&self) -> i64 {
        self.inner.creator_fee_bps
    }

    /// Platform fee in basis points.
    async fn platform_fee_bps(&self) -> i64 {
        self.inner.platform_fee_bps
    }

    /// Treasury fee in basis points.
    async fn treasury_fee_bps(&self) -> i64 {
        self.inner.treasury_fee_bps
    }

    /// Trading creator fee in basis points.
    async fn trading_creator_fee_bps(&self) -> i64 {
        self.inner.trading_creator_fee_bps
    }

    /// Trading platform fee in basis points.
    async fn trading_platform_fee_bps(&self) -> i64 {
        self.inner.trading_platform_fee_bps
    }

    /// Trading treasury fee in basis points.
    async fn trading_treasury_fee_bps(&self) -> i64 {
        self.inner.trading_treasury_fee_bps
    }

    /// Reservation creator fee in basis points.
    async fn reservation_creator_fee_bps(&self) -> i64 {
        self.inner.reservation_creator_fee_bps
    }

    /// Reservation platform fee in basis points.
    async fn reservation_platform_fee_bps(&self) -> i64 {
        self.inner.reservation_platform_fee_bps
    }

    /// Reservation treasury fee in basis points.
    async fn reservation_treasury_fee_bps(&self) -> i64 {
        self.inner.reservation_treasury_fee_bps
    }

    /// Max reservers per pool.
    async fn max_reservers_per_pool(&self) -> i64 {
        self.inner.max_reservers_per_pool
    }

    /// Base price for new tokens.
    async fn base_price(&self) -> i64 {
        self.inner.base_price
    }

    /// Quadratic coefficient for pricing curve.
    async fn quadratic_coefficient(&self) -> i64 {
        self.inner.quadratic_coefficient
    }

    /// Max hold percentage in basis points.
    async fn max_hold_percent_bps(&self) -> i64 {
        self.inner.max_hold_percent_bps
    }

    /// Whether trading is enabled.
    async fn trading_enabled(&self) -> bool {
        self.inner.trading_enabled
    }

    /// Share of the non-platform platform fee routed to the creator, in basis points (10000 = 100%).
    async fn non_platform_platform_to_creator_bps(&self) -> i64 {
        self.inner.non_platform_platform_to_creator_bps
    }

    /// Share of the non-platform platform fee routed to the ecosystem treasury, in basis points (10000 = 100%).
    async fn non_platform_platform_to_treasury_bps(&self) -> i64 {
        self.inner.non_platform_platform_to_treasury_bps
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of last update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct PostConfig {
    inner: PostConfigRow,
}

impl PostConfig {
    pub(crate) fn from_row(inner: PostConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PostConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Maximum content length in characters.
    async fn max_content_length(&self) -> i64 {
        self.inner.max_content_length
    }

    /// Maximum media URLs per post.
    async fn max_media_urls(&self) -> i64 {
        self.inner.max_media_urls
    }

    /// Maximum mentions per post.
    async fn max_mentions(&self) -> i64 {
        self.inner.max_mentions
    }

    /// Maximum metadata size in bytes.
    async fn max_metadata_size(&self) -> i64 {
        self.inner.max_metadata_size
    }

    /// Maximum report description length.
    async fn max_description_length(&self) -> i64 {
        self.inner.max_description_length
    }

    /// Maximum reaction text length.
    async fn max_reaction_length(&self) -> i64 {
        self.inner.max_reaction_length
    }

    /// Commenter tip percentage (remainder to post owner).
    async fn commenter_tip_percentage(&self) -> i64 {
        self.inner.commenter_tip_percentage
    }

    /// Repost tip percentage (remainder to original post owner).
    async fn repost_tip_percentage(&self) -> i64 {
        self.inner.repost_tip_percentage
    }

    /// Minimum promotion amount a post owner must deposit to start a promoted post campaign (MYSO base units).
    async fn min_promotion_amount(&self) -> i64 {
        self.inner.min_promotion_amount
    }

    /// Maximum promotion amount a post owner may deposit for a single promoted post campaign (MYSO base units).
    async fn max_promotion_amount(&self) -> i64 {
        self.inner.max_promotion_amount
    }

    /// Minimum view duration (ms) required for a promoted-post view to count toward the campaign.
    async fn min_view_duration_ms(&self) -> i64 {
        self.inner.min_view_duration_ms
    }

    /// Platform fee bps taken from each confirmed promo view gross.
    async fn platform_fee_bps(&self) -> i64 {
        self.inner.platform_fee_bps
    }

    /// Ecosystem fee bps taken from each confirmed promo view gross.
    async fn ecosystem_fee_bps(&self) -> i64 {
        self.inner.ecosystem_fee_bps
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct PocConfig {
    inner: PocConfigRow,
}

impl PocConfig {
    pub(crate) fn from_row(inner: PocConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocConfig {
    /// Similarity threshold for image content (0-100).
    async fn image_threshold(&self) -> i64 {
        self.inner.image_threshold
    }

    /// Similarity threshold for video content (0-100).
    async fn video_threshold(&self) -> i64 {
        self.inner.video_threshold
    }

    /// Similarity threshold for audio content (0-100).
    async fn audio_threshold(&self) -> i64 {
        self.inner.audio_threshold
    }

    /// Revenue redirect percentage when similarity detected.
    async fn revenue_redirect_percentage(&self) -> i64 {
        self.inner.revenue_redirect_percentage
    }

    /// Cost to submit a dispute.
    async fn dispute_cost(&self) -> i64 {
        self.inner.dispute_cost
    }

    /// Minimum aggregate voting stake (`uphold` + `overturn`) required for a full resolution on round 1; `0` disables the quorum gate.
    async fn dispute_quorum_base_stake(&self) -> i64 {
        self.inner.dispute_quorum_base_stake
    }

    /// Fee multiplier (basis points) applied only when opening the second dispute; must be at least `10000`.
    async fn dispute_second_round_fee_multiplier_bps(&self) -> i64 {
        self.inner.dispute_second_round_fee_multiplier_bps
    }

    /// Quorum multiplier (basis points) applied to `dispute_quorum_base_stake` only on round 2.
    async fn dispute_second_round_quorum_multiplier_bps(&self) -> i64 {
        self.inner.dispute_second_round_quorum_multiplier_bps
    }

    /// Minimum stake per vote on a dispute (MYSO base units).
    async fn min_vote_stake(&self) -> i64 {
        self.inner.min_vote_stake
    }

    /// Maximum stake per vote on a dispute (MYSO base units).
    async fn max_vote_stake(&self) -> i64 {
        self.inner.max_vote_stake
    }

    /// Voting period for disputes (milliseconds).
    async fn voting_duration_ms(&self) -> i64 {
        self.inner.voting_duration_ms
    }

    /// Max length of reasoning text allowed in PoC flows.
    async fn max_reasoning_length(&self) -> i64 {
        self.inner.max_reasoning_length
    }

    /// Max evidence URLs per submission.
    async fn max_evidence_urls(&self) -> i64 {
        self.inner.max_evidence_urls
    }

    /// Max votes allowed per dispute.
    async fn max_votes_per_dispute(&self) -> i64 {
        self.inner.max_votes_per_dispute
    }

    /// Oracle address used for verification.
    async fn oracle_address(&self) -> Option<&str> {
        self.inner.oracle_address.as_deref()
    }

    async fn claim_treasury_fee_bps(&self) -> i64 {
        self.inner.claim_treasury_fee_bps
    }

    async fn max_referral_bps(&self) -> i64 {
        self.inner.max_referral_bps
    }

    /// Max redirect ceiling (bps) when only embedded audio matches on a VIDEO post (on-chain delta ramp applies).
    async fn video_embedded_audio_redirect_bps(&self) -> i64 {
        self.inner.video_embedded_audio_redirect_bps
    }

    /// One-time join-referral fee (bps) on first username-beneficiary vault claim.
    async fn username_beneficiary_join_referral_bps(&self) -> i64 {
        self.inner.username_beneficiary_join_referral_bps
    }

    /// Maximum number of disputes allowed per post.
    async fn max_disputes_per_post(&self) -> i64 {
        self.inner.max_disputes_per_post.into()
    }

    /// Minimum vault deposit amount required to submit a dispute (MYSO base units).
    async fn min_vault_deposit_amount(&self) -> i64 {
        self.inner.min_vault_deposit_amount
    }

    /// Treasury fee for initiating a media-asset rights governance dispute (MYSO base units).
    async fn media_asset_dispute_cost(&self) -> i64 {
        self.inner.media_asset_dispute_cost
    }

    /// Lifetime cap on rights governance disputes per media asset.
    async fn max_disputes_per_media_asset(&self) -> i64 {
        self.inner.max_disputes_per_media_asset.into()
    }

    /// Max bps any embedded source asset may redirect from a post revenue pool (0-10000).
    async fn max_embedded_asset_redirect_bps(&self) -> i64 {
        self.inner.max_embedded_asset_redirect_bps
    }

    /// Shared PoC GovernanceDAO object ID (registry_type = 1).
    async fn dispute_governance_registry_id(&self) -> Option<&str> {
        self.inner.dispute_governance_registry_id.as_deref()
    }

    /// Address that last updated PoC configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }
}

#[derive(Clone)]
pub(crate) struct SpotConfig {
    inner: SpotConfigRow,
}

impl SpotConfig {
    pub(crate) fn from_row(inner: SpotConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpotConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Whether SPoT (Social Proof of Truth) is enabled.
    async fn truth_enabled(&self) -> bool {
        self.inner.truth_enabled
    }

    /// Confidence threshold in basis points.
    async fn confidence_threshold_bps(&self) -> i64 {
        self.inner.confidence_threshold_bps
    }

    /// Resolution window duration in milliseconds (global default).
    async fn resolution_window_ms(&self) -> i64 {
        self.inner.resolution_window_ms
    }

    /// Max resolution window duration in milliseconds.
    async fn max_resolution_window_ms(&self) -> i64 {
        self.inner.max_resolution_window_ms
    }

    /// Payout delay in milliseconds.
    async fn payout_delay_ms(&self) -> i64 {
        self.inner.payout_delay_ms
    }

    /// Platform fee as a direct percentage of gross bet amount, in basis points (10000 = 100%).
    async fn platform_fee_bps(&self) -> i64 {
        self.inner.platform_fee_bps
    }

    /// Ecosystem treasury fee as a direct percentage of gross bet amount, in basis points (10000 = 100%).
    async fn ecosystem_fee_bps(&self) -> i64 {
        self.inner.ecosystem_fee_bps
    }

    /// Creator referral fee in basis points (default 100 = 1.00%).
    async fn creator_fee_bps(&self) -> i64 {
        self.inner.creator_fee_bps
    }

    /// Window after resolution during which creators may claim referral rewards.
    async fn creator_claim_window_ms(&self) -> i64 {
        self.inner.creator_claim_window_ms
    }

    /// Share of expired unclaimed creator rewards routed to ecosystem on reclaim.
    async fn expired_creator_ecosystem_bps(&self) -> i64 {
        self.inner.expired_creator_ecosystem_bps
    }

    /// Oracle address for resolution.
    async fn oracle_address(&self) -> &str {
        &self.inner.oracle_address
    }

    /// Max single bet amount.
    async fn max_single_bet(&self) -> i64 {
        self.inner.max_single_bet
    }

    /// Maximum bets allowed per SPoT record.
    async fn max_bets_per_record(&self) -> i64 {
        self.inner.max_bets_per_record
    }

    /// Maximum claims per post at finalize time (Move range 1–20).
    async fn max_claim_per_post(&self) -> i64 {
        self.inner.max_claim_per_post
    }

    /// Shared SPoT GovernanceDAO object ID (registry_type = 2).
    async fn spot_governance_registry_id(&self) -> Option<&str> {
        self.inner.spot_governance_registry_id.as_deref()
    }

    /// Minimum number of betting options required on a SPoT record.
    async fn min_betting_options(&self) -> i64 {
        self.inner.min_betting_options
    }

    /// Maximum number of betting options allowed on a SPoT record.
    async fn max_betting_options(&self) -> i64 {
        self.inner.max_betting_options
    }

    /// Minimum reasoning text length required when creating a SPoT record.
    async fn min_reasoning_length(&self) -> i64 {
        self.inner.min_reasoning_length
    }

    /// Maximum reasoning text length allowed when creating a SPoT record.
    async fn max_reasoning_length(&self) -> i64 {
        self.inner.max_reasoning_length
    }

    /// Maximum number of evidence URLs allowed on a SPoT record.
    async fn max_evidence_urls(&self) -> i64 {
        self.inner.max_evidence_urls
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct MyDataConfig {
    inner: MyDataConfigRow,
}

impl MyDataConfig {
    pub(crate) fn from_row(inner: MyDataConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Whether buyers may start new broad-pool/snapshot MyData marketplace rounds.
    /// Direct profile-gated, one-time, and recurring MyData access is always available.
    async fn marketplace_enabled(&self) -> bool {
        self.inner.marketplace_enabled
    }

    /// Maximum tags per record.
    async fn max_tags(&self) -> i64 {
        self.inner.max_tags
    }

    /// Maximum subscription duration in days.
    async fn max_subscription_days(&self) -> i64 {
        self.inner.max_subscription_days
    }

    /// Maximum free access grants.
    async fn max_free_access_grants(&self) -> i64 {
        self.inner.max_free_access_grants
    }

    /// Maximum encryption id byte length accepted when creating a MyData record.
    async fn max_encryption_id_bytes(&self) -> i64 {
        self.inner.max_encryption_id_bytes
    }

    async fn max_encrypted_data_bytes(&self) -> i64 {
        self.inner.max_encrypted_data_bytes
    }
    async fn max_tag_bytes(&self) -> i64 {
        self.inner.max_tag_bytes
    }
    async fn max_metadata_bytes(&self) -> i64 {
        self.inner.max_metadata_bytes
    }
    async fn max_payment_reference_bytes(&self) -> i64 {
        self.inner.max_payment_reference_bytes
    }
    async fn max_pool_assignments(&self) -> i64 {
        self.inner.max_pool_assignments
    }
    async fn max_merkle_proof_depth(&self) -> i64 {
        self.inner.max_merkle_proof_depth
    }
    async fn max_paid_access_entries(&self) -> i64 {
        self.inner.max_paid_access_entries
    }
    async fn default_claim_window_ms(&self) -> i64 {
        self.inner.default_claim_window_ms
    }

    /// P2P marketplace platform fee in bps (default 250 = 2.5%).
    async fn p2p_platform_fee_bps(&self) -> i64 {
        self.inner.p2p_platform_fee_bps
    }

    /// P2P marketplace ecosystem fee in bps (default 250 = 2.5%).
    async fn p2p_ecosystem_fee_bps(&self) -> i64 {
        self.inner.p2p_ecosystem_fee_bps
    }

    /// MyData marketplace pool claim platform fee in bps (default 250 = 2.5%).
    async fn mydata_marketplace_platform_fee_bps(&self) -> i64 {
        self.inner.mydata_marketplace_platform_fee_bps
    }

    /// MyData marketplace pool claim ecosystem fee in bps (default 250 = 2.5%).
    async fn mydata_marketplace_ecosystem_fee_bps(&self) -> i64 {
        self.inner.mydata_marketplace_ecosystem_fee_bps
    }

    /// When no platform is present, share of the platform fee bucket routed to creators (bps).
    async fn non_platform_platform_to_creator_bps(&self) -> i64 {
        self.inner.non_platform_platform_to_creator_bps
    }

    /// When no platform is present, share of the platform fee bucket routed to ecosystem treasury (bps).
    async fn non_platform_platform_to_treasury_bps(&self) -> i64 {
        self.inner.non_platform_platform_to_treasury_bps
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct InsuranceConfig {
    inner: InsuranceConfigRow,
}

impl InsuranceConfig {
    pub(crate) fn from_row(inner: InsuranceConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Whether insurance is enabled.
    async fn insurance_enabled(&self) -> bool {
        self.inner.insurance_enabled
    }

    /// Minimum coverage in basis points.
    async fn min_coverage_bps(&self) -> i64 {
        self.inner.min_coverage_bps
    }

    /// Maximum coverage in basis points.
    async fn max_coverage_bps(&self) -> i64 {
        self.inner.max_coverage_bps
    }

    /// Maximum policy duration in milliseconds.
    async fn max_duration_ms(&self) -> i64 {
        self.inner.max_duration_ms
    }

    /// Fee in basis points.
    async fn fee_bps(&self) -> i64 {
        self.inner.fee_bps
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    /// Minimum SPoT pool total liquidity required for pricing.
    async fn min_spot_total_liquidity(&self) -> i64 {
        self.inner.min_spot_total_liquidity
    }

    /// Maximum coverage as a fraction of the insured option, in basis points.
    async fn max_coverage_fraction_of_option_bps(&self) -> i64 {
        self.inner.max_coverage_fraction_of_option_bps
    }

    /// Maximum combined risk multiplier cap, in basis points.
    async fn max_risk_multiplier_bps(&self) -> i64 {
        self.inner.max_risk_multiplier_bps
    }

    /// Minimum premium amount charged per policy.
    async fn min_premium_amount(&self) -> i64 {
        self.inner.min_premium_amount
    }

    /// SPoT smoothing applied per option for implied probability.
    async fn spot_smoothing_per_option(&self) -> i64 {
        self.inner.spot_smoothing_per_option
    }

    /// Implied probability floor, in basis points.
    async fn implied_prob_floor_bps(&self) -> i64 {
        self.inner.implied_prob_floor_bps
    }

    /// Whether to enforce a 1x odds floor.
    async fn odds_floor_1x(&self) -> bool {
        self.inner.odds_floor_1x
    }

    /// Odds multiplier cap, in basis points.
    async fn odds_cap_bps(&self) -> i64 {
        self.inner.odds_cap_bps
    }

    /// Liquidity multiplier cap, in basis points.
    async fn liq_cap_bps(&self) -> i64 {
        self.inner.liq_cap_bps
    }

    /// Reference pool size for the liquidity multiplier.
    async fn liq_ref_amount(&self) -> i64 {
        self.inner.liq_ref_amount
    }

    /// Exposure multiplier cap, in basis points.
    async fn exposure_cap_bps(&self) -> i64 {
        self.inner.exposure_cap_bps
    }

    /// Exposure curve parameter K, in basis points.
    async fn exposure_k_bps(&self) -> i64 {
        self.inner.exposure_k_bps
    }

    /// Base odds multiplier in basis points applied in `compute_spot_risk_quote` (replaces the prior hardcoded 5000 bps).
    async fn odds_base_bps(&self) -> i64 {
        self.inner.odds_base_bps
    }
}

#[derive(Clone)]
pub(crate) struct AiCreditConfig {
    inner: AiCreditConfigRow,
}

impl AiCreditConfig {
    pub(crate) fn from_row(inner: AiCreditConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl AiCreditConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Hex-encoded ed25519 public key the AI credit oracle uses to sign usage receipts.
    async fn oracle_pubkey_hex(&self) -> &str {
        &self.inner.oracle_pubkey_hex
    }

    /// Treasury address that receives AI credit fees.
    async fn treasury_address(&self) -> &str {
        &self.inner.treasury_address
    }

    /// Minimum deposit (in MYSO base units) required to open an AI credit balance.
    async fn min_deposit_mist(&self) -> i64 {
        self.inner.min_deposit_mist
    }

    /// Maximum single settlement amount (in MYSO base units) permitted per usage receipt.
    async fn max_single_settlement_mist(&self) -> i64 {
        self.inner.max_single_settlement_mist
    }

    /// Time-to-live (ms) for a usage receipt before it is considered stale.
    async fn receipt_ttl_ms(&self) -> i64 {
        self.inner.receipt_ttl_ms
    }

    /// Markup in basis points applied on top of oracle AI credit pricing (10000 = 100%).
    async fn oracle_markup_bps(&self) -> i64 {
        self.inner.oracle_markup_bps
    }

    /// Catalog version label carried by the AI credit pricing catalog, when set.
    async fn catalog_version(&self) -> Option<&str> {
        self.inner.catalog_version.as_deref()
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct MessagingConfig {
    inner: MessagingConfigRow,
}

impl MessagingConfig {
    pub(crate) fn from_row(inner: MessagingConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MessagingConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Platform fee in basis points charged on paid messages (10000 = 100%).
    async fn paid_msg_platform_fee_bps(&self) -> i64 {
        self.inner.paid_msg_platform_fee_bps
    }

    /// Ecosystem treasury fee in basis points charged on paid messages (10000 = 100%).
    async fn paid_msg_treasury_fee_bps(&self) -> i64 {
        self.inner.paid_msg_treasury_fee_bps
    }

    /// Payment expiration window in milliseconds for paid-message escrow claims.
    async fn payment_expiration_ms(&self) -> i64 {
        self.inner.payment_expiration_ms
    }

    /// Minimum character count required for a paid-message reply.
    async fn min_reply_chars(&self) -> i64 {
        self.inner.min_reply_chars
    }

    /// Maximum byte length accepted for a paid-message dedupe key.
    async fn max_dedupe_key_bytes(&self) -> i64 {
        self.inner.max_dedupe_key_bytes
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct SubscriptionConfig {
    inner: SubscriptionConfigRow,
}

impl SubscriptionConfig {
    pub(crate) fn from_row(inner: SubscriptionConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SubscriptionConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Billing period duration in milliseconds.
    async fn default_billing_period_ms(&self) -> i64 {
        self.inner.default_billing_period_ms
    }

    /// Maximum number of renewal months permitted per subscription.
    async fn max_renewal_months(&self) -> i64 {
        self.inner.max_renewal_months
    }

    /// Platform fee in bps deducted from gross subscription payments (default 250 = 2.5%).
    async fn platform_fee_bps(&self) -> i64 {
        self.inner.platform_fee_bps
    }

    /// Ecosystem treasury fee in bps deducted from gross subscription payments (default 250 = 2.5%).
    async fn ecosystem_fee_bps(&self) -> i64 {
        self.inner.ecosystem_fee_bps
    }

    /// When no platform is present, share of the platform fee bucket routed to creators (bps).
    async fn non_platform_platform_to_creator_bps(&self) -> i64 {
        self.inner.non_platform_platform_to_creator_bps
    }

    /// When no platform is present, share of the platform fee bucket routed to ecosystem treasury (bps).
    async fn non_platform_platform_to_treasury_bps(&self) -> i64 {
        self.inner.non_platform_platform_to_treasury_bps
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct ProfileConfig {
    inner: ProfileConfigRow,
}

impl ProfileConfig {
    pub(crate) fn from_row(inner: ProfileConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ProfileConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Maximum number of vesting pieces allowed on a profile vesting schedule.
    async fn max_vesting_pieces(&self) -> i64 {
        self.inner.max_vesting_pieces
    }

    /// Minimum curve factor accepted for profile vesting curves.
    async fn curve_factor_min(&self) -> i64 {
        self.inner.curve_factor_min
    }

    /// Maximum curve factor accepted for profile vesting curves.
    async fn curve_factor_max(&self) -> i64 {
        self.inner.curve_factor_max
    }

    /// Precision divisor used when interpreting curve factors.
    async fn curve_precision(&self) -> i64 {
        self.inner.curve_precision
    }

    /// Minimum divisor accepted for the claim threshold on profile vesting.
    async fn min_claim_threshold_divisor(&self) -> i64 {
        self.inner.min_claim_threshold_divisor
    }

    /// Minimum username length enforced at profile creation.
    async fn min_username_length(&self) -> i64 {
        self.inner.min_username_length
    }

    /// Maximum username length enforced at profile creation.
    async fn max_username_length(&self) -> i64 {
        self.inner.max_username_length
    }

    /// Fee in basis points taken on username marketplace sales (10000 = 100%).
    async fn username_sale_fee_bps(&self) -> i64 {
        self.inner.username_sale_fee_bps
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct MemoryConfig {
    inner: MemoryConfigRow,
}

impl MemoryConfig {
    pub(crate) fn from_row(inner: MemoryConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MemoryConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Maximum number of agentic organizations a single user may own.
    async fn max_organizations_per_user(&self) -> i64 {
        self.inner.max_organizations_per_user.into()
    }

    /// Cooldown (ms) between organization category updates.
    async fn org_category_update_cooldown_ms(&self) -> i64 {
        self.inner.org_category_update_cooldown_ms
    }

    /// Maximum agent nesting depth allowed in a memory hierarchy.
    async fn max_agent_depth(&self) -> i64 {
        self.inner.max_agent_depth.into()
    }

    /// Maximum byte length accepted for a memory label.
    async fn max_label_length(&self) -> i64 {
        self.inner.max_label_length
    }

    /// Maximum byte length accepted for an organization name.
    async fn max_org_name_length(&self) -> i64 {
        self.inner.max_org_name_length
    }

    /// Maximum byte length accepted for an organization description.
    async fn max_org_description_length(&self) -> i64 {
        self.inner.max_org_description_length
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct PlatformConfig {
    inner: PlatformConfigRow,
}

impl PlatformConfig {
    pub(crate) fn from_row(inner: PlatformConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PlatformConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Maximum reasoning text length accepted for platform-level submissions.
    async fn max_reasoning_length(&self) -> i64 {
        self.inner.max_reasoning_length
    }

    /// Maximum byte length accepted for a platform cover photo URL.
    async fn max_cover_photo_url_length(&self) -> i64 {
        self.inner.max_cover_photo_url_length
    }

    /// Maximum number of media previews allowed on a platform.
    async fn max_media_previews(&self) -> i64 {
        self.inner.max_media_previews
    }

    /// Maximum byte length accepted for a platform badge name.
    async fn max_badge_name_length(&self) -> i64 {
        self.inner.max_badge_name_length
    }

    /// Maximum byte length accepted for a platform badge description.
    async fn max_badge_description_length(&self) -> i64 {
        self.inner.max_badge_description_length
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct EcosystemTreasury {
    inner: EcosystemTreasuryRow,
}

impl EcosystemTreasury {
    pub(crate) fn from_row(inner: EcosystemTreasuryRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl EcosystemTreasury {
    /// Treasury address that receives ecosystem fees (e.g. profile sale fees).
    async fn treasury_address(&self) -> &str {
        &self.inner.treasury_address
    }

    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct InsuranceRouterConfig {
    inner: InsuranceRouterConfigRow,
}

impl InsuranceRouterConfig {
    pub(crate) fn from_row(inner: InsuranceRouterConfigRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl InsuranceRouterConfig {
    /// Address that last updated the configuration.
    async fn updated_by(&self) -> &str {
        &self.inner.updated_by
    }

    /// Whether the coverage router is paused.
    async fn paused(&self) -> bool {
        self.inner.paused
    }

    /// Maximum route reserve that may be locked across a single market.
    async fn max_route_reserve_market(&self) -> i64 {
        self.inner.max_route_reserve_market
    }

    /// Maximum route reserve that may be locked for a single user across routes.
    async fn max_route_reserve_user(&self) -> i64 {
        self.inner.max_route_reserve_user
    }

    /// Maximum route reserve that may be locked against a single SPoT option.
    async fn max_route_reserve_option(&self) -> i64 {
        self.inner.max_route_reserve_option
    }

    /// Maximum vault concentration allowed, in basis points (10000 = 100%).
    async fn max_vault_concentration_bps(&self) -> i64 {
        self.inner.max_vault_concentration_bps
    }

    /// Minimum vault health factor required, in basis points (10000 = 100%).
    async fn min_vault_health_factor_bps(&self) -> i64 {
        self.inner.min_vault_health_factor_bps
    }

    /// Maximum number of legs permitted in a single coverage route (enforced at runtime).
    async fn max_route_legs(&self) -> i64 {
        self.inner.max_route_legs
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
    }

    /// When the configuration was last updated.
    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }

    /// Transaction ID of the last config update.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

/// Unified GraphQL view over the three on-chain insurance configuration objects
/// (`InsuranceConfig` pricing, `InsuranceRouterConfig` router limits). The underlying
/// objects remain separate on-chain and in the indexer; this type only aggregates them
/// for read convenience. `pricing` or `router` may be `null` independently when one side
/// has not yet been indexed.
#[derive(Clone)]
pub(crate) struct InsuranceConfiguration {
    pub(crate) pricing: Option<InsuranceConfig>,
    pub(crate) router: Option<InsuranceRouterConfig>,
}

impl InsuranceConfiguration {
    pub(crate) fn new(
        pricing: Option<InsuranceConfig>,
        router: Option<InsuranceRouterConfig>,
    ) -> Self {
        Self { pricing, router }
    }
}

#[Object]
impl InsuranceConfiguration {
    /// Insurance pricing config (risk pricing, odds, exposure, fee bps).
    async fn pricing(&self) -> Option<&InsuranceConfig> {
        self.pricing.as_ref()
    }

    /// Insurance router config (reserve limits, vault health, max route legs).
    async fn router(&self) -> Option<&InsuranceRouterConfig> {
        self.router.as_ref()
    }
}
