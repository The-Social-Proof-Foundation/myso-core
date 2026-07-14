// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Universal MyData module for encrypted data monetization (one-time purchase, subscription, owner vault).
///
/// **Production decrypt path (client-only):** Plaintext must only exist off-chain. Callers encrypt before
/// `create` / dedicated `create_and_share_*` entry points, then authorized users use the MyData SDK: resolve access (indexer or
/// `has_access`), request keys via the key server (`fetch_key`) with policy approval (`mydata_approve`),
/// and decrypt locally. Do not rely on Move to produce user content plaintext for marketplace listings.
///
/// **On-chain state:** `encrypted_data` is opaque ciphertext. For BF-HMAC MyData blobs,
/// `mydata::bf_hmac_encryption::EncryptedObject` embeds `package_id` and `id`; the `encryption_id` field
/// must be the same `id` bytes used when encrypting so policy and clients stay aligned. For client-only
/// AES-GCM (or other app-managed schemes), ciphertext does not parse as `EncryptedObject`; encode the
/// scheme in `media_type` (e.g. prefix `aes_gcm:`) or app metadata so indexers pick the right decrypt path.
///
/// **Revocation:** Owners may call [`revoke_access`] to remove a buyer from marketplace access tables.
/// Permissioned key servers re-check [`mydata_approve`] on every `fetch_key`, so revoked buyers cannot
/// obtain new derived keys. Already-fetched keys may still decrypt offline client-side.
///
/// **Query marketplace:** Broad pools, snapshot anchors, claim vault, and Merkle settlement live in this
/// module. Manifest hash and payout trees are operator-defined; the chain records price paid and anchors,
/// not row-level dataset membership.
///

