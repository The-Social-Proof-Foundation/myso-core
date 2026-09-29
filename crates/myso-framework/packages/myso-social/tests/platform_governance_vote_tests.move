// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use)]
module social_contracts::platform_governance_vote_tests {
    use std::string;
    use std::option;

    use myso::test_scenario::{Self, Scenario};
    use myso::object::{Self, ID};
    use myso::clock::{Self, Clock};
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::permissioned_group::PermissionedGroup;

    use mydata::bf_hmac_encryption::{Self, EncryptedObject};

    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::profile::{Self, UsernameRegistry};
    use social_contracts::social_graph::{Self, SocialGraph};
    use social_contracts::governance::{Self, GovernanceDAO, Proposal};
    use social_contracts::platform::{Self, Platform, PlatformRegistry, PlatformPackage, PlatformConfig};

    const ADMIN: address = @0xAD;
    const PLATFORM_ADMIN: address = @0xF1;
    const OTHER_PLATFORM_ADMIN: address = @0xF2;
    const MEMBER: address = @0xB1;
    const OUTSIDER: address = @0xB2;

    const SUBMISSION_COST: u64 = 50_000_000;

    fun init_shared(scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            platform::test_init(&clock, test_scenario::ctx(scenario));
            block_list::test_init(&clock, test_scenario::ctx(scenario));
            social_graph::init_for_testing(&clock, test_scenario::ctx(scenario));
            profile::test_init(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
        };
    }

