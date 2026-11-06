/// MLS device registry (E2EE M6: certificate renewal).
///
/// Anchors each messaging device's long-lived **authority key** (ed25519) to its account on-chain,
/// so device certificates can be renewed by the device alone (no new zkLogin proof), verified by
/// anyone at any time, and revoked.
///
/// - `register`: the **account** (tx sender; for zkLogin the chain checked the proof when the tx ran)
///   binds `(owner, device_id)` to an authority key. Once per device id; gas sponsored by the app.
/// - `rotate`: the **device itself** replaces its authority key. The tx sender must be the current
///   authority key's address, so no account signature (and no zkLogin proof) is needed. The old key
///   keeps verifying for `ROTATION_GRACE_MS`, so certificates in flight don't fail.
/// - `revoke` (by the account) / `revoke_by_device` (by any active device of the same account,
///   including itself): permanent for that device id; both keys stop verifying at once.
///
/// The registry is a derived object of `MessagingNamespace` (key `mls_device_registry`), so its
/// address is deterministic. New chains create it in `messaging::create_genesis_objects`; an
/// existing chain creates it once via `messaging::create_mls_device_registry`.
///
/// Verification rule (clients; docs/myso-messaging-stack/E2EE-Protocol-MLS.md): a v2 device
/// certificate signed by key K is valid iff the record for `(wallet, device_id)` exists, is not
/// revoked, and K is `authority_key`, or K is `previous_key` and now < `previous_valid_until_ms`.
module messaging::mls_device_registry;

use messaging::version::Version;
use myso::address;
use myso::clock::Clock;
use myso::derived_object;
use myso::event;
use myso::hash;
use myso::table::{Self, Table};
use std::string::String;

// === Errors ===

const EBadKeyLength: u64 = 0;
const EAlreadyRegistered: u64 = 1;
const ENotFound: u64 = 2;
const ENotAuthorized: u64 = 3;
const ERevoked: u64 = 4;
const ESameKey: u64 = 5;
const EBadDeviceId: u64 = 6;
const EKeyInUse: u64 = 7;

// === Constants ===

const MLS_DEVICE_REGISTRY_DERIVATION_KEY: vector<u8> = b"mls_device_registry";
const ED25519_KEY_LENGTH: u64 = 32;
const MAX_DEVICE_ID_LENGTH: u64 = 128;
/// After `rotate`, certificates under the previous key keep verifying this long (7 days).
const ROTATION_GRACE_MS: u64 = 604_800_000;

// === Storage ===

public struct MlsDeviceRegistry has key {
    id: UID,
    devices: Table<DeviceKey, DeviceRecord>,
    /// Current authority key address → its device (authorizes `rotate` / `revoke_by_device`).
    by_authority: Table<address, DeviceKey>,
}

/// Table key. BCS: `owner (32 bytes) || device_id (ULEB128 length || UTF-8)`.
public struct DeviceKey has copy, drop, store {
    owner: address,
    device_id: String,
}

public struct DeviceRecord has copy, drop, store {
    /// Current ed25519 authority public key (32 bytes).
    authority_key: vector<u8>,
    /// Key replaced by the last rotation (empty if never rotated).
    previous_key: vector<u8>,
    /// `previous_key` verifies until this time (0 if never rotated).
    previous_valid_until_ms: u64,
    /// 1 at registration, +1 per rotation.
    generation: u64,
    registered_at_ms: u64,
    /// 0 = active.
    revoked_at_ms: u64,
}

// === Events ===

public struct MlsDeviceRegistered has copy, drop {
    owner: address,
    device_id: String,
    authority_key: vector<u8>,
}

public struct MlsDeviceKeyRotated has copy, drop {
    owner: address,
    device_id: String,
    authority_key: vector<u8>,
    previous_key: vector<u8>,
    generation: u64,
}

public struct MlsDeviceRevoked has copy, drop {
    owner: address,
    device_id: String,
    /// The device that revoked it (`none` = the account itself).
    by_device: Option<String>,
}

// === Lifecycle (package) ===

public(package) fun new(namespace_uid: &mut UID, ctx: &mut TxContext): MlsDeviceRegistry {
    MlsDeviceRegistry {
        id: derived_object::claim(namespace_uid, MLS_DEVICE_REGISTRY_DERIVATION_KEY.to_string()),
        devices: table::new(ctx),
        by_authority: table::new(ctx),
    }
}

public(package) fun share(self: MlsDeviceRegistry) {
    transfer::share_object(self);
}

/// Matches `MLS_DEVICE_REGISTRY_DERIVATION_KEY` in the TS SDK.
public(package) fun derivation_key(): String {
    MLS_DEVICE_REGISTRY_DERIVATION_KEY.to_string()
}

// === Helpers ===

/// The MySo address of an ed25519 public key: `blake2b256(0x00 || key)`.
public fun authority_address(key: &vector<u8>): address {
    let mut bytes = vector[0u8];
    bytes.append(*key);
    address::from_bytes(hash::blake2b256(&bytes))
}

fun assert_key(key: &vector<u8>) {
    assert!(key.length() == ED25519_KEY_LENGTH, EBadKeyLength);
}

fun assert_device_id(device_id: &String) {
    let len = device_id.length();
    assert!(len > 0 && len <= MAX_DEVICE_ID_LENGTH, EBadDeviceId);
}

