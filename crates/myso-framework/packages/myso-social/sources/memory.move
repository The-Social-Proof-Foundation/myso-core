// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Memory — account and encrypted-memory access policy for delegated keys.
///
/// Registry plus shared `MemoryAccount` per owner, linked to `social_contracts::profile::Profile`.
///
/// Register `approve_key_policy` with your key service alongside marketplace policies
/// (see `social_contracts::mydata`). `owner_key_suffix_bytes` is the canonical suffix for owner-scoped
/// key material clients construct at encrypt time.

#[allow(duplicate_alias, lint(public_entry))]
module social_contracts::memory {
    use std::option::{Self, Option};
    use std::string::{Self, String};
    use std::vector;

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        table::{Self, Table},
        clock::{Self, Clock},
        dynamic_field as df,
        package::{Self, UpgradeCap},
        bcs,
        event,
    };

    // ============================================================
    // Error codes
    // ============================================================

    const EDelegateKeyAlreadyExists: u64 = 0;
    const EDelegateKeyNotFound: u64 = 1;
    const ETooManyDelegateKeys: u64 = 2;
    const EAccountAlreadyExists: u64 = 3;
    const ENotOwner: u64 = 4;
    const EInvalidPublicKeyLength: u64 = 5;
    const EAccountDeactivated: u64 = 6;
    const EWrongVersion: u64 = 7;
    const ENotUpgradeAuthority: u64 = 8;
    const EAlreadyMigrated: u64 = 9;
    const ELabelTooLong: u64 = 10;
    const EAccountAlreadyActive: u64 = 11;
    const ENoAccess: u64 = 100;
    const ENewOwnerHasMemoryAccount: u64 = 12;
    const ERegistryAccountMismatch: u64 = 13;

    const MAX_DELEGATE_KEYS: u64 = 20;
    const ED25519_PUBLIC_KEY_LENGTH: u64 = 32;
    const MAX_LABEL_LENGTH: u64 = 64;

    const VERSION: u64 = 1;
    const VERSION_DF_KEY: vector<u8> = b"memory_version";

    // ============================================================
    // Structs
    // ============================================================

    /// Shared singleton — maps owner address → shared `MemoryAccount` id.
    public struct MemoryRegistry has key {
        id: UID,
        accounts: Table<address, ID>,
    }

    /// Shared memory account — one per owner when linked from profile flows.
    public struct MemoryAccount has key, store {
        id: UID,
        owner: address,
        /// `object::uid_to_address` of the linked [`social_contracts::profile::Profile`].
        profile_id: address,
        delegate_keys: vector<MemoryDelegateKey>,
        created_at: u64,
        active: bool,
    }

    /// Authorized Ed25519 delegate key and its derived on-chain address.
    public struct MemoryDelegateKey has store, copy, drop {
        public_key: vector<u8>,
        derived_address: address,
        label: String,
        created_at: u64,
    }

    // ============================================================
    // Events
    // ============================================================

    public struct MemoryAccountCreated has copy, drop {
        account_id: ID,
        owner: address,
        profile_id: address,
    }

    public struct MemoryDelegateKeyAdded has copy, drop {
        account_id: ID,
        public_key: vector<u8>,
        derived_address: address,
        label: String,
    }

    public struct MemoryDelegateKeyRemoved has copy, drop {
        account_id: ID,
        public_key: vector<u8>,
        derived_address: address,
    }

    public struct MemoryAccountDeactivated has copy, drop {
        account_id: ID,
        owner: address,
    }

    public struct MemoryAccountReactivated has copy, drop {
        account_id: ID,
        owner: address,
    }

    public struct MemoryAccountMigrated has copy, drop {
        account_id: ID,
        from: u64,
        to: u64,
    }

    public struct MemoryRegistryMigrated has copy, drop {
        registry_id: ID,
        from: u64,
        to: u64,
    }

    // ============================================================
    // Bootstrap
    // ============================================================

    public(package) fun bootstrap_init(ctx: &mut TxContext) {
        let mut registry = MemoryRegistry {
            id: object::new(ctx),
            accounts: table::new(ctx),
        };
        set_version(&mut registry.id, VERSION);
        transfer::share_object(registry);
    }

    // ============================================================
    // Package-only: profile integration
    // ============================================================

    /// Create and share a `MemoryAccount`, register by `tx_context::sender`.
    public(package) fun create_account_for_profile(
        registry: &mut MemoryRegistry,
        profile_id: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ): ID {
        assert_object_version(&registry.id);

        let sender = tx_context::sender(ctx);
        assert!(!table::contains(&registry.accounts, sender), EAccountAlreadyExists);

        let mut account = MemoryAccount {
            id: object::new(ctx),
            owner: sender,
            profile_id,
            delegate_keys: vector::empty(),
            created_at: clock::timestamp_ms(clock),
            active: true,
        };
        set_version(&mut account.id, VERSION);

        let account_id = object::id(&account);
        table::add(&mut registry.accounts, sender, account_id);

        event::emit(MemoryAccountCreated {
            account_id,
            owner: sender,
            profile_id,
        });

        transfer::share_object(account);
        account_id
    }

    /// Keep registry and account owner aligned when the profile is transferred.
    public(package) fun transfer_account_owner_with_profile(
        registry: &mut MemoryRegistry,
        account: &mut MemoryAccount,
        profile_id: address,
        old_owner: address,
        new_owner: address,
    ) {
        assert_object_version(&registry.id);
        assert_object_version(&account.id);
        assert!(account.owner == old_owner, ENotOwner);
        assert!(account.profile_id == profile_id, ENotOwner);
        assert!(table::contains(&registry.accounts, old_owner), ERegistryAccountMismatch);
        assert!(*table::borrow(&registry.accounts, old_owner) == object::id(account), ERegistryAccountMismatch);
        assert!(!table::contains(&registry.accounts, new_owner), ENewOwnerHasMemoryAccount);

        table::remove(&mut registry.accounts, old_owner);
        account.owner = new_owner;
        table::add(&mut registry.accounts, new_owner, object::id(account));
    }

    // ============================================================
    // Delegate keys
    // ============================================================

    public entry fun add_delegate_key(
        account: &mut MemoryAccount,
        public_key: vector<u8>,
        derived_address: address,
        label: String,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);

        assert!(account.owner == tx_context::sender(ctx), ENotOwner);
        assert!(account.active, EAccountDeactivated);
        assert!(vector::length(&public_key) == ED25519_PUBLIC_KEY_LENGTH, EInvalidPublicKeyLength);
        assert!(string::length(&label) <= MAX_LABEL_LENGTH, ELabelTooLong);
        assert!(vector::length(&account.delegate_keys) < MAX_DELEGATE_KEYS, ETooManyDelegateKeys);

        let len = vector::length(&account.delegate_keys);
        let mut i = 0;
        while (i < len) {
            let existing = vector::borrow(&account.delegate_keys, i);
            assert!(existing.public_key != public_key, EDelegateKeyAlreadyExists);
            i = i + 1;
        };

        let key = MemoryDelegateKey {
            public_key,
            derived_address,
            label,
            created_at: clock::timestamp_ms(clock),
        };

        let account_id = object::id(account);

        event::emit(MemoryDelegateKeyAdded {
            account_id,
            public_key: key.public_key,
            derived_address: key.derived_address,
            label: key.label,
        });

        vector::push_back(&mut account.delegate_keys, key);
    }

    public entry fun remove_delegate_key(
        account: &mut MemoryAccount,
        public_key: vector<u8>,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert!(account.owner == tx_context::sender(ctx), ENotOwner);

        let mut found = false;
        let mut derived_address = @0x0;
        let mut i = 0;
        let len = vector::length(&account.delegate_keys);

        while (i < len) {
            let k = vector::borrow(&account.delegate_keys, i);
            if (k.public_key == public_key) {
                derived_address = k.derived_address;
                vector::remove(&mut account.delegate_keys, i);
                found = true;
                break
            };
            i = i + 1;
        };

        assert!(found, EDelegateKeyNotFound);

        event::emit(MemoryDelegateKeyRemoved {
            account_id: object::id(account),
            public_key,
            derived_address,
        });
    }

    // ============================================================
    // Activation
    // ============================================================

    public entry fun deactivate_account(account: &mut MemoryAccount, ctx: &TxContext) {
        assert_object_version(&account.id);
        assert!(account.owner == tx_context::sender(ctx), ENotOwner);
        assert!(account.active, EAccountDeactivated);
        account.active = false;

        event::emit(MemoryAccountDeactivated {
            account_id: object::id(account),
            owner: account.owner,
        });
    }

    public entry fun reactivate_account(account: &mut MemoryAccount, ctx: &TxContext) {
        assert_object_version(&account.id);
        assert!(account.owner == tx_context::sender(ctx), ENotOwner);
        assert!(!account.active, EAccountAlreadyActive);
        account.active = true;

        event::emit(MemoryAccountReactivated {
            account_id: object::id(account),
            owner: account.owner,
        });
    }

    // ============================================================
    // Migration (dynamic-field object version)
    // ============================================================

    public entry fun migrate_account(account: &mut MemoryAccount, ctx: &TxContext) {
        assert!(account.owner == tx_context::sender(ctx), ENotOwner);
        let cur = get_version(&account.id);
        assert!(cur < VERSION, EAlreadyMigrated);
        bump_version(&mut account.id, VERSION);

        event::emit(MemoryAccountMigrated {
            account_id: object::id(account),
            from: cur,
            to: VERSION,
        });
    }

    public entry fun admin_migrate_account(cap: &UpgradeCap, account: &mut MemoryAccount) {
        assert_cap_for_this_package(cap);
        let cur = get_version(&account.id);
        assert!(cur < VERSION, EAlreadyMigrated);
        bump_version(&mut account.id, VERSION);

        event::emit(MemoryAccountMigrated {
            account_id: object::id(account),
            from: cur,
            to: VERSION,
        });
    }

    public entry fun migrate_registry(cap: &UpgradeCap, registry: &mut MemoryRegistry) {
        assert_cap_for_this_package(cap);
        let cur = get_version(&registry.id);
        assert!(cur < VERSION, EAlreadyMigrated);
        bump_version(&mut registry.id, VERSION);

        event::emit(MemoryRegistryMigrated {
            registry_id: object::id(registry),
            from: cur,
            to: VERSION,
        });
    }

    // ============================================================
    // Views
    // ============================================================

    public fun profile_id(account: &MemoryAccount): address {
        account.profile_id
    }

    public fun is_delegate(account: &MemoryAccount, public_key: &vector<u8>): bool {
        let mut i = 0;
        let len = vector::length(&account.delegate_keys);
        while (i < len) {
            if (&vector::borrow(&account.delegate_keys, i).public_key == public_key) {
                return true
            };
            i = i + 1;
        };
        false
    }

    public fun is_delegate_address(account: &MemoryAccount, addr: address): bool {
        let mut i = 0;
        let len = vector::length(&account.delegate_keys);
        while (i < len) {
            if (vector::borrow(&account.delegate_keys, i).derived_address == addr) {
                return true
            };
            i = i + 1;
        };
        false
    }

    public fun owner(account: &MemoryAccount): address {
        account.owner
    }

    public fun delegate_count(account: &MemoryAccount): u64 {
        vector::length(&account.delegate_keys)
    }

    public fun has_account(registry: &MemoryRegistry, addr: address): bool {
        table::contains(&registry.accounts, addr)
    }

    /// Account id registered for owner, if any.
    public fun account_id_for_owner(registry: &MemoryRegistry, owner: address): Option<ID> {
        if (table::contains(&registry.accounts, owner)) {
            option::some(*table::borrow(&registry.accounts, owner))
        } else {
            option::none()
        }
    }

    public fun is_active(account: &MemoryAccount): bool {
        account.active
    }

    public fun account_version(account: &MemoryAccount): u64 {
        get_version(&account.id)
    }

    public fun registry_version(registry: &MemoryRegistry): u64 {
        get_version(&registry.id)
    }

    public fun current_contract_version(): u64 {
        VERSION
    }

    // ============================================================
    // Key-server access policy
    // ============================================================

    /// Key-server dry-run entry: allow owner (matching key id suffix) or a registered delegate.
    public entry fun approve_key_policy(id: vector<u8>, account: &MemoryAccount, ctx: &TxContext) {
        assert_object_version(&account.id);
        assert!(account.active, EAccountDeactivated);

        let caller = tx_context::sender(ctx);

        let owner_bytes = bcs::to_bytes(&account.owner);
        let is_owner = (caller == account.owner) && has_suffix(&id, &owner_bytes);
        let is_delegate = is_delegate_address(account, caller);

        assert!(is_owner || is_delegate, ENoAccess);
    }

    /// Raw bytes for the owner portion of a key id (`package_id` prefix is added by client tooling).
    public fun owner_key_suffix_bytes(owner_addr: address): vector<u8> {
        bcs::to_bytes(&owner_addr)
    }

    // ============================================================
    // Internal helpers
    // ============================================================

    fun get_version(uid: &UID): u64 {
        if (df::exists_with_type<vector<u8>, u64>(uid, VERSION_DF_KEY)) {
            *df::borrow<vector<u8>, u64>(uid, VERSION_DF_KEY)
        } else {
            1
        }
    }

    fun set_version(uid: &mut UID, v: u64) {
        if (df::exists_with_type<vector<u8>, u64>(uid, VERSION_DF_KEY)) {
            let r = df::borrow_mut<vector<u8>, u64>(uid, VERSION_DF_KEY);
            *r = v;
        } else {
            df::add(uid, VERSION_DF_KEY, v);
        }
    }

    fun bump_version(uid: &mut UID, v: u64) {
        set_version(uid, v)
    }

    fun assert_object_version(uid: &UID) {
        assert!(get_version(uid) == VERSION, EWrongVersion);
    }

    fun assert_cap_for_this_package(cap: &UpgradeCap) {
        let cap_pkg = package::upgrade_package(cap);
        assert!(object::id_to_address(&cap_pkg) == @social_contracts, ENotUpgradeAuthority);
    }

    fun has_suffix(data: &vector<u8>, suffix: &vector<u8>): bool {
        let data_len = vector::length(data);
        let suffix_len = vector::length(suffix);
        if (suffix_len > data_len) return false;
        let offset = data_len - suffix_len;
        let mut i = 0;
        while (i < suffix_len) {
            if (*vector::borrow(data, offset + i) != *vector::borrow(suffix, i)) return false;
            i = i + 1;
        };
        true
    }

    // ============================================================
    // Test helpers
    // ============================================================

    #[test_only]
    public fun test_bootstrap_init(ctx: &mut TxContext) {
        bootstrap_init(ctx);
    }

    #[test_only]
    public fun test_make_upgrade_cap(ctx: &mut TxContext): UpgradeCap {
        package::test_publish(object::id_from_address(@social_contracts), ctx)
    }

    #[test_only]
    public fun test_make_foreign_upgrade_cap(ctx: &mut TxContext): UpgradeCap {
        package::test_publish(object::id_from_address(@0xBADBAD), ctx)
    }

    #[test_only]
    public fun test_force_account_version(account: &mut MemoryAccount, v: u64) {
        set_version(&mut account.id, v);
    }

    #[test_only]
    public fun test_force_registry_version(registry: &mut MemoryRegistry, v: u64) {
        set_version(&mut registry.id, v);
    }

    #[test_only]
    public fun test_strip_account_version(account: &mut MemoryAccount) {
        if (df::exists_with_type<vector<u8>, u64>(&account.id, VERSION_DF_KEY)) {
            let _: u64 = df::remove(&mut account.id, VERSION_DF_KEY);
        }
    }

    #[test_only]
    public fun test_strip_registry_version(registry: &mut MemoryRegistry) {
        if (df::exists_with_type<vector<u8>, u64>(&registry.id, VERSION_DF_KEY)) {
            let _: u64 = df::remove(&mut registry.id, VERSION_DF_KEY);
        }
    }
}
