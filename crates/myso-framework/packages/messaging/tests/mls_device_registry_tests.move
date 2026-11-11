#[test_only]
module messaging::mls_device_registry_tests;

use messaging::messaging::{Self as msg, MessagingNamespace};
use messaging::mls_device_registry::{Self as reg, MlsDeviceRegistry};
use messaging::version::Version;
use myso::clock::Clock;
use myso::test_scenario::{Self as ts, Scenario};
use std::string::{Self, String};

const ALICE: address = @0xA11CE;
const BOB: address = @0xB0B;

const KEY_A: vector<u8> = x"1111111111111111111111111111111111111111111111111111111111111111";
const KEY_B: vector<u8> = x"2222222222222222222222222222222222222222222222222222222222222222";
const KEY_C: vector<u8> = x"3333333333333333333333333333333333333333333333333333333333333333";
const KEY_D: vector<u8> = x"4444444444444444444444444444444444444444444444444444444444444444";

fun id(s: vector<u8>): String { string::utf8(s) }

/// Genesis objects + a clock at t = 1_000.
fun setup(): Scenario {
    let mut s = ts::begin(@0x0);
    msg::init_for_testing(s.ctx());
    s.next_tx(@0x0);
    let mut clock = s.take_shared<Clock>();
    clock.set_for_testing(1_000);
    ts::return_shared(clock);
    s
}

/// Run `register` as `owner`.
fun register(s: &mut Scenario, owner: address, device: vector<u8>, key: vector<u8>) {
    s.next_tx(owner);
    let mut r = s.take_shared<MlsDeviceRegistry>();
    let v = s.take_shared<Version>();
    let c = s.take_shared<Clock>();
    r.register(&v, id(device), key, &c, s.ctx());
    ts::return_shared(r);
    ts::return_shared(v);
    ts::return_shared(c);
}

/// Run `rotate` signed by the device's current authority key (`sender_key`).
fun rotate(s: &mut Scenario, sender_key: vector<u8>, new_key: vector<u8>) {
    s.next_tx(reg::authority_address(&sender_key));
    let mut r = s.take_shared<MlsDeviceRegistry>();
    let v = s.take_shared<Version>();
    let c = s.take_shared<Clock>();
    r.rotate(&v, new_key, &c, s.ctx());
    ts::return_shared(r);
    ts::return_shared(v);
    ts::return_shared(c);
}

fun revoke_by_device(s: &mut Scenario, sender_key: vector<u8>, target: vector<u8>) {
    s.next_tx(reg::authority_address(&sender_key));
    let mut r = s.take_shared<MlsDeviceRegistry>();
    let v = s.take_shared<Version>();
    let c = s.take_shared<Clock>();
    r.revoke_by_device(&v, id(target), &c, s.ctx());
    ts::return_shared(r);
    ts::return_shared(v);
    ts::return_shared(c);
}

fun valid(s: &mut Scenario, owner: address, device: vector<u8>, key: vector<u8>, now: u64): bool {
    s.next_tx(owner);
    let r = s.take_shared<MlsDeviceRegistry>();
    let ok = r.is_valid_key(owner, id(device), key, now);
    ts::return_shared(r);
    ok
}

fun set_time(s: &mut Scenario, ms: u64) {
    s.next_tx(@0x0);
    let mut c = s.take_shared<Clock>();
    c.set_for_testing(ms);
    ts::return_shared(c);
}

#[test]
fun register_then_verify() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    assert!(valid(&mut s, ALICE, b"phone", KEY_A, 1_000));
    // Wrong key, other device id, other owner: not valid.
    assert!(!valid(&mut s, ALICE, b"phone", KEY_B, 1_000));
    assert!(!valid(&mut s, ALICE, b"laptop", KEY_A, 1_000));
    assert!(!valid(&mut s, BOB, b"phone", KEY_A, 1_000));

    s.next_tx(ALICE);
    let r = s.take_shared<MlsDeviceRegistry>();
    let rec = r.device(ALICE, id(b"phone")).destroy_some();
    assert!(rec.generation() == 1);
    assert!(rec.registered_at_ms() == 1_000);
    assert!(rec.revoked_at_ms() == 0);
    assert!(rec.previous_key() == vector[]);
    assert!(r.device(BOB, id(b"phone")).is_none());
    ts::return_shared(r);
    s.end();
}

#[test, expected_failure(abort_code = reg::EAlreadyRegistered)]
fun register_twice_fails() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    register(&mut s, ALICE, b"phone", KEY_B);
    s.end();
}

#[test]
fun same_device_id_for_different_owners_is_fine() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    register(&mut s, BOB, b"phone", KEY_B);
    assert!(valid(&mut s, BOB, b"phone", KEY_B, 1_000));
    s.end();
}

#[test, expected_failure(abort_code = reg::EKeyInUse)]
fun one_key_cannot_serve_two_devices() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    register(&mut s, BOB, b"phone", KEY_A);
    s.end();
}

#[test, expected_failure(abort_code = reg::EBadKeyLength)]
fun bad_key_length_fails() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", x"01");
    s.end();
}

#[test, expected_failure(abort_code = reg::EBadDeviceId)]
fun empty_device_id_fails() {
    let mut s = setup();
    register(&mut s, ALICE, b"", KEY_A);
    s.end();
}

