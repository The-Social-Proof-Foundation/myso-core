// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(deprecated_usage)] // TODO: update tests to not use deprecated governance
module bridge::committee_test;

use bridge::chain_ids;
use bridge::committee::{
    BridgeCommittee,
    CommitteeMember,
    blocklisted,
    bridge_pubkey_bytes,
    create,
    members,
    member_registrations,
    register,
    try_create_next_committee,
    verify_signatures,
    voting_power,
    execute_blocklist,
    make_committee_member,
    make_bridge_committee
};
use bridge::crypto;
use bridge::message::{Self, BridgeMessage};
use std::unit_test::{destroy, assert_eq};
use myso::ecdsa_k1::{secp256k1_keypair_from_seed, secp256k1_sign, KeyPair};
use myso::hex;
use myso::test_scenario;
use myso::vec_map;
use myso_system::governance_test_utils::{
    advance_epoch_with_reward_amounts,
    create_myso_system_state_for_testing,
    create_validator_for_testing
};
use myso_system::myso_system::{Self, MySoSystemState};

// This is a token transfer message for testing
const TEST_MSG: vector<u8> =
    b"00010a0000000000000000200000000000000000000000000000000000000000000000000000000000000064012000000000000000000000000000000000000000000000000000000000000000c8033930000000000000";

const VALIDATOR1_PUBKEY: vector<u8> =
    b"029bef8d556d80e43ae7e0becb3a7e6838b95defe45896ed6075bb9035d06c9964";
const VALIDATOR2_PUBKEY: vector<u8> =
    b"033e99a541db69bd32040dfe5037fbf5210dafa8151a71e21c5204b05d95ce0a62";
const VALIDATOR3_PUBKEY: vector<u8> =
    b"033e99a541db69bd32040dfe5037fbf5210dafa8151a71e21c5204b05d95ce0a63";

const MYSO_MESSAGE_PREFIX: vector<u8> = b"MYSO_BRIDGE_MESSAGE";

#[test]
fun test_verify_signatures_good_path() {
    let (committee, kp1, kp2) = setup_signing_committee();
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    committee.verify_signatures(msg, vector[sign_test_msg(&kp1, &msg), sign_test_msg(&kp2, &msg)]);
    destroy(committee)
}

#[test, expected_failure(abort_code = bridge::committee::EDuplicatedSignature)]
fun test_verify_signatures_duplicated_sig() {
    let (committee, kp1, _kp2) = setup_signing_committee();
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    let sig = sign_test_msg(&kp1, &msg);
    committee.verify_signatures(msg, vector[sig, sig]);
    abort
}

#[test, expected_failure(abort_code = bridge::committee::EInvalidSignature)]
fun test_verify_signatures_invalid_signature() {
    let (committee, _kp1, _kp2) = setup_signing_committee();
    let outsider = secp256k1_keypair_from_seed(&signing_seed(9));
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    committee.verify_signatures(msg, vector[sign_test_msg(&outsider, &msg)]);
    abort
}

#[test, expected_failure(abort_code = bridge::committee::ESignatureBelowThreshold)]
fun test_verify_signatures_below_threshold() {
    let (committee, kp1, _kp2) = setup_signing_committee();
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    committee.verify_signatures(msg, vector[sign_test_msg(&kp1, &msg)]);
    abort
}