    fun give_profile(scenario: &mut Scenario, wallet: address, username: vector<u8>) {
        test_scenario::next_tx(scenario, wallet);
        {
            let mut username_registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            profile::register_username(
                &mut username_registry,
                string::utf8(username),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(username_registry);
        };
    }

    /// Creates an approved platform with DAO governance; `developer` becomes the founding delegate.
    /// Returns (platform id, governance registry id).
    fun create_dao_platform(scenario: &mut Scenario, developer: address, name: vector<u8>): (ID, ID) {
        test_scenario::next_tx(scenario, developer);
        {
            let platform_config = test_scenario::take_shared<PlatformConfig>(scenario);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            platform::create_platform(
                &mut registry,
                &platform_config,
                string::utf8(name),
                string::utf8(b"A DAO platform"),
                string::utf8(b"Platform with DAO governance"),
                string::utf8(b"https://example.com/logo.png"),
                string::utf8(b"https://example.com/terms"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"),
                option::none(),
                2,
                string::utf8(b"2023-01-01"),
                true,
                option::some(7),
                option::some(30),
                option::some(SUBMISSION_COST),
                option::some(5),
                option::some(5_000_000),
                option::some(1_000_000),
                option::some(15),
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(platform_config);
        };

        test_scenario::next_tx(scenario, developer);
        let platform_id = test_scenario::most_recent_id_shared<Platform>().destroy_some();
        let registry_id = test_scenario::most_recent_id_shared<GovernanceDAO>().destroy_some();
        {
            let platform = test_scenario::take_shared_by_id<Platform>(scenario, platform_id);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            platform::test_set_approval(&mut registry, object::id_to_address(&platform_id), true);
            assert!(*option::borrow(platform::governance_registry_id(&platform)) == registry_id, 0);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(platform);
        };
        (platform_id, registry_id)
    }

    /// Submits a platform proposal and has the founding delegate advance it to community voting.
    fun open_community_vote(scenario: &mut Scenario, developer: address, platform_id: ID, registry_id: ID) {
        test_scenario::next_tx(scenario, developer);
        {
            let mut registry = test_scenario::take_shared_by_id<GovernanceDAO>(scenario, registry_id);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let mut payment = coin::mint_for_testing<MYSO>(SUBMISSION_COST, test_scenario::ctx(scenario));
            governance::submit_proposal(
                &mut registry,
                governance::proposal_type_platform_value(),
                string::utf8(b"Platform proposal"),
                string::utf8(b"Change something"),
                option::none(),
                option::none(),
                option::none(),
                &mut payment,
                &clock,
                test_scenario::ctx(scenario),
            );
            coin::burn_for_testing(payment);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(scenario, developer);
        {
            let mut platform = test_scenario::take_shared_by_id<Platform>(scenario, platform_id);
            let mut registry = test_scenario::take_shared_by_id<GovernanceDAO>(scenario, registry_id);
            let mut proposal = test_scenario::take_shared<Proposal>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            platform::delegate_vote_on_platform_governance_proposal(
                &mut platform,
                &mut registry,
                &mut proposal,
                true,
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            assert!(governance::proposal_status(&proposal) == 2, 0);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(platform);
        };
    }

    fun join(scenario: &mut Scenario, wallet: address, platform_id: ID) {
        test_scenario::next_tx(scenario, wallet);
        {
            let registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let block_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            let mut platform = test_scenario::take_shared_by_id<Platform>(scenario, platform_id);
            let clock = test_scenario::take_shared<Clock>(scenario);
            platform::join_platform(&registry, &block_registry, &mut platform, &clock, test_scenario::ctx(scenario));
            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(block_registry);
            test_scenario::return_shared(registry);
        };
    }

    fun block(scenario: &mut Scenario, developer: address, platform_id: ID, wallet: address) {
        test_scenario::next_tx(scenario, developer);
        {
            let mut block_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            let mut graph = test_scenario::take_shared<SocialGraph>(scenario);
            let mut platform = test_scenario::take_shared_by_id<Platform>(scenario, platform_id);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(scenario);
            social_graph::block_platform_wallet(
                &mut block_registry,
                &mut graph,
                &mut platform,
                &group,
                wallet,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(graph);
            test_scenario::return_shared(block_registry);
        };
    }

    fun platform_vote(scenario: &mut Scenario, voter: address, platform_id: ID, registry_id: ID) {
        test_scenario::next_tx(scenario, voter);
        {
            let platform = test_scenario::take_shared_by_id<Platform>(scenario, platform_id);
            let block_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            let mut registry = test_scenario::take_shared_by_id<GovernanceDAO>(scenario, registry_id);
            let mut proposal = test_scenario::take_shared<Proposal>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let username_registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let mut payment = coin::zero<MYSO>(test_scenario::ctx(scenario));
            platform::community_vote_on_platform_governance_proposal(
                &platform,
                &block_registry,
                &mut registry,
                &username_registry,
                &mut proposal,
                1,
                true,
                &mut payment,
                &clock,
                test_scenario::ctx(scenario),
            );
            coin::destroy_zero(payment);
            test_scenario::return_shared(username_registry);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(block_registry);
            test_scenario::return_shared(platform);
        };
    }

    fun sample_encrypted_vote(): EncryptedObject {
        bf_hmac_encryption::parse_encrypted_object(
            x"00000000000000000000000000000000000000000000000000000000000000000020381dd9078c322a4663c392761a0211b527c127b29583851217f948d62131f40903034401905bebdf8c04f3cd5f04f442a39372c8dc321c29edfb4f9cb30b23ab96b7d726ecf6f7036ee3557cd6c7b93a49b231070e8eecada9cfa157e40e3f02e5d3fcdba72804cc9504a82bbaa13ed4a83a0e2c6219d7e45125cf57fd10cbab957a977b02008812277be43199222d173eed91b480ce4c8cda5aea008ef884e77c990311136486a7daf8e2d99c0389ae40319714ffef1212ffcb456f0de08a7fa1bb185c936f9efe86fb5e32232d5e433230d04b1f2b27614b3b5b13f04db7d5c3b995e7e02e036315d5a9515d050595ea15b326ebcd510baf50463afd6517b5895d0756e39878bd656bd98418df11556d1ced740c7f839d97b81ee60238b3221fb45adfb0a5d1e4aec4f777271e5674bd7ded20421aa929755426501ba8366e465f5ebb861722b2909e5ac2e8608abd885014f2fb6006dd5896ab76ea243dea0d6d6ff4c3396b010de6062eb2dcb2f86bca32f83c9301200000000000000000000000000000000000000000000000000000000000000001184b788b4f5168aff51c0e6da7e2970caa02386c4dc179666ef4c6296807cda9",
        )
    }

    fun platform_vote_anonymous(scenario: &mut Scenario, voter: address, platform_id: ID, registry_id: ID) {
        test_scenario::next_tx(scenario, voter);
        {
            let platform = test_scenario::take_shared_by_id<Platform>(scenario, platform_id);
            let block_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            let mut registry = test_scenario::take_shared_by_id<GovernanceDAO>(scenario, registry_id);
            let mut proposal = test_scenario::take_shared<Proposal>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let username_registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            platform::community_vote_anonymous_on_platform_governance_proposal(
                &platform,
                &block_registry,
                &mut registry,
                &username_registry,
                &mut proposal,
                sample_encrypted_vote(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(username_registry);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(block_registry);
            test_scenario::return_shared(platform);
        };
    }

    fun setup_open_vote(scenario: &mut Scenario): (ID, ID) {
        init_shared(scenario);
        let (platform_id, registry_id) = create_dao_platform(scenario, PLATFORM_ADMIN, b"DAO Platform");
        open_community_vote(scenario, PLATFORM_ADMIN, platform_id, registry_id);
        (platform_id, registry_id)
    }

    #[test]
    fun test_member_can_vote_on_platform_proposal() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        give_profile(&mut scenario, MEMBER, b"platformmember");
        join(&mut scenario, MEMBER, platform_id);
        platform_vote(&mut scenario, MEMBER, platform_id, registry_id);
        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = governance::EProfileRequired)]
    fun test_member_without_profile_cannot_vote_on_platform_proposal() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        join(&mut scenario, MEMBER, platform_id);
        platform_vote(&mut scenario, MEMBER, platform_id, registry_id);
        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = governance::EProfileRequired)]
    fun test_member_without_profile_cannot_vote_anonymously_on_platform_proposal() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        join(&mut scenario, MEMBER, platform_id);
        platform_vote_anonymous(&mut scenario, MEMBER, platform_id, registry_id);
        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = platform::ENotJoined)]
    fun test_non_member_cannot_vote_on_platform_proposal() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        platform_vote(&mut scenario, OUTSIDER, platform_id, registry_id);
        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = platform::EUnauthorized)]
    fun test_blocked_member_cannot_vote_on_platform_proposal() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        join(&mut scenario, MEMBER, platform_id);
        block(&mut scenario, PLATFORM_ADMIN, platform_id, MEMBER);
        platform_vote(&mut scenario, MEMBER, platform_id, registry_id);
        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = platform::EUnauthorized)]
    fun test_platform_must_own_registry() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (_platform_id, registry_id) = setup_open_vote(&mut scenario);
        let (other_platform_id, _other_registry_id) = create_dao_platform(&mut scenario, OTHER_PLATFORM_ADMIN, b"Other DAO Platform");
        join(&mut scenario, MEMBER, other_platform_id);
        platform_vote(&mut scenario, MEMBER, other_platform_id, registry_id);
        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = governance::EUsePlatformVoteEntry)]
    fun test_generic_entry_rejects_platform_registry() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        join(&mut scenario, MEMBER, platform_id);
        test_scenario::next_tx(&mut scenario, MEMBER);
        {
            let mut registry = test_scenario::take_shared_by_id<GovernanceDAO>(&scenario, registry_id);
            let mut proposal = test_scenario::take_shared<Proposal>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let username_registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut payment = coin::zero<MYSO>(test_scenario::ctx(&mut scenario));
            governance::community_vote_on_proposal(
                &mut registry,
                &username_registry,
                &mut proposal,
                1,
                true,
                &mut payment,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            coin::destroy_zero(payment);
            test_scenario::return_shared(username_registry);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(registry);
        };
        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = governance::EUsePlatformVoteEntry)]
    fun test_generic_anonymous_entry_rejects_platform_registry() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        join(&mut scenario, MEMBER, platform_id);
        test_scenario::next_tx(&mut scenario, MEMBER);
        {
            let mut registry = test_scenario::take_shared_by_id<GovernanceDAO>(&scenario, registry_id);
            let mut proposal = test_scenario::take_shared<Proposal>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let username_registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            governance::community_vote_anonymous(
                &mut registry,
                &username_registry,
                &mut proposal,
                sample_encrypted_vote(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(username_registry);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(registry);
        };
        test_scenario::end(scenario);
    }

    #[test]
    fun test_member_can_vote_anonymously_on_platform_proposal() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        give_profile(&mut scenario, MEMBER, b"platformmember");
        join(&mut scenario, MEMBER, platform_id);
        platform_vote_anonymous(&mut scenario, MEMBER, platform_id, registry_id);
        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = platform::ENotJoined)]
    fun test_non_member_cannot_vote_anonymously_on_platform_proposal() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        platform_vote_anonymous(&mut scenario, OUTSIDER, platform_id, registry_id);
        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = platform::EUnauthorized)]
    fun test_blocked_member_cannot_vote_anonymously_on_platform_proposal() {
        let mut scenario = test_scenario::begin(ADMIN);
        let (platform_id, registry_id) = setup_open_vote(&mut scenario);
        join(&mut scenario, MEMBER, platform_id);
        block(&mut scenario, PLATFORM_ADMIN, platform_id, MEMBER);
        platform_vote_anonymous(&mut scenario, MEMBER, platform_id, registry_id);
        test_scenario::end(scenario);
    }
}
