// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(deprecated_usage)] // TODO: update tests to not use deprecated governance
module locked_stake::locked_stake_tests;

use locked_stake::epoch_time_lock;
use locked_stake::locked_stake as ls;
use std::unit_test::{assert_eq, destroy};
use myso::balance;
use myso::coin;
use myso::test_scenario;
use myso::vec_map;
use myso_system::governance_test_utils::{advance_epoch, set_up_myso_system_state};
use myso_system::myso_system::{Self, MySoSystemState};

const MIST_PER_MYSO: u64 = 1_000_000_000;

#[test]
#[expected_failure(abort_code = epoch_time_lock::EEpochAlreadyPassed)]
fun test_incorrect_creation() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_myso_system_state(vector[@0x1, @0x2, @0x3]);

    // Advance epoch twice so we are now at epoch 2.
    advance_epoch(scenario);
    advance_epoch(scenario);
    let ctx = test_scenario::ctx(scenario);
    assert_eq!(tx_context::epoch(ctx), 2);

    // Create a locked stake with epoch 1. Should fail here.
    let ls = ls::new(1, ctx);

    destroy(ls);
    test_scenario::end(scenario_val);
}

#[test]
fun test_deposit_stake_unstake() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_myso_system_state(vector[@0x1, @0x2, @0x3]);

    let mut ls = ls::new(10, test_scenario::ctx(scenario));

    // Deposit 100 MYSO.
    ls::deposit_myso(&mut ls, balance::create_for_testing(100 * MIST_PER_MYSO));

    assert_eq!(ls::myso_balance(&ls), 100 * MIST_PER_MYSO);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<MySoSystemState>(scenario);

    // Stake 10 of the 100 MYSO.
    ls::stake(&mut ls, &mut system_state, 10 * MIST_PER_MYSO, @0x1, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);

    assert_eq!(ls::myso_balance(&ls), 90 * MIST_PER_MYSO);
    assert_eq!(vec_map::length(ls::staked_myso(&ls)), 1);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<MySoSystemState>(scenario);
    let ctx = test_scenario::ctx(scenario);

    // Create a StakedMySo object and add it to the LockedStake object.
    let staked_myso = myso_system::request_add_stake_non_entry(
        &mut system_state,
        coin::mint_for_testing(20 * MIST_PER_MYSO, ctx),
        @0x2,
        ctx,
    );
    test_scenario::return_shared(system_state);

    ls::deposit_staked_myso(&mut ls, staked_myso);
    assert_eq!(ls::myso_balance(&ls), 90 * MIST_PER_MYSO);
    assert_eq!(vec_map::length(ls::staked_myso(&ls)), 2);
    advance_epoch(scenario);

    test_scenario::next_tx(scenario, @0x1);
    let (staked_myso_id, _) = vec_map::get_entry_by_idx(ls::staked_myso(&ls), 0);
    let mut system_state = test_scenario::take_shared<MySoSystemState>(scenario);

    // Unstake both stake objects
    ls::unstake(&mut ls, &mut system_state, *staked_myso_id, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);
    assert_eq!(ls::myso_balance(&ls), 100 * MIST_PER_MYSO);
    assert_eq!(vec_map::length(ls::staked_myso(&ls)), 1);

    test_scenario::next_tx(scenario, @0x1);
    let (staked_myso_id, _) = vec_map::get_entry_by_idx(ls::staked_myso(&ls), 0);
    let mut system_state = test_scenario::take_shared<MySoSystemState>(scenario);
    ls::unstake(&mut ls, &mut system_state, *staked_myso_id, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);
    assert_eq!(ls::myso_balance(&ls), 120 * MIST_PER_MYSO);
    assert_eq!(vec_map::length(ls::staked_myso(&ls)), 0);

    destroy(ls);
    test_scenario::end(scenario_val);
}

#[test]
fun test_unlock_correct_epoch() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_myso_system_state(vector[@0x1, @0x2, @0x3]);

    let mut ls = ls::new(2, test_scenario::ctx(scenario));

    ls::deposit_myso(&mut ls, balance::create_for_testing(100 * MIST_PER_MYSO));

    assert_eq!(ls::myso_balance(&ls), 100 * MIST_PER_MYSO);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<MySoSystemState>(scenario);
    ls::stake(&mut ls, &mut system_state, 10 * MIST_PER_MYSO, @0x1, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);

    advance_epoch(scenario);
    advance_epoch(scenario);
    advance_epoch(scenario);
    advance_epoch(scenario);

    let (staked_myso, myso_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
    assert_eq!(balance::value(&myso_balance), 90 * MIST_PER_MYSO);
    assert_eq!(vec_map::length(&staked_myso), 1);

    destroy(staked_myso);
    destroy(myso_balance);
    test_scenario::end(scenario_val);
}

#[test]
#[expected_failure(abort_code = epoch_time_lock::EEpochNotYetEnded)]
fun test_unlock_incorrect_epoch() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_myso_system_state(vector[@0x1, @0x2, @0x3]);

    let ls = ls::new(2, test_scenario::ctx(scenario));
    let (staked_myso, myso_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
    destroy(staked_myso);
    destroy(myso_balance);
    test_scenario::end(scenario_val);
}
