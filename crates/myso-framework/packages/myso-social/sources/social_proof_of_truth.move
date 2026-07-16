// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Social Proof of Truth (SPoT)
/// Claim → Market → Post architecture. Semantic claims dedupe off-chain content; markets
/// hold escrow and resolution state; posts link to claims for attribution and creator fees.
/// Oracle/DAO resolves outcomes; winners and creators claim payouts after resolution.

#[allow(duplicate_alias, unused_use, unused_const, unused_variable, lint(self_transfer, share_owned, public_entry))]
module social_contracts::social_proof_of_truth {
    use std::bcs;
    use std::option::{Self, Option};
    use std::string::{Self, String};
    use std::vector;

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        event,
        coin::{Self, Coin},
        balance::{Self, Balance},
        table::{Self, Table},
        clock::{Self, Clock},
    };
    use myso::myso::MYSO;

    use social_contracts::post::{Self, Post};
    use social_contracts::platform::{Self, Platform};
    use social_contracts::profile::{Self, EcosystemTreasury};
    use social_contracts::block_list::BlockListRegistry;
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::governance::{Self, GovernanceDAO, Proposal};

    /// Errors
    const EDisabled: u64 = 1;
    const EInvalidAmount: u64 = 2;
    const EAlreadyResolved: u64 = 3;
    const ETooEarly: u64 = 4;
    const ETooClose: u64 = 5;
    const EWrongStatus: u64 = 6;
    const ENotOracle: u64 = 7;
    const ENoBets: u64 = 8;
    const EOverflow: u64 = 9;
    const EInvalidReasoning: u64 = 10;
    const EInvalidOptionId: u64 = 11;
    const EWithdrawalNotAllowed: u64 = 12;
    const EBetNotFound: u64 = 13;
    const EAlreadyInitialized: u64 = 14;
    const EDuplicateOption: u64 = 15;
    const ETooManyBets: u64 = 16;
    const EWrongVersion: u64 = 17;
    const EActiveProposalExists: u64 = 18;
    const ENoActiveProposal: u64 = 19;
    const EWrongProposal: u64 = 20;
    const ENotDaoRequired: u64 = 21;
    const EDaoDebateFrozen: u64 = 22;
    const EInvalidGovernanceRegistry: u64 = 23;
    const EProposalNotApproved: u64 = 24;
    const EClaimExists: u64 = 25;
    const EClaimNotFound: u64 = 26;
    const EMarketExists: u64 = 27;
    const EMarketNotOpen: u64 = 28;
    const EPostNotLinked: u64 = 29;
    const EPayoutNotFound: u64 = 30;
    const ENotCreator: u64 = 31;
    const ECreatorPayoutExpired: u64 = 32;
    const EInvalidHash: u64 = 33;
    const ENotFinalized: u64 = 34;
    const EPastVerdictMismatch: u64 = 35;
    const ENoOpenMarket: u64 = 36;

    /// Status
    const STATUS_OPEN: u8 = 1;
    const STATUS_DAO_REQUIRED: u8 = 2;
    const STATUS_RESOLVED: u8 = 3;
    const STATUS_REFUNDABLE: u8 = 4;

    /// Outcomes
    const OUTCOME_DRAW: u8 = 255;
    const OUTCOME_UNAPPLICABLE: u8 = 254;

    const MS_PER_DAY: u64 = 86400000;

    const DEFAULT_CONFIDENCE_THRESHOLD_BPS: u64 = 7000;
    const DEFAULT_ENABLE: bool = false;
    const DEFAULT_RESOLUTION_WINDOW_MS: u64 = 72 * MS_PER_DAY;
    const DEFAULT_MAX_RESOLUTION_WINDOW_MS: u64 = 144 * MS_PER_DAY;
    const DEFAULT_PAYOUT_DELAY_MS: u64 = 0;
    const DEFAULT_PLATFORM_FEE_BPS: u64 = 50;
    const DEFAULT_ECOSYSTEM_FEE_BPS: u64 = 50;
    const DEFAULT_CREATOR_FEE_BPS: u64 = 100;
    const DEFAULT_CREATOR_CLAIM_WINDOW_MS: u64 = 30 * MS_PER_DAY;
    const DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS: u64 = 10000;
    const DEFAULT_MIN_BETTING_OPTIONS: u64 = 2;
    const DEFAULT_MAX_BETTING_OPTIONS: u64 = 10;
    const DEFAULT_MIN_REASONING_LENGTH: u64 = 10;
    const DEFAULT_MAX_REASONING_LENGTH: u64 = 5000;
    const DEFAULT_MAX_EVIDENCE_URLS: u64 = 10;
    const DEFAULT_MAX_BETS_PER_RECORD: u64 = 10000;
    const DEFAULT_MAX_CLAIM_PER_POST: u64 = 10;
    const MIN_MAX_CLAIM_PER_POST: u64 = 1;
    const MAX_MAX_CLAIM_PER_POST: u64 = 20;

    /// Past-claim verdict values (mirror indexer/GraphQL): 1=true, 2=false, 3=unverifiable.
    const VERDICT_TRUE: u8 = 1;
    const VERDICT_FALSE: u8 = 2;
    const VERDICT_UNVERIFIABLE: u8 = 3;

    const MAX_U64: u64 = 18446744073709551615;
    const MIN_HASH_LEN: u64 = 8;

    public struct SpotAdminCap has key, store { id: UID }
    public struct SpotOracleAdminCap has key, store { id: UID }

    public struct SpotConfig has key {
        id: UID,
        truth_enabled: bool,
        confidence_threshold_bps: u64,
        resolution_window_ms: u64,
        max_resolution_window_ms: u64,
        payout_delay_ms: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        creator_fee_bps: u64,
        creator_claim_window_ms: u64,
        expired_creator_ecosystem_bps: u64,
        min_betting_options: u64,
        max_betting_options: u64,
        min_reasoning_length: u64,
        max_reasoning_length: u64,
        max_evidence_urls: u64,
        oracle_address: address,
        max_single_bet: u64,
        max_bets_per_record: u64,
        max_claim_per_post: u64,
        spot_governance_registry_id: ID,
        version: u64,
    }

    /// Post linked to a semantic claim at a specific claim index (creator stored for fee routing).
    public struct SpotPostLink has store, copy, drop {
        post_id: address,
        creator: address,
        claim_index: u64,
    }

    /// Registry key: a post's future-claim link at a given claim index.
    public struct PostClaimIndexKey has copy, drop, store {
        post_id: address,
        claim_index: u64,
    }

    /// Registry key: a (post, market) future-link — authoritative bet-eligibility check.
    public struct PostMarketKey has copy, drop, store {
        post_id: address,
        market_id: address,
    }

    /// Semantic claim object — deduped by `semantic_claim_hash`.
    public struct SpotClaim has key {
        id: UID,
        semantic_claim_hash: vector<u8>,
        created_at_ms: u64,
        linked_posts: vector<SpotPostLink>,
        version: u64,
    }

    /// Shared registry mapping hashes and open markets. Multi-claim: a post may hold
    /// several future-claim links keyed by claim index / market.
    public struct SpotClaimRegistry has key {
        id: UID,
        claims_by_semantic_hash: Table<vector<u8>, address>,
        markets_by_key_hash: Table<vector<u8>, address>,
        open_market_by_claim: Table<address, address>,
        post_claim_index_to_market: Table<PostClaimIndexKey, address>,
        post_market_to_claim: Table<PostMarketKey, address>,
        version: u64,
    }

    /// Pending creator payout (O(1) claim by `payout_id`).
    public struct SpotCreatorPayout has store, copy, drop {
        creator: address,
        source_post_id: address,
        amount: u64,
        expires_at_ms: u64,
    }

    /// Prediction market for a claim (evolved from per-post SpotRecord).
    public struct SpotMarket has key {
        id: UID,
        claim_id: address,
        market_key_hash: vector<u8>,
        primary_post_id: address,
        primary_creator: address,
        created_at_ms: u64,
        status: u8,
        outcome: Option<u8>,
        escrow: Balance<MYSO>,
        betting_options: vector<String>,
        option_escrow: Table<u8, u64>,
        user_option_amounts: Table<address, vector<u64>>,
        bets: vector<SpotBet>,
        resolution_window_ms: Option<u64>,
        max_resolution_window_ms: Option<u64>,
        resolution_at_ms: u64,
        last_resolution_at_ms: u64,
        resolution_timestamp_ms: u64,
        pending_payouts: Table<address, u64>,
        pending_creator_payouts: Table<u64, SpotCreatorPayout>,
        next_creator_payout_id: u64,
        creator_payout_index: Table<address, vector<u64>>,
        active_proposal_id: Option<ID>,
        oracle_proposed_outcome: Option<u8>,
        proposed_outcome: Option<u8>,
        dao_escalated_at_ms: u64,
        version: u64,
    }

    /// A single bet
    public struct SpotBet has store, copy, drop {
        user: address,
        option_id: u8,
        amount: u64,
        timestamp_ms: u64,
        referrer_post_id: Option<address>,
    }

    public struct SpotBetPlacedEvent has copy, drop {
        post_id: address,
        market_id: address,
        user: address,
        option_id: u8,
        amount: u64,
        timestamp_ms: u64,
        referrer_post_id: Option<address>,
    }

    public struct SpotResolvedEvent has copy, drop {
        post_id: address,
        market_id: address,
        claim_id: address,
        outcome: u8,
        total_escrow: u64,
        fee_taken: u64,
        creator_fee_total: u64,
        reasoning: String,
        evidence_urls: vector<String>,
    }

    public struct SpotDaoRequiredEvent has copy, drop {
        post_id: address,
        spot_record_id: address,
        confidence_bps: u64,
        oracle_proposed_outcome: u8,
        dao_escalated_at_ms: u64,
        reasoning: String,
    }

    public struct SpotGovernanceProposalLinkedEvent has copy, drop {
        post_id: address,
        spot_record_id: address,
        proposal_id: ID,
        proposed_outcome: u8,
    }

    public struct SpotGovernanceProposalClearedEvent has copy, drop {
        post_id: address,
        spot_record_id: address,
        proposal_id: ID,
    }

    public struct SpotPayoutEvent has copy, drop {
        post_id: address,
        user: address,
        amount: u64,
    }

    public struct SpotCreatorPayoutAccruedEvent has copy, drop {
        market_id: address,
        payout_id: u64,
        creator: address,
        referrer_post_id: address,
        amount: u64,
        expires_at_ms: u64,
    }

    public struct SpotCreatorPayoutClaimedEvent has copy, drop {
        market_id: address,
        payout_id: u64,
        creator: address,
        amount: u64,
    }

    public struct SpotCreatorPayoutReclaimedEvent has copy, drop {
        market_id: address,
        payout_id: u64,
        ecosystem_amount: u64,
        platform_amount: u64,
    }

    public struct SpotRefundEvent has copy, drop {
        post_id: address,
        user: address,
        amount: u64,
    }

    public struct SpotConfigUpdatedEvent has copy, drop {
        updated_by: address,
        truth_enabled: bool,
        confidence_threshold_bps: u64,
        resolution_window_ms: u64,
        max_resolution_window_ms: u64,
        payout_delay_ms: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        creator_fee_bps: u64,
        creator_claim_window_ms: u64,
        expired_creator_ecosystem_bps: u64,
        min_betting_options: u64,
        max_betting_options: u64,
        min_reasoning_length: u64,
        max_reasoning_length: u64,
        max_evidence_urls: u64,
        oracle_address: address,
        max_single_bet: u64,
        max_bets_per_record: u64,
        max_claim_per_post: u64,
        spot_governance_registry_id: ID,
        timestamp: u64,
    }

    public struct SpotBetWithdrawnEvent has copy, drop {
        post_id: address,
        user: address,
        option_id: u8,
        amount: u64,
        fee_taken: u64,
    }

    public struct SpotClaimCreatedEvent has copy, drop {
        claim_id: address,
        semantic_claim_hash: vector<u8>,
        created_at_ms: u64,
    }

    public struct SpotMarketCreatedEvent has copy, drop {
        market_id: address,
        claim_id: address,
        market_key_hash: vector<u8>,
        primary_post_id: address,
        claim_index: u64,
        resolution_policy_hash: vector<u8>,
        created_at_ms: u64,
        betting_options: vector<String>,
        resolution_at_ms: u64,
        max_resolution_window_ms: Option<u64>,
    }

    public struct SpotPostLinkedEvent has copy, drop {
        post_id: address,
        claim_id: address,
        market_id: Option<address>,
        claim_index: u64,
        policy_hash: vector<u8>,
    }

    /// Batch finalize projection for a post's multi-claim analysis. Carries future-link
    /// arrays (claim_index order) plus parallel past-verdict vectors for the indexer.
    public struct SpotClaimsFinalizedForPost has copy, drop {
        post_id: address,
        status: u8,
        detected_claim_count: u64,
        rejected_claim_count: u64,
        truncated_claim_count: u64,
        future_accepted_count: u64,
        past_verified_count: u64,
        max_claim_per_post_applied: u64,
        claim_manifest_hash: Option<vector<u8>>,
        veracity_manifest_hash: Option<vector<u8>>,
        future_claim_indexes: vector<u64>,
        future_claim_ids: vector<address>,
        future_market_ids: vector<address>,
        past_claim_indexes: vector<u64>,
        past_verdicts: vector<u8>,
        past_related_market_ids: vector<address>,
        past_evidence_hashes: vector<vector<u8>>,
        finalized_at_ms: u64,
    }

    // --- Getters (SpotMarket) ---
    public fun get_status(market: &SpotMarket): u8 { market.status }
    public fun get_bets_len(market: &SpotMarket): u64 { vector::length(&market.bets) }
    public fun get_betting_options(market: &SpotMarket): vector<String> { market.betting_options }
    public fun get_option_escrow(market: &SpotMarket, option_id: u8): u64 {
        if (table::contains(&market.option_escrow, option_id)) {
            *table::borrow(&market.option_escrow, option_id)
        } else { 0 }
    }
    public fun get_id_address(market: &SpotMarket): address {
        object::uid_to_address(&market.id)
    }
    public fun get_outcome(market: &SpotMarket): &Option<u8> { &market.outcome }
    public fun is_open(market: &SpotMarket): bool { market.status == STATUS_OPEN }
    public fun is_resolved(market: &SpotMarket): bool { market.status == STATUS_RESOLVED }
    public fun outcome_draw(): u8 { OUTCOME_DRAW }
    public fun outcome_unapplicable(): u8 { OUTCOME_UNAPPLICABLE }
    public fun get_user_option_amount(market: &SpotMarket, user: address, option_id: u8): u64 {
        if (!table::contains(&market.user_option_amounts, user)) {
            0
        } else {
            let amounts = table::borrow(&market.user_option_amounts, user);
            let idx = option_id as u64;
            if (idx >= vector::length(amounts)) { 0 } else { *vector::borrow(amounts, idx) }
        }
    }
    public fun num_betting_options(market: &SpotMarket): u64 {
        vector::length(&market.betting_options)
    }
    public fun total_option_escrow(market: &SpotMarket): u64 {
        let mut total = 0;
        let mut i = 0;
        let n = vector::length(&market.betting_options);
        while (i < n) {
            let option_id = (i as u8);
            let amt = get_option_escrow(market, option_id);
            assert!(total <= MAX_U64 - amt, EOverflow);
            total = total + amt;
            i = i + 1;
        };
        total
    }
    public fun assert_valid_option_id(market: &SpotMarket, option_id: u8) {
        assert!((option_id as u64) < vector::length(&market.betting_options), EInvalidOptionId);
    }
    public fun claim_id(market: &SpotMarket): address { market.claim_id }
    public fun market_key_hash(market: &SpotMarket): vector<u8> { market.market_key_hash }
    public fun primary_post_id(market: &SpotMarket): address { market.primary_post_id }

    public fun is_enabled(config: &SpotConfig): bool { config.truth_enabled }
    public fun max_claim_per_post(config: &SpotConfig): u64 { config.max_claim_per_post }
    public fun spot_governance_registry_id(config: &SpotConfig): ID {
        config.spot_governance_registry_id
    }
    public fun active_proposal_id(market: &SpotMarket): &Option<ID> {
        &market.active_proposal_id
    }
    public fun proposed_outcome(market: &SpotMarket): &Option<u8> {
        &market.proposed_outcome
    }
    public fun oracle_proposed_outcome(market: &SpotMarket): &Option<u8> {
        &market.oracle_proposed_outcome
    }
    public fun dao_escalated_at_ms(market: &SpotMarket): u64 {
        market.dao_escalated_at_ms
    }
    public fun semantic_claim_hash(claim: &SpotClaim): vector<u8> {
        claim.semantic_claim_hash
    }

    fun assert_valid_hash(hash: &vector<u8>) {
        assert!(vector::length(hash) >= MIN_HASH_LEN, EInvalidHash);
    }

    fun new_spot_config(
        spot_governance_registry_id: ID,
        ctx: &mut TxContext,
    ): SpotConfig {
        SpotConfig {
            id: object::new(ctx),
            truth_enabled: DEFAULT_ENABLE,
            confidence_threshold_bps: DEFAULT_CONFIDENCE_THRESHOLD_BPS,
            resolution_window_ms: DEFAULT_RESOLUTION_WINDOW_MS,
            max_resolution_window_ms: DEFAULT_MAX_RESOLUTION_WINDOW_MS,
            payout_delay_ms: DEFAULT_PAYOUT_DELAY_MS,
            platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
            ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
            creator_fee_bps: DEFAULT_CREATOR_FEE_BPS,
            creator_claim_window_ms: DEFAULT_CREATOR_CLAIM_WINDOW_MS,
            expired_creator_ecosystem_bps: DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS,
            min_betting_options: DEFAULT_MIN_BETTING_OPTIONS,
            max_betting_options: DEFAULT_MAX_BETTING_OPTIONS,
            min_reasoning_length: DEFAULT_MIN_REASONING_LENGTH,
            max_reasoning_length: DEFAULT_MAX_REASONING_LENGTH,
            max_evidence_urls: DEFAULT_MAX_EVIDENCE_URLS,
            oracle_address: tx_context::sender(ctx),
            max_single_bet: 0,
            max_bets_per_record: DEFAULT_MAX_BETS_PER_RECORD,
            max_claim_per_post: DEFAULT_MAX_CLAIM_PER_POST,
            spot_governance_registry_id,
            version: upgrade::current_version(),
        }
    }

    fun new_spot_claim_registry(ctx: &mut TxContext): SpotClaimRegistry {
        SpotClaimRegistry {
            id: object::new(ctx),
            claims_by_semantic_hash: table::new(ctx),
            markets_by_key_hash: table::new(ctx),
            open_market_by_claim: table::new(ctx),
            post_claim_index_to_market: table::new(ctx),
            post_market_to_claim: table::new(ctx),
            version: upgrade::current_version(),
        }
    }

    fun emit_config_updated(config: &SpotConfig, clock: &Clock, ctx: &TxContext) {
        event::emit(SpotConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            truth_enabled: config.truth_enabled,
            confidence_threshold_bps: config.confidence_threshold_bps,
            resolution_window_ms: config.resolution_window_ms,
            max_resolution_window_ms: config.max_resolution_window_ms,
            payout_delay_ms: config.payout_delay_ms,
            platform_fee_bps: config.platform_fee_bps,
            ecosystem_fee_bps: config.ecosystem_fee_bps,
            creator_fee_bps: config.creator_fee_bps,
            creator_claim_window_ms: config.creator_claim_window_ms,
            expired_creator_ecosystem_bps: config.expired_creator_ecosystem_bps,
            min_betting_options: config.min_betting_options,
            max_betting_options: config.max_betting_options,
            min_reasoning_length: config.min_reasoning_length,
            max_reasoning_length: config.max_reasoning_length,
            max_evidence_urls: config.max_evidence_urls,
            oracle_address: config.oracle_address,
            max_single_bet: config.max_single_bet,
            max_bets_per_record: config.max_bets_per_record,
            max_claim_per_post: config.max_claim_per_post,
            spot_governance_registry_id: config.spot_governance_registry_id,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    public(package) fun bootstrap_init(
        clock: &Clock,
        spot_governance_registry_id: ID,
        ctx: &mut TxContext
    ) {
        let config = new_spot_config(spot_governance_registry_id, ctx);
        emit_config_updated(&config, clock, ctx);
        transfer::share_object(config);
        transfer::share_object(new_spot_claim_registry(ctx));
    }

    public(package) fun create_spot_admin_cap(ctx: &mut TxContext): SpotAdminCap {
        SpotAdminCap { id: object::new(ctx) }
    }

    public(package) fun create_spot_oracle_admin_cap(ctx: &mut TxContext): SpotOracleAdminCap {
        SpotOracleAdminCap { id: object::new(ctx) }
    }

    #[test_only]
    public fun test_init(clock: &Clock, spot_governance_registry_id: ID, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        transfer::share_object(new_spot_config(spot_governance_registry_id, ctx));
        transfer::share_object(new_spot_claim_registry(ctx));
        transfer::public_transfer(SpotAdminCap { id: object::new(ctx) }, sender);
        transfer::public_transfer(SpotOracleAdminCap { id: object::new(ctx) }, sender);
    }

    fun assert_config_version(config: &SpotConfig) {
        assert!(config.version == upgrade::current_version(), EWrongVersion);
    }

    fun assert_registry_version(registry: &SpotClaimRegistry) {
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
    }

    fun assert_claim_version(claim: &SpotClaim) {
        assert!(claim.version == upgrade::current_version(), EWrongVersion);
    }

    fun assert_market_version(market: &SpotMarket) {
        assert!(market.version == upgrade::current_version(), EWrongVersion);
    }

    public entry fun update_spot_config(
        _: &SpotAdminCap,
        config: &mut SpotConfig,
        truth_enabled: bool,
        confidence_threshold_bps: u64,
        resolution_window_ms: u64,
        max_resolution_window_ms: u64,
        payout_delay_ms: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        creator_fee_bps: u64,
        creator_claim_window_ms: u64,
        expired_creator_ecosystem_bps: u64,
        min_betting_options: u64,
        max_betting_options: u64,
        min_reasoning_length: u64,
        max_reasoning_length: u64,
        max_evidence_urls: u64,
        oracle_address: address,
        max_single_bet: u64,
        max_bets_per_record: u64,
        max_claim_per_post: u64,
        spot_governance_registry_id: ID,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(config);
        assert!(confidence_threshold_bps <= 10000, EInvalidAmount);
        assert!(max_claim_per_post >= MIN_MAX_CLAIM_PER_POST, EInvalidAmount);
        assert!(max_claim_per_post <= MAX_MAX_CLAIM_PER_POST, EInvalidAmount);
        assert!(platform_fee_bps <= 10000, EInvalidAmount);
        assert!(ecosystem_fee_bps <= 10000, EInvalidAmount);
        assert!(creator_fee_bps <= 10000, EInvalidAmount);
        assert!(expired_creator_ecosystem_bps <= 10000, EInvalidAmount);
        assert!(platform_fee_bps + ecosystem_fee_bps + creator_fee_bps <= 10000, EInvalidAmount);
        assert!(min_betting_options > 0, EInvalidAmount);
        assert!(min_betting_options <= max_betting_options, EInvalidAmount);
        assert!(min_reasoning_length > 0, EInvalidReasoning);
        assert!(min_reasoning_length <= max_reasoning_length, EInvalidReasoning);
        assert!(max_evidence_urls > 0, EInvalidAmount);

        config.truth_enabled = truth_enabled;
        config.confidence_threshold_bps = confidence_threshold_bps;
        config.resolution_window_ms = resolution_window_ms;
        config.max_resolution_window_ms = max_resolution_window_ms;
        config.payout_delay_ms = payout_delay_ms;
        config.platform_fee_bps = platform_fee_bps;
        config.ecosystem_fee_bps = ecosystem_fee_bps;
        config.creator_fee_bps = creator_fee_bps;
        config.creator_claim_window_ms = creator_claim_window_ms;
        config.expired_creator_ecosystem_bps = expired_creator_ecosystem_bps;
        config.min_betting_options = min_betting_options;
        config.max_betting_options = max_betting_options;
        config.min_reasoning_length = min_reasoning_length;
        config.max_reasoning_length = max_reasoning_length;
        config.max_evidence_urls = max_evidence_urls;
        config.oracle_address = oracle_address;
        config.max_single_bet = max_single_bet;
        config.max_bets_per_record = max_bets_per_record;
        config.max_claim_per_post = max_claim_per_post;
        config.spot_governance_registry_id = spot_governance_registry_id;

        emit_config_updated(config, clock, ctx);
    }

    public entry fun rescale_spot_config_windows_from_epoch_counts(
        _: &SpotAdminCap,
        config: &mut SpotConfig,
        epoch_duration_ms: u64,
    ) {
        assert_config_version(config);
        assert!(epoch_duration_ms > 0, EInvalidAmount);
        config.resolution_window_ms = config.resolution_window_ms * epoch_duration_ms;
        config.max_resolution_window_ms = config.max_resolution_window_ms * epoch_duration_ms;
        config.creator_claim_window_ms = config.creator_claim_window_ms * epoch_duration_ms;
    }

    fun register_spot_claim(
        registry: &mut SpotClaimRegistry,
        semantic_claim_hash: vector<u8>,
        created_at_ms: u64,
        ctx: &mut TxContext,
    ): SpotClaim {
        assert_valid_hash(&semantic_claim_hash);
        assert!(!table::contains(&registry.claims_by_semantic_hash, semantic_claim_hash), EClaimExists);
        let claim = SpotClaim {
            id: object::new(ctx),
            semantic_claim_hash,
            created_at_ms,
            linked_posts: vector::empty(),
            version: upgrade::current_version(),
        };
        let claim_id = object::uid_to_address(&claim.id);
        table::add(&mut registry.claims_by_semantic_hash, semantic_claim_hash, claim_id);
        claim
    }

    /// Oracle-only: register a semantic claim (deduped by hash).
    public entry fun create_spot_claim(
        _: &SpotOracleAdminCap,
        config: &SpotConfig,
        registry: &mut SpotClaimRegistry,
        semantic_claim_hash: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(config);
        assert_registry_version(registry);
        assert!(config.truth_enabled, EDisabled);
        let created_at_ms = clock::timestamp_ms(clock);
        let claim = register_spot_claim(registry, semantic_claim_hash, created_at_ms, ctx);
        let claim_id = object::uid_to_address(&claim.id);
        let hash_copy = claim.semantic_claim_hash;
        transfer::share_object(claim);
        event::emit(SpotClaimCreatedEvent {
            claim_id,
            semantic_claim_hash: hash_copy,
            created_at_ms,
        });
    }

    /// Record a future-claim link: registers `(post, claim_index)` and `(post, market)`,
    /// pushes a `SpotPostLink`, and appends to the post's pending analysis vectors.
    fun register_future_link(
        registry: &mut SpotClaimRegistry,
        claim: &mut SpotClaim,
        post: &mut Post,
        market_id: address,
        claim_index: u64,
        resolution_policy_hash: vector<u8>,
        max_claim_per_post: u64,
    ) {
        let post_id = post::get_id_address(post);
        let claim_id = object::uid_to_address(&claim.id);
        let idx_key = PostClaimIndexKey { post_id, claim_index };
        assert!(!table::contains(&registry.post_claim_index_to_market, idx_key), EClaimExists);
        let creator = post::get_post_owner(post);
        vector::push_back(&mut claim.linked_posts, SpotPostLink { post_id, creator, claim_index });
        table::add(&mut registry.post_claim_index_to_market, idx_key, market_id);
        table::add(&mut registry.post_market_to_claim, PostMarketKey { post_id, market_id }, claim_id);
        post::ensure_spot_analysis_pending(post, max_claim_per_post);
        post::spot_analysis_append_future(post, claim_index, claim_id, market_id, resolution_policy_hash);
    }

    /// Oracle-only: open a market for an existing claim, linking `primary_post` at `claim_index`.
    public entry fun create_spot_market_for_claim(
        _: &SpotOracleAdminCap,
        config: &SpotConfig,
        registry: &mut SpotClaimRegistry,
        claim: &mut SpotClaim,
        primary_post: &mut Post,
        claim_index: u64,
        market_key_hash: vector<u8>,
        resolution_policy_hash: vector<u8>,
        betting_options: vector<String>,
        resolution_at_ms: u64,
        max_resolution_window_ms: Option<u64>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(config);
        assert_registry_version(registry);
        assert_claim_version(claim);
        assert!(config.truth_enabled, EDisabled);
        assert_valid_hash(&market_key_hash);
        assert!(!table::contains(&registry.markets_by_key_hash, market_key_hash), EMarketExists);

        let now_ms = clock::timestamp_ms(clock);
        assert!(resolution_at_ms >= now_ms, ETooEarly);

        let options_len = vector::length(&betting_options);
        assert!(options_len >= config.min_betting_options, EInvalidAmount);
        assert!(options_len <= config.max_betting_options, EInvalidAmount);

        let mut i = 0;
        while (i < options_len) {
            let option_i = vector::borrow(&betting_options, i);
            let mut j = i + 1;
            while (j < options_len) {
                assert!(*option_i != *vector::borrow(&betting_options, j), EDuplicateOption);
                j = j + 1;
            };
            i = i + 1;
        };

        let claim_id = object::uid_to_address(&claim.id);
        let primary_post_id = post::get_id_address(primary_post);
        let primary_creator = post::get_post_owner(primary_post);

        let market = SpotMarket {
            id: object::new(ctx),
            claim_id,
            market_key_hash,
            primary_post_id,
            primary_creator,
            created_at_ms: clock::timestamp_ms(clock),
            status: STATUS_OPEN,
            outcome: option::none(),
            escrow: balance::zero(),
            betting_options,
            option_escrow: table::new(ctx),
            user_option_amounts: table::new(ctx),
            bets: vector::empty(),
            resolution_window_ms: option::none(),
            max_resolution_window_ms,
            resolution_at_ms,
            last_resolution_at_ms: 0,
            resolution_timestamp_ms: 0,
            pending_payouts: table::new(ctx),
            pending_creator_payouts: table::new(ctx),
            next_creator_payout_id: 0,
            creator_payout_index: table::new(ctx),
            active_proposal_id: option::none(),
            oracle_proposed_outcome: option::none(),
            proposed_outcome: option::none(),
            dao_escalated_at_ms: 0,
            version: upgrade::current_version(),
        };

        let market_id = object::uid_to_address(&market.id);
        let hash_copy = market.market_key_hash;
        let betting_options_copy = market.betting_options;
        let max_resolution_window = market.max_resolution_window_ms;
        let resolution_at = market.resolution_at_ms;
        let created_at_ms = market.created_at_ms;

        table::add(&mut registry.markets_by_key_hash, hash_copy, market_id);
        if (table::contains(&registry.open_market_by_claim, claim_id)) {
            table::remove(&mut registry.open_market_by_claim, claim_id);
        };
        table::add(&mut registry.open_market_by_claim, claim_id, market_id);

        register_future_link(
            registry,
            claim,
            primary_post,
            market_id,
            claim_index,
            resolution_policy_hash,
            config.max_claim_per_post,
        );
        transfer::share_object(market);

        event::emit(SpotMarketCreatedEvent {
            market_id,
            claim_id,
            market_key_hash: hash_copy,
            primary_post_id,
            claim_index,
            resolution_policy_hash,
            created_at_ms,
            betting_options: betting_options_copy,
            resolution_at_ms: resolution_at,
            max_resolution_window_ms: max_resolution_window,
        });
    }

    /// Link an additional post as a future-claim referrer into an existing open market
    /// (hybrid liquidity reuse). Requires the claim to have a live open market.
    public entry fun link_post_to_spot_claim(
        _: &SpotOracleAdminCap,
        config: &SpotConfig,
        registry: &mut SpotClaimRegistry,
        claim: &mut SpotClaim,
        post: &mut Post,
        claim_index: u64,
        resolution_policy_hash: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(config);
        assert_registry_version(registry);
        assert_claim_version(claim);
        assert!(config.truth_enabled, EDisabled);
        let claim_id = object::uid_to_address(&claim.id);
        assert!(table::contains(&registry.open_market_by_claim, claim_id), ENoOpenMarket);
        let market_id = *table::borrow(&registry.open_market_by_claim, claim_id);
        let post_id = post::get_id_address(post);
        register_future_link(
            registry,
            claim,
            post,
            market_id,
            claim_index,
            resolution_policy_hash,
            config.max_claim_per_post,
        );
        event::emit(SpotPostLinkedEvent {
            post_id,
            claim_id,
            market_id: option::some(market_id),
            claim_index,
            policy_hash: resolution_policy_hash,
        });
        let _ = clock;
        let _ = ctx;
    }

    /// Oracle-only: commit a post's multi-claim analysis. Sets terminal status, counts and
    /// manifests, and emits the batch projection (future arrays + parallel past verdicts).
    public entry fun finalize_spot_claims_for_post(
        _: &SpotOracleAdminCap,
        config: &SpotConfig,
        post: &mut Post,
        detected_claim_count: u64,
        rejected_claim_count: u64,
        truncated_claim_count: u64,
        past_verified_count: u64,
        claim_manifest_hash: Option<vector<u8>>,
        veracity_manifest_hash: Option<vector<u8>>,
        past_claim_indexes: vector<u64>,
        past_verdicts: vector<u8>,
        past_related_market_ids: vector<address>,
        past_evidence_hashes: vector<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(config);
        assert!(config.truth_enabled, EDisabled);

        let past_len = vector::length(&past_claim_indexes);
        assert!(past_len == past_verified_count, EPastVerdictMismatch);
        assert!(vector::length(&past_verdicts) == past_len, EPastVerdictMismatch);
        assert!(vector::length(&past_related_market_ids) == past_len, EPastVerdictMismatch);
        assert!(vector::length(&past_evidence_hashes) == past_len, EPastVerdictMismatch);
        let mut vi = 0;
        while (vi < past_len) {
            let v = *vector::borrow(&past_verdicts, vi);
            assert!(v == VERDICT_TRUE || v == VERDICT_FALSE || v == VERDICT_UNVERIFIABLE, EPastVerdictMismatch);
            vi = vi + 1;
        };

        let future_accepted = post::spot_analysis_future_accepted_count(post);
        let status = if (future_accepted > 0 || past_verified_count > 0) {
            post::spot_status_completed()
        } else {
            post::spot_status_completed_no_actionable()
        };
        post::finalize_spot_analysis(
            post,
            status,
            detected_claim_count,
            rejected_claim_count,
            truncated_claim_count,
            past_verified_count,
            config.max_claim_per_post,
            claim_manifest_hash,
            veracity_manifest_hash,
        );

        let post_id = post::get_id_address(post);
        event::emit(SpotClaimsFinalizedForPost {
            post_id,
            status,
            detected_claim_count,
            rejected_claim_count,
            truncated_claim_count,
            future_accepted_count: future_accepted,
            past_verified_count,
            max_claim_per_post_applied: config.max_claim_per_post,
            claim_manifest_hash,
            veracity_manifest_hash,
            future_claim_indexes: post::spot_analysis_claim_indexes(post),
            future_claim_ids: post::spot_analysis_claim_ids(post),
            future_market_ids: post::spot_analysis_market_ids(post),
            past_claim_indexes,
            past_verdicts,
            past_related_market_ids,
            past_evidence_hashes,
            finalized_at_ms: clock::timestamp_ms(clock),
        });
        let _ = ctx;
    }

    /// Convenience one-shot: register claim + open one future market + finalize (single-claim
    /// posts and test setup). Emits `SpotMarketCreatedEvent` + `SpotClaimsFinalizedForPost`.
    public entry fun create_and_finalize_spot_market_for_post(
        oracle_cap: &SpotOracleAdminCap,
        config: &SpotConfig,
        registry: &mut SpotClaimRegistry,
        post: &mut Post,
        semantic_claim_hash: vector<u8>,
        market_key_hash: vector<u8>,
        resolution_policy_hash: vector<u8>,
        betting_options: vector<String>,
        resolution_at_ms: u64,
        max_resolution_window_ms: Option<u64>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(config);
        assert_registry_version(registry);
        assert!(config.truth_enabled, EDisabled);
        let created_at_ms = clock::timestamp_ms(clock);
        let mut claim = register_spot_claim(registry, semantic_claim_hash, created_at_ms, ctx);
        create_spot_market_for_claim(
            oracle_cap,
            config,
            registry,
            &mut claim,
            post,
            0,
            market_key_hash,
            resolution_policy_hash,
            betting_options,
            resolution_at_ms,
            max_resolution_window_ms,
            clock,
            ctx,
        );
        transfer::share_object(claim);

        let status = post::spot_status_completed();
        post::finalize_spot_analysis(
            post, status, 1, 0, 0, 0, config.max_claim_per_post, option::none(), option::none(),
        );
        let post_id = post::get_id_address(post);
        event::emit(SpotClaimsFinalizedForPost {
            post_id,
            status,
            detected_claim_count: 1,
            rejected_claim_count: 0,
            truncated_claim_count: 0,
            future_accepted_count: 1,
            past_verified_count: 0,
            max_claim_per_post_applied: config.max_claim_per_post,
            claim_manifest_hash: option::none(),
            veracity_manifest_hash: option::none(),
            future_claim_indexes: post::spot_analysis_claim_indexes(post),
            future_claim_ids: post::spot_analysis_claim_ids(post),
            future_market_ids: post::spot_analysis_market_ids(post),
            past_claim_indexes: vector::empty(),
            past_verdicts: vector::empty(),
            past_related_market_ids: vector::empty(),
            past_evidence_hashes: vector::empty(),
            finalized_at_ms: created_at_ms,
        });
    }

    fun assert_market_open_for_post(
        registry: &SpotClaimRegistry,
        market: &SpotMarket,
        post: &Post,
    ) {
        let post_id = post::get_id_address(post);
        assert!(market.status != STATUS_DAO_REQUIRED, EDaoDebateFrozen);
        assert!(market.status == STATUS_OPEN, EMarketNotOpen);
        let market_id = object::uid_to_address(&market.id);
        let key = PostMarketKey { post_id, market_id };
        assert!(table::contains(&registry.post_market_to_claim, key), EPostNotLinked);
        let linked_claim = *table::borrow(&registry.post_market_to_claim, key);
        assert!(linked_claim == market.claim_id, EPostNotLinked);
        let open_id = *table::borrow(&registry.open_market_by_claim, market.claim_id);
        assert!(open_id == market_id, EMarketNotOpen);
    }

    /// Sole public betting entry — registry validates the market is open for this claim.
    public entry fun place_spot_bet_for_post(
        spot_config: &SpotConfig,
        registry: &SpotClaimRegistry,
        market: &mut SpotMarket,
        post: &Post,
        mut payment: Coin<MYSO>,
        option_id: u8,
        amount: u64,
        referrer_post_id: Option<address>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_registry_version(registry);
        assert_market_version(market);
        assert!(spot_config.truth_enabled, EDisabled);
        assert!(post::spot_analysis_status(post) == post::spot_status_completed(), ENotFinalized);
        assert_market_open_for_post(registry, market, post);
        let post_id = post::get_id_address(post);
        let ref_id = if (option::is_some(&referrer_post_id)) {
            *option::borrow(&referrer_post_id)
        } else {
            post_id
        };
        place_spot_bet_internal(
            spot_config,
            market,
            post,
            &mut payment,
            option_id,
            amount,
            option::some(ref_id),
            clock,
            ctx,
        );
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, tx_context::sender(ctx));
        } else {
            coin::destroy_zero(payment);
        };
    }

    fun place_spot_bet_internal(
        spot_config: &SpotConfig,
        market: &mut SpotMarket,
        post: &Post,
        payment: &mut Coin<MYSO>,
        option_id: u8,
        amount: u64,
        referrer_post_id: Option<address>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(amount > 0, EInvalidAmount);
        if (spot_config.max_single_bet > 0) {
            assert!(amount <= spot_config.max_single_bet, EInvalidAmount);
        };
        assert!(coin::value(payment) >= amount, EInvalidAmount);
        if (spot_config.max_bets_per_record > 0) {
            assert!(vector::length(&market.bets) < spot_config.max_bets_per_record, ETooManyBets);
        };
        assert_valid_option_id(market, option_id);

        let bet_coin = coin::split(payment, amount, ctx);
        balance::join(&mut market.escrow, coin::into_balance(bet_coin));

        let current_escrow = if (table::contains(&market.option_escrow, option_id)) {
            *table::borrow(&market.option_escrow, option_id)
        } else { 0 };
        assert!(current_escrow <= MAX_U64 - amount, EOverflow);
        if (table::contains(&market.option_escrow, option_id)) {
            *table::borrow_mut(&mut market.option_escrow, option_id) = current_escrow + amount;
        } else {
            table::add(&mut market.option_escrow, option_id, amount);
        };

        let ts = clock::timestamp_ms(clock);
        let user = tx_context::sender(ctx);
        vector::push_back(&mut market.bets, SpotBet {
            user,
            option_id,
            amount,
            timestamp_ms: ts,
            referrer_post_id,
        });

        let options_len = vector::length(&market.betting_options);
        if (!table::contains(&market.user_option_amounts, user)) {
            let mut amounts = vector::empty<u64>();
            let mut i = 0;
            while (i < options_len) {
                vector::push_back(&mut amounts, 0);
                i = i + 1;
            };
            table::add(&mut market.user_option_amounts, user, amounts);
        };
        let user_amounts = table::borrow_mut(&mut market.user_option_amounts, user);
        let idx = option_id as u64;
        let current_user_amount = *vector::borrow(user_amounts, idx);
        assert!(current_user_amount <= MAX_U64 - amount, EOverflow);
        *vector::borrow_mut(user_amounts, idx) = current_user_amount + amount;

        event::emit(SpotBetPlacedEvent {
            post_id: post::get_id_address(post),
            market_id: object::uid_to_address(&market.id),
            user,
            option_id,
            amount,
            timestamp_ms: ts,
            referrer_post_id,
        });
    }

    public entry fun withdraw_spot_bet(
        spot_config: &SpotConfig,
        claim: &SpotClaim,
        market: &mut SpotMarket,
        post: &Post,
        referrer_post: &Post,
        platform: &mut Platform,
        treasury: &EcosystemTreasury,
        bet_index: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_claim_version(claim);
        assert_market_version(market);
        assert!(spot_config.truth_enabled, EDisabled);
        assert!(market.status == STATUS_OPEN, EWithdrawalNotAllowed);

        let bets_len = vector::length(&market.bets);
        assert!(bet_index < bets_len, EBetNotFound);
        let bet = *vector::borrow(&market.bets, bet_index);
        assert!(bet.user == tx_context::sender(ctx), EInvalidAmount);
        assert!(bet.amount > 0, EInvalidAmount);

        let ref_post_id = referrer_post_id_for_bet(&bet, market);
        assert!(post::get_id_address(referrer_post) == ref_post_id, EPostNotLinked);

        let platform_fee = (bet.amount * spot_config.platform_fee_bps) / 10000;
        let ecosystem_fee = (bet.amount * spot_config.ecosystem_fee_bps) / 10000;
        let creator_fee = (bet.amount * spot_config.creator_fee_bps) / 10000;
        let fee = platform_fee + ecosystem_fee + creator_fee;
        let refund_amount = bet.amount - fee;

        if (platform_fee + ecosystem_fee > 0) {
            let protocol_fee = platform_fee + ecosystem_fee;
            let mut fee_coin = coin::from_balance(balance::split(&mut market.escrow, protocol_fee), ctx);
            if (platform_fee > 0) {
                let mut platform_coin = coin::split(&mut fee_coin, platform_fee, ctx);
                platform::add_to_treasury(platform, &mut platform_coin, platform_fee, clock, ctx);
                coin::destroy_zero(platform_coin);
            };
            if (ecosystem_fee > 0) {
                transfer::public_transfer(fee_coin, profile::get_treasury_address(treasury));
            } else {
                coin::destroy_zero(fee_coin);
            };
        };

        if (creator_fee > 0) {
            let creator = creator_for_referrer_post(claim, ref_post_id, market.primary_creator);
            if (creator != @0x0) {
                transfer::public_transfer(
                    coin::from_balance(balance::split(&mut market.escrow, creator_fee), ctx),
                    creator,
                );
            };
        };

        if (refund_amount > 0) {
            transfer::public_transfer(
                coin::from_balance(balance::split(&mut market.escrow, refund_amount), ctx),
                bet.user,
            );
        };

        let option_id = bet.option_id;
        if (table::contains(&market.option_escrow, option_id)) {
            let current_escrow = *table::borrow(&market.option_escrow, option_id);
            if (current_escrow >= bet.amount) {
                *table::borrow_mut(&mut market.option_escrow, option_id) = current_escrow - bet.amount;
            };
        };
        if (table::contains(&market.user_option_amounts, bet.user)) {
            let user_amounts = table::borrow_mut(&mut market.user_option_amounts, bet.user);
            let idx = bet.option_id as u64;
            if (idx < vector::length(user_amounts)) {
                let current_user_amount = *vector::borrow(user_amounts, idx);
                if (current_user_amount >= bet.amount) {
                    *vector::borrow_mut(user_amounts, idx) = current_user_amount - bet.amount;
                };
            };
        };

        let last_index = bets_len - 1;
        if (bet_index != last_index) {
            *vector::borrow_mut(&mut market.bets, bet_index) = *vector::borrow(&market.bets, last_index);
        };
        vector::pop_back(&mut market.bets);

        event::emit(SpotBetWithdrawnEvent {
            post_id: post::get_id_address(post),
            user: bet.user,
            option_id: bet.option_id,
            amount: bet.amount,
            fee_taken: fee,
        });
    }

    public entry fun oracle_resolve(
        _: &SpotOracleAdminCap,
        spot_config: &SpotConfig,
        registry: &mut SpotClaimRegistry,
        claim: &SpotClaim,
        market: &mut SpotMarket,
        post: &Post,
        platform: &mut Platform,
        treasury: &EcosystemTreasury,
        outcome_option_id: u8,
        confidence_bps: u64,
        reasoning: String,
        evidence_urls: vector<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_registry_version(registry);
        assert_claim_version(claim);
        assert_market_version(market);
        assert!(market.status == STATUS_OPEN, EWrongStatus);
        assert!(option::is_none(&market.outcome), EAlreadyResolved);

        let now_ms = clock::timestamp_ms(clock);
        assert!(now_ms >= market.resolution_at_ms, ETooEarly);

        assert_valid_option_id(market, outcome_option_id);
        let reasoning_len = string::length(&reasoning);
        assert!(reasoning_len >= spot_config.min_reasoning_length, EInvalidReasoning);
        assert!(reasoning_len <= spot_config.max_reasoning_length, EInvalidReasoning);
        assert!(vector::length(&evidence_urls) > 0, EInvalidAmount);
        assert!(vector::length(&evidence_urls) <= spot_config.max_evidence_urls, EInvalidAmount);

        if (confidence_bps < spot_config.confidence_threshold_bps) {
            assert!(option::is_none(&market.active_proposal_id), EActiveProposalExists);
            market.status = STATUS_DAO_REQUIRED;
            market.oracle_proposed_outcome = option::some(outcome_option_id);
            market.dao_escalated_at_ms = now_ms;
            event::emit(SpotDaoRequiredEvent {
                post_id: post::get_id_address(post),
                spot_record_id: object::uid_to_address(&market.id),
                confidence_bps,
                oracle_proposed_outcome: outcome_option_id,
                dao_escalated_at_ms: now_ms,
                reasoning,
            });
            return
        };

        finalize_resolution_and_payout(
            spot_config,
            registry,
            claim,
            market,
            post,
            platform,
            treasury,
            outcome_option_id,
            reasoning,
            option::some(evidence_urls),
            clock,
            ctx,
        );
    }

    public entry fun submit_spot_resolution_proposal_to_governance(
        spot_config: &SpotConfig,
        registry: &mut GovernanceDAO,
        market: &mut SpotMarket,
        post: &Post,
        title: String,
        description: String,
        proposed_outcome: u8,
        metadata_json: Option<String>,
        coin: &mut Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_market_version(market);
        assert_spot_governance_registry(spot_config, registry);
        assert!(market.status == STATUS_DAO_REQUIRED, ENotDaoRequired);
        assert!(option::is_none(&market.active_proposal_id), EActiveProposalExists);
        validate_proposed_outcome(market, proposed_outcome);

        let spot_record_id = object::id(market);
        let proposal_id = governance::submit_spot_proposal_and_return_id(
            registry,
            title,
            description,
            spot_record_id,
            metadata_json,
            coin,
            clock,
            ctx,
        );

        market.active_proposal_id = option::some(proposal_id);
        market.proposed_outcome = option::some(proposed_outcome);

        event::emit(SpotGovernanceProposalLinkedEvent {
            post_id: post::get_id_address(post),
            spot_record_id: object::uid_to_address(&market.id),
            proposal_id,
            proposed_outcome,
        });
    }

    public entry fun implement_spot_resolution_from_governance(
        spot_config: &SpotConfig,
        registry_gov: &mut GovernanceDAO,
        proposal: &mut Proposal,
        spot_registry: &mut SpotClaimRegistry,
        claim: &SpotClaim,
        market: &mut SpotMarket,
        post: &Post,
        platform: &mut Platform,
        treasury: &EcosystemTreasury,
        reasoning: String,
        evidence_urls: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_registry_version(spot_registry);
        assert_claim_version(claim);
        assert_market_version(market);
        assert_spot_governance_registry(spot_config, registry_gov);
        assert!(market.status == STATUS_DAO_REQUIRED, ENotDaoRequired);
        assert!(option::is_some(&market.active_proposal_id), ENoActiveProposal);
        assert!(option::is_some(&market.proposed_outcome), EWrongProposal);
        let active_id = *option::borrow(&market.active_proposal_id);
        assert!(active_id == object::id(proposal), EWrongProposal);
        assert!(
            governance::proposal_status(proposal) == governance::status_approved_value(),
            EProposalNotApproved
        );

        let outcome = *option::borrow(&market.proposed_outcome);
        validate_proposed_outcome(market, outcome);

        let reasoning_len = string::length(&reasoning);
        assert!(reasoning_len >= spot_config.min_reasoning_length, EInvalidReasoning);
        assert!(reasoning_len <= spot_config.max_reasoning_length, EInvalidReasoning);
        if (option::is_some(&evidence_urls)) {
            assert!(vector::length(option::borrow(&evidence_urls)) <= spot_config.max_evidence_urls, EInvalidAmount);
        };

        let submitter = governance::proposal_submitter(proposal);
        let bal = governance::mark_proposal_implemented_take_pool(
            registry_gov,
            proposal,
            option::none(),
            clock,
            ctx,
        );
        let amount = balance::value(&bal);
        if (amount > 0) {
            transfer::public_transfer(coin::from_balance(bal, ctx), submitter);
        } else {
            balance::destroy_zero(bal);
        };

        let proposal_id = active_id;
        market.active_proposal_id = option::none();
        market.proposed_outcome = option::none();

        finalize_resolution_and_payout(
            spot_config,
            spot_registry,
            claim,
            market,
            post,
            platform,
            treasury,
            outcome,
            reasoning,
            evidence_urls,
            clock,
            ctx,
        );

        event::emit(SpotGovernanceProposalClearedEvent {
            post_id: post::get_id_address(post),
            spot_record_id: object::uid_to_address(&market.id),
            proposal_id,
        });
    }

    public entry fun clear_spot_proposal_link_on_reject(
        spot_config: &SpotConfig,
        registry: &GovernanceDAO,
        proposal: &Proposal,
        market: &mut SpotMarket,
        post: &Post,
    ) {
        assert_config_version(spot_config);
        assert_market_version(market);
        assert_spot_governance_registry(spot_config, registry);
        assert!(market.status == STATUS_DAO_REQUIRED, ENotDaoRequired);
        assert!(option::is_some(&market.active_proposal_id), ENoActiveProposal);
        let active_id = *option::borrow(&market.active_proposal_id);
        assert!(active_id == object::id(proposal), EWrongProposal);
        assert!(
            governance::proposal_status(proposal) == governance::status_rejected_value(),
            EProposalNotApproved
        );

        let proposal_id = active_id;
        market.active_proposal_id = option::none();
        market.proposed_outcome = option::none();

        event::emit(SpotGovernanceProposalClearedEvent {
            post_id: post::get_id_address(post),
            spot_record_id: object::uid_to_address(&market.id),
            proposal_id,
        });
    }

    public entry fun finalize_spot_governance_proposal(
        spot_config: &SpotConfig,
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        market: &mut SpotMarket,
        post: &Post,
        ecosystem_treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_market_version(market);
        assert_spot_governance_registry(spot_config, registry);
        governance::finalize_proposal(registry, proposal, ecosystem_treasury, clock, ctx);
        if (governance::proposal_status(proposal) == governance::status_rejected_value()) {
            clear_spot_proposal_link_on_reject(spot_config, registry, proposal, market, post);
        };
    }

    public entry fun finalize_via_dao(
        spot_config: &SpotConfig,
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        spot_registry: &mut SpotClaimRegistry,
        claim: &SpotClaim,
        market: &mut SpotMarket,
        post: &Post,
        platform: &mut Platform,
        treasury: &EcosystemTreasury,
        mut reasoning: Option<String>,
        evidence_urls: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let final_reasoning = if (option::is_some(&reasoning)) {
            option::extract(&mut reasoning)
        } else {
            string::utf8(b"DAO resolution based on community discussion")
        };
        implement_spot_resolution_from_governance(
            spot_config,
            registry,
            proposal,
            spot_registry,
            claim,
            market,
            post,
            platform,
            treasury,
            final_reasoning,
            evidence_urls,
            clock,
            ctx,
        );
    }

    fun assert_spot_governance_registry(spot_config: &SpotConfig, registry: &GovernanceDAO) {
        assert!(
            governance::registry_type(registry) == governance::proposal_type_spot_value(),
            EInvalidGovernanceRegistry
        );
        assert!(
            object::id(registry) == spot_config.spot_governance_registry_id,
            EInvalidGovernanceRegistry
        );
    }

    fun validate_proposed_outcome(market: &SpotMarket, outcome: u8) {
        if (outcome == OUTCOME_DRAW || outcome == OUTCOME_UNAPPLICABLE) {
            return
        };
        assert!((outcome as u64) < vector::length(&market.betting_options), EInvalidOptionId);
    }

    public entry fun refund_unresolved(
        _: &SpotOracleAdminCap,
        spot_config: &SpotConfig,
        registry: &mut SpotClaimRegistry,
        market: &mut SpotMarket,
        post: &Post,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_registry_version(registry);
        assert_market_version(market);
        assert!(option::is_some(&market.max_resolution_window_ms), EInvalidAmount);
        let now_ms = clock::timestamp_ms(clock);
        let max_window = *option::borrow(&market.max_resolution_window_ms);
        assert!(now_ms >= market.resolution_at_ms + max_window, ETooEarly);
        assert!(market.status == STATUS_OPEN || market.status == STATUS_DAO_REQUIRED, EWrongStatus);
        assert!(vector::length(&market.bets) > 0, ENoBets);

        let mut i = 0;
        let len = vector::length(&market.bets);
        while (i < len) {
            let bet = vector::borrow(&market.bets, i);
            if (bet.amount > 0) {
                let c = coin::from_balance(balance::split(&mut market.escrow, bet.amount), ctx);
                transfer::public_transfer(c, bet.user);
                event::emit(SpotRefundEvent {
                    post_id: market.primary_post_id,
                    user: bet.user,
                    amount: bet.amount,
                });
            };
            i = i + 1;
        };

        if (table::contains(&registry.open_market_by_claim, market.claim_id)) {
            table::remove(&mut registry.open_market_by_claim, market.claim_id);
        };
        market.status = STATUS_REFUNDABLE;
        market.outcome = option::none();
        market.last_resolution_at_ms = now_ms;
        let _ = post;
    }

    fun referrer_post_id_for_bet(bet: &SpotBet, market: &SpotMarket): address {
        if (option::is_some(&bet.referrer_post_id)) {
            *option::borrow(&bet.referrer_post_id)
        } else {
            market.primary_post_id
        }
    }

    fun track_creator_payout_index(market: &mut SpotMarket, creator: address, payout_id: u64) {
        if (table::contains(&market.creator_payout_index, creator)) {
            let ids = table::borrow_mut(&mut market.creator_payout_index, creator);
            vector::push_back(ids, payout_id);
        } else {
            let mut ids = vector::empty<u64>();
            vector::push_back(&mut ids, payout_id);
            table::add(&mut market.creator_payout_index, creator, ids);
        };
    }

    fun untrack_creator_payout_index(market: &mut SpotMarket, creator: address, payout_id: u64) {
        if (!table::contains(&market.creator_payout_index, creator)) {
            return
        };
        let ids = table::borrow_mut(&mut market.creator_payout_index, creator);
        let mut k = 0;
        let len = vector::length(ids);
        while (k < len) {
            if (*vector::borrow(ids, k) == payout_id) {
                vector::remove(ids, k);
                break
            };
            k = k + 1;
        };
        if (vector::is_empty(ids)) {
            table::remove(&mut market.creator_payout_index, creator);
        };
    }

    fun creator_for_referrer_post(
        claim: &SpotClaim,
        referrer_post_id: address,
        fallback_creator: address,
    ): address {
        let mut i = 0;
        let len = vector::length(&claim.linked_posts);
        while (i < len) {
            let link = vector::borrow(&claim.linked_posts, i);
            if (link.post_id == referrer_post_id) {
                return link.creator
            };
            i = i + 1;
        };
        fallback_creator
    }

    fun vector_contains_address(addrs: &vector<address>, addr: address): bool {
        let mut i = 0;
        let len = vector::length(addrs);
        while (i < len) {
            if (*vector::borrow(addrs, i) == addr) {
                return true
            };
            i = i + 1;
        };
        false
    }

    fun referred_volume_for_post(market: &SpotMarket, referrer_post_id: address): u64 {
        let mut total = 0u64;
        let mut i = 0;
        let len = vector::length(&market.bets);
        while (i < len) {
            let bet = vector::borrow(&market.bets, i);
            if (referrer_post_id_for_bet(bet, market) == referrer_post_id) {
                total = total + bet.amount;
            };
            i = i + 1;
        };
        total
    }

    fun total_referred_volume(market: &SpotMarket): u64 {
        let mut total = 0u64;
        let mut i = 0;
        let len = vector::length(&market.bets);
        while (i < len) {
            let bet = vector::borrow(&market.bets, i);
            total = total + bet.amount;
            i = i + 1;
        };
        total
    }

    fun accrue_creator_payouts(
        spot_config: &SpotConfig,
        claim: &SpotClaim,
        market: &mut SpotMarket,
        creator_fee_total: u64,
        resolution_timestamp_ms: u64,
    ) {
        if (creator_fee_total == 0) {
            return
        };
        let total_volume = total_referred_volume(market);
        if (total_volume == 0) {
            return
        };
        let expires_at_ms = resolution_timestamp_ms + spot_config.creator_claim_window_ms;
        let market_id = object::uid_to_address(&market.id);

        let mut unique_refs = vector::empty<address>();
        let mut i = 0;
        let bets_len = vector::length(&market.bets);
        while (i < bets_len) {
            let bet = vector::borrow(&market.bets, i);
            let ref_post = referrer_post_id_for_bet(bet, market);
            if (!vector_contains_address(&unique_refs, ref_post)) {
                vector::push_back(&mut unique_refs, ref_post);
            };
            i = i + 1;
        };

        let mut j = 0;
        let refs_len = vector::length(&unique_refs);
        while (j < refs_len) {
            let ref_post = *vector::borrow(&unique_refs, j);
            let volume = referred_volume_for_post(market, ref_post);
            if (volume > 0) {
                let amount = ((volume as u128) * (creator_fee_total as u128) / (total_volume as u128)) as u64;
                if (amount > 0) {
                    let creator = creator_for_referrer_post(claim, ref_post, market.primary_creator);
                    if (creator != @0x0) {
                        let payout_id = market.next_creator_payout_id;
                        market.next_creator_payout_id = payout_id + 1;
                        table::add(&mut market.pending_creator_payouts, payout_id, SpotCreatorPayout {
                            creator,
                            source_post_id: ref_post,
                            amount,
                            expires_at_ms,
                        });
                        event::emit(SpotCreatorPayoutAccruedEvent {
                            market_id,
                            payout_id,
                            creator,
                            referrer_post_id: ref_post,
                            amount,
                            expires_at_ms,
                        });
                        track_creator_payout_index(market, creator, payout_id);
                    };
                };
            };
            j = j + 1;
        };
    }

    fun finalize_resolution_and_payout(
        spot_config: &SpotConfig,
        registry: &mut SpotClaimRegistry,
        claim: &SpotClaim,
        market: &mut SpotMarket,
        post: &Post,
        platform: &mut Platform,
        treasury: &EcosystemTreasury,
        outcome: u8,
        reasoning: String,
        evidence_urls: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(market.status == STATUS_OPEN || market.status == STATUS_DAO_REQUIRED, EWrongStatus);
        assert!(vector::length(&market.bets) > 0, ENoBets);

        let total_escrow = total_option_escrow(market);
        let now_ms = clock::timestamp_ms(clock);
        let market_id = object::uid_to_address(&market.id);

        if (outcome == OUTCOME_DRAW || outcome == OUTCOME_UNAPPLICABLE) {
            let mut i = 0;
            let len = vector::length(&market.bets);
            while (i < len) {
                let bet = vector::borrow(&market.bets, i);
                if (bet.amount > 0) {
                    let c = coin::from_balance(balance::split(&mut market.escrow, bet.amount), ctx);
                    transfer::public_transfer(c, bet.user);
                    event::emit(SpotRefundEvent {
                        post_id: market.primary_post_id,
                        user: bet.user,
                        amount: bet.amount,
                    });
                };
                i = i + 1;
            };
            if (table::contains(&registry.open_market_by_claim, market.claim_id)) {
                table::remove(&mut registry.open_market_by_claim, market.claim_id);
            };
            market.status = STATUS_RESOLVED;
            market.outcome = option::some(outcome);
            market.last_resolution_at_ms = now_ms;
            market.resolution_timestamp_ms = now_ms;
            let evidence_urls_vec = if (option::is_some(&evidence_urls)) {
                *option::borrow(&evidence_urls)
            } else { vector::empty() };
            event::emit(SpotResolvedEvent {
                post_id: post::get_id_address(post),
                market_id,
                claim_id: market.claim_id,
                outcome,
                total_escrow,
                fee_taken: 0,
                creator_fee_total: 0,
                reasoning,
                evidence_urls: evidence_urls_vec,
            });
            return
        };

        let winning_total = get_option_escrow(market, outcome);

        let platform_fee = (total_escrow * spot_config.platform_fee_bps) / 10000;
        let ecosystem_fee = (total_escrow * spot_config.ecosystem_fee_bps) / 10000;
        let creator_fee_total = (total_escrow * spot_config.creator_fee_bps) / 10000;
        let protocol_fee = platform_fee + ecosystem_fee;
        let distributable = total_escrow - protocol_fee - creator_fee_total;

        if (protocol_fee > 0) {
            let mut fee_coin = coin::from_balance(balance::split(&mut market.escrow, protocol_fee), ctx);
            if (platform_fee > 0) {
                let mut platform_coin = coin::split(&mut fee_coin, platform_fee, ctx);
                platform::add_to_treasury(platform, &mut platform_coin, platform_fee, clock, ctx);
                coin::destroy_zero(platform_coin);
            };
            if (ecosystem_fee > 0) {
                transfer::public_transfer(fee_coin, profile::get_treasury_address(treasury));
            } else {
                coin::destroy_zero(fee_coin);
            };
        };

        if (creator_fee_total > 0) {
            accrue_creator_payouts(
                spot_config,
                claim,
                market,
                creator_fee_total,
                now_ms,
            );
        };

        let mut i = 0;
        let len = vector::length(&market.bets);
        while (i < len) {
            let bet = vector::borrow(&market.bets, i);
            if (bet.option_id == outcome && winning_total > 0 && bet.amount > 0) {
                let payout = (((bet.amount as u128) * (distributable as u128)) / (winning_total as u128)) as u64;
                if (payout > 0) {
                    if (table::contains(&market.pending_payouts, bet.user)) {
                        *table::borrow_mut(&mut market.pending_payouts, bet.user) =
                            *table::borrow(&market.pending_payouts, bet.user) + payout;
                    } else {
                        table::add(&mut market.pending_payouts, bet.user, payout);
                    };
                };
            };
            i = i + 1;
        };

        if (table::contains(&registry.open_market_by_claim, market.claim_id)) {
            table::remove(&mut registry.open_market_by_claim, market.claim_id);
        };
        market.status = STATUS_RESOLVED;
        market.outcome = option::some(outcome);
        market.last_resolution_at_ms = now_ms;
        market.resolution_timestamp_ms = now_ms;

        let evidence_urls_vec = if (option::is_some(&evidence_urls)) {
            *option::borrow(&evidence_urls)
        } else { vector::empty() };
        event::emit(SpotResolvedEvent {
            post_id: post::get_id_address(post),
            market_id,
            claim_id: market.claim_id,
            outcome,
            total_escrow,
            fee_taken: protocol_fee,
            creator_fee_total,
            reasoning,
            evidence_urls: evidence_urls_vec,
        });
    }

    public entry fun claim_payout(
        spot_config: &SpotConfig,
        market: &mut SpotMarket,
        post: &Post,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_market_version(market);
        assert!(spot_config.truth_enabled, EDisabled);
        assert!(market.status == STATUS_RESOLVED, EWrongStatus);
        assert!(option::is_some(&market.outcome), ENotOracle);

        let user = tx_context::sender(ctx);
        assert!(table::contains(&market.pending_payouts, user), EBetNotFound);
        let pending_amount = *table::borrow(&market.pending_payouts, user);
        assert!(pending_amount > 0, EInvalidAmount);

        let current_time = clock::timestamp_ms(clock);
        assert!(current_time >= market.resolution_timestamp_ms + spot_config.payout_delay_ms, ETooEarly);

        transfer::public_transfer(
            coin::from_balance(balance::split(&mut market.escrow, pending_amount), ctx),
            user,
        );
        table::remove(&mut market.pending_payouts, user);

        event::emit(SpotPayoutEvent {
            post_id: post::get_id_address(post),
            user,
            amount: pending_amount,
        });
    }

    /// Single O(1) creator fee claim by `payout_id`.
    public entry fun claim_creator_payout(
        spot_config: &SpotConfig,
        market: &mut SpotMarket,
        payout_id: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_market_version(market);
        assert!(spot_config.truth_enabled, EDisabled);
        assert!(market.status == STATUS_RESOLVED, EWrongStatus);
        assert!(table::contains(&market.pending_creator_payouts, payout_id), EPayoutNotFound);

        let payout = *table::borrow(&market.pending_creator_payouts, payout_id);
        assert!(tx_context::sender(ctx) == payout.creator, ENotCreator);

        let now = clock::timestamp_ms(clock);
        assert!(now >= market.resolution_timestamp_ms + spot_config.payout_delay_ms, ETooEarly);
        assert!(now <= payout.expires_at_ms, ECreatorPayoutExpired);

        untrack_creator_payout_index(market, payout.creator, payout_id);
        table::remove(&mut market.pending_creator_payouts, payout_id);
        transfer::public_transfer(
            coin::from_balance(balance::split(&mut market.escrow, payout.amount), ctx),
            payout.creator,
        );

        event::emit(SpotCreatorPayoutClaimedEvent {
            market_id: object::uid_to_address(&market.id),
            payout_id,
            creator: payout.creator,
            amount: payout.amount,
        });
    }

    /// Reclaim expired creator rewards to ecosystem (+ platform remainder).
    public entry fun reclaim_expired_creator_rewards(
        spot_config: &SpotConfig,
        market: &mut SpotMarket,
        platform: &mut Platform,
        treasury: &EcosystemTreasury,
        payout_id: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_config_version(spot_config);
        assert_market_version(market);
        assert!(market.status == STATUS_RESOLVED, EWrongStatus);
        assert!(table::contains(&market.pending_creator_payouts, payout_id), EPayoutNotFound);

        let now = clock::timestamp_ms(clock);
        let payout = *table::borrow(&market.pending_creator_payouts, payout_id);
        assert!(now > payout.expires_at_ms, ETooEarly);

        let payout = table::remove(&mut market.pending_creator_payouts, payout_id);
        untrack_creator_payout_index(market, payout.creator, payout_id);
        let amount = payout.amount;
        let ecosystem_amount = (amount * spot_config.expired_creator_ecosystem_bps) / 10000;
        let platform_amount = amount - ecosystem_amount;

        if (amount > 0) {
            let mut fee_coin = coin::from_balance(balance::split(&mut market.escrow, amount), ctx);
            if (platform_amount > 0) {
                let mut platform_coin = coin::split(&mut fee_coin, platform_amount, ctx);
                platform::add_to_treasury(platform, &mut platform_coin, platform_amount, clock, ctx);
                coin::destroy_zero(platform_coin);
            };
            if (ecosystem_amount > 0) {
                transfer::public_transfer(fee_coin, profile::get_treasury_address(treasury));
            } else {
                coin::destroy_zero(fee_coin);
            };
        };

        event::emit(SpotCreatorPayoutReclaimedEvent {
            market_id: object::uid_to_address(&market.id),
            payout_id,
            ecosystem_amount,
            platform_amount,
        });
    }

    public entry fun migrate_config(
        config: &mut SpotConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        assert!(config.version < current_version, EWrongVersion);
        let old_version = config.version;
        if (old_version == 0) {
            config.spot_governance_registry_id = object::id_from_address(@0x0);
        };
        if (old_version < 2) {
            config.creator_fee_bps = DEFAULT_CREATOR_FEE_BPS;
            config.creator_claim_window_ms = DEFAULT_CREATOR_CLAIM_WINDOW_MS;
            config.expired_creator_ecosystem_bps = DEFAULT_EXPIRED_CREATOR_ECOSYSTEM_BPS;
        };
        if (config.max_claim_per_post == 0) {
            config.max_claim_per_post = DEFAULT_MAX_CLAIM_PER_POST;
        };
        config.version = current_version;
        upgrade::emit_migration_event(
            object::id(config),
            string::utf8(b"SpotConfig"),
            old_version,
            tx_context::sender(ctx),
        );
    }

    public entry fun migrate_claim_registry(
        registry: &mut SpotClaimRegistry,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        assert!(registry.version < current_version, EWrongVersion);
        let old_version = registry.version;
        registry.version = current_version;
        upgrade::emit_migration_event(
            object::id(registry),
            string::utf8(b"SpotClaimRegistry"),
            old_version,
            tx_context::sender(ctx),
        );
    }

    public entry fun migrate_claim(
        claim: &mut SpotClaim,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        assert!(claim.version < current_version, EWrongVersion);
        let old_version = claim.version;
        claim.version = current_version;
        upgrade::emit_migration_event(
            object::id(claim),
            string::utf8(b"SpotClaim"),
            old_version,
            tx_context::sender(ctx),
        );
    }

    public entry fun migrate_market(
        market: &mut SpotMarket,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        assert!(market.version < current_version, EWrongVersion);
        let old_version = market.version;
        if (old_version == 0) {
            market.active_proposal_id = option::none();
            market.oracle_proposed_outcome = option::none();
            market.proposed_outcome = option::none();
            market.dao_escalated_at_ms = 0;
        };
        if (old_version < 2) {
            market.next_creator_payout_id = 0;
        };
        market.version = current_version;
        upgrade::emit_migration_event(
            object::id(market),
            string::utf8(b"SpotMarket"),
            old_version,
            tx_context::sender(ctx),
        );
    }

    /// Deprecated alias for `migrate_market`.
    public entry fun migrate_record(
        market: &mut SpotMarket,
        cap: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        migrate_market(market, cap, ctx);
    }

    /// Test-only direct bet helper (skips registry routing guard).
    #[test_only]
    public fun get_pending_creator_payout_amount(market: &SpotMarket, payout_id: u64): u64 {
        table::borrow(&market.pending_creator_payouts, payout_id).amount
    }

    #[test_only]
    public fun get_pending_creator_payout_creator(market: &SpotMarket, payout_id: u64): address {
        table::borrow(&market.pending_creator_payouts, payout_id).creator
    }

    #[test_only]
    public fun get_creator_fee_bps(config: &SpotConfig): u64 {
        config.creator_fee_bps
    }

    /// Test-only shim mirroring the removed one-shot record flow: deterministic per-post
    /// semantic/market hashes, claim + market + link + finalize for a single future claim.
    #[test_only]
    public fun create_spot_record_for_post(
        oracle_cap: &SpotOracleAdminCap,
        config: &SpotConfig,
        registry: &mut SpotClaimRegistry,
        post: &mut Post,
        betting_options: vector<String>,
        resolution_window_ms: Option<u64>,
        max_resolution_window_ms: Option<u64>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let post_id = post::get_id_address(post);
        let mut semantic_hash = bcs::to_bytes(&post_id);
        vector::push_back(&mut semantic_hash, 0);
        let mut market_hash = bcs::to_bytes(&post_id);
        vector::push_back(&mut market_hash, 1);
        let created_at_ms = clock::timestamp_ms(clock);
        let resolution_at_ms = if (option::is_some(&max_resolution_window_ms)) {
            created_at_ms + *option::borrow(&max_resolution_window_ms)
        } else if (option::is_some(&resolution_window_ms)) {
            created_at_ms + *option::borrow(&resolution_window_ms)
        } else {
            created_at_ms + DEFAULT_MAX_RESOLUTION_WINDOW_MS
        };
        create_and_finalize_spot_market_for_post(
            oracle_cap,
            config,
            registry,
            post,
            semantic_hash,
            market_hash,
            b"test_policy",
            betting_options,
            resolution_at_ms,
            max_resolution_window_ms,
            clock,
            ctx,
        );
    }

    /// Test-only: deterministic per-post semantic hash (matches the shim above).
    #[test_only]
    public fun test_semantic_hash_for_post(post_id: address): vector<u8> {
        let mut semantic_hash = bcs::to_bytes(&post_id);
        vector::push_back(&mut semantic_hash, 0);
        semantic_hash
    }

    #[test_only]
    public fun place_spot_bet(
        spot_config: &SpotConfig,
        market: &mut SpotMarket,
        post: &Post,
        mut payment: Coin<MYSO>,
        option_id: u8,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(spot_config.truth_enabled, EDisabled);
        assert!(market.status != STATUS_DAO_REQUIRED, EDaoDebateFrozen);
        assert!(market.status == STATUS_OPEN, EWrongStatus);
        let post_id = post::get_id_address(post);
        place_spot_bet_internal(
            spot_config,
            market,
            post,
            &mut payment,
            option_id,
            amount,
            option::some(post_id),
            clock,
            ctx,
        );
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, tx_context::sender(ctx));
        } else {
            coin::destroy_zero(payment);
        };
    }
}
