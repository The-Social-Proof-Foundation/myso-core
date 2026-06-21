// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias)]
module social_contracts::memory_test_helpers {
    use std::string;
    use std::option;

    use myso::test_scenario;
    use myso::clock::Clock;

    use social_contracts::memory::{Self, MemoryAccount, AgenticOrganization};
    use social_contracts::profile::{Self, Profile};

    /// Resolve the current transaction sender's linked [`MemoryAccount`].
    public fun take_owner_memory_account(
        scenario: &mut test_scenario::Scenario,
    ): MemoryAccount {
        let profile = test_scenario::take_from_sender<Profile>(scenario);
        let mem_id = *option::borrow(profile::linked_memory_account_id(&profile));
        test_scenario::return_to_sender(scenario, profile);
        test_scenario::take_shared_by_id<MemoryAccount>(scenario, mem_id)
    }

    /// Create and share an org in the current transaction. Call from a dedicated
    /// `next_tx` block; take the org in a follow-up transaction via
    /// [`take_created_org`].
    public fun create_org_in_tx(
        scenario: &mut test_scenario::Scenario,
        org_type: u8,
    ) {
        let mut memory_account = take_owner_memory_account(scenario);
        let clock = test_scenario::take_shared<Clock>(scenario);
        memory::test_force_account_version(&mut memory_account, 4);
        memory::test_create_agentic_organization(
            &mut memory_account,
            org_type,
            option::some(string::utf8(b"Test Org")),
            option::some(string::utf8(b"Org description")),
            &clock,
            test_scenario::ctx(scenario),
        );
        test_scenario::return_shared(memory_account);
        test_scenario::return_shared(clock);
    }

    public fun create_default_org_in_tx(
        scenario: &mut test_scenario::Scenario,
    ) {
        create_org_in_tx(scenario, memory::org_type_other());
    }

    /// Take the most recently shared org from a prior transaction.
    public fun take_created_org(
        scenario: &mut test_scenario::Scenario,
    ): AgenticOrganization {
        test_scenario::take_shared<AgenticOrganization>(scenario)
    }
}
