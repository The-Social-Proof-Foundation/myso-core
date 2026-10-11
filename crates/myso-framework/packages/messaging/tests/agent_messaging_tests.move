#[test_only]
#[allow(duplicate_alias, unused_use, unused_mut_ref, unused_let_mut)]
module messaging::agent_messaging_tests;

use std::option;
use std::string;
use std::unit_test::{assert_eq, destroy};

use myso::clock::{Self, Clock};
use myso::coin;
use myso::event;
use myso::hex;
use myso::myso::MYSO;
use myso::object;
use myso::test_scenario as ts;
use myso::vec_set;

use messaging::group_leaver::GroupLeaver;
use messaging::group_manager::{Self, GroupManager};
use messaging::encryption_history::EncryptionHistory;
use messaging::message_log::MessageLog;
use messaging::messaging::{
    Self as msg,
    Messaging,
    MessagingNamespace,
    MetadataAdmin,
    MessagingReader,
    MessagingSender,
};
use messaging::messaging_config::MessagingConfig;
use messaging::metadata;
use messaging::mydata_policies;
use messaging::paid_messaging_policy::{Self, PaidMessagingRegistry};
use messaging::version::{Self, Version};
use myso::permissioned_group::{PermissionedGroup, PermissionsAdmin};

use social_contracts::block_list::{Self, BlockListRegistry};
use social_contracts::ai_credit::AiCreditConfig;
use social_contracts::memory::{Self, MemoryAccount, MemoryRegistry, SubAgent, AgenticOrganization,
        MemoryConfig};
use social_contracts::memory_test_helpers;
use social_contracts::platform::{Self, Platform, PlatformRegistry,
        PlatformConfig};
use social_contracts::profile::{Self, UsernameRegistry,
        ProfileConfig};
use social_contracts::social_graph::{Self, SocialGraph};

const ADMIN: address = @0xAD;
const AUTHOR: address = @0x1;
const BOB: address = @0xB0B;
const CAROL: address = @0xCA01;
const AGENT_ADDR: address = @0xA11CE;
const CAROL_AGENT_ADDR: address = @0xCA_A11CE;
const AGENT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
const CAROL_AGENT_PUBKEY: vector<u8> = x"0202020202020202020202020202020202020202020202020202020202020202";
const TEST_ENCRYPTED_DEK: vector<u8> = b"test_encrypted_dek";
const TEST_UUID: vector<u8> = b"550e8400-e29b-41d4-a716-446655440010";
const TEST_UUID_PAID: vector<u8> = b"550e8400-e29b-41d4-a716-446655440011";
const TEST_UUID_CROSS: vector<u8> = b"550e8400-e29b-41d4-a716-446655440012";
const TEST_UUID_FOLLOWER: vector<u8> = b"550e8400-e29b-41d4-a716-446655440013";
const TEST_GROUP_NAME: vector<u8> = b"Agent Chat";