#[allow(duplicate_alias, unused_use, unused_const, lint(public_entry))]
module social_contracts::mydata {
    use std::string::{Self, String};
    use std::option::{Self, Option};
    use std::vector;
    
    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        table::{Self, Table},
        coin::{Self, Coin},
        balance::{Self, Balance},
        clock::{Self, Clock},
        event
    };
    use myso::myso::MYSO;
    use myso::hash;
    use myso::bcs;

    use mydata::merkle;

    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::platform::{Self, Platform};
    use social_contracts::profile::{Self, EcosystemTreasury};
    use social_contracts::subscription::{Self, ProfileSubscriptionService, ProfileSubscription};
    use social_contracts::block_list;

    // === Default constants for config initialization ===
    /// Controls only the broad-pool/snapshot/distribution MyData marketplace.
    /// Direct profile-gated, one-time, and recurring MyData access remain available independently.
    const DEFAULT_MARKETPLACE_ENABLED: bool = false;
    const BPS_DENOM: u64 = 10_000;
    const DEFAULT_P2P_PLATFORM_FEE_BPS: u64 = 250;
    const DEFAULT_P2P_ECOSYSTEM_FEE_BPS: u64 = 250;
    const DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS: u64 = 250;
    const DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS: u64 = 250;
    const DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS: u64 = 0;
    const DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS: u64 = 10_000;
    const DEFAULT_MAX_ENCRYPTED_DATA_BYTES: u64 = 262_144;
    const DEFAULT_MAX_TAG_BYTES: u64 = 64;
    const DEFAULT_MAX_METADATA_BYTES: u64 = 1_024;
    const DEFAULT_MAX_PAYMENT_REFERENCE_BYTES: u64 = 256;
    const DEFAULT_MAX_POOL_ASSIGNMENTS: u64 = 32;
    const DEFAULT_MAX_MERKLE_PROOF_DEPTH: u64 = 64;
    const DEFAULT_MAX_PAID_ACCESS_ENTRIES: u64 = 100_000;
    const DEFAULT_CLAIM_WINDOW_MS: u64 = 2_592_000_000;

    /// Event/indexer tags for [`AccessConfiguration`] (not used for on-chain policy).
    const ACCESS_KIND_PROFILE: u8 = 1;
    const ACCESS_KIND_ONE_TIME: u8 = 2;
    const ACCESS_KIND_RECURRING: u8 = 3;

    // === Error codes ===
    const EUnauthorized: u64 = 1;
    const ENotForSale: u64 = 2;
    const EPriceMismatch: u64 = 3;
    const ESelfPurchase: u64 = 4;
    const EAlreadyPurchased: u64 = 5;
    const EActiveSubscription: u64 = 6;
    const EInvalidInput: u64 = 7;
    const ESubscriptionExpired: u64 = 8;
    const EOverflow: u64 = 9;
    const EInvalidTimeRange: u64 = 10;
    const EDisabled: u64 = 11;
    const EPolicyIdMismatch: u64 = 12;
    const EPolicyNotEntitled: u64 = 13;
    const ENoAccessToRevoke: u64 = 14;
    const EInvalidConfig: u64 = 15;
    const EPlatformMismatch: u64 = 16;

    // === Constants ===
    const MAX_TAGS: u64 = 10;
    const MAX_SUBSCRIPTION_DAYS: u64 = 365;
    const MILLISECONDS_PER_DAY: u64 = 86_400_000;
    const MAX_FREE_ACCESS_GRANTS: u64 = 100_000; // Limit free access to 100k users
    const MAX_U64: u64 = 18446744073709551615; // Max u64 value for overflow protection
    const DEFAULT_MAX_ENCRYPTION_ID_BYTES: u64 = 1024;

    const EPqInvalidInput: u64 = 1;
    const EPqPoolNotFound: u64 = 2;
    const EPqSubPoolNotFound: u64 = 3;
    const EPqInvalidProof: u64 = 4;
    const EPqAlreadyClaimed: u64 = 5;
    const EPqMerkleRootNotPublished: u64 = 6;
    const EPqInsufficientPayment: u64 = 7;
    const EPqAnchorNotFound: u64 = 8;
    const EPqEscrowExceeded: u64 = 9;
    const EPqSnapshotEscrowMissing: u64 = 10;
    const EPqDistributionNotFound: u64 = 11;
    const EPqClaimExpired: u64 = 12;
    const EPqClaimNotExpired: u64 = 13;
    const EPqPlatformMismatch: u64 = 14;
    const EPqDistributionPublished: u64 = 15;

    /// Mutually exclusive access model for a MyData listing.
    public enum AccessConfiguration has store {
        /// Gated by profile subscription on a linked post (no marketplace pricing).
        ProfileSubscription,
        /// One-time marketplace purchase.
        MarketplaceOneTime {
            price: u64,
            purchasers: Table<address, bool>,
        },
        /// Recurring marketplace subscription with fixed duration per purchase.
        MarketplaceRecurring {
            price: u64,
            duration_days: u64,
            subscribers: Table<address, u64>,
        },
    }

    /// Universal MyData for encrypted data monetization
    public struct MyData has key {
        id: UID,
        owner: address,
        
        /// Content metadata; may include scheme prefix for client decrypt (e.g. `aes_gcm:image` vs plain `image`).
        media_type: String,
        tags: vector<String>,                   // Searchable tags
        platform_id: Option<address>,          // Optional platform identification
        
        /// Time and context
        timestamp_start: u64,
        timestamp_end: Option<u64>,             // For time-range data or updates
        created_at: u64,
        last_updated: u64,
        
        /// Opaque ciphertext (BF-HMAC `EncryptedObject` and/or app-defined encoding).
        encrypted_data: vector<u8>,
        /// Encryption identity bytes: must match `id` inside the MyData ciphertext and in `mydata_approve`.
        encryption_id: vector<u8>,

        /// Access model (profile subscription gate or marketplace one-time/recurring).
        access: AccessConfiguration,
        
        /// Extended metadata for data discovery
        geographic_region: Option<String>,
        data_quality: Option<String>,           // "high", "medium", "low"
        sample_size: Option<u64>,
        collection_method: Option<String>,
        is_updating: bool,                      // Whether this data updates over time
        update_frequency: Option<String>,       // "daily", "weekly", "monthly"
        
        /// Version for future upgrades
        version: u64,
    }

    /// Admin capability for MyData system management
    public struct MyDataAdminCap has key, store {
        id: UID,
    }

    /// Global configuration for MyData system.
    public struct MyDataConfig has key {
        id: UID,
        /// Whether buyers may start new query/pool marketplace snapshots.
        marketplace_enabled: bool,
        max_tags: u64,
        max_subscription_days: u64,
        max_free_access_grants: u64,
        max_encryption_id_bytes: u64,
        max_encrypted_data_bytes: u64,
        max_tag_bytes: u64,
        max_metadata_bytes: u64,
        max_payment_reference_bytes: u64,
        max_pool_assignments: u64,
        max_merkle_proof_depth: u64,
        max_paid_access_entries: u64,
        default_claim_window_ms: u64,
        p2p_platform_fee_bps: u64,
        p2p_ecosystem_fee_bps: u64,
        mydata_marketplace_platform_fee_bps: u64,
        mydata_marketplace_ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        version: u64,
    }

    /// Registry for tracking MyData ownership
    public struct MyDataRegistry has key {
        id: UID,
        ip_to_owner: Table<address, address>,
        version: u64,  // Added missing version field
    }

    public struct BroadPool has copy, drop, store {
        id: ID,
        name: String,
        description: String,
        platform_id: Option<address>,
        created_at: u64,
        version: u64,
    }

    public struct SubPool has copy, drop, store {
        id: ID,
        broad_pool_id: ID,
        name: String,
        description: String,
        schema_metadata: Option<vector<u8>>,
        created_at: u64,
        version: u64,
    }

    public struct MyDataPoolRegistry has key {
        id: UID,
        broad_pools: Table<ID, BroadPool>,
        sub_pools: Table<ID, SubPool>,
        broad_to_sub: Table<ID, vector<ID>>,
        mydata_to_sub_pools: Table<address, vector<ID>>,
        next_broad_pool_nonce: u64,
        next_sub_pool_nonce: u64,
        last_created_broad_pool_id: Option<ID>,
        last_created_sub_pool_id: Option<ID>,
        version: u64,
    }

    public struct MyDataPoolAdminCap has key, store {
        id: UID,
    }

    public struct BroadPoolCreatedEvent has copy, drop {
        pool_id: ID,
        name: String,
        platform_id: Option<address>,
        created_at: u64,
    }

    public struct SubPoolCreatedEvent has copy, drop {
        sub_pool_id: ID,
        broad_pool_id: ID,
        name: String,
        created_at: u64,
    }

    public struct MyDataAssignedToSubPoolEvent has copy, drop {
        ip_id: address,
        sub_pool_ids: vector<ID>,
        assigned_at: u64,
    }

    public struct QuerySnapshotAnchor has copy, drop, store {
        snapshot_id: ID,
        buyer_address: address,
        source_pool_id: ID,
        source_sub_pool_id: ID,
        price_paid: u64,
        created_at: u64,
        snapshot_manifest_hash: vector<u8>,
        payment_reference: vector<u8>,
        platform_id: Option<address>,
    }

    public struct SnapshotAnchorRegistry has key {
        id: UID,
        anchors: Table<ID, QuerySnapshotAnchor>,
        next_snapshot_nonce: u64,
        version: u64,
    }

    public struct SnapshotAnchorRecordedEvent has copy, drop {
        snapshot_id: ID,
        buyer_address: address,
        price_paid: u64,
        source_pool_id: ID,
        source_sub_pool_id: ID,
        platform_id: Option<address>,
        created_at: u64,
        snapshot_manifest_hash: vector<u8>,
        payment_reference: vector<u8>,
    }

    public struct DistributionRecordedEvent has copy, drop {
        snapshot_id: ID,
        total_amount: u64,
        contributor_count: u64,
        merkle_root: vector<u8>,
        platform_id: Option<address>,
        claim_deadline_ms: u64,
        published_at: u64,
    }

    public struct SnapshotEscrowFundedEvent has copy, drop {
        snapshot_id: ID,
        funder: address,
        amount: u64,
        total_funded: u64,
        funded_at: u64,
    }

    public struct SnapshotEscrowReclaimedEvent has copy, drop {
        snapshot_id: ID,
        buyer_address: address,
        amount: u64,
        reclaimed_at: u64,
    }

    public struct MyDataClaimVault has key {
        id: UID,
        balance: Balance<MYSO>,
        merkle_roots: Table<ID, vector<u8>>,
        snapshot_escrow: Table<ID, u64>,
        claimed: Table<ID, Table<address, bool>>,
        version: u64,
    }

    public struct MerkleRootPublishedEvent has copy, drop {
        snapshot_id: ID,
        root_hash: vector<u8>,
        published_at: u64,
    }

    public struct ClaimExecutedEvent has copy, drop {
        snapshot_id: ID,
        claimant: address,
        gross_amount: u64,
        platform_fee: u64,
        ecosystem_fee: u64,
        net_amount: u64,
        platform_id: Option<address>,
        claimed_at: u64,
    }

    public struct DistributionRound has copy, drop, store {
        snapshot_id: ID,
        total_amount: u64,
        contributor_count: u64,
        merkle_root: vector<u8>,
        platform_id: Option<address>,
        claim_deadline_ms: u64,
        published_at: u64,
    }

    public struct DistributionRegistry has key {
        id: UID,
        rounds: Table<ID, DistributionRound>,
        version: u64,
    }

    // === Events ===
    
    public struct MyDataCreatedEvent has copy, drop {
        ip_id: address,
        owner: address,
        media_type: String,
        platform_id: Option<address>,
        /// [`ACCESS_KIND_*`] tag for indexers (1=profile, 2=one_time, 3=recurring).
        access_configuration_kind: u8,
        created_at: u64,
    }

    public struct PurchaseEvent has copy, drop {
        ip_id: address,
        buyer: address,
        price: u64,
        purchase_type: String, // "one_time" or "subscription"
        timestamp: u64,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        platform_fee: u64,
        ecosystem_fee: u64,
        creator_amount: u64,
        platform_id: Option<address>,
    }

    public struct AccessGrantedEvent has copy, drop {
        ip_id: address,
        user: address,
        access_type: String,
        granted_by: address,
        timestamp: u64,
    }

    public struct AccessRevokedEvent has copy, drop {
        ip_id: address,
        user: address,
        access_type: String,
        revoked_by: address,
        timestamp: u64,
    }

    public struct MyDataPricingUpdatedEvent has copy, drop {
        ip_id: address,
        one_time_price: Option<u64>,
        subscription_price: Option<u64>,
        subscription_duration_days: Option<u64>,
        updated_by: address,
        timestamp: u64,
    }

    public struct MyDataContentUpdatedEvent has copy, drop {
        ip_id: address,
        encrypted_data_updated: bool,
        tags_updated: bool,
        updated_by: address,
        timestamp: u64,
    }

    public struct MyDataRegisteredEvent has copy, drop {
        ip_id: address,
        owner: address,
        registered_at: u64,
    }

    public struct MyDataUnregisteredEvent has copy, drop {
        ip_id: address,
        owner: address,
        unregistered_at: u64,
    }

    public struct MyDataConfigUpdatedEvent has copy, drop {
        updated_by: address,
        /// Query/pool marketplace availability; does not gate direct MyData listings or purchases.
        marketplace_enabled: bool,
        max_tags: u64,
        max_subscription_days: u64,
        max_free_access_grants: u64,
        max_encryption_id_bytes: u64,
        max_encrypted_data_bytes: u64,
        max_tag_bytes: u64,
        max_metadata_bytes: u64,
        max_payment_reference_bytes: u64,
        max_pool_assignments: u64,
        max_merkle_proof_depth: u64,
        max_paid_access_entries: u64,
        default_claim_window_ms: u64,
        p2p_platform_fee_bps: u64,
        p2p_ecosystem_fee_bps: u64,
        mydata_marketplace_platform_fee_bps: u64,
        mydata_marketplace_ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        timestamp: u64,
    }

    fun validate_fee_config(
        p2p_platform_fee_bps: u64,
        p2p_ecosystem_fee_bps: u64,
        mydata_marketplace_platform_fee_bps: u64,
        mydata_marketplace_ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
    ) {
        assert!(p2p_platform_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(p2p_ecosystem_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(p2p_platform_fee_bps + p2p_ecosystem_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(mydata_marketplace_platform_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(mydata_marketplace_ecosystem_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(mydata_marketplace_platform_fee_bps + mydata_marketplace_ecosystem_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(non_platform_platform_to_creator_bps <= BPS_DENOM, EInvalidConfig);
        assert!(non_platform_platform_to_treasury_bps <= BPS_DENOM, EInvalidConfig);
        assert!(
            non_platform_platform_to_creator_bps + non_platform_platform_to_treasury_bps == BPS_DENOM,
            EInvalidConfig,
        );
    }

    /// No-platform purchases do not assess a platform fee: only the ecosystem slice is deducted.
    fun calculate_p2p_fees_no_platform(config: &MyDataConfig, gross: u64): (u64, u64, u64) {
        let platform_fee = 0;
        let ecosystem_fee = (gross * config.p2p_ecosystem_fee_bps) / BPS_DENOM;
        let creator_amount = gross - ecosystem_fee;
        (platform_fee, ecosystem_fee, creator_amount)
    }

    /// With-platform purchases deduct both the configured platform and ecosystem fee slices.
    fun calculate_p2p_fees_with_platform(config: &MyDataConfig, gross: u64): (u64, u64, u64) {
        let platform_fee = (gross * config.p2p_platform_fee_bps) / BPS_DENOM;
        let ecosystem_fee = (gross * config.p2p_ecosystem_fee_bps) / BPS_DENOM;
        let creator_amount = gross - platform_fee - ecosystem_fee;
        (platform_fee, ecosystem_fee, creator_amount)
    }

    /// No-platform marketplace claims do not assess a platform fee.
    fun calculate_mydata_marketplace_fees_no_platform(config: &MyDataConfig, gross: u64): (u64, u64, u64) {
        let platform_fee = 0;
        let ecosystem_fee = (gross * config.mydata_marketplace_ecosystem_fee_bps) / BPS_DENOM;
        let net_amount = gross - ecosystem_fee;
        (platform_fee, ecosystem_fee, net_amount)
    }

    /// With-platform marketplace claims deduct both the configured platform and ecosystem fee slices.
    fun calculate_mydata_marketplace_fees_with_platform(config: &MyDataConfig, gross: u64): (u64, u64, u64) {
        let platform_fee = (gross * config.mydata_marketplace_platform_fee_bps) / BPS_DENOM;
        let ecosystem_fee = (gross * config.mydata_marketplace_ecosystem_fee_bps) / BPS_DENOM;
        let net_amount = gross - platform_fee - ecosystem_fee;
        (platform_fee, ecosystem_fee, net_amount)
    }

    fun distribute_p2p_fees_no_platform(
        config: &MyDataConfig,
        treasury: &EcosystemTreasury,
        owner: address,
        payment: Coin<MYSO>,
        ctx: &mut TxContext,
    ): (u64, u64, u64) {
        let gross = coin::value(&payment);
        let (platform_fee, ecosystem_fee, creator_amount) = calculate_p2p_fees_no_platform(config, gross);
        let mut payment = payment;

        if (ecosystem_fee > 0) {
            let eco_coin = coin::split(&mut payment, ecosystem_fee, ctx);
            transfer::public_transfer(eco_coin, profile::get_treasury_address(treasury));
        };

        transfer::public_transfer(payment, owner);
        (platform_fee, ecosystem_fee, creator_amount)
    }

    fun distribute_p2p_fees_with_platform(
        config: &MyDataConfig,
        treasury: &EcosystemTreasury,
        owner: address,
        payment: Coin<MYSO>,
        platform: &mut Platform,
        clock: &Clock,
        ctx: &mut TxContext,
    ): (u64, u64, u64) {
        let gross = coin::value(&payment);
        let (platform_fee, ecosystem_fee, creator_amount) = calculate_p2p_fees_with_platform(config, gross);
        let mut payment = payment;

        if (ecosystem_fee > 0) {
            let eco_coin = coin::split(&mut payment, ecosystem_fee, ctx);
            transfer::public_transfer(eco_coin, profile::get_treasury_address(treasury));
        };

        if (platform_fee > 0) {
            let mut platform_coin = coin::split(&mut payment, platform_fee, ctx);
            platform::add_to_treasury(platform, &mut platform_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_coin);
        };

        transfer::public_transfer(payment, owner);
        (platform_fee, ecosystem_fee, creator_amount)
    }

    fun assert_platform_matches_listing(mydata: &MyData, platform: &Platform) {
        if (option::is_some(&mydata.platform_id)) {
            let listing_platform = *option::borrow(&mydata.platform_id);
            let provided_platform = object::uid_to_address(platform::id(platform));
            assert!(listing_platform == provided_platform, EPlatformMismatch);
        };
    }

    fun emit_mydata_config_updated(config: &MyDataConfig, updated_by: address, timestamp: u64) {
        event::emit(MyDataConfigUpdatedEvent {
            updated_by,
            marketplace_enabled: config.marketplace_enabled,
            max_tags: config.max_tags,
            max_subscription_days: config.max_subscription_days,
            max_free_access_grants: config.max_free_access_grants,
            max_encryption_id_bytes: config.max_encryption_id_bytes,
            max_encrypted_data_bytes: config.max_encrypted_data_bytes,
            max_tag_bytes: config.max_tag_bytes,
            max_metadata_bytes: config.max_metadata_bytes,
            max_payment_reference_bytes: config.max_payment_reference_bytes,
            max_pool_assignments: config.max_pool_assignments,
            max_merkle_proof_depth: config.max_merkle_proof_depth,
            max_paid_access_entries: config.max_paid_access_entries,
            default_claim_window_ms: config.default_claim_window_ms,
            p2p_platform_fee_bps: config.p2p_platform_fee_bps,
            p2p_ecosystem_fee_bps: config.p2p_ecosystem_fee_bps,
            mydata_marketplace_platform_fee_bps: config.mydata_marketplace_platform_fee_bps,
            mydata_marketplace_ecosystem_fee_bps: config.mydata_marketplace_ecosystem_fee_bps,
            non_platform_platform_to_creator_bps: config.non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps: config.non_platform_platform_to_treasury_bps,
            timestamp,
        });
    }

    // === Admin Functions ===

    /// Create a MyDataAdminCap for bootstrap (package visibility only)
    public(package) fun create_mydata_admin_cap(ctx: &mut TxContext): MyDataAdminCap {
        MyDataAdminCap {
            id: object::new(ctx)
        }
    }

    /// Update MyData configuration (admin only).
    /// `marketplace_enabled` controls only new query/pool marketplace snapshots.
    public entry fun update_mydata_config(
        _: &MyDataAdminCap,
        config: &mut MyDataConfig,
        marketplace_enabled: bool,
        max_tags: u64,
        max_subscription_days: u64,
        max_free_access_grants: u64,
        max_encryption_id_bytes: u64,
        max_encrypted_data_bytes: u64,
        max_tag_bytes: u64,
        max_metadata_bytes: u64,
        max_payment_reference_bytes: u64,
        max_pool_assignments: u64,
        max_merkle_proof_depth: u64,
        max_paid_access_entries: u64,
        default_claim_window_ms: u64,
        p2p_platform_fee_bps: u64,
        p2p_ecosystem_fee_bps: u64,
        mydata_marketplace_platform_fee_bps: u64,
        mydata_marketplace_ecosystem_fee_bps: u64,
        non_platform_platform_to_creator_bps: u64,
        non_platform_platform_to_treasury_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(max_subscription_days > 0, EInvalidInput);
        assert!(max_tags > 0, EInvalidInput);
        assert!(max_free_access_grants > 0, EInvalidInput);
        assert!(max_encryption_id_bytes > 0, EInvalidInput);
        assert!(max_encrypted_data_bytes > 0, EInvalidInput);
        assert!(max_tag_bytes > 0, EInvalidInput);
        assert!(max_metadata_bytes > 0, EInvalidInput);
        assert!(max_payment_reference_bytes > 0, EInvalidInput);
        assert!(max_pool_assignments > 0, EInvalidInput);
        assert!(max_merkle_proof_depth > 0, EInvalidInput);
        assert!(max_paid_access_entries > 0, EInvalidInput);
        assert!(default_claim_window_ms > 0, EInvalidInput);
        validate_fee_config(
            p2p_platform_fee_bps,
            p2p_ecosystem_fee_bps,
            mydata_marketplace_platform_fee_bps,
            mydata_marketplace_ecosystem_fee_bps,
            non_platform_platform_to_creator_bps,
            non_platform_platform_to_treasury_bps,
        );

        config.marketplace_enabled = marketplace_enabled;
        config.max_tags = max_tags;
        config.max_subscription_days = max_subscription_days;
        config.max_free_access_grants = max_free_access_grants;
        config.max_encryption_id_bytes = max_encryption_id_bytes;
        config.max_encrypted_data_bytes = max_encrypted_data_bytes;
        config.max_tag_bytes = max_tag_bytes;
        config.max_metadata_bytes = max_metadata_bytes;
        config.max_payment_reference_bytes = max_payment_reference_bytes;
        config.max_pool_assignments = max_pool_assignments;
        config.max_merkle_proof_depth = max_merkle_proof_depth;
        config.max_paid_access_entries = max_paid_access_entries;
        config.default_claim_window_ms = default_claim_window_ms;
        config.p2p_platform_fee_bps = p2p_platform_fee_bps;
        config.p2p_ecosystem_fee_bps = p2p_ecosystem_fee_bps;
        config.mydata_marketplace_platform_fee_bps = mydata_marketplace_platform_fee_bps;
        config.mydata_marketplace_ecosystem_fee_bps = mydata_marketplace_ecosystem_fee_bps;
        config.non_platform_platform_to_creator_bps = non_platform_platform_to_creator_bps;
        config.non_platform_platform_to_treasury_bps = non_platform_platform_to_treasury_bps;

        emit_mydata_config_updated(
            config,
            tx_context::sender(ctx),
            clock::timestamp_ms(clock),
        );
    }

    /// Whether buyers may start new query/pool marketplace snapshots.
    public fun marketplace_enabled(config: &MyDataConfig): bool {
        config.marketplace_enabled
    }

    // === Core Functions ===

    fun share_mydata_system_objects(clock: &Clock, ctx: &mut TxContext, marketplace_enabled: bool) {
        let sender = tx_context::sender(ctx);
        let ver = upgrade::current_version();
        let config = MyDataConfig {
            id: object::new(ctx),
            marketplace_enabled,
            max_tags: MAX_TAGS,
            max_subscription_days: MAX_SUBSCRIPTION_DAYS,
            max_free_access_grants: MAX_FREE_ACCESS_GRANTS,
            max_encryption_id_bytes: DEFAULT_MAX_ENCRYPTION_ID_BYTES,
            max_encrypted_data_bytes: DEFAULT_MAX_ENCRYPTED_DATA_BYTES,
            max_tag_bytes: DEFAULT_MAX_TAG_BYTES,
            max_metadata_bytes: DEFAULT_MAX_METADATA_BYTES,
            max_payment_reference_bytes: DEFAULT_MAX_PAYMENT_REFERENCE_BYTES,
            max_pool_assignments: DEFAULT_MAX_POOL_ASSIGNMENTS,
            max_merkle_proof_depth: DEFAULT_MAX_MERKLE_PROOF_DEPTH,
            max_paid_access_entries: DEFAULT_MAX_PAID_ACCESS_ENTRIES,
            default_claim_window_ms: DEFAULT_CLAIM_WINDOW_MS,
            p2p_platform_fee_bps: DEFAULT_P2P_PLATFORM_FEE_BPS,
            p2p_ecosystem_fee_bps: DEFAULT_P2P_ECOSYSTEM_FEE_BPS,
            mydata_marketplace_platform_fee_bps: DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS,
            mydata_marketplace_ecosystem_fee_bps: DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS,
            non_platform_platform_to_creator_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS,
            non_platform_platform_to_treasury_bps: DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS,
            version: ver,
        };
        emit_mydata_config_updated(&config, sender, clock::timestamp_ms(clock));
        transfer::share_object(config);

        transfer::share_object(MyDataRegistry {
            id: object::new(ctx),
            ip_to_owner: table::new(ctx),
            version: ver,
        });

        transfer::share_object(MyDataPoolRegistry {
            id: object::new(ctx),
            broad_pools: table::new(ctx),
            sub_pools: table::new(ctx),
            broad_to_sub: table::new(ctx),
            mydata_to_sub_pools: table::new(ctx),
            next_broad_pool_nonce: 0,
            next_sub_pool_nonce: 0,
            last_created_broad_pool_id: option::none(),
            last_created_sub_pool_id: option::none(),
            version: ver,
        });

        transfer::share_object(SnapshotAnchorRegistry {
            id: object::new(ctx),
            anchors: table::new(ctx),
            next_snapshot_nonce: 0,
            version: ver,
        });

        transfer::share_object(MyDataClaimVault {
            id: object::new(ctx),
            balance: balance::zero(),
            merkle_roots: table::new(ctx),
            snapshot_escrow: table::new(ctx),
            claimed: table::new(ctx),
            version: ver,
        });

        transfer::share_object(DistributionRegistry {
            id: object::new(ctx),
            rounds: table::new(ctx),
            version: ver,
        });
    }

    /// Bootstrap: shared config, ownership registry, and query-marketplace objects (pools, anchors, vault).
    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        share_mydata_system_objects(clock, ctx, DEFAULT_MARKETPLACE_ENABLED);
    }

    public(package) fun create_mydata_pool_admin_cap(ctx: &mut TxContext): MyDataPoolAdminCap {
        MyDataPoolAdminCap { id: object::new(ctx) }
    }

    fun gen_pool_id(registry: &MyDataPoolRegistry, nonce: u64): ID {
        let mut data = bcs::to_bytes(&object::uid_to_address(&registry.id));
        vector::append(&mut data, bcs::to_bytes(&nonce));
        object::id_from_bytes(hash::blake2b256(&data))
    }

    fun create_broad_pool_internal(
        config: &MyDataConfig,
        _: &MyDataPoolAdminCap,
        registry: &mut MyDataPoolRegistry,
        name: String,
        description: String,
        platform_id: Option<address>,
        clock: &Clock,
    ) {
        assert!(string::length(&name) > 0 && string::length(&name) <= config.max_metadata_bytes, EPqInvalidInput);
        assert!(string::length(&description) <= config.max_metadata_bytes, EPqInvalidInput);
        let nonce = registry.next_broad_pool_nonce;
        registry.next_broad_pool_nonce = nonce + 1;

        let pool_id = gen_pool_id(registry, nonce);
        let broad_pool = BroadPool {
            id: pool_id,
            name,
            description,
            platform_id,
            created_at: clock::timestamp_ms(clock),
            version: registry.version,
        };

        table::add(&mut registry.broad_pools, pool_id, broad_pool);
        table::add(&mut registry.broad_to_sub, pool_id, vector::empty());
        registry.last_created_broad_pool_id = option::some(pool_id);

        event::emit(BroadPoolCreatedEvent {
            pool_id,
            name: broad_pool.name,
            platform_id,
            created_at: broad_pool.created_at,
        });
    }

    public entry fun create_broad_pool(
        config: &MyDataConfig,
        cap: &MyDataPoolAdminCap,
        registry: &mut MyDataPoolRegistry,
        name: String,
        description: String,
        clock: &Clock,
    ) {
        create_broad_pool_internal(config, cap, registry, name, description, option::none(), clock);
    }

    public entry fun create_broad_pool_with_platform(
        config: &MyDataConfig,
        cap: &MyDataPoolAdminCap,
        registry: &mut MyDataPoolRegistry,
        platform: &Platform,
        name: String,
        description: String,
        clock: &Clock,
    ) {
        create_broad_pool_internal(
            config,
            cap,
            registry,
            name,
            description,
            option::some(object::uid_to_address(platform::id(platform))),
            clock,
        );
    }

    public entry fun create_sub_pool(
        config: &MyDataConfig,
        _: &MyDataPoolAdminCap,
        registry: &mut MyDataPoolRegistry,
        broad_pool_id: ID,
        name: String,
        description: String,
        schema_metadata: Option<vector<u8>>,
        clock: &Clock,
    ) {
        assert!(table::contains(&registry.broad_pools, broad_pool_id), EPqPoolNotFound);
        assert!(string::length(&name) > 0 && string::length(&name) <= config.max_metadata_bytes, EPqInvalidInput);
        assert!(string::length(&description) <= config.max_metadata_bytes, EPqInvalidInput);
        if (option::is_some(&schema_metadata)) {
            assert!(vector::length(option::borrow(&schema_metadata)) <= config.max_metadata_bytes, EPqInvalidInput);
        };

        let nonce = registry.next_sub_pool_nonce;
        registry.next_sub_pool_nonce = nonce + 1;

        let sub_pool_id = gen_pool_id(registry, 0x100000000 | nonce);
        let sub_pool = SubPool {
            id: sub_pool_id,
            broad_pool_id,
            name,
            description,
            schema_metadata,
            created_at: clock::timestamp_ms(clock),
            version: registry.version,
        };

        table::add(&mut registry.sub_pools, sub_pool_id, sub_pool);
        registry.last_created_sub_pool_id = option::some(sub_pool_id);

        let sub_ids = table::borrow_mut(&mut registry.broad_to_sub, broad_pool_id);
        vector::push_back(sub_ids, sub_pool_id);

        event::emit(SubPoolCreatedEvent {
            sub_pool_id,
            broad_pool_id,
            name: sub_pool.name,
            created_at: sub_pool.created_at,
        });
    }

    fun assign_mydata_to_sub_pools(
        config: &MyDataConfig,
        registry: &mut MyDataPoolRegistry,
        ip_id: address,
        sub_pool_ids: vector<ID>,
        clock: &Clock,
    ) {
        assert!(vector::length(&sub_pool_ids) > 0, EPqInvalidInput);
        assert!(vector::length(&sub_pool_ids) <= config.max_pool_assignments, EPqInvalidInput);
        let mut existing = if (table::contains(&registry.mydata_to_sub_pools, ip_id)) {
            *table::borrow(&registry.mydata_to_sub_pools, ip_id)
        } else {
            vector::empty()
        };

        let mut i = 0u64;
        while (i < vector::length(&sub_pool_ids)) {
            let sub_id = *vector::borrow(&sub_pool_ids, i);
            assert!(table::contains(&registry.sub_pools, sub_id), EPqSubPoolNotFound);
            let (has, _) = vector::index_of(&existing, &sub_id);
            if (!has) {
                vector::push_back(&mut existing, sub_id);
                assert!(vector::length(&existing) <= config.max_pool_assignments, EPqInvalidInput);
            };
            i = i + 1;
        };

        if (table::contains(&registry.mydata_to_sub_pools, ip_id)) {
            *table::borrow_mut(&mut registry.mydata_to_sub_pools, ip_id) = existing;
        } else {
            table::add(&mut registry.mydata_to_sub_pools, ip_id, existing);
        };

        event::emit(MyDataAssignedToSubPoolEvent {
            ip_id,
            sub_pool_ids,
            assigned_at: clock::timestamp_ms(clock),
        });
    }

    fun remove_mydata_from_sub_pool(
        registry: &mut MyDataPoolRegistry,
        ip_id: address,
        sub_pool_id: ID,
    ) {
        assert!(table::contains(&registry.mydata_to_sub_pools, ip_id), EPqInvalidInput);
        let sub_ids = table::borrow_mut(&mut registry.mydata_to_sub_pools, ip_id);
        let (found, idx) = vector::index_of(sub_ids, &sub_pool_id);
        assert!(found, EPqInvalidInput);
        vector::remove(sub_ids, idx);
    }

    fun gen_snapshot_id(registry_id: &UID, nonce: u64): ID {
        let mut data = bcs::to_bytes(&object::uid_to_address(registry_id));
        vector::append(&mut data, bcs::to_bytes(&nonce));
        object::id_from_bytes(hash::blake2b256(&data))
    }

    public entry fun record_snapshot_anchor(
        config: &MyDataConfig,
        anchor_registry: &mut SnapshotAnchorRegistry,
        vault: &mut MyDataClaimVault,
        pool_registry: &MyDataPoolRegistry,
        source_pool_id: ID,
        source_sub_pool_id: ID,
        manifest_hash: vector<u8>,
        payment_reference: vector<u8>,
        payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.marketplace_enabled, EDisabled);
        assert!(table::contains(&pool_registry.broad_pools, source_pool_id), EPqPoolNotFound);
        assert!(table::contains(&pool_registry.sub_pools, source_sub_pool_id), EPqSubPoolNotFound);
        let broad_pool = table::borrow(&pool_registry.broad_pools, source_pool_id);
        let sub_pool = table::borrow(&pool_registry.sub_pools, source_sub_pool_id);
        assert!(sub_pool.broad_pool_id == source_pool_id, EPqSubPoolNotFound);
        assert!(vector::length(&manifest_hash) == 32, EPqInvalidInput);
        assert!(vector::length(&payment_reference) <= config.max_payment_reference_bytes, EPqInvalidInput);

        let price_paid = coin::value(&payment);
        assert!(price_paid > 0, EPqInsufficientPayment);

        let nonce = anchor_registry.next_snapshot_nonce;
        anchor_registry.next_snapshot_nonce = nonce + 1;

        let snapshot_id = gen_snapshot_id(&anchor_registry.id, nonce);
        let buyer = tx_context::sender(ctx);

        let anchor = QuerySnapshotAnchor {
            snapshot_id,
            buyer_address: buyer,
            source_pool_id,
            source_sub_pool_id,
            price_paid,
            created_at: clock::timestamp_ms(clock),
            snapshot_manifest_hash: manifest_hash,
            payment_reference,
            platform_id: broad_pool.platform_id,
        };

        let created_at = anchor.created_at;
        let snapshot_manifest_hash_ev = copy anchor.snapshot_manifest_hash;
        let payment_reference_ev = copy anchor.payment_reference;
        table::add(&mut anchor_registry.anchors, snapshot_id, anchor);
        balance::join(&mut vault.balance, coin::into_balance(payment));
        assert!(!table::contains(&vault.snapshot_escrow, snapshot_id), EPqInvalidInput);
        table::add(&mut vault.snapshot_escrow, snapshot_id, price_paid);

        event::emit(SnapshotAnchorRecordedEvent {
            snapshot_id,
            buyer_address: buyer,
            price_paid,
            source_pool_id,
            source_sub_pool_id,
            platform_id: broad_pool.platform_id,
            created_at,
            snapshot_manifest_hash: snapshot_manifest_hash_ev,
            payment_reference: payment_reference_ev,
        });
    }

    public fun get_snapshot_anchor(
        anchor_registry: &SnapshotAnchorRegistry,
        snapshot_id: ID,
    ): Option<QuerySnapshotAnchor> {
        if (table::contains(&anchor_registry.anchors, snapshot_id)) {
            option::some(*table::borrow(&anchor_registry.anchors, snapshot_id))
        } else {
            option::none()
        }
    }

    public entry fun deposit_snapshot_escrow(
        _: &MyDataPoolAdminCap,
        anchor_registry: &SnapshotAnchorRegistry,
        vault: &mut MyDataClaimVault,
        snapshot_id: ID,
        payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert!(table::contains(&anchor_registry.anchors, snapshot_id), EPqAnchorNotFound);
        assert!(!table::contains(&vault.merkle_roots, snapshot_id), EPqDistributionPublished);
        assert!(table::contains(&vault.snapshot_escrow, snapshot_id), EPqSnapshotEscrowMissing);
        let amount = coin::value(&payment);
        assert!(amount > 0, EPqInsufficientPayment);
        let escrow = table::borrow_mut(&mut vault.snapshot_escrow, snapshot_id);
        assert!(*escrow <= MAX_U64 - amount, EOverflow);
        *escrow = *escrow + amount;
        let total_funded = *escrow;
        balance::join(&mut vault.balance, coin::into_balance(payment));

        event::emit(SnapshotEscrowFundedEvent {
            snapshot_id,
            funder: tx_context::sender(ctx),
            amount,
            total_funded,
            funded_at: clock::timestamp_ms(clock),
        });
    }

    public entry fun publish_distribution(
        config: &MyDataConfig,
        _: &MyDataPoolAdminCap,
        anchor_registry: &SnapshotAnchorRegistry,
        dist_registry: &mut DistributionRegistry,
        vault: &mut MyDataClaimVault,
        snapshot_id: ID,
        root_hash: vector<u8>,
        total_amount: u64,
        contributor_count: u64,
        clock: &Clock,
    ) {
        assert!(table::contains(&anchor_registry.anchors, snapshot_id), EPqAnchorNotFound);
        assert!(table::contains(&vault.snapshot_escrow, snapshot_id), EPqSnapshotEscrowMissing);
        assert!(vector::length(&root_hash) == 32, EPqInvalidInput);
        assert!(total_amount > 0 && contributor_count > 0, EPqInvalidInput);
        assert!(*table::borrow(&vault.snapshot_escrow, snapshot_id) == total_amount, EPqEscrowExceeded);
        assert!(!table::contains(&vault.merkle_roots, snapshot_id), EPqDistributionPublished);
        assert!(!table::contains(&dist_registry.rounds, snapshot_id), EPqDistributionPublished);

        let now = clock::timestamp_ms(clock);
        assert!(now <= MAX_U64 - config.default_claim_window_ms, EOverflow);
        let claim_deadline_ms = now + config.default_claim_window_ms;
        let platform_id = table::borrow(&anchor_registry.anchors, snapshot_id).platform_id;
        table::add(&mut vault.merkle_roots, snapshot_id, copy root_hash);
        table::add(&mut dist_registry.rounds, snapshot_id, DistributionRound {
            snapshot_id,
            total_amount,
            contributor_count,
            merkle_root: copy root_hash,
            platform_id,
            claim_deadline_ms,
            published_at: now,
        });

        event::emit(MerkleRootPublishedEvent {
            snapshot_id,
            root_hash: copy root_hash,
            published_at: now,
        });
        event::emit(DistributionRecordedEvent {
            snapshot_id,
            total_amount,
            contributor_count,
            merkle_root: root_hash,
            platform_id,
            claim_deadline_ms,
            published_at: now,
        });
    }

    fun distribute_mydata_marketplace_claim_fees_no_platform(
        config: &MyDataConfig,
        treasury: &EcosystemTreasury,
        claimant: address,
        gross_amount: u64,
        vault_balance: &mut Balance<MYSO>,
        ctx: &mut TxContext,
    ): (u64, u64, u64) {
        let (platform_fee, ecosystem_fee, net_amount) =
            calculate_mydata_marketplace_fees_no_platform(config, gross_amount);
        let mut payout_coin = coin::from_balance(balance::split(vault_balance, gross_amount), ctx);

        if (ecosystem_fee > 0) {
            let eco_coin = coin::split(&mut payout_coin, ecosystem_fee, ctx);
            transfer::public_transfer(eco_coin, profile::get_treasury_address(treasury));
        };

        transfer::public_transfer(payout_coin, claimant);
        (platform_fee, ecosystem_fee, net_amount)
    }

    fun distribute_mydata_marketplace_claim_fees_with_platform(
        config: &MyDataConfig,
        treasury: &EcosystemTreasury,
        claimant: address,
        gross_amount: u64,
        platform: &mut Platform,
        vault_balance: &mut Balance<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ): (u64, u64, u64) {
        let (platform_fee, ecosystem_fee, net_amount) =
            calculate_mydata_marketplace_fees_with_platform(config, gross_amount);
        let mut payout_coin = coin::from_balance(balance::split(vault_balance, gross_amount), ctx);

        if (ecosystem_fee > 0) {
            let eco_coin = coin::split(&mut payout_coin, ecosystem_fee, ctx);
            transfer::public_transfer(eco_coin, profile::get_treasury_address(treasury));
        };

        if (platform_fee > 0) {
            let mut platform_coin = coin::split(&mut payout_coin, platform_fee, ctx);
            platform::add_to_treasury(platform, &mut platform_coin, platform_fee, clock, ctx);
            coin::destroy_zero(platform_coin);
        };

        transfer::public_transfer(payout_coin, claimant);
        (platform_fee, ecosystem_fee, net_amount)
    }

    fun claim_internal_no_platform(
        config: &MyDataConfig,
        dist_registry: &DistributionRegistry,
        vault: &mut MyDataClaimVault,
        treasury: &EcosystemTreasury,
        snapshot_id: ID,
        amount: u64,
        leaf_index: u64,
        proof: vector<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(amount > 0, EPqInvalidInput);
        assert!(vector::length(&proof) <= config.max_merkle_proof_depth, EPqInvalidProof);
        assert!(table::contains(&dist_registry.rounds, snapshot_id), EPqDistributionNotFound);
        let round = table::borrow(&dist_registry.rounds, snapshot_id);
        assert!(option::is_none(&round.platform_id), EPqPlatformMismatch);
        assert!(clock::timestamp_ms(clock) <= round.claim_deadline_ms, EPqClaimExpired);
        assert!(table::contains(&vault.merkle_roots, snapshot_id), EPqMerkleRootNotPublished);
        assert!(table::contains(&vault.snapshot_escrow, snapshot_id), EPqSnapshotEscrowMissing);
        assert!(*table::borrow(&vault.snapshot_escrow, snapshot_id) >= amount, EPqEscrowExceeded);

        let claimant = tx_context::sender(ctx);
        let leaf = merkle::leaf_hash_with_platform(
            claimant,
            amount,
            object::id_to_bytes(&snapshot_id),
            option::none(),
        );
        let root = *table::borrow(&vault.merkle_roots, snapshot_id);
        assert!(merkle::verify_proof(leaf, &proof, leaf_index, root), EPqInvalidProof);

        if (table::contains(&vault.claimed, snapshot_id)) {
            assert!(!table::contains(table::borrow(&vault.claimed, snapshot_id), claimant), EPqAlreadyClaimed);
        };

        let escrow_remaining = table::borrow_mut(&mut vault.snapshot_escrow, snapshot_id);
        *escrow_remaining = *escrow_remaining - amount;

        if (!table::contains(&vault.claimed, snapshot_id)) {
            table::add(&mut vault.claimed, snapshot_id, table::new(ctx));
        };
        let claimed_table = table::borrow_mut(&mut vault.claimed, snapshot_id);
        table::add(claimed_table, claimant, true);

        let (platform_fee, ecosystem_fee, net_amount) = distribute_mydata_marketplace_claim_fees_no_platform(
            config,
            treasury,
            claimant,
            amount,
            &mut vault.balance,
            ctx,
        );

        event::emit(ClaimExecutedEvent {
            snapshot_id,
            claimant,
            gross_amount: amount,
            platform_fee,
            ecosystem_fee,
            net_amount,
            platform_id: option::none(),
            claimed_at: clock::timestamp_ms(clock),
        });
    }

    fun claim_internal_with_platform(
        config: &MyDataConfig,
        dist_registry: &DistributionRegistry,
        vault: &mut MyDataClaimVault,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        snapshot_id: ID,
        amount: u64,
        leaf_index: u64,
        proof: vector<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(amount > 0, EPqInvalidInput);
        assert!(vector::length(&proof) <= config.max_merkle_proof_depth, EPqInvalidProof);
        assert!(table::contains(&dist_registry.rounds, snapshot_id), EPqDistributionNotFound);
        let round = table::borrow(&dist_registry.rounds, snapshot_id);
        assert!(option::is_some(&round.platform_id), EPqPlatformMismatch);
        assert!(clock::timestamp_ms(clock) <= round.claim_deadline_ms, EPqClaimExpired);
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(*option::borrow(&round.platform_id) == platform_id, EPqPlatformMismatch);
        assert!(table::contains(&vault.merkle_roots, snapshot_id), EPqMerkleRootNotPublished);
        assert!(table::contains(&vault.snapshot_escrow, snapshot_id), EPqSnapshotEscrowMissing);
        assert!(*table::borrow(&vault.snapshot_escrow, snapshot_id) >= amount, EPqEscrowExceeded);

        let claimant = tx_context::sender(ctx);
        let leaf = merkle::leaf_hash_with_platform(
            claimant,
            amount,
            object::id_to_bytes(&snapshot_id),
            option::some(platform_id),
        );
        let root = *table::borrow(&vault.merkle_roots, snapshot_id);
        assert!(merkle::verify_proof(leaf, &proof, leaf_index, root), EPqInvalidProof);

        if (table::contains(&vault.claimed, snapshot_id)) {
            assert!(!table::contains(table::borrow(&vault.claimed, snapshot_id), claimant), EPqAlreadyClaimed);
        };

        let escrow_remaining = table::borrow_mut(&mut vault.snapshot_escrow, snapshot_id);
        *escrow_remaining = *escrow_remaining - amount;

        if (!table::contains(&vault.claimed, snapshot_id)) {
            table::add(&mut vault.claimed, snapshot_id, table::new(ctx));
        };
        let claimed_table = table::borrow_mut(&mut vault.claimed, snapshot_id);
        table::add(claimed_table, claimant, true);

        let (platform_fee, ecosystem_fee, net_amount) = distribute_mydata_marketplace_claim_fees_with_platform(
            config,
            treasury,
            claimant,
            amount,
            platform,
            &mut vault.balance,
            clock,
            ctx,
        );

        event::emit(ClaimExecutedEvent {
            snapshot_id,
            claimant,
            gross_amount: amount,
            platform_fee,
            ecosystem_fee,
            net_amount,
            platform_id: option::some(platform_id),
            claimed_at: clock::timestamp_ms(clock),
        });
    }

    /// Claim MyData marketplace pool payout from vault escrow (no platform).
    public entry fun claim(
        config: &MyDataConfig,
        dist_registry: &DistributionRegistry,
        vault: &mut MyDataClaimVault,
        treasury: &EcosystemTreasury,
        snapshot_id: ID,
        amount: u64,
        leaf_index: u64,
        proof: vector<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        claim_internal_no_platform(
            config,
            dist_registry,
            vault,
            treasury,
            snapshot_id,
            amount,
            leaf_index,
            proof,
            clock,
            ctx,
        );
    }

    /// Claim MyData marketplace pool payout with platform treasury routing.
    public entry fun claim_with_platform(
        config: &MyDataConfig,
        dist_registry: &DistributionRegistry,
        vault: &mut MyDataClaimVault,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        snapshot_id: ID,
        amount: u64,
        leaf_index: u64,
        proof: vector<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        claim_internal_with_platform(
            config,
            dist_registry,
            vault,
            treasury,
            platform,
            snapshot_id,
            amount,
            leaf_index,
            proof,
            clock,
            ctx,
        );
    }

    public entry fun reclaim_expired_snapshot_escrow(
        anchor_registry: &SnapshotAnchorRegistry,
        dist_registry: &DistributionRegistry,
        vault: &mut MyDataClaimVault,
        snapshot_id: ID,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(table::contains(&anchor_registry.anchors, snapshot_id), EPqAnchorNotFound);
        assert!(table::contains(&dist_registry.rounds, snapshot_id), EPqDistributionNotFound);
        assert!(table::contains(&vault.snapshot_escrow, snapshot_id), EPqSnapshotEscrowMissing);
        let anchor = table::borrow(&anchor_registry.anchors, snapshot_id);
        assert!(tx_context::sender(ctx) == anchor.buyer_address, EUnauthorized);
        let round = table::borrow(&dist_registry.rounds, snapshot_id);
        let now = clock::timestamp_ms(clock);
        assert!(now > round.claim_deadline_ms, EPqClaimNotExpired);
        let remaining = table::borrow_mut(&mut vault.snapshot_escrow, snapshot_id);
        let amount = *remaining;
        assert!(amount > 0, EPqEscrowExceeded);
        *remaining = 0;
        let refund = coin::from_balance(balance::split(&mut vault.balance, amount), ctx);
        transfer::public_transfer(refund, anchor.buyer_address);
        event::emit(SnapshotEscrowReclaimedEvent {
            snapshot_id,
            buyer_address: anchor.buyer_address,
            amount,
            reclaimed_at: now,
        });
    }

    public fun get_broad_pool(registry: &MyDataPoolRegistry, pool_id: ID): Option<BroadPool> {
        if (table::contains(&registry.broad_pools, pool_id)) {
            option::some(*table::borrow(&registry.broad_pools, pool_id))
        } else {
            option::none()
        }
    }

    public fun get_sub_pool(registry: &MyDataPoolRegistry, sub_pool_id: ID): Option<SubPool> {
        if (table::contains(&registry.sub_pools, sub_pool_id)) {
            option::some(*table::borrow(&registry.sub_pools, sub_pool_id))
        } else {
            option::none()
        }
    }

    public fun get_mydata_sub_pools(registry: &MyDataPoolRegistry, ip_id: address): Option<vector<ID>> {
        if (table::contains(&registry.mydata_to_sub_pools, ip_id)) {
            option::some(*table::borrow(&registry.mydata_to_sub_pools, ip_id))
        } else {
            option::none()
        }
    }

    public fun get_distribution_round(
        registry: &DistributionRegistry,
        snapshot_id: ID,
    ): Option<DistributionRound> {
        if (table::contains(&registry.rounds, snapshot_id)) {
            option::some(*table::borrow(&registry.rounds, snapshot_id))
        } else {
            option::none()
        }
    }

    public fun broad_pool_id(pool: &BroadPool): ID { pool.id }
    public fun sub_pool_id(pool: &SubPool): ID { pool.id }

    fun access_configuration(mydata: &MyData): &AccessConfiguration {
        &mydata.access
    }

    public fun requires_profile_subscription_access(mydata: &MyData): bool {
        match (access_configuration(mydata)) {
            AccessConfiguration::ProfileSubscription => true,
            _ => false,
        }
    }

    public fun requires_marketplace_purchase(mydata: &MyData): bool {
        match (access_configuration(mydata)) {
            AccessConfiguration::MarketplaceOneTime { .. } => true,
            _ => false,
        }
    }

    public fun requires_marketplace_subscription(mydata: &MyData): bool {
        match (access_configuration(mydata)) {
            AccessConfiguration::MarketplaceRecurring { .. } => true,
            _ => false,
        }
    }

    public fun linked_one_time_price(mydata: &MyData): Option<u64> {
        match (access_configuration(mydata)) {
            AccessConfiguration::MarketplaceOneTime { price, .. } => option::some(*price),
            _ => option::none(),
        }
    }

    public fun access_configuration_kind(mydata: &MyData): u8 {
        match (access_configuration(mydata)) {
            AccessConfiguration::ProfileSubscription => ACCESS_KIND_PROFILE,
            AccessConfiguration::MarketplaceOneTime { .. } => ACCESS_KIND_ONE_TIME,
            AccessConfiguration::MarketplaceRecurring { .. } => ACCESS_KIND_RECURRING,
        }
    }

    fun validate_marketplace_price(price: u64) {
        assert!(price > 0 && price <= MAX_U64, EInvalidInput);
    }

    fun validate_recurring_duration(config: &MyDataConfig, duration_days: u64): u64 {
        let sub_duration = if (duration_days == 0) { 30 } else { duration_days };
        assert!(sub_duration <= config.max_subscription_days, EInvalidInput);
        let duration_ms = (sub_duration as u128) * (MILLISECONDS_PER_DAY as u128);
        assert!(duration_ms <= (MAX_U64 as u128), EOverflow);
        sub_duration
    }

    fun validate_access_configuration(config: &MyDataConfig, access: &AccessConfiguration) {
        match (access) {
            AccessConfiguration::ProfileSubscription => {},
            AccessConfiguration::MarketplaceOneTime { price, .. } => validate_marketplace_price(*price),
            AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } => {
                validate_marketplace_price(*price);
                let _ = validate_recurring_duration(config, *duration_days);
            },
        }
    }

    fun validate_optional_metadata(config: &MyDataConfig, value: &Option<String>) {
        if (option::is_some(value)) {
            assert!(string::length(option::borrow(value)) <= config.max_metadata_bytes, EInvalidInput);
        };
    }

    fun validate_tags(config: &MyDataConfig, tags: &vector<String>) {
        assert!(vector::length(tags) <= config.max_tags, EInvalidInput);
        let mut i = 0;
        while (i < vector::length(tags)) {
            assert!(string::length(vector::borrow(tags, i)) <= config.max_tag_bytes, EInvalidInput);
            i = i + 1;
        };
    }

    fun emit_mydata_created_event(mydata: &MyData, ip_id: address) {
        event::emit(MyDataCreatedEvent {
            ip_id,
            owner: mydata.owner,
            media_type: mydata.media_type,
            platform_id: mydata.platform_id,
            access_configuration_kind: access_configuration_kind(mydata),
            created_at: mydata.created_at,
        });
    }

    /// Create new MyData data with proper MyData encryption
    public fun create(
        config: &MyDataConfig,
        media_type: String,
        tags: vector<String>,
        platform_id: Option<address>,
        timestamp_start: u64,
        timestamp_end: Option<u64>,
        encrypted_data: vector<u8>,  // Pre-encrypted data from client
        encryption_id: vector<u8>,   // MyData encryption ID
        access: AccessConfiguration,
        geographic_region: Option<String>,
        data_quality: Option<String>,
        sample_size: Option<u64>,
        collection_method: Option<String>,
        is_updating: bool,
        update_frequency: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ): MyData {
        // Input validation
        validate_tags(config, &tags);
        assert!(string::length(&media_type) > 0 && string::length(&media_type) <= config.max_metadata_bytes, EInvalidInput);
        assert!(!vector::is_empty(&encrypted_data), EInvalidInput);
        assert!(vector::length(&encrypted_data) <= config.max_encrypted_data_bytes, EInvalidInput);
        assert!(!vector::is_empty(&encryption_id), EInvalidInput);
        assert!(vector::length(&encryption_id) <= config.max_encryption_id_bytes, EInvalidInput);
        validate_optional_metadata(config, &geographic_region);
        validate_optional_metadata(config, &data_quality);
        validate_optional_metadata(config, &collection_method);
        validate_optional_metadata(config, &update_frequency);
        validate_access_configuration(config, &access);
        
        // Validate time range
        if (option::is_some(&timestamp_end)) {
            let end_time = *option::borrow(&timestamp_end);
            assert!(end_time >= timestamp_start, EInvalidTimeRange);
        };
        
        let current_time = clock::timestamp_ms(clock);
        
        let mydata = MyData {
            id: object::new(ctx),
            owner: tx_context::sender(ctx),
            media_type,
            tags,
            platform_id,
            timestamp_start,
            timestamp_end,
            created_at: current_time,
            last_updated: current_time,
            encrypted_data,
            encryption_id,
            access,
            geographic_region,
            data_quality,
            sample_size,
            collection_method,
            is_updating,
            update_frequency,
            version: upgrade::current_version(),
        };

        let ip_id = object::uid_to_address(&mydata.id);
        emit_mydata_created_event(&mydata, ip_id);

        mydata
    }

    fun share_created_mydata(registry: &mut MyDataRegistry, mydata: MyData) {
        let ip_id = object::uid_to_address(&mydata.id);
        table::add(&mut registry.ip_to_owner, ip_id, mydata.owner);
        transfer::share_object(mydata);
    }

    fun create_and_share_internal(
        config: &MyDataConfig,
        registry: &mut MyDataRegistry,
        media_type: String,
        tags: vector<String>,
        platform_id: Option<address>,
        timestamp_start: u64,
        timestamp_end: Option<u64>,
        encrypted_data: vector<u8>,
        encryption_id: vector<u8>,
        access: AccessConfiguration,
        geographic_region: Option<String>,
        data_quality: Option<String>,
        sample_size: Option<u64>,
        collection_method: Option<String>,
        is_updating: bool,
        update_frequency: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let mydata = create(
            config,
            media_type,
            tags,
            platform_id,
            timestamp_start,
            timestamp_end,
            encrypted_data,
            encryption_id,
            access,
            geographic_region,
            data_quality,
            sample_size,
            collection_method,
            is_updating,
            update_frequency,
            clock,
            ctx,
        );

        share_created_mydata(registry, mydata);
    }

    /// Create and share profile-subscription-gated MyData (no marketplace pricing).
    #[allow(lint(share_owned))]
    public entry fun create_and_share_profile_subscription_mydata(
        config: &MyDataConfig,
        registry: &mut MyDataRegistry,
        media_type: String,
        tags: vector<String>,
        platform_id: Option<address>,
        timestamp_start: u64,
        timestamp_end: Option<u64>,
        encrypted_data: vector<u8>,
        encryption_id: vector<u8>,
        geographic_region: Option<String>,
        data_quality: Option<String>,
        sample_size: Option<u64>,
        collection_method: Option<String>,
        is_updating: bool,
        update_frequency: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        create_and_share_internal(
            config,
            registry,
            media_type,
            tags,
            platform_id,
            timestamp_start,
            timestamp_end,
            encrypted_data,
            encryption_id,
            AccessConfiguration::ProfileSubscription,
            geographic_region,
            data_quality,
            sample_size,
            collection_method,
            is_updating,
            update_frequency,
            clock,
            ctx,
        );
    }

    /// Create and share marketplace one-time purchase MyData.
    #[allow(lint(share_owned))]
    public entry fun create_and_share_marketplace_one_time_mydata(
        config: &MyDataConfig,
        registry: &mut MyDataRegistry,
        media_type: String,
        tags: vector<String>,
        platform_id: Option<address>,
        timestamp_start: u64,
        timestamp_end: Option<u64>,
        encrypted_data: vector<u8>,
        encryption_id: vector<u8>,
        price: u64,
        geographic_region: Option<String>,
        data_quality: Option<String>,
        sample_size: Option<u64>,
        collection_method: Option<String>,
        is_updating: bool,
        update_frequency: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        validate_marketplace_price(price);
        create_and_share_internal(
            config,
            registry,
            media_type,
            tags,
            platform_id,
            timestamp_start,
            timestamp_end,
            encrypted_data,
            encryption_id,
            AccessConfiguration::MarketplaceOneTime {
                price,
                purchasers: table::new(ctx),
            },
            geographic_region,
            data_quality,
            sample_size,
            collection_method,
            is_updating,
            update_frequency,
            clock,
            ctx,
        );
    }

    /// Create and share marketplace recurring subscription MyData.
    #[allow(lint(share_owned))]
    public entry fun create_and_share_marketplace_recurring_mydata(
        config: &MyDataConfig,
        registry: &mut MyDataRegistry,
        media_type: String,
        tags: vector<String>,
        platform_id: Option<address>,
        timestamp_start: u64,
        timestamp_end: Option<u64>,
        encrypted_data: vector<u8>,
        encryption_id: vector<u8>,
        price: u64,
        duration_days: u64,
        geographic_region: Option<String>,
        data_quality: Option<String>,
        sample_size: Option<u64>,
        collection_method: Option<String>,
        is_updating: bool,
        update_frequency: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        validate_marketplace_price(price);
        let sub_duration = validate_recurring_duration(config, duration_days);
        create_and_share_internal(
            config,
            registry,
            media_type,
            tags,
            platform_id,
            timestamp_start,
            timestamp_end,
            encrypted_data,
            encryption_id,
            AccessConfiguration::MarketplaceRecurring {
                price,
                duration_days: sub_duration,
                subscribers: table::new(ctx),
            },
            geographic_region,
            data_quality,
            sample_size,
            collection_method,
            is_updating,
            update_frequency,
            clock,
            ctx,
        );
    }

    fun purchase_one_time_no_platform(
        config: &MyDataConfig,
        block_list_registry: &block_list::BlockListRegistry,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &mut MyData,
        treasury: &EcosystemTreasury,
        payment: &mut Coin<MYSO>,
        account: &social_contracts::memory::MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);

        let buyer = tx_context::sender(ctx);
        block_list::assert_not_blocked(block_list_registry, buyer, mydata.owner);
        assert!(requires_marketplace_purchase(mydata), ENotForSale);
        let price = match (&mydata.access) {
            AccessConfiguration::MarketplaceOneTime { price, .. } => *price,
            _ => abort ENotForSale,
        };

        let mut sub_agent_id = option::none();
        let mut organization_id = option::none();
        if (social_contracts::memory::is_registered_agent(account, buyer)) {
            let acting = social_contracts::memory::resolve_actor_with_cap(
                memory_config,
                account,
                0,
                mydata.platform_id,
                price,
                clock,
                ctx,
            );
            sub_agent_id = social_contracts::memory::acting_sub_agent_id(&acting);
            organization_id = social_contracts::memory::acting_organization_id(&acting);
        };

        assert!(coin::value(payment) >= price, EPriceMismatch);
        match (&mydata.access) {
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } => {
                assert!(!table::contains(purchasers, buyer), EAlreadyPurchased);
                assert!(table::length(purchasers) < config.max_paid_access_entries, EInvalidInput);
            },
            _ => abort ENotForSale,
        };
        assert!(buyer != mydata.owner, ESelfPurchase);

        let price_coin = coin::split(payment, price, ctx);
        let (platform_fee, ecosystem_fee, creator_amount) = distribute_p2p_fees_no_platform(
            config,
            treasury,
            mydata.owner,
            price_coin,
            ctx,
        );

        match (&mut mydata.access) {
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } => {
                table::add(purchasers, buyer, true);
            },
            _ => abort ENotForSale,
        };

        event::emit(PurchaseEvent {
            ip_id: object::uid_to_address(&mydata.id),
            buyer,
            price,
            purchase_type: string::utf8(b"one_time"),
            timestamp: clock::timestamp_ms(clock),
            sub_agent_id,
            organization_id,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            platform_id: mydata.platform_id,
        });
    }

    fun purchase_one_time_with_platform_internal(
        config: &MyDataConfig,
        block_list_registry: &block_list::BlockListRegistry,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &mut MyData,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: &mut Coin<MYSO>,
        account: &social_contracts::memory::MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        assert_platform_matches_listing(mydata, platform);

        let buyer = tx_context::sender(ctx);
        block_list::assert_not_blocked(block_list_registry, buyer, mydata.owner);
        assert!(requires_marketplace_purchase(mydata), ENotForSale);
        let price = match (&mydata.access) {
            AccessConfiguration::MarketplaceOneTime { price, .. } => *price,
            _ => abort ENotForSale,
        };

        let mut sub_agent_id = option::none();
        let mut organization_id = option::none();
        if (social_contracts::memory::is_registered_agent(account, buyer)) {
            let acting = social_contracts::memory::resolve_actor_with_cap(
                memory_config,
                account,
                0,
                mydata.platform_id,
                price,
                clock,
                ctx,
            );
            sub_agent_id = social_contracts::memory::acting_sub_agent_id(&acting);
            organization_id = social_contracts::memory::acting_organization_id(&acting);
        };

        assert!(coin::value(payment) >= price, EPriceMismatch);
        match (&mydata.access) {
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } => {
                assert!(!table::contains(purchasers, buyer), EAlreadyPurchased);
                assert!(table::length(purchasers) < config.max_paid_access_entries, EInvalidInput);
            },
            _ => abort ENotForSale,
        };
        assert!(buyer != mydata.owner, ESelfPurchase);

        let price_coin = coin::split(payment, price, ctx);
        let (platform_fee, ecosystem_fee, creator_amount) = distribute_p2p_fees_with_platform(
            config,
            treasury,
            mydata.owner,
            price_coin,
            platform,
            clock,
            ctx,
        );

        match (&mut mydata.access) {
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } => {
                table::add(purchasers, buyer, true);
            },
            _ => abort ENotForSale,
        };

        event::emit(PurchaseEvent {
            ip_id: object::uid_to_address(&mydata.id),
            buyer,
            price,
            purchase_type: string::utf8(b"one_time"),
            timestamp: clock::timestamp_ms(clock),
            sub_agent_id,
            organization_id,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            platform_id: option::some(object::uid_to_address(platform::id(platform))),
        });
    }

    /// Purchase one-time access to MyData data.
    public entry fun purchase_one_time(
        config: &MyDataConfig,
        block_list_registry: &block_list::BlockListRegistry,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &mut MyData,
        treasury: &EcosystemTreasury,
        payment: &mut Coin<MYSO>,
        account: &social_contracts::memory::MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        purchase_one_time_no_platform(
            config,
            block_list_registry,
            memory_config,
            mydata,
            treasury,
            payment,
            account,
            clock,
            ctx,
        );
    }

    /// Purchase one-time access with platform treasury routing.
    public entry fun purchase_one_time_with_platform(
        config: &MyDataConfig,
        block_list_registry: &block_list::BlockListRegistry,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &mut MyData,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: &mut Coin<MYSO>,
        account: &social_contracts::memory::MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        purchase_one_time_with_platform_internal(
            config,
            block_list_registry,
            memory_config,
            mydata,
            treasury,
            platform,
            payment,
            account,
            clock,
            ctx,
        );
    }

    fun purchase_subscription_no_platform(
        config: &MyDataConfig,
        block_list_registry: &block_list::BlockListRegistry,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &mut MyData,
        treasury: &EcosystemTreasury,
        payment: &mut Coin<MYSO>,
        account: &social_contracts::memory::MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);

        let buyer = tx_context::sender(ctx);
        block_list::assert_not_blocked(block_list_registry, buyer, mydata.owner);
        assert!(requires_marketplace_subscription(mydata), ENotForSale);
        let (price, duration_days) = match (&mydata.access) {
            AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } => (*price, *duration_days),
            _ => abort ENotForSale,
        };

        let mut sub_agent_id = option::none();
        let mut organization_id = option::none();
        if (social_contracts::memory::is_registered_agent(account, buyer)) {
            let acting = social_contracts::memory::resolve_actor_with_cap(
                memory_config,
                account,
                0,
                mydata.platform_id,
                price,
                clock,
                ctx,
            );
            sub_agent_id = social_contracts::memory::acting_sub_agent_id(&acting);
            organization_id = social_contracts::memory::acting_organization_id(&acting);
        };

        assert!(coin::value(payment) >= price, EPriceMismatch);
        assert!(buyer != mydata.owner, ESelfPurchase);
        assert!(duration_days > 0, EInvalidInput);
        assert!(duration_days <= config.max_subscription_days, EInvalidInput);

        let current_time = clock::timestamp_ms(clock);
        let duration_ms = (duration_days as u128) * (MILLISECONDS_PER_DAY as u128);
        let expiry_time = (current_time as u128) + duration_ms;
        assert!(expiry_time <= (MAX_U64 as u128), EOverflow);
        let expiry_time_u64 = expiry_time as u64;

        let price_coin = coin::split(payment, price, ctx);
        let (platform_fee, ecosystem_fee, creator_amount) = distribute_p2p_fees_no_platform(
            config,
            treasury,
            mydata.owner,
            price_coin,
            ctx,
        );

        match (&mut mydata.access) {
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } => {
                if (table::contains(subscribers, buyer)) {
                    let current_expiry = table::remove(subscribers, buyer);
                    let new_expiry = if (current_expiry > current_time) {
                        let extended_time = (current_expiry as u128) + duration_ms;
                        assert!(extended_time <= (MAX_U64 as u128), EOverflow);
                        extended_time as u64
                    } else {
                        expiry_time_u64
                    };
                    table::add(subscribers, buyer, new_expiry);
                } else {
                    assert!(table::length(subscribers) < config.max_paid_access_entries, EInvalidInput);
                    table::add(subscribers, buyer, expiry_time_u64);
                };
            },
            _ => abort ENotForSale,
        };

        event::emit(PurchaseEvent {
            ip_id: object::uid_to_address(&mydata.id),
            buyer,
            price,
            purchase_type: string::utf8(b"subscription"),
            timestamp: clock::timestamp_ms(clock),
            sub_agent_id,
            organization_id,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            platform_id: mydata.platform_id,
        });
    }

    fun purchase_subscription_with_platform_internal(
        config: &MyDataConfig,
        block_list_registry: &block_list::BlockListRegistry,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &mut MyData,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: &mut Coin<MYSO>,
        account: &social_contracts::memory::MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        assert_platform_matches_listing(mydata, platform);

        let buyer = tx_context::sender(ctx);
        block_list::assert_not_blocked(block_list_registry, buyer, mydata.owner);
        assert!(requires_marketplace_subscription(mydata), ENotForSale);
        let (price, duration_days) = match (&mydata.access) {
            AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } => (*price, *duration_days),
            _ => abort ENotForSale,
        };

        let mut sub_agent_id = option::none();
        let mut organization_id = option::none();
        if (social_contracts::memory::is_registered_agent(account, buyer)) {
            let acting = social_contracts::memory::resolve_actor_with_cap(
                memory_config,
                account,
                0,
                mydata.platform_id,
                price,
                clock,
                ctx,
            );
            sub_agent_id = social_contracts::memory::acting_sub_agent_id(&acting);
            organization_id = social_contracts::memory::acting_organization_id(&acting);
        };

        assert!(coin::value(payment) >= price, EPriceMismatch);
        assert!(buyer != mydata.owner, ESelfPurchase);
        assert!(duration_days > 0, EInvalidInput);
        assert!(duration_days <= config.max_subscription_days, EInvalidInput);

        let current_time = clock::timestamp_ms(clock);
        let duration_ms = (duration_days as u128) * (MILLISECONDS_PER_DAY as u128);
        let expiry_time = (current_time as u128) + duration_ms;
        assert!(expiry_time <= (MAX_U64 as u128), EOverflow);
        let expiry_time_u64 = expiry_time as u64;

        let price_coin = coin::split(payment, price, ctx);
        let (platform_fee, ecosystem_fee, creator_amount) = distribute_p2p_fees_with_platform(
            config,
            treasury,
            mydata.owner,
            price_coin,
            platform,
            clock,
            ctx,
        );

        match (&mut mydata.access) {
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } => {
                if (table::contains(subscribers, buyer)) {
                    let current_expiry = table::remove(subscribers, buyer);
                    let new_expiry = if (current_expiry > current_time) {
                        let extended_time = (current_expiry as u128) + duration_ms;
                        assert!(extended_time <= (MAX_U64 as u128), EOverflow);
                        extended_time as u64
                    } else {
                        expiry_time_u64
                    };
                    table::add(subscribers, buyer, new_expiry);
                } else {
                    assert!(table::length(subscribers) < config.max_paid_access_entries, EInvalidInput);
                    table::add(subscribers, buyer, expiry_time_u64);
                };
            },
            _ => abort ENotForSale,
        };

        event::emit(PurchaseEvent {
            ip_id: object::uid_to_address(&mydata.id),
            buyer,
            price,
            purchase_type: string::utf8(b"subscription"),
            timestamp: clock::timestamp_ms(clock),
            sub_agent_id,
            organization_id,
            platform_fee,
            ecosystem_fee,
            creator_amount,
            platform_id: option::some(object::uid_to_address(platform::id(platform))),
        });
    }

    /// Purchase subscription access to MyData data.
    public entry fun purchase_subscription(
        config: &MyDataConfig,
        block_list_registry: &block_list::BlockListRegistry,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &mut MyData,
        treasury: &EcosystemTreasury,
        payment: &mut Coin<MYSO>,
        account: &social_contracts::memory::MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        purchase_subscription_no_platform(
            config,
            block_list_registry,
            memory_config,
            mydata,
            treasury,
            payment,
            account,
            clock,
            ctx,
        );
    }

    /// Purchase subscription access with platform treasury routing.
    public entry fun purchase_subscription_with_platform(
        config: &MyDataConfig,
        block_list_registry: &block_list::BlockListRegistry,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &mut MyData,
        treasury: &EcosystemTreasury,
        platform: &mut Platform,
        payment: &mut Coin<MYSO>,
        account: &social_contracts::memory::MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        purchase_subscription_with_platform_internal(
            config,
            block_list_registry,
            memory_config,
            mydata,
            treasury,
            platform,
            payment,
            account,
            clock,
            ctx,
        );
    }

    /// Update pricing (owner only; marketplace listings only).
    public entry fun update_pricing(
        config: &MyDataConfig,
        mydata: &mut MyData,
        new_one_time_price: Option<u64>,
        new_subscription_price: Option<u64>,
        new_subscription_duration_days: Option<u64>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);

        match (&mut mydata.access) {
            AccessConfiguration::MarketplaceOneTime { price, .. } => {
                assert!(option::is_some(&new_one_time_price), EInvalidInput);
                let price_val = *option::borrow(&new_one_time_price);
                validate_marketplace_price(price_val);
                *price = price_val;
            },
            AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } => {
                assert!(option::is_some(&new_subscription_price) || option::is_some(&new_subscription_duration_days), EInvalidInput);
                if (option::is_some(&new_subscription_price)) {
                    let price_val = *option::borrow(&new_subscription_price);
                    validate_marketplace_price(price_val);
                    *price = price_val;
                };
                if (option::is_some(&new_subscription_duration_days)) {
                    let duration = *option::borrow(&new_subscription_duration_days);
                    *duration_days = validate_recurring_duration(config, duration);
                };
            },
            AccessConfiguration::ProfileSubscription => abort ENotForSale,
        };

        let (one_time_price, subscription_price, subscription_duration_days) = match (&mydata.access) {
            AccessConfiguration::ProfileSubscription => (option::none(), option::none(), option::none()),
            AccessConfiguration::MarketplaceOneTime { price, .. } => (option::some(*price), option::none(), option::none()),
            AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } => {
                (option::none(), option::some(*price), option::some(*duration_days))
            },
        };
        event::emit(MyDataPricingUpdatedEvent {
            ip_id: object::uid_to_address(&mydata.id),
            one_time_price,
            subscription_price,
            subscription_duration_days,
            updated_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Update MyData content and metadata (owner only)
    public entry fun update_content(
        config: &MyDataConfig,
        mydata: &mut MyData,
        new_encrypted_data: Option<vector<u8>>,
        new_encryption_id: Option<vector<u8>>,
        new_tags: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        // Check version compatibility
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        let encrypted_data_updated = option::is_some(&new_encrypted_data);
        let encryption_id_updated = option::is_some(&new_encryption_id);
        let tags_updated = option::is_some(&new_tags);
        assert!(encrypted_data_updated == encryption_id_updated, EInvalidInput);
        assert!(encrypted_data_updated || tags_updated, EInvalidInput);

        if (encrypted_data_updated) {
            let data = option::borrow(&new_encrypted_data);
            let encryption_id = option::borrow(&new_encryption_id);
            assert!(!vector::is_empty(data) && vector::length(data) <= config.max_encrypted_data_bytes, EInvalidInput);
            assert!(!vector::is_empty(encryption_id) && vector::length(encryption_id) <= config.max_encryption_id_bytes, EInvalidInput);
            mydata.encrypted_data = *option::borrow(&new_encrypted_data);
            mydata.encryption_id = *option::borrow(&new_encryption_id);
        };
        
        if (tags_updated) {
            validate_tags(config, option::borrow(&new_tags));
            mydata.tags = *option::borrow(&new_tags);
        };
        
        mydata.last_updated = clock::timestamp_ms(clock);

        event::emit(MyDataContentUpdatedEvent {
            ip_id: object::uid_to_address(&mydata.id),
            encrypted_data_updated,
            tags_updated,
            updated_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Assign MyData to sub-pools (owner only).
    public entry fun assign_mydata_to_pools(
        config: &MyDataConfig,
        mydata: &MyData,
        pool_registry: &mut MyDataPoolRegistry,
        sub_pool_ids: vector<ID>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        let ip_id = object::uid_to_address(&mydata.id);
        assign_mydata_to_sub_pools(config, pool_registry, ip_id, sub_pool_ids, clock);
    }

    /// Remove this listing from a sub-pool (owner only).
    public entry fun remove_mydata_from_sub_pools(
        mydata: &MyData,
        pool_registry: &mut MyDataPoolRegistry,
        sub_pool_id: ID,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        let ip_id = object::uid_to_address(&mydata.id);
        remove_mydata_from_sub_pool(pool_registry, ip_id, sub_pool_id);
    }

    /// Check if user has access to MyData data
    public fun has_access(mydata: &MyData, user: address, clock: &Clock): bool {
        if (user == mydata.owner) return true;

        match (&mydata.access) {
            AccessConfiguration::ProfileSubscription => false,
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } => {
                table::contains(purchasers, user)
            },
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } => {
                if (!table::contains(subscribers, user)) return false;
                let expiry = *table::borrow(subscribers, user);
                let current_time = clock::timestamp_ms(clock);
                current_time <= expiry
            },
        }
    }

    /// True if `candidate` matches this listing’s `encryption_id` (the MyData policy `id` bytes).
    public fun encryption_id_matches(mydata: &MyData, candidate: &vector<u8>): bool {
        bytes_equal_u8(&mydata.encryption_id, candidate)
    }

    /// Key-server policy hook for `fetch_key`: marketplace listings only.
    public entry fun mydata_approve(
        id: vector<u8>,
        block_list_registry: &block_list::BlockListRegistry,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &MyData,
        account: &social_contracts::memory::MemoryAccount,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert!(encryption_id_matches(mydata, &id), EPolicyIdMismatch);
        assert!(
            requires_marketplace_purchase(mydata) || requires_marketplace_subscription(mydata),
            EPolicyNotEntitled,
        );

        let sender = tx_context::sender(ctx);
        if (sender != mydata.owner) {
            block_list::assert_not_blocked(block_list_registry, sender, mydata.owner);
        };
        if (has_access(mydata, sender, clock)) {
            return
        };

        assert!(
            social_contracts::memory::owner(account) == mydata.owner,
            EPolicyNotEntitled,
        );

        if (!social_contracts::memory::is_registered_agent(account, sender)) {
            abort EPolicyNotEntitled
        };

        let acting = social_contracts::memory::resolve_actor_with_cap(
            memory_config,
            account,
            social_contracts::memory::cap_mydata_read(),
            mydata.platform_id,
            0,
            clock,
            ctx,
        );
        assert!(
            social_contracts::memory::acting_principal_owner(&acting) == mydata.owner,
            EPolicyNotEntitled,
        );
    }

    /// Key-server policy hook for profile-subscription-gated MyData linked to a post.
    /// `id` is first so key-server `ValidPtb` can extract the encryption identity from arg 0.
    /// `post_service_id` / `post_linked_mydata_id` come from the linked post's [`PostAccess`] fields.
    public entry fun mydata_approve_profile_subscription(
        id: vector<u8>,
        block_list_registry: &block_list::BlockListRegistry,
        post_service_id: ID,
        post_linked_mydata_id: ID,
        post_min_tier_level: Option<u64>,
        memory_config: &social_contracts::memory::MemoryConfig,
        mydata: &MyData,
        account: &social_contracts::memory::MemoryAccount,
        service: &ProfileSubscriptionService,
        subscription: &ProfileSubscription,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert!(encryption_id_matches(mydata, &id), EPolicyIdMismatch);
        assert!(requires_profile_subscription_access(mydata), EPolicyNotEntitled);
        assert!(post_linked_mydata_id == object::id(mydata), EPolicyNotEntitled);
        assert!(post_service_id == object::id(service), EPolicyNotEntitled);

        let sender = tx_context::sender(ctx);
        if (sender == mydata.owner) {
            return
        };

        block_list::assert_not_blocked(block_list_registry, sender, mydata.owner);

        let content_platform_id = mydata.platform_id;
        if (subscription::subscription_satisfies_access(
            subscription,
            service,
            sender,
            post_min_tier_level,
            content_platform_id,
            clock,
        )) {
            return
        };

        assert!(
            social_contracts::memory::owner(account) == mydata.owner,
            EPolicyNotEntitled,
        );

        if (!social_contracts::memory::is_registered_agent(account, sender)) {
            abort EPolicyNotEntitled
        };

        let acting = social_contracts::memory::resolve_actor_with_cap(
            memory_config,
            account,
            social_contracts::memory::cap_mydata_read(),
            mydata.platform_id,
            0,
            clock,
            ctx,
        );
        assert!(
            social_contracts::memory::acting_principal_owner(&acting) == mydata.owner,
            EPolicyNotEntitled,
        );
    }

    fun bytes_equal_u8(a: &vector<u8>, b: &vector<u8>): bool {
        if (vector::length(a) != vector::length(b)) {
            return false
        };
        let len = vector::length(a);
        let mut i = 0;
        while (i < len) {
            if (*vector::borrow(a, i) != *vector::borrow(b, i)) {
                return false
            };
            i = i + 1;
        };
        true
    }

    /// Grant free access (owner only) - useful for samples or promotions
    public entry fun grant_access(
        config: &MyDataConfig,
        mydata: &mut MyData,
        user: address,
        access_type: u8, // 0 = one-time, 1 = subscription
        subscription_days: Option<u64>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        // Check version compatibility
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        assert!(user != mydata.owner, ESelfPurchase);

        let total_grants = match (&mydata.access) {
            AccessConfiguration::ProfileSubscription => abort ENotForSale,
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } => table::length(purchasers),
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } => table::length(subscribers),
        };
        assert!(total_grants < config.max_free_access_grants, EInvalidInput);
        
        if (access_type == 0) {
            match (&mut mydata.access) {
                AccessConfiguration::MarketplaceOneTime { purchasers, .. } => {
                    if (!table::contains(purchasers, user)) {
                        table::add(purchasers, user, true);
                    };
                },
                _ => abort ENotForSale,
            };
        } else {
            let duration_days = if (option::is_some(&subscription_days)) {
                let days = *option::borrow(&subscription_days);
                assert!(days > 0 && days <= config.max_subscription_days, EInvalidInput);
                days
            } else {
                match (&mydata.access) {
                    AccessConfiguration::MarketplaceRecurring { duration_days, .. } => *duration_days,
                    _ => abort ENotForSale,
                }
            };
            
            let current_time = clock::timestamp_ms(clock);
            let duration_ms = (duration_days as u128) * (MILLISECONDS_PER_DAY as u128);
            let expiry_time = (current_time as u128) + duration_ms;
            assert!(expiry_time <= (MAX_U64 as u128), EOverflow);
            let expiry_time_u64 = expiry_time as u64;
            
            match (&mut mydata.access) {
                AccessConfiguration::MarketplaceRecurring { subscribers, .. } => {
                    if (table::contains(subscribers, user)) {
                        table::remove(subscribers, user);
                    };
                    table::add(subscribers, user, expiry_time_u64);
                },
                _ => abort ENotForSale,
            };
        };

        event::emit(AccessGrantedEvent {
            ip_id: object::uid_to_address(&mydata.id),
            user,
            access_type: if (access_type == 0) { string::utf8(b"one_time") } else { string::utf8(b"subscription") },
            granted_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Revoke a buyer's access (owner only). Removes the user from marketplace access tables.
    /// `access_type`: 0 = one-time, 1 = subscription, 2 = both.
    public entry fun revoke_access(
        mydata: &mut MyData,
        user: address,
        access_type: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        assert!(user != mydata.owner, EInvalidInput);
        assert!(access_type <= 2, EInvalidInput);

        let mut revoked_one_time = false;
        let mut revoked_subscription = false;

        if (access_type == 0 || access_type == 2) {
            match (&mut mydata.access) {
                AccessConfiguration::MarketplaceOneTime { purchasers, .. } => {
                    if (table::contains(purchasers, user)) {
                        table::remove(purchasers, user);
                        revoked_one_time = true;
                    };
                },
                _ => {},
            };
        };

        if (access_type == 1 || access_type == 2) {
            match (&mut mydata.access) {
                AccessConfiguration::MarketplaceRecurring { subscribers, .. } => {
                    if (table::contains(subscribers, user)) {
                        table::remove(subscribers, user);
                        revoked_subscription = true;
                    };
                },
                _ => {},
            };
        };

        assert!(revoked_one_time || revoked_subscription, ENoAccessToRevoke);

        let access_type_str = if (revoked_one_time && revoked_subscription) {
            string::utf8(b"all")
        } else if (revoked_one_time) {
            string::utf8(b"one_time")
        } else {
            string::utf8(b"subscription")
        };

        event::emit(AccessRevokedEvent {
            ip_id: object::uid_to_address(&mydata.id),
            user,
            access_type: access_type_str,
            revoked_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    // === Getter Functions ===
    
    public fun owner(mydata: &MyData): address { mydata.owner }
    public fun object_address(mydata: &MyData): address { object::uid_to_address(&mydata.id) }
    /// Listing object address for PTB binding in `fetch_key` policy transactions.
    public fun listing_id(mydata: &MyData): address { object::uid_to_address(&mydata.id) }
    /// Encryption identity bytes; must match `EncryptedObject.id` and the `id` arg to `mydata_approve`.
    public fun encryption_identity(mydata: &MyData): vector<u8> { mydata.encryption_id }
    public fun media_type(mydata: &MyData): String { mydata.media_type }
    public fun tags(mydata: &MyData): vector<String> { mydata.tags }
    public fun platform_id(mydata: &MyData): Option<address> { mydata.platform_id }
    public fun one_time_price(mydata: &MyData): Option<u64> { linked_one_time_price(mydata) }
    public fun subscription_price(mydata: &MyData): Option<u64> {
        match (access_configuration(mydata)) {
            AccessConfiguration::MarketplaceRecurring { price, .. } => option::some(*price),
            _ => option::none(),
        }
    }
    public fun subscription_duration_days(mydata: &MyData): u64 {
        match (access_configuration(mydata)) {
            AccessConfiguration::MarketplaceRecurring { duration_days, .. } => *duration_days,
            _ => 0,
        }
    }
    public fun created_at(mydata: &MyData): u64 { mydata.created_at }
    public fun last_updated(mydata: &MyData): u64 { mydata.last_updated }
    public fun timestamp_start(mydata: &MyData): u64 { mydata.timestamp_start }
    public fun timestamp_end(mydata: &MyData): Option<u64> { mydata.timestamp_end }
    public fun geographic_region(mydata: &MyData): Option<String> { mydata.geographic_region }
    public fun data_quality(mydata: &MyData): Option<String> { mydata.data_quality }
    public fun sample_size(mydata: &MyData): Option<u64> { mydata.sample_size }
    public fun collection_method(mydata: &MyData): Option<String> { mydata.collection_method }
    public fun is_updating(mydata: &MyData): bool { mydata.is_updating }
    public fun update_frequency(mydata: &MyData): Option<String> { mydata.update_frequency }
    public fun purchaser_count(mydata: &MyData): u64 {
        match (access_configuration(mydata)) {
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } => table::length(purchasers),
            _ => 0,
        }
    }
    public fun subscriber_count(mydata: &MyData): u64 {
        match (access_configuration(mydata)) {
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } => table::length(subscribers),
            _ => 0,
        }
    }
    public fun is_one_time_for_sale(mydata: &MyData): bool { requires_marketplace_purchase(mydata) }
    public fun is_subscription_available(mydata: &MyData): bool { requires_marketplace_subscription(mydata) }

    /// Check if a user has an active subscription
    public fun has_active_subscription(mydata: &MyData, user: address, clock: &Clock): bool {
        match (&mydata.access) {
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } => {
                if (!table::contains(subscribers, user)) return false;
                let expiry = *table::borrow(subscribers, user);
                let current_time = clock::timestamp_ms(clock);
                current_time <= expiry
            },
            _ => false,
        }
    }

    /// Get subscription expiry time for a user
    public fun get_subscription_expiry(mydata: &MyData, user: address): Option<u64> {
        match (&mydata.access) {
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } => {
                if (table::contains(subscribers, user)) {
                    option::some(*table::borrow(subscribers, user))
                } else {
                    option::none()
                }
            },
            _ => option::none(),
        }
    }

    /// Get total revenue potential (for analytics) with overflow protection
    public fun get_revenue_potential(mydata: &MyData): u64 {
        let one_time_revenue = match (access_configuration(mydata)) {
            AccessConfiguration::MarketplaceOneTime { price, purchasers, .. } => {
                let count = table::length(purchasers);
                let revenue = (*price as u128) * (count as u128);
                if (revenue > (MAX_U64 as u128)) {
                    MAX_U64
                } else {
                    revenue as u64
                }
            },
            _ => 0,
        };
        
        let subscription_revenue = match (access_configuration(mydata)) {
            AccessConfiguration::MarketplaceRecurring { price, subscribers, .. } => {
                let count = table::length(subscribers);
                let revenue = (*price as u128) * (count as u128);
                if (revenue > (MAX_U64 as u128)) {
                    MAX_U64
                } else {
                    revenue as u64
                }
            },
            _ => 0,
        };
        
        // Safe addition with overflow protection
        let total_revenue = (one_time_revenue as u128) + (subscription_revenue as u128);
        if (total_revenue > (MAX_U64 as u128)) {
            MAX_U64
        } else {
            total_revenue as u64
        }
    }

    /// Check if MyData has any sales (one-time or subscription)
    public fun has_any_sales(mydata: &MyData): bool {
        purchaser_count(mydata) > 0 || subscriber_count(mydata) > 0
    }

    // === Registry Functions ===
    
    /// Get owner of a MyData by ID
    public fun registry_get_owner(registry: &MyDataRegistry, ip_id: address): Option<address> {
        if (table::contains(&registry.ip_to_owner, ip_id)) {
            option::some(*table::borrow(&registry.ip_to_owner, ip_id))
        } else {
            option::none()
        }
    }

    /// Check if a MyData is registered
    public fun is_registered(registry: &MyDataRegistry, ip_id: address): bool {
        table::contains(&registry.ip_to_owner, ip_id)
    }

    /// Register a MyData in the registry
    public entry fun register_in_registry(
        registry: &mut MyDataRegistry,
        mydata: &MyData,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EInvalidInput);
        
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        let ip_id = object::uid_to_address(&mydata.id);
        
        if (!table::contains(&registry.ip_to_owner, ip_id)) {
            table::add(&mut registry.ip_to_owner, ip_id, mydata.owner);
            
            // Emit registration event
            event::emit(MyDataRegisteredEvent {
                ip_id,
                owner: mydata.owner,
                registered_at: clock::timestamp_ms(clock),
            });
        };
    }

    /// Remove a MyData from the registry
    public entry fun unregister_from_registry(
        registry: &mut MyDataRegistry,
        ip_id: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EInvalidInput);
        
        if (table::contains(&registry.ip_to_owner, ip_id)) {
            let owner = *table::borrow(&registry.ip_to_owner, ip_id);
            assert!(tx_context::sender(ctx) == owner, EUnauthorized);
            table::remove(&mut registry.ip_to_owner, ip_id);
            
            // Emit unregistration event
            event::emit(MyDataUnregisteredEvent {
                ip_id,
                owner,
                unregistered_at: clock::timestamp_ms(clock),
            });
        };
    }

    // === Versioning Functions ===
    
    public fun version(mydata: &MyData): u64 {
        mydata.version
    }

    public(package) fun borrow_version_mut(mydata: &mut MyData): &mut u64 {
        &mut mydata.version
    }

    public fun registry_version(registry: &MyDataRegistry): u64 {
        registry.version
    }

    public(package) fun borrow_registry_version_mut(registry: &mut MyDataRegistry): &mut u64 {
        &mut registry.version
    }

    /// Migration function for MyData
    public entry fun migrate_mydata(
        mydata: &mut MyData,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        assert!(mydata.version < current_version, EInvalidInput);
        
        let old_version = mydata.version;
        mydata.version = current_version;
        
        let mydata_id = object::id(mydata);
        upgrade::emit_migration_event(
            mydata_id,
            string::utf8(b"MyData"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Migration function for MyDataRegistry
    public entry fun migrate_registry(
        registry: &mut MyDataRegistry,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        assert!(registry.version < current_version, EInvalidInput);
        
        let old_version = registry.version;
        registry.version = current_version;
        
        let registry_id = object::id(registry);
        upgrade::emit_migration_event(
            registry_id,
            string::utf8(b"MyDataRegistry"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Migration function for MyDataConfig
    public entry fun migrate_config(
        config: &mut MyDataConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(config.version < current_version, EInvalidInput);
        
        // Remember old version and update to new version
        let old_version = config.version;
        config.max_encrypted_data_bytes = DEFAULT_MAX_ENCRYPTED_DATA_BYTES;
        config.max_tag_bytes = DEFAULT_MAX_TAG_BYTES;
        config.max_metadata_bytes = DEFAULT_MAX_METADATA_BYTES;
        config.max_payment_reference_bytes = DEFAULT_MAX_PAYMENT_REFERENCE_BYTES;
        config.max_pool_assignments = DEFAULT_MAX_POOL_ASSIGNMENTS;
        config.max_merkle_proof_depth = DEFAULT_MAX_MERKLE_PROOF_DEPTH;
        config.max_paid_access_entries = DEFAULT_MAX_PAID_ACCESS_ENTRIES;
        config.default_claim_window_ms = DEFAULT_CLAIM_WINDOW_MS;
        config.p2p_platform_fee_bps = DEFAULT_P2P_PLATFORM_FEE_BPS;
        config.p2p_ecosystem_fee_bps = DEFAULT_P2P_ECOSYSTEM_FEE_BPS;
        config.mydata_marketplace_platform_fee_bps = DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS;
        config.mydata_marketplace_ecosystem_fee_bps = DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS;
        config.non_platform_platform_to_creator_bps = DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS;
        config.non_platform_platform_to_treasury_bps = DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS;
        config.version = current_version;
        
        // Emit event for object migration
        let config_id = object::id(config);
        upgrade::emit_migration_event(
            config_id,
            string::utf8(b"MyDataConfig"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    // === Test Functions ===

    #[test_only]
    public fun last_created_pool_id(registry: &MyDataPoolRegistry): ID {
        assert!(option::is_some(&registry.last_created_broad_pool_id), EPqInvalidInput);
        *option::borrow(&registry.last_created_broad_pool_id)
    }

    #[test_only]
    public fun last_created_sub_pool_id(registry: &MyDataPoolRegistry): ID {
        assert!(option::is_some(&registry.last_created_sub_pool_id), EPqInvalidInput);
        *option::borrow(&registry.last_created_sub_pool_id)
    }

    #[test_only]
    public fun p2p_fee_breakdown_no_platform_for_testing(config: &MyDataConfig, gross: u64): (u64, u64, u64) {
        calculate_p2p_fees_no_platform(config, gross)
    }

    #[test_only]
    public fun p2p_fee_breakdown_with_platform_for_testing(config: &MyDataConfig, gross: u64): (u64, u64, u64) {
        calculate_p2p_fees_with_platform(config, gross)
    }

    #[test_only]
    public fun mydata_marketplace_fee_breakdown_no_platform_for_testing(
        config: &MyDataConfig,
        gross: u64,
    ): (u64, u64, u64) {
        calculate_mydata_marketplace_fees_no_platform(config, gross)
    }

    #[test_only]
    public fun mydata_marketplace_fee_breakdown_with_platform_for_testing(
        config: &MyDataConfig,
        gross: u64,
    ): (u64, u64, u64) {
        calculate_mydata_marketplace_fees_with_platform(config, gross)
    }

    #[test_only]
    public fun test_init(clock: &Clock, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        transfer::public_transfer(MyDataAdminCap { id: object::new(ctx) }, sender);
        transfer::public_transfer(create_mydata_pool_admin_cap(ctx), sender);
        // Keep the query/pool marketplace disabled so direct-listing tests prove that
        // profile-gated, one-time, and recurring MyData do not depend on this flag.
        share_mydata_system_objects(clock, ctx, false);
    }

    #[test_only]
    public fun set_marketplace_enabled_for_testing(config: &mut MyDataConfig, enabled: bool) {
        config.marketplace_enabled = enabled;
    }

    #[test_only]
    public fun last_snapshot_id_for_testing(registry: &SnapshotAnchorRegistry): ID {
        assert!(registry.next_snapshot_nonce > 0, EPqAnchorNotFound);
        gen_snapshot_id(&registry.id, registry.next_snapshot_nonce - 1)
    }

    #[test_only]
    public fun test_destroy(mydata: MyData) {
        let MyData { 
            id, owner: _, media_type: _, tags: _, platform_id: _,
            timestamp_start: _, timestamp_end: _, created_at: _, last_updated: _,
            encrypted_data: _, encryption_id: _, access,
            geographic_region: _, data_quality: _, sample_size: _, collection_method: _,
            is_updating: _, update_frequency: _, version: _
        } = mydata;
        match (access) {
            AccessConfiguration::ProfileSubscription => {},
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } => table::drop(purchasers),
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } => table::drop(subscribers),
        };
        object::delete(id);
    }
}
