// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Social Proof of Truth (SPoT)
/// Prediction market for post truthfulness. Users bet on custom options (2-10 options per record).
/// All bets go directly to escrow. Oracle/DAO resolves the outcome, and winners receive
/// pro-rata payouts from the total escrow pool. Users can withdraw bets before resolution
/// with the same fee structure as payouts. Time-based resolution windows are optional per record.

#[allow(duplicate_alias, unused_use, unused_const, unused_variable, lint(self_transfer, share_owned, public_entry))]
module social_contracts::social_proof_of_truth {
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

    /// Status
    const STATUS_OPEN: u8 = 1;
    const STATUS_DAO_REQUIRED: u8 = 2;
    const STATUS_RESOLVED: u8 = 3;
    const STATUS_REFUNDABLE: u8 = 4;

    /// Outcomes
    /// Note: For multi-option betting, outcome is the winning option_id (0-indexed)
    /// Special outcomes: DRAW = 255, UNAPPLICABLE = 254
    const OUTCOME_DRAW: u8 = 255;
    const OUTCOME_UNAPPLICABLE: u8 = 254;

    /// Milliseconds per nominal day (used to map old default epoch-window counts to wall time).
    const MS_PER_DAY: u64 = 86400000;

    /// Config defaults
    const DEFAULT_CONFIDENCE_THRESHOLD_BPS: u64 = 7000; // 70%
    const DEFAULT_ENABLE: bool = false;
    /// ~72 days if legacy assumed ~1 day per chain epoch; adjust in config as needed.
    const DEFAULT_RESOLUTION_WINDOW_MS: u64 = 72 * MS_PER_DAY;
    const DEFAULT_MAX_RESOLUTION_WINDOW_MS: u64 = 144 * MS_PER_DAY;
    const DEFAULT_PAYOUT_DELAY_MS: u64 = 0;
    const DEFAULT_PLATFORM_FEE_BPS: u64 = 50;
    const DEFAULT_ECOSYSTEM_FEE_BPS: u64 = 50;
    const DEFAULT_MIN_BETTING_OPTIONS: u64 = 2;
    const DEFAULT_MAX_BETTING_OPTIONS: u64 = 10;
    const DEFAULT_MIN_REASONING_LENGTH: u64 = 10;
    const DEFAULT_MAX_REASONING_LENGTH: u64 = 5000;
    const DEFAULT_MAX_EVIDENCE_URLS: u64 = 10;
    /// Default cap on `SpotRecord.bets` length at init; admins may set `0` for no limit via `update_spot_config`.
    const DEFAULT_MAX_BETS_PER_RECORD: u64 = 10000;

    /// Maximum u64 value for overflow protection
    const MAX_U64: u64 = 18446744073709551615;

    /// Admin capability for SPoT (controls SpotConfig updates)
    public struct SpotAdminCap has key, store { id: UID }

    /// Oracle admin capability for SPoT (controls oracle decisions: record creation and resolution)
    public struct SpotOracleAdminCap has key, store { id: UID }

    /// Global configuration for SPoT
    public struct SpotConfig has key {
        id: UID,
        enable_flag: bool,
        confidence_threshold_bps: u64,
        resolution_window_ms: u64,
        max_resolution_window_ms: u64,
        payout_delay_ms: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        min_betting_options: u64,
        max_betting_options: u64,
        min_reasoning_length: u64,
        max_reasoning_length: u64,
        max_evidence_urls: u64,
        oracle_address: address,
        max_single_bet: u64,
        max_bets_per_record: u64,
        spot_governance_registry_id: ID,
        version: u64,
    }

    /// A single bet
    public struct SpotBet has store, copy, drop {
        user: address,
        option_id: u8,
        amount: u64,
        /// Wall-clock ms from `Clock` when the bet was placed.
        timestamp_ms: u64,
    }

    /// SPoT record per post
    public struct SpotRecord has key {
        id: UID,
        post_id: address,
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
        last_resolution_at_ms: u64,
        resolution_timestamp_ms: u64,
        pending_payouts: Table<address, u64>,
        active_proposal_id: Option<ID>,
        oracle_proposed_outcome: Option<u8>,
        proposed_outcome: Option<u8>,
        dao_escalated_at_ms: u64,
        version: u64,
    }

    /// Events
    public struct SpotBetPlacedEvent has copy, drop {
        post_id: address,
        user: address,
        option_id: u8,
        amount: u64,
        timestamp_ms: u64,
    }

    public struct SpotResolvedEvent has copy, drop {
        post_id: address,
        outcome: u8, // Winning option_id, or OUTCOME_DRAW/OUTCOME_UNAPPLICABLE
        total_escrow: u64,
        fee_taken: u64,
        reasoning: String, // Required reasoning from oracle
        evidence_urls: vector<String>, // Required array of evidence URLs (at least 1)
    }

    public struct SpotDaoRequiredEvent has copy, drop {
        post_id: address,
        spot_record_id: address,
        confidence_bps: u64,
        oracle_proposed_outcome: u8,
        dao_escalated_at_ms: u64,
        reasoning: String, // Required reasoning why DAO is needed
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

    public struct SpotRefundEvent has copy, drop {
        post_id: address,
        user: address,
        amount: u64,
    }

    public struct SpotConfigUpdatedEvent has copy, drop {
        updated_by: address,
        enable_flag: bool,
        confidence_threshold_bps: u64,
        resolution_window_ms: u64,
        max_resolution_window_ms: u64,
        payout_delay_ms: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        min_betting_options: u64,
        max_betting_options: u64,
        min_reasoning_length: u64,
        max_reasoning_length: u64,
        max_evidence_urls: u64,
        oracle_address: address,
        max_single_bet: u64,
        max_bets_per_record: u64,
        timestamp: u64,
    }