fun setup(scenario: &mut ts::Scenario) {
    ts::next_tx(scenario, ADMIN);
    {
        let clock = clock::create_for_testing(ts::ctx(scenario));
        profile::init_for_testing(&clock, ts::ctx(scenario));
        platform::test_init(&clock, ts::ctx(scenario));
        msg::init_for_testing_with_clock(&clock, ts::ctx(scenario));
        version::init_for_testing(ts::ctx(scenario));
        clock::share_for_testing(clock);
    };

    ts::next_tx(scenario, ADMIN);
    {
        let mut registry = ts::take_shared<PlatformRegistry>(scenario);
        let clock = ts::take_shared<Clock>(scenario);
        let platform_config = ts::take_shared<PlatformConfig>(scenario);
        platform::create_platform(
            &mut registry,
            &platform_config,
            string::utf8(b"Messaging Platform"),
            string::utf8(b"tagline"),
            string::utf8(b"desc"),
            string::utf8(b"https://example.com/logo.png"),
            string::utf8(b"https://example.com/tos"),
            string::utf8(b"https://example.com/privacy"),
            vector[string::utf8(b"web")],
            vector[string::utf8(b"https://example.com")],
            string::utf8(b"Social Network"),
            option::none(),
            3,
            string::utf8(b"2023-01-01"),
            false,
            option::none(),
            option::none(),
            option::none(),
            option::none(),
            option::none(),
            option::none(),
            option::none(),
            option::none(),
            option::none(),

            option::none(),
            &clock,
            ts::ctx(scenario),
        );
        ts::return_shared(clock);
        ts::return_shared(platform_config);
        ts::return_shared(registry);
    };

    ts::next_tx(scenario, ADMIN);
    {
        let platform_obj = ts::take_shared<Platform>(scenario);
        let mut registry = ts::take_shared<PlatformRegistry>(scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        platform::test_set_approval(&mut registry, platform_id, true);
        ts::return_shared(platform_obj);
        ts::return_shared(registry);
    };

    ts::next_tx(scenario, AUTHOR);
    {
        let mut registry = ts::take_shared<UsernameRegistry>(scenario);
        let mut memory_registry = ts::take_shared<MemoryRegistry>(scenario);
        let mut ai_credit_config = ts::take_shared<AiCreditConfig>(scenario);
        let clock = ts::take_shared<Clock>(scenario);
        let profile_config = ts::take_shared<ProfileConfig>(scenario);
        profile::create_profile(
            &mut registry,
            &profile_config,
            &mut memory_registry,
            &mut ai_credit_config,
            string::utf8(b"Author"),
            string::utf8(b"author"),
            string::utf8(b"bio"),
            b"",
            b"",
            &clock,
            ts::ctx(scenario),
        );
        ts::return_shared(clock);
        ts::return_shared(profile_config);
        ts::return_shared(ai_credit_config);
        ts::return_shared(memory_registry);
        ts::return_shared(registry);
    };

    ts::next_tx(scenario, AUTHOR);
    {
        let mut platform_obj = ts::take_shared<Platform>(scenario);
        platform::test_join_platform(&mut platform_obj, AUTHOR);
        ts::return_shared(platform_obj);
    };
}

fun register_agent(
    memory_config: &MemoryConfig,
    memory_account: &mut MemoryAccount,
    organization: &mut AgenticOrganization,
    platform_id: address,
    clock: &Clock,
    ctx: &mut myso::tx_context::TxContext,
) {
    memory::register_sub_agent(
        memory_config,
        memory_account,
        organization,
        AGENT_PUBKEY,
        AGENT_ADDR,
        string::utf8(b"messaging-agent"),
        memory::class_delegated_ai(),
        0,
        memory::cap_message_send() | memory::cap_message_read(),
        memory::cap_message_send() | memory::cap_message_read(),
        3,
        0,
        option::none(),
        option::some(platform_id),
        option::none(),
        clock,
        ctx,
    );
}

fun setup_carol_with_agent(scenario: &mut ts::Scenario) {
    ts::next_tx(scenario, CAROL);
    {
        let mut registry = ts::take_shared<UsernameRegistry>(scenario);
        let mut memory_registry = ts::take_shared<MemoryRegistry>(scenario);
        let mut ai_credit_config = ts::take_shared<AiCreditConfig>(scenario);
        let clock = ts::take_shared<Clock>(scenario);
        let profile_config = ts::take_shared<ProfileConfig>(scenario);
        profile::create_profile(
            &mut registry,
            &profile_config,
            &mut memory_registry,
            &mut ai_credit_config,
            string::utf8(b"Carol"),
            string::utf8(b"carol"),
            string::utf8(b"bio"),
            b"",
            b"",
            &clock,
            ts::ctx(scenario),
        );
        ts::return_shared(clock);
        ts::return_shared(profile_config);
        ts::return_shared(ai_credit_config);
        ts::return_shared(memory_registry);
        ts::return_shared(registry);
    };

    ts::next_tx(scenario, CAROL);
    {
        let mut platform_obj = ts::take_shared<Platform>(scenario);
        platform::test_join_platform(&mut platform_obj, CAROL);
        ts::return_shared(platform_obj);
    };

    ts::next_tx(scenario, CAROL);
    {
        memory_test_helpers::create_default_org_in_tx(scenario);
    };

    ts::next_tx(scenario, CAROL);
    {
        let platform_obj = ts::take_shared<Platform>(scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        ts::return_shared(platform_obj);

        let mut org = memory_test_helpers::take_created_org(scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(scenario);
        let clock = ts::take_shared<Clock>(scenario);
        let memory_config = ts::take_shared<MemoryConfig>(scenario);
        memory::register_sub_agent(
            &memory_config,
            &mut memory_account,
            &mut org,
            CAROL_AGENT_PUBKEY,
            CAROL_AGENT_ADDR,
            string::utf8(b"carol-agent"),
            memory::class_delegated_ai(),
            0,
            memory::cap_message_send() | memory::cap_message_read(),
            memory::cap_message_send() | memory::cap_message_read(),
            3,
            0,
            option::none(),
            option::some(platform_id),
            option::none(),
            &clock,
            ts::ctx(scenario),
        );
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
    };
}

#[test]
fun agent_creates_dm_with_human_peer() {
    let mut scenario = ts::begin(AUTHOR);
    setup(&mut scenario);

    ts::next_tx(&mut scenario, AUTHOR);
    {
        memory_test_helpers::create_default_org_in_tx(&mut scenario);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let platform_obj = ts::take_shared<Platform>(&scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let mut org = memory_test_helpers::take_created_org(&mut scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        register_agent(&memory_config, &mut memory_account, &mut org, platform_id, &clock, ts::ctx(&mut scenario));
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
        ts::return_shared(platform_obj);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let mut namespace = ts::take_shared<MessagingNamespace>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let group_leaver = ts::take_shared<GroupLeaver>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);

        let mut members = vec_set::empty<address>();
        vec_set::insert(&mut members, BOB);

        let (group, encryption_history, msg_log) = msg::create_agent_group(
            &version,
            &mut namespace,
            &group_manager,
            &group_leaver,
            &block_list,
            &platform,
            &memory_config,
            &memory_account,
            &memory_account,
            string::utf8(TEST_GROUP_NAME),
            string::utf8(TEST_UUID),
            TEST_ENCRYPTED_DEK,
            members,
            option::none(),
            option::none(),
            &clock,
            ts::ctx(&mut scenario),
        );

        assert!(group.has_permission<Messaging, MessagingSender>(AGENT_ADDR), 0);
        assert!(group.has_permission<Messaging, MessagingReader>(AGENT_ADDR), 1);
        assert!(!group.has_permission<Messaging, PermissionsAdmin>(AGENT_ADDR), 2);
        assert!(group.has_permission<Messaging, MessagingReader>(AUTHOR), 3);
        assert!(group.has_permission<Messaging, PermissionsAdmin>(AUTHOR), 4);
        // The principal is the room curator; the agent creator is not.
        assert!(group.has_permission<Messaging, MetadataAdmin>(AUTHOR), 40);
        assert!(!group.has_permission<Messaging, MetadataAdmin>(AGENT_ADDR), 41);
        assert!(group.has_permission<Messaging, MessagingSender>(BOB), 5);
        assert!(group.has_permission<Messaging, MessagingReader>(BOB), 6);
        assert!(!group.has_permission<Messaging, MessagingSender>(AUTHOR), 7);

        let m = group_manager::borrow_metadata<Messaging>(&group_manager, &group);
        let agent_chat_key = string::utf8(b"agent_chat");
        let agent_chat = metadata::get_data_value(m, &agent_chat_key);
        assert!(option::is_some(&agent_chat), 8);

        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(memory_account);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_leaver);
        ts::return_shared(group_manager);
        ts::return_shared(namespace);
        ts::return_shared(version);
        destroy(group);
        destroy(encryption_history);
        destroy(msg_log);
    };

    scenario.end();
}

const ROOM_UUID: vector<u8> = b"550e8400-e29b-41d4-a716-446655440020";

/// Shared setup: messaging stack, an org, and a registered messaging agent for AUTHOR.
fun prepare_agent(scenario: &mut ts::Scenario) {
    setup(scenario);

    ts::next_tx(scenario, AUTHOR);
    {
        memory_test_helpers::create_default_org_in_tx(scenario);
    };

    ts::next_tx(scenario, AUTHOR);
    {
        let platform_obj = ts::take_shared<Platform>(scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let mut org = memory_test_helpers::take_created_org(scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(scenario);
        let clock = ts::take_shared<Clock>(scenario);
        let memory_config = ts::take_shared<MemoryConfig>(scenario);
        register_agent(&memory_config, &mut memory_account, &mut org, platform_id, &clock, ts::ctx(scenario));
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
        ts::return_shared(platform_obj);
    };
}

/// Creates a group as the agent with the given room parameters and returns after
/// running `check` against it (inlined by each test via the returned flag).
fun agent_create_group(
    scenario: &mut ts::Scenario,
    room_kind: Option<u8>,
    room_task_id: Option<ID>,
    check_room: bool,
    expect_task: bool,
) {
    ts::next_tx(scenario, AGENT_ADDR);
    let version = ts::take_shared<Version>(scenario);
    let mut namespace = ts::take_shared<MessagingNamespace>(scenario);
    let group_manager = ts::take_shared<GroupManager>(scenario);
    let group_leaver = ts::take_shared<GroupLeaver>(scenario);
    let block_list = ts::take_shared<BlockListRegistry>(scenario);
    let platform = ts::take_shared<Platform>(scenario);
    let memory_account = ts::take_shared<MemoryAccount>(scenario);
    let memory_config = ts::take_shared<MemoryConfig>(scenario);
    let clock = ts::take_shared<Clock>(scenario);

    let (group, encryption_history, msg_log) = msg::create_agent_group(
        &version,
        &mut namespace,
        &group_manager,
        &group_leaver,
        &block_list,
        &platform,
        &memory_config,
        &memory_account,
        &memory_account,
        string::utf8(TEST_GROUP_NAME),
        string::utf8(ROOM_UUID),
        TEST_ENCRYPTED_DEK,
        vec_set::empty<address>(),
        room_kind,
        room_task_id,
        &clock,
        ts::ctx(scenario),
    );

    let kind_value = msg::group_data(&group_manager, &group, string::utf8(b"room_kind"));
    let task_value = msg::group_data(&group_manager, &group, string::utf8(b"task_id"));
    let bound_events = event::events_by_type<msg::AgentRoomBound>();
    if (check_room) {
        assert!(group.has_permission<Messaging, MetadataAdmin>(AUTHOR), 0);
        assert!(option::is_some(&kind_value), 1);
        assert!(*option::borrow(&kind_value) == string::utf8(b"agent_room"), 2);
        assert_eq!(bound_events.length(), 1);
        if (expect_task) {
            let expected = string::utf8(hex::encode(option::borrow(&room_task_id).to_bytes()));
            assert!(option::is_some(&task_value), 3);
            assert!(*option::borrow(&task_value) == expected, 4);
        } else {
            assert!(option::is_none(&task_value), 5);
        };
        // The agent-chat tag is still present: a room is a tagged agent group.
        let chat = msg::group_data(&group_manager, &group, string::utf8(b"agent_chat"));
        assert!(option::is_some(&chat), 6);
    } else {
        assert!(option::is_none(&kind_value), 7);
        assert!(option::is_none(&task_value), 8);
        assert_eq!(bound_events.length(), 0);
    };

    ts::return_shared(clock);
    ts::return_shared(memory_config);
    ts::return_shared(memory_account);
    ts::return_shared(platform);
    ts::return_shared(block_list);
    ts::return_shared(group_leaver);
    ts::return_shared(group_manager);
    ts::return_shared(namespace);
    ts::return_shared(version);
    destroy(group);
    destroy(encryption_history);
    destroy(msg_log);
}

#[test]
fun agent_room_is_tagged_and_bound_to_task() {
    let mut scenario = ts::begin(AUTHOR);
    prepare_agent(&mut scenario);
    agent_create_group(
        &mut scenario,
        option::some(msg::room_kind_agent_room()),
        option::some(object::id_from_address(@0x7A5C)),
        true,
        true,
    );
    scenario.end();
}

#[test]
fun agent_room_without_task_binding() {
    let mut scenario = ts::begin(AUTHOR);
    prepare_agent(&mut scenario);
    agent_create_group(&mut scenario, option::some(1), option::none(), true, false);
    scenario.end();
}

#[test]
fun plain_agent_group_has_no_room_tags_and_no_room_event() {
    let mut scenario = ts::begin(AUTHOR);
    prepare_agent(&mut scenario);
    agent_create_group(&mut scenario, option::none(), option::none(), false, false);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = msg::EUnknownRoomKind)]
fun unmapped_room_kind_aborts() {
    let mut scenario = ts::begin(AUTHOR);
    prepare_agent(&mut scenario);
    agent_create_group(&mut scenario, option::some(2), option::none(), false, false);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = msg::EUnknownRoomKind)]
fun task_binding_without_room_kind_aborts() {
    let mut scenario = ts::begin(AUTHOR);
    prepare_agent(&mut scenario);
    agent_create_group(
        &mut scenario,
        option::none(),
        option::some(object::id_from_address(@0x7A5C)),
        false,
        false,
    );
    scenario.end();
}

#[test]
#[expected_failure(abort_code = social_contracts::memory::ESubAgentMissingCap)]
fun agent_without_message_cap_cannot_create_group() {
    let mut scenario = ts::begin(AUTHOR);
    setup(&mut scenario);

    ts::next_tx(&mut scenario, AUTHOR);
    {
        memory_test_helpers::create_default_org_in_tx(&mut scenario);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let platform_obj = ts::take_shared<Platform>(&scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let mut org = memory_test_helpers::take_created_org(&mut scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        memory::register_sub_agent(
            &memory_config,
            &mut memory_account,
            &mut org,
            AGENT_PUBKEY,
            AGENT_ADDR,
            string::utf8(b"no-cap-agent"),
            memory::class_delegated_ai(),
            0,
            memory::cap_memory_read(),
            memory::cap_memory_read(),
            3,
            0,
            option::none(),
            option::some(platform_id),
            option::none(),
            &clock,
            ts::ctx(&mut scenario),
        );
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
        ts::return_shared(platform_obj);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let mut namespace = ts::take_shared<MessagingNamespace>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let group_leaver = ts::take_shared<GroupLeaver>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);

        let members = vec_set::empty<address>();
        let (_group, _eh, _ml) = msg::create_agent_group(
            &version,
            &mut namespace,
            &group_manager,
            &group_leaver,
            &block_list,
            &platform,
            &memory_config,
            &memory_account,
            &memory_account,
            string::utf8(TEST_GROUP_NAME),
            string::utf8(TEST_UUID),
            TEST_ENCRYPTED_DEK,
            members,
            option::none(),
            option::none(),
            &clock,
            ts::ctx(&mut scenario),
        );

        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(memory_account);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_leaver);
        ts::return_shared(group_manager);
        ts::return_shared(namespace);
        ts::return_shared(version);
        destroy(_group);
        destroy(_eh);
        destroy(_ml);
    };

    scenario.end();
}

#[test]
fun principal_mydata_oversight_fallback() {
    let mut scenario = ts::begin(AUTHOR);
    setup(&mut scenario);

    ts::next_tx(&mut scenario, AUTHOR);
    {
        memory_test_helpers::create_default_org_in_tx(&mut scenario);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let platform_obj = ts::take_shared<Platform>(&scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let mut org = memory_test_helpers::take_created_org(&mut scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        register_agent(&memory_config, &mut memory_account, &mut org, platform_id, &clock, ts::ctx(&mut scenario));
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
        ts::return_shared(platform_obj);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let mut namespace = ts::take_shared<MessagingNamespace>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let group_leaver = ts::take_shared<GroupLeaver>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);

        msg::create_agent_and_share_group(
            &version,
            &mut namespace,
            &group_manager,
            &group_leaver,
            &block_list,
            &platform,
            &memory_config,
            &memory_account,
            &memory_account,
            string::utf8(TEST_GROUP_NAME),
            string::utf8(TEST_UUID),
            TEST_ENCRYPTED_DEK,
            vector[],
            option::none(),
            option::none(),
            &clock,
            ts::ctx(&mut scenario),
        );

        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(memory_account);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_leaver);
        ts::return_shared(group_manager);
        ts::return_shared(namespace);
        ts::return_shared(version);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let group = ts::take_shared<PermissionedGroup<Messaging>>(&scenario);
        let encryption_history = ts::take_shared<EncryptionHistory>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);

        let mut bytes = object::id_to_address(&object::id(&group)).to_bytes();
        bytes.append(myso::bcs::to_bytes(&0u64));
        mydata_policies::mydata_approve_reader_with_oversight(
            bytes,
            &version,
            &group,
            &encryption_history,
            &memory_account,
            AGENT_ADDR,
            ts::ctx(&mut scenario),
        );

        ts::return_shared(memory_account);
        ts::return_shared(encryption_history);
        ts::return_shared(group);
        ts::return_shared(version);
    };

    scenario.end();
}

#[test]
#[expected_failure(abort_code = messaging::messaging::ENotPermitted)]
fun agent_cannot_bypass_via_human_create_group() {
    let mut scenario = ts::begin(AUTHOR);
    setup(&mut scenario);

    ts::next_tx(&mut scenario, AUTHOR);
    {
        memory_test_helpers::create_default_org_in_tx(&mut scenario);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let platform_obj = ts::take_shared<Platform>(&scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let mut org = memory_test_helpers::take_created_org(&mut scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        register_agent(&memory_config, &mut memory_account, &mut org, platform_id, &clock, ts::ctx(&mut scenario));
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
        ts::return_shared(platform_obj);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let mut namespace = ts::take_shared<MessagingNamespace>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);

        let (_group, _eh, _ml) = msg::create_group(
            &version,
            &mut namespace,
            &group_manager,
            &block_list,
            &memory_account,
            string::utf8(TEST_GROUP_NAME),
            string::utf8(TEST_UUID),
            TEST_ENCRYPTED_DEK,
            vec_set::empty(),
            ts::ctx(&mut scenario),
        );

        ts::return_shared(memory_account);
        ts::return_shared(block_list);
        ts::return_shared(group_manager);
        ts::return_shared(namespace);
        ts::return_shared(version);
        destroy(_group);
        destroy(_eh);
        destroy(_ml);
    };

    scenario.end();
}

#[test]
fun cross_principal_agent_peer_gets_permissions_and_oversight() {
    let mut scenario = ts::begin(AUTHOR);
    setup(&mut scenario);

    ts::next_tx(&mut scenario, AUTHOR);
    {
        memory_test_helpers::create_default_org_in_tx(&mut scenario);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let platform_obj = ts::take_shared<Platform>(&scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let mut org = memory_test_helpers::take_created_org(&mut scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        register_agent(&memory_config, &mut memory_account, &mut org, platform_id, &clock, ts::ctx(&mut scenario));
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
        ts::return_shared(platform_obj);
    };

    setup_carol_with_agent(&mut scenario);

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let mut namespace = ts::take_shared<MessagingNamespace>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let group_leaver = ts::take_shared<GroupLeaver>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_one = ts::take_shared<MemoryAccount>(&scenario);
        let memory_two = ts::take_shared<MemoryAccount>(&scenario);
        let (author_memory, carol_memory) = if (memory::owner(&memory_one) == AUTHOR) {
            (memory_one, memory_two)
        } else {
            (memory_two, memory_one)
        };
        let clock = ts::take_shared<Clock>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);

        let mut members = vec_set::empty<address>();
        vec_set::insert(&mut members, CAROL_AGENT_ADDR);

        let (group, encryption_history, msg_log) = msg::create_agent_group(
            &version,
            &mut namespace,
            &group_manager,
            &group_leaver,
            &block_list,
            &platform,
            &memory_config,
            &author_memory,
            &carol_memory,
            string::utf8(TEST_GROUP_NAME),
            string::utf8(TEST_UUID_CROSS),
            TEST_ENCRYPTED_DEK,
            members,
            option::none(),
            option::none(),
            &clock,
            ts::ctx(&mut scenario),
        );

        assert!(group.has_permission<Messaging, MessagingSender>(CAROL_AGENT_ADDR), 0);
        assert!(group.has_permission<Messaging, MessagingReader>(CAROL_AGENT_ADDR), 1);
        assert!(group.has_permission<Messaging, PermissionsAdmin>(CAROL), 2);
        assert!(group.has_permission<Messaging, MessagingReader>(CAROL), 3);

        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(carol_memory);
        ts::return_shared(author_memory);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_leaver);
        ts::return_shared(group_manager);
        ts::return_shared(namespace);
        ts::return_shared(version);
        destroy(group);
        destroy(encryption_history);
        destroy(msg_log);
    };

    scenario.end();
}

#[test]
fun agent_paid_dm_stranger_at_min_succeeds() {
    let mut scenario = ts::begin(AUTHOR);
    setup(&mut scenario);

    ts::next_tx(&mut scenario, AUTHOR);
    {
        memory_test_helpers::create_default_org_in_tx(&mut scenario);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let platform_obj = ts::take_shared<Platform>(&scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let mut org = memory_test_helpers::take_created_org(&mut scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        register_agent(&memory_config, &mut memory_account, &mut org, platform_id, &clock, ts::ctx(&mut scenario));
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
        ts::return_shared(platform_obj);
    };

    ts::next_tx(&mut scenario, BOB);
    {
        let mut paid_registry = ts::take_shared<PaidMessagingRegistry>(&scenario);
        paid_messaging_policy::set_paid_messaging_policy(
            &mut paid_registry,
            true,
            option::some(1000),
            ts::ctx(&mut scenario),
        );
        ts::return_shared(paid_registry);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let mut namespace = ts::take_shared<MessagingNamespace>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let group_leaver = ts::take_shared<GroupLeaver>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);

        let mut members = vec_set::empty<address>();
        vec_set::insert(&mut members, BOB);

        let (group, encryption_history, msg_log) = msg::create_agent_group(
            &version,
            &mut namespace,
            &group_manager,
            &group_leaver,
            &block_list,
            &platform,
            &memory_config,
            &memory_account,
            &memory_account,
            string::utf8(TEST_GROUP_NAME),
            string::utf8(TEST_UUID_PAID),
            TEST_ENCRYPTED_DEK,
            members,
            option::none(),
            option::none(),
            &clock,
            ts::ctx(&mut scenario),
        );
        let _gid = object::id(&group);
        let _msg_log_id = object::id(&msg_log);
        transfer::public_share_object(group);
        transfer::public_share_object(encryption_history);
        transfer::public_share_object(msg_log);

        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(memory_account);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_leaver);
        ts::return_shared(group_manager);
        ts::return_shared(namespace);
        ts::return_shared(version);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let group = ts::take_shared<PermissionedGroup<Messaging>>(&scenario);
        let mut msg_log = ts::take_shared<MessageLog>(&scenario);
        let paid_registry = ts::take_shared<PaidMessagingRegistry>(&scenario);
        let social_graph = ts::take_shared<SocialGraph>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let messaging_config = ts::take_shared<MessagingConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let payment = coin::mint_for_testing<MYSO>(1000, ts::ctx(&mut scenario));

        msg::send_agent_paid_message_digest(
            &version,
            &messaging_config,
            &group,
            &mut msg_log,
            &paid_registry,
            &social_graph,
            &block_list,
            &group_manager,
            &platform,
            &memory_config,
            &memory_account,
            BOB,
            payment,
            1000,
            b"dedupe",
            1u128,
            &clock,
            ts::ctx(&mut scenario),
        );

        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(messaging_config);
        ts::return_shared(memory_account);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_manager);
        ts::return_shared(social_graph);
        ts::return_shared(paid_registry);
        ts::return_shared(msg_log);
        ts::return_shared(group);
        ts::return_shared(version);
    };

    scenario.end();
}

#[test]
#[expected_failure(abort_code = msg::EBelowMinMessageCost)]
fun agent_paid_dm_below_min_fails() {
    let mut scenario = ts::begin(AUTHOR);
    setup(&mut scenario);

    ts::next_tx(&mut scenario, AUTHOR);
    {
        memory_test_helpers::create_default_org_in_tx(&mut scenario);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let platform_obj = ts::take_shared<Platform>(&scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let mut org = memory_test_helpers::take_created_org(&mut scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        register_agent(&memory_config, &mut memory_account, &mut org, platform_id, &clock, ts::ctx(&mut scenario));
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
        ts::return_shared(platform_obj);
    };

    ts::next_tx(&mut scenario, BOB);
    {
        let mut paid_registry = ts::take_shared<PaidMessagingRegistry>(&scenario);
        paid_messaging_policy::set_paid_messaging_policy(
            &mut paid_registry,
            true,
            option::some(1000),
            ts::ctx(&mut scenario),
        );
        ts::return_shared(paid_registry);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let mut namespace = ts::take_shared<MessagingNamespace>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let group_leaver = ts::take_shared<GroupLeaver>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);

        let mut members = vec_set::empty<address>();
        vec_set::insert(&mut members, BOB);

        msg::create_agent_and_share_group(
            &version,
            &mut namespace,
            &group_manager,
            &group_leaver,
            &block_list,
            &platform,
            &memory_config,
            &memory_account,
            &memory_account,
            string::utf8(TEST_GROUP_NAME),
            string::utf8(TEST_UUID_PAID),
            TEST_ENCRYPTED_DEK,
            vector[BOB],
            option::none(),
            option::none(),
            &clock,
            ts::ctx(&mut scenario),
        );

        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(memory_account);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_leaver);
        ts::return_shared(group_manager);
        ts::return_shared(namespace);
        ts::return_shared(version);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let group = ts::take_shared<PermissionedGroup<Messaging>>(&scenario);
        let mut msg_log = ts::take_shared<MessageLog>(&scenario);
        let paid_registry = ts::take_shared<PaidMessagingRegistry>(&scenario);
        let social_graph = ts::take_shared<SocialGraph>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let messaging_config = ts::take_shared<MessagingConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let payment = coin::mint_for_testing<MYSO>(500, ts::ctx(&mut scenario));

        msg::send_agent_paid_message_digest(
            &version,
            &messaging_config,
            &group,
            &mut msg_log,
            &paid_registry,
            &social_graph,
            &block_list,
            &group_manager,
            &platform,
            &memory_config,
            &memory_account,
            BOB,
            payment,
            500,
            b"dedupe",
            1u128,
            &clock,
            ts::ctx(&mut scenario),
        );

        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(messaging_config);
        ts::return_shared(memory_account);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_manager);
        ts::return_shared(social_graph);
        ts::return_shared(paid_registry);
        ts::return_shared(msg_log);
        ts::return_shared(group);
        ts::return_shared(version);
    };

    scenario.end();
}

