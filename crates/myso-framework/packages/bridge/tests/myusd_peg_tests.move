#[test_only]
module bridge::myusd_peg_tests;

use bridge::bridge_env::{Self, claimed};
use bridge::chain_ids;
use bridge::eth::ETH;
use bridge::myusd::MYUSD;
use bridge::myusd_peg::{Self, MyUsdPeg, StableClaimConvertedToMyUsd};
use bridge::usdc::USDC;
use bridge::usdt::USDT;
use myso::coin;
use myso::event;
use myso::test_scenario;

const RECIPIENT: address = @0xABCDEF;
const ETH_SENDER: vector<u8> = x"0000000000000000000000000000000000001234";
const EVM_DEST: vector<u8> = x"00000000000000000000000000000000000000aa";

fun setup_with_conversion(
    env: &mut bridge_env::BridgeEnv,
    token_id: u8,
    boundary_seq: u64,
) {
    env.create_bridge_default();
    let admin = env.setup_myusd_peg(@0x0);
    env.enable_stable_conversion(&admin, token_id, boundary_seq);
    transfer::public_transfer(admin, @0x0);
}

fun take_peg(env: &mut bridge_env::BridgeEnv): MyUsdPeg {
    env.scenario().next_tx(@0x0);
    env.scenario().take_shared<MyUsdPeg>()
}

fun return_peg(peg: MyUsdPeg) {
    test_scenario::return_shared(peg);
}

#[test]
fun test_usdc_converts_1_to_1() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    setup_with_conversion(&mut env, 3, 0);
    let seq = env.bridge_to_myso<USDC>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        1_000_000,
    );
    env.claim_stable_into_myusd<USDC>(chain_ids::eth_custom(), seq);

    let events = event::events_by_type<StableClaimConvertedToMyUsd>();
    assert!(events.length() == 1);
    assert!(myusd_peg::converted_recipient(&events[0]) == RECIPIENT);
    assert!(myusd_peg::converted_input_amount(&events[0]) == 1_000_000);
    assert!(myusd_peg::converted_myusd_amount(&events[0]) == 1_000_000);

    let peg = take_peg(&mut env);
    assert!(peg.usdc_reserve() == 1_000_000);
    assert!(peg.usdt_reserve() == 0);
    assert!(peg.usdc_issued() == 1_000_000);
    peg.assert_backing_invariant();
    return_peg(peg);

    env.scenario().next_tx(RECIPIENT);
    let myusd = env.scenario().take_from_sender<coin::Coin<MYUSD>>();
    assert!(myusd.value() == 1_000_000);
    env.scenario().return_to_sender(myusd);
    env.destroy_env();
}

#[test]
fun test_usdt_converts_1_to_1() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    setup_with_conversion(&mut env, 4, 0);
    let seq = env.bridge_to_myso<USDT>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        250_000,
    );
    env.claim_stable_into_myusd<USDT>(chain_ids::eth_custom(), seq);

    let peg = take_peg(&mut env);
    assert!(peg.usdt_reserve() == 250_000);
    assert!(peg.usdc_reserve() == 0);
    peg.assert_backing_invariant();
    return_peg(peg);
    env.destroy_env();
}

#[test, expected_failure(abort_code = 22, location = bridge::bridge)]
fun test_direct_usdc_claim_aborts_after_boundary() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    setup_with_conversion(&mut env, 3, 0);
    let seq = env.bridge_to_myso<USDC>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        100,
    );
    env.claim_and_transfer_token<USDC>(chain_ids::eth_custom(), seq);
    abort
}

#[test]
fun test_eth_direct_claim_still_works() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    setup_with_conversion(&mut env, 3, 0);
    let seq = env.bridge_to_myso<ETH>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        50,
    );
    assert!(env.claim_and_transfer_token<ETH>(chain_ids::eth_custom(), seq) == claimed());
    env.destroy_env();
}

#[test]
fun test_pre_boundary_usdc_still_direct_claimable() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    setup_with_conversion(&mut env, 3, 20);
    let seq = env.bridge_to_myso<USDC>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        77,
    );
    assert!(seq < 20);
    assert!(env.claim_and_transfer_token<USDC>(chain_ids::eth_custom(), seq) == claimed());
    env.destroy_env();
}

#[test, expected_failure(abort_code = 2, location = bridge::myusd_peg)]
fun test_disabled_rail_rejects_convert() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    env.create_bridge_default();
    let admin = env.setup_myusd_peg(@0x0);
    env.enable_conversion_policy_only(&admin, 3, 0);
    transfer::public_transfer(admin, @0x0);

    let seq = env.bridge_to_myso<USDC>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        10,
    );
    env.claim_stable_into_myusd<USDC>(chain_ids::eth_custom(), seq);
    abort
}

#[test]
fun test_duplicate_convert_is_noop() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    setup_with_conversion(&mut env, 3, 0);
    let seq = env.bridge_to_myso<USDC>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        9,
    );
    env.claim_stable_into_myusd<USDC>(chain_ids::eth_custom(), seq);
    env.claim_stable_into_myusd<USDC>(chain_ids::eth_custom(), seq);
    let peg = take_peg(&mut env);
    assert!(peg.usdc_reserve() == 9);
    assert!(peg.usdc_issued() == 9);
    return_peg(peg);
    env.destroy_env();
}

#[test, expected_failure(abort_code = 4, location = bridge::myusd_peg)]
fun test_underfunded_rail_aborts_without_burn() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    setup_with_conversion(&mut env, 3, 0);
    env.scenario().next_tx(RECIPIENT);
    let fake = coin::mint_for_testing<MYUSD>(100, env.scenario().ctx());
    env.redeem_myusd_and_send_to_evm<USDC>(
        RECIPIENT,
        fake,
        chain_ids::eth_custom(),
        EVM_DEST,
    );
    abort
}

#[test]
fun test_redeem_and_send_burns_and_withdraws() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    setup_with_conversion(&mut env, 3, 0);
    let seq = env.bridge_to_myso<USDC>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        500,
    );
    env.claim_stable_into_myusd<USDC>(chain_ids::eth_custom(), seq);

    env.scenario().next_tx(RECIPIENT);
    let myusd = env.scenario().take_from_sender<coin::Coin<MYUSD>>();
    env.redeem_myusd_and_send_to_evm<USDC>(
        RECIPIENT,
        myusd,
        chain_ids::eth_custom(),
        EVM_DEST,
    );

    let peg = take_peg(&mut env);
    assert!(peg.usdc_reserve() == 0);
    assert!(peg.usdc_redeemed() == 500);
    peg.assert_backing_invariant();
    return_peg(peg);
    env.destroy_env();
}
