// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module bridge::native_myso_bridge_tests;

use bridge::bridge::{Self, new_for_testing};
use bridge::chain_ids;
use bridge::treasury;
use myso::coin::{Self};
use myso::myso::MYSO;
use myso::test_scenario;
use std::unit_test::destroy;

const BOOTSTRAP_MIST: u64 = 50_000_000_000_000_000;

#[test, expected_failure(abort_code = 6)]
fun test_bootstrap_wrong_amount_aborts() {
    let mut scenario = test_scenario::begin(@0x1);
    let ctx = scenario.ctx();
    let mut bridge = new_for_testing(chain_ids::myso_testnet(), ctx);
    let wrong = coin::mint_for_testing<MYSO>(1, ctx);
    bridge.bootstrap_native_myso(wrong);
    destroy(bridge);
    scenario.end();
    abort 0
}

#[test, expected_failure(abort_code = 5)]
fun test_bootstrap_twice_aborts() {
    let mut scenario = test_scenario::begin(@0x1);
    let ctx = scenario.ctx();
    let mut bridge = new_for_testing(chain_ids::myso_testnet(), ctx);
    let c1 = coin::mint_for_testing<MYSO>(BOOTSTRAP_MIST, ctx);
    bridge.bootstrap_native_myso(c1);
    let c2 = coin::mint_for_testing<MYSO>(BOOTSTRAP_MIST, ctx);
    bridge.bootstrap_native_myso(c2);
    destroy(bridge);
    scenario.end();
    abort 0
}

#[test]
fun test_bootstrap_locks_escrow_and_registers_myso_id_zero() {
    let mut scenario = test_scenario::begin(@0x1);
    let ctx = scenario.ctx();
    let mut bridge = new_for_testing(chain_ids::myso_testnet(), ctx);
    let c = coin::mint_for_testing<MYSO>(BOOTSTRAP_MIST, ctx);
    bridge.bootstrap_native_myso(c);

    let inner = bridge.test_load_inner();
    let t = inner.inner_treasury();
    assert!(treasury::native_myso_locked_amount(t) == BOOTSTRAP_MIST, 0);
    assert!(treasury::native_bridge_ready(t), 1);
    assert!(treasury::token_id<MYSO>(t) == 0, 2);

    destroy(bridge);
    scenario.end();
}

#[test, expected_failure(abort_code = 20)]
fun test_send_token_myso_aborts_after_bootstrap() {
    let mut scenario = test_scenario::begin(@0x1);
    let ctx = scenario.ctx();
    let mut bridge = new_for_testing(chain_ids::myso_testnet(), ctx);
    let c = coin::mint_for_testing<MYSO>(BOOTSTRAP_MIST, ctx);
    bridge.bootstrap_native_myso(c);

    let extra = coin::mint_for_testing<MYSO>(1_000, ctx);
    bridge.send_token(
        chain_ids::eth_sepolia(),
        x"0000000000000000000000000000000000000000",
        extra,
        ctx,
    );
    destroy(bridge);
    scenario.end();
    abort 0
}

#[test]
fun test_send_myso_token_locks_additional_mist() {
    let mut scenario = test_scenario::begin(@0xA);
    let ctx = scenario.ctx();
    let mut bridge = new_for_testing(chain_ids::myso_testnet(), ctx);
    let c = coin::mint_for_testing<MYSO>(BOOTSTRAP_MIST, ctx);
    bridge.bootstrap_native_myso(c);

    let send_amt = 5_000u64;
    let coin_out = coin::mint_for_testing<MYSO>(send_amt, ctx);
    bridge.send_myso_token(
        chain_ids::eth_sepolia(),
        x"0000000000000000000000000000000000000000",
        coin_out,
        ctx,
    );

    let inner = bridge.test_load_inner();
    let t = inner.inner_treasury();
    assert!(treasury::native_myso_locked_amount(t) == BOOTSTRAP_MIST + send_amt, 0);

    destroy(bridge);
    scenario.end();
}