fun do_revoke(
    registry: &mut MlsDeviceRegistry,
    key: DeviceKey,
    by_device: Option<String>,
    clock: &Clock,
) {
    assert!(registry.devices.contains(key), ENotFound);
    let record = registry.devices.borrow_mut(key);
    assert!(record.revoked_at_ms == 0, ERevoked);
    record.revoked_at_ms = clock.timestamp_ms();
    let authority = authority_address(&record.authority_key);
    if (registry.by_authority.contains(authority)) {
        registry.by_authority.remove(authority);
    };
    event::emit(MlsDeviceRevoked { owner: key.owner, device_id: key.device_id, by_device });
}

/// The active device whose current authority key sent this tx.
fun sender_device(registry: &MlsDeviceRegistry, ctx: &TxContext): DeviceKey {
    let sender = ctx.sender();
    assert!(registry.by_authority.contains(sender), ENotAuthorized);
    let key = *registry.by_authority.borrow(sender);
    assert!(registry.devices.borrow(key).revoked_at_ms == 0, ERevoked);
    key
}

// === Entry points ===

/// The account (tx sender) registers one of its devices. Gas is sponsored by the app.
public fun register(
    registry: &mut MlsDeviceRegistry,
    version: &Version,
    device_id: String,
    authority_key: vector<u8>,
    clock: &Clock,
    ctx: &TxContext,
) {
    version.validate_version();
    assert_device_id(&device_id);
    assert_key(&authority_key);
    let key = DeviceKey { owner: ctx.sender(), device_id };
    assert!(!registry.devices.contains(key), EAlreadyRegistered);
    let authority = authority_address(&authority_key);
    assert!(!registry.by_authority.contains(authority), EKeyInUse);
    registry
        .devices
        .add(
            key,
            DeviceRecord {
                authority_key,
                previous_key: vector[],
                previous_valid_until_ms: 0,
                generation: 1,
                registered_at_ms: clock.timestamp_ms(),
                revoked_at_ms: 0,
            },
        );
    registry.by_authority.add(authority, key);
    event::emit(MlsDeviceRegistered { owner: key.owner, device_id: key.device_id, authority_key });
}

/// The device (tx sender = its current authority key's address) rotates to `new_key`.
public fun rotate(
    registry: &mut MlsDeviceRegistry,
    version: &Version,
    new_key: vector<u8>,
    clock: &Clock,
    ctx: &TxContext,
) {
    version.validate_version();
    assert_key(&new_key);
    let key = sender_device(registry, ctx);
    assert!(registry.devices.borrow(key).authority_key != new_key, ESameKey);
    let new_authority = authority_address(&new_key);
    assert!(!registry.by_authority.contains(new_authority), EKeyInUse);
    let record = registry.devices.borrow_mut(key);
    let old = record.authority_key;
    record.previous_key = old;
    record.previous_valid_until_ms = clock.timestamp_ms() + ROTATION_GRACE_MS;
    record.authority_key = new_key;
    record.generation = record.generation + 1;
    let generation = record.generation;
    registry.by_authority.remove(ctx.sender());
    registry.by_authority.add(new_authority, key);
    event::emit(MlsDeviceKeyRotated {
        owner: key.owner,
        device_id: key.device_id,
        authority_key: new_key,
        previous_key: old,
        generation,
    });
}

/// The account revokes one of its devices (permanent for that device id).
public fun revoke(
    registry: &mut MlsDeviceRegistry,
    version: &Version,
    device_id: String,
    clock: &Clock,
    ctx: &TxContext,
) {
    version.validate_version();
    do_revoke(registry, DeviceKey { owner: ctx.sender(), device_id }, option::none(), clock);
}

/// An active device of the same account (tx sender = its authority key's address) revokes
/// `device_id`: another device, or itself (logout). No account signature needed.
public fun revoke_by_device(
    registry: &mut MlsDeviceRegistry,
    version: &Version,
    device_id: String,
    clock: &Clock,
    ctx: &TxContext,
) {
    version.validate_version();
    let revoker = sender_device(registry, ctx);
    let target = DeviceKey { owner: revoker.owner, device_id };
    do_revoke(registry, target, option::some(revoker.device_id), clock);
}

// === Reads ===

public fun device(
    registry: &MlsDeviceRegistry,
    owner: address,
    device_id: String,
): Option<DeviceRecord> {
    let key = DeviceKey { owner, device_id };
    if (registry.devices.contains(key)) option::some(*registry.devices.borrow(key))
    else option::none()
}

/// Whether a certificate signed by `key` verifies for `(owner, device_id)` at `now_ms`.
public fun is_valid_key(
    registry: &MlsDeviceRegistry,
    owner: address,
    device_id: String,
    key: vector<u8>,
    now_ms: u64,
): bool {
    let k = DeviceKey { owner, device_id };
    if (!registry.devices.contains(k)) return false;
    let r = registry.devices.borrow(k);
    if (r.revoked_at_ms != 0) return false;
    key == r.authority_key || (key == r.previous_key && now_ms < r.previous_valid_until_ms)
}

public fun authority_key(r: &DeviceRecord): vector<u8> { r.authority_key }

public fun previous_key(r: &DeviceRecord): vector<u8> { r.previous_key }

public fun previous_valid_until_ms(r: &DeviceRecord): u64 { r.previous_valid_until_ms }

public fun generation(r: &DeviceRecord): u64 { r.generation }

public fun registered_at_ms(r: &DeviceRecord): u64 { r.registered_at_ms }

public fun revoked_at_ms(r: &DeviceRecord): u64 { r.revoked_at_ms }

public fun rotation_grace_ms(): u64 { ROTATION_GRACE_MS }
