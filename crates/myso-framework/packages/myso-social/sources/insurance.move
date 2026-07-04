// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Insurance module for SPoT positions
/// Sells coverage against losing outcomes and pays out deterministically on SPoT resolution.

#[allow(duplicate_alias, unused_use, unused_const, unused_variable, lint(self_transfer, share_owned, public_entry))]
module social_contracts::insurance {
    use std::option::{Self, Option};
    use std::vector;
    use std::string;

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        clock::{Self, Clock},
        coin::{Self, Coin},
        balance::{Self, Balance},
        table::{Self, Table},
        event,
    };
    use myso::myso::MYSO;

    use social_contracts::social_proof_of_truth as spot;
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::profile::{Self, EcosystemTreasury};

    /// Errors
    const ENotAdmin: u64 = 1;
    const EDisabled: u64 = 2;
    const EInvalidCoverage: u64 = 3;
    const EInvalidDuration: u64 = 4;
    const EInvalidAmount: u64 = 5;
    const EInvalidVault: u64 = 6;
    const EInsufficientCapital: u64 = 7;
    const EMarketClosed: u64 = 8;
    const EPolicyNotActive: u64 = 9;
    const EPolicyExpired: u64 = 10;
    const ENotPolicyOwner: u64 = 11;
    const EOverflow: u64 = 12;
    const EMarketMismatch: u64 = 13;
    const EExposureLimit: u64 = 14;
    const EInsufficientPremium: u64 = 15;
    const EExposureInvariantBroken: u64 = 16;
    const EWrongVersion: u64 = 17;
    const EThinMarket: u64 = 18;
    const ECoverageTooLargeVersusPool: u64 = 19;
    const ERiskMultiplierTooHigh: u64 = 20;
    const EVaultDisabled: u64 = 21;
    const EVaultPaused: u64 = 22;
    const ERouterPaused: u64 = 23;
    const ERouteDisabled: u64 = 24;
    const EDeadlinePassed: u64 = 25;
    const ESlippagePremium: u64 = 26;
    const ESlippageCovered: u64 = 27;
    const EDuplicateVaultInRoute: u64 = 28;
    const EBackstopPaused: u64 = 29;
    const ETailModeDisabled: u64 = 30;
    const EBackstopPayoutLimit: u64 = 31;
    const EInvalidFills: u64 = 32;
    const EVaultConcentration: u64 = 33;

    /// Status
    const STATUS_ACTIVE: u8 = 1;
    const STATUS_CANCELLED: u8 = 2;
    const STATUS_CLAIMED: u8 = 3;
    const STATUS_EXPIRED: u8 = 4;

    /// Constants
    const BPS_DENOM: u64 = 10_000;
    const DAY_MS: u64 = 86_400_000;
    const MAX_U64: u64 = 18446744073709551615;
    const DEFAULT_VERSION: u64 = 1;
    const DEFAULT_MIN_COVERAGE_BPS: u64 = 1000;
    const DEFAULT_MAX_COVERAGE_BPS: u64 = 9000;
    const DEFAULT_MAX_DURATION_MS: u64 = 30 * DAY_MS;
    const DEFAULT_FEE_BPS: u64 = 50;
    const DEFAULT_ODDS_BASE_BPS: u64 = 5000;
    const DEFAULT_MAX_ROUTE_LEGS: u64 = 4;

    /// Default SPoT risk pricing (baseline pool size ~1000 MYSO at 10^9 scaling).
    const DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY: u64 = 1;
    const DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS: u64 = 10_000;
    const DEFAULT_MAX_RISK_MULTIPLIER_BPS: u64 = 500_000;
    const DEFAULT_MIN_PREMIUM_AMOUNT: u64 = 1;
    const DEFAULT_SPOT_SMOOTHING_PER_OPTION: u64 = 0;
    const DEFAULT_IMPLIED_PROB_FLOOR_BPS: u64 = 10;
    const DEFAULT_ODDS_CAP_BPS: u64 = 500_000;
    const DEFAULT_LIQ_CAP_BPS: u64 = 500_000;
    /// Target pool size such that liquidity multiplier ≈ 1× when `total_option_escrow == liq_ref_amount`.
    const DEFAULT_LIQ_REF_AMOUNT: u64 = 1_000_000_000_000;
    const DEFAULT_EXPOSURE_CAP_BPS: u64 = 30_000;
    const DEFAULT_EXPOSURE_K_BPS: u64 = 5000;

    public struct InsuranceAdminCap has key, store {
        id: UID,
    }

    public struct InsuranceConfig has key {
        id: UID,
        enable_flag: bool,
        min_coverage_bps: u64,
        max_coverage_bps: u64,
        max_duration_ms: u64,
        fee_bps: u64,
        min_spot_total_liquidity: u64,
        max_coverage_fraction_of_option_bps: u64,
        max_risk_multiplier_bps: u64,
        min_premium_amount: u64,
        spot_smoothing_per_option: u64,
        implied_prob_floor_bps: u64,
        odds_floor_1x: bool,
        odds_cap_bps: u64,
        liq_cap_bps: u64,
        liq_ref_amount: u64,
        exposure_cap_bps: u64,
        exposure_k_bps: u64,
        odds_base_bps: u64,
        version: u64,
    }

    public struct InsuranceRouterConfig has key {
        id: UID,
        router_enabled: bool,
        router_paused: bool,
        max_route_reserve_market: u64,
        max_route_reserve_user: u64,
        max_route_reserve_option: u64,
        max_vault_concentration_bps: u64,
        min_vault_health_factor_bps: u64,
        max_route_legs: u64,
        market_pause: Table<address, bool>,
        version: u64,
    }

    public struct InsuranceBackstopPool has key {
        id: UID,
        capital: Balance<MYSO>,
        total_paid_out: u64,
        paid_by_market: Table<address, u64>,
        max_payout_per_market: u64,
        max_payout_per_event: u64,
        global_hard_cap: u64,
        tail_mode_enabled: bool,
        paused: bool,
        sweep_premium_bps: u64,
        tail_pay_partial_on_cap: bool,
        version: u64,
    }

    public struct CoverageRoute has key {
        id: UID,
        insured: address,
        market_id: address,
        option_id: u8,
        coverage_bps: u64,
        start_time_ms: u64,
        expiry_time_ms: u64,
        policy_ids: vector<ID>,
        vault_ids: vector<ID>,
        total_covered: u64,
        total_premium: u64,
        total_reserve: u64,
        total_backstop_sweep: u64,
        version: u64,
    }

    public struct UnderwriterVault has key {
        id: UID,
        underwriter: address,
        capital: Balance<MYSO>,
        reserved: u64,
        base_rate_bps_per_day: u64,
        utilization_multiplier_bps: u64,
        max_exposure_per_market: u64,
        max_exposure_per_user: u64,
        max_exposure_per_option: u64,
        enabled: bool,
        paused: bool,
        market_exposures: Table<address, MarketExposure>,
        user_exposures: Table<address, u64>,
        version: u64,
    }

    public struct MarketExposure has store {
        market_id: address,
        total_reserved: u64,
        reserved_by_option: Table<u8, u64>,
    }

    public struct CoveragePolicy has key {
        id: UID,
        market_id: address,
        insured: address,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        premium_paid: u64,
        start_time_ms: u64,
        expiry_time_ms: u64,
        vault_id: ID,
        status: u8,
        route_id: Option<ID>,
        route_leg_index: u8,
    }

    public struct PremiumQuote has copy, drop {
        premium: u64,
        premium_raw: u64,
        implied_prob_win_bps: u64,
        risk_multiplier_bps: u64,
        market_total_amount: u64,
        option_amount: u64,
        base_premium: u64,
    }

    public struct VaultCoverageQuote has copy, drop {
        premium: u64,
        premium_raw: u64,
        reserve_required: u64,
        available_capacity_reserve: u64,
        risk_multiplier_bps: u64,
        implied_prob_win_bps: u64,
        utilization_bps: u64,
        max_fill_covered_amount: u64,
        skipped_reason: u8,
    }

    /// `VaultCoverageQuote.skipped_reason` — 0 = quotable at `max_fill_covered_amount`.
    const SKIPPED_OK: u8 = 0;
    const SKIPPED_VAULT_DISABLED: u8 = 1;
    const SKIPPED_VAULT_PAUSED: u8 = 2;
    const SKIPPED_ROUTER_PAUSED: u8 = 3;
    const SKIPPED_MARKET_PAUSED: u8 = 4;
    const SKIPPED_UNHEALTHY_VAULT: u8 = 5;
    const SKIPPED_RISK_MULTIPLIER: u8 = 6;
    const SKIPPED_ZERO_CAPACITY: u8 = 7;
    const SKIPPED_THIN_OR_POOL: u8 = 8;

    /// Events
    public struct RiskPricingConfigUpdatedEvent has copy, drop {
        updated_by: address,
        min_spot_total_liquidity: u64,
        max_coverage_fraction_of_option_bps: u64,
        max_risk_multiplier_bps: u64,
        min_premium_amount: u64,
        spot_smoothing_per_option: u64,
        implied_prob_floor_bps: u64,
        odds_floor_1x: bool,
        odds_cap_bps: u64,
        liq_cap_bps: u64,
        liq_ref_amount: u64,
        exposure_cap_bps: u64,
        exposure_k_bps: u64,
        timestamp: u64,
    }

    public struct ConfigInitializedEvent has copy, drop {
        admin: address,
        min_coverage_bps: u64,
        max_coverage_bps: u64,
        max_duration_ms: u64,
        fee_bps: u64,
    }

    public struct UnderwriterVaultCreatedEvent has copy, drop {
        vault_id: ID,
        underwriter: address,
        base_rate_bps_per_day: u64,
        utilization_multiplier_bps: u64,
        max_exposure_per_market: u64,
        max_exposure_per_user: u64,
        max_exposure_per_option: u64,
        enabled: bool,
        paused: bool,
    }

    public struct VaultStatusUpdatedEvent has copy, drop {
        vault_id: ID,
        enabled: bool,
        paused: bool,
        max_exposure_per_option: u64,
        max_exposure_per_market: u64,
        max_exposure_per_user: u64,
        base_rate_bps_per_day: u64,
        utilization_multiplier_bps: u64,
        updated_by: address,
        timestamp_ms: u64,
    }

    public struct UnderwriterVaultDepositedEvent has copy, drop {
        vault_id: ID,
        amount: u64,
        new_balance: u64,
    }

    public struct UnderwriterVaultWithdrawnEvent has copy, drop {
        vault_id: ID,
        amount: u64,
        new_balance: u64,
    }

    public struct CoveragePurchasedEvent has copy, drop {
        policy_id: ID,
        vault_id: ID,
        market_id: address,
        insured: address,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        premium_paid: u64,
        premium_raw: u64,
        reserve_locked: u64,
        expiry_time_ms: u64,
        implied_probability_bps: u64,
        risk_multiplier_bps: u64,
        base_premium: u64,
        market_total_amount: u64,
        option_amount: u64,
        backstop_sweep_amount: u64,
        route_id: Option<ID>,
        route_leg_index: u8,
    }

    public struct CoverageRoutedEvent has copy, drop {
        route_id: ID,
        insured: address,
        market_id: address,
        option_id: u8,
        coverage_bps: u64,
        duration_ms: u64,
        total_covered: u64,
        total_premium: u64,
        total_reserve: u64,
        total_backstop_sweep: u64,
        expiry_time_ms: u64,
        policy_ids: vector<ID>,
        vault_ids: vector<ID>,
    }

    public struct RouteFillEvent has copy, drop {
        route_id: ID,
        leg_index: u8,
        vault_id: ID,
        policy_id: ID,
        covered_amount: u64,
        premium_paid: u64,
        reserve_locked: u64,
        backstop_sweep_amount: u64,
    }

    public struct BackstopUsedEvent has copy, drop {
        market_id: address,
        recipient: address,
        amount: u64,
        total_paid_out_after: u64,
        tail_mode_enabled: bool,
    }

    public struct BackstopTreasuryDepositEvent has copy, drop {
        depositor: address,
        amount: u64,
        new_balance: u64,
    }

    public struct CoverageCancelledEvent has copy, drop {
        policy_id: ID,
        insured: address,
        refunded_amount: u64,
        fee_paid: u64,
    }

    public struct CoverageClaimedEvent has copy, drop {
        policy_id: ID,
        insured: address,
        payout: u64,
    }

    public struct ConfigUpdatedEvent has copy, drop {
        updated_by: address,
        enable_flag: bool,
        min_coverage_bps: u64,
        max_coverage_bps: u64,
        max_duration_ms: u64,
        fee_bps: u64,
        odds_base_bps: u64,
        timestamp: u64,
    }

    public struct RouterLimitsUpdatedEvent has copy, drop {
        updated_by: address,
        max_route_reserve_market: u64,
        max_route_reserve_user: u64,
        max_route_reserve_option: u64,
        max_vault_concentration_bps: u64,
        min_vault_health_factor_bps: u64,
        max_route_legs: u64,
        timestamp: u64,
    }

    public struct PolicyExpiredEvent has copy, drop {
        policy_id: ID,
        insured: address,
        market_id: address,
        vault_id: ID,
        reserve_released: u64,
        expiry_time_ms: u64,
    }

    /// Initialize config (package only)
    /// Creates InsuranceConfig and transfers InsuranceAdminCap to caller.
    public(package) fun init_config(
        min_coverage_bps: u64,
        max_coverage_bps: u64,
        max_duration_ms: u64,
        fee_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(min_coverage_bps > 0, EInvalidCoverage);
        assert!(min_coverage_bps <= max_coverage_bps, EInvalidCoverage);
        assert!(max_coverage_bps <= BPS_DENOM, EInvalidCoverage);
        assert!(max_duration_ms > 0, EInvalidDuration);
        assert!(fee_bps <= BPS_DENOM, EInvalidCoverage);

        let admin = tx_context::sender(ctx);
        let ts = clock::timestamp_ms(clock);
        transfer::share_object(InsuranceConfig {
            id: object::new(ctx),
            enable_flag: false,
            min_coverage_bps,
            max_coverage_bps,
            max_duration_ms,
            fee_bps,
            min_spot_total_liquidity: DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY,
            max_coverage_fraction_of_option_bps: DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS,
            max_risk_multiplier_bps: DEFAULT_MAX_RISK_MULTIPLIER_BPS,
            min_premium_amount: DEFAULT_MIN_PREMIUM_AMOUNT,
            spot_smoothing_per_option: DEFAULT_SPOT_SMOOTHING_PER_OPTION,
            implied_prob_floor_bps: DEFAULT_IMPLIED_PROB_FLOOR_BPS,
            odds_floor_1x: true,
            odds_cap_bps: DEFAULT_ODDS_CAP_BPS,
            liq_cap_bps: DEFAULT_LIQ_CAP_BPS,
            liq_ref_amount: DEFAULT_LIQ_REF_AMOUNT,
            exposure_cap_bps: DEFAULT_EXPOSURE_CAP_BPS,
            exposure_k_bps: DEFAULT_EXPOSURE_K_BPS,
            odds_base_bps: DEFAULT_ODDS_BASE_BPS,
            version: DEFAULT_VERSION,
        });
        transfer::share_object(new_router_config_defaults(ctx));
        transfer::share_object(new_backstop_pool_defaults(ctx));
        transfer::public_transfer(InsuranceAdminCap { id: object::new(ctx) }, admin);

        event::emit(ConfigInitializedEvent {
            admin,
            min_coverage_bps,
            max_coverage_bps,
            max_duration_ms,
            fee_bps,
        });
        event::emit(RiskPricingConfigUpdatedEvent {
            updated_by: admin,
            min_spot_total_liquidity: DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY,
            max_coverage_fraction_of_option_bps: DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS,
            max_risk_multiplier_bps: DEFAULT_MAX_RISK_MULTIPLIER_BPS,
            min_premium_amount: DEFAULT_MIN_PREMIUM_AMOUNT,
            spot_smoothing_per_option: DEFAULT_SPOT_SMOOTHING_PER_OPTION,
            implied_prob_floor_bps: DEFAULT_IMPLIED_PROB_FLOOR_BPS,
            odds_floor_1x: true,
            odds_cap_bps: DEFAULT_ODDS_CAP_BPS,
            liq_cap_bps: DEFAULT_LIQ_CAP_BPS,
            liq_ref_amount: DEFAULT_LIQ_REF_AMOUNT,
            exposure_cap_bps: DEFAULT_EXPOSURE_CAP_BPS,
            exposure_k_bps: DEFAULT_EXPOSURE_K_BPS,
            timestamp: ts,
        });
    }

    /// Update config (admin only)
    public entry fun set_config(
        _: &InsuranceAdminCap,
        config: &mut InsuranceConfig,
        min_coverage_bps: u64,
        max_coverage_bps: u64,
        max_duration_ms: u64,
        fee_bps: u64,
        odds_base_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(min_coverage_bps > 0, EInvalidCoverage);
        assert!(min_coverage_bps <= max_coverage_bps, EInvalidCoverage);
        assert!(max_coverage_bps <= BPS_DENOM, EInvalidCoverage);
        assert!(max_duration_ms > 0, EInvalidDuration);
        assert!(fee_bps <= BPS_DENOM, EInvalidCoverage);
        assert!(odds_base_bps > 0, EInvalidCoverage);

        config.min_coverage_bps = min_coverage_bps;
        config.max_coverage_bps = max_coverage_bps;
        config.max_duration_ms = max_duration_ms;
        config.fee_bps = fee_bps;
        config.odds_base_bps = odds_base_bps;

        let updated_by = tx_context::sender(ctx);
        let timestamp = clock::timestamp_ms(clock);
        event::emit(ConfigUpdatedEvent {
            updated_by,
            enable_flag: config.enable_flag,
            min_coverage_bps,
            max_coverage_bps,
            max_duration_ms,
            fee_bps,
            odds_base_bps,
            timestamp,
        });
    }

    /// Update SPoT-linked risk pricing (admin only).
    public entry fun set_risk_pricing_config(
        _: &InsuranceAdminCap,
        config: &mut InsuranceConfig,
        min_spot_total_liquidity: u64,
        max_coverage_fraction_of_option_bps: u64,
        max_risk_multiplier_bps: u64,
        min_premium_amount: u64,
        spot_smoothing_per_option: u64,
        implied_prob_floor_bps: u64,
        odds_floor_1x: bool,
        odds_cap_bps: u64,
        liq_cap_bps: u64,
        liq_ref_amount: u64,
        exposure_cap_bps: u64,
        exposure_k_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(
            max_coverage_fraction_of_option_bps > 0 && max_coverage_fraction_of_option_bps <= BPS_DENOM,
            EInvalidCoverage
        );
        assert!(
            exposure_cap_bps >= BPS_DENOM && odds_cap_bps >= BPS_DENOM && liq_cap_bps >= BPS_DENOM,
            EInvalidCoverage
        );
        assert!(
            implied_prob_floor_bps > 0 && implied_prob_floor_bps <= BPS_DENOM,
            EInvalidCoverage
        );
        assert!(max_risk_multiplier_bps >= BPS_DENOM, EInvalidCoverage);
        assert!(min_premium_amount > 0, EInvalidAmount);

        config.min_spot_total_liquidity = min_spot_total_liquidity;
        config.max_coverage_fraction_of_option_bps = max_coverage_fraction_of_option_bps;
        config.max_risk_multiplier_bps = max_risk_multiplier_bps;
        config.min_premium_amount = min_premium_amount;
        config.spot_smoothing_per_option = spot_smoothing_per_option;
        config.implied_prob_floor_bps = implied_prob_floor_bps;
        config.odds_floor_1x = odds_floor_1x;
        config.odds_cap_bps = odds_cap_bps;
        config.liq_cap_bps = liq_cap_bps;
        config.liq_ref_amount = liq_ref_amount;
        config.exposure_cap_bps = exposure_cap_bps;
        config.exposure_k_bps = exposure_k_bps;

        event::emit(RiskPricingConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            min_spot_total_liquidity,
            max_coverage_fraction_of_option_bps,
            max_risk_multiplier_bps,
            min_premium_amount,
            spot_smoothing_per_option,
            implied_prob_floor_bps,
            odds_floor_1x,
            odds_cap_bps,
            liq_cap_bps,
            liq_ref_amount,
            exposure_cap_bps,
            exposure_k_bps,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Emergency enable/disable toggle (admin only)
    public entry fun set_enable_flag(
        _: &InsuranceAdminCap,
        config: &mut InsuranceConfig,
        enable_flag: bool,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        config.enable_flag = enable_flag;

        let updated_by = tx_context::sender(ctx);
        let timestamp = clock::timestamp_ms(clock);
        event::emit(ConfigUpdatedEvent {
            updated_by,
            enable_flag: config.enable_flag,
            min_coverage_bps: config.min_coverage_bps,
            max_coverage_bps: config.max_coverage_bps,
            max_duration_ms: config.max_duration_ms,
            fee_bps: config.fee_bps,
            odds_base_bps: config.odds_base_bps,
            timestamp,
        });
    }

    public(package) fun create_insurance_admin_cap(ctx: &mut TxContext): InsuranceAdminCap {
        InsuranceAdminCap { id: object::new(ctx) }
    }

    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        let admin = tx_context::sender(ctx);
        let ts = clock::timestamp_ms(clock);
        let config = InsuranceConfig {
            id: object::new(ctx),
            enable_flag: false,
            min_coverage_bps: DEFAULT_MIN_COVERAGE_BPS,
            max_coverage_bps: DEFAULT_MAX_COVERAGE_BPS,
            max_duration_ms: DEFAULT_MAX_DURATION_MS,
            fee_bps: DEFAULT_FEE_BPS,
            min_spot_total_liquidity: DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY,
            max_coverage_fraction_of_option_bps: DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS,
            max_risk_multiplier_bps: DEFAULT_MAX_RISK_MULTIPLIER_BPS,
            min_premium_amount: DEFAULT_MIN_PREMIUM_AMOUNT,
            spot_smoothing_per_option: DEFAULT_SPOT_SMOOTHING_PER_OPTION,
            implied_prob_floor_bps: DEFAULT_IMPLIED_PROB_FLOOR_BPS,
            odds_floor_1x: true,
            odds_cap_bps: DEFAULT_ODDS_CAP_BPS,
            liq_cap_bps: DEFAULT_LIQ_CAP_BPS,
            liq_ref_amount: DEFAULT_LIQ_REF_AMOUNT,
            exposure_cap_bps: DEFAULT_EXPOSURE_CAP_BPS,
            exposure_k_bps: DEFAULT_EXPOSURE_K_BPS,
            odds_base_bps: DEFAULT_ODDS_BASE_BPS,
            version: DEFAULT_VERSION,
        };

        transfer::share_object(new_router_config_defaults(ctx));
        transfer::share_object(new_backstop_pool_defaults(ctx));

        event::emit(ConfigUpdatedEvent {
            updated_by: admin,
            enable_flag: false,
            min_coverage_bps: DEFAULT_MIN_COVERAGE_BPS,
            max_coverage_bps: DEFAULT_MAX_COVERAGE_BPS,
            max_duration_ms: DEFAULT_MAX_DURATION_MS,
            fee_bps: DEFAULT_FEE_BPS,
            odds_base_bps: DEFAULT_ODDS_BASE_BPS,
            timestamp: ts,
        });
        event::emit(RiskPricingConfigUpdatedEvent {
            updated_by: admin,
            min_spot_total_liquidity: DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY,
            max_coverage_fraction_of_option_bps: DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS,
            max_risk_multiplier_bps: DEFAULT_MAX_RISK_MULTIPLIER_BPS,
            min_premium_amount: DEFAULT_MIN_PREMIUM_AMOUNT,
            spot_smoothing_per_option: DEFAULT_SPOT_SMOOTHING_PER_OPTION,
            implied_prob_floor_bps: DEFAULT_IMPLIED_PROB_FLOOR_BPS,
            odds_floor_1x: true,
            odds_cap_bps: DEFAULT_ODDS_CAP_BPS,
            liq_cap_bps: DEFAULT_LIQ_CAP_BPS,
            liq_ref_amount: DEFAULT_LIQ_REF_AMOUNT,
            exposure_cap_bps: DEFAULT_EXPOSURE_CAP_BPS,
            exposure_k_bps: DEFAULT_EXPOSURE_K_BPS,
            timestamp: ts,
        });

        transfer::share_object(config);
    }

    /// Create an underwriter vault
    public entry fun create_vault(
        base_rate_bps_per_day: u64,
        utilization_multiplier_bps: u64,
        max_exposure_per_market: u64,
        max_exposure_per_user: u64,
        ctx: &mut TxContext
    ) {
        let underwriter = tx_context::sender(ctx);
        let max_exposure_per_option = 0;
        let enabled = true;
        let paused = false;
        let vault = UnderwriterVault {
            id: object::new(ctx),
            underwriter,
            capital: balance::zero(),
            reserved: 0,
            base_rate_bps_per_day,
            utilization_multiplier_bps,
            max_exposure_per_market,
            max_exposure_per_user,
            max_exposure_per_option,
            enabled,
            paused,
            market_exposures: table::new(ctx),
            user_exposures: table::new(ctx),
            version: DEFAULT_VERSION,
        };
        let vault_id = object::id(&vault);
        transfer::share_object(vault);

        event::emit(UnderwriterVaultCreatedEvent {
            vault_id,
            underwriter,
            base_rate_bps_per_day,
            utilization_multiplier_bps,
            max_exposure_per_market,
            max_exposure_per_user,
            max_exposure_per_option,
            enabled,
            paused,
        });
    }

    /// Underwriter updates vault listing parameters (emit for indexer discovery).
    public entry fun set_vault_status(
        vault: &mut UnderwriterVault,
        enabled: bool,
        paused: bool,
        max_exposure_per_option: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == vault.underwriter, ENotAdmin);
        vault.enabled = enabled;
        vault.paused = paused;
        vault.max_exposure_per_option = max_exposure_per_option;

        event::emit(VaultStatusUpdatedEvent {
            vault_id: object::id(vault),
            enabled,
            paused,
            max_exposure_per_option,
            max_exposure_per_market: vault.max_exposure_per_market,
            max_exposure_per_user: vault.max_exposure_per_user,
            base_rate_bps_per_day: vault.base_rate_bps_per_day,
            utilization_multiplier_bps: vault.utilization_multiplier_bps,
            updated_by: tx_context::sender(ctx),
            timestamp_ms: clock::timestamp_ms(clock),
        });
    }

    public entry fun set_router_flags(
        _: &InsuranceAdminCap,
        router_cfg: &mut InsuranceRouterConfig,
        router_enabled: bool,
        router_paused: bool,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        router_cfg.router_enabled = router_enabled;
        router_cfg.router_paused = router_paused;
        let _ = clock;
        let _ = ctx;
    }

    public entry fun set_router_limits(
        _: &InsuranceAdminCap,
        router_cfg: &mut InsuranceRouterConfig,
        max_route_reserve_market: u64,
        max_route_reserve_user: u64,
        max_route_reserve_option: u64,
        max_vault_concentration_bps: u64,
        min_vault_health_factor_bps: u64,
        max_route_legs: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(
            max_vault_concentration_bps > 0 && max_vault_concentration_bps <= BPS_DENOM,
            EInvalidCoverage
        );
        assert!(min_vault_health_factor_bps > 0, EInvalidAmount);
        assert!(max_route_legs > 0, EInvalidAmount);
        router_cfg.max_route_reserve_market = max_route_reserve_market;
        router_cfg.max_route_reserve_user = max_route_reserve_user;
        router_cfg.max_route_reserve_option = max_route_reserve_option;
        router_cfg.max_vault_concentration_bps = max_vault_concentration_bps;
        router_cfg.min_vault_health_factor_bps = min_vault_health_factor_bps;
        router_cfg.max_route_legs = max_route_legs;
        event::emit(RouterLimitsUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            max_route_reserve_market,
            max_route_reserve_user,
            max_route_reserve_option,
            max_vault_concentration_bps,
            min_vault_health_factor_bps,
            max_route_legs,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    public entry fun set_market_pause(
        _: &InsuranceAdminCap,
        router_cfg: &mut InsuranceRouterConfig,
        market_id: address,
        paused: bool,
        ctx: &mut TxContext,
    ) {
        if (table::contains(&router_cfg.market_pause, market_id)) {
            *table::borrow_mut(&mut router_cfg.market_pause, market_id) = paused;
        } else {
            table::add(&mut router_cfg.market_pause, market_id, paused);
        };
        let _ = ctx;
    }

    public entry fun set_backstop_caps(
        _: &InsuranceAdminCap,
        pool: &mut InsuranceBackstopPool,
        max_payout_per_market: u64,
        max_payout_per_event: u64,
        global_hard_cap: u64,
        sweep_premium_bps: u64,
        tail_pay_partial_on_cap: bool,
        ctx: &mut TxContext,
    ) {
        assert!(sweep_premium_bps <= BPS_DENOM, EInvalidCoverage);
        pool.max_payout_per_market = max_payout_per_market;
        pool.max_payout_per_event = max_payout_per_event;
        pool.global_hard_cap = global_hard_cap;
        pool.sweep_premium_bps = sweep_premium_bps;
        pool.tail_pay_partial_on_cap = tail_pay_partial_on_cap;
        let _ = ctx;
    }

    public entry fun set_tail_mode(
        _: &InsuranceAdminCap,
        pool: &mut InsuranceBackstopPool,
        tail_mode_enabled: bool,
        ctx: &mut TxContext,
    ) {
        pool.tail_mode_enabled = tail_mode_enabled;
        let _ = ctx;
    }

    public entry fun set_backstop_paused(
        _: &InsuranceAdminCap,
        pool: &mut InsuranceBackstopPool,
        paused: bool,
        ctx: &mut TxContext,
    ) {
        pool.paused = paused;
        let _ = ctx;
    }

    public entry fun deposit_backstop_treasury(
        _: &InsuranceAdminCap,
        pool: &mut InsuranceBackstopPool,
        payment: Coin<MYSO>,
        ctx: &mut TxContext,
    ) {
        let amt = coin::value(&payment);
        assert!(amt > 0, EInvalidAmount);
        let sender = tx_context::sender(ctx);
        balance::join(&mut pool.capital, coin::into_balance(payment));
        let new_balance = balance::value(&pool.capital);
        event::emit(BackstopTreasuryDepositEvent {
            depositor: sender,
            amount: amt,
            new_balance,
        });
    }

    /// Tail shortfall payout only (`tail_mode_enabled` + caps). Does not interact with `claim`.
    public entry fun tail_pay_shortfall(
        _: &InsuranceAdminCap,
        pool: &mut InsuranceBackstopPool,
        config: &InsuranceConfig,
        recipient: address,
        market_id: address,
        amount_requested: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.enable_flag, EDisabled);
        assert!(pool.tail_mode_enabled, ETailModeDisabled);
        assert!(!pool.paused, EBackstopPaused);
        assert!(amount_requested > 0, EInvalidAmount);

        let pool_balance = balance::value(&pool.capital);
        let mut paid_market = 0;
        if (table::contains(&pool.paid_by_market, market_id)) {
            paid_market = *table::borrow(&pool.paid_by_market, market_id);
        };
        let remaining_market = min_cap_sub(pool.max_payout_per_market, paid_market);
        let remaining_global = min_cap_sub(pool.global_hard_cap, pool.total_paid_out);
        let cap_event = pool.max_payout_per_event;
        let pay_cap = min_u64(
            min_u64(min_u64(amount_requested, pool_balance), remaining_market),
            min_u64(remaining_global, cap_event),
        );
        if (pay_cap == 0) {
            assert!(pool.tail_pay_partial_on_cap, EBackstopPayoutLimit);
        } else {
            if (pay_cap < amount_requested) {
                assert!(pool.tail_pay_partial_on_cap, EBackstopPayoutLimit);
            };

            let pay_bal = balance::split(&mut pool.capital, pay_cap);
            let coin_out = coin::from_balance(pay_bal, ctx);
            transfer::public_transfer(coin_out, recipient);

            pool.total_paid_out = pool.total_paid_out + pay_cap;
            if (table::contains(&pool.paid_by_market, market_id)) {
                let e = table::borrow_mut(&mut pool.paid_by_market, market_id);
                *e = *e + pay_cap;
            } else {
                table::add(&mut pool.paid_by_market, market_id, pay_cap);
            };

            event::emit(BackstopUsedEvent {
                market_id,
                recipient,
                amount: pay_cap,
                total_paid_out_after: pool.total_paid_out,
                tail_mode_enabled: pool.tail_mode_enabled,
            });
        };
        let _ = clock;
        let _ = ctx;
    }

    fun min_cap_sub(cap: u64, used: u64): u64 {
        if (cap >= used) {
            cap - used
        } else {
            0
        }
    }

    fun min_u64(a: u64, b: u64): u64 {
        if (a < b) {
            a
        } else {
            b
        }
    }

    fun copy_id_vec(src: &vector<ID>): vector<ID> {
        let mut out = vector::empty();
        let len = vector::length(src);
        let mut i = 0;
        while (i < len) {
            vector::push_back(&mut out, *vector::borrow(src, i));
            i = i + 1;
        };
        out
    }

    fun new_router_config_defaults(ctx: &mut TxContext): InsuranceRouterConfig {
        InsuranceRouterConfig {
            id: object::new(ctx),
            router_enabled: true,
            router_paused: false,
            max_route_reserve_market: 0,
            max_route_reserve_user: 0,
            max_route_reserve_option: 0,
            max_vault_concentration_bps: BPS_DENOM,
            min_vault_health_factor_bps: BPS_DENOM,
            max_route_legs: DEFAULT_MAX_ROUTE_LEGS,
            market_pause: table::new(ctx),
            version: DEFAULT_VERSION,
        }
    }

    fun new_backstop_pool_defaults(ctx: &mut TxContext): InsuranceBackstopPool {
        InsuranceBackstopPool {
            id: object::new(ctx),
            capital: balance::zero(),
            total_paid_out: 0,
            paid_by_market: table::new(ctx),
            max_payout_per_market: MAX_U64,
            max_payout_per_event: MAX_U64,
            global_hard_cap: MAX_U64,
            tail_mode_enabled: false,
            paused: false,
            sweep_premium_bps: 0,
            tail_pay_partial_on_cap: true,
            version: DEFAULT_VERSION,
        }
    }

    fun assert_market_router_open(router_cfg: &InsuranceRouterConfig, market_id: address) {
        if (table::contains(&router_cfg.market_pause, market_id)) {
            assert!(!*table::borrow(&router_cfg.market_pause, market_id), EMarketClosed);
        };
    }

    fun assert_vault_buy_guards(
        vault: &UnderwriterVault,
        router_cfg: &InsuranceRouterConfig,
        check_health: bool,
    ) {
        assert!(vault.enabled, EVaultDisabled);
        assert!(!vault.paused, EVaultPaused);
        if (check_health) {
            let cap = balance::value(&vault.capital);
            let r = vault.reserved;
            assert!(cap * BPS_DENOM >= r * router_cfg.min_vault_health_factor_bps, EInsufficientCapital);
        };
    }

    /// Deposit capital into vault
    public entry fun deposit_capital(
        config: &InsuranceConfig,
        vault: &mut UnderwriterVault,
        payment: Coin<MYSO>,
        ctx: &mut TxContext
    ) {
        assert!(config.enable_flag, EDisabled);
        let deposit_amount = coin::value(&payment);
        assert!(deposit_amount > 0, EInvalidAmount);
        balance::join(&mut vault.capital, coin::into_balance(payment));
        event::emit(UnderwriterVaultDepositedEvent {
            vault_id: object::id(vault),
            amount: deposit_amount,
            new_balance: balance::value(&vault.capital),
        });
    }

    /// Withdraw unreserved capital (underwriter only)
    public entry fun withdraw_capital(
        config: &InsuranceConfig,
        vault: &mut UnderwriterVault,
        amount: u64,
        ctx: &mut TxContext
    ) {
        assert!(config.enable_flag, EDisabled);
        assert!(tx_context::sender(ctx) == vault.underwriter, ENotAdmin);
        assert!(amount > 0, EInvalidAmount);

        let capital_value = balance::value(&vault.capital);
        assert!(capital_value >= vault.reserved, EOverflow);
        let free_capital = capital_value - vault.reserved;
        assert!(free_capital >= amount, EInsufficientCapital);

        let payout_balance = balance::split(&mut vault.capital, amount);
        let payout_coin = coin::from_balance(payout_balance, ctx);
        transfer::public_transfer(payout_coin, vault.underwriter);

        event::emit(UnderwriterVaultWithdrawnEvent {
            vault_id: object::id(vault),
            amount,
            new_balance: balance::value(&vault.capital),
        });
    }

    public fun premium_quote_premium(q: &PremiumQuote): u64 {
        q.premium
    }

    public fun premium_quote_implied_prob_win_bps(q: &PremiumQuote): u64 {
        q.implied_prob_win_bps
    }

    public fun premium_quote_risk_multiplier_bps(q: &PremiumQuote): u64 {
        q.risk_multiplier_bps
    }

    public fun premium_quote_premium_raw(q: &PremiumQuote): u64 {
        q.premium_raw
    }

    fun quote_base_premium(
        vault: &UnderwriterVault,
        covered_amount: u64,
        coverage_bps: u64,
        duration_ms: u64
    ): u64 {
        let capital_value = balance::value(&vault.capital);
        let utilization_bps = if (capital_value == 0) {
            BPS_DENOM
        } else {
            let utilization_u128 = (vault.reserved as u128) * (BPS_DENOM as u128) / (capital_value as u128);
            assert!(utilization_u128 <= (MAX_U64 as u128), EOverflow);
            utilization_u128 as u64
        };
        let utilization_factor = (utilization_bps * vault.utilization_multiplier_bps) / BPS_DENOM;
        let total_rate_bps_per_day = vault.base_rate_bps_per_day + utilization_factor;

        let numerator = (covered_amount as u128)
            * (coverage_bps as u128)
            * (total_rate_bps_per_day as u128)
            * (duration_ms as u128);
        let denominator = (BPS_DENOM as u128) * (BPS_DENOM as u128) * (DAY_MS as u128);
        let premium_u128 = numerator / denominator;
        assert!(premium_u128 <= (MAX_U64 as u128), EOverflow);
        premium_u128 as u64
    }

    /// Utilization curve only (`quote_base_premium`).
    public fun quote_premium(
        vault: &UnderwriterVault,
        covered_amount: u64,
        coverage_bps: u64,
        duration_ms: u64
    ): u64 {
        quote_base_premium(vault, covered_amount, coverage_bps, duration_ms)
    }

    fun get_market_option_reserved(vault: &UnderwriterVault, market_id: address, option_id: u8): u64 {
        if (!table::contains(&vault.market_exposures, market_id)) {
            0
        } else {
            let exposure = table::borrow(&vault.market_exposures, market_id);
            get_option_reserved(exposure, option_id)
        }
    }

    fun compute_spot_risk_quote(
        config: &InsuranceConfig,
        vault: &UnderwriterVault,
        record: &spot::SpotRecord,
        vault_market_id: address,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        duration_ms: u64,
        enforce_max_risk: bool,
    ): PremiumQuote {
        spot::assert_valid_option_id(record, option_id);

        let t_total = spot::total_option_escrow(record);
        assert!(t_total >= config.min_spot_total_liquidity, EThinMarket);

        let a_opt = spot::get_option_escrow(record, option_id);
        let denom_cov = if (a_opt >= 1) {
            a_opt
        } else {
            1
        };
        assert!(
            (covered_amount as u128) * (BPS_DENOM as u128)
                <= (config.max_coverage_fraction_of_option_bps as u128) * (denom_cov as u128),
            ECoverageTooLargeVersusPool
        );

        let n_opts = spot::num_betting_options(record);
        let w = config.spot_smoothing_per_option;
        let nw_u128 = (n_opts as u128) * (w as u128);
        assert!(nw_u128 <= (MAX_U64 as u128), EOverflow);
        let smoothed_t = (t_total as u128) + nw_u128;
        assert!(smoothed_t > 0 && smoothed_t <= (MAX_U64 as u128) * 2, EOverflow);

        let smoothed_a = (a_opt as u128) + (w as u128);
        let p_win_u128 = (smoothed_a * (BPS_DENOM as u128)) / smoothed_t;
        assert!(p_win_u128 <= (MAX_U64 as u128), EOverflow);
        let p_win_bps = p_win_u128 as u64;

        let reserved_opt = get_market_option_reserved(vault, vault_market_id, option_id);

        let p_floor = config.implied_prob_floor_bps;
        let denom_p = if (p_win_bps > p_floor) { p_win_bps } else { p_floor };

        let odds_core_u128 = (config.odds_base_bps as u128) * (BPS_DENOM as u128) / (denom_p as u128);
        assert!(odds_core_u128 <= (MAX_U64 as u128), EOverflow);
        let odds_core = odds_core_u128 as u64;
        let mut odds_mult_bps = if (config.odds_cap_bps < odds_core) {
            config.odds_cap_bps
        } else {
            odds_core
        };
        if (config.odds_floor_1x && odds_mult_bps < BPS_DENOM) {
            odds_mult_bps = BPS_DENOM;
        };

        let t_for_liq = if (t_total >= 1) {
            t_total
        } else {
            1
        };
        let liq_uncapped_u128 = (config.liq_ref_amount as u128)
            * (BPS_DENOM as u128) / (t_for_liq as u128);
        assert!(liq_uncapped_u128 <= (MAX_U64 as u128), EOverflow);
        let liq_uncapped = liq_uncapped_u128 as u64;
        let liq_mult_bps = if (config.liq_cap_bps < liq_uncapped) {
            config.liq_cap_bps
        } else {
            liq_uncapped
        };

        let extra_num_u128 = (config.exposure_k_bps as u128) * (reserved_opt as u128) / (denom_cov as u128);
        assert!(extra_num_u128 <= (MAX_U64 as u128), EOverflow);
        let extra_term = extra_num_u128 as u64;
        let max_extra_bps = config.exposure_cap_bps - BPS_DENOM;
        let extra_bounded = if (extra_term > max_extra_bps) {
            max_extra_bps
        } else {
            extra_term
        };
        assert!(extra_bounded <= MAX_U64 - BPS_DENOM, EOverflow);
        let exposure_mult_bps = BPS_DENOM + extra_bounded;

        let risk_u128 =
            ((odds_mult_bps as u128) * (liq_mult_bps as u128) * (exposure_mult_bps as u128))
                / (BPS_DENOM as u128) / (BPS_DENOM as u128);
        assert!(risk_u128 <= (MAX_U64 as u128), EOverflow);
        let risk_multiplier_bps = risk_u128 as u64;

        if (enforce_max_risk) {
            assert!(risk_multiplier_bps <= config.max_risk_multiplier_bps, ERiskMultiplierTooHigh);
        };

        let base_premium =
            quote_base_premium(vault, covered_amount, coverage_bps, duration_ms);

        let premium_raw_u128 = ((base_premium as u128) * (risk_multiplier_bps as u128))
            / (BPS_DENOM as u128);
        assert!(premium_raw_u128 <= (MAX_U64 as u128), EOverflow);
        let premium_raw = premium_raw_u128 as u64;

        let premium = if (premium_raw >= config.min_premium_amount) {
            premium_raw
        } else {
            config.min_premium_amount
        };
        assert!(premium > 0, EInsufficientPremium);

        PremiumQuote {
            premium,
            premium_raw,
            implied_prob_win_bps: p_win_bps,
            risk_multiplier_bps,
            market_total_amount: t_total,
            option_amount: a_opt,
            base_premium,
        }
    }

    /// Preview premium with SPoT pool odds, liquidity, and vault concentration on this option (`reserved` excludes a not-yet-open policy).
    public fun quote_premium_with_spot_risk(
        config: &InsuranceConfig,
        vault: &UnderwriterVault,
        record: &spot::SpotRecord,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        duration_ms: u64,
    ): PremiumQuote {
        let market_id = spot::get_id_address(record);
        compute_spot_risk_quote(
            config,
            vault,
            record,
            market_id,
            option_id,
            covered_amount,
            coverage_bps,
            duration_ms,
            true,
        )
    }

    fun coverage_quote_skipped(reason: u8): VaultCoverageQuote {
        VaultCoverageQuote {
            premium: 0,
            premium_raw: 0,
            reserve_required: 0,
            available_capacity_reserve: 0,
            risk_multiplier_bps: 0,
            implied_prob_win_bps: 0,
            utilization_bps: 0,
            max_fill_covered_amount: 0,
            skipped_reason: reason,
        }
    }

    fun max_fill_covered_for_vault(
        config: &InsuranceConfig,
        vault: &UnderwriterVault,
        record: &spot::SpotRecord,
        market_id: address,
        insured: address,
        option_id: u8,
        coverage_bps: u64,
    ): u64 {
        let position = spot::get_user_option_amount(record, insured, option_id);
        let a_opt = spot::get_option_escrow(record, option_id);
        let denom_cov = if (a_opt >= 1) {
            a_opt
        } else {
            1
        };
        let pool_max_u128 =
            (config.max_coverage_fraction_of_option_bps as u128) * (denom_cov as u128) / (BPS_DENOM as u128);
        assert!(pool_max_u128 <= (MAX_U64 as u128), EOverflow);
        let pool_max = pool_max_u128 as u64;

        let capital_value = balance::value(&vault.capital);
        let free_capital = if (capital_value >= vault.reserved) {
            capital_value - vault.reserved
        } else {
            0
        };
        let vault_cov_max = if (coverage_bps > 0) {
            let v = (free_capital as u128) * (BPS_DENOM as u128) / (coverage_bps as u128);
            assert!(v <= (MAX_U64 as u128), EOverflow);
            v as u64
        } else {
            MAX_U64
        };

        let mut market_head_reserve = MAX_U64;
        if (vault.max_exposure_per_market > 0) {
            let tr = if (table::contains(&vault.market_exposures, market_id)) {
                table::borrow(&vault.market_exposures, market_id).total_reserved
            } else {
                0
            };
            market_head_reserve = min_cap_sub(vault.max_exposure_per_market, tr);
        };
        let market_cov_max = reserve_to_covered(market_head_reserve, coverage_bps);

        let mut opt_head_reserve = MAX_U64;
        if (vault.max_exposure_per_option > 0) {
            let ors = get_market_option_reserved(vault, market_id, option_id);
            opt_head_reserve = min_cap_sub(vault.max_exposure_per_option, ors);
        };
        let opt_cov_max = reserve_to_covered(opt_head_reserve, coverage_bps);

        let mut user_head_reserve = MAX_U64;
        if (vault.max_exposure_per_user > 0) {
            let ue = get_user_exposure(vault, insured);
            user_head_reserve = min_cap_sub(vault.max_exposure_per_user, ue);
        };
        let user_cov_max = reserve_to_covered(user_head_reserve, coverage_bps);

        let mut m = position;
        if (pool_max < m) {
            m = pool_max
        };
        if (vault_cov_max < m) {
            m = vault_cov_max
        };
        if (market_cov_max < m) {
            m = market_cov_max
        };
        if (opt_cov_max < m) {
            m = opt_cov_max
        };
        if (user_cov_max < m) {
            m = user_cov_max
        };
        m
    }

    fun reserve_to_covered(head_reserve: u64, coverage_bps: u64): u64 {
        if (coverage_bps == 0) {
            MAX_U64
        } else {
            let v = (head_reserve as u128) * (BPS_DENOM as u128) / (coverage_bps as u128);
            if (v > (MAX_U64 as u128)) {
                MAX_U64
            } else {
                v as u64
            }
        }
    }

    public fun quote_vault_for_spot_coverage(
        config: &InsuranceConfig,
        router_cfg: &InsuranceRouterConfig,
        vault: &UnderwriterVault,
        record: &spot::SpotRecord,
        insured: address,
        option_id: u8,
        requested_coverage_amount: u64,
        coverage_bps: u64,
        duration_ms: u64,
    ): VaultCoverageQuote {
        if (router_cfg.router_paused) {
            return coverage_quote_skipped(SKIPPED_ROUTER_PAUSED)
        };
        let market_id = spot::get_id_address(record);
        if (table::contains(&router_cfg.market_pause, market_id)
            && *table::borrow(&router_cfg.market_pause, market_id)) {
            return coverage_quote_skipped(SKIPPED_MARKET_PAUSED)
        };
        if (!vault.enabled) {
            return coverage_quote_skipped(SKIPPED_VAULT_DISABLED)
        };
        if (vault.paused) {
            return coverage_quote_skipped(SKIPPED_VAULT_PAUSED)
        };
        let cap = balance::value(&vault.capital);
        let r = vault.reserved;
        if (cap * BPS_DENOM < r * router_cfg.min_vault_health_factor_bps) {
            return coverage_quote_skipped(SKIPPED_UNHEALTHY_VAULT)
        };
        let t_total = spot::total_option_escrow(record);
        if (t_total < config.min_spot_total_liquidity) {
            return coverage_quote_skipped(SKIPPED_THIN_OR_POOL)
        };
        if ((option_id as u64) >= spot::num_betting_options(record)) {
            return coverage_quote_skipped(SKIPPED_THIN_OR_POOL)
        };

        let max_fill =
            max_fill_covered_for_vault(config, vault, record, market_id, insured, option_id, coverage_bps);
        if (max_fill == 0) {
            return coverage_quote_skipped(SKIPPED_ZERO_CAPACITY)
        };

        let trade = if (requested_coverage_amount <= max_fill) {
            requested_coverage_amount
        } else {
            max_fill
        };

        let pq = compute_spot_risk_quote(
            config,
            vault,
            record,
            market_id,
            option_id,
            trade,
            coverage_bps,
            duration_ms,
            false,
        );
        if (pq.risk_multiplier_bps > config.max_risk_multiplier_bps) {
            return VaultCoverageQuote {
                premium: 0,
                premium_raw: 0,
                reserve_required: 0,
                available_capacity_reserve: compute_reserve(max_fill, coverage_bps),
                risk_multiplier_bps: pq.risk_multiplier_bps,
                implied_prob_win_bps: pq.implied_prob_win_bps,
                utilization_bps: vault_utilization_bps(vault),
                max_fill_covered_amount: max_fill,
                skipped_reason: SKIPPED_RISK_MULTIPLIER,
            }
        };

        let res_req = compute_reserve(trade, coverage_bps);
        let avail_res = compute_reserve(max_fill, coverage_bps);

        let util_bps = vault_utilization_bps(vault);

        VaultCoverageQuote {
            premium: pq.premium,
            premium_raw: pq.premium_raw,
            reserve_required: res_req,
            available_capacity_reserve: avail_res,
            risk_multiplier_bps: pq.risk_multiplier_bps,
            implied_prob_win_bps: pq.implied_prob_win_bps,
            utilization_bps: util_bps,
            max_fill_covered_amount: max_fill,
            skipped_reason: SKIPPED_OK,
        }
    }

    fun vault_utilization_bps(vault: &UnderwriterVault): u64 {
        let capital_value = balance::value(&vault.capital);
        if (capital_value == 0) {
            BPS_DENOM
        } else {
            let utilization_u128 = (vault.reserved as u128) * (BPS_DENOM as u128) / (capital_value as u128);
            assert!(utilization_u128 <= (MAX_U64 as u128), EOverflow);
            utilization_u128 as u64
        }
    }

    fun buy_coverage_execute(
        config: &InsuranceConfig,
        router_cfg: &InsuranceRouterConfig,
        backstop: &mut InsuranceBackstopPool,
        spot_config: &spot::SpotConfig,
        vault: &mut UnderwriterVault,
        record: &spot::SpotRecord,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        duration_ms: u64,
        payment: &mut Coin<MYSO>,
        clock: &Clock,
        route_id: Option<ID>,
        route_leg_index: u8,
        check_market_router: bool,
        ctx: &mut TxContext,
    ): (ID, ID, u64, u64, u64, u64, u64) {
        assert!(config.enable_flag, EDisabled);
        assert!(spot::is_enabled(spot_config), EMarketClosed);
        assert!(spot::is_open(record), EMarketClosed);
        assert!(coverage_bps >= config.min_coverage_bps, EInvalidCoverage);
        assert!(coverage_bps <= config.max_coverage_bps, EInvalidCoverage);
        assert!(duration_ms > 0 && duration_ms <= config.max_duration_ms, EInvalidDuration);
        assert!(covered_amount > 0, EInvalidAmount);

        let insured = tx_context::sender(ctx);
        let market_id = spot::get_id_address(record);
        if (check_market_router) {
            assert_market_router_open(router_cfg, market_id);
        };

        assert_vault_buy_guards(vault, router_cfg, true);

        let position_amount = spot::get_user_option_amount(record, insured, option_id);
        assert!(covered_amount <= position_amount, EInvalidAmount);

        let reserve_amount = compute_reserve(covered_amount, coverage_bps);
        let capital_value = balance::value(&vault.capital);
        assert!(capital_value >= vault.reserved, EOverflow);
        let free_capital = capital_value - vault.reserved;
        assert!(free_capital >= reserve_amount, EInsufficientCapital);
        assert!(vault.reserved <= MAX_U64 - reserve_amount, EOverflow);

        let pq = compute_spot_risk_quote(
            config,
            vault,
            record,
            market_id,
            option_id,
            covered_amount,
            coverage_bps,
            duration_ms,
            true,
        );
        let premium = pq.premium;

        enforce_exposure_limits(vault, market_id, insured, option_id, reserve_amount, ctx);

        assert!(coin::value(payment) >= premium, EInsufficientPremium);

        let sweep_bps = backstop.sweep_premium_bps;
        if (sweep_bps > 0) {
            assert!(!backstop.paused, EBackstopPaused);
        };
        let sweep_amt = (premium * sweep_bps) / BPS_DENOM;
        let to_vault_amt = premium - sweep_amt;

        let mut prem_coin = coin::split(payment, premium, ctx);
        if (sweep_amt > 0) {
            let sc = coin::split(&mut prem_coin, sweep_amt, ctx);
            balance::join(&mut backstop.capital, coin::into_balance(sc));
        };
        balance::join(&mut vault.capital, coin::into_balance(prem_coin));

        vault.reserved = vault.reserved + reserve_amount;
        add_exposure(vault, market_id, insured, option_id, reserve_amount, ctx);

        let now = clock::timestamp_ms(clock);
        assert!(now <= MAX_U64 - duration_ms, EOverflow);
        let expiry_time_ms = now + duration_ms;
        let vault_id_ins = object::id(vault);
        let policy = CoveragePolicy {
            id: object::new(ctx),
            market_id,
            insured,
            option_id,
            covered_amount,
            coverage_bps,
            premium_paid: premium,
            start_time_ms: now,
            expiry_time_ms,
            vault_id: vault_id_ins,
            status: STATUS_ACTIVE,
            route_id,
            route_leg_index,
        };
        let policy_id = object::id(&policy);
        transfer::share_object(policy);

        event::emit(CoveragePurchasedEvent {
            policy_id,
            vault_id: vault_id_ins,
            market_id,
            insured,
            option_id,
            covered_amount,
            coverage_bps,
            premium_paid: premium,
            premium_raw: pq.premium_raw,
            reserve_locked: reserve_amount,
            expiry_time_ms,
            implied_probability_bps: pq.implied_prob_win_bps,
            risk_multiplier_bps: pq.risk_multiplier_bps,
            base_premium: pq.base_premium,
            market_total_amount: pq.market_total_amount,
            option_amount: pq.option_amount,
            backstop_sweep_amount: sweep_amt,
            route_id,
            route_leg_index,
        });

        (policy_id, vault_id_ins, premium, reserve_amount, sweep_amt, covered_amount, expiry_time_ms)
    }

    /// Buy coverage for a SPoT position
    public entry fun buy_coverage(
        config: &InsuranceConfig,
        router_cfg: &InsuranceRouterConfig,
        backstop: &mut InsuranceBackstopPool,
        spot_config: &spot::SpotConfig,
        vault: &mut UnderwriterVault,
        record: &spot::SpotRecord,
        option_id: u8,
        requested_coverage_amount: u64,
        coverage_bps: u64,
        duration_ms: u64,
        mut payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(config.enable_flag, EDisabled);
        assert!(spot::is_enabled(spot_config), EMarketClosed);
        assert!(spot::is_open(record), EMarketClosed);
        assert!(coverage_bps >= config.min_coverage_bps, EInvalidCoverage);
        assert!(coverage_bps <= config.max_coverage_bps, EInvalidCoverage);
        assert!(duration_ms > 0 && duration_ms <= config.max_duration_ms, EInvalidDuration);
        assert!(requested_coverage_amount > 0, EInvalidAmount);

        let insured = tx_context::sender(ctx);
        let position_amount = spot::get_user_option_amount(record, insured, option_id);
        let covered_amount = if (requested_coverage_amount <= position_amount) {
            requested_coverage_amount
        } else {
            position_amount
        };
        assert!(covered_amount > 0, EInvalidAmount);

        buy_coverage_execute(
            config,
            router_cfg,
            backstop,
            spot_config,
            vault,
            record,
            option_id,
            covered_amount,
            coverage_bps,
            duration_ms,
            &mut payment,
            clock,
            option::none(),
            0,
            false,
            ctx,
        );

        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, insured);
        } else {
            coin::destroy_zero(payment);
        };
    }

    public entry fun route_buy_coverage_4(
        config: &InsuranceConfig,
        router_cfg: &InsuranceRouterConfig,
        backstop: &mut InsuranceBackstopPool,
        spot_config: &spot::SpotConfig,
        record: &spot::SpotRecord,
        v0: &mut UnderwriterVault,
        v1: &mut UnderwriterVault,
        v2: &mut UnderwriterVault,
        v3: &mut UnderwriterVault,
        option_id: u8,
        fill_0: u64,
        fill_1: u64,
        fill_2: u64,
        fill_3: u64,
        coverage_bps: u64,
        duration_ms: u64,
        deadline_ms: u64,
        min_total_covered: u64,
        max_total_premium: u64,
        mut payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(clock::timestamp_ms(clock) <= deadline_ms, EDeadlinePassed);
        assert!(config.enable_flag, EDisabled);
        assert!(router_cfg.router_enabled, ERouteDisabled);
        assert!(!router_cfg.router_paused, ERouterPaused);
        assert!(spot::is_enabled(spot_config), EMarketClosed);
        assert!(spot::is_open(record), EMarketClosed);
        assert!(coverage_bps >= config.min_coverage_bps, EInvalidCoverage);
        assert!(coverage_bps <= config.max_coverage_bps, EInvalidCoverage);
        assert!(duration_ms > 0 && duration_ms <= config.max_duration_ms, EInvalidDuration);

        let insured = tx_context::sender(ctx);
        let market_id = spot::get_id_address(record);
        assert_market_router_open(router_cfg, market_id);
        let position_amount = spot::get_user_option_amount(record, insured, option_id);

        let total_covered = fill_0 + fill_1 + fill_2 + fill_3;
        assert!(total_covered > 0, EInvalidAmount);
        assert!(total_covered >= min_total_covered, ESlippageCovered);
        assert!(total_covered <= position_amount, EInvalidAmount);

        let mut route_leg_count = 0;
        if (fill_0 > 0) { route_leg_count = route_leg_count + 1; };
        if (fill_1 > 0) { route_leg_count = route_leg_count + 1; };
        if (fill_2 > 0) { route_leg_count = route_leg_count + 1; };
        if (fill_3 > 0) { route_leg_count = route_leg_count + 1; };
        assert!(route_leg_count <= router_cfg.max_route_legs, EInvalidCoverage);

        if (fill_0 > 0 && fill_1 > 0) {
            assert!(object::id(v0) != object::id(v1), EDuplicateVaultInRoute);
        };
        if (fill_0 > 0 && fill_2 > 0) {
            assert!(object::id(v0) != object::id(v2), EDuplicateVaultInRoute);
        };
        if (fill_0 > 0 && fill_3 > 0) {
            assert!(object::id(v0) != object::id(v3), EDuplicateVaultInRoute);
        };
        if (fill_1 > 0 && fill_2 > 0) {
            assert!(object::id(v1) != object::id(v2), EDuplicateVaultInRoute);
        };
        if (fill_1 > 0 && fill_3 > 0) {
            assert!(object::id(v1) != object::id(v3), EDuplicateVaultInRoute);
        };
        if (fill_2 > 0 && fill_3 > 0) {
            assert!(object::id(v2) != object::id(v3), EDuplicateVaultInRoute);
        };

        let r0 = if (fill_0 > 0) {
            compute_reserve(fill_0, coverage_bps)
        } else {
            0
        };
        let r1 = if (fill_1 > 0) {
            compute_reserve(fill_1, coverage_bps)
        } else {
            0
        };
        let r2 = if (fill_2 > 0) {
            compute_reserve(fill_2, coverage_bps)
        } else {
            0
        };
        let r3 = if (fill_3 > 0) {
            compute_reserve(fill_3, coverage_bps)
        } else {
            0
        };
        let total_res = r0 + r1 + r2 + r3;

        if (router_cfg.max_route_reserve_market > 0) {
            assert!(total_res <= router_cfg.max_route_reserve_market, EExposureLimit);
        };
        if (router_cfg.max_route_reserve_user > 0) {
            assert!(total_res <= router_cfg.max_route_reserve_user, EExposureLimit);
        };
        if (router_cfg.max_route_reserve_option > 0) {
            assert!(total_res <= router_cfg.max_route_reserve_option, EExposureLimit);
        };

        let conc = router_cfg.max_vault_concentration_bps;
        if (r0 > 0) {
            assert!(
                (r0 as u128) * (BPS_DENOM as u128) <= (total_res as u128) * (conc as u128),
                EVaultConcentration
            );
        };
        if (r1 > 0) {
            assert!(
                (r1 as u128) * (BPS_DENOM as u128) <= (total_res as u128) * (conc as u128),
                EVaultConcentration
            );
        };
        if (r2 > 0) {
            assert!(
                (r2 as u128) * (BPS_DENOM as u128) <= (total_res as u128) * (conc as u128),
                EVaultConcentration
            );
        };
        if (r3 > 0) {
            assert!(
                (r3 as u128) * (BPS_DENOM as u128) <= (total_res as u128) * (conc as u128),
                EVaultConcentration
            );
        };

        let now = clock::timestamp_ms(clock);
        assert!(now <= MAX_U64 - duration_ms, EOverflow);
        let expiry_time_ms = now + duration_ms;

        let mut route = CoverageRoute {
            id: object::new(ctx),
            insured,
            market_id,
            option_id,
            coverage_bps,
            start_time_ms: now,
            expiry_time_ms,
            policy_ids: vector::empty(),
            vault_ids: vector::empty(),
            total_covered: 0,
            total_premium: 0,
            total_reserve: 0,
            total_backstop_sweep: 0,
            version: DEFAULT_VERSION,
        };
        let route_id = object::id(&route);

        let mut leg: u8 = 0;
        let mut total_premium: u64 = 0;

        if (fill_0 > 0) {
            let (pid, vid, prem, res, sw, cov, _) = buy_coverage_execute(
                config,
                router_cfg,
                backstop,
                spot_config,
                v0,
                record,
                option_id,
                fill_0,
                coverage_bps,
                duration_ms,
                &mut payment,
                clock,
                option::some(route_id),
                leg,
                true,
                ctx,
            );
            vector::push_back(&mut route.policy_ids, pid);
            vector::push_back(&mut route.vault_ids, vid);
            route.total_covered = route.total_covered + cov;
            route.total_premium = route.total_premium + prem;
            route.total_reserve = route.total_reserve + res;
            route.total_backstop_sweep = route.total_backstop_sweep + sw;
            total_premium = total_premium + prem;
            event::emit(RouteFillEvent {
                route_id,
                leg_index: leg,
                vault_id: vid,
                policy_id: pid,
                covered_amount: cov,
                premium_paid: prem,
                reserve_locked: res,
                backstop_sweep_amount: sw,
            });
            leg = leg + 1;
        };
        if (fill_1 > 0) {
            let (pid, vid, prem, res, sw, cov, _) = buy_coverage_execute(
                config,
                router_cfg,
                backstop,
                spot_config,
                v1,
                record,
                option_id,
                fill_1,
                coverage_bps,
                duration_ms,
                &mut payment,
                clock,
                option::some(route_id),
                leg,
                true,
                ctx,
            );
            vector::push_back(&mut route.policy_ids, pid);
            vector::push_back(&mut route.vault_ids, vid);
            route.total_covered = route.total_covered + cov;
            route.total_premium = route.total_premium + prem;
            route.total_reserve = route.total_reserve + res;
            route.total_backstop_sweep = route.total_backstop_sweep + sw;
            total_premium = total_premium + prem;
            event::emit(RouteFillEvent {
                route_id,
                leg_index: leg,
                vault_id: vid,
                policy_id: pid,
                covered_amount: cov,
                premium_paid: prem,
                reserve_locked: res,
                backstop_sweep_amount: sw,
            });
            leg = leg + 1;
        };
        if (fill_2 > 0) {
            let (pid, vid, prem, res, sw, cov, _) = buy_coverage_execute(
                config,
                router_cfg,
                backstop,
                spot_config,
                v2,
                record,
                option_id,
                fill_2,
                coverage_bps,
                duration_ms,
                &mut payment,
                clock,
                option::some(route_id),
                leg,
                true,
                ctx,
            );
            vector::push_back(&mut route.policy_ids, pid);
            vector::push_back(&mut route.vault_ids, vid);
            route.total_covered = route.total_covered + cov;
            route.total_premium = route.total_premium + prem;
            route.total_reserve = route.total_reserve + res;
            route.total_backstop_sweep = route.total_backstop_sweep + sw;
            total_premium = total_premium + prem;
            event::emit(RouteFillEvent {
                route_id,
                leg_index: leg,
                vault_id: vid,
                policy_id: pid,
                covered_amount: cov,
                premium_paid: prem,
                reserve_locked: res,
                backstop_sweep_amount: sw,
            });
            leg = leg + 1;
        };
        if (fill_3 > 0) {
            let (pid, vid, prem, res, sw, cov, _) = buy_coverage_execute(
                config,
                router_cfg,
                backstop,
                spot_config,
                v3,
                record,
                option_id,
                fill_3,
                coverage_bps,
                duration_ms,
                &mut payment,
                clock,
                option::some(route_id),
                leg,
                true,
                ctx,
            );
            vector::push_back(&mut route.policy_ids, pid);
            vector::push_back(&mut route.vault_ids, vid);
            route.total_covered = route.total_covered + cov;
            route.total_premium = route.total_premium + prem;
            route.total_reserve = route.total_reserve + res;
            route.total_backstop_sweep = route.total_backstop_sweep + sw;
            total_premium = total_premium + prem;
            event::emit(RouteFillEvent {
                route_id,
                leg_index: leg,
                vault_id: vid,
                policy_id: pid,
                covered_amount: cov,
                premium_paid: prem,
                reserve_locked: res,
                backstop_sweep_amount: sw,
            });
        };

        assert!(total_premium <= max_total_premium, ESlippagePremium);

        let pids = copy_id_vec(&route.policy_ids);
        let vids = copy_id_vec(&route.vault_ids);
        event::emit(CoverageRoutedEvent {
            route_id,
            insured,
            market_id,
            option_id,
            coverage_bps,
            duration_ms,
            total_covered: route.total_covered,
            total_premium: route.total_premium,
            total_reserve: route.total_reserve,
            total_backstop_sweep: route.total_backstop_sweep,
            expiry_time_ms: route.expiry_time_ms,
            policy_ids: pids,
            vault_ids: vids,
        });
        transfer::share_object(route);

        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, insured);
        } else {
            coin::destroy_zero(payment);
        };
    }

    /// Cancel coverage while the market is open
    /// Cancellation can result in 0 refund due to fee + rounding
    public entry fun cancel_coverage(
        config: &InsuranceConfig,
        spot_config: &spot::SpotConfig,
        treasury: &EcosystemTreasury,
        vault: &mut UnderwriterVault,
        record: &spot::SpotRecord,
        policy: &mut CoveragePolicy,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(config.enable_flag, EDisabled);
        assert!(spot::is_enabled(spot_config), EMarketClosed);
        assert!(spot::is_open(record), EMarketClosed);
        assert!(policy.status == STATUS_ACTIVE, EPolicyNotActive);
        assert!(tx_context::sender(ctx) == policy.insured, ENotPolicyOwner);
        assert!(policy.market_id == spot::get_id_address(record), EMarketMismatch);
        assert!(policy.vault_id == object::id(vault), EInvalidVault);

        let now = clock::timestamp_ms(clock);
        assert!(now < policy.expiry_time_ms, EPolicyExpired);

        let total_duration = policy.expiry_time_ms - policy.start_time_ms;
        let remaining = policy.expiry_time_ms - now;
        let refund_u128 = (policy.premium_paid as u128) * (remaining as u128) / (total_duration as u128);
        assert!(refund_u128 <= (MAX_U64 as u128), EOverflow);
        let original_refund = refund_u128 as u64;

        let fee = (original_refund * config.fee_bps) / BPS_DENOM;
        let net_refund = original_refund - fee;
        // original_refund == fee + net_refund; ensure vault can fund both splits
        let capital_value = balance::value(&vault.capital);
        assert!(capital_value >= original_refund, EInsufficientCapital);
        
        if (fee > 0) {
            let fee_balance = balance::split(&mut vault.capital, fee);
            let fee_coin = coin::from_balance(fee_balance, ctx);
            transfer::public_transfer(fee_coin, profile::get_treasury_address(treasury));
        };

        if (net_refund > 0) {
            let refund_balance = balance::split(&mut vault.capital, net_refund);
            let refund_coin = coin::from_balance(refund_balance, ctx);
            transfer::public_transfer(refund_coin, policy.insured);
        };

        let reserve_amount = compute_reserve(policy.covered_amount, policy.coverage_bps);
        release_exposure(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
        assert!(vault.reserved >= reserve_amount, EOverflow);
        vault.reserved = vault.reserved - reserve_amount;
        policy.status = STATUS_CANCELLED;

        event::emit(CoverageCancelledEvent {
            policy_id: object::id(policy),
            insured: policy.insured,
            refunded_amount: net_refund,
            fee_paid: fee,
        });
    }

    /// Claim payout after SPoT resolution
    /// Payout is calculated as min(current_position, covered_amount) * coverage_bps / BPS_DENOM
    /// Dynamic coverage: payout adjusts if user reduces their SPoT position after buying insurance.
    /// This prevents exploitation where user buys insurance then exits bet.
    public entry fun claim(
        config: &InsuranceConfig,
        spot_config: &spot::SpotConfig,
        vault: &mut UnderwriterVault,
        record: &spot::SpotRecord,
        policy: &mut CoveragePolicy,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(config.enable_flag, EDisabled);
        assert!(spot::is_enabled(spot_config), EMarketClosed);
        assert!(policy.status == STATUS_ACTIVE, EPolicyNotActive);
        assert!(tx_context::sender(ctx) == policy.insured, ENotPolicyOwner);
        assert!(policy.market_id == spot::get_id_address(record), EMarketMismatch);
        assert!(policy.vault_id == object::id(vault), EInvalidVault);
        assert!(spot::is_resolved(record), EMarketClosed);

        let now = clock::timestamp_ms(clock);
        assert!(now <= policy.expiry_time_ms, EPolicyExpired);

        let outcome_opt = spot::get_outcome(record);
        assert!(option::is_some(outcome_opt), EMarketClosed);
        let outcome = *option::borrow(outcome_opt);

        let mut payout = 0;
        if (outcome != spot::outcome_draw() && outcome != spot::outcome_unapplicable()) {
            if (outcome != policy.option_id) {
                // Dynamic coverage: payout adjusts if user reduces their SPoT position after buying insurance
                let current_position = spot::get_user_option_amount(record, policy.insured, policy.option_id);
                let eligible_amount = if (current_position < policy.covered_amount) {
                    current_position
                } else {
                    policy.covered_amount
                };
                let payout_u128 = (eligible_amount as u128) * (policy.coverage_bps as u128) / (BPS_DENOM as u128);
                assert!(payout_u128 <= (MAX_U64 as u128), EOverflow);
                payout = payout_u128 as u64;
            };
        };

        if (payout > 0) {
            let payout_balance = balance::split(&mut vault.capital, payout);
            let payout_coin = coin::from_balance(payout_balance, ctx);
            transfer::public_transfer(payout_coin, policy.insured);
        };

        let reserve_amount = compute_reserve(policy.covered_amount, policy.coverage_bps);
        release_exposure(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
        assert!(vault.reserved >= reserve_amount, EOverflow);
        vault.reserved = vault.reserved - reserve_amount;
        policy.status = STATUS_CLAIMED;

        event::emit(CoverageClaimedEvent {
            policy_id: object::id(policy),
            insured: policy.insured,
            payout,
        });
    }

    /// Expire policy and release reserves
    public entry fun expire_policy(
        vault: &mut UnderwriterVault,
        policy: &mut CoveragePolicy,
        clock: &Clock
    ) {
        if (policy.status != STATUS_ACTIVE) {
            return
        };
        if (policy.vault_id != object::id(vault)) {
            return
        };
        let now = clock::timestamp_ms(clock);
        if (now < policy.expiry_time_ms) {
            return
        };

        let reserve_amount = compute_reserve(policy.covered_amount, policy.coverage_bps);
        release_exposure(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
        assert!(vault.reserved >= reserve_amount, EOverflow);
        vault.reserved = vault.reserved - reserve_amount;
        policy.status = STATUS_EXPIRED;

        event::emit(PolicyExpiredEvent {
            policy_id: object::id(policy),
            insured: policy.insured,
            market_id: policy.market_id,
            vault_id: policy.vault_id,
            reserve_released: reserve_amount,
            expiry_time_ms: policy.expiry_time_ms,
        });
    }

    fun compute_reserve(covered_amount: u64, coverage_bps: u64): u64 {
        let reserve_u128 = (covered_amount as u128) * (coverage_bps as u128);
        let reserve_u128 = reserve_u128 / (BPS_DENOM as u128);
        assert!(reserve_u128 <= (MAX_U64 as u128), EOverflow);
        reserve_u128 as u64
    }

    fun enforce_exposure_limits(
        vault: &mut UnderwriterVault,
        market_id: address,
        insured: address,
        option_id: u8,
        reserve_amount: u64,
        ctx: &mut TxContext
    ) {
        // Read limit values before creating mutable borrows
        let max_exposure_per_market = vault.max_exposure_per_market;
        let max_exposure_per_user = vault.max_exposure_per_user;
        let max_exposure_per_option = vault.max_exposure_per_option;
        
        // Check user exposure limit first (doesn't require market exposure)
        if (max_exposure_per_user > 0) {
            let current_user = get_user_exposure(vault, insured);
            assert!(current_user <= MAX_U64 - reserve_amount, EOverflow);
            let new_user = current_user + reserve_amount;
            assert!(new_user <= max_exposure_per_user, EExposureLimit);
        };

        // Now get mutable reference to market exposure for market and option checks
        let exposure = get_market_exposure_mut(vault, market_id, ctx);
        
        if (max_exposure_per_market > 0) {
            assert!(exposure.total_reserved <= MAX_U64 - reserve_amount, EOverflow);
            let new_total = exposure.total_reserved + reserve_amount;
            assert!(new_total <= max_exposure_per_market, EExposureLimit);
        };

        let option_reserved = get_option_reserved(exposure, option_id);
        assert!(option_reserved <= MAX_U64 - reserve_amount, EOverflow);
        let new_opt_reserved = option_reserved + reserve_amount;
        if (max_exposure_per_option > 0) {
            assert!(new_opt_reserved <= max_exposure_per_option, EExposureLimit);
        };
    }

    fun add_exposure(
        vault: &mut UnderwriterVault,
        market_id: address,
        insured: address,
        option_id: u8,
        reserve_amount: u64,
        ctx: &mut TxContext
    ) {
        let exposure = get_market_exposure_mut(vault, market_id, ctx);
        assert!(exposure.total_reserved <= MAX_U64 - reserve_amount, EOverflow);
        exposure.total_reserved = exposure.total_reserved + reserve_amount;
        let option_reserved = get_option_reserved(exposure, option_id);
        assert!(option_reserved <= MAX_U64 - reserve_amount, EOverflow);
        let new_option_reserved = option_reserved + reserve_amount;
        set_option_reserved(exposure, option_id, new_option_reserved);

        let current_user = get_user_exposure(vault, insured);
        assert!(current_user <= MAX_U64 - reserve_amount, EOverflow);
        let new_user = current_user + reserve_amount;
        set_user_exposure(vault, insured, new_user);
    }

    fun release_exposure(
        vault: &mut UnderwriterVault,
        market_id: address,
        insured: address,
        option_id: u8,
        reserve_amount: u64
    ) {
        if (reserve_amount == 0) {
            return
        };
        
        assert!(table::contains(&vault.market_exposures, market_id), EExposureInvariantBroken);
        let exposure = table::borrow_mut(&mut vault.market_exposures, market_id);
        assert!(exposure.total_reserved >= reserve_amount, EExposureInvariantBroken);
        exposure.total_reserved = exposure.total_reserved - reserve_amount;

        assert!(table::contains(&exposure.reserved_by_option, option_id), EExposureInvariantBroken);
        let current_option = *table::borrow(&exposure.reserved_by_option, option_id);
        assert!(current_option >= reserve_amount, EExposureInvariantBroken);
        let option_ref = table::borrow_mut(&mut exposure.reserved_by_option, option_id);
        *option_ref = current_option - reserve_amount;

        assert!(table::contains(&vault.user_exposures, insured), EExposureInvariantBroken);
        let current_user = *table::borrow(&vault.user_exposures, insured);
        assert!(current_user >= reserve_amount, EExposureInvariantBroken);
        let user_ref = table::borrow_mut(&mut vault.user_exposures, insured);
        *user_ref = current_user - reserve_amount;
    }

    fun get_market_exposure_mut(
        vault: &mut UnderwriterVault,
        market_id: address,
        ctx: &mut TxContext
    ): &mut MarketExposure {
        if (!table::contains(&vault.market_exposures, market_id)) {
            let exposure = MarketExposure {
                market_id,
                total_reserved: 0,
                reserved_by_option: table::new(ctx),
            };
            table::add(&mut vault.market_exposures, market_id, exposure);
        };
        table::borrow_mut(&mut vault.market_exposures, market_id)
    }

    fun get_user_exposure(vault: &UnderwriterVault, insured: address): u64 {
        if (table::contains(&vault.user_exposures, insured)) {
            *table::borrow(&vault.user_exposures, insured)
        } else {
            0
        }
    }

    fun set_user_exposure(vault: &mut UnderwriterVault, insured: address, amount: u64) {
        if (table::contains(&vault.user_exposures, insured)) {
            let user_ref = table::borrow_mut(&mut vault.user_exposures, insured);
            *user_ref = amount;
        } else {
            table::add(&mut vault.user_exposures, insured, amount);
        };
    }

    fun get_option_reserved(exposure: &MarketExposure, option_id: u8): u64 {
        if (table::contains(&exposure.reserved_by_option, option_id)) {
            *table::borrow(&exposure.reserved_by_option, option_id)
        } else {
            0
        }
    }

    fun set_option_reserved(exposure: &mut MarketExposure, option_id: u8, amount: u64) {
        if (table::contains(&exposure.reserved_by_option, option_id)) {
            let option_ref = table::borrow_mut(&mut exposure.reserved_by_option, option_id);
            *option_ref = amount;
        } else {
            table::add(&mut exposure.reserved_by_option, option_id, amount);
        };
    }

    /// Migration function for InsuranceConfig
    public entry fun migrate_config(
        config: &mut InsuranceConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(config.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = config.version;
        config.version = current_version;
        
        // Emit event for object migration
        let config_id = object::id(config);
        upgrade::emit_migration_event(
            config_id,
            string::utf8(b"InsuranceConfig"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Migration function for UnderwriterVault
    public entry fun migrate_vault(
        vault: &mut UnderwriterVault,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(vault.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = vault.version;
        vault.version = current_version;
        
        // Emit event for object migration
        let vault_id = object::id(vault);
        upgrade::emit_migration_event(
            vault_id,
            string::utf8(b"UnderwriterVault"),
            old_version,
            tx_context::sender(ctx)
        );
    }
}
