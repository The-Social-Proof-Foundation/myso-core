// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;

use myso_indexer_alt_social_reader::insurance::InsuranceConfigRow;
use myso_indexer_alt_social_reader::mydata::MyDataConfigRow;
use myso_indexer_alt_social_reader::post::PostConfigRow;
use myso_indexer_alt_social_reader::spot::SpotConfigRow;
use myso_indexer_alt_social_reader::spt::SptExchangeConfigRow;
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

    /// Post reservation threshold (basis points or amount).
    async fn post_threshold(&self) -> i64 {
        self.inner.post_threshold
    }

    /// Profile reservation threshold.
    async fn profile_threshold(&self) -> i64 {
        self.inner.profile_threshold
    }

    /// Max individual reservation in basis points.
    async fn max_individual_reservation_bps(&self) -> i64 {
        self.inner.max_individual_reservation_bps
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

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
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

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
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

    /// Oracle address used for verification.
    async fn oracle_address(&self) -> Option<&str> {
        self.inner.oracle_address.as_deref()
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn updated_at(&self) -> i64 {
        self.inner.updated_at
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
    async fn enable_flag(&self) -> bool {
        self.inner.enable_flag
    }

    /// Confidence threshold in basis points.
    async fn confidence_threshold_bps(&self) -> i64 {
        self.inner.confidence_threshold_bps
    }

    /// Resolution window in epochs.
    async fn resolution_window_epochs(&self) -> i64 {
        self.inner.resolution_window_epochs
    }

    /// Max resolution window in epochs.
    async fn max_resolution_window_epochs(&self) -> i64 {
        self.inner.max_resolution_window_epochs
    }

    /// Payout delay in milliseconds.
    async fn payout_delay_ms(&self) -> i64 {
        self.inner.payout_delay_ms
    }

    /// Fee in basis points.
    async fn fee_bps(&self) -> i64 {
        self.inner.fee_bps
    }

    /// Fee split to platform in basis points.
    async fn fee_split_bps_platform(&self) -> i64 {
        self.inner.fee_split_bps_platform
    }

    /// Oracle address for resolution.
    async fn oracle_address(&self) -> &str {
        &self.inner.oracle_address
    }

    /// Max single bet amount.
    async fn max_single_bet(&self) -> i64 {
        self.inner.max_single_bet
    }

    /// Configuration version.
    async fn version(&self) -> i64 {
        self.inner.version
    }

    /// Last updated timestamp (epoch milliseconds).
    async fn timestamp_ms(&self) -> i64 {
        self.inner.timestamp_ms
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

    /// Whether MyData marketplace is enabled.
    async fn enable_flag(&self) -> bool {
        self.inner.enable_flag
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

    /// Last updated timestamp (epoch milliseconds).
    async fn timestamp_ms(&self) -> i64 {
        self.inner.timestamp_ms
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
    async fn enable_flag(&self) -> bool {
        self.inner.enable_flag
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
    async fn timestamp_ms(&self) -> i64 {
        self.inner.timestamp_ms
    }
}
