// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Profile module for the MySocial network
/// Handles user identity, profile creation, management, and username registration

#[allow(duplicate_alias, lint(public_entry))]
module social_contracts::profile {
    use std::string::{Self, String};
    use std::ascii;
    use std::option::{Self, Option};
    
    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        event,
        table::{Self, Table},
        coin::{Self, Coin},
        balance::{Self, Balance},
        url::{Self, Url},
        clock::{Self, Clock}
    };
    use myso::myso::MYSO;
    use myso::address;

    use social_contracts::upgrade;
    use social_contracts::memory as memory;
    use social_contracts::ai_credit::{Self, AiCreditConfig};

    /// Error codes
    const EProfileAlreadyExists: u64 = 0;
    const EUnauthorized: u64 = 1;
    const EInvalidUsername: u64 = 2;
    const EReservedName: u64 = 4;
    const EUsernameNotAvailable: u64 = 5;
    const EOfferAlreadyExists: u64 = 7;
    const EOfferDoesNotExist: u64 = 8;
    const ECannotOfferOwnProfile: u64 = 9;
    const EInsufficientTokens: u64 = 10;
    const EUnauthorizedOfferAction: u64 = 11;
    const EOfferBelowMinimum: u64 = 12;
    const EBadgeNotFound: u64 = 13;
    const EBadgeAlreadyExists: u64 = 14;
    const ESelectedBadgeNotFound: u64 = 18;
    const EInvalidBadgeType: u64 = 19;
    const EBadgeNameTooLong: u64 = 20;
    const EBadgeDescriptionTooLong: u64 = 21;
    const EBadgeMediaUrlTooLong: u64 = 22;
    const EBadgeIconUrlTooLong: u64 = 23;
    const ENotEcosystemBadge: u64 = 24;
    const EInvalidStartTime: u64 = 15;
    const ENotVestingWalletOwner: u64 = 16;
    const EOverflow: u64 = 17;
    const EMemoryAlreadyLinked: u64 = 25;
    const EUsernameNotFound: u64 = 29;
    const EListingAlreadyExists: u64 = 37;
    const EListingNotFound: u64 = 38;
    const EListingHasOffers: u64 = 39;
    const EBuyerHasNoProfile: u64 = 40;
    const EUsernameLocked: u64 = 41;
    const EProfileAlreadyHasUsername: u64 = 42;
    const EUsernameProfileMismatch: u64 = 31;
    const EInvalidSchedule: u64 = 30;
    const EInvalidPieceDuration: u64 = 32;
    const EInvalidPieceKind: u64 = 33;
    const ETooManyPieces: u64 = 34;
    const EScheduleOverflow: u64 = 35;
    const EInvalidConfig: u64 = 36;

    const USERNAME_SALE_FEE_BPS: u64 = 500;

    /// Username lock reasons (stored in [`UsernameRegistry::username_locks`]).
    const USERNAME_LOCK_BENEFICIARY: u8 = 1;
    const USERNAME_LOCK_MARKETPLACE: u8 = 2;

    /// Default bootstrap values for ProfileConfig
    const CURVE_PRECISION: u64 = 1000;
    const CURVE_FACTOR_MIN: u64 = 100;
    const CURVE_FACTOR_MAX: u64 = 10000;
    const MAX_VESTING_PIECES: u64 = 10;
    const MIN_CLAIM_THRESHOLD_DIVISOR: u64 = 1000;
    const MIN_USERNAME_LENGTH: u64 = 2;
    const MAX_USERNAME_LENGTH: u64 = 50;
    const BPS_DENOMINATOR: u64 = 10_000;
    const PIECE_KIND_CLIFF: u8 = 0;
    const PIECE_KIND_CONTINUOUS: u8 = 1;

    // Maximum u64 value for overflow protection
    const MAX_U64: u64 = 18446744073709551615;

    /// Reserved usernames that cannot be registered
    const RESERVED_NAMES: vector<vector<u8>> = vector[
        b"admin",
        b"administrator",
        b"mod",
        b"moderator",
        b"staff",
        b"support",
        b"myso",
        b"mysocial",
        b"system",
        b"root",
        b"foundation",
    ];

    const MAX_BADGE_NAME_LENGTH: u64 = 100;
    const MAX_BADGE_DESCRIPTION_LENGTH: u64 = 500;
    const MAX_BADGE_MEDIA_URL_LENGTH: u64 = 2048;
    const MAX_BADGE_ICON_URL_LENGTH: u64 = 2048;
    const ECOSYSTEM_BADGE_PREFIX: vector<u8> = b"ecosystem_badge_";

    /// Admin capability for profile configuration
    public struct ProfileAdminCap has key, store {
        id: UID,
    }

    /// Global profile feature configuration
    public struct ProfileConfig has key {
        id: UID,
        max_vesting_pieces: u64,
        curve_factor_min: u64,
        curve_factor_max: u64,
        curve_precision: u64,
        min_claim_threshold_divisor: u64,
        min_username_length: u64,
        max_username_length: u64,
        /// Fee (bps) taken on username marketplace sales (`10_000` = 100%)
        username_sale_fee_bps: u64,
        version: u64,
    }

    public struct ProfileConfigUpdatedEvent has copy, drop {
        updated_by: address,
        max_vesting_pieces: u64,
        curve_factor_min: u64,
        curve_factor_max: u64,
        curve_precision: u64,
        min_claim_threshold_divisor: u64,
        min_username_length: u64,
        max_username_length: u64,
        username_sale_fee_bps: u64,
        timestamp: u64,
    }

    /// Admin capability for Ecosystem Treasury management
    public struct EcosystemTreasuryAdminCap has key, store {
        id: UID,
    }

    /// Admin capability for assigning ecosystem badges to user profiles
    public struct EcosystemBadgeAdminCap has key, store {
        id: UID,
    }

    /// Admin capability for username registry management (reserve, revoke, reassign)
    public struct UsernameAdminCap has key, store {
        id: UID,
    }

    /// Social Ecosystem Treasury that receives fees from username marketplace sales
    public struct EcosystemTreasury has key {
        id: UID,
        /// Treasury address that receives fees
        treasury_address: address,
        /// Version for upgrades
        version: u64,
    }

    /// Username Registry — sole on-chain store for username ownership
    public struct UsernameRegistry has key {
        id: UID,
        /// Canonical username → profile_id
        usernames: Table<String, address>,
        /// Owner wallet → profile_id
        address_profiles: Table<address, address>,
        /// profile_id → canonical username (enforces one username per profile)
        profile_username: Table<address, String>,
        /// Canonical username → lock reason while reserved in escrow (PoC/marketplace)
        username_locks: Table<String, u8>,
        version: u64,
    }

    /// Shared escrow for username listings and locked purchase offers
    public struct UsernameMarketplace has key {
        id: UID,
        listings: Table<String, UsernameListing>,
        version: u64,
    }

    /// Active listing for a canonical username
    public struct UsernameListing has store {
        seller: address,
        seller_profile_id: address,
        username: String,
        min_price: u64,
        created_at: u64,
        offerors: vector<address>,
        offers: Table<address, UsernameOffer>,
    }

    /// Locked bid on a username listing
    public struct UsernameOffer has store {
        buyer: address,
        buyer_profile_id: address,
        amount: u64,
        created_at: u64,
        locked_myso: Balance<MYSO>,
    }
    
    /// Profile object that contains user information
    public struct Profile has key {
        id: UID,
        /// Display name of the profile (optional)
        display_name: Option<String>,
        /// Bio of the profile
        bio: String,
        /// Profile picture URL
        profile_picture: Option<Url>,
        /// Cover photo URL
        cover_photo: Option<Url>,
        /// Creation timestamp
        created_at: u64,
        /// Profile owner address
        owner: address,
        /// X/Twitter username as encrypted string (optional)
        x_username: Option<String>,
        /// Profile website URL (optional)
        website: Option<String>,
        /// Birthdate as opaque string, e.g. ISO-8601 (optional)
        birthdate: Option<String>,
        /// Location label (optional)
        location: Option<String>,
        /// Collection of badges assigned to the profile
        badges: vector<ProfileBadge>,
        /// Badge ID of the selected/primary badge to display (optional)
        /// If None, the first badge in the badges vector should be displayed
        selected_badge_id: Option<String>,
        /// Badge ID of the selected ecosystem badge to display (optional)
        /// If None, the first ecosystem badge in the badges vector should be displayed
        selected_ecosystem_badge_id: Option<String>,
        /// Shared [`memory::MemoryAccount`] object id when linked (`None` until [`profile::ensure_memory_account`] or legacy profiles).
        memory_account_id: Option<ID>,
        /// Shared [`ai_credit::AiCreditBalance`] object id — always set at profile creation (greenfield).
        ai_credit_balance_id: Option<ID>,
        /// Version for upgrades
        version: u64,
    }

    /// Profile Badge that can be assigned to profiles by platform admins/moderators
    /// These badges cannot be transferred, sold, or copied and stay with the profile
    public struct ProfileBadge has store, drop {
        /// Unique identifier for the badge (platform ID + badge name)
        badge_id: String,
        /// Name of the badge
        name: String,
        /// Description of what the badge represents
        description: String,
        /// Media URL for the badge (can be image, video, etc.)
        media_url: String,
        /// Icon URL for the badge (small icon displayed next to username)
        icon_url: String,
        /// ID of the platform that issued the badge
        platform_id: address,
        /// Timestamp when the badge was issued
        issued_at: u64,
        /// Address of the admin/moderator who issued the badge
        issued_by: address,
        /// Badge type/tier (1-100), allows for badge hierarchy
        badge_type: u8,
    }

    /// Read-only badge data returned by query functions
    /// This struct has copy ability to allow returning badge information,
    /// but the actual ProfileBadge cannot be copied or transferred
    public struct BadgeData has copy, drop {
        badge_id: String,
        name: String,
        description: String,
        media_url: String,
        icon_url: String,
        platform_id: address,
        issued_at: u64,
        issued_by: address,
        badge_type: u8,
    }

    /// One schedule piece: lump cliff unlock or continuous curved vesting.
    public struct VestingPiece has copy, drop, store {
        /// 0 = CliffLump (instant unlock), 1 = ContinuousVest
        kind: u8,
        /// Milliseconds from `start_time` when this piece activates
        time_offset: u64,
        /// 0 for cliff lumps; vest window length for continuous pieces
        duration: u64,
        /// Share of total_amount in basis points; all pieces sum to 10_000
        amount_bps: u64,
        /// Curve factor for continuous pieces (1000 = linear)
        curve_factor: u64,
    }

    /// Vesting Wallet contains MYSO coins released over a piecewise schedule.
    public struct VestingWallet has key, store {
        id: UID,
        balance: Balance<MYSO>,
        owner: address,
        start_time: u64,
        claimed_amount: u64,
        total_amount: u64,
        schedule_end: u64,
        pieces: vector<VestingPiece>,
    }

    // === Events ===

    /// Event emitted when a badge is assigned to a profile
    public struct BadgeAssignedEvent has copy, drop {
        /// ID of the profile receiving the badge
        profile_id: address,
        /// Badge identifier
        badge_id: String,
        /// Badge name
        name: String,
        /// Description of what the badge represents
        description: String,
        /// Media URL for the badge (can be image, video, etc.)
        media_url: String,
        /// Icon URL for the badge (small icon displayed next to username)
        icon_url: String,
        /// Platform ID that issued the badge
        platform_id: address,
        /// Admin/moderator who assigned the badge
        assigned_by: address,
        /// Timestamp when assigned
        assigned_at: u64,
        /// Badge type/tier
        badge_type: u8,
    }

    /// Event emitted when a badge is revoked from a profile
    public struct BadgeRevokedEvent has copy, drop {
        /// ID of the profile losing the badge
        profile_id: address,
        /// Badge identifier
        badge_id: String,
        /// Platform ID that issued the badge
        platform_id: address,
        /// Admin/moderator who revoked the badge
        revoked_by: address,
        /// Timestamp when revoked
        revoked_at: u64,
    }

    /// Event emitted when a profile owner selects a badge to display
    public struct BadgeSelectedEvent has copy, drop {
        /// ID of the profile
        profile_id: address,
        /// Badge identifier that was selected
        badge_id: String,
        /// Owner who selected the badge
        selected_by: address,
        /// Timestamp when selected
        selected_at: u64,
    }

    /// Emitted when only ecosystem badge selection is cleared (see [`clear_selected_ecosystem_badge`]).
    public struct EcosystemBadgeSelectionClearedEvent has copy, drop {
        profile_id: address,
        cleared_by: address,
        cleared_at: u64,
    }

    /// Event emitted when a profile owner removes their own badge
    public struct BadgeRemovedEvent has copy, drop {
        /// ID of the profile
        profile_id: address,
        /// Badge identifier that was removed
        badge_id: String,
        /// Platform ID that issued the badge
        platform_id: address,
        /// Owner who removed the badge
        removed_by: address,
        /// Timestamp when removed
        removed_at: u64,
    }

    /// Profile created event (username is emitted via [`UsernameClaimedEvent`])
    public struct ProfileCreatedEvent has copy, drop {
        profile_id: address,
        display_name: String,
        bio: String,
        profile_picture: Option<String>,
        cover_photo: Option<String>,
        owner: address,
        created_at: u64,
    }

    /// Profile updated event with all profile details (username lives in registry)
    public struct ProfileUpdatedEvent has copy, drop {
        profile_id: address,
        display_name: Option<String>,
        bio: String,
        profile_picture: Option<String>,
        cover_photo: Option<String>,
        owner: address,
        updated_at: u64,
        x_username: Option<String>,
        website: Option<String>,
        birthdate: Option<String>,
        location: Option<String>,
    }

    /// Emitted when a username is claimed at profile creation
    public struct UsernameClaimedEvent has copy, drop {
        username: String,
        profile_id: address,
    }

    /// Emitted when an admin assigns an unclaimed username to a single profile.
    /// `prior_username` is set when that profile already owned a username that was freed.
    public struct UsernameReassignedEvent has copy, drop {
        username: String,
        profile_id: address,
        admin: address,
        reason_code: u8,
        prior_username: Option<String>,
    }

    /// Emitted when a username string is reserved via [`UsernameRegistry::username_locks`]
    /// (PoC beneficiary provision or marketplace listing escrow).
    public struct UsernameReservedEvent has copy, drop {
        username: String,
        reason: u8,
        reserved_by: address,
    }

    /// Emitted when a username reservation is released (PoC claim/end, listing cancel, sale settle).
    public struct UsernameReleasedEvent has copy, drop {
        username: String,
        reason: u8,
        released_by: address,
    }

    /// X username set or cleared by an EcosystemBadgeAdminCap holder (audit trail).
    public struct ProfileXUsernameUpdatedEvent has copy, drop {
        profile_id: address,
        owner: address,
        x_username: Option<String>,
        updated_by: address,
        updated_at: u64,
    }

    /// Emitted when a username listing is created on the marketplace
    public struct UsernameListingCreatedEvent has copy, drop {
        username: String,
        seller: address,
        seller_profile_id: address,
        min_price: u64,
        created_at: u64,
    }

    /// Emitted when a username listing is cancelled
    public struct UsernameListingCancelledEvent has copy, drop {
        username: String,
        seller: address,
        seller_profile_id: address,
        cancelled_at: u64,
    }

    /// Emitted when a buyer locks MYSO on a username listing
    public struct UsernameOfferCreatedEvent has copy, drop {
        username: String,
        seller_profile_id: address,
        buyer: address,
        buyer_profile_id: address,
        amount: u64,
        created_at: u64,
    }

    /// Emitted when a seller accepts a username offer
    public struct UsernameOfferAcceptedEvent has copy, drop {
        username: String,
        replacement_username: String,
        seller: address,
        seller_profile_id: address,
        buyer: address,
        buyer_profile_id: address,
        amount: u64,
        accepted_at: u64,
    }

    /// Emitted when a username offer is rejected or revoked
    public struct UsernameOfferRejectedEvent has copy, drop {
        username: String,
        seller_profile_id: address,
        buyer: address,
        buyer_profile_id: address,
        rejected_by: address,
        amount: u64,
        rejected_at: u64,
        is_revoked: bool,
    }

    /// Emitted when registry username mappings are swapped during marketplace settlement.
    /// `prior_buyer_username` is set when the buyer's previous username was freed.
    public struct UsernameSaleSettledEvent has copy, drop {
        listed_username: String,
        replacement_username: String,
        seller: address,
        seller_profile_id: address,
        buyer: address,
        buyer_profile_id: address,
        amount: u64,
        settled_at: u64,
        prior_buyer_username: Option<String>,
    }

    /// Emitted when a fee is collected from a username marketplace sale
    public struct UsernameSaleFeeEvent has copy, drop {
        username: String,
        seller: address,
        seller_profile_id: address,
        buyer: address,
        buyer_profile_id: address,
        sale_amount: u64,
        fee_amount: u64,
        fee_recipient: address,
        timestamp: u64,
    }

    /// Copyable piece snapshot for vesting events
    public struct VestingPieceEvent has copy, drop {
        kind: u8,
        time_offset: u64,
        duration: u64,
        amount_bps: u64,
        curve_factor: u64,
    }

    /// Event emitted when MYSO tokens are vested
    public struct TokensVestedEvent has copy, drop {
        wallet_id: address,
        owner: address,
        total_amount: u64,
        start_time: u64,
        schedule_end: u64,
        pieces: vector<VestingPieceEvent>,
        vested_at: u64,
    }

    /// Event emitted when vested tokens are claimed
    public struct TokensClaimedEvent has copy, drop {
        wallet_id: address,
        owner: address,
        claimed_amount: u64,
        remaining_balance: u64,
        claimed_at: u64,
    }

    /// Event emitted when a vesting wallet is deleted
    public struct VestingWalletDeletedEvent has copy, drop {
        wallet_id: address,
        owner: address,
        deleted_at: u64,
    }

    /// Event emitted when Ecosystem Treasury address is updated
    public struct EcosystemTreasuryUpdatedEvent has copy, drop {
        updated_by: address,
        new_treasury_address: address,
        timestamp: u64,
    }
    
    /// Bootstrap initialization function - creates the username registry and treasury
    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        // Import current version from upgrade module
        let current_version = upgrade::current_version();
        
        let registry = UsernameRegistry {
            id: object::new(ctx),
            usernames: table::new(ctx),
            address_profiles: table::new(ctx),
            profile_username: table::new(ctx),
            username_locks: table::new(ctx),
            version: current_version,
        };
        
        // Create the Ecosystem treasury owned by the contract deployer
        let sender = tx_context::sender(ctx);
        let treasury = EcosystemTreasury {
            id: object::new(ctx),
            treasury_address: sender,
            version: current_version,
        };

        // Emit event so indexer can populate ecosystem_treasury table
        event::emit(EcosystemTreasuryUpdatedEvent {
            updated_by: sender,
            new_treasury_address: sender,
            timestamp: clock::timestamp_ms(clock),
        });
        
        // Share the registry to make it globally accessible
        transfer::share_object(registry);
        
        // Share the treasury to make it globally accessible
        transfer::share_object(treasury);

        let config = ProfileConfig {
            id: object::new(ctx),
            max_vesting_pieces: MAX_VESTING_PIECES,
            curve_factor_min: CURVE_FACTOR_MIN,
            curve_factor_max: CURVE_FACTOR_MAX,
            curve_precision: CURVE_PRECISION,
            min_claim_threshold_divisor: MIN_CLAIM_THRESHOLD_DIVISOR,
            min_username_length: MIN_USERNAME_LENGTH,
            max_username_length: MAX_USERNAME_LENGTH,
            username_sale_fee_bps: USERNAME_SALE_FEE_BPS,
            version: current_version,
        };
        event::emit(ProfileConfigUpdatedEvent {
            updated_by: sender,
            max_vesting_pieces: MAX_VESTING_PIECES,
            curve_factor_min: CURVE_FACTOR_MIN,
            curve_factor_max: CURVE_FACTOR_MAX,
            curve_precision: CURVE_PRECISION,
            min_claim_threshold_divisor: MIN_CLAIM_THRESHOLD_DIVISOR,
            min_username_length: MIN_USERNAME_LENGTH,
            max_username_length: MAX_USERNAME_LENGTH,
            username_sale_fee_bps: USERNAME_SALE_FEE_BPS,
            timestamp: clock::timestamp_ms(clock),
        });
        transfer::share_object(config);

        let marketplace = UsernameMarketplace {
            id: object::new(ctx),
            listings: table::new(ctx),
            version: current_version,
        };
        transfer::share_object(marketplace);
    }

    // === Username Management Functions ===

    /// Check if a name is reserved and cannot be registered
    public fun is_reserved_name(name: &String): bool {
        // Convert name string to lowercase for comparison
        let name_bytes = string::as_bytes(name);
        let lowercase_name = to_lowercase_bytes(name_bytes);
        
        // Make a local copy of RESERVED_NAMES to avoid implicit copies
        let reserved_names = RESERVED_NAMES;
        let reserved_count = vector::length(&reserved_names);
        
        let mut i = 0;
        while (i < reserved_count) {
            let reserved = *vector::borrow(&reserved_names, i);
            
            // Exact match with reserved name (case-insensitive)
            if (vector::length(&lowercase_name) == vector::length(&reserved)) {
                let mut is_match = true;
                let mut j = 0;
                while (j < vector::length(&reserved)) {
                    if (*vector::borrow(&lowercase_name, j) != *vector::borrow(&reserved, j)) {
                        is_match = false;
                        break
                    };
                    j = j + 1;
                };
                
                if (is_match) {
                    return true
                };
            };
            
            i = i + 1;
        };
        
        false
    }

    /// Convert a byte vector to lowercase
    fun to_lowercase_bytes(bytes: &vector<u8>): vector<u8> {
        let mut result = vector::empty<u8>();
        let mut i = 0;
        let len = vector::length(bytes);
        
        while (i < len) {
            let b = *vector::borrow(bytes, i);
            vector::push_back(&mut result, to_lowercase_byte(b));
            i = i + 1;
        };
        
        result
    }

    /// Convert a single ASCII byte to lowercase
    fun to_lowercase_byte(b: u8): u8 {
        if (b >= 65 && b <= 90) { // A-Z
            return b + 32 // convert to a-z
        };
        b
    }

    /// Canonical username for [`UsernameRegistry`] keys.
    /// Folds ASCII `A–Z` to `a–z` only; does not apply Unicode case folding.
    fun canonical_registry_username(username: &String): String {
        let lowered = to_lowercase_bytes(string::as_bytes(username));
        string::utf8(lowered)
    }

    /// Validate that a canonical username contains only allowed bytes:
    /// `a-z`, `0-9`, `_`, `.`. Aborts with [`EInvalidUsername`] on any other byte
    /// (rejects Unicode lookalikes, spaces, `@`, `-`, `/`, emoji, etc.).
    fun validate_username_format(username: &String) {
        let bytes = string::as_bytes(username);
        let len = vector::length(bytes);
        let mut i = 0;
        while (i < len) {
            let b = *vector::borrow(bytes, i);
            let is_lower = b >= 97 && b <= 122; // a-z
            let is_digit = b >= 48 && b <= 57;  // 0-9
            let is_underscore = b == 95;         // _
            let is_dot = b == 46;                // .
            assert!(is_lower || is_digit || is_underscore || is_dot, EInvalidUsername);
            i = i + 1;
        };
    }

    /// Canonicalize and validate a username: ASCII lowercase + charset check.
    /// Use this at every registry/marketplace entry point so `Brandon` and `brandon`
    /// collide and disallowed characters abort before any state mutation.
    fun normalize_username(raw: &String): String {
        let canonical = canonical_registry_username(raw);
        validate_username_format(&canonical);
        canonical
    }

    /// Convert an ASCII String to a String
    fun ascii_to_string(ascii_str: ascii::String): String {
        string::utf8(ascii::into_bytes(ascii_str))
    }

    fun is_ecosystem_badge(badge_id: &String): bool {
        let bytes = string::as_bytes(badge_id);
        let prefix = ECOSYSTEM_BADGE_PREFIX;
        let prefix_len = vector::length(&prefix);
        if (vector::length(bytes) < prefix_len) {
            return false
        };
        let mut i = 0;
        while (i < prefix_len) {
            if (*vector::borrow(bytes, i) != *vector::borrow(&prefix, i)) {
                return false
            };
            i = i + 1;
        };
        true
    }

    fun copy_string(s: &String): String {
        let bytes = string::as_bytes(s);
        let len = vector::length(bytes);
        let mut result = vector::empty<u8>();
        let mut i = 0;
        while (i < len) {
            vector::push_back(&mut result, *vector::borrow(bytes, i));
            i = i + 1;
        };
        string::utf8(result)
    }

    /// Canonical username for [`UsernameRegistry`] keys (package helper for PoC beneficiary flows).
    /// Lowercases ASCII `A–Z` then validates charset (`a-z`, `0-9`, `_`, `.`).
    public(package) fun canonical_registry_username_from_bytes(username: vector<u8>): String {
        let canonical = string::utf8(to_lowercase_bytes(&username));
        validate_username_format(&canonical);
        canonical
    }

    /// Lock a username string in [`UsernameRegistry::username_locks`] with `reason`.
    /// Aborts with [`EUsernameLocked`] if already reserved (mutual exclusion: PoC vs marketplace).
    fun lock_username_internal(registry: &mut UsernameRegistry, username: String, reason: u8) {
        let username = normalize_username(&username);
        assert!(
            !table::contains(&registry.username_locks, username),
            EUsernameLocked,
        );
        table::add(&mut registry.username_locks, username, reason);
    }

    /// Release a username reservation if present (idempotent).
    fun unlock_username_internal(registry: &mut UsernameRegistry, username: String) {
        let username = normalize_username(&username);
        if (table::contains(&registry.username_locks, username)) {
            table::remove(&mut registry.username_locks, username);
        };
    }

    /// Reserve a username with `reason` and emit [`UsernameReservedEvent`].
    public(package) fun lock_username(
        registry: &mut UsernameRegistry,
        username: String,
        reason: u8,
        reserved_by: address,
    ) {
        lock_username_internal(registry, username, reason);
        event::emit(UsernameReservedEvent {
            username: normalize_username(&username),
            reason,
            reserved_by,
        });
    }

    /// Release a username reservation and emit [`UsernameReleasedEvent`].
    public(package) fun unlock_username(
        registry: &mut UsernameRegistry,
        username: String,
        reason: u8,
        released_by: address,
    ) {
        unlock_username_internal(registry, username);
        event::emit(UsernameReleasedEvent {
            username: normalize_username(&username),
            reason,
            released_by,
        });
    }

    /// True when the canonical username is currently reserved in escrow.
    public fun is_username_locked(registry: &UsernameRegistry, username: String): bool {
        let username = normalize_username(&username);
        table::contains(&registry.username_locks, username)
    }

    /// Active lock reason for a canonical username, if any.
    public fun username_lock_reason(registry: &UsernameRegistry, username: String): Option<u8> {
        let username = normalize_username(&username);
        if (table::contains(&registry.username_locks, username)) {
            option::some(*table::borrow(&registry.username_locks, username))
        } else {
            option::none()
        }
    }

    /// Lock a username while an ACTIVE PoC username beneficiary provision exists.
    public(package) fun lock_username_for_beneficiary(
        registry: &mut UsernameRegistry,
        username: String,
    ) {
        lock_username_internal(registry, username, USERNAME_LOCK_BENEFICIARY);
    }

    /// Release a username beneficiary lock after claim or admin end.
    public(package) fun unlock_username_for_beneficiary(
        registry: &mut UsernameRegistry,
        username: String,
    ) {
        unlock_username_internal(registry, username);
    }

    public(package) fun assign_username(
        registry: &mut UsernameRegistry,
        username: String,
        profile_id: address,
    ) {
        assert!(!table::contains(&registry.usernames, username), EUsernameNotAvailable);
        assert!(
            !table::contains(&registry.profile_username, profile_id),
            EProfileAlreadyHasUsername,
        );
        assert!(
            !table::contains(&registry.username_locks, username),
            EUsernameLocked,
        );
        table::add(&mut registry.usernames, username, profile_id);
        table::add(&mut registry.profile_username, profile_id, username);
    }

    public(package) fun claim_username(
        registry: &mut UsernameRegistry,
        username: String,
        profile_id: address,
    ) {
        assign_username(registry, username, profile_id);
        event::emit(UsernameClaimedEvent {
            username,
            profile_id,
        });
    }

    fun claim_username_internal(registry: &mut UsernameRegistry, username: String, profile_id: address) {
        claim_username(registry, username, profile_id);
    }

    fun revoke_username(registry: &mut UsernameRegistry, username: String): address {
        assert!(table::contains(&registry.usernames, username), EUsernameNotFound);
        assert!(
            !table::contains(&registry.username_locks, username),
            EUsernameLocked,
        );
        let profile_id = table::remove(&mut registry.usernames, username);
        if (table::contains(&registry.profile_username, profile_id)) {
            table::remove(&mut registry.profile_username, profile_id);
        };
        profile_id
    }

    fun move_username(
        registry: &mut UsernameRegistry,
        username: String,
        to_profile_id: address,
    ): address {
        assert!(table::contains(&registry.usernames, username), EUsernameNotFound);
        let from_profile_id = *table::borrow(&registry.usernames, username);
        assert!(from_profile_id != to_profile_id, EUsernameProfileMismatch);
        *table::borrow_mut(&mut registry.usernames, username) = to_profile_id;
        if (table::contains(&registry.profile_username, from_profile_id)) {
            table::remove(&mut registry.profile_username, from_profile_id);
        };
        table::add(&mut registry.profile_username, to_profile_id, username);
        from_profile_id
    }

    fun profile_picture_event_string(profile: &Profile): Option<String> {
        if (option::is_some(&profile.profile_picture)) {
            let url = option::borrow(&profile.profile_picture);
            option::some(ascii_to_string(url::inner_url(url)))
        } else {
            option::none()
        }
    }

    fun cover_photo_event_string(profile: &Profile): Option<String> {
        if (option::is_some(&profile.cover_photo)) {
            let url = option::borrow(&profile.cover_photo);
            option::some(ascii_to_string(url::inner_url(url)))
        } else {
            option::none()
        }
    }

    fun apply_optional_string_update(field: &mut Option<String>, update: Option<String>) {
        if (option::is_some(&update)) {
            let value = option::destroy_some(update);
            if (string::length(&value) == 0) {
                *field = option::none();
            } else {
                *field = option::some(value);
            };
        };
    }

    fun emit_profile_updated_event(profile: &Profile, clock: &Clock, _ctx: &TxContext) {
        event::emit(ProfileUpdatedEvent {
            profile_id: object::uid_to_address(&profile.id),
            display_name: profile.display_name,
            bio: profile.bio,
            profile_picture: profile_picture_event_string(profile),
            cover_photo: cover_photo_event_string(profile),
            owner: profile.owner,
            updated_at: clock::timestamp_ms(clock),
            x_username: profile.x_username,
            website: profile.website,
            birthdate: profile.birthdate,
            location: profile.location,
        });
    }

    // === Profile Creation and Management ===

    /// Create a new profile with a required username
    /// Main entry: also creates a linked [`memory::MemoryAccount`] shared object.
    public entry fun create_profile(
        registry: &mut UsernameRegistry,
        config: &ProfileConfig,
        memory_registry: &mut memory::MemoryRegistry,
        ai_credit_config: &mut AiCreditConfig,
        display_name: String,
        username: String,
        bio: String,
        profile_picture_url: vector<u8>,
        cover_photo_url: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), 1);
        
        let owner = tx_context::sender(ctx);
        let now = clock::timestamp_ms(clock);

        // Check that the sender doesn't already have a profile
        assert!(!table::contains(&registry.address_profiles, owner), EProfileAlreadyExists);

        let username = normalize_username(&username);
        
        // Validate the username
        let username_bytes = string::as_bytes(&username);
        let username_length = vector::length(username_bytes);
        
        assert!(
            username_length >= config.min_username_length && username_length <= config.max_username_length,
            EInvalidUsername,
        );
        
        // Check if username is reserved in the hard coded list
        assert!(!is_reserved_name(&username), EReservedName);
        
        // Check that the username isn't already registered
        assert!(!table::contains(&registry.usernames, username), EUsernameNotAvailable);
        assert!(!table::contains(&registry.username_locks, username), EUsernameLocked);
        
        // Create the profile object
        let profile_picture = if (vector::length(&profile_picture_url) > 0) {
            option::some(url::new_unsafe_from_bytes(profile_picture_url))
        } else {
            option::none()
        };
        
        let cover_photo = if (vector::length(&cover_photo_url) > 0) {
            option::some(url::new_unsafe_from_bytes(cover_photo_url))
        } else {
            option::none()
        };
        
        let display_name_option = if (string::length(&display_name) > 0) {
            option::some(display_name)
        } else {
            option::none()
        };
        
        let mut profile = Profile {
            id: object::new(ctx),
            display_name: display_name_option,
            bio,
            profile_picture,
            cover_photo,
            created_at: now,
            owner,
            x_username: option::none(),
            website: option::none(),
            birthdate: option::none(),
            location: option::none(),
            badges: vector::empty<ProfileBadge>(),
            selected_badge_id: option::none(),
            selected_ecosystem_badge_id: option::none(),
            memory_account_id: option::none(),
            ai_credit_balance_id: option::none(),
            version: upgrade::current_version(),
        };
        
        let profile_id = object::uid_to_address(&profile.id);
        let memory_id = memory::create_account_for_profile(memory_registry, profile_id, clock, ctx);
        profile.memory_account_id = option::some(memory_id);
        let balance_id = ai_credit::create_and_share_balance(
            ai_credit_config,
            memory_id,
            owner,
            profile_id,
            clock,
            ctx,
        );
        profile.ai_credit_balance_id = option::some(balance_id);
        
        claim_username_internal(registry, username, profile_id);
        table::add(&mut registry.address_profiles, owner, profile_id);
        
        // Extract display name value for the event (if available)
        let display_name_value = if (option::is_some(&profile.display_name)) {
            let name_copy = *option::borrow(&profile.display_name);
            name_copy
        } else {
            string::utf8(b"")
        };
        
        // Emit profile creation event
        event::emit(ProfileCreatedEvent {
            profile_id,
            display_name: display_name_value,
            bio: profile.bio,
            profile_picture: profile_picture_event_string(&profile),
            cover_photo: cover_photo_event_string(&profile),
            owner,
            created_at: now,
        });

        // Transfer profile to owner
        transfer::transfer(profile, owner);
    }

    /// Create a profile from an oracle-verified PoC username beneficiary claim.
    public(package) fun create_profile_from_beneficiary_claim(
        registry: &mut UsernameRegistry,
        config: &ProfileConfig,
        memory_registry: &mut memory::MemoryRegistry,
        ai_credit_config: &mut AiCreditConfig,
        display_name: vector<u8>,
        username: String,
        bio: vector<u8>,
        profile_picture_url: vector<u8>,
        cover_photo_url: vector<u8>,
        owner: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ): address {
        assert!(registry.version == upgrade::current_version(), 1);

        let username = normalize_username(&username);
        let username_length = vector::length(string::as_bytes(&username));
        assert!(
            username_length >= config.min_username_length && username_length <= config.max_username_length,
            EInvalidUsername,
        );
        assert!(!is_reserved_name(&username), EReservedName);
        assert!(!table::contains(&registry.usernames, username), EUsernameNotAvailable);
        assert!(!table::contains(&registry.address_profiles, owner), EProfileAlreadyExists);

        let now = clock::timestamp_ms(clock);
        let display_name_str = string::utf8(display_name);
        let bio_str = string::utf8(bio);

        let profile_picture = if (vector::length(&profile_picture_url) > 0) {
            option::some(url::new_unsafe_from_bytes(profile_picture_url))
        } else {
            option::none()
        };
        let cover_photo = if (vector::length(&cover_photo_url) > 0) {
            option::some(url::new_unsafe_from_bytes(cover_photo_url))
        } else {
            option::none()
        };
        let display_name_option = if (string::length(&display_name_str) > 0) {
            option::some(display_name_str)
        } else {
            option::none()
        };

        let mut profile = Profile {
            id: object::new(ctx),
            display_name: display_name_option,
            bio: bio_str,
            profile_picture,
            cover_photo,
            created_at: now,
            owner,
            x_username: option::none(),
            website: option::none(),
            birthdate: option::none(),
            location: option::none(),
            badges: vector::empty<ProfileBadge>(),
            selected_badge_id: option::none(),
            selected_ecosystem_badge_id: option::none(),
            memory_account_id: option::none(),
            ai_credit_balance_id: option::none(),
            version: upgrade::current_version(),
        };

        let profile_id = object::uid_to_address(&profile.id);
        let memory_id = memory::create_account_for_profile(memory_registry, profile_id, clock, ctx);
        profile.memory_account_id = option::some(memory_id);
        let balance_id = ai_credit::create_and_share_balance(
            ai_credit_config,
            memory_id,
            owner,
            profile_id,
            clock,
            ctx,
        );
        profile.ai_credit_balance_id = option::some(balance_id);

        // Release the PoC beneficiary reservation before claiming so `claim_username`
        // does not abort on the lock assertion (1 username per profile enforced inside).
        unlock_username_for_beneficiary(registry, username);
        claim_username(registry, username, profile_id);
        table::add(&mut registry.address_profiles, owner, profile_id);

        let display_name_value = if (option::is_some(&profile.display_name)) {
            *option::borrow(&profile.display_name)
        } else {
            string::utf8(b"")
        };

        event::emit(ProfileCreatedEvent {
            profile_id,
            display_name: display_name_value,
            bio: profile.bio,
            profile_picture: profile_picture_event_string(&profile),
            cover_photo: cover_photo_event_string(&profile),
            owner,
            created_at: now,
        });

        transfer::share_object(profile);
        profile_id
    }

    /// Backfill a Memory account for profiles created before Memory integration, or test-only paths.
    /// Transfers the same `Profile` back to the caller.
    public entry fun ensure_memory_account(
        memory_registry: &mut memory::MemoryRegistry,
        mut profile: Profile,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let sender = tx_context::sender(ctx);
        assert!(profile.owner == sender, EUnauthorized);
        assert!(profile.version == upgrade::current_version(), 1);
        assert!(option::is_none(&profile.memory_account_id), EMemoryAlreadyLinked);

        let profile_id = object::uid_to_address(&profile.id);
        let mem_id = memory::create_account_for_profile(memory_registry, profile_id, clock, ctx);
        profile.memory_account_id = option::some(mem_id);
        transfer::transfer(profile, sender);
    }

    /// Only the profile owner can update profile information
    public entry fun update_profile(
        profile: &mut Profile,
        // Basic profile fields
        new_display_name: String,
        new_bio: String,
        new_profile_picture_url: vector<u8>,
        new_cover_photo_url: vector<u8>,
        new_website: Option<String>,
        new_birthdate: Option<String>,
        new_location: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Verify sender is the owner
        assert!(profile.owner == tx_context::sender(ctx), EUnauthorized);

        // Update basic profile information
        // Set display name if provided, otherwise keep existing
        if (string::length(&new_display_name) > 0) {
            profile.display_name = option::some(new_display_name);
        };
        
        profile.bio = new_bio;
        
        if (vector::length(&new_profile_picture_url) > 0) {
            profile.profile_picture = option::some(url::new_unsafe_from_bytes(new_profile_picture_url));
        };
        
        if (vector::length(&new_cover_photo_url) > 0) {
            profile.cover_photo = option::some(url::new_unsafe_from_bytes(new_cover_photo_url));
        };

        apply_optional_string_update(&mut profile.website, new_website);
        apply_optional_string_update(&mut profile.birthdate, new_birthdate);
        apply_optional_string_update(&mut profile.location, new_location);

        emit_profile_updated_event(profile, clock, ctx);
    }
    
    // === Accessor functions ===

    /// Get the display name of a profile
    public fun display_name(profile: &Profile): Option<String> {
        profile.display_name
    }

    /// Get the bio of a profile
    public fun bio(profile: &Profile): String {
        profile.bio
    }

    /// Get the profile picture URL of a profile
    public fun profile_picture(profile: &Profile): &Option<Url> {
        &profile.profile_picture
    }
    
    /// Get the cover photo URL of a profile
    public fun cover_photo(profile: &Profile): &Option<Url> {
        &profile.cover_photo
    }

    /// Get the owner of a profile
    public fun owner(profile: &Profile): address {
        profile.owner
    }

    /// Get the profile website (optional)
    public fun website(profile: &Profile): Option<String> {
        profile.website
    }

    /// Get the profile birthdate (optional)
    public fun birthdate(profile: &Profile): Option<String> {
        profile.birthdate
    }

    /// Get the profile location (optional)
    public fun location(profile: &Profile): Option<String> {
        profile.location
    }

    /// Get the ID of a profile
    public fun id(profile: &Profile): &UID {
        &profile.id
    }

    /// Lookup profile ID by username in the registry
    public fun lookup_profile_by_username(registry: &UsernameRegistry, username: String): Option<address> {
        let username = normalize_username(&username);
        if (table::contains(&registry.usernames, username)) {
            option::some(*table::borrow(&registry.usernames, username))
        } else {
            option::none()
        }
    }

    /// True when the canonical username is currently reserved in escrow (PoC or marketplace).
    public fun is_username_beneficiary_locked(registry: &UsernameRegistry, username: &String): bool {
        let canonical = normalize_username(username);
        table::contains(&registry.username_locks, canonical)
    }

    public fun is_username_available(registry: &UsernameRegistry, username: String): bool {
        let username = normalize_username(&username);
        !table::contains(&registry.usernames, username)
            && !table::contains(&registry.username_locks, username)
    }
    
    /// Lookup profile ID by owner address
    public fun lookup_profile_by_owner(registry: &UsernameRegistry, owner: address): Option<address> {
        if (table::contains(&registry.address_profiles, owner)) {
            option::some(*table::borrow(&registry.address_profiles, owner))
        } else {
            option::none()
        }
    }
    
    /// Get the ID address of a profile
    public fun get_id_address(profile: &Profile): address {
        object::uid_to_address(&profile.id)
    }

    /// Get the owner of a profile
    public fun get_owner(profile: &Profile): address {
        profile.owner
    }

    // === Username Marketplace ===

    fun remove_offeror_from_vector(offerors: &mut vector<address>, buyer: address) {
        let mut i = 0;
        let len = vector::length(offerors);
        while (i < len) {
            if (*vector::borrow(offerors, i) == buyer) {
                vector::remove(offerors, i);
                return
            };
            i = i + 1;
        };
    }

    fun refund_username_offer(offer: UsernameOffer, ctx: &mut TxContext) {
        let UsernameOffer { buyer, buyer_profile_id: _, amount: _, created_at: _, locked_myso } = offer;
        let refund = coin::from_balance(locked_myso, ctx);
        transfer::public_transfer(refund, buyer);
    }

    fun refund_all_offers_except(
        offerors: &vector<address>,
        offers: &mut Table<address, UsernameOffer>,
        except_buyer: address,
        ctx: &mut TxContext,
    ) {
        let mut i = 0;
        let len = vector::length(offerors);
        while (i < len) {
            let buyer = *vector::borrow(offerors, i);
            if (buyer != except_buyer && table::contains(offers, buyer)) {
                let offer = table::remove(offers, buyer);
                refund_username_offer(offer, ctx);
            };
            i = i + 1;
        };
    }

    fun destroy_username_listing(listing: UsernameListing) {
        let UsernameListing {
            seller: _,
            seller_profile_id: _,
            username: _,
            min_price: _,
            created_at: _,
            offerors: _,
            offers,
        } = listing;
        table::destroy_empty(offers);
    }

    fun execute_username_sale(
        registry: &mut UsernameRegistry,
        listed_username: String,
        replacement_username: String,
        seller_profile_id: address,
        buyer_profile_id: address,
        seller: address,
        buyer: address,
        amount: u64,
        now: u64,
    ) {
        assert!(table::contains(&registry.usernames, listed_username), EUsernameNotFound);
        assert!(
            *table::borrow(&registry.usernames, listed_username) == seller_profile_id,
            EUsernameProfileMismatch,
        );
        assert!(!table::contains(&registry.usernames, replacement_username), EUsernameNotAvailable);
        assert!(
            !table::contains(&registry.username_locks, replacement_username),
            EUsernameLocked,
        );

        // 1. Free buyer's prior username so the buyer ends with exactly one username
        //    (one-per-wallet invariant). The buyer's prior username is never the listed
        //    (marketplace-locked) string, so `revoke_username` does not trip the lock assert.
        //    Freed username is carried on UsernameSaleSettledEvent for indexer registry delete.
        let prior_buyer_username = if (table::contains(&registry.profile_username, buyer_profile_id)) {
            let old_buyer_username = *table::borrow(&registry.profile_username, buyer_profile_id);
            revoke_username(registry, old_buyer_username);
            option::some(old_buyer_username)
        } else {
            option::none()
        };

        // 2. Move the listed username to the buyer.
        move_username(registry, listed_username, buyer_profile_id);

        // 3. Assign the replacement for the seller (seller has zero usernames after step 2).
        //    Silent registry update: UsernameSaleSettledEvent is the audit record.
        assign_username(registry, replacement_username, seller_profile_id);

        // 4. Release the marketplace reservation on the listed username.
        unlock_username(registry, listed_username, USERNAME_LOCK_MARKETPLACE, seller);

        event::emit(UsernameSaleSettledEvent {
            listed_username,
            replacement_username,
            seller,
            seller_profile_id,
            buyer,
            buyer_profile_id,
            amount,
            settled_at: now,
            prior_buyer_username,
        });
    }

    /// List a username for sale on the marketplace
    public entry fun create_username_listing(
        marketplace: &mut UsernameMarketplace,
        registry: &mut UsernameRegistry,
        profile: &Profile,
        username: String,
        min_price: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(marketplace.version == upgrade::current_version(), 1);
        let sender = tx_context::sender(ctx);
        assert!(profile.owner == sender, EUnauthorized);
        assert!(min_price > 0, EInsufficientTokens);

        let username = normalize_username(&username);
        let seller_profile_id = object::uid_to_address(&profile.id);
        assert!(table::contains(&registry.usernames, username), EUsernameNotFound);
        assert!(
            *table::borrow(&registry.usernames, username) == seller_profile_id,
            EUsernameProfileMismatch,
        );
        assert!(!table::contains(&marketplace.listings, username), EListingAlreadyExists);

        // Reserve the username in the registry so no second claim, PoC provision, or admin
        // revoke/reassign can mutate it while listed. Aborts with EUsernameLocked if already held.
        lock_username(registry, username, USERNAME_LOCK_MARKETPLACE, sender);

        let listing = UsernameListing {
            seller: sender,
            seller_profile_id,
            username,
            min_price,
            created_at: clock::timestamp_ms(clock),
            offerors: vector::empty(),
            offers: table::new(ctx),
        };
        table::add(&mut marketplace.listings, username, listing);

        event::emit(UsernameListingCreatedEvent {
            username,
            seller: sender,
            seller_profile_id,
            min_price,
            created_at: clock::timestamp_ms(clock),
        });
    }

    /// Cancel a username listing when there are no pending offers
    public entry fun cancel_username_listing(
        marketplace: &mut UsernameMarketplace,
        registry: &mut UsernameRegistry,
        profile: &Profile,
        username: String,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(marketplace.version == upgrade::current_version(), 1);
        let sender = tx_context::sender(ctx);
        assert!(profile.owner == sender, EUnauthorized);

        let username = normalize_username(&username);
        assert!(table::contains(&marketplace.listings, username), EListingNotFound);
        let listing = table::borrow(&marketplace.listings, username);
        assert!(listing.seller_profile_id == object::uid_to_address(&profile.id), EUnauthorized);
        assert!(vector::length(&listing.offerors) == 0, EListingHasOffers);

        let listing = table::remove(&mut marketplace.listings, username);
        let seller_profile_id = listing.seller_profile_id;
        destroy_username_listing(listing);

        // Release the marketplace reservation so the username can be claimed/listed again.
        unlock_username(registry, username, USERNAME_LOCK_MARKETPLACE, sender);

        event::emit(UsernameListingCancelledEvent {
            username,
            seller: sender,
            seller_profile_id,
            cancelled_at: clock::timestamp_ms(clock),
        });
    }

    /// Lock MYSO to bid on a listed username
    public entry fun create_username_offer(
        marketplace: &mut UsernameMarketplace,
        registry: &UsernameRegistry,
        username: String,
        coin: &mut Coin<MYSO>,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(marketplace.version == upgrade::current_version(), 1);
        let buyer = tx_context::sender(ctx);
        assert!(coin::value(coin) >= amount && amount > 0, EInsufficientTokens);
        assert!(table::contains(&registry.address_profiles, buyer), EBuyerHasNoProfile);

        let username = normalize_username(&username);
        assert!(table::contains(&marketplace.listings, username), EListingNotFound);
        let listing = table::borrow(&marketplace.listings, username);
        assert!(buyer != listing.seller, ECannotOfferOwnProfile);
        assert!(amount >= listing.min_price, EOfferBelowMinimum);
        assert!(!table::contains(&listing.offers, buyer), EOfferAlreadyExists);

        let buyer_profile_id = *table::borrow(&registry.address_profiles, buyer);
        let seller_profile_id = listing.seller_profile_id;
        let now = clock::timestamp_ms(clock);

        let offer_coin = coin::split(coin, amount, ctx);
        let locked_myso = coin::into_balance(offer_coin);
        let offer = UsernameOffer {
            buyer,
            buyer_profile_id,
            amount,
            created_at: now,
            locked_myso,
        };

        let listing = table::borrow_mut(&mut marketplace.listings, username);
        vector::push_back(&mut listing.offerors, buyer);
        table::add(&mut listing.offers, buyer, offer);

        event::emit(UsernameOfferCreatedEvent {
            username,
            seller_profile_id,
            buyer,
            buyer_profile_id,
            amount,
            created_at: now,
        });
    }

    /// Accept a buyer offer and atomically swap username registry mappings
    public entry fun accept_username_offer(
        marketplace: &mut UsernameMarketplace,
        registry: &mut UsernameRegistry,
        profile: &Profile,
        username: String,
        buyer: address,
        replacement_username: String,
        config: &ProfileConfig,
        treasury: &EcosystemTreasury,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(marketplace.version == upgrade::current_version(), 1);
        let sender = tx_context::sender(ctx);
        assert!(profile.owner == sender, EUnauthorized);

        let username = normalize_username(&username);
        let replacement_username = normalize_username(&replacement_username);
        assert!(table::contains(&marketplace.listings, username), EListingNotFound);

        let seller_profile_id;
        let seller;
        let buyer_profile_id;
        let amount;
        {
            let listing = table::borrow(&marketplace.listings, username);
            assert!(listing.seller_profile_id == object::uid_to_address(&profile.id), EUnauthorized);
            assert!(table::contains(&listing.offers, buyer), EOfferDoesNotExist);
            seller_profile_id = listing.seller_profile_id;
            seller = listing.seller;
            let offer = table::borrow(&listing.offers, buyer);
            buyer_profile_id = offer.buyer_profile_id;
            amount = offer.amount;
        };
        let now = clock::timestamp_ms(clock);

        execute_username_sale(
            registry,
            username,
            replacement_username,
            seller_profile_id,
            buyer_profile_id,
            seller,
            buyer,
            amount,
            now,
        );

        let UsernameListing {
            seller: _,
            seller_profile_id: _,
            username: listed_username,
            min_price: _,
            created_at: _,
            offerors,
            mut offers,
        } = table::remove(&mut marketplace.listings, username);

        let UsernameOffer { buyer: _, buyer_profile_id: _, amount, created_at: _, locked_myso } =
            table::remove(&mut offers, buyer);

        let fee_amount = (amount * config.username_sale_fee_bps) / BPS_DENOMINATOR;
        let mut payment = coin::from_balance(locked_myso, ctx);
        let fee_payment = coin::split(&mut payment, fee_amount, ctx);
        transfer::public_transfer(fee_payment, get_treasury_address(treasury));
        transfer::public_transfer(payment, seller);

        refund_all_offers_except(&offerors, &mut offers, buyer, ctx);
        table::destroy_empty(offers);

        event::emit(UsernameOfferAcceptedEvent {
            username: listed_username,
            replacement_username,
            seller,
            seller_profile_id,
            buyer,
            buyer_profile_id,
            amount,
            accepted_at: now,
        });

        event::emit(UsernameSaleFeeEvent {
            username: listed_username,
            seller,
            seller_profile_id,
            buyer,
            buyer_profile_id,
            sale_amount: amount,
            fee_amount,
            fee_recipient: get_treasury_address(treasury),
            timestamp: now,
        });
    }

    /// Reject or revoke a username marketplace offer
    public entry fun reject_or_revoke_username_offer(
        marketplace: &mut UsernameMarketplace,
        profile: &Profile,
        username: String,
        buyer: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(marketplace.version == upgrade::current_version(), 1);
        let sender = tx_context::sender(ctx);
        let username = normalize_username(&username);
        assert!(table::contains(&marketplace.listings, username), EListingNotFound);

        let listing = table::borrow_mut(&mut marketplace.listings, username);
        let seller_profile_id = listing.seller_profile_id;
        if (sender != buyer) {
            assert!(profile.owner == sender, EUnauthorizedOfferAction);
            assert!(seller_profile_id == object::uid_to_address(&profile.id), EUnauthorizedOfferAction);
        } else {
            assert!(table::contains(&listing.offers, buyer), EOfferDoesNotExist);
        };
        let offer = table::remove(&mut listing.offers, buyer);
        remove_offeror_from_vector(&mut listing.offerors, buyer);
        let buyer_profile_id = offer.buyer_profile_id;
        let amount = offer.amount;
        refund_username_offer(offer, ctx);

        let is_revoked = buyer == sender;
        event::emit(UsernameOfferRejectedEvent {
            username,
            seller_profile_id,
            buyer,
            buyer_profile_id,
            rejected_by: sender,
            amount,
            rejected_at: clock::timestamp_ms(clock),
            is_revoked,
        });
    }

    /// Check if a username listing has an offer from a specific buyer
    public fun has_username_offer_from(
        marketplace: &UsernameMarketplace,
        username: String,
        buyer: address,
    ): bool {
        let username = normalize_username(&username);
        if (!table::contains(&marketplace.listings, username)) {
            return false
        };
        let listing = table::borrow(&marketplace.listings, username);
        table::contains(&listing.offers, buyer)
    }

    /// Check if a username has any active marketplace offers
    public fun has_username_offers(
        marketplace: &UsernameMarketplace,
        username: String,
    ): bool {
        let username = normalize_username(&username);
        if (!table::contains(&marketplace.listings, username)) {
            return false
        };
        let listing = table::borrow(&marketplace.listings, username);
        vector::length(&listing.offerors) > 0
    }

    /// Check if a username is actively listed on the marketplace
    public fun is_username_listed(
        marketplace: &UsernameMarketplace,
        username: String,
    ): bool {
        let username = normalize_username(&username);
        table::contains(&marketplace.listings, username)
    }

    /// Minimum price for a listed username
    public fun listing_min_price(
        marketplace: &UsernameMarketplace,
        username: String,
    ): Option<u64> {
        let username = normalize_username(&username);
        if (table::contains(&marketplace.listings, username)) {
            option::some(table::borrow(&marketplace.listings, username).min_price)
        } else {
            option::none()
        }
    }

    /// Get the treasury address from the EcosystemTreasury
    public fun get_treasury_address(treasury: &EcosystemTreasury): address {
        treasury.treasury_address
    }

    /// Update Ecosystem Treasury address (admin only)
    public entry fun update_treasury_address(
        _: &EcosystemTreasuryAdminCap,
        treasury: &mut EcosystemTreasury,
        new_address: address,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        treasury.treasury_address = new_address;

        // Emit event for treasury address update
        event::emit(EcosystemTreasuryUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            new_treasury_address: new_address,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Update Ecosystem Treasury address (admin only).
    public entry fun update_ecosystem_treasury_config(
        admin_cap: &EcosystemTreasuryAdminCap,
        treasury: &mut EcosystemTreasury,
        new_address: address,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        update_treasury_address(admin_cap, treasury, new_address, clock, ctx);
    }

    /// Read the configured username marketplace sale fee (bps).
    public fun username_sale_fee_bps(config: &ProfileConfig): u64 {
        config.username_sale_fee_bps
    }

    /// Get the version of the EcosystemTreasury
    public fun treasury_version(treasury: &EcosystemTreasury): u64 {
        treasury.version
    }

    /// Migration function for EcosystemTreasury
    public entry fun migrate_ecosystem_treasury(
        treasury: &mut EcosystemTreasury,
        _: &upgrade::UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade
        assert!(treasury.version < current_version, 1);
        
        // Remember old version and update to new version
        let old_version = treasury.version;
        treasury.version = current_version;
        
        // Emit event for object migration
        let treasury_id = object::id(treasury);
        upgrade::emit_migration_event(
            treasury_id,
            string::utf8(b"EcosystemTreasury"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Create a ProfileAdminCap for bootstrap (package visibility only)
    public(package) fun create_profile_admin_cap(ctx: &mut TxContext): ProfileAdminCap {
        ProfileAdminCap {
            id: object::new(ctx),
        }
    }

    /// Update profile configuration (admin only)
    public entry fun update_profile_config(
        _: &ProfileAdminCap,
        config: &mut ProfileConfig,
        max_vesting_pieces: u64,
        curve_factor_min: u64,
        curve_factor_max: u64,
        curve_precision: u64,
        min_claim_threshold_divisor: u64,
        min_username_length: u64,
        max_username_length: u64,
        username_sale_fee_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(max_vesting_pieces > 0, EInvalidConfig);
        assert!(curve_precision > 0, EInvalidConfig);
        assert!(curve_factor_min > 0 && curve_factor_max >= curve_factor_min, EInvalidConfig);
        assert!(min_claim_threshold_divisor > 0, EInvalidConfig);
        assert!(min_username_length > 0 && max_username_length >= min_username_length, EInvalidConfig);
        assert!(username_sale_fee_bps <= 10000, EInvalidConfig);

        config.max_vesting_pieces = max_vesting_pieces;
        config.curve_factor_min = curve_factor_min;
        config.curve_factor_max = curve_factor_max;
        config.curve_precision = curve_precision;
        config.min_claim_threshold_divisor = min_claim_threshold_divisor;
        config.min_username_length = min_username_length;
        config.max_username_length = max_username_length;
        config.username_sale_fee_bps = username_sale_fee_bps;

        event::emit(ProfileConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            max_vesting_pieces,
            curve_factor_min,
            curve_factor_max,
            curve_precision,
            min_claim_threshold_divisor,
            min_username_length,
            max_username_length,
            username_sale_fee_bps,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Migration function for ProfileConfig — copies the username sale fee from the pre-upgrade value.
    public entry fun migrate_profile_config(
        config: &mut ProfileConfig,
        username_sale_fee_bps: u64,
        _: &upgrade::UpgradeAdminCap,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let current_version = upgrade::current_version();
        assert!(config.version < current_version, EInvalidConfig);
        assert!(username_sale_fee_bps <= 10000, EInvalidConfig);

        let old_version = config.version;
        config.username_sale_fee_bps = username_sale_fee_bps;
        config.version = current_version;

        upgrade::emit_migration_event(
            object::id(config),
            string::utf8(b"ProfileConfig"),
            old_version,
            tx_context::sender(ctx),
        );

        event::emit(ProfileConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            max_vesting_pieces: config.max_vesting_pieces,
            curve_factor_min: config.curve_factor_min,
            curve_factor_max: config.curve_factor_max,
            curve_precision: config.curve_precision,
            min_claim_threshold_divisor: config.min_claim_threshold_divisor,
            min_username_length: config.min_username_length,
            max_username_length: config.max_username_length,
            username_sale_fee_bps: config.username_sale_fee_bps,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Migration function for Profile — initializes on-chain website/birthdate/location fields.
    public entry fun migrate_profile(
        profile: &mut Profile,
        _: &upgrade::UpgradeAdminCap,
        ctx: &mut TxContext,
    ) {
        let current_version = upgrade::current_version();
        assert!(profile.version < current_version, 1);

        let old_version = profile.version;
        profile.website = option::none();
        profile.birthdate = option::none();
        profile.location = option::none();
        profile.version = current_version;

        upgrade::emit_migration_event(
            object::id(profile),
            string::utf8(b"Profile"),
            old_version,
            tx_context::sender(ctx),
        );
    }

    /// Create an EcosystemTreasuryAdminCap for bootstrap (package visibility only)
    /// This function is only callable by other modules in the same package
    public(package) fun create_ecosystem_treasury_admin_cap(ctx: &mut TxContext): EcosystemTreasuryAdminCap {
        EcosystemTreasuryAdminCap {
            id: object::new(ctx)
        }
    }

    /// Create an EcosystemBadgeAdminCap for bootstrap (package visibility only)
    public(package) fun create_ecosystem_badge_admin_cap(ctx: &mut TxContext): EcosystemBadgeAdminCap {
        EcosystemBadgeAdminCap {
            id: object::new(ctx)
        }
    }

    /// Create a UsernameAdminCap for bootstrap (package visibility only)
    public(package) fun create_username_admin_cap(ctx: &mut TxContext): UsernameAdminCap {
        UsernameAdminCap {
            id: object::new(ctx)
        }
    }

    /// Assign an ecosystem badge to a profile - called by EcosystemBadgeAdminCap holder
    public entry fun assign_ecosystem_badge(
        _: &EcosystemBadgeAdminCap,
        profile: &mut Profile,
        badge_name: String,
        badge_description: String,
        badge_media_url: String,
        badge_icon_url: String,
        badge_type: u8,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(badge_type >= 1 && badge_type <= 100, EInvalidBadgeType);
        assert!(string::length(&badge_name) > 0 && string::length(&badge_name) <= MAX_BADGE_NAME_LENGTH, EBadgeNameTooLong);
        assert!(string::length(&badge_description) <= MAX_BADGE_DESCRIPTION_LENGTH, EBadgeDescriptionTooLong);
        assert!(string::length(&badge_media_url) > 0 && string::length(&badge_media_url) <= MAX_BADGE_MEDIA_URL_LENGTH, EBadgeMediaUrlTooLong);
        assert!(string::length(&badge_icon_url) > 0 && string::length(&badge_icon_url) <= MAX_BADGE_ICON_URL_LENGTH, EBadgeIconUrlTooLong);

        let issuer = tx_context::sender(ctx);
        let now = clock::timestamp_ms(clock);

        let badge_name_for_id = copy_string(&badge_name);
        let mut badge_id = string::utf8(ECOSYSTEM_BADGE_PREFIX);
        let issuer_str = address::to_string(issuer);
        string::append(&mut badge_id, issuer_str);
        string::append(&mut badge_id, string::utf8(b"_"));
        string::append(&mut badge_id, badge_name_for_id);

        let badge_id_for_select = copy_string(&badge_id);

        add_badge_to_profile(
            profile,
            badge_id,
            badge_name,
            badge_description,
            badge_media_url,
            badge_icon_url,
            issuer,
            now,
            issuer,
            badge_type
        );

        if (option::is_none(&profile.selected_ecosystem_badge_id)) {
            profile.selected_ecosystem_badge_id = option::some(badge_id_for_select);
        };
    }

    /// Set or clear a profile X username — only callable by an EcosystemBadgeAdminCap holder.
    public entry fun admin_set_profile_x_username(
        _: &EcosystemBadgeAdminCap,
        profile: &mut Profile,
        new_x_username: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        profile.x_username = new_x_username;
        let now = clock::timestamp_ms(clock);
        event::emit(ProfileXUsernameUpdatedEvent {
            profile_id: object::uid_to_address(&profile.id),
            owner: profile.owner,
            x_username: profile.x_username,
            updated_by: tx_context::sender(ctx),
            updated_at: now,
        });
    }

    /// Revoke an ecosystem badge from a profile - called by EcosystemBadgeAdminCap holder
    public entry fun revoke_ecosystem_badge(
        _: &EcosystemBadgeAdminCap,
        profile: &mut Profile,
        badge_id: String,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let revoker = tx_context::sender(ctx);
        let now = clock::timestamp_ms(clock);
        remove_badge_from_profile(profile, &badge_id, revoker, revoker, now);
    }

    // === Username Admin (registry-only; no Profile object required) ===

    /// Assign an unclaimed username to one profile. If that profile already owns a username,
    /// it is freed for reuse and reported via `UsernameReassignedEvent.prior_username`.
    /// No other profile is modified.
    public entry fun admin_reassign_username(
        _: &UsernameAdminCap,
        registry: &mut UsernameRegistry,
        profile_id: address,
        new_username: String,
        reason_code: u8,
        ctx: &mut TxContext,
    ) {
        assert!(registry.version == upgrade::current_version(), 1);
        let canonical = normalize_username(&new_username);
        assert!(!table::contains(&registry.usernames, canonical), EUsernameNotAvailable);
        assert!(
            !table::contains(&registry.username_locks, canonical),
            EUsernameLocked,
        );

        let admin = tx_context::sender(ctx);
        // Free the target's prior name first (abort if marketplace-locked).
        let prior_username = if (table::contains(&registry.profile_username, profile_id)) {
            let old_username = *table::borrow(&registry.profile_username, profile_id);
            revoke_username(registry, old_username);
            option::some(old_username)
        } else {
            option::none()
        };
        assign_username(registry, canonical, profile_id);
        event::emit(UsernameReassignedEvent {
            username: canonical,
            profile_id,
            admin,
            reason_code,
            prior_username,
        });
    }

    // Accessor for version field
    public fun version(registry: &UsernameRegistry): u64 {
        registry.version
    }

    // Mutable accessor for version field (only for upgrade module)
    public(package) fun borrow_version_mut(registry: &mut UsernameRegistry): &mut u64 {
        &mut registry.version
    }

    /// Migration function for the registry
    public entry fun migrate_registry(
        registry: &mut UsernameRegistry,
        _: &upgrade::UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(registry.version < current_version, 1);
        
        // Remember old version and update to new version
        let old_version = registry.version;
        registry.version = current_version;
        
        // Emit event for object migration
        let registry_id = object::id(registry);
        upgrade::emit_migration_event(
            registry_id,
            string::utf8(b"UsernameRegistry"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    #[test_only]
    /// Initialize test environment for profile module
    public fun test_init(clock: &Clock, ctx: &mut TxContext) {
        let registry = UsernameRegistry {
            id: object::new(ctx),
            usernames: table::new(ctx),
            address_profiles: table::new(ctx),
            profile_username: table::new(ctx),
            username_locks: table::new(ctx),
            version: 1,
        };
        
        transfer::share_object(registry);

        memory::bootstrap_init(clock, ctx);
    }

    #[test_only]
    /// Initialize the profile registry for testing
    public fun init_for_testing(clock: &Clock, ctx: &mut TxContext) {
        bootstrap_init(clock, ctx);
        memory::bootstrap_init(clock, ctx);
        ai_credit::bootstrap_init(
            tx_context::sender(ctx),
            x"cc62332e34bb2d5cd69f60efbb2a36cb916c7eb458301ea36636c4dbb012bd88",
            ctx,
        );
    }

    #[test_only]
    /// Register a test username for testing
    public fun register_username(
        registry: &mut UsernameRegistry,
        username: String,
        display_name: Option<String>,
        _profile_picture: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let username = normalize_username(&username);
        let owner = tx_context::sender(ctx);
        let epoch = clock::timestamp_ms(clock);
        
        // Create a profile with a proper ID
        let profile = Profile {
            id: object::new(ctx),
            display_name,
            bio: string::utf8(b"Test bio"),
            profile_picture: option::none(),
            cover_photo: option::none(),
            created_at: epoch,
            owner,
            x_username: option::none(),
            website: option::none(),
            birthdate: option::none(),
            location: option::none(),
            badges: vector::empty<ProfileBadge>(),
            selected_badge_id: option::none(),
            selected_ecosystem_badge_id: option::none(),
            memory_account_id: option::none(),
            ai_credit_balance_id: option::none(),
            version: upgrade::current_version(),
        };
        
        // Get the profile ID and use it for registration
        let profile_id = object::uid_to_address(&profile.id);

        claim_username(registry, username, profile_id);
        table::add(&mut registry.address_profiles, owner, profile_id);
        
        // Share the profile
        transfer::share_object(profile);
    }

    /// Linked [`social_contracts::memory::MemoryAccount`] object id (`None` for legacy/shared test profiles).
    public fun linked_memory_account_id(profile: &Profile): &Option<ID> {
        &profile.memory_account_id
    }

    /// Linked [`social_contracts::ai_credit::AiCreditBalance`] object id (always set for greenfield profiles).
    public fun linked_ai_credit_balance_id(profile: &Profile): &Option<ID> {
        &profile.ai_credit_balance_id
    }

    /// X/Twitter username on the profile (set or cleared only via admin entry).
    public fun x_username(profile: &Profile): &Option<String> {
        &profile.x_username
    }

    /// Adds a badge to a profile - called by platform module
    /// This function trusts the caller has done authorization checks
    public(package) fun add_badge_to_profile(
        profile: &mut Profile,
        badge_id: String,
        badge_name: String,
        badge_description: String,
        badge_media_url: String,
        badge_icon_url: String,
        platform_id: address,
        timestamp: u64,
        issuer: address,
        badge_type: u8
    ) {
        // Create the new badge
        let badge = ProfileBadge {
            badge_id: badge_id,
            name: badge_name,
            description: badge_description,
            media_url: badge_media_url,
            icon_url: badge_icon_url,
            platform_id,
            issued_at: timestamp,
            issued_by: issuer,
            badge_type,
        };
        
        // Check if badge with same ID already exists
        let mut i = 0;
        let len = vector::length(&profile.badges);
        while (i < len) {
            let existing_badge = vector::borrow(&profile.badges, i);
            if (string::as_bytes(&existing_badge.badge_id) == string::as_bytes(&badge_id)) {
                abort EBadgeAlreadyExists
            };
            i = i + 1;
        };
        
        // Add the badge to the profile
        vector::push_back(&mut profile.badges, badge);
        
        // If no badge is currently selected and this is the first badge, auto-select it
        if (option::is_none(&profile.selected_badge_id) && vector::length(&profile.badges) == 1) {
            profile.selected_badge_id = option::some(badge_id);
        };
        
        // Emit badge assigned event
        event::emit(BadgeAssignedEvent {
            profile_id: object::uid_to_address(&profile.id),
            badge_id: badge_id,
            name: badge_name,
            description: badge_description,
            media_url: badge_media_url,
            icon_url: badge_icon_url,
            platform_id,
            assigned_by: issuer,
            assigned_at: timestamp,
            badge_type,
        });
    }
    
    /// Removes a badge from a profile - called by platform module
    /// This function trusts the caller has done authorization checks
    public(package) fun remove_badge_from_profile(
        profile: &mut Profile,
        badge_id: &String,
        platform_id: address,
        revoker: address,
        timestamp: u64
    ) {
        // Search for and remove the badge with the given ID
        let mut found = false;
        let mut i = 0;
        let len = vector::length(&profile.badges);
        
        while (i < len) {
            let badge = vector::borrow(&profile.badges, i);
            if (string::as_bytes(&badge.badge_id) == string::as_bytes(badge_id)) {
                // Ensure badge was issued by this platform
                assert!(badge.platform_id == platform_id, EUnauthorized);
                
                // Remove the badge at this index
                vector::remove(&mut profile.badges, i);
                found = true;
                
                // If the removed badge was the selected badge, clear the selection
                if (option::is_some(&profile.selected_badge_id)) {
                    let selected_id = option::borrow(&profile.selected_badge_id);
                    if (string::as_bytes(selected_id) == string::as_bytes(badge_id)) {
                        profile.selected_badge_id = option::none();
                    };
                };
                if (option::is_some(&profile.selected_ecosystem_badge_id) && is_ecosystem_badge(badge_id)) {
                    let selected_id = option::borrow(&profile.selected_ecosystem_badge_id);
                    if (string::as_bytes(selected_id) == string::as_bytes(badge_id)) {
                        profile.selected_ecosystem_badge_id = option::none();
                    };
                };
                
                // Emit badge revoked event
                event::emit(BadgeRevokedEvent {
                    profile_id: object::uid_to_address(&profile.id),
                    badge_id: *badge_id,
                    platform_id,
                    revoked_by: revoker,
                    revoked_at: timestamp,
                });
                
                break
            };
            i = i + 1;
        };
        
        // Make sure we found and removed the badge
        assert!(found, EBadgeNotFound);
    }

    /// Remove a badge from a profile - can be called by profile owner
    /// Users can delete badges they don't want to display
    /// Note: Badges are tied to profile identity and cannot be transferred separately
    public entry fun remove_own_badge(
        profile: &mut Profile,
        badge_id: String,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        
        // Verify sender is the profile owner
        assert!(profile.owner == sender, EUnauthorized);
        
        // Search for and remove the badge with the given ID
        let mut found = false;
        let mut i = 0;
        let len = vector::length(&profile.badges);
        
        while (i < len) {
            let badge = vector::borrow(&profile.badges, i);
            if (string::as_bytes(&badge.badge_id) == string::as_bytes(&badge_id)) {
                // Get platform_id before removing (needed for event)
                let platform_id = badge.platform_id;
                
                // Remove the badge at this index
                vector::remove(&mut profile.badges, i);
                found = true;
                
                // If the removed badge was the selected badge, clear the selection
                if (option::is_some(&profile.selected_badge_id)) {
                    let selected_id = option::borrow(&profile.selected_badge_id);
                    if (string::as_bytes(selected_id) == string::as_bytes(&badge_id)) {
                        profile.selected_badge_id = option::none();
                    };
                };
                if (option::is_some(&profile.selected_ecosystem_badge_id) && is_ecosystem_badge(&badge_id)) {
                    let selected_id = option::borrow(&profile.selected_ecosystem_badge_id);
                    if (string::as_bytes(selected_id) == string::as_bytes(&badge_id)) {
                        profile.selected_ecosystem_badge_id = option::none();
                    };
                };
                
                // Emit badge removed event (user-initiated, different from revoked)
                event::emit(BadgeRemovedEvent {
                    profile_id: object::uid_to_address(&profile.id),
                    badge_id: badge_id,
                    platform_id,
                    removed_by: sender,
                    removed_at: clock::timestamp_ms(clock),
                });
                
                break
            };
            i = i + 1;
        };
        
        // Make sure we found and removed the badge
        assert!(found, EBadgeNotFound);
    }

    /// Get all badges associated with a profile
    /// Returns vector of BadgeData for querying badge information
    /// Note: Badges are tied to this profile and cannot be transferred to other profiles
    public fun get_profile_badges(profile: &Profile): vector<BadgeData> {
        let mut result = vector::empty<BadgeData>();
        let mut i = 0;
        let len = vector::length(&profile.badges);
        
        while (i < len) {
            let badge = vector::borrow(&profile.badges, i);
            vector::push_back(&mut result, BadgeData {
                badge_id: badge.badge_id,
                name: badge.name,
                description: badge.description,
                media_url: badge.media_url,
                icon_url: badge.icon_url,
                platform_id: badge.platform_id,
                issued_at: badge.issued_at,
                issued_by: badge.issued_by,
                badge_type: badge.badge_type,
            });
            i = i + 1;
        };
        
        result
    }
    
    /// Check if a profile has a specific badge
    public fun has_badge(profile: &Profile, badge_id: &String): bool {
        let mut i = 0;
        let len = vector::length(&profile.badges);
        
        while (i < len) {
            let badge = vector::borrow(&profile.badges, i);
            if (string::as_bytes(&badge.badge_id) == string::as_bytes(badge_id)) {
                return true
            };
            i = i + 1;
        };
        
        false
    }
    
    /// Get a specific badge from a profile by badge ID
    /// Returns BadgeData for querying badge information
    public fun get_badge(profile: &Profile, badge_id: &String): Option<BadgeData> {
        let mut i = 0;
        let len = vector::length(&profile.badges);
        
        while (i < len) {
            let badge = vector::borrow(&profile.badges, i);
            if (string::as_bytes(&badge.badge_id) == string::as_bytes(badge_id)) {
                return option::some(BadgeData {
                    badge_id: badge.badge_id,
                    name: badge.name,
                    description: badge.description,
                    media_url: badge.media_url,
                    icon_url: badge.icon_url,
                    platform_id: badge.platform_id,
                    issued_at: badge.issued_at,
                    issued_by: badge.issued_by,
                    badge_type: badge.badge_type,
                })
            };
            i = i + 1;
        };
        
        option::none()
    }
    
    /// Get badges issued by a specific platform
    /// Returns vector of BadgeData for querying badge information
    public fun get_platform_badges(profile: &Profile, platform_id: address): vector<BadgeData> {
        let mut result = vector::empty<BadgeData>();
        
        let mut i = 0;
        let len = vector::length(&profile.badges);
        
        while (i < len) {
            let badge = vector::borrow(&profile.badges, i);
            if (badge.platform_id == platform_id) {
                vector::push_back(&mut result, BadgeData {
                    badge_id: badge.badge_id,
                    name: badge.name,
                    description: badge.description,
                    media_url: badge.media_url,
                    icon_url: badge.icon_url,
                    platform_id: badge.platform_id,
                    issued_at: badge.issued_at,
                    issued_by: badge.issued_by,
                    badge_type: badge.badge_type,
                });
            };
            i = i + 1;
        };
        
        result
    }
    
    /// Count the number of badges a profile has
    /// Get the badge ID from a ProfileBadge
    public fun badge_id(badge: &ProfileBadge): String {
        badge.badge_id
    }

    // === BadgeData Accessor Functions ===

    /// Get badge_id from BadgeData
    public fun badge_data_id(data: &BadgeData): String {
        data.badge_id
    }

    /// Get name from BadgeData
    public fun badge_data_name(data: &BadgeData): String {
        data.name
    }

    /// Get description from BadgeData
    public fun badge_data_description(data: &BadgeData): String {
        data.description
    }

    /// Get media_url from BadgeData
    public fun badge_data_media_url(data: &BadgeData): String {
        data.media_url
    }

    /// Get icon_url from BadgeData
    public fun badge_data_icon_url(data: &BadgeData): String {
        data.icon_url
    }

    /// Get platform_id from BadgeData
    public fun badge_data_platform_id(data: &BadgeData): address {
        data.platform_id
    }

    /// Get issued_at from BadgeData
    public fun badge_data_issued_at(data: &BadgeData): u64 {
        data.issued_at
    }

    /// Get issued_by from BadgeData
    public fun badge_data_issued_by(data: &BadgeData): address {
        data.issued_by
    }

    /// Get badge_type from BadgeData
    public fun badge_data_badge_type(data: &BadgeData): u8 {
        data.badge_type
    }

    public fun badge_count(profile: &Profile): u64 {
        vector::length(&profile.badges)
    }

    /// Set the selected badge to display for a profile (owner only)
    /// The badge must exist in the profile's badges collection
    public entry fun set_selected_badge(
        profile: &mut Profile,
        badge_id: String,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        
        // Verify sender is the profile owner
        assert!(profile.owner == sender, EUnauthorized);
        
        // Verify the badge exists in the profile's badges
        let mut badge_exists = false;
        let mut i = 0;
        let len = vector::length(&profile.badges);
        while (i < len) {
            let badge = vector::borrow(&profile.badges, i);
            if (string::as_bytes(&badge.badge_id) == string::as_bytes(&badge_id)) {
                badge_exists = true;
                break
            };
            i = i + 1;
        };
        
        assert!(badge_exists, ESelectedBadgeNotFound);
        
        // Set the selected badge
        profile.selected_badge_id = option::some(badge_id);
        
        // Emit badge selected event
        event::emit(BadgeSelectedEvent {
            profile_id: object::uid_to_address(&profile.id),
            badge_id: badge_id,
            selected_by: sender,
            selected_at: clock::timestamp_ms(clock),
        });
    }

    /// Get the selected badge ID for a profile
    public fun get_selected_badge_id(profile: &Profile): Option<String> {
        profile.selected_badge_id
    }

    /// Get the badge that should be displayed for a profile
    /// Returns the selected badge if one is set, otherwise returns the first badge
    /// Returns None if the profile has no badges
    /// Returns BadgeData for querying badge information
    public fun get_display_badge(profile: &Profile): Option<BadgeData> {
        let badge_count = vector::length(&profile.badges);
        
        // If no badges exist, return None
        if (badge_count == 0) {
            return option::none()
        };
        
        // If a badge is selected, find and return it
        if (option::is_some(&profile.selected_badge_id)) {
            let selected_id = option::borrow(&profile.selected_badge_id);
            let mut i = 0;
            while (i < badge_count) {
                let badge = vector::borrow(&profile.badges, i);
                if (string::as_bytes(&badge.badge_id) == string::as_bytes(selected_id)) {
                    return option::some(BadgeData {
                        badge_id: badge.badge_id,
                        name: badge.name,
                        description: badge.description,
                        media_url: badge.media_url,
                        icon_url: badge.icon_url,
                        platform_id: badge.platform_id,
                        issued_at: badge.issued_at,
                        issued_by: badge.issued_by,
                        badge_type: badge.badge_type,
                    })
                };
                i = i + 1;
            };
        };
        
        // If no badge is selected or selected badge not found, return the first badge
        let badge = vector::borrow(&profile.badges, 0);
        option::some(BadgeData {
            badge_id: badge.badge_id,
            name: badge.name,
            description: badge.description,
            media_url: badge.media_url,
            icon_url: badge.icon_url,
            platform_id: badge.platform_id,
            issued_at: badge.issued_at,
            issued_by: badge.issued_by,
            badge_type: badge.badge_type,
        })
    }

    /// Clear the selected badge (owner only). Clears both platform and ecosystem selection overrides;
    /// display falls back to first badge per [`get_display_badge`] / [`get_display_ecosystem_badge`].
    public entry fun clear_selected_badge(
        profile: &mut Profile,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        
        // Verify sender is the profile owner
        assert!(profile.owner == sender, EUnauthorized);
        
        let had_platform = option::is_some(&profile.selected_badge_id);
        let had_ecosystem = option::is_some(&profile.selected_ecosystem_badge_id);
        profile.selected_badge_id = option::none();
        profile.selected_ecosystem_badge_id = option::none();

        if (had_platform || had_ecosystem) {
            event::emit(BadgeSelectedEvent {
                profile_id: object::uid_to_address(&profile.id),
                badge_id: string::utf8(b""),
                selected_by: sender,
                selected_at: clock::timestamp_ms(clock),
            });
        };
    }

    /// Set the selected ecosystem badge to display for a profile (owner only)
    /// The badge must exist and have the ecosystem_badge_ prefix
    public fun set_selected_ecosystem_badge(
        profile: &mut Profile,
        badge_id: String,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(profile.owner == sender, EUnauthorized);
        assert!(is_ecosystem_badge(&badge_id), ENotEcosystemBadge);

        let mut badge_exists = false;
        let mut i = 0;
        let len = vector::length(&profile.badges);
        while (i < len) {
            let badge = vector::borrow(&profile.badges, i);
            if (string::as_bytes(&badge.badge_id) == string::as_bytes(&badge_id)) {
                badge_exists = true;
                break
            };
            i = i + 1;
        };
        assert!(badge_exists, ESelectedBadgeNotFound);

        let badge_id_for_event = copy_string(&badge_id);
        profile.selected_ecosystem_badge_id = option::some(badge_id);

        event::emit(BadgeSelectedEvent {
            profile_id: object::uid_to_address(&profile.id),
            badge_id: badge_id_for_event,
            selected_by: sender,
            selected_at: clock::timestamp_ms(clock),
        });
    }

    /// Get the selected ecosystem badge ID for a profile
    public fun get_selected_ecosystem_badge_id(profile: &Profile): Option<String> {
        profile.selected_ecosystem_badge_id
    }

    /// Get the ecosystem badge that should be displayed for a profile
    /// Returns the selected ecosystem badge if one is set, otherwise the first ecosystem badge
    /// Returns None if the profile has no ecosystem badges
    public fun get_display_ecosystem_badge(profile: &Profile): Option<BadgeData> {
        let mut ecosystem_badges = vector::empty<BadgeData>();
        let mut i = 0;
        let len = vector::length(&profile.badges);
        while (i < len) {
            let badge = vector::borrow(&profile.badges, i);
            if (is_ecosystem_badge(&badge.badge_id)) {
                vector::push_back(&mut ecosystem_badges, BadgeData {
                    badge_id: badge.badge_id,
                    name: badge.name,
                    description: badge.description,
                    media_url: badge.media_url,
                    icon_url: badge.icon_url,
                    platform_id: badge.platform_id,
                    issued_at: badge.issued_at,
                    issued_by: badge.issued_by,
                    badge_type: badge.badge_type,
                });
            };
            i = i + 1;
        };

        let count = vector::length(&ecosystem_badges);
        if (count == 0) {
            return option::none()
        };

        if (option::is_some(&profile.selected_ecosystem_badge_id)) {
            let selected_id = option::borrow(&profile.selected_ecosystem_badge_id);
            let mut j = 0;
            while (j < count) {
                let data = vector::borrow(&ecosystem_badges, j);
                if (string::as_bytes(&data.badge_id) == string::as_bytes(selected_id)) {
                    return option::some(*data)
                };
                j = j + 1;
            };
        };

        let first = vector::borrow(&ecosystem_badges, 0);
        option::some(*first)
    }

    /// Clear the selected ecosystem badge (owner only)
    public fun clear_selected_ecosystem_badge(
        profile: &mut Profile,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(profile.owner == sender, EUnauthorized);

        if (option::is_some(&profile.selected_ecosystem_badge_id)) {
            profile.selected_ecosystem_badge_id = option::none();
            event::emit(EcosystemBadgeSelectionClearedEvent {
                profile_id: object::uid_to_address(&profile.id),
                cleared_by: sender,
                cleared_at: clock::timestamp_ms(clock),
            });
        };
    }

    // === Vesting Functions ===

    fun normalize_curve_factor(config: &ProfileConfig, curve_factor: u64): u64 {
        if (curve_factor == 0) {
            config.curve_precision
        } else {
            assert!(
                curve_factor >= config.curve_factor_min && curve_factor <= config.curve_factor_max,
                EInvalidSchedule,
            );
            curve_factor
        }
    }

    fun piece_amount(total_amount: u64, amount_bps: u64): u64 {
        ((total_amount as u128) * (amount_bps as u128) / (BPS_DENOMINATOR as u128)) as u64
    }

    fun validate_piece(config: &ProfileConfig, piece: &VestingPiece, total_amount: u64) {
        if (piece.kind == PIECE_KIND_CLIFF) {
            assert!(piece.duration == 0, EInvalidPieceDuration);
        } else if (piece.kind == PIECE_KIND_CONTINUOUS) {
            assert!(piece.duration > 0, EInvalidPieceDuration);
            let _ = normalize_curve_factor(config, piece.curve_factor);
        } else {
            abort EInvalidPieceKind
        };
        assert!(piece.amount_bps > 0, EInvalidSchedule);
        assert!(piece_amount(total_amount, piece.amount_bps) >= 1, EInvalidSchedule);
    }

    /// Validate schedule pieces and return `schedule_end` (absolute ms timestamp).
    fun validate_schedule(
        config: &ProfileConfig,
        start_time: u64,
        total_amount: u64,
        pieces: &vector<VestingPiece>,
    ): u64 {
        let num_pieces = vector::length(pieces);
        assert!(num_pieces >= 1 && num_pieces <= config.max_vesting_pieces, ETooManyPieces);

        let mut total_bps = 0u64;
        let mut schedule_end = start_time;
        let mut prev_offset = 0u64;
        let mut i = 0;
        while (i < num_pieces) {
            let piece = vector::borrow(pieces, i);
            validate_piece(config, piece, total_amount);

            assert!(piece.time_offset >= prev_offset, EInvalidSchedule);
            prev_offset = piece.time_offset;

            total_bps = total_bps + piece.amount_bps;

            let piece_end_offset = if (piece.kind == PIECE_KIND_CLIFF) {
                piece.time_offset
            } else {
                assert!(piece.time_offset <= MAX_U64 - piece.duration, EScheduleOverflow);
                piece.time_offset + piece.duration
            };

            if (piece_end_offset > schedule_end - start_time) {
                assert!(start_time <= MAX_U64 - piece_end_offset, EScheduleOverflow);
                schedule_end = start_time + piece_end_offset;
            };

            i = i + 1;
        };

        assert!(total_bps == BPS_DENOMINATOR, EInvalidSchedule);
        schedule_end
    }

    fun apply_curve(config: &ProfileConfig, progress: u128, curve_factor: u64): u128 {
        let precision = config.curve_precision;
        if (curve_factor == precision) {
            progress
        } else if (curve_factor > precision) {
            let steepness = (curve_factor - precision) as u128;
            let quadratic = (progress * progress) / (precision as u128);
            let linear_part = progress;
            (linear_part * ((precision as u128) - steepness) + quadratic * steepness)
                / (precision as u128)
        } else {
            let steepness = (precision - curve_factor) as u128;
            let sqrt_approx = sqrt_approximation(progress * (precision as u128));
            let linear_part = progress;
            (sqrt_approx * steepness + linear_part * ((precision as u128) - steepness))
                / (precision as u128)
        }
    }

    fun vested_amount_for_piece(
        config: &ProfileConfig,
        total_amount: u64,
        start_time: u64,
        current_time: u64,
        piece: &VestingPiece,
    ): u64 {
        if (current_time < start_time) {
            return 0
        };

        let activation_time = start_time + piece.time_offset;
        if (current_time < activation_time) {
            return 0
        };

        let alloc = piece_amount(total_amount, piece.amount_bps);

        if (piece.kind == PIECE_KIND_CLIFF) {
            return alloc
        };

        let end_time = activation_time + piece.duration;
        if (current_time >= end_time) {
            return alloc
        };

        let elapsed = current_time - activation_time;
        let precision = config.curve_precision;
        let progress = ((elapsed as u128) * (precision as u128)) / (piece.duration as u128);
        let curved = apply_curve(config, progress, piece.curve_factor);
        ((alloc as u128) * curved / (precision as u128)) as u64
    }

    fun calculate_total_vested(config: &ProfileConfig, wallet: &VestingWallet, current_time: u64): u64 {
        if (current_time < wallet.start_time) {
            return 0
        };

        let mut total_released = 0u64;
        let num_pieces = vector::length(&wallet.pieces);
        let mut i = 0;
        while (i < num_pieces) {
            let piece = vector::borrow(&wallet.pieces, i);
            let piece_vested = vested_amount_for_piece(
                config,
                wallet.total_amount,
                wallet.start_time,
                current_time,
                piece,
            );
            assert!(total_released <= MAX_U64 - piece_vested, EOverflow);
            total_released = total_released + piece_vested;
            i = i + 1;
        };

        if (total_released > wallet.total_amount) {
            wallet.total_amount
        } else {
            total_released
        }
    }

    fun finalize_claimable(
        config: &ProfileConfig,
        capped: u64,
        remaining_balance: u64,
        total_amount: u64,
        current_time: u64,
        schedule_end: u64,
    ): u64 {
        if (current_time >= schedule_end) {
            return remaining_balance
        };

        if (capped == 0) {
            return 0
        };

        let mut threshold = total_amount / config.min_claim_threshold_divisor;
        if (threshold == 0) {
            threshold = 1
        };

        if (capped < threshold && capped < remaining_balance) {
            0
        } else {
            capped
        }
    }

    /// Build a continuous vesting piece (linear when curve_factor is 0 or 1000).
    public fun continuous_vesting_piece(
        time_offset: u64,
        duration: u64,
        amount_bps: u64,
        curve_factor: u64,
    ): VestingPiece {
        VestingPiece {
            kind: PIECE_KIND_CONTINUOUS,
            time_offset,
            duration,
            amount_bps,
            curve_factor,
        }
    }

    /// Build a cliff lump unlock piece.
    public fun cliff_lump_piece(time_offset: u64, amount_bps: u64): VestingPiece {
        VestingPiece {
            kind: PIECE_KIND_CLIFF,
            time_offset,
            duration: 0,
            amount_bps,
            curve_factor: 0,
        }
    }

    fun pieces_from_vectors(
        kinds: vector<u8>,
        time_offsets: vector<u64>,
        durations: vector<u64>,
        amount_bps_list: vector<u64>,
        curve_factors: vector<u64>,
    ): vector<VestingPiece> {
        let len = vector::length(&kinds);
        assert!(len == vector::length(&time_offsets), EInvalidSchedule);
        assert!(len == vector::length(&durations), EInvalidSchedule);
        assert!(len == vector::length(&amount_bps_list), EInvalidSchedule);
        assert!(len == vector::length(&curve_factors), EInvalidSchedule);

        let mut pieces = vector::empty<VestingPiece>();
        let mut i = 0;
        while (i < len) {
            vector::push_back(&mut pieces, VestingPiece {
                kind: *vector::borrow(&kinds, i),
                time_offset: *vector::borrow(&time_offsets, i),
                duration: *vector::borrow(&durations, i),
                amount_bps: *vector::borrow(&amount_bps_list, i),
                curve_factor: *vector::borrow(&curve_factors, i),
            });
            i = i + 1;
        };
        pieces
    }

    /// Create a vesting wallet from parallel piece vectors (entry-compatible).
    /// Cliff lumps unlock instantly at `time_offset`; continuous pieces vest over `duration`.
    public entry fun vest_myso(
        config: &ProfileConfig,
        coin: Coin<MYSO>,
        recipient: address,
        start_time: u64,
        kinds: vector<u8>,
        time_offsets: vector<u64>,
        durations: vector<u64>,
        amount_bps_list: vector<u64>,
        curve_factors: vector<u64>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let pieces = pieces_from_vectors(
            kinds,
            time_offsets,
            durations,
            amount_bps_list,
            curve_factors,
        );
        vest_myso_internal(config, coin, recipient, start_time, pieces, clock, ctx);
    }

    fun vest_myso_internal(
        config: &ProfileConfig,
        coin: Coin<MYSO>,
        recipient: address,
        start_time: u64,
        pieces: vector<VestingPiece>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let current_time = clock::timestamp_ms(clock);
        assert!(start_time > current_time, EInvalidStartTime);

        let total_amount = coin::value(&coin);
        assert!(total_amount > 0, EInsufficientTokens);

        let schedule_end = validate_schedule(config, start_time, total_amount, &pieces);

        let wallet = VestingWallet {
            id: object::new(ctx),
            balance: coin::into_balance(coin),
            owner: recipient,
            start_time,
            claimed_amount: 0,
            total_amount,
            schedule_end,
            pieces,
        };

        let wallet_id = object::uid_to_address(&wallet.id);

        let mut piece_events = vector::empty<VestingPieceEvent>();
        let num_pieces = vector::length(&wallet.pieces);
        let mut j = 0;
        while (j < num_pieces) {
            let p = vector::borrow(&wallet.pieces, j);
            vector::push_back(&mut piece_events, VestingPieceEvent {
                kind: p.kind,
                time_offset: p.time_offset,
                duration: p.duration,
                amount_bps: p.amount_bps,
                curve_factor: p.curve_factor,
            });
            j = j + 1;
        };

        event::emit(TokensVestedEvent {
            wallet_id,
            owner: recipient,
            total_amount,
            start_time,
            schedule_end,
            pieces: piece_events,
            vested_at: current_time,
        });

        transfer::public_transfer(wallet, recipient);
    }

    /// Claim vested tokens. Sub-threshold amounts during active vesting are no-ops.
    public entry fun claim_vested_tokens(
        config: &ProfileConfig,
        wallet: &mut VestingWallet,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(wallet.owner == sender, ENotVestingWalletOwner);

        let claimable_amount = calculate_claimable(config, wallet, clock);

        if (claimable_amount > 0) {
            assert!(wallet.claimed_amount <= MAX_U64 - claimable_amount, EOverflow);
            wallet.claimed_amount = wallet.claimed_amount + claimable_amount;

            let claimed_coin = coin::from_balance<MYSO>(
                balance::split(&mut wallet.balance, claimable_amount),
                ctx
            );

            let wallet_id = object::uid_to_address(&wallet.id);
            let remaining_balance = balance::value(&wallet.balance);

            event::emit(TokensClaimedEvent {
                wallet_id,
                owner: sender,
                claimed_amount: claimable_amount,
                remaining_balance,
                claimed_at: clock::timestamp_ms(clock),
            });

            transfer::public_transfer(claimed_coin, sender);
        };
    }

    public fun claimable(config: &ProfileConfig, wallet: &VestingWallet, clock: &Clock): u64 {
        calculate_claimable(config, wallet, clock)
    }

    fun calculate_claimable(config: &ProfileConfig, wallet: &VestingWallet, clock: &Clock): u64 {
        let current_time = clock::timestamp_ms(clock);
        let remaining_balance = balance::value(&wallet.balance);

        if (current_time < wallet.start_time) {
            return 0
        };

        if (current_time >= wallet.schedule_end) {
            return remaining_balance
        };

        let total_vested = calculate_total_vested(config, wallet, current_time);
        let newly_claimable = if (total_vested >= wallet.claimed_amount) {
            total_vested - wallet.claimed_amount
        } else {
            0
        };

        let capped = if (newly_claimable > remaining_balance) {
            remaining_balance
        } else {
            newly_claimable
        };

        finalize_claimable(
            config,
            capped,
            remaining_balance,
            wallet.total_amount,
            current_time,
            wallet.schedule_end,
        )
    }

    /// Simple square root approximation using Newton's method
    fun sqrt_approximation(n: u128): u128 {
        if (n == 0) return 0;
        if (n == 1) return 1;

        let mut x = n;
        let mut y = (x + 1) / 2;

        let mut i = 0;
        while (y < x && i < 10u64) {
            x = y;
            y = (x + n / x) / 2;
            i = i + 1;
        };

        x
    }

    fun destroy_vesting_pieces(mut pieces: vector<VestingPiece>) {
        while (!vector::is_empty(&pieces)) {
            vector::pop_back(&mut pieces);
        };
        vector::destroy_empty(pieces);
    }

    /// Delete an empty vesting wallet
    public entry fun delete_vesting_wallet(wallet: VestingWallet, clock: &Clock, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        assert!(wallet.owner == sender, ENotVestingWalletOwner);

        let wallet_id = object::uid_to_address(&wallet.id);
        let owner = wallet.owner;

        let VestingWallet {
            id,
            balance,
            owner: _,
            start_time: _,
            claimed_amount: _,
            total_amount: _,
            schedule_end: _,
            pieces,
        } = wallet;

        destroy_vesting_pieces(pieces);

        event::emit(VestingWalletDeletedEvent {
            wallet_id,
            owner,
            deleted_at: clock::timestamp_ms(clock),
        });

        object::delete(id);
        balance::destroy_zero(balance);
    }

    // === Vesting Wallet Accessors ===

    public fun vesting_balance(wallet: &VestingWallet): u64 {
        balance::value(&wallet.balance)
    }

    public fun vesting_owner(wallet: &VestingWallet): address {
        wallet.owner
    }

    public fun vesting_start_time(wallet: &VestingWallet): u64 {
        wallet.start_time
    }

    public fun vesting_schedule_end(wallet: &VestingWallet): u64 {
        wallet.schedule_end
    }

    public fun vesting_total_amount(wallet: &VestingWallet): u64 {
        wallet.total_amount
    }

    public fun vesting_claimed_amount(wallet: &VestingWallet): u64 {
        wallet.claimed_amount
    }

    public fun vesting_piece_count(wallet: &VestingWallet): u64 {
        vector::length(&wallet.pieces)
    }

    public fun vesting_pieces(wallet: &VestingWallet): vector<VestingPiece> {
        let mut out = vector::empty<VestingPiece>();
        let len = vector::length(&wallet.pieces);
        let mut i = 0;
        while (i < len) {
            vector::push_back(&mut out, *vector::borrow(&wallet.pieces, i));
            i = i + 1;
        };
        out
    }

}