    public struct SpotBetWithdrawnEvent has copy, drop {
        post_id: address,
        user: address,
        option_id: u8,
        amount: u64,
        fee_taken: u64,
    }

    public struct SpotRecordCreatedEvent has copy, drop {
        record_id: address,
        post_id: address,
        created_at_ms: u64,
        betting_options: vector<String>,
        resolution_window_ms: Option<u64>,
        max_resolution_window_ms: Option<u64>,
    }

    // Public getters for testing/inspection
    public fun get_status(rec: &SpotRecord): u8 { rec.status }
    public fun get_bets_len(rec: &SpotRecord): u64 { vector::length(&rec.bets) }
    public fun get_betting_options(rec: &SpotRecord): vector<String> { rec.betting_options }
    public fun get_option_escrow(rec: &SpotRecord, option_id: u8): u64 {
        if (table::contains(&rec.option_escrow, option_id)) {
            *table::borrow(&rec.option_escrow, option_id)
        } else {
            0
        }
    }
    public fun get_id_address(rec: &SpotRecord): address {
        object::uid_to_address(&rec.id)
    }
    public fun get_outcome(rec: &SpotRecord): &Option<u8> { &rec.outcome }
    public fun is_open(rec: &SpotRecord): bool { rec.status == STATUS_OPEN }
    public fun is_resolved(rec: &SpotRecord): bool { rec.status == STATUS_RESOLVED }
    public fun outcome_draw(): u8 { OUTCOME_DRAW }
    public fun outcome_unapplicable(): u8 { OUTCOME_UNAPPLICABLE }
    public fun get_user_option_amount(rec: &SpotRecord, user: address, option_id: u8): u64 {
        if (!table::contains(&rec.user_option_amounts, user)) {
            0
        } else {
            let amounts = table::borrow(&rec.user_option_amounts, user);
            let idx = option_id as u64;
            if (idx >= vector::length(amounts)) {
                0
            } else {
                *vector::borrow(amounts, idx)
            }
        }
    }

    public fun num_betting_options(rec: &SpotRecord): u64 {
        vector::length(&rec.betting_options)
    }

    /// Sum of per-option escrow (same aggregation as resolve).
    public fun total_option_escrow(rec: &SpotRecord): u64 {
        let mut total = 0;
        let mut i = 0;
        let n = vector::length(&rec.betting_options);
        while (i < n) {
            let option_id = (i as u8);
            let amt = get_option_escrow(rec, option_id);
            assert!(total <= MAX_U64 - amt, EOverflow);
            total = total + amt;
            i = i + 1;
        };
        total
    }

    public fun assert_valid_option_id(rec: &SpotRecord, option_id: u8) {
        assert!((option_id as u64) < vector::length(&rec.betting_options), EInvalidOptionId);
    }

    // Public getter for SpotConfig
    public fun is_enabled(config: &SpotConfig): bool { config.enable_flag }

    public fun spot_governance_registry_id(config: &SpotConfig): ID {
        config.spot_governance_registry_id
    }

    public fun active_proposal_id(record: &SpotRecord): &Option<ID> {
        &record.active_proposal_id
    }

    public fun proposed_outcome(record: &SpotRecord): &Option<u8> {
        &record.proposed_outcome
    }

    public fun oracle_proposed_outcome(record: &SpotRecord): &Option<u8> {
        &record.oracle_proposed_outcome
    }

    public fun dao_escalated_at_ms(record: &SpotRecord): u64 {
        record.dao_escalated_at_ms
    }

    // Bootstrap
    public(package) fun bootstrap_init(
        clock: &Clock,
        spot_governance_registry_id: ID,
        ctx: &mut TxContext
    ) {
        let admin = tx_context::sender(ctx);
        let config = SpotConfig {
            id: object::new(ctx),
            enable_flag: DEFAULT_ENABLE,
            confidence_threshold_bps: DEFAULT_CONFIDENCE_THRESHOLD_BPS,
            resolution_window_ms: DEFAULT_RESOLUTION_WINDOW_MS,
            max_resolution_window_ms: DEFAULT_MAX_RESOLUTION_WINDOW_MS,
            payout_delay_ms: DEFAULT_PAYOUT_DELAY_MS,
            platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
            ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
            min_betting_options: DEFAULT_MIN_BETTING_OPTIONS,
            max_betting_options: DEFAULT_MAX_BETTING_OPTIONS,
            min_reasoning_length: DEFAULT_MIN_REASONING_LENGTH,
            max_reasoning_length: DEFAULT_MAX_REASONING_LENGTH,
            max_evidence_urls: DEFAULT_MAX_EVIDENCE_URLS,
            oracle_address: admin,
            max_single_bet: 0,
            max_bets_per_record: DEFAULT_MAX_BETS_PER_RECORD,
            spot_governance_registry_id,
            version: upgrade::current_version(),
        };

        // Emit event so indexer can populate spot_config table
        event::emit(SpotConfigUpdatedEvent {
            updated_by: admin,
            enable_flag: DEFAULT_ENABLE,
            confidence_threshold_bps: DEFAULT_CONFIDENCE_THRESHOLD_BPS,
            resolution_window_ms: DEFAULT_RESOLUTION_WINDOW_MS,
            max_resolution_window_ms: DEFAULT_MAX_RESOLUTION_WINDOW_MS,
            payout_delay_ms: DEFAULT_PAYOUT_DELAY_MS,
            platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
            ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
            min_betting_options: DEFAULT_MIN_BETTING_OPTIONS,
            max_betting_options: DEFAULT_MAX_BETTING_OPTIONS,
            min_reasoning_length: DEFAULT_MIN_REASONING_LENGTH,
            max_reasoning_length: DEFAULT_MAX_REASONING_LENGTH,
            max_evidence_urls: DEFAULT_MAX_EVIDENCE_URLS,
            oracle_address: admin,
            max_single_bet: 0,
            max_bets_per_record: DEFAULT_MAX_BETS_PER_RECORD,
            timestamp: clock::timestamp_ms(clock),
        });

        transfer::share_object(config);
    }

