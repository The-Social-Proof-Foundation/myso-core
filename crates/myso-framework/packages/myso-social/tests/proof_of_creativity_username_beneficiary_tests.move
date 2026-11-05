// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, duplicate_alias)]
module social_contracts::proof_of_creativity_username_beneficiary_tests {
    use std::string;
    use std::option;
    use std::vector;

    use social_contracts::proof_of_creativity as poc;
    use social_contracts::governance;
    use social_contracts::profile::{Self, EcosystemTreasury, UsernameRegistry,
        ProfileConfig, Profile};
    use social_contracts::ai_credit::AiCreditConfig;
    use social_contracts::poc_username_beneficiary::{
        Self as ub,
        PoCBeneficiaryAdminCap,
        PoCUsernameBeneficiary,
        PoCUsernameBeneficiaryDirectory,
        PoCUsernameBeneficiaryShard,
    };
    use social_contracts::poc_vault::{Self, PoCBeneficiaryVault, PoCVaultDirectory};
    use social_contracts::memory;

    use myso::test_scenario::{Self, Scenario};
    use myso::clock::{Self, Clock};

    const ADMIN: address = @0xA0;
    const CREATOR: address = @0xC1;
    const RECIPIENT: address = @0xC2;
    const REFERRER: address = @0xF1;
    const USERNAME: vector<u8> = b"offplatform";
    const X_HANDLE: vector<u8> = b"offplatform";
    const IDENTITY_HASH: vector<u8> = b"x-user-123";

