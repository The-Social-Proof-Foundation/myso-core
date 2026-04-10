// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Universal MyData module for encrypted data monetization (one-time purchase, subscription, owner vault).
///
/// **Production decrypt path (client-only):** Plaintext must only exist off-chain. Callers encrypt before
/// `create` / `create_and_share`, then authorized users use the MyData SDK: resolve access (indexer or
/// `has_access`), request keys via the key server (`fetch_key`) with policy approval (`mydata_approve`),
/// and decrypt locally. Do not rely on Move to produce user content plaintext for marketplace listings.
///
/// **On-chain state:** `encrypted_data` is opaque ciphertext. For BF-HMAC MyData blobs,
/// `mydata::bf_hmac_encryption::EncryptedObject` embeds `package_id` and `id`; the `encryption_id` field
/// must be the same `id` bytes used when encrypting so policy and clients stay aligned. For client-only
/// AES-GCM (or other app-managed schemes), ciphertext does not parse as `EncryptedObject`; encode the
/// scheme in `media_type` (e.g. prefix `aes_gcm:`) or app metadata so indexers pick the right decrypt path.
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

    // === Default constants for config initialization ===
    const DEFAULT_ENABLE: bool = false;

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

    // === Constants ===
    const MAX_TAGS: u64 = 10;
    const MAX_SUBSCRIPTION_DAYS: u64 = 365;
    const MILLISECONDS_PER_DAY: u64 = 86_400_000;
    const MAX_FREE_ACCESS_GRANTS: u64 = 100_000; // Limit free access to 100k users
    const MAX_U64: u64 = 18446744073709551615; // Max u64 value for overflow protection
    const MAX_ENCRYPTION_ID_BYTES: u64 = 1024;

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
        
        /// Pricing options - user controlled
        one_time_price: Option<u64>,            // Price for one-time access (0 = free)
        subscription_price: Option<u64>,        // Price for subscription access
        subscription_duration_days: u64,       // Subscription duration in days
        
        /// Access tracking
        purchasers: Table<address, bool>,       // One-time purchase access
        subscribers: Table<address, u64>,       // address -> expiry timestamp
        
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

    /// Global configuration for MyData system
    public struct MyDataConfig has key {
        id: UID,
        enable_flag: bool,
        max_tags: u64,
        max_subscription_days: u64,
        max_free_access_grants: u64,
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
        created_at: u64,
        snapshot_manifest_hash: vector<u8>,
        payment_reference: vector<u8>,
    }

    public struct DistributionRecordedEvent has copy, drop {
        snapshot_id: ID,
        total_amount: u64,
        contributor_count: u64,
        merkle_root: vector<u8>,
        published_at: u64,
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
        amount: u64,
        claimed_at: u64,
    }

    public struct DistributionRound has copy, drop, store {
        snapshot_id: ID,
        total_amount: u64,
        contributor_count: u64,
        merkle_root: vector<u8>,
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
        one_time_price: Option<u64>,
        subscription_price: Option<u64>,
        created_at: u64,
    }

    public struct PurchaseEvent has copy, drop {
        ip_id: address,
        buyer: address,
        price: u64,
        purchase_type: String, // "one_time" or "subscription"
        timestamp: u64,
    }

    public struct AccessGrantedEvent has copy, drop {
        ip_id: address,
        user: address,
        access_type: String,
        granted_by: address,
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
        enable_flag: bool,
        max_tags: u64,
        max_subscription_days: u64,
        max_free_access_grants: u64,
        timestamp: u64,
    }

    // === Admin Functions ===

    /// Create a MyDataAdminCap for bootstrap (package visibility only)
    public(package) fun create_mydata_admin_cap(ctx: &mut TxContext): MyDataAdminCap {
        MyDataAdminCap {
            id: object::new(ctx)
        }
    }

    /// Update MyData configuration (admin only)
    public entry fun update_mydata_config(
        _: &MyDataAdminCap,
        config: &mut MyDataConfig,
        enable_flag: bool,
        max_tags: u64,
        max_subscription_days: u64,
        max_free_access_grants: u64,
        ctx: &mut TxContext
    ) {
        // Validate parameters
        assert!(max_subscription_days > 0, EInvalidInput);
        assert!(max_tags > 0, EInvalidInput);
        assert!(max_free_access_grants > 0, EInvalidInput);

        config.enable_flag = enable_flag;
        config.max_tags = max_tags;
        config.max_subscription_days = max_subscription_days;
        config.max_free_access_grants = max_free_access_grants;
        
        // Emit config updated event
        event::emit(MyDataConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            enable_flag,
            max_tags,
            max_subscription_days,
            max_free_access_grants,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }

    // === Core Functions ===

    fun share_mydata_system_objects(ctx: &mut TxContext, enable_flag: bool) {
        let sender = tx_context::sender(ctx);
        let ver = upgrade::current_version();
        let config = MyDataConfig {
            id: object::new(ctx),
            enable_flag,
            max_tags: MAX_TAGS,
            max_subscription_days: MAX_SUBSCRIPTION_DAYS,
            max_free_access_grants: MAX_FREE_ACCESS_GRANTS,
            version: ver,
        };
        event::emit(MyDataConfigUpdatedEvent {
            updated_by: sender,
            enable_flag,
            max_tags: MAX_TAGS,
            max_subscription_days: MAX_SUBSCRIPTION_DAYS,
            max_free_access_grants: MAX_FREE_ACCESS_GRANTS,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
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
    public(package) fun bootstrap_init(ctx: &mut TxContext) {
        share_mydata_system_objects(ctx, DEFAULT_ENABLE);
    }

    public fun create_mydata_pool_admin_cap(ctx: &mut TxContext): MyDataPoolAdminCap {
        MyDataPoolAdminCap { id: object::new(ctx) }
    }

    fun gen_pool_id(registry: &MyDataPoolRegistry, nonce: u64): ID {
        let mut data = bcs::to_bytes(&object::uid_to_address(&registry.id));
        vector::append(&mut data, bcs::to_bytes(&nonce));
        object::id_from_bytes(hash::blake2b256(&data))
    }

    public entry fun create_broad_pool(
        _: &MyDataPoolAdminCap,
        registry: &mut MyDataPoolRegistry,
        name: String,
        description: String,
        clock: &Clock,
    ) {
        let nonce = registry.next_broad_pool_nonce;
        registry.next_broad_pool_nonce = nonce + 1;

        let pool_id = gen_pool_id(registry, nonce);
        let broad_pool = BroadPool {
            id: pool_id,
            name,
            description,
            created_at: clock::timestamp_ms(clock),
            version: registry.version,
        };

        table::add(&mut registry.broad_pools, pool_id, broad_pool);
        table::add(&mut registry.broad_to_sub, pool_id, vector::empty());
        registry.last_created_broad_pool_id = option::some(pool_id);

        event::emit(BroadPoolCreatedEvent {
            pool_id,
            name: broad_pool.name,
            created_at: broad_pool.created_at,
        });
    }

    public entry fun create_sub_pool(
        _: &MyDataPoolAdminCap,
        registry: &mut MyDataPoolRegistry,
        broad_pool_id: ID,
        name: String,
        description: String,
        schema_metadata: Option<vector<u8>>,
        clock: &Clock,
    ) {
        assert!(table::contains(&registry.broad_pools, broad_pool_id), EPqPoolNotFound);

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
        registry: &mut MyDataPoolRegistry,
        ip_id: address,
        sub_pool_ids: vector<ID>,
        clock: &Clock,
    ) {
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
        assert!(table::contains(&pool_registry.broad_pools, source_pool_id), EPqPoolNotFound);
        assert!(table::contains(&pool_registry.sub_pools, source_sub_pool_id), EPqSubPoolNotFound);

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

    public entry fun publish_merkle_root(
        _: &MyDataPoolAdminCap,
        anchor_registry: &SnapshotAnchorRegistry,
        vault: &mut MyDataClaimVault,
        snapshot_id: ID,
        root_hash: vector<u8>,
        clock: &Clock,
    ) {
        assert!(table::contains(&anchor_registry.anchors, snapshot_id), EPqAnchorNotFound);
        assert!(vector::length(&root_hash) == 32, EPqInvalidInput);
        assert!(!table::contains(&vault.merkle_roots, snapshot_id), EPqInvalidInput);
        table::add(&mut vault.merkle_roots, snapshot_id, root_hash);

        event::emit(MerkleRootPublishedEvent {
            snapshot_id,
            root_hash,
            published_at: clock::timestamp_ms(clock),
        });
    }

    public entry fun claim(
        vault: &mut MyDataClaimVault,
        snapshot_id: ID,
        amount: u64,
        leaf_index: u64,
        proof: vector<vector<u8>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(table::contains(&vault.merkle_roots, snapshot_id), EPqMerkleRootNotPublished);
        assert!(table::contains(&vault.snapshot_escrow, snapshot_id), EPqSnapshotEscrowMissing);
        assert!(*table::borrow(&vault.snapshot_escrow, snapshot_id) >= amount, EPqEscrowExceeded);

        let claimant = tx_context::sender(ctx);
        let leaf = merkle::leaf_hash(claimant, amount, object::id_to_bytes(&snapshot_id));
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

        let payout = balance::split(&mut vault.balance, amount);
        transfer::public_transfer(coin::from_balance(payout, ctx), claimant);

        event::emit(ClaimExecutedEvent {
            snapshot_id,
            claimant,
            amount,
            claimed_at: clock::timestamp_ms(clock),
        });
    }

    public entry fun deposit(
        _: &MyDataPoolAdminCap,
        vault: &mut MyDataClaimVault,
        _snapshot_id: ID,
        payment: Coin<MYSO>,
    ) {
        balance::join(&mut vault.balance, coin::into_balance(payment));
    }

    public entry fun record_distribution(
        _: &MyDataPoolAdminCap,
        dist_registry: &mut DistributionRegistry,
        vault: &MyDataClaimVault,
        snapshot_id: ID,
        total_amount: u64,
        contributor_count: u64,
        clock: &Clock,
    ) {
        assert!(table::contains(&vault.merkle_roots, snapshot_id), EPqMerkleRootNotPublished);
        let root = *table::borrow(&vault.merkle_roots, snapshot_id);
        let published_at = clock::timestamp_ms(clock);

        let round = DistributionRound {
            snapshot_id,
            total_amount,
            contributor_count,
            merkle_root: copy root,
            published_at,
        };
        table::add(&mut dist_registry.rounds, snapshot_id, round);
        event::emit(DistributionRecordedEvent {
            snapshot_id,
            total_amount,
            contributor_count,
            merkle_root: root,
            published_at,
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
        one_time_price: Option<u64>,
        subscription_price: Option<u64>,
        subscription_duration_days: u64,
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
        assert!(vector::length(&tags) <= config.max_tags, EInvalidInput);
        assert!(!vector::is_empty(&encryption_id), EInvalidInput);
        assert!(vector::length(&encryption_id) <= MAX_ENCRYPTION_ID_BYTES, EInvalidInput);
        
        // Validate prices with overflow protection
        if (option::is_some(&one_time_price)) {
            let price_val = *option::borrow(&one_time_price);
            assert!(price_val > 0 && price_val <= MAX_U64, EInvalidInput);
        };
        
        if (option::is_some(&subscription_price)) {
            let price_val = *option::borrow(&subscription_price);
            assert!(price_val > 0 && price_val <= MAX_U64, EInvalidInput);
        };
        
        // Validate subscription duration with overflow protection
        let sub_duration = if (subscription_duration_days == 0) { 30 } else { subscription_duration_days };
        assert!(sub_duration <= config.max_subscription_days, EInvalidInput);
        
        // Check for potential overflow in millisecond conversion
        let duration_ms = (sub_duration as u128) * (MILLISECONDS_PER_DAY as u128);
        assert!(duration_ms <= (MAX_U64 as u128), EOverflow);
        
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
            one_time_price,
            subscription_price,
            subscription_duration_days: sub_duration,
            purchasers: table::new(ctx),
            subscribers: table::new(ctx),
            geographic_region,
            data_quality,
            sample_size,
            collection_method,
            is_updating,
            update_frequency,
            version: upgrade::current_version(),
        };

        let ip_id = object::uid_to_address(&mydata.id);
        
        event::emit(MyDataCreatedEvent {
            ip_id,
            owner: mydata.owner,
            media_type: mydata.media_type,
            platform_id: mydata.platform_id,
            one_time_price: mydata.one_time_price,
            subscription_price: mydata.subscription_price,
            created_at: mydata.created_at,
        });

        mydata
    }

    /// Create and share MyData publicly
    #[allow(lint(share_owned))]
    public entry fun create_and_share(
        config: &MyDataConfig,
        registry: &mut MyDataRegistry,
        media_type: String,
        tags: vector<String>,
        platform_id: Option<address>,
        timestamp_start: u64,
        timestamp_end: Option<u64>,
        encrypted_data: vector<u8>,
        encryption_id: vector<u8>,
        one_time_price: Option<u64>,
        subscription_price: Option<u64>,
        subscription_duration_days: u64,
        geographic_region: Option<String>,
        data_quality: Option<String>,
        sample_size: Option<u64>,
        collection_method: Option<String>,
        is_updating: bool,
        update_frequency: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.enable_flag, EDisabled);
        
        let mydata = create(
            config,
            media_type,
            tags,
            platform_id,
            timestamp_start,
            timestamp_end,
            encrypted_data,
            encryption_id,
            one_time_price,
            subscription_price,
            subscription_duration_days,
            geographic_region,
            data_quality,
            sample_size,
            collection_method,
            is_updating,
            update_frequency,
            clock,
            ctx,
        );

        // Register in the registry
        let ip_id = object::uid_to_address(&mydata.id);
        table::add(&mut registry.ip_to_owner, ip_id, mydata.owner);

        transfer::share_object(mydata);
    }

    /// Purchase one-time access to MyData data
    public entry fun purchase_one_time(
        config: &MyDataConfig,
        mydata: &mut MyData,
        payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.enable_flag, EDisabled);
        
        // Check version compatibility
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        
        let buyer = tx_context::sender(ctx);
        
        // Check if one-time purchase is available
        assert!(option::is_some(&mydata.one_time_price), ENotForSale);
        let price = *option::borrow(&mydata.one_time_price);
        
        // Check payment amount
        assert!(coin::value(&payment) >= price, EPriceMismatch);
        
        // Check if buyer already has access
        assert!(!table::contains(&mydata.purchasers, buyer), EAlreadyPurchased);
        
        // Prevent self-purchase
        assert!(buyer != mydata.owner, ESelfPurchase);
        
        let mut payment = payment;
        let to_owner = coin::split(&mut payment, price, ctx);
        transfer::public_transfer(to_owner, mydata.owner);
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, buyer);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Grant access
        table::add(&mut mydata.purchasers, buyer, true);

        event::emit(PurchaseEvent {
            ip_id: object::uid_to_address(&mydata.id),
            buyer,
            price,
            purchase_type: string::utf8(b"one_time"),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Purchase subscription access to MyData data
    public entry fun purchase_subscription(
        config: &MyDataConfig,
        mydata: &mut MyData,
        payment: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(config.enable_flag, EDisabled);
        
        // Check version compatibility
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        
        let buyer = tx_context::sender(ctx);
        
        // Check if subscription is available
        assert!(option::is_some(&mydata.subscription_price), ENotForSale);
        let price = *option::borrow(&mydata.subscription_price);
        
        // Check payment amount
        assert!(coin::value(&payment) >= price, EPriceMismatch);
        
        // Prevent self-purchase
        assert!(buyer != mydata.owner, ESelfPurchase);
        
        // Validate subscription duration to prevent overflow
        assert!(mydata.subscription_duration_days > 0, EInvalidInput);
        assert!(mydata.subscription_duration_days <= config.max_subscription_days, EInvalidInput);
        
        // Calculate subscription expiry safely with overflow protection
        let current_time = clock::timestamp_ms(clock);
        let duration_ms = (mydata.subscription_duration_days as u128) * (MILLISECONDS_PER_DAY as u128);
        let expiry_time = (current_time as u128) + duration_ms;
        
        // Ensure we don't overflow u64
        assert!(expiry_time <= (MAX_U64 as u128), EOverflow);
        let expiry_time_u64 = expiry_time as u64;
        
        let mut payment = payment;
        let to_owner = coin::split(&mut payment, price, ctx);
        transfer::public_transfer(to_owner, mydata.owner);
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, buyer);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Grant/extend subscription access
        if (table::contains(&mydata.subscribers, buyer)) {
            // Extend existing subscription
            let current_expiry = table::remove(&mut mydata.subscribers, buyer);
            let new_expiry = if (current_expiry > current_time) {
                // Add to existing time, but check for overflow
                let extended_time = (current_expiry as u128) + duration_ms;
                assert!(extended_time <= (MAX_U64 as u128), EOverflow);
                extended_time as u64
            } else {
                expiry_time_u64
            };
            table::add(&mut mydata.subscribers, buyer, new_expiry);
        } else {
            // New subscription
            table::add(&mut mydata.subscribers, buyer, expiry_time_u64);
        };

        event::emit(PurchaseEvent {
            ip_id: object::uid_to_address(&mydata.id),
            buyer,
            price,
            purchase_type: string::utf8(b"subscription"),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Update pricing (owner only)
    public entry fun update_pricing(
        mydata: &mut MyData,
        new_one_time_price: Option<u64>,
        new_subscription_price: Option<u64>,
        new_subscription_duration_days: Option<u64>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        // Check version compatibility
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        
        // Validate new prices
        if (option::is_some(&new_one_time_price)) {
            let price_val = *option::borrow(&new_one_time_price);
            assert!(price_val > 0, EInvalidInput);
        };
        
        if (option::is_some(&new_subscription_price)) {
            let price_val = *option::borrow(&new_subscription_price);
            assert!(price_val > 0, EInvalidInput);
        };

        mydata.one_time_price = new_one_time_price;
        mydata.subscription_price = new_subscription_price;
        
        if (option::is_some(&new_subscription_duration_days)) {
            let duration = *option::borrow(&new_subscription_duration_days);
            if (duration > 0) {
                mydata.subscription_duration_days = duration;
            };
        };

        event::emit(AccessGrantedEvent {
            ip_id: object::uid_to_address(&mydata.id),
            user: mydata.owner,
            access_type: string::utf8(b"pricing_update"),
            granted_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Update MyData content and metadata (owner only)
    public entry fun update_content(
        mydata: &mut MyData,
        new_encrypted_data: Option<vector<u8>>,
        new_tags: Option<vector<String>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        // Check version compatibility
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        
        if (option::is_some(&new_encrypted_data)) {
            mydata.encrypted_data = *option::borrow(&new_encrypted_data);
        };
        
        if (option::is_some(&new_tags)) {
            mydata.tags = *option::borrow(&new_tags);
        };
        
        mydata.last_updated = clock::timestamp_ms(clock);

        event::emit(AccessGrantedEvent {
            ip_id: object::uid_to_address(&mydata.id),
            user: mydata.owner,
            access_type: string::utf8(b"content_update"),
            granted_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Assign MyData to sub-pools (owner only).
    public entry fun assign_mydata_to_pools(
        mydata: &MyData,
        pool_registry: &mut MyDataPoolRegistry,
        sub_pool_ids: vector<ID>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        let ip_id = object::uid_to_address(&mydata.id);
        assign_mydata_to_sub_pools(pool_registry, ip_id, sub_pool_ids, clock);
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
        // Owner always has access
        if (user == mydata.owner) return true;
        
        // Check one-time purchase
        if (table::contains(&mydata.purchasers, user)) return true;
        
        // Check active subscription
        if (table::contains(&mydata.subscribers, user)) {
            let expiry = *table::borrow(&mydata.subscribers, user);
            let current_time = clock::timestamp_ms(clock);
            return current_time <= expiry
        };
        
        false
    }

    /// True if `candidate` matches this listing’s `encryption_id` (the MyData policy `id` bytes).
    public fun encryption_id_matches(mydata: &MyData, candidate: &vector<u8>): bool {
        bytes_equal_u8(&mydata.encryption_id, candidate)
    }

    /// Key-server policy hook for `fetch_key`: `id` must match `encryption_id`, and the sender must have
    /// [`has_access`] (owner, purchaser, or active subscriber). Register this package on the key server when
    /// using permissioned mode; `EncryptedObject.package_id` at encrypt time must match this package.
    public entry fun mydata_approve(
        id: vector<u8>,
        mydata: &MyData,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert!(encryption_id_matches(mydata, &id), EPolicyIdMismatch);
        assert!(has_access(mydata, tx_context::sender(ctx), clock), EPolicyNotEntitled);
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
        assert!(config.enable_flag, EDisabled);
        
        // Check version compatibility
        assert!(mydata.version == upgrade::current_version(), EInvalidInput);
        
        assert!(tx_context::sender(ctx) == mydata.owner, EUnauthorized);
        assert!(user != mydata.owner, ESelfPurchase); // Owner doesn't need granted access
        
        // Check max free access grants limit
        let total_grants = table::length(&mydata.purchasers) + table::length(&mydata.subscribers);
        assert!(total_grants < config.max_free_access_grants, EInvalidInput);
        
        if (access_type == 0) {
            // Grant one-time access
            if (!table::contains(&mydata.purchasers, user)) {
                table::add(&mut mydata.purchasers, user, true);
            };
        } else {
            // Grant subscription access
            let duration_days = if (option::is_some(&subscription_days)) {
                let days = *option::borrow(&subscription_days);
                assert!(days > 0 && days <= config.max_subscription_days, EInvalidInput);
                days
            } else {
                mydata.subscription_duration_days
            };
            
            let current_time = clock::timestamp_ms(clock);
            let duration_ms = (duration_days as u128) * (MILLISECONDS_PER_DAY as u128);
            let expiry_time = (current_time as u128) + duration_ms;
            
            // Ensure we don't overflow u64
            assert!(expiry_time <= (MAX_U64 as u128), EOverflow);
            let expiry_time_u64 = expiry_time as u64;
            
            if (table::contains(&mydata.subscribers, user)) {
                table::remove(&mut mydata.subscribers, user);
            };
            table::add(&mut mydata.subscribers, user, expiry_time_u64);
        };

        event::emit(AccessGrantedEvent {
            ip_id: object::uid_to_address(&mydata.id),
            user,
            access_type: if (access_type == 0) { string::utf8(b"one_time") } else { string::utf8(b"subscription") },
            granted_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    // === Getter Functions ===
    
    public fun owner(mydata: &MyData): address { mydata.owner }
    public fun object_address(mydata: &MyData): address { object::uid_to_address(&mydata.id) }
    public fun media_type(mydata: &MyData): String { mydata.media_type }
    public fun tags(mydata: &MyData): vector<String> { mydata.tags }
    public fun platform_id(mydata: &MyData): Option<address> { mydata.platform_id }
    public fun one_time_price(mydata: &MyData): Option<u64> { mydata.one_time_price }
    public fun subscription_price(mydata: &MyData): Option<u64> { mydata.subscription_price }
    public fun subscription_duration_days(mydata: &MyData): u64 { mydata.subscription_duration_days }
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
    public fun purchaser_count(mydata: &MyData): u64 { table::length(&mydata.purchasers) }
    public fun subscriber_count(mydata: &MyData): u64 { table::length(&mydata.subscribers) }
    public fun is_one_time_for_sale(mydata: &MyData): bool { option::is_some(&mydata.one_time_price) }
    public fun is_subscription_available(mydata: &MyData): bool { option::is_some(&mydata.subscription_price) }

    /// Check if a user has an active subscription
    public fun has_active_subscription(mydata: &MyData, user: address, clock: &Clock): bool {
        if (!table::contains(&mydata.subscribers, user)) return false;
        let expiry = *table::borrow(&mydata.subscribers, user);
        let current_time = clock::timestamp_ms(clock);
        current_time <= expiry
    }

    /// Get subscription expiry time for a user
    public fun get_subscription_expiry(mydata: &MyData, user: address): Option<u64> {
        if (table::contains(&mydata.subscribers, user)) {
            option::some(*table::borrow(&mydata.subscribers, user))
        } else {
            option::none()
        }
    }

    /// Get total revenue potential (for analytics) with overflow protection
    public fun get_revenue_potential(mydata: &MyData): u64 {
        let one_time_revenue = if (option::is_some(&mydata.one_time_price)) {
            let price = *option::borrow(&mydata.one_time_price);
            let count = table::length(&mydata.purchasers);
            // Use u128 for calculation to detect overflow
            let revenue = (price as u128) * (count as u128);
            if (revenue > (MAX_U64 as u128)) {
                MAX_U64
            } else {
                revenue as u64
            }
        } else {
            0
        };
        
        let subscription_revenue = if (option::is_some(&mydata.subscription_price)) {
            let price = *option::borrow(&mydata.subscription_price);
            let count = table::length(&mydata.subscribers);
            // Use u128 for calculation to detect overflow
            let revenue = (price as u128) * (count as u128);
            if (revenue > (MAX_U64 as u128)) {
                MAX_U64
            } else {
                revenue as u64
            }
        } else {
            0
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
        table::length(&mydata.purchasers) > 0 || table::length(&mydata.subscribers) > 0
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
    public fun test_init(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        transfer::public_transfer(MyDataAdminCap { id: object::new(ctx) }, sender);
        transfer::public_transfer(create_mydata_pool_admin_cap(ctx), sender);
        share_mydata_system_objects(ctx, true);
    }

    #[test_only]
    public fun test_destroy(mydata: MyData) {
        let MyData { 
            id, owner: _, media_type: _, tags: _, platform_id: _,
            timestamp_start: _, timestamp_end: _, created_at: _, last_updated: _,
            encrypted_data: _, encryption_id: _, one_time_price: _, subscription_price: _,
            subscription_duration_days: _, purchasers, subscribers, geographic_region: _,
            data_quality: _, sample_size: _, collection_method: _, is_updating: _,
            update_frequency: _, version: _
        } = mydata;
        table::drop(purchasers);
        table::drop(subscribers);
        object::delete(id);
    }
}
