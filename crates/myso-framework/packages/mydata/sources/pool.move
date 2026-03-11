// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// MyData Query Marketplace: Broad Pools, Sub-Pools, Query Snapshots, Claim Vault, Merkle Settlement.
/// All pool/marketplace logic in one module.

#[allow(duplicate_alias, unused_use, lint(public_entry))]
module mydata::pool {
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

    const VERSION: u64 = 1;

    const EUnauthorized: u64 = 1;
    const EInvalidInput: u64 = 2;
    const EPoolNotFound: u64 = 3;
    const ESubPoolNotFound: u64 = 4;
    const EInvalidProof: u64 = 5;
    const EAlreadyClaimed: u64 = 6;
    const EMerkleRootNotPublished: u64 = 7;
    const EInsufficientPayment: u64 = 8;

    // === Phase 1: Pool Registry ===

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

    // === Phase 2: Snapshot Anchor ===

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
    }

    // === Phase 3: Claim Vault ===

    public struct MyDataClaimVault has key {
        id: UID,
        balance: Balance<MYSO>,
        merkle_roots: Table<ID, vector<u8>>,
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

    // === Phase 4: Distribution ===

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

    // === Bootstrap & Admin ===

    public fun create_admin_cap(ctx: &mut TxContext): MyDataPoolAdminCap {
        MyDataPoolAdminCap { id: object::new(ctx) }
    }

    public fun bootstrap_init(ctx: &mut TxContext) {
        let registry = MyDataPoolRegistry {
            id: object::new(ctx),
            broad_pools: table::new(ctx),
            sub_pools: table::new(ctx),
            broad_to_sub: table::new(ctx),
            mydata_to_sub_pools: table::new(ctx),
            next_broad_pool_nonce: 0,
            next_sub_pool_nonce: 0,
            last_created_broad_pool_id: option::none(),
            last_created_sub_pool_id: option::none(),
            version: VERSION,
        };
        transfer::share_object(registry);

        let anchor_registry = SnapshotAnchorRegistry {
            id: object::new(ctx),
            anchors: table::new(ctx),
            next_snapshot_nonce: 0,
            version: VERSION,
        };
        transfer::share_object(anchor_registry);

        let vault = MyDataClaimVault {
            id: object::new(ctx),
            balance: balance::zero(),
            merkle_roots: table::new(ctx),
            claimed: table::new(ctx),
            version: VERSION,
        };
        transfer::share_object(vault);

        let dist_registry = DistributionRegistry {
            id: object::new(ctx),
            rounds: table::new(ctx),
            version: VERSION,
        };
        transfer::share_object(dist_registry);
    }

    fun gen_pool_id(registry: &MyDataPoolRegistry, nonce: u64): ID {
        let mut data = bcs::to_bytes(&object::uid_to_address(&registry.id));
        vector::append(&mut data, bcs::to_bytes(&nonce));
        object::id_from_bytes(hash::blake2b256(&data))
    }

    // === Phase 1 Entry Points ===

    public entry fun create_broad_pool(
        _: &MyDataPoolAdminCap,
        registry: &mut MyDataPoolRegistry,
        name: String,
        description: String,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let nonce = registry.next_broad_pool_nonce;
        registry.next_broad_pool_nonce = nonce + 1;

        let pool_id = gen_pool_id(registry, nonce);
        let broad_pool = BroadPool {
            id: pool_id,
            name,
            description,
            created_at: clock::timestamp_ms(clock),
            version: VERSION,
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
        ctx: &mut TxContext,
    ) {
        assert!(table::contains(&registry.broad_pools, broad_pool_id), EPoolNotFound);

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
            version: VERSION,
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

    public fun assign_mydata_to_sub_pools(
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
            assert!(table::contains(&registry.sub_pools, sub_id), ESubPoolNotFound);
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

    public entry fun remove_mydata_from_sub_pool(
        registry: &mut MyDataPoolRegistry,
        ip_id: address,
        sub_pool_id: ID,
        clock: &Clock,
    ) {
        assert!(table::contains(&registry.mydata_to_sub_pools, ip_id), EInvalidInput);
        let sub_ids = table::borrow_mut(&mut registry.mydata_to_sub_pools, ip_id);
        let (found, idx) = vector::index_of(sub_ids, &sub_pool_id);
        assert!(found, EInvalidInput);
        vector::remove(sub_ids, idx);
    }

    // === Phase 2 Entry Points ===

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
        assert!(table::contains(&pool_registry.broad_pools, source_pool_id), EPoolNotFound);
        assert!(table::contains(&pool_registry.sub_pools, source_sub_pool_id), ESubPoolNotFound);

        let price_paid = coin::value(&payment);
        assert!(price_paid > 0, EInsufficientPayment);

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

        table::add(&mut anchor_registry.anchors, snapshot_id, anchor);
        balance::join(&mut vault.balance, coin::into_balance(payment));

        event::emit(SnapshotAnchorRecordedEvent {
            snapshot_id,
            buyer_address: buyer,
            price_paid,
            created_at: anchor.created_at,
        });
    }

    public fun get_snapshot_anchor(anchor_registry: &SnapshotAnchorRegistry, snapshot_id: ID): Option<QuerySnapshotAnchor> {
        if (table::contains(&anchor_registry.anchors, snapshot_id)) {
            option::some(*table::borrow(&anchor_registry.anchors, snapshot_id))
        } else {
            option::none()
        }
    }

    // === Phase 3 Entry Points ===

    public entry fun publish_merkle_root(
        _: &MyDataPoolAdminCap,
        vault: &mut MyDataClaimVault,
        snapshot_id: ID,
        root_hash: vector<u8>,
        clock: &Clock,
    ) {
        assert!(vector::length(&root_hash) == 32, EInvalidInput);
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
        assert!(table::contains(&vault.merkle_roots, snapshot_id), EMerkleRootNotPublished);

        let claimant = tx_context::sender(ctx);
        let leaf = merkle::leaf_hash(claimant, amount, object::id_to_bytes(&snapshot_id));
        let root = *table::borrow(&vault.merkle_roots, snapshot_id);

        assert!(merkle::verify_proof(leaf, &proof, leaf_index, root), EInvalidProof);

        if (!table::contains(&vault.claimed, snapshot_id)) {
            table::add(&mut vault.claimed, snapshot_id, table::new(ctx));
        };
        let claimed_table = table::borrow_mut(&mut vault.claimed, snapshot_id);
        assert!(!table::contains(claimed_table, claimant), EAlreadyClaimed);
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

    public entry fun deposit(vault: &mut MyDataClaimVault, _snapshot_id: ID, payment: Coin<MYSO>) {
        balance::join(&mut vault.balance, coin::into_balance(payment));
    }

    // === Phase 4 Entry Points ===

    public entry fun record_distribution(
        _: &MyDataPoolAdminCap,
        dist_registry: &mut DistributionRegistry,
        vault: &MyDataClaimVault,
        snapshot_id: ID,
        total_amount: u64,
        contributor_count: u64,
        clock: &Clock,
    ) {
        assert!(table::contains(&vault.merkle_roots, snapshot_id), EMerkleRootNotPublished);
        let root = *table::borrow(&vault.merkle_roots, snapshot_id);

        let round = DistributionRound {
            snapshot_id,
            total_amount,
            contributor_count,
            merkle_root: root,
            published_at: clock::timestamp_ms(clock),
        };
        table::add(&mut dist_registry.rounds, snapshot_id, round);
    }

    // === Getters ===

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

    public fun get_distribution_round(registry: &DistributionRegistry, snapshot_id: ID): Option<DistributionRound> {
        if (table::contains(&registry.rounds, snapshot_id)) {
            option::some(*table::borrow(&registry.rounds, snapshot_id))
        } else {
            option::none()
        }
    }

    public fun broad_pool_id(pool: &BroadPool): ID { pool.id }
    public fun sub_pool_id(pool: &SubPool): ID { pool.id }

    #[test_only]
    public fun last_created_pool_id(registry: &MyDataPoolRegistry): ID {
        assert!(option::is_some(&registry.last_created_broad_pool_id), EInvalidInput);
        *option::borrow(&registry.last_created_broad_pool_id)
    }

    #[test_only]
    public fun last_created_sub_pool_id(registry: &MyDataPoolRegistry): ID {
        assert!(option::is_some(&registry.last_created_sub_pool_id), EInvalidInput);
        *option::borrow(&registry.last_created_sub_pool_id)
    }

    #[test_only]
    public fun test_init(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        bootstrap_init(ctx);
        transfer::public_transfer(create_admin_cap(ctx), sender);
    }
}