    fun setup(scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));
            let ctx = test_scenario::ctx(scenario);
            let gov_ids = governance::bootstrap_init(&clock, ctx);
            poc::test_init(&clock, gov_ids.poc_governance_registry_id(), ctx);
            clock::share_for_testing(clock);
        };
    }

    fun provision_beneficiary(scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            let cap = test_scenario::take_from_sender<PoCBeneficiaryAdminCap>(scenario);
            let mut directory = test_scenario::take_shared<PoCUsernameBeneficiaryDirectory>(scenario);
            let shard_id = ub::beneficiary_shard_object_id(&directory, USERNAME);
            let mut shard = test_scenario::take_shared_by_id<PoCUsernameBeneficiaryShard>(scenario, shard_id);
            let mut vault_dir = test_scenario::take_shared<PoCVaultDirectory>(scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);

            poc::create_username_beneficiary(
                &cap,
                &mut directory,
                &mut shard,
                &mut vault_dir,
                &mut registry,
                USERNAME,
                1,
                IDENTITY_HASH,
                X_HANDLE,
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(scenario, cap);
            test_scenario::return_shared(directory);
            test_scenario::return_shared(shard);
            test_scenario::return_shared(vault_dir);
            test_scenario::return_shared(registry);
        };
    }

    #[test]
    fun test_verified_identity_transfer_requires_recipient_acceptance_and_binds_claim() {
        let mut scen = test_scenario::begin(ADMIN);
        setup(&mut scen);
        provision_beneficiary(&mut scen);

        // The configured PoC oracle proposes for the provisioned, verified identity.
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(&scen);
            poc::propose_username_beneficiary_transfer(
                &cfg,
                &mut beneficiary,
                RECIPIENT,
                b"0123456789abcdef0123456789abcdef",
                X_HANDLE,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(beneficiary);
        };

        // The proposed recipient accepts with their own transaction signer.
        test_scenario::next_tx(&mut scen, RECIPIENT);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(&scen);
            poc::accept_username_beneficiary_transfer(
                &mut beneficiary,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(beneficiary);
        };

        // The oracle can now create the profile only for the wallet that accepted.
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let cfg = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scen);
            let mut directory = test_scenario::take_shared<PoCUsernameBeneficiaryDirectory>(&scen);
            let shard_id = ub::beneficiary_shard_object_id(&directory, USERNAME);
            let mut shard = test_scenario::take_shared_by_id<PoCUsernameBeneficiaryShard>(&scen, shard_id);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scen);
            let mut memory_registry = test_scenario::take_shared<memory::MemoryRegistry>(&scen);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scen);
            let mut beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(&scen);

            poc::claim_username_beneficiary(
                &cfg, &profile_config, &mut directory, &mut shard, &mut registry,
                &mut memory_registry, &mut ai_credit_config, &mut beneficiary,
                vector::empty(), X_HANDLE, b"Creator", b"bio", vector::empty(),
                vector::empty(), RECIPIENT, &clock, test_scenario::ctx(&mut scen),
            );
            assert!(ub::beneficiary_status(&beneficiary) == 2);
            assert!(ub::linked_wallet(&beneficiary) == option::some(RECIPIENT));

            test_scenario::return_shared(clock);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(profile_config);
            test_scenario::return_shared(directory);
            test_scenario::return_shared(shard);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(beneficiary);
        };

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = profile::EUsernameLocked, location = social_contracts::profile)]
    fun test_username_beneficiary_provision_blocks_create_profile() {
        let mut scen = test_scenario::begin(ADMIN);
        setup(&mut scen);
        provision_beneficiary(&mut scen);

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scen);
            let mut memory_registry = test_scenario::take_shared<memory::MemoryRegistry>(&scen);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scen);

            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"Blocked"),
                string::utf8(USERNAME),
                string::utf8(b""),
                vector::empty(),
                vector::empty(),
                &clock,
                test_scenario::ctx(&mut scen),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_username_beneficiary_claim_and_join_referral_vault_claim() {
        let mut scen = test_scenario::begin(ADMIN);
        setup(&mut scen);
        provision_beneficiary(&mut scen);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let cfg = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scen);
            let mut directory = test_scenario::take_shared<PoCUsernameBeneficiaryDirectory>(&scen);
            let shard_id = ub::beneficiary_shard_object_id(&directory, USERNAME);
            let mut shard = test_scenario::take_shared_by_id<PoCUsernameBeneficiaryShard>(&scen, shard_id);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scen);
            let mut memory_registry = test_scenario::take_shared<memory::MemoryRegistry>(&scen);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scen);
            let mut beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(&scen);

            poc::claim_username_beneficiary(
                &cfg,
                &profile_config,
                &mut directory,
                &mut shard,
                &mut registry,
                &mut memory_registry,
                &mut ai_credit_config,
                &mut beneficiary,
                vector::empty(),
                X_HANDLE,
                b"Creator",
                b"bio",
                vector::empty(),
                vector::empty(),
                CREATOR,
                &clock,
                test_scenario::ctx(&mut scen),
            );

            assert!(ub::beneficiary_status(&beneficiary) == 2);

            test_scenario::return_shared(clock);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(profile_config);
            test_scenario::return_shared(directory);
            test_scenario::return_shared(shard);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(beneficiary);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let cfg = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let directory = test_scenario::take_shared<PoCUsernameBeneficiaryDirectory>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let mut beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(&scen);
            let mut vault = test_scenario::take_shared<PoCBeneficiaryVault>(&scen);

            poc_vault::test_deposit_mys(&mut vault, 10_000_000_000, &clock, test_scenario::ctx(&mut scen));

            poc::claim_username_beneficiary_vault_balance<myso::myso::MYSO>(
                &cfg,
                &directory,
                &mut beneficiary,
                &treasury,
                &mut vault,
                option::some(REFERRER),
                &clock,
                test_scenario::ctx(&mut scen),
            );

            assert!(ub::join_referral_paid(&beneficiary));

            test_scenario::return_shared(clock);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(directory);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(beneficiary);
            test_scenario::return_shared(vault);
        };

        test_scenario::end(scen);
    }
    const EXISTING_USERNAME: vector<u8> = b"mainname";

    fun create_profile_for(scenario: &mut Scenario, owner: address, username: vector<u8>) {
        test_scenario::next_tx(scenario, owner);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let mut memory_registry = test_scenario::take_shared<memory::MemoryRegistry>(scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"Existing"),
                string::utf8(username),
                string::utf8(b""),
                vector::empty(),
                vector::empty(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(profile_config);
        };
    }

    fun oracle_claim_for(scenario: &mut Scenario, wallet: address) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            let cfg = test_scenario::take_shared<poc::PoCConfig>(scenario);
            let profile_config = test_scenario::take_shared<ProfileConfig>(scenario);
            let mut directory = test_scenario::take_shared<PoCUsernameBeneficiaryDirectory>(scenario);
            let shard_id = ub::beneficiary_shard_object_id(&directory, USERNAME);
            let mut shard = test_scenario::take_shared_by_id<PoCUsernameBeneficiaryShard>(scenario, shard_id);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let mut memory_registry = test_scenario::take_shared<memory::MemoryRegistry>(scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(scenario);
            let mut beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(scenario);
            poc::claim_username_beneficiary(
                &cfg, &profile_config, &mut directory, &mut shard, &mut registry,
                &mut memory_registry, &mut ai_credit_config, &mut beneficiary,
                vector::empty(), X_HANDLE, b"Creator", b"bio", vector::empty(),
                vector::empty(), wallet, &clock, test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(profile_config);
            test_scenario::return_shared(directory);
            test_scenario::return_shared(shard);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(beneficiary);
        };
    }

    fun adopt_or_release(scenario: &mut Scenario, sender: address, adopt: bool) {
        test_scenario::next_tx(scenario, sender);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            let directory = test_scenario::take_shared<PoCUsernameBeneficiaryDirectory>(scenario);
            let shard_id = ub::beneficiary_shard_object_id(&directory, USERNAME);
            let mut shard = test_scenario::take_shared_by_id<PoCUsernameBeneficiaryShard>(scenario, shard_id);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let mut beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(scenario);
            if (adopt) {
                poc::adopt_reserved_username(
                    &mut shard, &mut registry, &mut beneficiary, &clock, test_scenario::ctx(scenario),
                );
            } else {
                poc::release_reserved_username(
                    &mut shard, &mut registry, &mut beneficiary, &clock, test_scenario::ctx(scenario),
                );
            };
            test_scenario::return_shared(clock);
            test_scenario::return_shared(directory);
            test_scenario::return_shared(shard);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(beneficiary);
        };
    }

    fun claim_vault_as(scenario: &mut Scenario, wallet: address) {
        test_scenario::next_tx(scenario, wallet);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            let cfg = test_scenario::take_shared<poc::PoCConfig>(scenario);
            let directory = test_scenario::take_shared<PoCUsernameBeneficiaryDirectory>(scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scenario);
            let mut beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(scenario);
            let mut vault = test_scenario::take_shared<PoCBeneficiaryVault>(scenario);
            poc_vault::test_deposit_mys(&mut vault, 10_000_000_000, &clock, test_scenario::ctx(scenario));
            poc::claim_username_beneficiary_vault_balance<myso::myso::MYSO>(
                &cfg, &directory, &mut beneficiary, &treasury, &mut vault,
                option::none(), &clock, test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(directory);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(beneficiary);
            test_scenario::return_shared(vault);
        };
    }

    #[test]
    fun test_claim_without_profile_creates_owned_profile() {
        let mut scen = test_scenario::begin(ADMIN);
        setup(&mut scen);
        provision_beneficiary(&mut scen);
        oracle_claim_for(&mut scen, CREATOR);

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            // Owned by the creator wallet (not shared).
            let p = test_scenario::take_from_sender<Profile>(&scen);
            let beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(&scen);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scen);
            let pid = myso::object::uid_to_address(profile::id(&p));
            assert!(ub::linked_wallet(&beneficiary) == option::some(CREATOR));
            assert!(option::is_none(&ub::reserved_for(&beneficiary)));
            assert!(ub::claimed_profile_id(&beneficiary) == option::some(pid));
            assert!(profile::lookup_profile_by_username(&registry, string::utf8(USERNAME)) == option::some(pid));
            assert!(!profile::is_username_locked(&registry, string::utf8(USERNAME)));
            test_scenario::return_to_sender(&scen, p);
            test_scenario::return_shared(beneficiary);
            test_scenario::return_shared(registry);
        };
        test_scenario::end(scen);
    }

    #[test]
    fun test_claim_with_existing_profile_reserves_username_and_vault_works() {
        let mut scen = test_scenario::begin(ADMIN);
        setup(&mut scen);
        provision_beneficiary(&mut scen);
        create_profile_for(&mut scen, CREATOR, EXISTING_USERNAME);
        oracle_claim_for(&mut scen, CREATOR);

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(&scen);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scen);
            let pid = *option::borrow(&profile::lookup_profile_by_owner(&registry, CREATOR));
            assert!(ub::beneficiary_status(&beneficiary) == 2);
            assert!(ub::linked_wallet(&beneficiary) == option::some(CREATOR));
            assert!(ub::reserved_for(&beneficiary) == option::some(CREATOR));
            assert!(option::is_none(&ub::claimed_profile_id(&beneficiary)));
            assert!(profile::is_username_locked(&registry, string::utf8(USERNAME)));
            assert!(profile::lookup_profile_by_username(&registry, string::utf8(EXISTING_USERNAME)) == option::some(pid));
            test_scenario::return_shared(beneficiary);
            test_scenario::return_shared(registry);
        };
        claim_vault_as(&mut scen, CREATOR);
        test_scenario::end(scen);
    }

    #[test]
    fun test_adopt_reserved_username_renames_profile_and_keeps_vault_access() {
        let mut scen = test_scenario::begin(ADMIN);
        setup(&mut scen);
        provision_beneficiary(&mut scen);
        create_profile_for(&mut scen, CREATOR, EXISTING_USERNAME);
        oracle_claim_for(&mut scen, CREATOR);
        adopt_or_release(&mut scen, CREATOR, true);

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(&scen);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scen);
            let pid = *option::borrow(&profile::lookup_profile_by_owner(&registry, CREATOR));
            assert!(profile::lookup_profile_by_username(&registry, string::utf8(USERNAME)) == option::some(pid));
            assert!(profile::is_username_available(&registry, string::utf8(EXISTING_USERNAME)));
            assert!(!profile::is_username_locked(&registry, string::utf8(USERNAME)));
            assert!(option::is_none(&ub::reserved_for(&beneficiary)));
            assert!(ub::claimed_profile_id(&beneficiary) == option::some(pid));
            assert!(ub::linked_wallet(&beneficiary) == option::some(CREATOR));
            test_scenario::return_shared(beneficiary);
            test_scenario::return_shared(registry);
        };
        claim_vault_as(&mut scen, CREATOR);
        test_scenario::end(scen);
    }

    #[test]
    fun test_release_reserved_username_frees_it_and_keeps_vault_access() {
        let mut scen = test_scenario::begin(ADMIN);
        setup(&mut scen);
        provision_beneficiary(&mut scen);
        create_profile_for(&mut scen, CREATOR, EXISTING_USERNAME);
        oracle_claim_for(&mut scen, CREATOR);
        adopt_or_release(&mut scen, CREATOR, false);

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let beneficiary = test_scenario::take_shared<PoCUsernameBeneficiary>(&scen);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scen);
            assert!(!profile::is_username_locked(&registry, string::utf8(USERNAME)));
            assert!(profile::is_username_available(&registry, string::utf8(USERNAME)));
            assert!(option::is_none(&ub::reserved_for(&beneficiary)));
            assert!(ub::linked_wallet(&beneficiary) == option::some(CREATOR));
            test_scenario::return_shared(beneficiary);
            test_scenario::return_shared(registry);
        };
        claim_vault_as(&mut scen, CREATOR);
        // Someone else can now take the released name.
        create_profile_for(&mut scen, RECIPIENT, USERNAME);
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = ub::ENotReservedForSender, location = social_contracts::poc_username_beneficiary)]
    fun test_adopt_by_wrong_wallet_aborts() {
        let mut scen = test_scenario::begin(ADMIN);
        setup(&mut scen);
        provision_beneficiary(&mut scen);
        create_profile_for(&mut scen, CREATOR, EXISTING_USERNAME);
        create_profile_for(&mut scen, RECIPIENT, b"otheruser");
        oracle_claim_for(&mut scen, CREATOR);
        adopt_or_release(&mut scen, RECIPIENT, true);
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = ub::ENotReservedForSender, location = social_contracts::poc_username_beneficiary)]
    fun test_adopt_after_profile_created_by_claim_aborts() {
        let mut scen = test_scenario::begin(ADMIN);
        setup(&mut scen);
        provision_beneficiary(&mut scen);
        oracle_claim_for(&mut scen, CREATOR);
        adopt_or_release(&mut scen, CREATOR, true);
        test_scenario::end(scen);
    }
}