#[test]
fun test_init_committee() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);

    // validator registration
    committee.register(
        &mut system_state,
        hex::decode(VALIDATOR1_PUBKEY),
        b"",
        &tx(@0xA, 0),
    );
    committee.register(
        &mut system_state,
        hex::decode(VALIDATOR2_PUBKEY),
        b"",
        &tx(@0xC, 0),
    );

    // Check committee before creation
    assert!(committee.members().is_empty());

    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = system_state.validator_voting_powers_for_testing();
    committee.try_create_next_committee(voting_powers, 6000, ctx);

    assert_eq!(2, committee.members().length());
    let (_, member0) = committee.members().get_entry_by_idx(0);
    let (_, member1) = committee.members().get_entry_by_idx(1);
    assert_eq!(5000, member0.voting_power());
    assert_eq!(5000, member1.voting_power());

    let members = committee.members();
    assert!(members.length() == 2); // must succeed

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test]
fun test_update_node_url() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[create_validator_for_testing(@0xA, 100, ctx)];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);

    // validator registration
    committee.register(
        &mut system_state,
        hex::decode(VALIDATOR1_PUBKEY),
        b"test url 1",
        &tx(@0xA, 0),
    );

    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = system_state.validator_voting_powers_for_testing();
    committee.try_create_next_committee(voting_powers, 6000, ctx);

    let members = committee.members();
    assert!(members.length() == 1);
    let (_, member) = members.get_entry_by_idx(0);
    assert_eq!(member.http_rest_url(), b"test url 1");

    // Update URL
    committee.update_node_url(
        b"test url 2",
        &tx(@0xA, 0),
    );

    let members = committee.members();
    let (_, member) = members.get_entry_by_idx(0);
    assert_eq!(member.http_rest_url(), b"test url 2");

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test, expected_failure(abort_code = bridge::committee::ESenderIsNotInBridgeCommittee)]
fun test_update_node_url_not_validator() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[create_validator_for_testing(@0xA, 100, ctx)];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);

    // validator registration
    committee.register(
        &mut system_state,
        hex::decode(VALIDATOR1_PUBKEY),
        b"test url 1",
        &tx(@0xA, 0),
    );

    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = system_state.validator_voting_powers_for_testing();
    committee.try_create_next_committee(voting_powers, 6000, ctx);

    // Update URL should fail for validator @0xB
    committee.update_node_url(
        b"test url",
        &tx(@0xB, 0),
    );

    // test should have failed, abort
    abort
}

#[test, expected_failure(abort_code = bridge::committee::ENotSystemAddress)]
fun test_init_non_system_sender() {
    let mut scenario = test_scenario::begin(@0x1);
    let ctx = test_scenario::ctx(&mut scenario);
    let _committee = create(ctx);

    abort
}

#[test, expected_failure(abort_code = bridge::committee::ESenderNotActiveValidator)]
fun test_init_committee_not_validator() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);

    // validator registration
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xD, 0));

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test, expected_failure(abort_code = bridge::committee::EDuplicatePubkey)]
fun test_init_committee_dup_pubkey() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);

    // validator registration
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xC, 0));

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test]
fun test_init_committee_validator_become_inactive() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
        create_validator_for_testing(@0xD, 100, ctx),
        create_validator_for_testing(@0xE, 100, ctx),
        create_validator_for_testing(@0xF, 100, ctx),
    ];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);

    // validator registration, 3 validators registered, should have 60% voting power in total
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));
    committee.register(&mut system_state, hex::decode(VALIDATOR2_PUBKEY), b"", &tx(@0xC, 0));
    committee.register(&mut system_state, hex::decode(VALIDATOR3_PUBKEY), b"", &tx(@0xD, 0));

    // Verify validator registration
    assert_eq!(3, committee.member_registrations().length());

    // Validator 0xA become inactive, total voting power become 50%
    myso_system::request_remove_validator(&mut system_state, &mut tx(@0xA, 0));
    test_scenario::return_shared(system_state);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);

    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);

    // create committee should not create a committe because of not enough stake.
    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = myso_system::validator_voting_powers_for_testing(&mut system_state);
    try_create_next_committee(&mut committee, voting_powers, 6000, ctx);

    assert!(committee.members().is_empty());

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test]
fun test_update_committee_registration() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);

    // validator registration
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));

    // Verify registration info
    assert_eq!(1, committee.member_registrations().length());
    let (address, registration) = committee.member_registrations().get_entry_by_idx(0);
    assert_eq!(@0xA, *address);
    assert!(&hex::decode(VALIDATOR1_PUBKEY) == registration.bridge_pubkey_bytes(), 0);

    // Register again with different pub key.
    committee.register(&mut system_state, hex::decode(VALIDATOR2_PUBKEY), b"", &tx(@0xA, 0));

    // Verify registration info, registration count should still be 1
    assert_eq!(1, committee.member_registrations().length());
    let (address, registration) = committee.member_registrations().get_entry_by_idx(0);
    assert_eq!(@0xA, *address);
    assert!(&hex::decode(VALIDATOR2_PUBKEY) == registration.bridge_pubkey_bytes(), 0);

    // teardown
    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test]