    /// Create a SpotAdminCap for bootstrap (package visibility only)
    public(package) fun create_spot_admin_cap(ctx: &mut TxContext): SpotAdminCap {
        SpotAdminCap {
            id: object::new(ctx)
        }
    }

    /// Create a SpotOracleAdminCap for bootstrap (package visibility only)
    public(package) fun create_spot_oracle_admin_cap(ctx: &mut TxContext): SpotOracleAdminCap {
        SpotOracleAdminCap {
            id: object::new(ctx)
        }
    }

    #[test_only]
    /// Initialize SPoT for testing - creates admin caps and config
    public fun test_init(clock: &Clock, spot_governance_registry_id: ID, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        
        // Create and share config
        transfer::share_object(SpotConfig {
            id: object::new(ctx),
            enable_flag: DEFAULT_ENABLE,
            confidence_threshold_bps: DEFAULT_CONFIDENCE_THRESHOLD_BPS,
            resolution_window_ms: DEFAULT_RESOLUTION_WINDOW_MS,
            max_resolution_window_ms: DEFAULT_MAX_RESOLUTION_WINDOW_MS,
            payout_delay_ms: DEFAULT_PAYOUT_DELAY_MS,
            platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
            ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
            min_betting_options: DEFAULT_MIN_BETTING_OPTIONS,
            max_betting_options: DEFAULT_MAX_BETTING_OPTIONS,
            min_reasoning_length: DEFAULT_MIN_REASONING_LENGTH,
            max_reasoning_length: DEFAULT_MAX_REASONING_LENGTH,
            max_evidence_urls: DEFAULT_MAX_EVIDENCE_URLS,
            oracle_address: sender,
            max_single_bet: 0,
            max_bets_per_record: DEFAULT_MAX_BETS_PER_RECORD,
            spot_governance_registry_id,
            version: upgrade::current_version(),
        });
        
        // Create and transfer admin capabilities to the transaction sender
        transfer::public_transfer(SpotAdminCap { id: object::new(ctx) }, sender);
        transfer::public_transfer(SpotOracleAdminCap { id: object::new(ctx) }, sender);
    }