#[test]
#[expected_failure(abort_code = msg::EPaidNotRequiredForFollower)]
fun agent_paid_dm_principal_follower_bypass_fails() {
    let mut scenario = ts::begin(AUTHOR);
    setup(&mut scenario);

    ts::next_tx(&mut scenario, AUTHOR);
    {
        memory_test_helpers::create_default_org_in_tx(&mut scenario);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let platform_obj = ts::take_shared<Platform>(&scenario);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let mut org = memory_test_helpers::take_created_org(&mut scenario);
        let mut memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        register_agent(&memory_config, &mut memory_account, &mut org, platform_id, &clock, ts::ctx(&mut scenario));
        ts::return_shared(memory_config);
        ts::return_shared(clock);
        ts::return_shared(memory_account);
        ts::return_shared(org);
        ts::return_shared(platform_obj);
    };

    ts::next_tx(&mut scenario, BOB);
    {
        let mut paid_registry = ts::take_shared<PaidMessagingRegistry>(&scenario);
        paid_messaging_policy::set_paid_messaging_policy(
            &mut paid_registry,
            true,
            option::some(1000),
            ts::ctx(&mut scenario),
        );
        ts::return_shared(paid_registry);
    };

    ts::next_tx(&mut scenario, AUTHOR);
    {
        let mut social_graph = ts::take_shared<SocialGraph>(&scenario);
        social_graph::follow(&mut social_graph, BOB, ts::ctx(&mut scenario));
        ts::return_shared(social_graph);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let mut namespace = ts::take_shared<MessagingNamespace>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let group_leaver = ts::take_shared<GroupLeaver>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);

        msg::create_agent_and_share_group(
            &version,
            &mut namespace,
            &group_manager,
            &group_leaver,
            &block_list,
            &platform,
            &memory_config,
            &memory_account,
            &memory_account,
            string::utf8(TEST_GROUP_NAME),
            string::utf8(TEST_UUID_FOLLOWER),
            TEST_ENCRYPTED_DEK,
            vector[BOB],
            option::none(),
            option::none(),
            &clock,
            ts::ctx(&mut scenario),
        );

        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(memory_account);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_leaver);
        ts::return_shared(group_manager);
        ts::return_shared(namespace);
        ts::return_shared(version);
    };

    ts::next_tx(&mut scenario, AGENT_ADDR);
    {
        let version = ts::take_shared<Version>(&scenario);
        let group = ts::take_shared<PermissionedGroup<Messaging>>(&scenario);
        let mut msg_log = ts::take_shared<MessageLog>(&scenario);
        let paid_registry = ts::take_shared<PaidMessagingRegistry>(&scenario);
        let social_graph = ts::take_shared<SocialGraph>(&scenario);
        let group_manager = ts::take_shared<GroupManager>(&scenario);
        let block_list = ts::take_shared<BlockListRegistry>(&scenario);
        let platform = ts::take_shared<Platform>(&scenario);
        let memory_account = ts::take_shared<MemoryAccount>(&scenario);
        let memory_config = ts::take_shared<MemoryConfig>(&scenario);
        let messaging_config = ts::take_shared<MessagingConfig>(&scenario);
        let clock = ts::take_shared<Clock>(&scenario);
        let payment = coin::mint_for_testing<MYSO>(1000, ts::ctx(&mut scenario));

        msg::send_agent_paid_message_digest(
            &version,
            &messaging_config,
            &group,
            &mut msg_log,
            &paid_registry,
            &social_graph,
            &block_list,
            &group_manager,
            &platform,
            &memory_config,
            &memory_account,
            BOB,
            payment,
            1000,
            b"dedupe",
            1u128,
            &clock,
            ts::ctx(&mut scenario),
        );

        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(messaging_config);
        ts::return_shared(memory_account);
        ts::return_shared(platform);
        ts::return_shared(block_list);
        ts::return_shared(group_manager);
        ts::return_shared(social_graph);
        ts::return_shared(paid_registry);
        ts::return_shared(msg_log);
        ts::return_shared(group);
        ts::return_shared(version);
    };

    scenario.end();
}
