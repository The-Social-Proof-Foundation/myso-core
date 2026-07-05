// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Social Proof Tokens module for the MySocial network.
/// This module provides functionality for creation and trading of both profile tokens
/// and post tokens using an Automated Market Maker (AMM) with a quadratic pricing curve.
/// It includes fee distribution mechanisms for transactions, splitting between profile owner,
/// platform, and ecosystem treasury.
///
/// **SPT amounts (nano-SPT):** `SocialToken.amount`, `TokenInfo.circulating_supply`, pool holder
/// balances, and SPT quantity fields in events are stored in fixed-point **nano-SPT** units:
/// `10^9` nano-SPT = `1.0` display token (same decimal count as native MYSO). MYSO payments and
/// prices remain in MYSO smallest units. Buy/sell cost uses a continuous integral of the marginal
/// price curve over nano-supply to stay well-defined at sub-token precision.
///
/// Use `nano_spt_from_whole_tokens` / `nano_spt_from_whole_and_fraction` to convert display amounts
/// plus optional sub-token nano remainder into the `u64` nano-SPT values passed to buy/sell entrypoints.

#[allow(unused_field, deprecated_usage, unused_const, duplicate_alias, unused_use, lint(public_entry))]
module social_contracts::social_proof_tokens {
    use std::string::{Self, String};
    use std::option::{Self, Option};
    use std::vector;
    use std::u256;

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        event,
        table::{Self, Table},
        coin::{Self, Coin},
        myso::MYSO,
        balance::{Self, Balance},
        clock::{Self, Clock},
        package::{Self, Publisher}
    };
    
    use social_contracts::profile::{Self, Profile, UsernameRegistry, EcosystemTreasury};
    use social_contracts::post::{Self, Post};
    use social_contracts::poc_vault::{Self as poc_vault, PoCBeneficiaryVault};
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::platform::{Self, PlatformRegistry};
    use social_contracts::upgrade::{Self, UpgradeAdminCap};

    // === Error codes ===
    /// Operation can only be performed by the admin
    const ENotAuthorized: u64 = 0;
    /// Invalid fee percentages configuration
    const EInvalidFeeConfig: u64 = 1;
    /// The token already exists
    const ETokenAlreadyExists: u64 = 2;
    /// The token does not exist
    const ETokenNotFound: u64 = 3;
    /// Exceeded maximum token hold percentage
    const EExceededMaxHold: u64 = 4;
    /// Insufficient funds for operation
    const EInsufficientFunds: u64 = 5;
    /// Sender doesn't own any tokens
    const ENoTokensOwned: u64 = 6;
    /// Invalid post or profile ID
    const EInvalidID: u64 = 7;
    /// Insufficient token liquidity
    const EInsufficientLiquidity: u64 = 8;
    /// Token already initialized in pool
    const ETokenAlreadyInitialized: u64 = 9;
    /// Curve parameters must be positive
    const EInvalidCurveParams: u64 = 10;
    /// Invalid token type
    const EInvalidTokenType: u64 = 11;
    /// Viral threshold not met
    const EViralThresholdNotMet: u64 = 12;
    /// Auction already in progress
    const EAuctionInProgress: u64 = 13;
    /// Invalid auction duration
    const EInvalidAuctionDuration: u64 = 14;
    /// Auction not active
    const EAuctionNotActive: u64 = 15;
    /// Auction not ended
    const EAuctionNotEnded: u64 = 16;
    /// Auction already finalized
    const EAuctionAlreadyFinalized: u64 = 17;
    /// No contribution to auction
    const ENoContribution: u64 = 18;
    /// Cannot buy token from a blocked user
    const EBlockedUser: u64 = 19;
    /// Trading is halted by emergency kill switch
    const ETradingHalted: u64 = 20;
    /// Arithmetic overflow detected
    const EOverflow: u64 = 21;
    /// Wrong version - object version mismatch
    const EWrongVersion: u64 = 22;
    /// User has not joined the platform
    const EUserNotJoinedPlatform: u64 = 23;
    /// User is blocked by the platform
    const EUserBlockedByPlatform: u64 = 24;
    /// Reservation pool already converted to token
    const EReservationPoolConverted: u64 = 25;
    /// User already owns tokens for this pool
    const EAlreadyOwnsTokens: u64 = 26;
    /// Too many reservers for conversion (DoS prevention)
    const ETooManyReservers: u64 = 27;
    /// Cannot split token - amount must be positive and less than token amount
    const ECannotSplit: u64 = 28;
    /// Cannot merge tokens - tokens must be from the same pool
    const ECannotMerge: u64 = 29;
    /// Post token pools in on-post PoC escrow mode require an entrypoint that supplies `&Post` for PoC-aware fee routing.
    const EPostPoolEscrowTradingBlocked: u64 = 30;

    // === Constants ===
    // Token types
    const TOKEN_TYPE_PROFILE: u8 = 1;
    const TOKEN_TYPE_POST: u8 = 2;

    // Default trading fee percentages (in basis points; `BPS_DENOM` = 100%)
    const DEFAULT_TRADING_CREATOR_FEE_BPS: u64 = 100; // 1.0% to creator (profile/post owner)
    const DEFAULT_TRADING_PLATFORM_FEE_BPS: u64 = 25; // 0.25% to platform
    const DEFAULT_TRADING_TREASURY_FEE_BPS: u64 = 25; // 0.25% to ecosystem treasury

    // Default reservation fee percentages (in basis points; `BPS_DENOM` = 100%)
    const DEFAULT_RESERVATION_CREATOR_FEE_BPS: u64 = 100; // 1.0% to creator (profile/post owner)
    const DEFAULT_RESERVATION_PLATFORM_FEE_BPS: u64 = 25; // 0.25% to platform
    const DEFAULT_RESERVATION_TREASURY_FEE_BPS: u64 = 25; // 0.25% to ecosystem treasury

    // Default non-platform fee split: when no platform is associated, the configured platform-fee
    // bucket is split between creator and ecosystem treasury by these bps (must sum to `BPS_DENOM`).
    const DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS: u64 = 5000; // 50% of platform-fee bucket to creator
    const DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS: u64 = 5000; // 50% of platform-fee bucket to ecosystem treasury

    // Maximum single-wallet hold of circulating supply after launch (default **5%** = `500` bps of supply; admin may raise up to `MAX_SPT_ADMIN_PERCENT_BPS`).
    const MAX_HOLD_PERCENT_BPS: u64 = 500;

    // --- Widen bounds: on-chain MYSO / nano-SPT amounts are `u64` ---
    /// **Permyriad / fee scale only:** `10_000` = 100%. Not SPT decimal places (`SPT_DECIMALS` = 9).
    const BPS_DENOM: u64 = 10000;

    /// Numeric value of `MAX_U64`; max storable on-chain amount as `u128` for safe compares / products.
    const MAX_ONCHAIN_U64_U128: u128 = 18446744073709551615u128;

    /// Max `threshold * max_individual_reservation_bps` so `(product / BPS_DENOM)` fits `u64` (equals `MAX_U64 * BPS_DENOM`).
    const MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128: u128 =
        MAX_ONCHAIN_U64_U128 * (BPS_DENOM as u128);

    // Package-enforced ceilings on what the admin may store in `SocialProofTokensConfig` (raise via upgrade).
    // Actual `max_hold_percent_bps`, `max_individual_reservation_bps`, and `max_reservers_per_pool` are set
    // via `update_social_proof_tokens_config` with `SocialProofTokensAdminCap`, subject to these caps.
    // Fresh installs still default to **5%** live hold (`MAX_HOLD_PERCENT_BPS`) and **20%** per-reserver reservation (`DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS`).
    const MAX_SPT_ADMIN_PERCENT_BPS: u64 = 1_000_000;

    const MAX_RESERVERS_PER_POOL_CAP: u64 = 10_000_000;

    /// Display decimals for SPT (matches native MYSO coin metadata).
    const SPT_DECIMALS: u8 = 9;
    /// Nano-SPT per 1.0 whole display token.
    const SPT_SCALE: u64 = 1_000_000_000;

    // Default AMM curve parameters
    const DEFAULT_BASE_PRICE: u64 = 1_000_000_000; // 1.0 MYSO in smallest units
    const DEFAULT_QUADRATIC_COEFFICIENT: u64 = 100_000; // Coefficient for quadratic curve

    // Reservation threshold constants for social proof token creation
    const DEFAULT_POST_THRESHOLD: u64 = 1_000_000_000_000; // 1,000 MYSO in smallest units (9 decimals)
    const DEFAULT_PROFILE_THRESHOLD: u64 = 10_000_000_000_000; // 10,000 MYSO in smallest units (9 decimals)
    const DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS: u64 = 2000; // default **20%** of pool threshold per reserver (`2000` / `BPS_DENOM`)
    const DEFAULT_MAX_RESERVERS_PER_POOL: u64 = 1000;

    // Maximum u64 value for overflow protection
    const MAX_U64: u64 = 18446744073709551615;

    // === Structs ===

    /// Admin capability for the social proof tokens system
    public struct SocialProofTokensAdminCap has key, store {
        id: UID,
    }

    /// Global social proof tokens configuration
    public struct SocialProofTokensConfig has key {
        id: UID,
        /// Version for upgrades
        version: u64,
        /// Creator fee percentage in basis points (for trading)
        trading_creator_fee_bps: u64,
        /// Platform fee percentage in basis points (for trading)
        trading_platform_fee_bps: u64,
        /// Treasury fee percentage in basis points (for trading)
        trading_treasury_fee_bps: u64,
        /// Creator reservation fee percentage in basis points
        reservation_creator_fee_bps: u64,
        /// Platform reservation fee percentage in basis points
        reservation_platform_fee_bps: u64,
        /// Treasury reservation fee percentage in basis points
        reservation_treasury_fee_bps: u64,
        /// Base price for new tokens
        base_price: u64,
        /// Quadratic coefficient for pricing curve
        quadratic_coefficient: u64,
        /// Max fraction of circulating supply one wallet may hold, in basis points (`10_000` = 100%).
        max_hold_percent_bps: u64,
        /// Reservation thresholds for social proof token creation
        post_threshold: u64,
        profile_threshold: u64,
        /// Max MYSO reservation per wallet vs pool threshold, in bps (`10_000` = 100% of threshold).
        max_individual_reservation_bps: u64,
        /// Maximum unique reservers per reservation pool (DoS / gas bound).
        max_reservers_per_pool: u64,
        /// Emergency kill switch - when false, all trading is halted
        trading_enabled: bool,
        /// Non-platform path: share of the platform-fee bucket routed to the creator (bps of that bucket).
        non_platform_platform_to_creator_bps: u64,
        /// Non-platform path: share of the platform-fee bucket routed to the ecosystem treasury (bps of that bucket).
        non_platform_platform_to_treasury_bps: u64,
    }

    /// Registry of all tokens in the exchange
    public struct TokenRegistry has key {
        id: UID,
        /// Table keyed by associated_id (post/profile ID), not pool ID, to token info
        tokens: Table<address, TokenInfo>,
        /// Table from profile/post ID to reservation pool info
        reservation_pools: Table<address, ReservationPool>,
        /// Version for upgrades
        version: u64,
    }

    /// Reservation pool for a specific post or profile
    /// Note: reservers vector is only stored in ReservationPoolObject, not in registry
    public struct ReservationPool has store, drop {
        /// Associated profile or post ID
        associated_id: address,
        /// Token type (1=profile, 2=post)
        token_type: u8,
        /// Owner of the profile/post
        owner: address,
        /// Total MYSO reserved towards this post/profile
        total_reserved: u64,
        /// Required threshold to enable auction creation
        required_threshold: u64,
        /// Creation timestamp
        created_at: u64,
    }

    /// Information about a token
    public struct TokenInfo has store, drop {
        /// The token ID (object ID of the pool)
        id: address,
        /// Type of token (1=profile, 2=post)
        token_type: u8,
        /// Owner/creator of the token
        owner: address,
        /// Associated profile or post ID
        associated_id: address,
        /// Circulating supply in **nano-SPT** (`10^9` per 1.0 token).
        circulating_supply: u64,
        /// Base price for this token
        base_price: u64,
        /// Quadratic coefficient for this token's pricing curve
        quadratic_coefficient: u64,
        /// Creation timestamp
        created_at: u64,
    }

    /// Liquidity pool for a token (key only - not transferable)
    public struct TokenPool has key {
        id: UID,
        /// The token's info
        info: TokenInfo,
        /// MYSO balance in the pool
        myso_balance: Balance<MYSO>,
        /// Holder balances in **nano-SPT**.
        holders: Table<address, u64>,
        /// PoC revenue redirection address (for post tokens only)
        poc_redirect_to: Option<address>,
        /// PoC revenue redirection percentage (for post tokens only)
        poc_redirect_percentage: Option<u64>,
        /// Mirrors `post::poc_redirection_kind` for fee routing (`0` = none, `1` wallet, `2` escrow)
        poc_redirection_kind: u8,
        /// Version for upgrades
        version: u64,
    }

    /// Social token that represents a user's owned tokens.
    /// Intentionally has only `key` (no `store`) to prevent free P2P transfer: the pool
    /// `holders` table is keyed by transaction sender, so transferring this object between
    /// addresses would permanently desynchronise the on-chain balance state.
    public struct SocialToken has key {
        id: UID,
        /// Token pool ID
        pool_id: address,
        /// Token type (1=profile, 2=post)
        token_type: u8,
        /// Balance in **nano-SPT** (`10^9` per 1.0 token).
        amount: u64,
    }

    /// Reservation pool for collecting MYSO reservations towards posts/profiles
    public struct ReservationPoolObject has key {
        id: UID,
        /// Reservation pool info (without reservers - kept separately below)
        info: ReservationPool,
        /// MYSO balance reserved in this pool
        myso_balance: Balance<MYSO>,
        /// Mapping of reservers' addresses to their reservation amounts
        reservations: Table<address, u64>,
        /// List of all reservers (for efficient iteration) - only in object, not in registry
        reservers: vector<address>,
        /// Flag indicating if this pool has been converted to a token
        converted: bool,
        /// Version for upgrades
        version: u64,
    }

    // === Events ===

    /// Event emitted when a token pool is created
    public struct TokenPoolCreatedEvent has copy, drop {
        id: address,
        token_type: u8,
        owner: address,
        associated_id: address,
        base_price: u64,
        quadratic_coefficient: u64,
        /// Circulating supply in **nano-SPT** (`10^9` per 1.0 token).
        circulating_supply: u64,
        /// MYSO reserved (smallest units).
        total_reserved_at_launch: u64,
    }

    /// Event emitted when a post pool is auto-initialized by SPoT flow
    /// Event emitted when tokens are bought
    public struct TokenBoughtEvent has copy, drop {
        id: address,
        buyer: address,
        /// SPT quantity in **nano-SPT**.
        amount: u64,
        myso_amount: u64,
        fee_amount: u64,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
        new_price: u64,
    }

    /// Event emitted when tokens are sold
    public struct TokenSoldEvent has copy, drop {
        id: address,
        seller: address,
        /// SPT quantity in **nano-SPT**.
        amount: u64,
        myso_amount: u64,
        fee_amount: u64,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
        new_price: u64,
    }

    /// Event emitted when MYSO is reserved towards a post/profile
    public struct ReservationCreatedEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        reserver: address,
        amount: u64,
        total_reserved: u64,
        threshold_met: bool,
        reserved_at: u64,
        fee_amount: u64,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
    }

    /// Event emitted when MYSO reservation is withdrawn
    public struct ReservationWithdrawnEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        reserver: address,
        amount: u64,
        total_reserved: u64,
        withdrawn_at: u64,
        fee_amount: u64,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
    }

    /// Event emitted when reservation threshold is met for the first time
    public struct ThresholdMetEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        owner: address,
        total_reserved: u64,
        required_threshold: u64,
        timestamp: u64,
    }

    /// Event emitted when a reservation pool is created
    public struct ReservationPoolCreatedEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        owner: address,
        required_threshold: u64,
        pool_object_id: address,
        created_at: u64,
    }

    /// Event emitted when social proof tokens config is updated
    public struct ConfigUpdatedEvent has copy, drop {
        /// Who performed the update
        updated_by: address,
        /// When the update occurred
        timestamp: u64,
        /// Trading fee percentages
        total_fee_bps: u64,
        trading_creator_fee_bps: u64,
        trading_platform_fee_bps: u64,
        trading_treasury_fee_bps: u64,
        /// Reservation fee percentages
        reservation_total_fee_bps: u64,
        reservation_creator_fee_bps: u64,
        reservation_platform_fee_bps: u64,
        reservation_treasury_fee_bps: u64,
        /// Curve parameters
        base_price: u64,
        quadratic_coefficient: u64,
        /// Maximum hold percentage
        max_hold_percent_bps: u64,
        /// Reservation thresholds
        post_threshold: u64,
        profile_threshold: u64,
        max_individual_reservation_bps: u64,
        max_reservers_per_pool: u64,
        /// Non-platform split: creator share of platform-fee bucket (bps)
        non_platform_platform_to_creator_bps: u64,
        /// Non-platform split: ecosystem treasury share of platform-fee bucket (bps)
        non_platform_platform_to_treasury_bps: u64,
        /// Whether SPT trading is enabled
        trading_enabled: bool,
    }

    /// Event emitted when tokens are purchased by someone who already has a social token
    public struct TokensAddedEvent has copy, drop {
        owner: address, 
        pool_id: address,
        /// SPT quantity in **nano-SPT**.
        amount: u64,
    }

    /// Event emitted when emergency kill switch is toggled
    public struct EmergencyKillSwitchEvent has copy, drop {
        /// Admin who activated/deactivated the kill switch
        admin: address,
        /// New state of trading (true = enabled, false = halted)
        trading_enabled: bool,
        /// Timestamp of the action
        timestamp: u64,
        /// Reason for the action (optional)
        reason: String,
    }

    /// Event emitted when PoC redirection data is updated for a token pool
    public struct PocRedirectionUpdatedEvent has copy, drop {
        /// Token pool ID
        pool_id: address,
        /// Associated post ID
        post_id: address,
        /// Address to redirect revenue to (None if cleared)
        redirect_to: Option<address>,
        /// Percentage of revenue to redirect (None if cleared)
        redirect_percentage: Option<u64>,
        /// Mirrors post PoC redirect kind (`0` none, `1` wallet, `2` escrow)
        poc_redirection_kind: u8,
        /// Who performed the update
        updated_by: address,
        /// Timestamp of the update
        timestamp: u64,
    }

    // === SocialToken Split and Merge Functions ===

    /// Split a SocialToken into two tokens
    /// Returns a new SocialToken with the specified amount
    /// The original token's amount is reduced by the split amount
    public fun split_social_token(
        token: &mut SocialToken,
        split_amount: u64,
        ctx: &mut TxContext
    ): SocialToken {
        // Validation
        assert!(split_amount > 0, ECannotSplit);
        assert!(token.amount >= split_amount, EInsufficientFunds);
        assert!(split_amount < token.amount, ECannotSplit);
        
        // Update original token
        token.amount = token.amount - split_amount;
        
        // Create new token
        SocialToken {
            id: object::new(ctx),
            pool_id: token.pool_id,
            token_type: token.token_type,
            amount: split_amount,
        }
    }

    /// Merge two SocialTokens from the same pool
    /// Consumes the second token and adds its amount to the first
    /// Both tokens must have the same pool_id and token_type
    public fun merge_social_tokens(
        token1: &mut SocialToken,
        token2: SocialToken
    ) {
        // Validation
        assert!(token1.pool_id == token2.pool_id, ECannotMerge);
        assert!(token1.token_type == token2.token_type, ECannotMerge);
        assert!(token1.amount <= MAX_U64 - token2.amount, EOverflow);
        
        // Merge amounts
        token1.amount = token1.amount + token2.amount;
        
        // Destroy second token's ID
        let SocialToken { id, pool_id: _, token_type: _, amount: _ } = token2;
        object::delete(id);
    }

    /// Entry function to split a SocialToken
    public entry fun split_social_token_entry(
        token: &mut SocialToken,
        split_amount: u64,
        ctx: &mut TxContext
    ) {
        let new_token = split_social_token(token, split_amount, ctx);
        transfer::transfer(new_token, tx_context::sender(ctx));
    }

    /// Entry function to merge two SocialTokens
    public entry fun merge_social_tokens_entry(
        token1: &mut SocialToken,
        token2: SocialToken,
        _ctx: &mut TxContext
    ) {
        merge_social_tokens(token1, token2);
    }

    // === Test Helpers ===

    /// Get the amount of a SocialToken (test only)
    #[test_only]
    public fun amount(token: &SocialToken): u64 {
        token.amount
    }

    /// Get the pool_id of a SocialToken (test only)
    #[test_only]
    public fun pool_id(token: &SocialToken): address {
        token.pool_id
    }

    /// Get the token_type of a SocialToken (test only)
    #[test_only]
    public fun token_type(token: &SocialToken): u8 {
        token.token_type
    }

    /// Create a SocialToken for testing
    #[test_only]
    public fun create_social_token_for_testing(
        pool_id: address,
        token_type: u8,
        amount: u64,
        ctx: &mut TxContext
    ): SocialToken {
        SocialToken {
            id: object::new(ctx),
            pool_id,
            token_type,
            amount,
        }
    }

    /// Transfer a SocialToken to an address from within tests.
    /// Needed because SocialToken has only `key` (no `store`), so external code cannot call
    /// `transfer::public_transfer` on it; this helper delegates to the module-internal variant.
    #[test_only]
    public fun transfer_social_token_for_testing(token: SocialToken, to: address) {
        transfer::transfer(token, to)
    }

    /// Destroy a SocialToken in tests without going through a sell transaction.
    #[test_only]
    public fun destroy_social_token_for_testing(token: SocialToken) {
        let SocialToken { id, pool_id: _, token_type: _, amount: _ } = token;
        object::delete(id)
    }

    /// Bootstrap initialization function - creates the social proof tokens configuration and registry
    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        let config = SocialProofTokensConfig {
            id: object::new(ctx),
            version: upgrade::current_version(),
            trading_creator_fee_bps: DEFAULT_TRADING_CREATOR_FEE_BPS,
            trading_platform_fee_bps: DEFAULT_TRADING_PLATFORM_FEE_BPS,
            trading_treasury_fee_bps: DEFAULT_TRADING_TREASURY_FEE_BPS,
            reservation_creator_fee_bps: DEFAULT_RESERVATION_CREATOR_FEE_BPS,
            reservation_platform_fee_bps: DEFAULT_RESERVATION_PLATFORM_FEE_BPS,
            reservation_treasury_fee_bps: DEFAULT_RESERVATION_TREASURY_FEE_BPS,
            base_price: DEFAULT_BASE_PRICE,
            quadratic_coefficient: DEFAULT_QUADRATIC_COEFFICIENT,
            max_hold_percent_bps: MAX_HOLD_PERCENT_BPS,
            post_threshold: DEFAULT_POST_THRESHOLD,
            profile_threshold: DEFAULT_PROFILE_THRESHOLD,
            max_individual_reservation_bps: DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS,
            max_reservers_per_pool: DEFAULT_MAX_RESERVERS_PER_POOL,
            trading_enabled: true,
            non_platform_platform_to_creator_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS,
            non_platform_platform_to_treasury_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS,
        };

        // Emit event so indexer can populate spt_exchange_config table
        let total_fee_bps = DEFAULT_TRADING_CREATOR_FEE_BPS + DEFAULT_TRADING_PLATFORM_FEE_BPS + DEFAULT_TRADING_TREASURY_FEE_BPS;
        let reservation_total_fee_bps = DEFAULT_RESERVATION_CREATOR_FEE_BPS + DEFAULT_RESERVATION_PLATFORM_FEE_BPS + DEFAULT_RESERVATION_TREASURY_FEE_BPS;
        event::emit(ConfigUpdatedEvent {
            updated_by: sender,
            timestamp: clock::timestamp_ms(clock),
            total_fee_bps,
            trading_creator_fee_bps: DEFAULT_TRADING_CREATOR_FEE_BPS,
            trading_platform_fee_bps: DEFAULT_TRADING_PLATFORM_FEE_BPS,
            trading_treasury_fee_bps: DEFAULT_TRADING_TREASURY_FEE_BPS,
            reservation_total_fee_bps,
            reservation_creator_fee_bps: DEFAULT_RESERVATION_CREATOR_FEE_BPS,
            reservation_platform_fee_bps: DEFAULT_RESERVATION_PLATFORM_FEE_BPS,
            reservation_treasury_fee_bps: DEFAULT_RESERVATION_TREASURY_FEE_BPS,
            base_price: DEFAULT_BASE_PRICE,
            quadratic_coefficient: DEFAULT_QUADRATIC_COEFFICIENT,
            max_hold_percent_bps: MAX_HOLD_PERCENT_BPS,
            post_threshold: DEFAULT_POST_THRESHOLD,
            profile_threshold: DEFAULT_PROFILE_THRESHOLD,
            max_individual_reservation_bps: DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS,
            max_reservers_per_pool: DEFAULT_MAX_RESERVERS_PER_POOL,
            non_platform_platform_to_creator_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS,
            non_platform_platform_to_treasury_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS,
            trading_enabled: true,
        });

        // Create and share social proof tokens config with proper treasury
        transfer::share_object(config);
        
        // Create and share token registry
        transfer::share_object(
            TokenRegistry {
                id: object::new(ctx),
                tokens: table::new(ctx),
                reservation_pools: table::new(ctx),
                version: upgrade::current_version(),
            }
        );
    }

    // === Admin Functions ===

    /// Update social proof tokens configuration
    public entry fun update_social_proof_tokens_config(
        _admin_cap: &SocialProofTokensAdminCap,
        config: &mut SocialProofTokensConfig,
        trading_creator_fee_bps: u64,
        trading_platform_fee_bps: u64,
        trading_treasury_fee_bps: u64,
        reservation_creator_fee_bps: u64,
        reservation_platform_fee_bps: u64,
        reservation_treasury_fee_bps: u64,
        base_price: u64,
        quadratic_coefficient: u64,
        max_hold_percent_bps: u64,
        post_threshold: u64,
        profile_threshold: u64,
        max_individual_reservation_bps: u64,
        max_reservers_per_pool: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Verify curve parameters are valid
        assert!(base_price > 0 && quadratic_coefficient > 0, EInvalidCurveParams);
        
        // Validate fee configurations to prevent division by zero and overflow
        // Calculate totals before updating to validate
        let total_fee_bps = trading_creator_fee_bps + trading_platform_fee_bps + trading_treasury_fee_bps;
        let reservation_total_fee_bps = reservation_creator_fee_bps + reservation_platform_fee_bps + reservation_treasury_fee_bps;
        
        // Ensure fee totals are valid (prevent division by zero and overflow)
        assert!(total_fee_bps > 0 && total_fee_bps <= BPS_DENOM, EInvalidFeeConfig);
        assert!(reservation_total_fee_bps > 0 && reservation_total_fee_bps <= BPS_DENOM, EInvalidFeeConfig);
        
        // Validate individual fee components don't exceed 100%
        
        // Hold cap: bps may exceed 10_000 (100%) so whales can hold >100% of supply if policy allows.
        assert!(max_hold_percent_bps > 0 && max_hold_percent_bps <= MAX_SPT_ADMIN_PERCENT_BPS, EInvalidFeeConfig);
        
        // Validate thresholds (must be positive)
        assert!(post_threshold > 0, EInvalidFeeConfig);
        assert!(profile_threshold > 0, EInvalidFeeConfig);
        
        assert!(max_individual_reservation_bps > 0 && max_individual_reservation_bps <= MAX_SPT_ADMIN_PERCENT_BPS, EInvalidFeeConfig);

        // `(threshold * bps) / BPS_DENOM` must fit `u64` (reservation cap in MYSO smallest units).
        assert!(
            (post_threshold as u128) * (max_individual_reservation_bps as u128)
                <= MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128,
            EInvalidFeeConfig
        );
        assert!(
            (profile_threshold as u128) * (max_individual_reservation_bps as u128)
                <= MAX_RESERVATION_THRESHOLD_BPS_PRODUCT_U128,
            EInvalidFeeConfig
        );
        
        assert!(
            max_reservers_per_pool > 0 && max_reservers_per_pool <= MAX_RESERVERS_PER_POOL_CAP,
            EInvalidFeeConfig
        );
        
        assert!(trading_creator_fee_bps <= BPS_DENOM, EInvalidFeeConfig);
        assert!(trading_platform_fee_bps <= BPS_DENOM, EInvalidFeeConfig);
        assert!(trading_treasury_fee_bps <= BPS_DENOM, EInvalidFeeConfig);
        assert!(reservation_creator_fee_bps <= BPS_DENOM, EInvalidFeeConfig);
        assert!(reservation_platform_fee_bps <= BPS_DENOM, EInvalidFeeConfig);
        assert!(reservation_treasury_fee_bps <= BPS_DENOM, EInvalidFeeConfig);

        // Non-platform split must partition the platform-fee bucket exactly (sum == BPS_DENOM).
        assert!(non_platform_platform_to_creator_bps <= BPS_DENOM, EInvalidFeeConfig);
        assert!(non_platform_platform_to_treasury_bps <= BPS_DENOM, EInvalidFeeConfig);
        assert!(
            non_platform_platform_to_creator_bps + non_platform_platform_to_treasury_bps == BPS_DENOM,
            EInvalidFeeConfig,
        );
        
        // Update trading fee config
        config.trading_creator_fee_bps = trading_creator_fee_bps;
        config.trading_platform_fee_bps = trading_platform_fee_bps;
        config.trading_treasury_fee_bps = trading_treasury_fee_bps;
        
        // Update reservation fee config
        config.reservation_creator_fee_bps = reservation_creator_fee_bps;
        config.reservation_platform_fee_bps = reservation_platform_fee_bps;
        config.reservation_treasury_fee_bps = reservation_treasury_fee_bps;
        
        // Update curve parameters
        config.base_price = base_price;
        config.quadratic_coefficient = quadratic_coefficient;
        
        // Update max hold percentage
        config.max_hold_percent_bps = max_hold_percent_bps;
        
        // Update reservation thresholds
        config.post_threshold = post_threshold;
        config.profile_threshold = profile_threshold;
        config.max_individual_reservation_bps = max_individual_reservation_bps;
        config.max_reservers_per_pool = max_reservers_per_pool;
        config.non_platform_platform_to_creator_bps = non_platform_platform_to_creator_bps;
        config.non_platform_platform_to_treasury_bps = non_platform_platform_to_treasury_bps;
        
        // Calculate totals for event emission
        let total_fee_bps = calculate_total_fee_bps(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        
        // Emit config updated event
        event::emit(ConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
            total_fee_bps,
            trading_creator_fee_bps,
            trading_platform_fee_bps,
            trading_treasury_fee_bps,
            reservation_total_fee_bps,
            reservation_creator_fee_bps,
            reservation_platform_fee_bps,
            reservation_treasury_fee_bps,
            base_price,
            quadratic_coefficient,
            max_hold_percent_bps,
            post_threshold,
            profile_threshold,
            max_individual_reservation_bps,
            max_reservers_per_pool,
            non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps,
            trading_enabled: config.trading_enabled,
        });
    }

    /// Emergency kill switch function - only callable by admin
    /// This function can immediately enable or halt all trading on the platform
    public entry fun toggle_emergency_kill_switch(
        _admin_cap: &SocialProofTokensAdminCap,
        config: &mut SocialProofTokensConfig,
        enable_trading: bool,
        reason: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Update the trading enabled status
        config.trading_enabled = enable_trading;
        
        // Emit event for audit trail
        event::emit(EmergencyKillSwitchEvent {
            admin: tx_context::sender(ctx),
            trading_enabled: enable_trading,
            timestamp: clock::timestamp_ms(clock),
            reason: string::utf8(reason),
        });
    }

    /// Check if trading is currently enabled
    public fun is_trading_enabled(config: &SocialProofTokensConfig): bool {
        config.trading_enabled
    }

    /// Calculate total trading fee from component fees
    public(package) fun calculate_total_fee_bps(config: &SocialProofTokensConfig): u64 {
        config.trading_creator_fee_bps + config.trading_platform_fee_bps + config.trading_treasury_fee_bps
    }

    /// Calculate total reservation fee from component fees
    public(package) fun calculate_reservation_total_fee_bps(config: &SocialProofTokensConfig): u64 {
        config.reservation_creator_fee_bps + config.reservation_platform_fee_bps + config.reservation_treasury_fee_bps
    }

    /// Validate trading fee config before use
    public(package) fun validate_trading_fees(config: &SocialProofTokensConfig) {
        let total_fee_bps = calculate_total_fee_bps(config);
        assert!(total_fee_bps > 0 && total_fee_bps <= BPS_DENOM, EInvalidFeeConfig);
    }

    /// Validate reservation fee config before use
    public(package) fun validate_reservation_fees(config: &SocialProofTokensConfig) {
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        assert!(reservation_total_fee_bps > 0 && reservation_total_fee_bps <= BPS_DENOM, EInvalidFeeConfig);
    }

    /// Calculate fee amount with overflow protection
    /// amount * fee_bps can overflow before division, so check first
    public(package) fun calculate_fee_amount_safe(amount: u64, fee_bps: u64): u64 {
        // Overflow protection: amount * fee_bps can overflow before division
        if (amount == 0 || fee_bps == 0) {
            return 0
        };
        assert!(amount <= MAX_U64 / fee_bps, EOverflow);
        (amount * fee_bps) / BPS_DENOM
    }

    /// Calculate component fee from total fee amount
    /// component_fee = (fee_amount * component_bps) / total_bps
    /// With overflow protection
    public(package) fun calculate_component_fee_safe(fee_amount: u64, component_bps: u64, total_bps: u64): u64 {
        if (fee_amount == 0 || component_bps == 0 || total_bps == 0) {
            return 0
        };
        // Overflow protection: fee_amount * component_bps can overflow
        assert!(fee_amount <= MAX_U64 / component_bps, EOverflow);
        (fee_amount * component_bps) / total_bps
    }

    // === Reservation Functions ===

    /// Reserve MYSO tokens towards a post to support social proof token creation
    /// Anyone can call this function - the post owner is stored in the reservation pool
    /// Reserve MYSO tokens towards a post to support social proof token creation
    /// Non-platform version: platform fees go to ecosystem treasury
    #[allow(lint(self_transfer))]
    public fun reserve_towards_post(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        min_vault_deposit_amount: u64,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        payment: Coin<MYSO>,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        // Prevent reservations after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        let reserver = tx_context::sender(ctx);
        // Get post ID and owner from reservation pool
        let post_id = reservation_pool_object.info.associated_id;
        let post_owner = reservation_pool_object.info.owner;
        let now = clock::timestamp_ms(clock);
        
        // Verify reservation pool is for a post
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        
        // Verify post matches reservation pool
        assert!(post::get_id_address(post) == post_id, EInvalidID);
        
        // Ensure reserver has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Calculate fees upfront based on desired reservation amount
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        
        // Determine if fees should be on top or deducted from amount
        let fees_on_top = coin::value(&payment) >= amount + fee_amount;
        let net_amount = if (fees_on_top) {
            // User has enough: reserve full amount, fees on top
            amount
        } else {
            // User doesn't have enough for fees on top: deduct fees from amount
            assert!(coin::value(&payment) >= amount, EInsufficientFunds);
            amount - fee_amount
        };
        
        // Calculate and distribute fees (non-platform version)
        // Fee distribution calculates fees from 'amount' and deducts from payment
        // When fees_on_top: payment has amount+fees, after distribution: remaining = amount (correct!)
        // When fees deducted: payment has amount, after distribution: remaining = amount - fees (correct!)
        let (mut remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = distribute_reservation_fees_with_post(
            config,
            min_vault_deposit_amount,
            reservation_pool_object,
            post,
            beneficiary_vault,
            amount,
            payment,
            treasury,
            clock,
            ctx
        );
        
        // Check individual reservation limit (based on net amount)
        let max_individual_reservation = (config.post_threshold * config.max_individual_reservation_bps) / BPS_DENOM;
        let current_reservation = if (table::contains(&reservation_pool_object.reservations, reserver)) {
            *table::borrow(&reservation_pool_object.reservations, reserver)
        } else {
            0
        };
        assert!(current_reservation + net_amount <= max_individual_reservation, EExceededMaxHold);
        
        // Extract net reservation payment
        let reservation_payment = coin::split(&mut remaining_payment, net_amount, ctx);
        balance::join(&mut reservation_pool_object.myso_balance, coin::into_balance(reservation_payment));
        
        // Update reserver's balance in the pool (store net amount)
        if (table::contains(&reservation_pool_object.reservations, reserver)) {
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            assert!(*reservation_balance <= MAX_U64 - net_amount, EOverflow);
            *reservation_balance = *reservation_balance + net_amount;
        } else {
            // DoS protection: limit number of unique reservers per pool
            let current_reservers_count = vector::length(&reservation_pool_object.reservers);
            assert!(current_reservers_count < config.max_reservers_per_pool, ETooManyReservers);
            
            table::add(&mut reservation_pool_object.reservations, reserver, net_amount);
            // Add to reservers list for tracking
            vector::push_back(&mut reservation_pool_object.reservers, reserver);
        };

        // Update total reserved (with net amount)

        assert!(reservation_pool_object.info.total_reserved <= MAX_U64 - net_amount, EOverflow);
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;

        // Update registry
        if (table::contains(&registry.reservation_pools, post_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, post_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        } else {
            // Create registry entry if it doesn't exist
            let reservation_pool = ReservationPool {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.post_threshold,
                created_at: now,
            };
            table::add(&mut registry.reservation_pools, post_id, reservation_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = reservation_pool_object.info.total_reserved >= config.post_threshold;
        let was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) >= config.post_threshold;
        
        // Emit threshold met event if this reservation pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.post_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&remaining_payment) > 0) {
            transfer::public_transfer(remaining_payment, reserver);
        } else {
            coin::destroy_zero(remaining_payment);
        };
        
        // Emit reservation created event
        // amount field represents the actual reserved amount (net_amount)
        event::emit(ReservationCreatedEvent {
            associated_id: post_id,
            token_type: TOKEN_TYPE_POST,
            reserver,
            amount: net_amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            threshold_met,
            reserved_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Reserve MYSO tokens towards a post to support social proof token creation
    /// Platform version: platform fees go to platform treasury, includes platform validation
    #[allow(lint(self_transfer))]
    public fun reserve_towards_post_with_platform(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        min_vault_deposit_amount: u64,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        platform: &mut social_contracts::platform::Platform,
        block_list_registry: &BlockListRegistry,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        payment: Coin<MYSO>,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        // Prevent reservations after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        let reserver = tx_context::sender(ctx);
        // Get post ID and owner from reservation pool
        let post_id = reservation_pool_object.info.associated_id;
        let post_owner = reservation_pool_object.info.owner;
        let now = clock::timestamp_ms(clock);
        
        // Verify reservation pool is for a post
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        
        // Verify post matches reservation pool
        assert!(post::get_id_address(post) == post_id, EInvalidID);
        
        // Ensure reserver has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, reserver), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, reserver), EUserBlockedByPlatform);
        
        // Calculate fees upfront based on desired reservation amount
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        
        // Determine if fees should be on top or deducted from amount
        let fees_on_top = coin::value(&payment) >= amount + fee_amount;
        let net_amount = if (fees_on_top) {
            // User has enough: reserve full amount, fees on top
            amount
        } else {
            // User doesn't have enough for fees on top: deduct fees from amount
            assert!(coin::value(&payment) >= amount, EInsufficientFunds);
            amount - fee_amount
        };
        
        // Calculate and distribute fees (platform version)
        // Fee distribution calculates fees from 'amount' and deducts from payment
        // When fees_on_top: payment has amount+fees, after distribution: remaining = amount (correct!)
        // When fees deducted: payment has amount, after distribution: remaining = amount - fees (correct!)
        let (mut remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = distribute_reservation_fees_with_post_and_platform(
            config,
            min_vault_deposit_amount,
            reservation_pool_object,
            post,
            beneficiary_vault,
            amount,
            payment,
            treasury,
            platform,
            clock,
            ctx
        );
        
        // Check individual reservation limit (based on net amount)
        let max_individual_reservation = (config.post_threshold * config.max_individual_reservation_bps) / BPS_DENOM;
        let current_reservation = if (table::contains(&reservation_pool_object.reservations, reserver)) {
            *table::borrow(&reservation_pool_object.reservations, reserver)
        } else {
            0
        };
        assert!(current_reservation + net_amount <= max_individual_reservation, EExceededMaxHold);
        
        // Extract net reservation payment
        let reservation_payment = coin::split(&mut remaining_payment, net_amount, ctx);
        balance::join(&mut reservation_pool_object.myso_balance, coin::into_balance(reservation_payment));
        
        // Update reserver's balance in the pool (store net amount)
        if (table::contains(&reservation_pool_object.reservations, reserver)) {
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            assert!(*reservation_balance <= MAX_U64 - net_amount, EOverflow);
            *reservation_balance = *reservation_balance + net_amount;
        } else {
            // DoS protection: limit number of unique reservers per pool
            let current_reservers_count = vector::length(&reservation_pool_object.reservers);
            assert!(current_reservers_count < config.max_reservers_per_pool, ETooManyReservers);
            
            table::add(&mut reservation_pool_object.reservations, reserver, net_amount);
            // Add to reservers list for tracking
            vector::push_back(&mut reservation_pool_object.reservers, reserver);
        };

        // Update total reserved (with net amount)
        assert!(reservation_pool_object.info.total_reserved <= MAX_U64 - net_amount, EOverflow);
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;

        // Update registry
        if (table::contains(&registry.reservation_pools, post_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, post_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        } else {
            // Create registry entry if it doesn't exist
            let reservation_pool = ReservationPool {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.post_threshold,
                created_at: now,
            };
            table::add(&mut registry.reservation_pools, post_id, reservation_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = reservation_pool_object.info.total_reserved >= config.post_threshold;
        let was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) >= config.post_threshold;
        
        // Emit threshold met event if this reservation pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.post_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&remaining_payment) > 0) {
            transfer::public_transfer(remaining_payment, reserver);
        } else {
            coin::destroy_zero(remaining_payment);
        };
        
        // Emit reservation created event
        // amount field represents the actual reserved amount (net_amount)
        event::emit(ReservationCreatedEvent {
            associated_id: post_id,
            token_type: TOKEN_TYPE_POST,
            reserver,
            amount: net_amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            threshold_met,
            reserved_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Reserve MYSO tokens towards a profile to support social proof token creation
    /// Non-platform version: platform fees go to ecosystem treasury
    /// Anyone can call this function - the profile owner is stored in the reservation pool
    #[allow(lint(self_transfer))]
    public fun reserve_towards_profile(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        payment: Coin<MYSO>,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        // Prevent reservations after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        let reserver = tx_context::sender(ctx);
        // Get profile ID and owner from reservation pool
        let profile_id = reservation_pool_object.info.associated_id;
        let profile_owner = reservation_pool_object.info.owner;
        let now = clock::timestamp_ms(clock);
        
        // Verify reservation pool is for a profile
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_PROFILE, EInvalidTokenType);
        
        // Ensure reserver has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Calculate fees upfront based on desired reservation amount
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        
        // Determine if fees should be on top or deducted from amount
        let fees_on_top = coin::value(&payment) >= amount + fee_amount;
        let net_amount = if (fees_on_top) {
            // User has enough: reserve full amount, fees on top
            amount
        } else {
            // User doesn't have enough for fees on top: deduct fees from amount
            assert!(coin::value(&payment) >= amount, EInsufficientFunds);
            amount - fee_amount
        };
        
        // Calculate and distribute fees (non-platform version, no PoC for profiles)
        // Fee distribution calculates fees from 'amount' and deducts from payment
        // When fees_on_top: payment has amount+fees, after distribution: remaining = amount (correct!)
        // When fees deducted: payment has amount, after distribution: remaining = amount - fees (correct!)
        let (mut remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = distribute_reservation_fees_no_poc(
            config,
            reservation_pool_object,
            amount,
            payment,
            treasury,
            ctx
        );
        
        // Check individual reservation limit (based on net amount)
        let max_individual_reservation = (config.profile_threshold * config.max_individual_reservation_bps) / BPS_DENOM;
        let current_reservation = if (table::contains(&reservation_pool_object.reservations, reserver)) {
            *table::borrow(&reservation_pool_object.reservations, reserver)
        } else {
            0
        };
        assert!(current_reservation + net_amount <= max_individual_reservation, EExceededMaxHold);
        
        // Extract net reservation payment
        let reservation_payment = coin::split(&mut remaining_payment, net_amount, ctx);
        balance::join(&mut reservation_pool_object.myso_balance, coin::into_balance(reservation_payment));

        // Update reserver's balance in the pool (store net amount)
        if (table::contains(&reservation_pool_object.reservations, reserver)) {
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            assert!(*reservation_balance <= MAX_U64 - net_amount, EOverflow);
            *reservation_balance = *reservation_balance + net_amount;
        } else {
            // DoS protection: limit number of unique reservers per pool
            let current_reservers_count = vector::length(&reservation_pool_object.reservers);
            assert!(current_reservers_count < config.max_reservers_per_pool, ETooManyReservers);
            
            table::add(&mut reservation_pool_object.reservations, reserver, net_amount);
            // Add to reservers list for tracking
            vector::push_back(&mut reservation_pool_object.reservers, reserver);
        };

        // Update total reserved (with net amount)

        assert!(reservation_pool_object.info.total_reserved <= MAX_U64 - net_amount, EOverflow);
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;

        // Update registry
        if (table::contains(&registry.reservation_pools, profile_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, profile_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        } else {
            // Create registry entry if it doesn't exist
            let reservation_pool = ReservationPool {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.profile_threshold,
                created_at: now,
            };
            table::add(&mut registry.reservation_pools, profile_id, reservation_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = reservation_pool_object.info.total_reserved >= config.profile_threshold;
        let was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) >= config.profile_threshold;
        
        // Emit threshold met event if this reservation pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.profile_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&remaining_payment) > 0) {
            transfer::public_transfer(remaining_payment, reserver);
        } else {
            coin::destroy_zero(remaining_payment);
        };
        
        // Emit reservation created event
        // amount field represents the actual reserved amount (net_amount)
        event::emit(ReservationCreatedEvent {
            associated_id: profile_id,
            token_type: TOKEN_TYPE_PROFILE,
            reserver,
            amount: net_amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            threshold_met,
            reserved_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Reserve MYSO tokens towards a profile to support social proof token creation
    /// Platform version: platform fees go to platform treasury, includes platform validation
    /// Anyone can call this function - the profile owner is stored in the reservation pool
    #[allow(lint(self_transfer))]
    public fun reserve_towards_profile_with_platform(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        platform: &mut social_contracts::platform::Platform,
        block_list_registry: &BlockListRegistry,
        payment: Coin<MYSO>,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        // Prevent reservations after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        let reserver = tx_context::sender(ctx);
        // Get profile ID and owner from reservation pool
        let profile_id = reservation_pool_object.info.associated_id;
        let profile_owner = reservation_pool_object.info.owner;
        let now = clock::timestamp_ms(clock);
        
        // Verify reservation pool is for a profile
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_PROFILE, EInvalidTokenType);
        
        // Ensure reserver has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, reserver), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, reserver), EUserBlockedByPlatform);
        
        // Calculate fees upfront based on desired reservation amount
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        
        // Determine if fees should be on top or deducted from amount
        let fees_on_top = coin::value(&payment) >= amount + fee_amount;
        let net_amount = if (fees_on_top) {
            // User has enough: reserve full amount, fees on top
            amount
        } else {
            // User doesn't have enough for fees on top: deduct fees from amount
            assert!(coin::value(&payment) >= amount, EInsufficientFunds);
            amount - fee_amount
        };
        
        // Calculate and distribute fees (platform version, no PoC for profiles)
        // Fee distribution calculates fees from 'amount' and deducts from payment
        // When fees_on_top: payment has amount+fees, after distribution: remaining = amount (correct!)
        // When fees deducted: payment has amount, after distribution: remaining = amount - fees (correct!)
        let (mut remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = distribute_reservation_fees_no_poc_with_platform(
            config,
            reservation_pool_object,
            amount,
            payment,
            treasury,
            platform,
            clock,
            ctx
        );
        
        // Check individual reservation limit (based on net amount)
        let max_individual_reservation = (config.profile_threshold * config.max_individual_reservation_bps) / BPS_DENOM;
        let current_reservation = if (table::contains(&reservation_pool_object.reservations, reserver)) {
            *table::borrow(&reservation_pool_object.reservations, reserver)
        } else {
            0
        };
        assert!(current_reservation + net_amount <= max_individual_reservation, EExceededMaxHold);
        
        // Extract net reservation payment
        let reservation_payment = coin::split(&mut remaining_payment, net_amount, ctx);
        balance::join(&mut reservation_pool_object.myso_balance, coin::into_balance(reservation_payment));

        // Update reserver's balance in the pool (store net amount)
        if (table::contains(&reservation_pool_object.reservations, reserver)) {
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            assert!(*reservation_balance <= MAX_U64 - net_amount, EOverflow);
            *reservation_balance = *reservation_balance + net_amount;
        } else {
            // DoS protection: limit number of unique reservers per pool
            let current_reservers_count = vector::length(&reservation_pool_object.reservers);
            assert!(current_reservers_count < config.max_reservers_per_pool, ETooManyReservers);
            
            table::add(&mut reservation_pool_object.reservations, reserver, net_amount);
            // Add to reservers list for tracking
            vector::push_back(&mut reservation_pool_object.reservers, reserver);
        };

        // Update total reserved (with net amount)
        assert!(reservation_pool_object.info.total_reserved <= MAX_U64 - net_amount, EOverflow);
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;

        // Update registry
        if (table::contains(&registry.reservation_pools, profile_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, profile_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        } else {
            // Create registry entry if it doesn't exist
            let reservation_pool = ReservationPool {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.profile_threshold,
                created_at: now,
            };
            table::add(&mut registry.reservation_pools, profile_id, reservation_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = reservation_pool_object.info.total_reserved >= config.profile_threshold;
        let was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) >= config.profile_threshold;
        
        // Emit threshold met event if this reservation pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.profile_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&remaining_payment) > 0) {
            transfer::public_transfer(remaining_payment, reserver);
        } else {
            coin::destroy_zero(remaining_payment);
        };
        
        // Emit reservation created event
        // amount field represents the actual reserved amount (net_amount)
        event::emit(ReservationCreatedEvent {
            associated_id: profile_id,
            token_type: TOKEN_TYPE_PROFILE,
            reserver,
            amount: net_amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            threshold_met,
            reserved_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Deduct gross withdrawal from reservation ledger and registry mirror.
    fun apply_reservation_withdrawal_ledger(
        registry: &mut TokenRegistry,
        reservation_pool_object: &mut ReservationPoolObject,
        reserver: address,
        associated_id: address,
        amount: u64,
    ) {
        let current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
        assert!(current_reservation >= amount, EInsufficientLiquidity);
        if (current_reservation == amount) {
            table::remove(&mut reservation_pool_object.reservations, reserver);
            let mut i = 0;
            let len = vector::length(&reservation_pool_object.reservers);
            while (i < len) {
                if (*vector::borrow(&reservation_pool_object.reservers, i) == reserver) {
                    vector::remove(&mut reservation_pool_object.reservers, i);
                    break
                };
                i = i + 1;
            };
        } else {
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            *reservation_balance = *reservation_balance - amount;
        };
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved - amount;
        if (table::contains(&registry.reservation_pools, associated_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, associated_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        };
    }

    fun reservation_withdrawal_fee_split(
        config: &SocialProofTokensConfig,
        amount: u64
    ): (u64, u64, u64, u64, u64) {
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(
            fee_amount,
            config.reservation_creator_fee_bps,
            reservation_total_fee_bps
        );
        let platform_fee = calculate_component_fee_safe(
            fee_amount,
            config.reservation_platform_fee_bps,
            reservation_total_fee_bps
        );
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        let net_refund = amount - fee_amount;
        (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund)
    }

    /// Non-platform post withdrawal: split configured platform fee between PoC-aware creator
    /// routing and ecosystem per `config.non_platform_platform_to_*_bps` (defaults 50/50).
    fun distribute_reservation_withdraw_fees_non_platform_post(
        config: &SocialProofTokensConfig,
        min_vault_deposit_amount: u64,
        pool_owner: address,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        treasury: &EcosystemTreasury,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
        pool_balance: &mut Balance<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ): (u64, u64, u64) {
        let platform_fee_to_creator = (platform_fee * config.non_platform_platform_to_creator_bps) / BPS_DENOM;
        let platform_fee_to_treasury = platform_fee - platform_fee_to_creator;
        let creator_total = creator_fee + platform_fee_to_creator;
        let treasury_total = treasury_fee + platform_fee_to_treasury;
        if (creator_total > 0) {
            let mut creator_coin = coin::from_balance(balance::split(pool_balance, creator_total), ctx);
            distribute_reservation_creator_fee_with_owner(
                pool_owner,
                post,
                beneficiary_vault,
                creator_total,
                &mut creator_coin,
                min_vault_deposit_amount,
                clock,
                ctx
            );
            coin::destroy_zero(creator_coin);
        };
        if (treasury_total > 0) {
            let treasury_coin = coin::from_balance(balance::split(pool_balance, treasury_total), ctx);
            transfer::public_transfer(treasury_coin, profile::get_treasury_address(treasury));
        };
        (creator_total, 0, treasury_total)
    }

    /// Non-platform profile withdrawal: same 50/50 platform-fee convention as
    /// `distribute_reservation_fees_no_poc`.
    fun distribute_reservation_withdraw_fees_non_platform_profile(
        config: &SocialProofTokensConfig,
        pool_owner: address,
        treasury: &EcosystemTreasury,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
        pool_balance: &mut Balance<MYSO>,
        ctx: &mut TxContext
    ): (u64, u64, u64) {
        let platform_fee_to_creator = (platform_fee * config.non_platform_platform_to_creator_bps) / BPS_DENOM;
        let platform_fee_to_treasury = platform_fee - platform_fee_to_creator;
        let creator_total = creator_fee + platform_fee_to_creator;
        let treasury_total = treasury_fee + platform_fee_to_treasury;
        if (creator_total > 0) {
            let mut creator_coin = coin::from_balance(balance::split(pool_balance, creator_total), ctx);
            distribute_reservation_creator_fee_no_poc_with_owner(
                pool_owner,
                creator_total,
                &mut creator_coin,
                ctx
            );
            coin::destroy_zero(creator_coin);
        };
        if (treasury_total > 0) {
            let treasury_coin = coin::from_balance(balance::split(pool_balance, treasury_total), ctx);
            transfer::public_transfer(treasury_coin, profile::get_treasury_address(treasury));
        };
        (creator_total, 0, treasury_total)
    }

    fun distribute_reservation_withdraw_fees_platform_post(
        pool_owner: address,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        treasury: &EcosystemTreasury,
        platform: &mut social_contracts::platform::Platform,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
        pool_balance: &mut Balance<MYSO>,
        min_vault_deposit_amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        if (creator_fee > 0) {
            let mut creator_coin = coin::from_balance(balance::split(pool_balance, creator_fee), ctx);
            distribute_reservation_creator_fee_with_owner(
                pool_owner,
                post,
                beneficiary_vault,
                creator_fee,
                &mut creator_coin,
                min_vault_deposit_amount,
                clock,
                ctx
            );
            coin::destroy_zero(creator_coin);
        };
        if (platform_fee > 0) {
            let mut platform_fee_coin = coin::from_balance(balance::split(pool_balance, platform_fee), ctx);
            social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_fee_coin);
        };
        if (treasury_fee > 0) {
            let treasury_coin = coin::from_balance(balance::split(pool_balance, treasury_fee), ctx);
            transfer::public_transfer(treasury_coin, profile::get_treasury_address(treasury));
        };
    }

    fun distribute_reservation_withdraw_fees_platform_profile(
        pool_owner: address,
        treasury: &EcosystemTreasury,
        platform: &mut social_contracts::platform::Platform,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
        pool_balance: &mut Balance<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        if (creator_fee > 0) {
            let mut creator_coin = coin::from_balance(balance::split(pool_balance, creator_fee), ctx);
            distribute_reservation_creator_fee_no_poc_with_owner(
                pool_owner,
                creator_fee,
                &mut creator_coin,
                ctx
            );
            coin::destroy_zero(creator_coin);
        };
        if (platform_fee > 0) {
            let mut platform_fee_coin = coin::from_balance(balance::split(pool_balance, platform_fee), ctx);
            social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_fee_coin);
        };
        if (treasury_fee > 0) {
            let treasury_coin = coin::from_balance(balance::split(pool_balance, treasury_fee), ctx);
            transfer::public_transfer(treasury_coin, profile::get_treasury_address(treasury));
        };
    }

    /// Withdraw MYSO reservation for a **post** pool (non-platform).
    /// `amount` is gross ledger reduction; caller receives `amount - reservation_fees`.
    /// Matches non-platform reserve fee routing (50/50 platform share; PoC on creator path).
    #[allow(lint(self_transfer))]
    public fun withdraw_reservation_for_post(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        min_vault_deposit_amount: u64,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let reserver = tx_context::sender(ctx);
        let associated_id = reservation_pool_object.info.associated_id;
        let now = clock::timestamp_ms(clock);

        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        assert!(amount > 0, EInsufficientFunds);
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        assert!(post::get_id_address(post) == associated_id, EInvalidID);

        assert!(table::contains(&reservation_pool_object.reservations, reserver), ENoTokensOwned);
        let current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
        let pool_owner = reservation_pool_object.info.owner;

        let (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund) =
            reservation_withdrawal_fee_split(config, amount);

        assert!(current_reservation >= amount, EInsufficientLiquidity);
        assert!(balance::value(&reservation_pool_object.myso_balance) >= amount, EInsufficientLiquidity);

        apply_reservation_withdrawal_ledger(registry, reservation_pool_object, reserver, associated_id, amount);

        let (ev_creator, ev_platform, ev_treasury) = if (fee_amount > 0) {
            distribute_reservation_withdraw_fees_non_platform_post(
                config,
                min_vault_deposit_amount,
                pool_owner,
                post,
                beneficiary_vault,
                treasury,
                creator_fee,
                platform_fee,
                treasury_fee,
                &mut reservation_pool_object.myso_balance,
                clock,
                ctx
            )
        } else {
            (0, 0, 0)
        };

        let refund_balance = balance::split(&mut reservation_pool_object.myso_balance, net_refund);
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, reserver);

        event::emit(ReservationWithdrawnEvent {
            associated_id,
            token_type: reservation_pool_object.info.token_type,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            withdrawn_at: now,
            fee_amount,
            creator_fee: ev_creator,
            platform_fee: ev_platform,
            treasury_fee: ev_treasury,
        });
    }

    /// Withdraw MYSO reservation for a **profile** pool (non-platform).
    #[allow(lint(self_transfer))]
    public fun withdraw_reservation_for_profile(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let reserver = tx_context::sender(ctx);
        let associated_id = reservation_pool_object.info.associated_id;
        let now = clock::timestamp_ms(clock);

        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        assert!(amount > 0, EInsufficientFunds);
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_PROFILE, EInvalidTokenType);

        assert!(table::contains(&reservation_pool_object.reservations, reserver), ENoTokensOwned);
        let current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
        let pool_owner = reservation_pool_object.info.owner;

        let (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund) =
            reservation_withdrawal_fee_split(config, amount);

        assert!(current_reservation >= amount, EInsufficientLiquidity);
        assert!(balance::value(&reservation_pool_object.myso_balance) >= amount, EInsufficientLiquidity);

        apply_reservation_withdrawal_ledger(registry, reservation_pool_object, reserver, associated_id, amount);

        let (ev_creator, ev_platform, ev_treasury) = if (fee_amount > 0) {
            distribute_reservation_withdraw_fees_non_platform_profile(
                config,
                pool_owner,
                treasury,
                creator_fee,
                platform_fee,
                treasury_fee,
                &mut reservation_pool_object.myso_balance,
                ctx
            )
        } else {
            (0, 0, 0)
        };

        let refund_balance = balance::split(&mut reservation_pool_object.myso_balance, net_refund);
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, reserver);

        event::emit(ReservationWithdrawnEvent {
            associated_id,
            token_type: reservation_pool_object.info.token_type,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            withdrawn_at: now,
            fee_amount,
            creator_fee: ev_creator,
            platform_fee: ev_platform,
            treasury_fee: ev_treasury,
        });
    }

    /// Withdraw from a **post** reservation pool via an approved platform (PoC-aware creator fees).
    #[allow(lint(self_transfer))]
    public fun withdraw_reservation_with_platform_for_post(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        min_vault_deposit_amount: u64,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        platform: &mut social_contracts::platform::Platform,
        block_list_registry: &BlockListRegistry,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let reserver = tx_context::sender(ctx);
        let associated_id = reservation_pool_object.info.associated_id;
        let now = clock::timestamp_ms(clock);

        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        assert!(amount > 0, EInsufficientFunds);
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        assert!(post::get_id_address(post) == associated_id, EInvalidID);

        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, reserver), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, reserver), EUserBlockedByPlatform);

        assert!(table::contains(&reservation_pool_object.reservations, reserver), ENoTokensOwned);
        let current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
        let pool_owner = reservation_pool_object.info.owner;

        let (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund) =
            reservation_withdrawal_fee_split(config, amount);

        assert!(current_reservation >= amount, EInsufficientLiquidity);
        assert!(balance::value(&reservation_pool_object.myso_balance) >= amount, EInsufficientLiquidity);

        apply_reservation_withdrawal_ledger(registry, reservation_pool_object, reserver, associated_id, amount);

        if (fee_amount > 0) {
            distribute_reservation_withdraw_fees_platform_post(
                pool_owner,
                post,
                beneficiary_vault,
                treasury,
                platform,
                creator_fee,
                platform_fee,
                treasury_fee,
                &mut reservation_pool_object.myso_balance,
                min_vault_deposit_amount,
                clock,
                ctx
            );
        };

        let refund_balance = balance::split(&mut reservation_pool_object.myso_balance, net_refund);
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, reserver);

        event::emit(ReservationWithdrawnEvent {
            associated_id,
            token_type: reservation_pool_object.info.token_type,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            withdrawn_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Withdraw from a **profile** reservation pool via an approved platform.
    #[allow(lint(self_transfer))]
    public fun withdraw_reservation_with_platform_for_profile(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        platform: &mut social_contracts::platform::Platform,
        block_list_registry: &BlockListRegistry,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let reserver = tx_context::sender(ctx);
        let associated_id = reservation_pool_object.info.associated_id;
        let now = clock::timestamp_ms(clock);

        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        assert!(amount > 0, EInsufficientFunds);
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_PROFILE, EInvalidTokenType);

        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, reserver), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, reserver), EUserBlockedByPlatform);

        assert!(table::contains(&reservation_pool_object.reservations, reserver), ENoTokensOwned);
        let current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
        let pool_owner = reservation_pool_object.info.owner;

        let (fee_amount, creator_fee, platform_fee, treasury_fee, net_refund) =
            reservation_withdrawal_fee_split(config, amount);

        assert!(current_reservation >= amount, EInsufficientLiquidity);
        assert!(balance::value(&reservation_pool_object.myso_balance) >= amount, EInsufficientLiquidity);

        apply_reservation_withdrawal_ledger(registry, reservation_pool_object, reserver, associated_id, amount);

        if (fee_amount > 0) {
            distribute_reservation_withdraw_fees_platform_profile(
                pool_owner,
                treasury,
                platform,
                creator_fee,
                platform_fee,
                treasury_fee,
                &mut reservation_pool_object.myso_balance,
                clock,
                ctx
            );
        };

        let refund_balance = balance::split(&mut reservation_pool_object.myso_balance, net_refund);
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, reserver);

        event::emit(ReservationWithdrawnEvent {
            associated_id,
            token_type: reservation_pool_object.info.token_type,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            withdrawn_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Create a new reservation pool for a post
    public entry fun create_reservation_pool_for_post(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        post: &Post,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let caller = tx_context::sender(ctx);
        let associated_id = post::get_id_address(post);
        let owner = post::get_post_owner(post);
        
        // Verify caller is the actual post owner
        assert!(caller == owner, ENotAuthorized);
        
        // Verify post ID matches
        assert!(associated_id == post::get_id_address(post), EInvalidID);
        
        // Check if reservation pool already exists
        assert!(!table::contains(&registry.reservation_pools, associated_id), ETokenAlreadyExists);
        
        let now = clock::timestamp_ms(clock);
        let required_threshold = config.post_threshold;
        
        // Create reservation pool info (without reservers vector - only in ReservationPoolObject)
        let reservation_pool = ReservationPool {
            associated_id,
            token_type: TOKEN_TYPE_POST,
            owner,
            total_reserved: 0,
            required_threshold,
            created_at: now,
        };
        
        // Create reservation pool object first (before moving reservation_pool)
        let reservation_pool_object = ReservationPoolObject {
            id: object::new(ctx),
            info: reservation_pool,
            myso_balance: balance::zero(),
            reservations: table::new(ctx),
            reservers: vector::empty(),
            converted: false,
            version: upgrade::current_version(),
        };
        
        // Add to registry (reconstruct ReservationPool from object's info since original was moved)
        let pool_info = ReservationPool {
            associated_id: reservation_pool_object.info.associated_id,
            token_type: reservation_pool_object.info.token_type,
            owner: reservation_pool_object.info.owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: reservation_pool_object.info.required_threshold,
            created_at: reservation_pool_object.info.created_at,
        };
        table::add(&mut registry.reservation_pools, associated_id, pool_info);
        
        let pool_object_id = object::uid_to_address(&reservation_pool_object.id);
        
        // Emit reservation pool created event
        event::emit(ReservationPoolCreatedEvent {
            associated_id,
            token_type: TOKEN_TYPE_POST,
            owner,
            required_threshold,
            pool_object_id,
            created_at: now,
        });
        
        transfer::share_object(reservation_pool_object);
    }

    /// Create a new reservation pool for a profile
    public entry fun create_reservation_pool_for_profile(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        profile: &Profile,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let caller = tx_context::sender(ctx);
        let associated_id = profile::get_id_address(profile);
        let owner = profile::owner(profile);
        
        // Verify caller is the actual profile owner
        assert!(caller == owner, ENotAuthorized);
        
        // Verify profile ID matches
        assert!(associated_id == profile::get_id_address(profile), EInvalidID);
        
        // Check if reservation pool already exists
        assert!(!table::contains(&registry.reservation_pools, associated_id), ETokenAlreadyExists);
        
        let now = clock::timestamp_ms(clock);
        let required_threshold = config.profile_threshold;
        
        // Create reservation pool info (without reservers vector - only in ReservationPoolObject)
        let reservation_pool = ReservationPool {
            associated_id,
            token_type: TOKEN_TYPE_PROFILE,
            owner,
            total_reserved: 0,
            required_threshold,
            created_at: now,
        };
        
        // Create reservation pool object first (before moving reservation_pool)
        let reservation_pool_object = ReservationPoolObject {
            id: object::new(ctx),
            info: reservation_pool,
            myso_balance: balance::zero(),
            reservations: table::new(ctx),
            reservers: vector::empty(),
            converted: false,
            version: upgrade::current_version(),
        };
        
        // Add to registry (reconstruct ReservationPool from object's info since original was moved)
        let pool_info = ReservationPool {
            associated_id: reservation_pool_object.info.associated_id,
            token_type: reservation_pool_object.info.token_type,
            owner: reservation_pool_object.info.owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: reservation_pool_object.info.required_threshold,
            created_at: reservation_pool_object.info.created_at,
        };
        table::add(&mut registry.reservation_pools, associated_id, pool_info);
        
        let pool_object_id = object::uid_to_address(&reservation_pool_object.id);
        
        // Emit reservation pool created event
        event::emit(ReservationPoolCreatedEvent {
            associated_id,
            token_type: TOKEN_TYPE_PROFILE,
            owner,
            required_threshold,
            pool_object_id,
            created_at: now,
        });
        
        transfer::share_object(reservation_pool_object);
    }

    /// Check if reservation threshold is met for auction creation
    public fun can_create_auction(
        registry: &TokenRegistry,
        config: &SocialProofTokensConfig,
        associated_id: address
    ): bool {
        if (!table::contains(&registry.reservation_pools, associated_id)) {
            return false
        };
        
        let reservation_pool = table::borrow(&registry.reservation_pools, associated_id);
        let required_threshold = if (reservation_pool.token_type == TOKEN_TYPE_PROFILE) {
            config.profile_threshold
        } else {
            config.post_threshold
        };
        reservation_pool.total_reserved >= required_threshold
    }
    
    /// Create a social proof token directly from a reservation pool once threshold is met
    /// This replaces the auction system - only the post/profile owner can call this
    public entry fun create_social_proof_token(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let caller = tx_context::sender(ctx);
        let associated_id = reservation_pool_object.info.associated_id;
        
        // Verify caller is the owner of the post/profile
        assert!(caller == reservation_pool_object.info.owner, ENotAuthorized);
        
        // Check if reservation threshold has been met
        assert!(can_create_auction(registry, config, associated_id), EViralThresholdNotMet);
        
        // Verify token has not already been created
        assert!(!table::contains(&registry.tokens, associated_id), ETokenAlreadyExists);
        
        // Initial nano-SPT supply: net nano-MYSO reserved × (nano-SPT per whole SPT) / `base_price`,
        // so implied cost per display SPT at the linear curve leg matches reservation (same `base_price`
        // stored on the pool). Reservers still split this supply proportionally by reservation_amount.
        let total_reserved = reservation_pool_object.info.total_reserved;
        assert!(total_reserved > 0, ENoContribution);
        let base_price = config.base_price;
        let product_u128 = (total_reserved as u128) * (SPT_SCALE as u128);
        assert!(product_u128 <= MAX_ONCHAIN_U64_U128, EOverflow);
        let initial_u128 = product_u128 / (base_price as u128);
        assert!(initial_u128 > 0 && initial_u128 <= MAX_ONCHAIN_U64_U128, EInvalidCurveParams);
        let initial_token_supply = initial_u128 as u64;
        
        // Create token info
        let token_info = TokenInfo {
            id: @0x0, // Temporary, will be updated
            token_type: reservation_pool_object.info.token_type,
            owner: reservation_pool_object.info.owner,
            associated_id,
            circulating_supply: initial_token_supply,
            base_price: config.base_price,
            quadratic_coefficient: config.quadratic_coefficient,
            created_at: clock::timestamp_ms(clock),
        };
        
        // Create token pool
        let pool_id = object::new(ctx);
        let pool_address = object::uid_to_address(&pool_id);
        
        // Update token info with actual pool address
        let mut updated_token_info = token_info;
        updated_token_info.id = pool_address;
        
        // Create a copy of token info for the pool (since TokenInfo doesn't have copy, we'll reconstruct)
        let pool_token_info = TokenInfo {
            id: updated_token_info.id,
            token_type: updated_token_info.token_type,
            owner: updated_token_info.owner,
            associated_id: updated_token_info.associated_id,
            circulating_supply: updated_token_info.circulating_supply,
            base_price: updated_token_info.base_price,
            quadratic_coefficient: updated_token_info.quadratic_coefficient,
            created_at: updated_token_info.created_at,
        };
        
        let mut token_pool = TokenPool {
            id: pool_id,
            info: pool_token_info,
            myso_balance: balance::zero(),
            holders: table::new(ctx),
            poc_redirect_to: option::none(),
            poc_redirect_percentage: option::none(),
            poc_redirection_kind: 0,
            version: upgrade::current_version(),
        };
        
        // Distribute tokens to reservers proportionally
        // Limit number of reservers to prevent DoS via gas exhaustion
        let reservers = &reservation_pool_object.reservers;
        let num_reservers = vector::length(reservers);
        assert!(num_reservers <= config.max_reservers_per_pool, ETooManyReservers);
        
        let mut distributed_total = 0;
        let mut i = 0;
        while (i < num_reservers) {
            let reserver = *vector::borrow(reservers, i);
            let reservation_amount = *table::borrow(&reservation_pool_object.reservations, reserver);
            
            // Calculate token amount based on reserver's proportion of total reservation
            // Use u128 to avoid overflow when reservation_amount * initial_token_supply exceeds u64
            let token_amount = (((reservation_amount as u128) * (initial_token_supply as u128)) / (total_reserved as u128)) as u64;
            
            if (token_amount > 0) {
                // Update holder's balance in the pool
                // Handle duplicate reservers: if already exists, add to existing balance
                if (table::contains(&token_pool.holders, reserver)) {
                    let existing_balance = table::borrow_mut(&mut token_pool.holders, reserver);
                    assert!(*existing_balance <= MAX_U64 - token_amount, EOverflow);
                    *existing_balance = *existing_balance + token_amount;
                } else {
                    table::add(&mut token_pool.holders, reserver, token_amount);
                };
                
                // Track distributed tokens to ensure accurate circulating supply
                assert!(distributed_total <= MAX_U64 - token_amount, EOverflow);
                distributed_total = distributed_total + token_amount;
                
                // Create social token for the reserver
                let social_token = SocialToken {
                    id: object::new(ctx),
                    pool_id: pool_address,
                    token_type: reservation_pool_object.info.token_type,
                    amount: token_amount,
                };
                
                // Transfer social token to reserver
                transfer::transfer(social_token, reserver);
            };
            
            i = i + 1;
        };
        
        // Handle rounding remainder: allocate any undistributed tokens to the owner
        let remainder = initial_token_supply - distributed_total;
        if (remainder > 0) {
            // Allocate remainder to the owner
            if (table::contains(&token_pool.holders, reservation_pool_object.info.owner)) {
                let owner_balance = table::borrow_mut(&mut token_pool.holders, reservation_pool_object.info.owner);
                assert!(*owner_balance <= MAX_U64 - remainder, EOverflow);
                *owner_balance = *owner_balance + remainder;
            } else {
                table::add(&mut token_pool.holders, reservation_pool_object.info.owner, remainder);
            };
            distributed_total = distributed_total + remainder;
        };
        
        // Update circulating supply to match actually distributed tokens
        token_pool.info.circulating_supply = distributed_total;
        updated_token_info.circulating_supply = distributed_total;
        
        // Transfer all reserved MYSO to the token pool as initial liquidity
        balance::join(&mut token_pool.myso_balance, balance::withdraw_all(&mut reservation_pool_object.myso_balance));
        
        // Snapshot for event (denominator for indexer proportional split) before clearing on-chain state
        let total_reserved_at_launch = reservation_pool_object.info.total_reserved;

        // Mark reservation pool as converted and clear total reserved
        reservation_pool_object.converted = true;
        reservation_pool_object.info.total_reserved = 0;
        
        // Update registry reservation pool entry to reflect conversion
        if (table::contains(&registry.reservation_pools, associated_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, associated_id);
            registry_pool.total_reserved = 0;
        };
        
        // Add to registry (use updated_token_info which has the correct circulating_supply)
        table::add(&mut registry.tokens, associated_id, updated_token_info);
        
        // Emit token created event (use token_pool.info which has the final state)
        event::emit(TokenPoolCreatedEvent {
            id: pool_address,
            token_type: token_pool.info.token_type,
            owner: token_pool.info.owner,
            associated_id: token_pool.info.associated_id,
            base_price: token_pool.info.base_price,
            quadratic_coefficient: token_pool.info.quadratic_coefficient,
            circulating_supply: token_pool.info.circulating_supply,
            total_reserved_at_launch,
        });
        
        // Share the token pool
        transfer::share_object(token_pool);
    }

    // === PoC Revenue Redirection Functions ===

    /// Copy PoC redirect fields from `post` into a matching POST `TokenPool` and emit `PocRedirectionUpdatedEvent`.
    public(package) fun sync_token_pool_poc_from_post(
        registry: &TokenRegistry,
        pool: &mut TokenPool,
        post: &Post,
        updated_by: address,
        clock: &Clock,
        _ctx: &TxContext,
    ) {
        assert!(pool.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        let post_id = post::get_id_address(post);
        assert!(post_id == pool.info.associated_id, EInvalidID);
        assert!(token_exists(registry, post_id), ETokenNotFound);

        let redirect_to = if (option::is_some(post::get_revenue_redirect_to(post))) {
            option::some(*option::borrow(post::get_revenue_redirect_to(post)))
        } else {
            option::none()
        };

        let redirect_percentage = if (option::is_some(post::get_revenue_redirect_percentage(post))) {
            let percentage = *option::borrow(post::get_revenue_redirect_percentage(post));
            assert!(percentage <= 100, EInvalidFeeConfig);
            option::some(percentage)
        } else {
            option::none()
        };

        pool.poc_redirect_to = redirect_to;
        pool.poc_redirect_percentage = redirect_percentage;
        pool.poc_redirection_kind = post::poc_redirection_kind(post);

        event::emit(PocRedirectionUpdatedEvent {
            pool_id: object::uid_to_address(&pool.id),
            post_id,
            redirect_to,
            redirect_percentage,
            poc_redirection_kind: pool.poc_redirection_kind,
            updated_by,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Update PoC redirection data for a token pool (called by PoC system)
    /// This function copies PoC data from a post into the corresponding token pool
    public entry fun update_token_poc_data(
        registry: &TokenRegistry,
        pool: &mut TokenPool,
        post: &Post,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        assert!(caller == post::get_post_owner(post), ENotAuthorized);
        sync_token_pool_poc_from_post(registry, pool, post, caller, clock, ctx);
    }

    /// Calculate PoC revenue split - shared utility for consistent logic
    fun calculate_poc_split(amount: u64, redirect_percentage: u64): (u64, u64) {
        // Validate redirect percentage to prevent underflow
        assert!(redirect_percentage <= 100, EInvalidFeeConfig);
        let redirected_amount = (amount * redirect_percentage) / 100;
        let remaining_amount = amount - redirected_amount;
        (redirected_amount, remaining_amount)
    }

    /// Apply PoC redirection to creator fees with consolidated logic
    fun apply_token_poc_redirection(
        pool: &TokenPool,
        amount: u64,
        _ctx: &mut TxContext  
    ): (u64, u64) {
        if (has_poc_redirection(pool)) {
            let redirect_percentage = *option::borrow(&pool.poc_redirect_percentage);
            // Use shared utility function for consistent calculation
            calculate_poc_split(amount, redirect_percentage)
        } else {
            (0, amount)
        }
    }

    /// Distribute creator fees with automatic PoC redirection  
    fun distribute_creator_fee(
        pool: &TokenPool,
        creator_fee_amount: u64,
        creator_fee_coin: &mut Coin<MYSO>,
        ctx: &mut TxContext
    ) {
        if (creator_fee_amount == 0) {
            return
        };

        assert!(
            !(pool.info.token_type == TOKEN_TYPE_POST && pool.poc_redirection_kind == 2),
            EPostPoolEscrowTradingBlocked
        );

        let (redirected_amount, _remaining_amount) = apply_token_poc_redirection(pool, creator_fee_amount, ctx);
        let mut fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);
        
        if (redirected_amount > 0) {
            // Split the fee: redirected portion goes to original creator, remainder to post owner
            let redirected_fee = coin::split(&mut fee_coin, redirected_amount, ctx);
            let redirect_to = *option::borrow(&pool.poc_redirect_to);
            transfer::public_transfer(redirected_fee, redirect_to);
            
            // Send remainder to current post owner
            if (coin::value(&fee_coin) > 0) {
                transfer::public_transfer(fee_coin, pool.info.owner);
            } else {
                coin::destroy_zero(fee_coin);
            };
        } else {
            // No redirection - send full amount to current post owner
            transfer::public_transfer(fee_coin, pool.info.owner);
        };
    }

    /// Distribute creator fees from pool balance with PoC redirection support
    fun distribute_creator_fee_from_pool(
        pool: &mut TokenPool,
        creator_fee: u64,
        ctx: &mut TxContext
    ) {
        if (creator_fee == 0) {
            return
        };

        assert!(
            !(pool.info.token_type == TOKEN_TYPE_POST && pool.poc_redirection_kind == 2),
            EPostPoolEscrowTradingBlocked
        );

        let (redirected_amount, _remaining_amount) = apply_token_poc_redirection(pool, creator_fee, ctx);
        let mut fee_coin = coin::from_balance(balance::split(&mut pool.myso_balance, creator_fee), ctx);
        
        if (redirected_amount > 0) {
            // Split the fee: redirected portion goes to original creator, remainder to post owner
            let redirected_fee = coin::split(&mut fee_coin, redirected_amount, ctx);
            let redirect_to = *option::borrow(&pool.poc_redirect_to);
            transfer::public_transfer(redirected_fee, redirect_to);
            
            // Send remainder to current post owner
            if (coin::value(&fee_coin) > 0) {
                transfer::public_transfer(fee_coin, pool.info.owner);
            } else {
                coin::destroy_zero(fee_coin);
            };
        } else {
            // No redirection - send full amount to current post owner
            transfer::public_transfer(fee_coin, pool.info.owner);
        };
    }

    // === Reservation Fee Distribution Functions ===

    /// Apply PoC redirection from post (reuses calculate_poc_split utility)
    public(package) fun apply_post_poc_redirection(
        post: &Post,
        amount: u64
    ): (u64, u64) {
        let k = post::poc_redirection_kind(post);
        let has_split = option::is_some(post::get_revenue_redirect_percentage(post)) &&
            k != post::poc_redirection_none() &&
            (k == 2 || option::is_some(post::get_revenue_redirect_to(post)));
        if (has_split) {
            let redirect_percentage = *option::borrow(post::get_revenue_redirect_percentage(post));
            assert!(redirect_percentage <= 100, EInvalidFeeConfig);
            calculate_poc_split(amount, redirect_percentage)
        } else {
            (0, amount)
        }
    }

    /// PoC-aware post reservation creator fee using an explicit pool owner (avoids borrow conflicts
    /// when paying from `&mut reservation_pool_object.myso_balance` during withdrawal).
    fun distribute_reservation_creator_fee_with_owner(
        pool_owner: address,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        creator_fee_amount: u64,
        creator_fee_coin: &mut Coin<MYSO>,
        min_vault_deposit_amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        if (creator_fee_amount == 0) {
            return
        };

        let (redirected_amount, _remaining_amount) = apply_post_poc_redirection(post, creator_fee_amount);
        let mut fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);

        if (redirected_amount > 0) {
            let redirected_fee = coin::split(&mut fee_coin, redirected_amount, ctx);
            let k = post::poc_redirection_kind(post);
            if (k == 2) {
                post::deposit_coin_to_beneficiary_vault<MYSO>(post, beneficiary_vault, redirected_fee, min_vault_deposit_amount, clock, ctx);
            } else {
                let redirect_to = *option::borrow(post::get_revenue_redirect_to(post));
                transfer::public_transfer(redirected_fee, redirect_to);
            };

            if (coin::value(&fee_coin) > 0) {
                transfer::public_transfer(fee_coin, pool_owner);
            } else {
                coin::destroy_zero(fee_coin);
            };
        } else {
            transfer::public_transfer(fee_coin, pool_owner);
        };
    }

    /// Distribute creator fees with PoC redirection from post (reuses existing pattern)
    /// This follows the same logic as distribute_creator_fee but works with Post instead of TokenPool
    public(package) fun distribute_reservation_creator_fee(
        reservation_pool: &ReservationPoolObject,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        creator_fee_amount: u64,
        creator_fee_coin: &mut Coin<MYSO>,
        min_vault_deposit_amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        distribute_reservation_creator_fee_with_owner(
            reservation_pool.info.owner,
            post,
            beneficiary_vault,
            creator_fee_amount,
            creator_fee_coin,
            min_vault_deposit_amount,
            clock,
            ctx
        );
    }

    fun distribute_reservation_creator_fee_no_poc_with_owner(
        pool_owner: address,
        creator_fee_amount: u64,
        creator_fee_coin: &mut Coin<MYSO>,
        ctx: &mut TxContext
    ) {
        if (creator_fee_amount == 0) {
            return
        };

        let fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);
        transfer::public_transfer(fee_coin, pool_owner);
    }

    /// Distribute creator fees without PoC (for profile reservations)
    public(package) fun distribute_reservation_creator_fee_no_poc(
        reservation_pool: &ReservationPoolObject,
        creator_fee_amount: u64,
        creator_fee_coin: &mut Coin<MYSO>,
        ctx: &mut TxContext
    ) {
        distribute_reservation_creator_fee_no_poc_with_owner(
            reservation_pool.info.owner,
            creator_fee_amount,
            creator_fee_coin,
            ctx
        );
    }

    /// Calculate and distribute all reservation fees (for post reservations with PoC)
    /// Non-platform version: split platform fee 50/50 between creator and treasury; emit platform_fee 0
    public(package) fun distribute_reservation_fees_with_post(
        config: &SocialProofTokensConfig,
        min_vault_deposit_amount: u64,
        reservation_pool: &ReservationPoolObject,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        amount: u64,
        mut payment: Coin<MYSO>,
        treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext
    ): (Coin<MYSO>, u64, u64, u64, u64) {
        // Validate fees and calculate with overflow protection
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;

        let platform_fee_to_creator = (platform_fee * config.non_platform_platform_to_creator_bps) / BPS_DENOM;
        let platform_fee_to_treasury = platform_fee - platform_fee_to_creator;

        if (fee_amount > 0) {
            let creator_total = creator_fee + platform_fee_to_creator;
            if (creator_total > 0) {
                distribute_reservation_creator_fee(
                    reservation_pool,
                    post,
                    beneficiary_vault,
                    creator_total,
                    &mut payment,
                    min_vault_deposit_amount,
                    clock,
                    ctx
                );
            };
            let treasury_total = treasury_fee + platform_fee_to_treasury;
            if (treasury_total > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_total, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };

        (payment, fee_amount, creator_fee + platform_fee_to_creator, 0, treasury_fee + platform_fee_to_treasury)
    }

    /// Calculate and distribute all reservation fees (for post reservations with PoC)
    /// Platform version: routes platform fees to platform treasury
    public(package) fun distribute_reservation_fees_with_post_and_platform(
        config: &SocialProofTokensConfig,
        min_vault_deposit_amount: u64,
        reservation_pool: &ReservationPoolObject,
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        amount: u64,
        mut payment: Coin<MYSO>,
        treasury: &EcosystemTreasury,
        platform: &mut social_contracts::platform::Platform,
        clock: &Clock,
        ctx: &mut TxContext
    ): (Coin<MYSO>, u64, u64, u64, u64) {
        // Validate fees and calculate with overflow protection
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Distribute fees (same pattern as trading fees)
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_reservation_creator_fee(
                    reservation_pool,
                    post,
                    beneficiary_vault,
                    creator_fee,
                    &mut payment,
                    min_vault_deposit_amount,
                    clock,
                    ctx
                );
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, clock, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Return remaining payment and fee amounts
        (payment, fee_amount, creator_fee, platform_fee, treasury_fee)
    }

    /// Calculate and distribute all reservation fees (for profile reservations without PoC)
    /// Non-platform version: split platform fee between creator and treasury per config; emit platform_fee 0
    public(package) fun distribute_reservation_fees_no_poc(
        config: &SocialProofTokensConfig,
        reservation_pool: &ReservationPoolObject,
        amount: u64,
        mut payment: Coin<MYSO>,
        treasury: &EcosystemTreasury,
        ctx: &mut TxContext
    ): (Coin<MYSO>, u64, u64, u64, u64) {
        // Validate fees and calculate with overflow protection
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;

        let platform_fee_to_creator = (platform_fee * config.non_platform_platform_to_creator_bps) / BPS_DENOM;
        let platform_fee_to_treasury = platform_fee - platform_fee_to_creator;

        if (fee_amount > 0) {
            let creator_total = creator_fee + platform_fee_to_creator;
            if (creator_total > 0) {
                distribute_reservation_creator_fee_no_poc(reservation_pool, creator_total, &mut payment, ctx);
            };
            let treasury_total = treasury_fee + platform_fee_to_treasury;
            if (treasury_total > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_total, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };

        (payment, fee_amount, creator_fee + platform_fee_to_creator, 0, treasury_fee + platform_fee_to_treasury)
    }

    /// Calculate and distribute all reservation fees (for profile reservations without PoC)
    /// Platform version: routes platform fees to platform treasury
    public(package) fun distribute_reservation_fees_no_poc_with_platform(
        config: &SocialProofTokensConfig,
        reservation_pool: &ReservationPoolObject,
        amount: u64,
        mut payment: Coin<MYSO>,
        treasury: &EcosystemTreasury,
        platform: &mut social_contracts::platform::Platform,
        clock: &Clock,
        ctx: &mut TxContext
    ): (Coin<MYSO>, u64, u64, u64, u64) {
        // Validate fees and calculate with overflow protection
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Distribute fees (same pattern as trading fees)
        if (fee_amount > 0) {
            // Send creator fee without PoC redirection
            if (creator_fee > 0) {
                distribute_reservation_creator_fee_no_poc(reservation_pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, clock, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Return remaining payment and fee amounts
        (payment, fee_amount, creator_fee, platform_fee, treasury_fee)
    }

    // === Trading Functions ===

    /// Buy tokens from the pool - first purchase
    /// Non-platform version: platform fees go to ecosystem treasury
    /// This function handles buying tokens for first-time buyers of a specific token
    #[allow(lint(self_transfer))]
    public fun buy_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        mut payment: Coin<MYSO>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let buyer = tx_context::sender(ctx);
        
        // Look up the buyer's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, buyer);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
        // Check if token owner is blocked by the buyer
        assert!(!block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
        // Calculate the price for the tokens based on quadratic curve
        let (price, _) = calculate_buy_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );
        
        // Ensure buyer has enough funds
        assert!(coin::value(&payment) >= price, EInsufficientFunds);
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(price, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to ecosystem treasury (no platform involved)
            if (platform_fee > 0) {
                let platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                transfer::public_transfer(platform_fee_coin, profile::get_treasury_address(treasury));
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Add remaining payment to pool
        let pool_payment = coin::split(&mut payment, net_amount, ctx);
        balance::join(&mut pool.myso_balance, coin::into_balance(pool_payment));
        
        // Refund any excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, buyer);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Update holder's balance with overflow protection
        // First check addition overflow
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        let new_supply = pool.info.circulating_supply + amount;
        
        // Then check multiplication overflow for max_hold calculation
        assert!(new_supply == 0 || new_supply <= MAX_U64 / config.max_hold_percent_bps, EOverflow);
        let max_hold = (new_supply * config.max_hold_percent_bps) / BPS_DENOM;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit with overflow protection
        assert!(current_hold <= MAX_U64 - amount, EOverflow);
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Check that this is the first purchase (user must not already own tokens)
        assert!(current_hold == 0, EAlreadyOwnsTokens);
        
        // Update holder's balance
        table::add(&mut pool.holders, buyer, amount);

        // Update circulating supply

        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        pool.info.circulating_supply = pool.info.circulating_supply + amount;
        
        // Mint new social token for the user
        let social_token = SocialToken {
            id: object::new(ctx),
            pool_id: object::uid_to_address(&pool.id),
            token_type: pool.info.token_type,
            amount,
        };
        transfer::transfer(social_token, buyer);
        
        // Calculate the new price after purchase
        let new_price = calculate_token_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        );
        
        // Emit buy event
        event::emit(TokenBoughtEvent {
            id: object::uid_to_address(&pool.id),
            buyer,
            amount,
            myso_amount: price,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            new_price,
        });
    }

    /// Buy tokens from the pool - first purchase
    /// Platform version: platform fees go to platform treasury, includes platform validation
    /// This function handles buying tokens for first-time buyers of a specific token
    #[allow(lint(self_transfer))]
    public fun buy_tokens_with_platform(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        platform: &mut social_contracts::platform::Platform,
        mut payment: Coin<MYSO>,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let buyer = tx_context::sender(ctx);
        
        // Look up the buyer's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, buyer);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, buyer), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, buyer), EUserBlockedByPlatform);
        
        // Check if token owner is blocked by the buyer
        assert!(!block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
        // Calculate the price for the tokens based on quadratic curve
        let (price, _) = calculate_buy_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );
        
        // Ensure buyer has enough funds
        assert!(coin::value(&payment) >= price, EInsufficientFunds);
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(price, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, clock, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Add remaining payment to pool
        let pool_payment = coin::split(&mut payment, net_amount, ctx);
        balance::join(&mut pool.myso_balance, coin::into_balance(pool_payment));
        
        // Refund any excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, buyer);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Update holder's balance with overflow protection
        // First check addition overflow
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        let new_supply = pool.info.circulating_supply + amount;
        
        // Then check multiplication overflow for max_hold calculation
        assert!(new_supply == 0 || new_supply <= MAX_U64 / config.max_hold_percent_bps, EOverflow);
        let max_hold = (new_supply * config.max_hold_percent_bps) / BPS_DENOM;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit with overflow protection
        assert!(current_hold <= MAX_U64 - amount, EOverflow);
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Check that this is the first purchase (user must not already own tokens)
        assert!(current_hold == 0, EAlreadyOwnsTokens);
        
        // Update holder's balance
        table::add(&mut pool.holders, buyer, amount);

        // Update circulating supply
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        pool.info.circulating_supply = pool.info.circulating_supply + amount;
        
        // Mint new social token for the user
        let social_token = SocialToken {
            id: object::new(ctx),
            pool_id: object::uid_to_address(&pool.id),
            token_type: pool.info.token_type,
            amount,
        };
        transfer::transfer(social_token, buyer);
        
        // Calculate the new price after purchase
        let new_price = calculate_token_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        );
        
        // Emit buy event
        event::emit(TokenBoughtEvent {
            id: object::uid_to_address(&pool.id),
            buyer,
            amount,
            myso_amount: price,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            new_price,
        });
    }

    /// Buy more tokens when you already have a social token
    /// Non-platform version: platform fees go to ecosystem treasury
    /// This function allows users to add to their existing token holdings using MYSO Coin
    #[allow(lint(self_transfer))]
    public fun buy_more_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        mut payment: Coin<MYSO>,
        amount: u64,
        social_token: &mut SocialToken,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let buyer = tx_context::sender(ctx);
        
        // Look up the buyer's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, buyer);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
        // Check if token owner is blocked by the buyer
        assert!(!block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
        // Verify social token matches the pool and is an active position
        assert!(social_token.pool_id == object::uid_to_address(&pool.id), EInvalidID);
        assert!(social_token.amount > 0, ENoTokensOwned);
        
        // Calculate the price for the tokens based on quadratic curve
        let (price, _) = calculate_buy_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );
        
        // Ensure buyer has enough funds
        assert!(coin::value(&payment) >= price, EInsufficientFunds);
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(price, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to ecosystem treasury (no platform involved)
            if (platform_fee > 0) {
                let platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                transfer::public_transfer(platform_fee_coin, profile::get_treasury_address(treasury));
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Add remaining payment to pool
        let pool_payment = coin::split(&mut payment, net_amount, ctx);
        balance::join(&mut pool.myso_balance, coin::into_balance(pool_payment));
        
        // Refund any excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, buyer);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Update holder's balance with overflow protection
        // First check addition overflow
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        let new_supply = pool.info.circulating_supply + amount;
        
        // Then check multiplication overflow for max_hold calculation
        assert!(new_supply == 0 || new_supply <= MAX_U64 / config.max_hold_percent_bps, EOverflow);
        let max_hold = (new_supply * config.max_hold_percent_bps) / BPS_DENOM;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit with overflow protection
        assert!(current_hold <= MAX_U64 - amount, EOverflow);
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Update holder's balance
        if (table::contains(&pool.holders, buyer)) {
            let holder_balance = table::borrow_mut(&mut pool.holders, buyer);
            assert!(*holder_balance <= MAX_U64 - amount, EOverflow);
            *holder_balance = *holder_balance + amount;
        } else {
            table::add(&mut pool.holders, buyer, amount);
        };

        // Update circulating supply
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        pool.info.circulating_supply = pool.info.circulating_supply + amount;

        // Update the user's social token

        assert!(social_token.amount <= MAX_U64 - amount, EOverflow);
        social_token.amount = social_token.amount + amount;
        
        // Calculate the new price after purchase
        let new_price = calculate_token_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        );
        
        // Emit buy event
        event::emit(TokenBoughtEvent {
            id: object::uid_to_address(&pool.id),
            buyer,
            amount,
            myso_amount: price,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            new_price,
        });
    }

    /// Buy more tokens when you already have a social token
    /// Platform version: platform fees go to platform treasury, includes platform validation
    /// This function allows users to add to their existing token holdings using MYSO Coin
    #[allow(lint(self_transfer))]
    public fun buy_more_tokens_with_platform(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        platform: &mut social_contracts::platform::Platform,
        mut payment: Coin<MYSO>,
        amount: u64,
        social_token: &mut SocialToken,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let buyer = tx_context::sender(ctx);
        
        // Look up the buyer's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, buyer);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, buyer), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, buyer), EUserBlockedByPlatform);
        
        // Check if token owner is blocked by the buyer
        assert!(!block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
        // Verify social token matches the pool and is an active position
        assert!(social_token.pool_id == object::uid_to_address(&pool.id), EInvalidID);
        assert!(social_token.amount > 0, ENoTokensOwned);
        
        // Calculate the price for the tokens based on quadratic curve
        let (price, _) = calculate_buy_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );
        
        // Ensure buyer has enough funds
        assert!(coin::value(&payment) >= price, EInsufficientFunds);
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(price, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, clock, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Add remaining payment to pool
        let pool_payment = coin::split(&mut payment, net_amount, ctx);
        balance::join(&mut pool.myso_balance, coin::into_balance(pool_payment));
        
        // Refund any excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, buyer);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Update holder's balance with overflow protection
        // First check addition overflow
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        let new_supply = pool.info.circulating_supply + amount;
        
        // Then check multiplication overflow for max_hold calculation
        assert!(new_supply == 0 || new_supply <= MAX_U64 / config.max_hold_percent_bps, EOverflow);
        let max_hold = (new_supply * config.max_hold_percent_bps) / BPS_DENOM;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit with overflow protection
        assert!(current_hold <= MAX_U64 - amount, EOverflow);
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Update holder's balance
        if (table::contains(&pool.holders, buyer)) {
            let holder_balance = table::borrow_mut(&mut pool.holders, buyer);
            assert!(*holder_balance <= MAX_U64 - amount, EOverflow);
            *holder_balance = *holder_balance + amount;
        } else {
            table::add(&mut pool.holders, buyer, amount);
        };

        // Update circulating supply
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        pool.info.circulating_supply = pool.info.circulating_supply + amount;

        // Update the user's social token
        assert!(social_token.amount <= MAX_U64 - amount, EOverflow);
        social_token.amount = social_token.amount + amount;
        
        // Calculate the new price after purchase
        let new_price = calculate_token_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        );
        
        // Emit buy event
        event::emit(TokenBoughtEvent {
            id: object::uid_to_address(&pool.id),
            buyer,
            amount,
            myso_amount: price,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            new_price,
        });
    }

    /// Sell tokens back to the pool
    /// Non-platform version: platform fees go to ecosystem treasury
    /// Consumes the SocialToken by value. On a partial sell a new remainder token is minted and
    /// transferred back to the seller; on a full sell the object is deleted — no zombie tokens.
    #[allow(lint(self_transfer))]
    public fun sell_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        profile_registry: &UsernameRegistry,
        _block_list_registry: &BlockListRegistry,
        social_token: SocialToken,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);

        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);

        let seller = tx_context::sender(ctx);
        let pool_id = object::uid_to_address(&pool.id);

        // Look up the seller's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, seller);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);

        // Verify social token matches the pool and has sufficient balance
        assert!(social_token.pool_id == pool_id, EInvalidID);
        assert!(social_token.amount >= amount, EInsufficientLiquidity);

        // Calculate the sell price based on quadratic curve
        let (refund_amount, _) = calculate_sell_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );

        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(refund_amount, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;

        // Calculate net refund
        let net_refund = refund_amount - fee_amount;

        // Ensure pool has enough liquidity for refund + all fees
        assert!(balance::value(&pool.myso_balance) >= refund_amount, EInsufficientLiquidity);

        // Verify seller has tokens in the pool
        assert!(table::contains(&pool.holders, seller), ENoTokensOwned);

        // Update holder balance
        let holder_balance = table::borrow_mut(&mut pool.holders, seller);
        if (*holder_balance == amount) {
            // Remove holder completely if selling all tokens
            table::remove(&mut pool.holders, seller);
        } else {
            // Reduce balance
            *holder_balance = *holder_balance - amount;
        };

        // Consume the social token: on partial sell mint a remainder token; on full sell delete.
        let remainder = social_token.amount - amount;
        let SocialToken { id, pool_id: token_pool_id, token_type, amount: _ } = social_token;
        object::delete(id);
        if (remainder > 0) {
            let remainder_token = SocialToken {
                id: object::new(ctx),
                pool_id: token_pool_id,
                token_type,
                amount: remainder,
            };
            transfer::transfer(remainder_token, seller);
        };

        // Update circulating supply
        pool.info.circulating_supply = pool.info.circulating_supply - amount;

        // Extract net refund from pool
        let refund_balance = balance::split(&mut pool.myso_balance, net_refund);

        // Process and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send fee to creator with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee_from_pool(pool, creator_fee, ctx);
            };

            // Send platform fee to ecosystem treasury (no platform involved)
            if (platform_fee > 0) {
                let platform_fee_coin = coin::from_balance(balance::split(&mut pool.myso_balance, platform_fee), ctx);
                transfer::public_transfer(platform_fee_coin, profile::get_treasury_address(treasury));
            };

            // Send fee to treasury
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::from_balance(balance::split(&mut pool.myso_balance, treasury_fee), ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };

        // Transfer refund to seller
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, seller);

        // Calculate the new price after sale
        let new_price = calculate_token_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        );

        // Emit sell event
        event::emit(TokenSoldEvent {
            id: pool_id,
            seller,
            amount,
            myso_amount: refund_amount,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            new_price,
        });
    }

    /// Sell tokens back to the pool
    /// Platform version: platform fees go to platform treasury, includes platform validation.
    /// Consumes the SocialToken by value. On a partial sell a new remainder token is minted and
    /// transferred back to the seller; on a full sell the object is deleted — no zombie tokens.
    #[allow(lint(self_transfer))]
    public fun sell_tokens_with_platform(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        platform: &mut social_contracts::platform::Platform,
        social_token: SocialToken,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);

        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);

        let seller = tx_context::sender(ctx);
        let pool_id = object::uid_to_address(&pool.id);

        // Look up the seller's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, seller);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);

        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, seller), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, seller), EUserBlockedByPlatform);

        // Verify social token matches the pool and has sufficient balance
        assert!(social_token.pool_id == pool_id, EInvalidID);
        assert!(social_token.amount >= amount, EInsufficientLiquidity);

        // Calculate the sell price based on quadratic curve
        let (refund_amount, _) = calculate_sell_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );

        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(refund_amount, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;

        // Calculate net refund
        let net_refund = refund_amount - fee_amount;

        // Ensure pool has enough liquidity for refund + all fees
        assert!(balance::value(&pool.myso_balance) >= refund_amount, EInsufficientLiquidity);

        // Verify seller has tokens in the pool
        assert!(table::contains(&pool.holders, seller), ENoTokensOwned);

        // Update holder balance
        let holder_balance = table::borrow_mut(&mut pool.holders, seller);
        if (*holder_balance == amount) {
            // Remove holder completely if selling all tokens
            table::remove(&mut pool.holders, seller);
        } else {
            // Reduce balance
            *holder_balance = *holder_balance - amount;
        };

        // Consume the social token: on partial sell mint a remainder token; on full sell delete.
        let remainder = social_token.amount - amount;
        let SocialToken { id, pool_id: token_pool_id, token_type, amount: _ } = social_token;
        object::delete(id);
        if (remainder > 0) {
            let remainder_token = SocialToken {
                id: object::new(ctx),
                pool_id: token_pool_id,
                token_type,
                amount: remainder,
            };
            transfer::transfer(remainder_token, seller);
        };

        // Update circulating supply
        pool.info.circulating_supply = pool.info.circulating_supply - amount;

        // Extract net refund from pool
        let refund_balance = balance::split(&mut pool.myso_balance, net_refund);

        // Process and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send fee to creator with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee_from_pool(pool, creator_fee, ctx);
            };

            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::from_balance(balance::split(&mut pool.myso_balance, platform_fee), ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, clock, ctx);
                coin::destroy_zero(platform_fee_coin);
            };

            // Send fee to treasury
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::from_balance(balance::split(&mut pool.myso_balance, treasury_fee), ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };

        // Transfer refund to seller
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, seller);

        // Calculate the new price after sale
        let new_price = calculate_token_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        );

        // Emit sell event
        event::emit(TokenSoldEvent {
            id: pool_id,
            seller,
            amount,
            myso_amount: refund_amount,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            new_price,
        });
    }

    // --- Wide `u256` AMM math (`u256::max_value!()` in stdlib); avoids overflowing `coeff * cube_diff` ---

    fun unwrap_u256_opt(o: Option<u256>): u256 {
        assert!(option::is_some(&o), EOverflow);
        option::destroy_some(o)
    }

    /// Returns `(sum mod 2^256, carry)` where carry is `0` or `1`.
    fun u256_add_with_carry(a: u256, b: u256): (u256, u256) {
        let s = u256::checked_add(a, b);
        if (option::is_some(&s)) {
            (option::destroy_some(s), 0u256)
        } else {
            let max = u256::max_value!();
            let t = unwrap_u256_opt(u256::checked_sub(max, b));
            let u = unwrap_u256_opt(u256::checked_add(t, 1u256));
            let low = unwrap_u256_opt(u256::checked_sub(a, u));
            (low, 1u256)
        }
    }

    fun u256_mul_widen(x: u256, y: u256): (u256, u256) {
        let b128 = 1u256 << 128;
        let mask = b128 - 1u256;
        let xl = x & mask;
        let xh = x >> 128;
        let yl = y & mask;
        let yh = y >> 128;
        let p0 = xl * yl;
        let p1 = xh * yl;
        let p2 = xl * yh;
        let p3 = xh * yh;
        let p1_lo = p1 & mask;
        let p1_hi = p1 >> 128;
        let p2_lo = p2 & mask;
        let p2_hi = p2 >> 128;
        let t_lo = p1_lo + p2_lo;
        let carry_tl = t_lo >> 128;
        let t_lo_final = t_lo & mask;
        let t_hi = p1_hi + p2_hi + carry_tl;
        let shift_part = t_lo_final * b128;
        let (w0, c0) = u256_add_with_carry(p0, shift_part);
        let (t_sum, c1) = u256_add_with_carry(p3, t_hi);
        let (hi_acc, c2) = u256_add_with_carry(t_sum, c0);
        assert!(c1 == 0u256 && c2 == 0u256, EOverflow);
        (hi_acc, w0)
    }

    fun u512_bit(n_hi: u256, n_lo: u256, i: u64): bool {
        if (i < 256) {
            ((n_lo >> (i as u8)) & 1u256) != 0u256
        } else {
            let j = i - 256;
            ((n_hi >> (j as u8)) & 1u256) != 0u256
        }
    }

    fun u512_shl1_or_bit(r_hi: u256, r_lo: u256, bit: bool): (u256, u256) {
        let carry = r_lo >> 255;
        let nl_lo = (r_lo << 1) | (if (bit) { 1u256 } else { 0u256 });
        let nl_hi = (r_hi << 1) | carry;
        (nl_hi, nl_lo)
    }

    fun u512_ge_u256(r_hi: u256, r_lo: u256, d: u256): bool {
        r_hi > 0u256 || r_lo >= d
    }

    fun u512_sub_u256(r_hi: u256, r_lo: u256, d: u256): (u256, u256) {
        let sub_lo = u256::checked_sub(r_lo, d);
        if (option::is_some(&sub_lo)) {
            (r_hi, option::destroy_some(sub_lo))
        } else {
            let new_hi = unwrap_u256_opt(u256::checked_sub(r_hi, 1u256));
            let bump = u256::max_value!() - d + 1u256;
            let new_lo = unwrap_u256_opt(u256::checked_add(r_lo, bump));
            (new_hi, new_lo)
        }
    }

    /// `floor((n_hi*2^256 + n_lo) / d)` for `d > 0`.
    fun u512_div_u256_floor(n_hi: u256, n_lo: u256, d: u256): u256 {
        assert!(d > 0u256, EInvalidCurveParams);
        if (n_hi == 0u256) {
            return n_lo / d
        };
        let mut r_hi = 0u256;
        let mut r_lo = 0u256;
        let mut q = 0u256;
        let mut i = 512u64;
        while (i > 0) {
            i = i - 1;
            let bit = u512_bit(n_hi, n_lo, i);
            let (nr_hi, nr_lo) = u512_shl1_or_bit(r_hi, r_lo, bit);
            r_hi = nr_hi;
            r_lo = nr_lo;
            if (u512_ge_u256(r_hi, r_lo, d)) {
                let (sr_hi, sr_lo) = u512_sub_u256(r_hi, r_lo, d);
                r_hi = sr_hi;
                r_lo = sr_lo;
                let inc = 1u256 << (i as u8);
                q = unwrap_u256_opt(u256::checked_add(q, inc));
            };
        };
        q
    }

    /// `3*s*s + 3*s*a + a*a` for buy integral `(s+a)^3 - s^3 = a * poly`.
    fun quad_poly_buy(s: u256, a: u256): u256 {
        let s2 = unwrap_u256_opt(u256::checked_mul(s, s));
        let three_s2 = unwrap_u256_opt(u256::checked_mul(3u256, s2));
        let sa = unwrap_u256_opt(u256::checked_mul(s, a));
        let three_sa = unwrap_u256_opt(u256::checked_mul(3u256, sa));
        let a2 = unwrap_u256_opt(u256::checked_mul(a, a));
        let t = unwrap_u256_opt(u256::checked_add(three_s2, three_sa));
        unwrap_u256_opt(u256::checked_add(t, a2))
    }

    /// `3*s*s - 3*s*a + a*a` for sell integral `s^3 - (s-a)^3 = a * poly`.
    fun quad_poly_sell(s: u256, a: u256): u256 {
        let s2 = unwrap_u256_opt(u256::checked_mul(s, s));
        let three_s2 = unwrap_u256_opt(u256::checked_mul(3u256, s2));
        let sa = unwrap_u256_opt(u256::checked_mul(s, a));
        let three_sa = unwrap_u256_opt(u256::checked_mul(3u256, sa));
        let a2 = unwrap_u256_opt(u256::checked_mul(a, a));
        let t = unwrap_u256_opt(u256::checked_sub(three_s2, three_sa));
        unwrap_u256_opt(u256::checked_add(t, a2))
    }

    fun quad_integral_leg_mist(coeff: u256, s: u256, a: u256, scale: u256, is_buy: bool): u256 {
        let poly = if (is_buy) {
            quad_poly_buy(s, a)
        } else {
            quad_poly_sell(s, a)
        };
        let ca = unwrap_u256_opt(u256::checked_mul(coeff, a));
        let (numer_hi, numer_lo) = u256_mul_widen(ca, poly);
        let denom = unwrap_u256_opt(u256::checked_mul(
            30000u256,
            unwrap_u256_opt(u256::checked_mul(scale, unwrap_u256_opt(u256::checked_mul(scale, scale))))
        ));
        u512_div_u256_floor(numer_hi, numer_lo, denom)
    }

    /// MYSO amounts on-chain are `u64` (smallest units). Abort `EOverflow` if `x` does not fit.
    fun mist_amount_u256_to_u64(x: u256): u64 {
        let o = u256::try_as_u64(x);
        assert!(option::is_some(&o), EOverflow);
        option::destroy_some(o)
    }

    // === Utility Functions ===

    /// Marginal MYSO price for the next infinitesimal nano-SPT at `supply_nano` (nano-SPT in pool).
    /// `p(s) = base_price + quadratic_coefficient * (s / SPT_SCALE)^2 / BPS_DENOM` (permyriad).
    public fun calculate_token_price(
        base_price: u64,
        quadratic_coefficient: u64,
        supply_nano: u64
    ): u64 {
        let base = base_price as u256;
        let coeff = quadratic_coefficient as u256;
        let s = supply_nano as u256;
        let scale = SPT_SCALE as u256;
        let scale2 = unwrap_u256_opt(u256::checked_mul(scale, scale));
        let denom = unwrap_u256_opt(u256::checked_mul(scale2, BPS_DENOM as u256));
        assert!(denom > 0u256, EInvalidCurveParams);
        let s2 = unwrap_u256_opt(u256::checked_mul(s, s));
        let coeff_s2 = unwrap_u256_opt(u256::checked_mul(coeff, s2));
        let quad = coeff_s2 / denom;
        let total = unwrap_u256_opt(u256::checked_add(base, quad));
        mist_amount_u256_to_u64(total)
    }

    /// Total MYSO cost to buy `amount_nano` nano-SPT when current circulating supply is `current_supply_nano`.
    /// Uses the closed-form integral of the marginal quadratic curve over human supply
    /// (continuous approximation; `amount` and `supply` are nano-SPT).
    /// Returns `(total_mysos, avg_mysos_per_nano_unit)`.
    public fun calculate_buy_price(
        base_price: u64,
        quadratic_coefficient: u64,
        current_supply_nano: u64,
        amount_nano: u64
    ): (u64, u64) {
        if (amount_nano == 0) {
            return (0, 0)
        };
        let base = base_price as u256;
        let coeff = quadratic_coefficient as u256;
        let s = current_supply_nano as u256;
        let a = amount_nano as u256;
        let scale = SPT_SCALE as u256;

        let base_prod = unwrap_u256_opt(u256::checked_mul(base, a));
        let base_part = base_prod / scale;

        let quad_part = quad_integral_leg_mist(coeff, s, a, scale, true);

        let total = unwrap_u256_opt(u256::checked_add(base_part, quad_part));
        let total_u64 = mist_amount_u256_to_u64(total);
        let avg_u64 = mist_amount_u256_to_u64(total / a);
        (total_u64, avg_u64)
    }

    /// MYSO refund for selling `amount_nano` nano-SPT when current circulating supply is `current_supply_nano`.
    /// Returns `(total_refund_mysos, avg_mysos_per_nano_unit)`.
    public fun calculate_sell_price(
        base_price: u64,
        quadratic_coefficient: u64,
        current_supply_nano: u64,
        amount_nano: u64
    ): (u64, u64) {
        if (amount_nano == 0) {
            return (0, 0)
        };
        assert!(current_supply_nano >= amount_nano, EInsufficientLiquidity);

        let base = base_price as u256;
        let coeff = quadratic_coefficient as u256;
        let s = current_supply_nano as u256;
        let a = amount_nano as u256;
        let scale = SPT_SCALE as u256;

        let base_prod = unwrap_u256_opt(u256::checked_mul(base, a));
        let base_part = base_prod / scale;

        let quad_part = quad_integral_leg_mist(coeff, s, a, scale, false);

        let total = unwrap_u256_opt(u256::checked_add(base_part, quad_part));
        let total_u64 = mist_amount_u256_to_u64(total);
        let avg_u64 = mist_amount_u256_to_u64(total / a);
        (total_u64, avg_u64)
    }

    /// `10^9` nano-SPT per 1.0 display token (for clients / indexers).
    public fun spt_amount_scale(): u64 {
        SPT_SCALE
    }

    /// Display decimals for SPT quantities (matches native MYSO).
    public fun spt_amount_decimals(): u8 {
        SPT_DECIMALS
    }

    /// Whole display tokens → nano-SPT (`whole * SPT_SCALE`). Aborts with `EOverflow` if the product does not fit `u64`.
    public fun nano_spt_from_whole_tokens(whole: u64): u64 {
        let p = (whole as u128) * (SPT_SCALE as u128);
        assert!(p <= MAX_ONCHAIN_U64_U128, EOverflow);
        p as u64
    }

    /// `whole` display tokens plus `fraction_nano` nano-SPT remainder (`fraction_nano < SPT_SCALE`).
    public fun nano_spt_from_whole_and_fraction(whole: u64, fraction_nano: u64): u64 {
        assert!(fraction_nano < SPT_SCALE, EInvalidCurveParams);
        let w = nano_spt_from_whole_tokens(whole);
        assert!(w <= MAX_U64 - fraction_nano, EOverflow);
        w + fraction_nano
    }

    /// Get token info from registry by associated_id (post/profile ID), not pool ID
    /// Returns a reference since TokenInfo no longer has copy ability
    public fun get_token_info(registry: &TokenRegistry, id: address): &TokenInfo {
        assert!(table::contains(&registry.tokens, id), ETokenNotFound);
        table::borrow(&registry.tokens, id)
    }

    /// Circulating supply (nano-SPT) from a `TokenInfo` reference.
    public fun token_info_circulating_supply(info: &TokenInfo): u64 {
        info.circulating_supply
    }

    /// Check if a token exists in the registry
    public fun token_exists(registry: &TokenRegistry, id: address): bool {
        table::contains(&registry.tokens, id)
    }

    /// Get token owner's address
    public fun get_token_owner(registry: &TokenRegistry, id: address): address {
        let info = get_token_info(registry, id);
        info.owner
    }

    /// Get current token price for a specific pool
    public fun get_pool_price(pool: &TokenPool): u64 {
        calculate_token_price(
            pool.info.base_price, 
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        )
    }

    /// Get user's token balance
    public fun get_user_balance(pool: &TokenPool, user: address): u64 {
        if (table::contains(&pool.holders, user)) {
            *table::borrow(&pool.holders, user)
        } else {
            0
        }
    }

    /// Get PoC redirection data from token pool
    public fun get_poc_redirect_to(pool: &TokenPool): &Option<address> {
        &pool.poc_redirect_to
    }

    /// Get PoC redirection percentage from token pool
    public fun get_poc_redirect_percentage(pool: &TokenPool): &Option<u64> {
        &pool.poc_redirect_percentage
    }

    /// Check if token pool has PoC redirection configured
    public fun has_poc_redirection(pool: &TokenPool): bool {
        option::is_some(&pool.poc_redirect_to) && option::is_some(&pool.poc_redirect_percentage)
    }

    /// Get the associated ID (post/profile ID) from a token pool
    public fun get_pool_associated_id(pool: &TokenPool): address {
        pool.info.associated_id
    }

    /// Set PoC redirection data for a token pool (called by PoC system)
    /// Set PoC redirection for a token pool (package-only, requires auth via entry function)
    public(package) fun set_poc_redirection(
        pool: &mut TokenPool,
        redirect_to: Option<address>,
        redirect_percentage: Option<u64>,
        poc_redirection_kind: u8,
    ) {
        assert!(poc_redirection_kind <= 2, EInvalidFeeConfig);
        if (poc_redirection_kind == 0) {
            assert!(option::is_none(&redirect_to), EInvalidFeeConfig);
            assert!(option::is_none(&redirect_percentage), EInvalidFeeConfig);
        } else {
            assert!(option::is_some(&redirect_to), EInvalidFeeConfig);
            assert!(option::is_some(&redirect_percentage), EInvalidFeeConfig);
            let percentage = *option::borrow(&redirect_percentage);
            assert!(percentage <= 100, EInvalidFeeConfig);
        };
        pool.poc_redirect_to = redirect_to;
        pool.poc_redirect_percentage = redirect_percentage;
        pool.poc_redirection_kind = poc_redirection_kind;
    }
    
    /// Entry function to set PoC redirection (requires pool owner)
    public entry fun set_poc_redirection_entry(
        registry: &TokenRegistry,
        pool: &mut TokenPool,
        redirect_to: Option<address>,
        redirect_percentage: Option<u64>,
        poc_redirection_kind: u8,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        let token_info = get_token_info(registry, pool.info.associated_id);
        
        // Require caller to be pool owner
        assert!(caller == token_info.owner, ENotAuthorized);
        
        set_poc_redirection(pool, redirect_to, redirect_percentage, poc_redirection_kind);
    }
    
    /// Admin entry function to set PoC redirection (requires admin cap)
    public entry fun set_poc_redirection_admin(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        _admin_cap: &SocialProofTokensAdminCap,
        redirect_to: Option<address>,
        redirect_percentage: Option<u64>,
        poc_redirection_kind: u8,
        _ctx: &mut TxContext
    ) {
        // Admin can set redirection for any pool
        set_poc_redirection(pool, redirect_to, redirect_percentage, poc_redirection_kind);
    }

    /// Clear PoC redirection data from a token pool (called by PoC system)
    public(package) fun clear_poc_redirection(pool: &mut TokenPool) {
        pool.poc_redirect_to = option::none();
        pool.poc_redirect_percentage = option::none();
        pool.poc_redirection_kind = 0;
    }


    // Test-only functions
    #[test_only]
    public fun register_token_info_for_testing(
        registry: &mut TokenRegistry,
        token_key: address,
        info: TokenInfo,
    ) {
        assert!(!table::contains(&registry.tokens, token_key), ETokenAlreadyExists);
        table::add(&mut registry.tokens, token_key, info);
    }

    #[test_only]
    public fun poc_redirection_kind_for_testing(pool: &TokenPool): u8 {
        pool.poc_redirection_kind
    }

    #[test_only]
    /// Initialize the social proof tokens system for testing
    /// In testing, we create admin caps directly for convenience
    public fun init_for_testing(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        
        // Create and transfer admin capability to the transaction sender
        transfer::public_transfer(
            SocialProofTokensAdminCap {
                id: object::new(ctx),
            },
            sender
        );
        
        // Create and share social proof tokens config (same as production init)
        transfer::share_object(
            SocialProofTokensConfig {
                id: object::new(ctx),
                version: upgrade::current_version(),
                trading_creator_fee_bps: DEFAULT_TRADING_CREATOR_FEE_BPS,
                trading_platform_fee_bps: DEFAULT_TRADING_PLATFORM_FEE_BPS,
                trading_treasury_fee_bps: DEFAULT_TRADING_TREASURY_FEE_BPS,
                reservation_creator_fee_bps: DEFAULT_RESERVATION_CREATOR_FEE_BPS,
                reservation_platform_fee_bps: DEFAULT_RESERVATION_PLATFORM_FEE_BPS,
                reservation_treasury_fee_bps: DEFAULT_RESERVATION_TREASURY_FEE_BPS,
                base_price: DEFAULT_BASE_PRICE,
                quadratic_coefficient: DEFAULT_QUADRATIC_COEFFICIENT,
                max_hold_percent_bps: MAX_HOLD_PERCENT_BPS,
                post_threshold: DEFAULT_POST_THRESHOLD,
                profile_threshold: DEFAULT_PROFILE_THRESHOLD,
                max_individual_reservation_bps: DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS,
                max_reservers_per_pool: DEFAULT_MAX_RESERVERS_PER_POOL,
                trading_enabled: true,
                non_platform_platform_to_creator_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS,
                non_platform_platform_to_treasury_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS,
            }
        );
        
        // Create and share token registry
        transfer::share_object(
            TokenRegistry {
                id: object::new(ctx),
                tokens: table::new(ctx),
                reservation_pools: table::new(ctx),
                version: upgrade::current_version(),
            }
        );
    }
    /// Create a new SocialProofTokensAdminCap for testing
    #[test_only]
    public fun create_admin_cap_for_testing(ctx: &mut TxContext): SocialProofTokensAdminCap {
        SocialProofTokensAdminCap {
            id: object::new(ctx)
        }
    }

    /// Create a mock TokenInfo for testing
    #[test_only]
    public fun create_mock_token_info(
        id: address,
        token_type: u8,
        owner: address,
        associated_id: address,
        circulating_supply: u64,
        base_price: u64,
        quadratic_coefficient: u64,
        created_at: u64
    ): TokenInfo {
        TokenInfo {
            id,
            token_type,
            owner,
            associated_id,
            circulating_supply,
            base_price,
            quadratic_coefficient,
            created_at,
        }
    }

    /// Create a mock TokenPool for testing
    #[test_only]
    public fun create_mock_token_pool(
        token_info: TokenInfo,
        ctx: &mut TxContext
    ): TokenPool {
        TokenPool {
            id: object::new(ctx),
            info: token_info,
            myso_balance: balance::zero(),
            holders: table::new(ctx),
            poc_redirect_to: option::none(),
            poc_redirect_percentage: option::none(),
            poc_redirection_kind: 0,
            version: upgrade::current_version(),
        }
    }

    /// Share a TokenPool for test cleanup (TokenPool is not transferable)
    #[test_only]
    public fun share_token_pool_for_testing(pool: TokenPool) {
        transfer::share_object(pool);
    }

    // === Versioning Functions ===

    /// Get the version of the token registry
    public fun registry_version(registry: &TokenRegistry): u64 {
        registry.version
    }

    /// Get a mutable reference to the registry version (for upgrade module)
    public(package) fun borrow_registry_version_mut(registry: &mut TokenRegistry): &mut u64 {
        &mut registry.version
    }

    /// Get the version of a token pool
    public fun pool_version(pool: &TokenPool): u64 {
        pool.version
    }

    /// Get a mutable reference to the pool version (for upgrade module)
    public(package) fun borrow_pool_version_mut(pool: &mut TokenPool): &mut u64 {
        &mut pool.version
    }

    /// Get the version of a reservation pool
    public fun reservation_pool_version(pool: &ReservationPoolObject): u64 {
        pool.version
    }

    #[test_only]
    public fun reservation_pool_total_reserved_for_testing(pool: &ReservationPoolObject): u64 {
        pool.info.total_reserved
    }

    #[test_only]
    public fun reservation_pool_myso_balance_value_for_testing(pool: &ReservationPoolObject): u64 {
        balance::value(&pool.myso_balance)
    }

    /// Get a mutable reference to the reservation pool version (for upgrade module)
    public(package) fun borrow_reservation_pool_version_mut(pool: &mut ReservationPoolObject): &mut u64 {
        &mut pool.version
    }

    /// Get the version of the social proof tokens config
    public fun config_version(config: &SocialProofTokensConfig): u64 {
        config.version
    }

    /// Get a mutable reference to the config version (for upgrade module)
    public(package) fun borrow_config_version_mut(config: &mut SocialProofTokensConfig): &mut u64 {
        &mut config.version
    }

    /// Migration function for TokenRegistry
    public entry fun migrate_token_registry(
        registry: &mut TokenRegistry,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(registry.version < current_version, EInvalidFeeConfig);
        
        // Remember old version and update to new version
        let old_version = registry.version;
        registry.version = current_version;
        
        // Emit event for object migration
        let registry_id = object::id(registry);
        upgrade::emit_migration_event(
            registry_id,
            string::utf8(b"TokenRegistry"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    /// Migration function for TokenPool
    public entry fun migrate_token_pool(
        pool: &mut TokenPool,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(pool.version < current_version, EInvalidFeeConfig);
        
        // Remember old version and update to new version
        let old_version = pool.version;
        pool.version = current_version;
        
        // Emit event for object migration
        let pool_id = object::id(pool);
        upgrade::emit_migration_event(
            pool_id,
            string::utf8(b"TokenPool"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    /// Migration function for ReservationPoolObject
    public entry fun migrate_reservation_pool(
        pool: &mut ReservationPoolObject,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(pool.version < current_version, EInvalidFeeConfig);
        
        // Remember old version and update to new version
        let old_version = pool.version;
        pool.version = current_version;
        
        // Emit event for object migration
        let pool_id = object::id(pool);
        upgrade::emit_migration_event(
            pool_id,
            string::utf8(b"ReservationPoolObject"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    /// Migration function for SocialProofTokensConfig
    public entry fun migrate_social_proof_tokens_config(
        config: &mut SocialProofTokensConfig,
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
            string::utf8(b"SocialProofTokensConfig"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }
    
    /// Create a SocialProofTokensAdminCap for bootstrap (package visibility only)
    /// This function is only callable by other modules in the same package
    public(package) fun create_social_proof_tokens_admin_cap(ctx: &mut TxContext): SocialProofTokensAdminCap {
        SocialProofTokensAdminCap {
            id: object::new(ctx)
        }
    }
} 