    /// Update SPoT configuration (admin only).
    /// `max_single_bet` and `max_bets_per_record` use `0` for no limit; positive values enforce caps.
    public entry fun update_spot_config(
        _: &SpotAdminCap,
        config: &mut SpotConfig,
        enable_flag: bool,
        confidence_threshold_bps: u64,
        resolution_window_ms: u64,
        max_resolution_window_ms: u64,
        payout_delay_ms: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        min_betting_options: u64,
        max_betting_options: u64,
        min_reasoning_length: u64,
        max_reasoning_length: u64,
        max_evidence_urls: u64,
        oracle_address: address,
        max_single_bet: u64,
        max_bets_per_record: u64,
        spot_governance_registry_id: ID,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Basic bounds
        assert!(confidence_threshold_bps <= 10000, EInvalidAmount);
        assert!(platform_fee_bps <= 10000, EInvalidAmount);
        assert!(ecosystem_fee_bps <= 10000, EInvalidAmount);
        assert!(platform_fee_bps + ecosystem_fee_bps <= 10000, EInvalidAmount);
        assert!(min_betting_options > 0, EInvalidAmount);
        assert!(min_betting_options <= max_betting_options, EInvalidAmount);
        assert!(min_reasoning_length > 0, EInvalidReasoning);
        assert!(min_reasoning_length <= max_reasoning_length, EInvalidReasoning);
        assert!(max_evidence_urls > 0, EInvalidAmount);
        // windows may be zero in tests to resolve immediately

        config.enable_flag = enable_flag;
        config.confidence_threshold_bps = confidence_threshold_bps;
        config.resolution_window_ms = resolution_window_ms;
        config.max_resolution_window_ms = max_resolution_window_ms;
        config.payout_delay_ms = payout_delay_ms;
        config.platform_fee_bps = platform_fee_bps;
        config.ecosystem_fee_bps = ecosystem_fee_bps;
        config.min_betting_options = min_betting_options;
        config.max_betting_options = max_betting_options;
        config.min_reasoning_length = min_reasoning_length;
        config.max_reasoning_length = max_reasoning_length;
        config.max_evidence_urls = max_evidence_urls;
        config.oracle_address = oracle_address;
        config.max_single_bet = max_single_bet;
        config.max_bets_per_record = max_bets_per_record;
        config.spot_governance_registry_id = spot_governance_registry_id;
        
        // Emit config updated event
        event::emit(SpotConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            enable_flag,
            confidence_threshold_bps,
            resolution_window_ms,
            max_resolution_window_ms,
            payout_delay_ms,
            platform_fee_bps,
            ecosystem_fee_bps,
            min_betting_options,
            max_betting_options,
            min_reasoning_length,
            max_reasoning_length,
            max_evidence_urls,
            oracle_address,
            max_single_bet,
            max_bets_per_record,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// After upgrading from epoch-count semantics, multiply stored window fields by `epoch_duration_ms` so they become wall-clock durations.
    public entry fun rescale_spot_config_windows_from_epoch_counts(
        _: &SpotAdminCap,
        config: &mut SpotConfig,
        epoch_duration_ms: u64,
    ) {
        assert!(epoch_duration_ms > 0, EInvalidAmount);
        config.resolution_window_ms = config.resolution_window_ms * epoch_duration_ms;
        config.max_resolution_window_ms = config.max_resolution_window_ms * epoch_duration_ms;
    }

    /// Oracle-only: fix record timestamps/window fields after upgrade (off-chain supplies correct `created_at_ms` and optional windows in ms).
    public entry fun patch_spot_record_times_for_migration(
        _: &SpotOracleAdminCap,
        record: &mut SpotRecord,
        created_at_ms: u64,
        resolution_window_ms: Option<u64>,
        max_resolution_window_ms: Option<u64>,
        last_resolution_at_ms: u64,
    ) {
        record.created_at_ms = created_at_ms;
        record.resolution_window_ms = resolution_window_ms;
        record.max_resolution_window_ms = max_resolution_window_ms;
        record.last_resolution_at_ms = last_resolution_at_ms;
    }

    // Create a SPoT record for a post
    public entry fun create_spot_record_for_post(
        _: &SpotOracleAdminCap,
        config: &SpotConfig,
        post: &mut Post,
        betting_options: vector<String>,
        resolution_window_ms: Option<u64>,
        max_resolution_window_ms: Option<u64>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(config.enable_flag, EDisabled);
        
        // Verify SPoT is enabled for this post
        assert!(social_contracts::post::is_spot_enabled(post), EDisabled);
        
        // Validate betting options
        let options_len = vector::length(&betting_options);
        assert!(options_len >= config.min_betting_options, EInvalidAmount);
        assert!(options_len <= config.max_betting_options, EInvalidAmount);
        
        // Check for duplicate options (case-sensitive comparison)
        let mut i = 0;
        while (i < options_len) {
            let option_i = vector::borrow(&betting_options, i);
            let mut j = i + 1;
            while (j < options_len) {
                let option_j = vector::borrow(&betting_options, j);
                assert!(*option_i != *option_j, EDuplicateOption);
                j = j + 1;
            };
            i = i + 1;
        };
        
        let record = SpotRecord {
            id: object::new(ctx),
            post_id: post::get_id_address(post),
            created_at_ms: clock::timestamp_ms(clock),
            status: STATUS_OPEN,
            outcome: option::none(),
            escrow: balance::zero(),
            betting_options,
            option_escrow: table::new(ctx),
            user_option_amounts: table::new(ctx),
            bets: vector::empty<SpotBet>(),
            resolution_window_ms,
            max_resolution_window_ms,
            last_resolution_at_ms: 0,
            resolution_timestamp_ms: 0,
            pending_payouts: table::new(ctx),
            active_proposal_id: option::none(),
            oracle_proposed_outcome: option::none(),
            proposed_outcome: option::none(),
            dao_escalated_at_ms: 0,
            version: upgrade::current_version(),
        };
        let record_id = object::uid_to_address(&record.id);
        let created_at_ms = record.created_at_ms;
        let post_id = record.post_id;
        let betting_options_copy = record.betting_options;
        let resolution_window = record.resolution_window_ms;
        let max_resolution_window = record.max_resolution_window_ms;
        
        // Store SPoT record ID in post
        social_contracts::post::set_spot_id(post, record_id);
        
        transfer::share_object(record);
        
        // Emit record created event
        event::emit(SpotRecordCreatedEvent {
            record_id,
            post_id,
            created_at_ms,
            betting_options: betting_options_copy,
            resolution_window_ms: resolution_window,
            max_resolution_window_ms: max_resolution_window,
        });
    }

    /// Withdraw a bet before resolution
    /// Applies same fee structure as payouts
    /// Only allowed when status is OPEN (not DAO_REQUIRED, not RESOLVED, not REFUNDABLE)
    public entry fun withdraw_spot_bet(
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        platform: &mut Platform,
        treasury: &EcosystemTreasury,
        bet_index: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(spot_config.enable_flag, EDisabled);
        // Only allow withdrawal when status is OPEN (not DAO_REQUIRED or RESOLVED)
        assert!(record.status == STATUS_OPEN, EWithdrawalNotAllowed);
        
        let bets_len = vector::length(&record.bets);
        assert!(bet_index < bets_len, EBetNotFound);
        
        // Copy bet data before mutating vector
        let bet = *vector::borrow(&record.bets, bet_index);
        assert!(bet.user == tx_context::sender(ctx), EInvalidAmount); // User must own the bet
        assert!(bet.amount > 0, EInvalidAmount);
        
        // Calculate fee (same as payout fee structure)
        let platform_fee = (bet.amount * spot_config.platform_fee_bps) / 10000;
        let ecosystem_fee = (bet.amount * spot_config.ecosystem_fee_bps) / 10000;
        let fee = platform_fee + ecosystem_fee;
        let refund_amount = bet.amount - platform_fee - ecosystem_fee;
        
        // Split fee between platform and ecosystem treasury
        if (fee > 0) {
            let mut fee_coin = coin::from_balance(balance::split(&mut record.escrow, fee), ctx);
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_coin = coin::split(&mut fee_coin, platform_fee, ctx);
                platform::add_to_treasury(platform, &mut platform_coin, platform_fee, clock, ctx);
                coin::destroy_zero(platform_coin);
            };
            
            // Send ecosystem treasury fee
            if (ecosystem_fee > 0) {
                transfer::public_transfer(fee_coin, profile::get_treasury_address(treasury));
            } else {
                coin::destroy_zero(fee_coin);
            };
        };
        
        // Refund remaining amount to user
        if (refund_amount > 0) {
            let refund_coin = coin::from_balance(balance::split(&mut record.escrow, refund_amount), ctx);
            transfer::public_transfer(refund_coin, bet.user);
        };
        
        // Update option escrow table
        let option_id = bet.option_id;
        if (table::contains(&record.option_escrow, option_id)) {
            let current_escrow = *table::borrow(&record.option_escrow, option_id);
            if (current_escrow >= bet.amount) {
                let escrow_ref = table::borrow_mut(&mut record.option_escrow, option_id);
                *escrow_ref = current_escrow - bet.amount;
            };
        };

        if (table::contains(&record.user_option_amounts, bet.user)) {
            let user_amounts = table::borrow_mut(&mut record.user_option_amounts, bet.user);
            let idx = bet.option_id as u64;
            if (idx < vector::length(user_amounts)) {
                let current_user_amount = *vector::borrow(user_amounts, idx);
                if (current_user_amount >= bet.amount) {
                    let user_amount_ref = vector::borrow_mut(user_amounts, idx);
                    *user_amount_ref = current_user_amount - bet.amount;
                };
            };
        };
        
        // Remove bet from vector (swap with last and pop)
        let last_index = bets_len - 1;
        if (bet_index != last_index) {
            let last_bet = *vector::borrow(&record.bets, last_index);
            let bet_ref = vector::borrow_mut(&mut record.bets, bet_index);
            *bet_ref = last_bet;
        };
        vector::pop_back(&mut record.bets);
        
        // Emit withdrawal event
        event::emit(SpotBetWithdrawnEvent {
            post_id: post::get_id_address(post),
            user: bet.user,
            option_id: bet.option_id,
            amount: bet.amount,
            fee_taken: fee,
        });
    }

    /// Place bet - all funds go to escrow
    public entry fun place_spot_bet(
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        mut payment: Coin<MYSO>,
        option_id: u8,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(spot_config.enable_flag, EDisabled);
        assert!(record.status == STATUS_OPEN, EDaoDebateFrozen);
        assert!(amount > 0, EInvalidAmount);
        if (spot_config.max_single_bet > 0) { assert!(amount <= spot_config.max_single_bet, EInvalidAmount); };
        assert!(coin::value(&payment) >= amount, EInvalidAmount);
        
        // Per-record bet count cap; `max_bets_per_record == 0` means unlimited (see module docs).
        if (spot_config.max_bets_per_record > 0) {
            let current_bets = vector::length(&record.bets);
            assert!(current_bets < spot_config.max_bets_per_record, ETooManyBets);
        };
        
        // Validate option_id exists
        let options_len = vector::length(&record.betting_options);
        assert!((option_id as u64) < options_len, EInvalidOptionId);

        // All funds go to escrow
        let bet_coin = coin::split(&mut payment, amount, ctx);
        balance::join(&mut record.escrow, coin::into_balance(bet_coin));

        // Update escrow totals with overflow protection
        let current_escrow = if (table::contains(&record.option_escrow, option_id)) {
            *table::borrow(&record.option_escrow, option_id)
        } else {
            0
        };
        assert!(current_escrow <= MAX_U64 - amount, EOverflow);
        if (table::contains(&record.option_escrow, option_id)) {
            let escrow_ref = table::borrow_mut(&mut record.option_escrow, option_id);
            *escrow_ref = current_escrow + amount;
        } else {
            table::add(&mut record.option_escrow, option_id, amount);
        };

        // Refund any excess
        if (coin::value(&payment) > 0) { 
            transfer::public_transfer(payment, tx_context::sender(ctx)); 
        } else { 
            coin::destroy_zero(payment); 
        };

        let ts = clock::timestamp_ms(clock);
        // Record bet
        vector::push_back(&mut record.bets, SpotBet {
            user: tx_context::sender(ctx),
            option_id,
            amount,
            timestamp_ms: ts,
        });

        let user = tx_context::sender(ctx);
        let options_len = vector::length(&record.betting_options);
        if (!table::contains(&record.user_option_amounts, user)) {
            let mut amounts = vector::empty<u64>();
            let mut i = 0;
            while (i < options_len) {
                vector::push_back(&mut amounts, 0);
                i = i + 1;
            };
            table::add(&mut record.user_option_amounts, user, amounts);
        };
        let user_amounts = table::borrow_mut(&mut record.user_option_amounts, user);
        let idx = option_id as u64;
        let current_user_amount = *vector::borrow(user_amounts, idx);
        assert!(current_user_amount <= MAX_U64 - amount, EOverflow);
        let user_amount_ref = vector::borrow_mut(user_amounts, idx);
        *user_amount_ref = current_user_amount + amount;

        event::emit(SpotBetPlacedEvent {
            post_id: post::get_id_address(post),
            user: tx_context::sender(ctx),
            option_id,
            amount,
            timestamp_ms: ts,
        });
    }

    /// Oracle resolution (option_id, or too close → DAO_REQUIRED)
    /// Requires reasoning and at least one evidence URL for transparency and accountability
    public entry fun oracle_resolve(
        _: &SpotOracleAdminCap,
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
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
        // Prevent resolving already resolved or refundable markets
        assert!(record.status == STATUS_OPEN, EWrongStatus);
        assert!(option::is_none(&record.outcome), EAlreadyResolved);
        
        // Enforce resolution window if specified
        let now_ms = clock::timestamp_ms(clock);
        if (option::is_some(&record.resolution_window_ms)) {
            let window = *option::borrow(&record.resolution_window_ms);
            assert!(now_ms >= record.created_at_ms + window, ETooEarly);
        };
        
        // Validate outcome_option_id exists
        let options_len = vector::length(&record.betting_options);
        assert!((outcome_option_id as u64) < options_len, EInvalidOptionId);

        // Validate reasoning is required and within limits
        let reasoning_len = string::length(&reasoning);
        assert!(reasoning_len >= spot_config.min_reasoning_length, EInvalidReasoning);
        assert!(reasoning_len <= spot_config.max_reasoning_length, EInvalidReasoning);
        
        // Validate evidence URLs - at least one required
        let evidence_urls_len = vector::length(&evidence_urls);
        assert!(evidence_urls_len > 0, EInvalidAmount); // At least one evidence URL required
        assert!(evidence_urls_len <= spot_config.max_evidence_urls, EInvalidAmount);

        if (confidence_bps < spot_config.confidence_threshold_bps) {
            assert!(option::is_none(&record.active_proposal_id), EActiveProposalExists);
            record.status = STATUS_DAO_REQUIRED;
            record.oracle_proposed_outcome = option::some(outcome_option_id);
            record.dao_escalated_at_ms = now_ms;
            event::emit(SpotDaoRequiredEvent {
                post_id: post::get_id_address(post),
                spot_record_id: object::uid_to_address(&record.id),
                confidence_bps,
                oracle_proposed_outcome: outcome_option_id,
                dao_escalated_at_ms: now_ms,
                reasoning,
            });
            return
        };

        // Resolve outcome - outcome_option_id is the winning option
        // Convert required vector to Option for internal function
        finalize_resolution_and_payout(spot_config, record, post, platform, treasury, outcome_option_id, reasoning, option::some(evidence_urls), clock, ctx);
    }

    /// Submit a governance proposal to ratify one outcome for a contested SPoT market.
    public entry fun submit_spot_resolution_proposal_to_governance(
        spot_config: &SpotConfig,
        registry: &mut GovernanceDAO,
        record: &mut SpotRecord,
        post: &Post,
        title: String,
        description: String,
        proposed_outcome: u8,
        metadata_json: Option<String>,
        coin: &mut Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_spot_governance_registry(spot_config, registry);
        assert!(record.status == STATUS_DAO_REQUIRED, ENotDaoRequired);
        assert!(option::is_none(&record.active_proposal_id), EActiveProposalExists);
        validate_proposed_outcome(record, proposed_outcome);

        let spot_record_id = object::id(record);
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

        record.active_proposal_id = option::some(proposal_id);
        record.proposed_outcome = option::some(proposed_outcome);

        event::emit(SpotGovernanceProposalLinkedEvent {
            post_id: post::get_id_address(post),
            spot_record_id: object::uid_to_address(&record.id),
            proposal_id,
            proposed_outcome,
        });
    }

    /// After community voting approves a linked proposal, resolve the market and pay winners.
    public entry fun implement_spot_resolution_from_governance(
        spot_config: &SpotConfig,
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        record: &mut SpotRecord,
        post: &Post,
        platform: &mut Platform,
        treasury: &EcosystemTreasury,
        reasoning: String,
        evidence_urls: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_spot_governance_registry(spot_config, registry);
        assert!(record.status == STATUS_DAO_REQUIRED, ENotDaoRequired);
        assert!(option::is_some(&record.active_proposal_id), ENoActiveProposal);
        assert!(option::is_some(&record.proposed_outcome), EWrongProposal);
        let active_id = *option::borrow(&record.active_proposal_id);
        assert!(active_id == object::id(proposal), EWrongProposal);
        assert!(
            governance::proposal_status(proposal) == governance::status_approved_value(),
            EProposalNotApproved
        );

        let outcome = *option::borrow(&record.proposed_outcome);
        validate_proposed_outcome(record, outcome);

        let reasoning_len = string::length(&reasoning);
        assert!(reasoning_len >= spot_config.min_reasoning_length, EInvalidReasoning);
        assert!(reasoning_len <= spot_config.max_reasoning_length, EInvalidReasoning);
        if (option::is_some(&evidence_urls)) {
            let urls = option::borrow(&evidence_urls);
            assert!(vector::length(urls) <= spot_config.max_evidence_urls, EInvalidAmount);
        };

        let submitter = governance::proposal_submitter(proposal);
        let bal = governance::mark_proposal_implemented_take_pool(
            registry,
            proposal,
            option::none(),
            clock,
            ctx,
        );
        let amount = balance::value(&bal);
        if (amount > 0) {
            let c = coin::from_balance(bal, ctx);
            transfer::public_transfer(c, submitter);
        } else {
            balance::destroy_zero(bal);
        };

        let proposal_id = active_id;
        record.active_proposal_id = option::none();
        record.proposed_outcome = option::none();

        finalize_resolution_and_payout(
            spot_config,
            record,
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
            spot_record_id: object::uid_to_address(&record.id),
            proposal_id,
        });
    }