#[test]
fun device_rotates_its_own_key_with_a_grace_period() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    set_time(&mut s, 5_000);
    rotate(&mut s, KEY_A, KEY_B); // signed by the device's key, not the account
    let grace = reg::rotation_grace_ms();
    assert!(valid(&mut s, ALICE, b"phone", KEY_B, 5_000));
    // Old key keeps verifying through the grace window, then stops.
    assert!(valid(&mut s, ALICE, b"phone", KEY_A, 5_000 + grace - 1));
    assert!(!valid(&mut s, ALICE, b"phone", KEY_A, 5_000 + grace));

    s.next_tx(ALICE);
    let r = s.take_shared<MlsDeviceRegistry>();
    let rec = r.device(ALICE, id(b"phone")).destroy_some();
    assert!(rec.generation() == 2);
    assert!(rec.authority_key() == KEY_B);
    assert!(rec.previous_key() == KEY_A);
    assert!(rec.previous_valid_until_ms() == 5_000 + grace);
    ts::return_shared(r);

    // Rotating again replaces the previous key: only the newest old key has grace.
    rotate(&mut s, KEY_B, KEY_C);
    assert!(valid(&mut s, ALICE, b"phone", KEY_C, 5_000));
    assert!(valid(&mut s, ALICE, b"phone", KEY_B, 5_000));
    assert!(!valid(&mut s, ALICE, b"phone", KEY_A, 5_000));
    s.end();
}

#[test, expected_failure(abort_code = reg::ENotAuthorized)]
fun the_old_key_cannot_rotate_after_rotation() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    rotate(&mut s, KEY_A, KEY_B);
    rotate(&mut s, KEY_A, KEY_C);
    s.end();
}

#[test, expected_failure(abort_code = reg::ENotAuthorized)]
fun the_account_address_cannot_rotate_a_device_key() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    s.next_tx(ALICE);
    let mut r = s.take_shared<MlsDeviceRegistry>();
    let v = s.take_shared<Version>();
    let c = s.take_shared<Clock>();
    r.rotate(&v, KEY_B, &c, s.ctx());
    abort 0
}

#[test, expected_failure(abort_code = reg::ESameKey)]
fun rotate_to_the_same_key_fails() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    rotate(&mut s, KEY_A, KEY_A);
    s.end();
}

#[test]
fun account_revokes_a_device_and_both_keys_stop_at_once() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    rotate(&mut s, KEY_A, KEY_B);
    set_time(&mut s, 9_000);
    s.next_tx(ALICE);
    let mut r = s.take_shared<MlsDeviceRegistry>();
    let v = s.take_shared<Version>();
    let c = s.take_shared<Clock>();
    r.revoke(&v, id(b"phone"), &c, s.ctx());
    ts::return_shared(r);
    ts::return_shared(v);
    ts::return_shared(c);
    // Revocation beats the rotation grace window.
    assert!(!valid(&mut s, ALICE, b"phone", KEY_B, 9_000));
    assert!(!valid(&mut s, ALICE, b"phone", KEY_A, 9_000));
    s.next_tx(ALICE);
    let r = s.take_shared<MlsDeviceRegistry>();
    assert!(r.device(ALICE, id(b"phone")).destroy_some().revoked_at_ms() == 9_000);
    ts::return_shared(r);
    s.end();
}

#[test, expected_failure(abort_code = reg::EAlreadyRegistered)]
fun a_revoked_device_id_cannot_be_registered_again() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    revoke_by_device(&mut s, KEY_A, b"phone");
    register(&mut s, ALICE, b"phone", KEY_B);
    s.end();
}

#[test, expected_failure(abort_code = reg::ENotAuthorized)]
fun a_revoked_device_key_can_no_longer_act() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    revoke_by_device(&mut s, KEY_A, b"phone"); // logout: revokes itself
    rotate(&mut s, KEY_A, KEY_B);
    s.end();
}

#[test]
fun one_device_revokes_another_of_the_same_account() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    register(&mut s, ALICE, b"laptop", KEY_B);
    revoke_by_device(&mut s, KEY_A, b"laptop"); // lost laptop, revoked from the phone
    assert!(!valid(&mut s, ALICE, b"laptop", KEY_B, 1_000));
    assert!(valid(&mut s, ALICE, b"phone", KEY_A, 1_000));
    s.end();
}

#[test, expected_failure(abort_code = reg::ENotFound)]
fun a_device_cannot_revoke_another_accounts_device() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    register(&mut s, BOB, b"laptop", KEY_D);
    // Bob's device only addresses Bob's devices: Alice's "laptop" doesn't exist under Bob.
    revoke_by_device(&mut s, KEY_D, b"phone");
    s.end();
}

#[test, expected_failure(abort_code = reg::ERevoked)]
fun revoking_twice_fails() {
    let mut s = setup();
    register(&mut s, ALICE, b"phone", KEY_A);
    register(&mut s, ALICE, b"laptop", KEY_B);
    revoke_by_device(&mut s, KEY_A, b"laptop");
    revoke_by_device(&mut s, KEY_A, b"laptop");
    s.end();
}

#[test, expected_failure(abort_code = myso::derived_object::EObjectAlreadyExists)]
fun the_registry_is_a_singleton() {
    let mut s = setup();
    s.next_tx(ALICE);
    let mut ns = s.take_shared<MessagingNamespace>();
    msg::create_mls_device_registry_for_testing(&mut ns, s.ctx());
    abort 0
}