fun test_init_committee_not_enough_stake() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);

    // validator registration
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));

    // Check committee before creation
    assert!(committee.members().is_empty());

    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = myso_system::validator_voting_powers_for_testing(&mut system_state);
    try_create_next_committee(&mut committee, voting_powers, 6000, ctx);

    // committee should be empty because registration did not reach min stake threshold.
    assert!(committee.members().is_empty());

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test, expected_failure(abort_code = bridge::committee::ECommitteeAlreadyInitiated)]
fun test_register_already_initialized() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);

    test_scenario::next_tx(&mut scenario, @0x0);
    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));
    committee.register(&mut system_state, hex::decode(VALIDATOR2_PUBKEY), b"", &tx(@0xC, 0));
    assert!(committee.members().is_empty());
    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = myso_system::validator_voting_powers_for_testing(&mut system_state);
    try_create_next_committee(&mut committee, voting_powers, 6000, ctx);

    test_scenario::next_tx(&mut scenario, @0x0);
    assert!(committee.members().length() == 2); // must succeed
    // this fails because committee is already initiated
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));

    abort
}

#[test, expected_failure(abort_code = bridge::committee::EInvalidPubkeyLength)]
fun test_register_bad_pubkey() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_myso_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);

    test_scenario::next_tx(&mut scenario, @0x0);
    let mut system_state = test_scenario::take_shared<MySoSystemState>(&scenario);
    committee.register(&mut system_state, hex::decode(VALIDATOR2_PUBKEY), b"", &tx(@0xC, 0));
    // this fails with invalid public key
    committee.register(&mut system_state, b"029bef8", b"", &tx(@0xA, 0));

    abort
}

fun tx(sender: address, hint: u64): TxContext {
    tx_context::new_from_hint(sender, hint, 1, 0, 0)
}