    /// Clear the active proposal link after a rejected or quorum-failed governance outcome.
    public entry fun clear_spot_proposal_link_on_reject(
        spot_config: &SpotConfig,
        registry: &GovernanceDAO,
        proposal: &Proposal,
        record: &mut SpotRecord,
        post: &Post,
    ) {
        assert_spot_governance_registry(spot_config, registry);
        assert!(record.status == STATUS_DAO_REQUIRED, ENotDaoRequired);
        assert!(option::is_some(&record.active_proposal_id), ENoActiveProposal);
        let active_id = *option::borrow(&record.active_proposal_id);
        assert!(active_id == object::id(proposal), EWrongProposal);
        let status = governance::proposal_status(proposal);
        assert!(
            status == governance::status_rejected_value(),
            EProposalNotApproved
        );

        let proposal_id = active_id;
        record.active_proposal_id = option::none();
        record.proposed_outcome = option::none();

        event::emit(SpotGovernanceProposalClearedEvent {
            post_id: post::get_id_address(post),
            spot_record_id: object::uid_to_address(&record.id),
            proposal_id,
        });
    }

    /// Finalize linked SPoT governance voting; clears the record link when the proposal is rejected.
    public entry fun finalize_spot_governance_proposal(
        spot_config: &SpotConfig,
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        record: &mut SpotRecord,
        post: &Post,
        ecosystem_treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert_spot_governance_registry(spot_config, registry);
        governance::finalize_proposal(registry, proposal, ecosystem_treasury, clock, ctx);
        if (governance::proposal_status(proposal) == governance::status_rejected_value()) {
            clear_spot_proposal_link_on_reject(spot_config, registry, proposal, record, post);
        };
    }

