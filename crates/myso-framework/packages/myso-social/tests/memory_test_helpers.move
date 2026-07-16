// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias)]
module social_contracts::memory_test_helpers {
    use std::string;
    use std::option;

    use myso::test_scenario;
    use myso::clock::Clock;

    use social_contracts::memory::{Self, MemoryAccount, AgenticOrganization,
        MemoryConfig,
    };
    use social_contracts::profile::{Self, Profile};

    /// Resolve the current transaction sender's linked [`MemoryAccount`].
    public fun take_owner_memory_account(
        sc: &mut test_scenario::Scenario,
    ): MemoryAccount {
        let profile = test_scenario::take_from_sender<Profile>(sc);
        let mem_id = *option::borrow(profile::linked_memory_account_id(&profile));
        test_scenario::return_to_sender(sc, profile);
        test_scenario::take_shared_by_id<MemoryAccount>(sc, mem_id)
    }

    /// Create and share an org in the current transaction. Call from a dedicated
    /// `next_tx` block; take the org in a follow-up transaction via
    /// [`take_created_org`].
    public fun create_org_in_tx(
        sc: &mut test_scenario::Scenario,
        org_type: u8
    ) {
        let mut memory_account = take_owner_memory_account(sc);
        let clock = test_scenario::take_shared<Clock>(sc);
        let memory_config = test_scenario::take_shared<MemoryConfig>(sc);
        memory::test_create_agentic_organization(
            &memory_config,
            &mut memory_account,
            org_type,
            option::some(string::utf8(b"Test Org")),
            option::some(string::utf8(b"Org description")),
            &clock,
            test_scenario::ctx(sc),
        );
        test_scenario::return_shared(memory_config);
        test_scenario::return_shared(memory_account);
        test_scenario::return_shared(clock);
    }

    public fun create_default_org_in_tx(
        sc: &mut test_scenario::Scenario,
    ) {
        create_org_in_tx(sc, memory::org_type_other());
    }

    /// Take the most recently shared org from a prior transaction.
    public fun take_created_org(
        sc: &mut test_scenario::Scenario,
    ): AgenticOrganization {
        test_scenario::take_shared<AgenticOrganization>(sc)
    }
}
