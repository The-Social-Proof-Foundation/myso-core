// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, duplicate_alias)]
module social_contracts::proof_of_creativity_username_beneficiary_tests {
    use std::string;
    use std::option;
    use std::vector;

    use social_contracts::proof_of_creativity as poc;
    use social_contracts::profile::{Self, EcosystemTreasury, UsernameRegistry,
        ProfileConfig};
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
    const REFERRER: address = @0xF1;
    const USERNAME: vector<u8> = b"offplatform";
    const X_HANDLE: vector<u8> = b"offplatform";
    const IDENTITY_HASH: vector<u8> = b"x-user-123";

    fun setup(scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));
            poc::test_init(&clock, test_scenario::ctx(scenario));
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
    #[expected_failure(abort_code = profile::EUsernameBeneficiaryActive, location = social_contracts::profile)]
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
}