    /// Deprecated direct DAO finalization — requires an approved linked governance proposal.
    public entry fun finalize_via_dao(
        spot_config: &SpotConfig,
        registry: &mut GovernanceDAO,
        proposal: &mut Proposal,
        record: &mut SpotRecord,
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
            record,
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

    fun validate_proposed_outcome(record: &SpotRecord, outcome: u8) {
        if (outcome == OUTCOME_DRAW || outcome == OUTCOME_UNAPPLICABLE) {
            return
        };
        let options_len = vector::length(&record.betting_options);
        assert!((outcome as u64) < options_len, EInvalidOptionId);
    }

    /// Refund all escrow if unresolved beyond max window
    /// Requires SpotOracleAdminCap authorization
    /// If max_resolution_window_ms is None, this function cannot be called (must be explicitly set)
    public entry fun refund_unresolved(
        _: &SpotOracleAdminCap,
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Require max_resolution_window_ms to be Some - prevents permissionless abuse
        assert!(option::is_some(&record.max_resolution_window_ms), EInvalidAmount);
        
        let now_ms = clock::timestamp_ms(clock);
        let max_window = *option::borrow(&record.max_resolution_window_ms);
        assert!(now_ms >= record.created_at_ms + max_window, ETooEarly);
        
        assert!(record.status == STATUS_OPEN || record.status == STATUS_DAO_REQUIRED, EWrongStatus);
        assert!(vector::length(&record.bets) > 0, ENoBets);

        // Iterate all bets and refund escrow
        let mut i = 0;
        let len = vector::length(&record.bets);
        while (i < len) {
            let bet = vector::borrow(&record.bets, i);
            if (bet.amount > 0) {
                let c = coin::from_balance(balance::split(&mut record.escrow, bet.amount), ctx);
                transfer::public_transfer(c, bet.user);
                event::emit(SpotRefundEvent { post_id: record.post_id, user: bet.user, amount: bet.amount });
            };
            i = i + 1;
        };
        record.status = STATUS_REFUNDABLE;
        record.outcome = option::none();
        record.last_resolution_at_ms = now_ms;
        // Any dust stays in escrow balance if math rounding occurred
    }

    // Internal: finalize with payouts and fees
    fun finalize_resolution_and_payout(
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        platform: &mut Platform,
        treasury: &EcosystemTreasury,
        outcome: u8, // Winning option_id, or OUTCOME_DRAW/OUTCOME_UNAPPLICABLE
        reasoning: String,
        evidence_urls: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(record.status == STATUS_OPEN || record.status == STATUS_DAO_REQUIRED, EWrongStatus);
        assert!(vector::length(&record.bets) > 0, ENoBets);

        // Calculate total escrow across all options
        let mut total_escrow = 0;
        let mut i = 0;
        let options_len = vector::length(&record.betting_options);
        while (i < options_len) {
            let option_id = (i as u8);
            if (table::contains(&record.option_escrow, option_id)) {
                total_escrow = total_escrow + *table::borrow(&record.option_escrow, option_id);
            };
            i = i + 1;
        };

        let now_ms = clock::timestamp_ms(clock);

        // Handle DRAW/UNAPPLICABLE: refund all escrow
        if (outcome == OUTCOME_DRAW || outcome == OUTCOME_UNAPPLICABLE) {
            let mut i = 0; let len = vector::length(&record.bets);
            while (i < len) {
                let bet = vector::borrow(&record.bets, i);
                if (bet.amount > 0) {
                    let c = coin::from_balance(balance::split(&mut record.escrow, bet.amount), ctx);
                    transfer::public_transfer(c, bet.user);
                    event::emit(SpotRefundEvent { post_id: record.post_id, user: bet.user, amount: bet.amount });
                };
                i = i + 1;
            };
            record.status = STATUS_RESOLVED;
            record.outcome = option::some(outcome);
            record.last_resolution_at_ms = now_ms;
            record.resolution_timestamp_ms = now_ms;
            // Convert Option to vector for event (use empty vector if None)
            let evidence_urls_vec = if (option::is_some(&evidence_urls)) {
                *option::borrow(&evidence_urls)
            } else {
                vector::empty<String>()
            };
            event::emit(SpotResolvedEvent { 
                post_id: post::get_id_address(post), 
                outcome, 
                total_escrow, 
                fee_taken: 0,
                reasoning,
                evidence_urls: evidence_urls_vec,
            });
            return
        };

        // Get winning option escrow total
        let winning_total = if (table::contains(&record.option_escrow, outcome)) {
            *table::borrow(&record.option_escrow, outcome)
        } else {
            0
        };

        // Fees on payouts (apply to total escrow)
        let platform_fee = (total_escrow * spot_config.platform_fee_bps) / 10000;
        let ecosystem_fee = (total_escrow * spot_config.ecosystem_fee_bps) / 10000;
        let fee = platform_fee + ecosystem_fee;
        let distributable = total_escrow - platform_fee - ecosystem_fee;

        // Split fee between platform and ecosystem treasury (configurable)
        if (fee > 0) {
            let mut fee_coin = coin::from_balance(balance::split(&mut record.escrow, fee), ctx);
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_coin = coin::split(&mut fee_coin, platform_fee, ctx);
                platform::add_to_treasury(platform, &mut platform_coin, platform_fee, clock, ctx);
                coin::destroy_zero(platform_coin);
            };
            
            // Send ecosystem treasury fee
            if (ecosystem_fee > 0) {
                transfer::public_transfer(fee_coin, profile::get_treasury_address(treasury));
            } else {
                coin::destroy_zero(fee_coin);
            };
        };

        // Calculate and store pending payouts for winners (pro-rata of winning option escrow)
        // Payouts will be claimable after payout_delay_ms
        let mut i = 0; let len = vector::length(&record.bets);
        while (i < len) {
            let bet = vector::borrow(&record.bets, i);
            let winner = bet.option_id == outcome;
            if (winner && winning_total > 0 && bet.amount > 0) {
                let payout = (((bet.amount as u128) * (distributable as u128)) / (winning_total as u128)) as u64;
                if (payout > 0) {
                    // Store payout in pending_payouts table (funds remain in escrow)
                    if (table::contains(&record.pending_payouts, bet.user)) {
                        let current_payout = *table::borrow(&record.pending_payouts, bet.user);
                        let payout_ref = table::borrow_mut(&mut record.pending_payouts, bet.user);
                        *payout_ref = current_payout + payout;
                    } else {
                        table::add(&mut record.pending_payouts, bet.user, payout);
                    };
                };
            };
            i = i + 1;
        };

        record.status = STATUS_RESOLVED;
        record.outcome = option::some(outcome);
        record.last_resolution_at_ms = now_ms;
        record.resolution_timestamp_ms = now_ms;
        // Convert Option to vector for event (use empty vector if None)
        let evidence_urls_vec = if (option::is_some(&evidence_urls)) {
            *option::borrow(&evidence_urls)
        } else {
            vector::empty<String>()
        };
        event::emit(SpotResolvedEvent { 
            post_id: post::get_id_address(post), 
            outcome, 
            total_escrow, 
            fee_taken: fee,
            reasoning,
            evidence_urls: evidence_urls_vec,
        });
    }

