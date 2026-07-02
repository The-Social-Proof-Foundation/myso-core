#[test_only]
module messaging::paid_message_claim_settled_tests;

use messaging::group_manager::GroupManager;
use messaging::message_log::{Self, MessageLog};
use messaging::paid_messaging_policy::PaidMessagingRegistry;
use messaging::messaging::{
    Self,
    Messaging,
    MessagingNamespace,
};
use messaging::version::{Self, Version};
use myso::permissioned_group::PermissionedGroup;
use social_contracts::block_list::BlockListRegistry;
use social_contracts::profile::{Self, EcosystemTreasury};
use social_contracts::social_graph::SocialGraph;
use myso::clock;
use myso::coin::{Self, Coin};
use myso::myso::MYSO;
use myso::test_scenario as ts;
use myso::vec_set;
use std::string;
use std::unit_test::assert_eq;

const ALICE: address = @0xA11CE;
const BOB: address = @0xB0B;
const PLATFORM_ADDR: address = @0xBADF00;
const TEST_UUID: vector<u8> = b"550e8400-e29b-41d4-a716-4466554400ab";

#[test]
fun reply_claim_settled_resolves_ecosystem_treasury_on_chain() {
    let mut s = ts::begin(ALICE);

    s.next_tx(ALICE);
    let clock = clock::create_for_testing(s.ctx());
    profile::init_for_testing(&clock, s.ctx());
    messaging::init_for_testing_with_clock(&clock, s.ctx());
    version::init_for_testing(s.ctx());
    clock::share_for_testing(clock);

    s.next_tx(ALICE);
    let version = s.take_shared<Version>();
    let mut namespace = s.take_shared<MessagingNamespace>();
    let group_manager = s.take_shared<GroupManager>();
    let block_list = s.take_shared<BlockListRegistry>();
    let mut members = vec_set::empty();
    vec_set::insert(&mut members, BOB);
    let (group, encryption_history, msg_log) = messaging::create_group_unchecked(
        &version,
        &mut namespace,
        &group_manager,
        &block_list,
        string::utf8(b"Paid settled"),
        string::utf8(TEST_UUID),
        b"dek",
        members,
        s.ctx(),
    );

    let msg_log_id = object::id(&msg_log);
    let gid = object::id(&group);
    transfer::public_share_object(group);
    transfer::public_share_object(encryption_history);
    transfer::public_share_object(msg_log);
    ts::return_shared(version);
    ts::return_shared(namespace);
    ts::return_shared(group_manager);
    ts::return_shared(block_list);

    s.next_tx(ALICE);
    let version = s.take_shared<Version>();
    let group = s.take_shared_by_id<PermissionedGroup<Messaging>>(gid);
    let mut msg_log = s.take_shared_by_id<MessageLog>(msg_log_id);
    let paid_registry = s.take_shared<PaidMessagingRegistry>();
    let social_graph = s.take_shared<SocialGraph>();
    let group_manager = s.take_shared<GroupManager>();
    let block_list = s.take_shared<BlockListRegistry>();
    let clock = s.take_shared<clock::Clock>();
    let payment = coin::mint_for_testing<MYSO>(10_000, s.ctx());
    messaging::send_paid_message_digest(
        &version,
        &group,
        &mut msg_log,
        &paid_registry,
        &social_graph,
        &block_list,
        &group_manager,
        BOB,
        payment,
        10_000,
        b"dedupe-send",
        1u128,
        &clock,
        s.ctx(),
    );
    ts::return_shared(version);
    ts::return_shared(group);
    ts::return_shared(msg_log);
    ts::return_shared(paid_registry);
    ts::return_shared(social_graph);
    ts::return_shared(group_manager);
    ts::return_shared(block_list);
    ts::return_shared(clock);

    s.next_tx(BOB);
    let version = s.take_shared<Version>();
    let group = s.take_shared_by_id<PermissionedGroup<Messaging>>(gid);
    let mut msg_log = s.take_shared_by_id<MessageLog>(msg_log_id);
    let block_list = s.take_shared<BlockListRegistry>();
    let ecosystem_treasury = s.take_shared<EcosystemTreasury>();
    let eco_addr = profile::get_treasury_address(&ecosystem_treasury);
    let clock = s.take_shared<clock::Clock>();
    messaging::reply_to_paid_message_claim_settled(
        &version,
        &group,
        &mut msg_log,
        &block_list,
        0,
        10,
        b"dedupe-claim",
        2u128,
        &clock,
        PLATFORM_ADDR,
        &ecosystem_treasury,
        s.ctx(),
    );
    ts::return_shared(version);
    ts::return_shared(group);
    ts::return_shared(msg_log);
    ts::return_shared(block_list);
    ts::return_shared(ecosystem_treasury);
    ts::return_shared(clock);

    s.next_tx(PLATFORM_ADDR);
    let platform_coin = s.take_from_sender<Coin<MYSO>>();
    assert_eq!(coin::value(&platform_coin), 250);
    coin::burn_for_testing(platform_coin);

    s.next_tx(eco_addr);
    let eco_coin = s.take_from_sender<Coin<MYSO>>();
    assert_eq!(coin::value(&eco_coin), 250);
    coin::burn_for_testing(eco_coin);

    s.next_tx(BOB);
    let recipient_coin = s.take_from_sender<Coin<MYSO>>();
    assert_eq!(coin::value(&recipient_coin), 9_500);
    coin::burn_for_testing(recipient_coin);

    s.next_tx(BOB);
    let msg_log = s.take_shared_by_id<MessageLog>(msg_log_id);
    assert_eq!(message_log::next_seq(&msg_log), 1);
    ts::return_shared(msg_log);

    s.end();
}