#[test, expected_failure(abort_code = bridge::committee::ESignatureBelowThreshold)]
fun test_verify_signatures_with_blocked_committee_member() {
    let (mut committee, kp1, kp2) = setup_signing_committee();
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    committee.verify_signatures(msg, vector[sign_test_msg(&kp1, &msg), sign_test_msg(&kp2, &msg)]);

    let (validator1, member) = committee.members().get_entry_by_idx(0);
    assert!(!member.blocklisted());

    let blocklist = message::create_blocklist_message(
        chain_ids::myso_testnet(),
        0,
        0,
        vector[crypto::ecdsa_pub_key_to_eth_address(validator1)],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    let (_, blocked_member) = committee.members().get_entry_by_idx(0);
    assert!(blocked_member.blocklisted());

    committee.verify_signatures(msg, vector[sign_test_msg(&kp1, &msg), sign_test_msg(&kp2, &msg)]);

    destroy(committee);
}

#[test, expected_failure(abort_code = bridge::committee::EValidatorBlocklistContainsUnknownKey)]
fun test_execute_blocklist_abort_upon_unknown_validator() {
    let mut committee = setup_test();

    // // val0 and val1 are not blocked yet
    let (validator0, _) = committee.members().get_entry_by_idx(0);
    // assert!(!member0.blocklisted());
    // let (validator1, member1) = committee.members().get_entry_by_idx(1);
    // assert!(!member1.blocklisted());

    let eth_address0 = crypto::ecdsa_pub_key_to_eth_address(validator0);
    let invalid_eth_address1 = x"0000000000000000000000000000000000000000";

    // Blocklist both
    let blocklist = message::create_blocklist_message(
        chain_ids::myso_testnet(),
        0, // seq
        0, // type 0 is blocklist
        vector[eth_address0, invalid_eth_address1],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    // Clean up
    destroy(committee);
}

#[test]
fun test_execute_blocklist() {
    let mut committee = setup_test();

    // val0 and val1 are not blocked yet
    let (validator0, member0) = committee.members().get_entry_by_idx(0);
    assert!(!member0.blocklisted());
    let (validator1, member1) = committee.members().get_entry_by_idx(1);
    assert!(!member1.blocklisted());

    let eth_address0 = crypto::ecdsa_pub_key_to_eth_address(validator0);
    let eth_address1 = crypto::ecdsa_pub_key_to_eth_address(validator1);

    // Blocklist both
    let blocklist = message::create_blocklist_message(
        chain_ids::myso_testnet(),
        0, // seq
        0, // type 0 is blocklist
        vector[eth_address0, eth_address1],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    // Blocklist both reverse order
    let blocklist = message::create_blocklist_message(
        chain_ids::myso_testnet(),
        0, // seq
        0, // type 0 is blocklist
        vector[eth_address1, eth_address0],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    // val 0 is blocklisted
    let (_, blocked_member) = committee.members().get_entry_by_idx(0);
    assert!(blocked_member.blocklisted());
    // val 1 is too
    let (_, blocked_member) = committee.members().get_entry_by_idx(1);
    assert!(blocked_member.blocklisted());

    // unblocklist val1
    let blocklist = message::create_blocklist_message(
        chain_ids::myso_testnet(),
        1, // seq, this is supposed to increment, but we don't test it here
        1, // type 1 is unblocklist
        vector[eth_address1],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    // val 0 is still blocklisted
    let (_, blocked_member) = committee.members().get_entry_by_idx(0);
    assert!(blocked_member.blocklisted());
    // val 1 is not
    let (_, blocked_member) = committee.members().get_entry_by_idx(1);
    assert!(!blocked_member.blocklisted());

    // Clean up
    destroy(committee);
}

fun signing_seed(tag: u8): vector<u8> {
    let mut seed = vector[];
    let mut i = 0u64;
    while (i < 31) {
        seed.push_back(0);
        i = i + 1;
    };
    seed.push_back(tag);
    seed
}

fun sign_test_msg(kp: &KeyPair, msg: &BridgeMessage): vector<u8> {
    let mut message_bytes = MYSO_MESSAGE_PREFIX;
    message_bytes.append((*msg).serialize_message());
    secp256k1_sign(kp.private_key(), &message_bytes, 0, true)
}

fun setup_signing_committee(): (BridgeCommittee, KeyPair, KeyPair) {
    let kp1 = secp256k1_keypair_from_seed(&signing_seed(1));
    let kp2 = secp256k1_keypair_from_seed(&signing_seed(2));
    let mut members = vec_map::empty<vector<u8>, CommitteeMember>();

    let pk1 = *kp1.public_key();
    members.insert(
        pk1,
        make_committee_member(@0xA, pk1, 3333, b"https://127.0.0.1:9191", false),
    );
    let pk2 = *kp2.public_key();
    members.insert(
        pk2,
        make_committee_member(@0xC, pk2, 3333, b"https://127.0.0.1:9192", false),
    );

    (make_bridge_committee(members, vec_map::empty(), 1), kp1, kp2)
}

fun setup_test(): BridgeCommittee {
    let mut members = vec_map::empty<vector<u8>, CommitteeMember>();

    let bridge_pubkey_bytes = hex::decode(VALIDATOR1_PUBKEY);
    members.insert(
        bridge_pubkey_bytes,
        make_committee_member(
            @0xA,
            bridge_pubkey_bytes,
            3333,
            b"https://127.0.0.1:9191",
            false,
        ),
    );

    let bridge_pubkey_bytes = hex::decode(VALIDATOR2_PUBKEY);
    members.insert(
        bridge_pubkey_bytes,
        make_committee_member(
            @0xC,
            bridge_pubkey_bytes,
            3333,
            b"https://127.0.0.1:9192",
            false,
        ),
    );

    make_bridge_committee(members, vec_map::empty(), 1)
}