    /// Claim payout after delay period has passed
    /// Users can claim their winnings after payout_delay_ms has elapsed since resolution
    public entry fun claim_payout(
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(spot_config.enable_flag, EDisabled);
        assert!(record.status == STATUS_RESOLVED, EWrongStatus);
        assert!(option::is_some(&record.outcome), ENotOracle);
        
        let user = tx_context::sender(ctx);
        assert!(table::contains(&record.pending_payouts, user), EBetNotFound);
        
        let pending_amount = *table::borrow(&record.pending_payouts, user);
        assert!(pending_amount > 0, EInvalidAmount);
        
        // Check if delay period has passed
        let current_time = clock::timestamp_ms(clock);
        assert!(record.resolution_timestamp_ms > 0, EInvalidAmount); // Must be resolved
        assert!(current_time >= record.resolution_timestamp_ms + spot_config.payout_delay_ms, ETooEarly);
        
        // Transfer payout from escrow
        let payout_coin = coin::from_balance(balance::split(&mut record.escrow, pending_amount), ctx);
        transfer::public_transfer(payout_coin, user);
        
        // Remove from pending payouts
        table::remove(&mut record.pending_payouts, user);
        
        // Emit payout event
        event::emit(SpotPayoutEvent {
            post_id: post::get_id_address(post),
            user,
            amount: pending_amount,
        });
    }

    /// Migration function for SpotConfig
    public entry fun migrate_config(
        config: &mut SpotConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(config.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = config.version;
        if (old_version == 0) {
            config.spot_governance_registry_id = object::id_from_address(@0x0);
        };
        config.version = current_version;
        
        // Emit event for object migration
        let config_id = object::id(config);
        upgrade::emit_migration_event(
            config_id,
            string::utf8(b"SpotConfig"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Migration function for SpotRecord
    public entry fun migrate_record(
        record: &mut SpotRecord,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(record.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = record.version;
        if (old_version == 0) {
            record.active_proposal_id = option::none();
            record.oracle_proposed_outcome = option::none();
            record.proposed_outcome = option::none();
            record.dao_escalated_at_ms = 0;
        };
        record.version = current_version;
        
        // Emit event for object migration
        let record_id = object::id(record);
        upgrade::emit_migration_event(
            record_id,
            string::utf8(b"SpotRecord"),
            old_version,
            tx_context::sender(ctx)
        );
    }

}
