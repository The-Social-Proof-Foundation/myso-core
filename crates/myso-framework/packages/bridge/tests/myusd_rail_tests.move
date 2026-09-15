#[test_only]
module bridge::myusd_rail_tests;

use bridge::bridge_env::{Self, claimed};
use bridge::chain_ids;
use bridge::myusd::MYUSD;
use myso::coin;

const RECIPIENT: address = @0xABCDEF;
const ETH_SENDER: vector<u8> = x"0000000000000000000000000000000000001234";
const EVM_DEST: vector<u8> = x"00000000000000000000000000000000000000aa";

#[test]
fun test_usdc_and_usdt_rails_mint_same_myusd() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    env.create_bridge_default();
    let start_usdc = env.rail_reserve(bridge_env::usdc_id());
    let start_usdt = env.rail_reserve(bridge_env::usdt_id());

    let seq3 = env.bridge_to_myso_rail<MYUSD>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        1_000_000,
        bridge_env::usdc_id(),
    );
    assert!(env.claim_and_transfer_token<MYUSD>(chain_ids::eth_custom(), seq3) == claimed());
    assert!(env.rail_reserve(bridge_env::usdc_id()) == start_usdc + 1_000_000);
    assert!(env.rail_reserve(bridge_env::usdt_id()) == start_usdt);

    let seq4 = env.bridge_to_myso_rail<MYUSD>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        250_000,
        bridge_env::usdt_id(),
    );
    assert!(env.claim_and_transfer_token<MYUSD>(chain_ids::eth_custom(), seq4) == claimed());
    assert!(env.rail_reserve(bridge_env::usdc_id()) == start_usdc + 1_000_000);
    assert!(env.rail_reserve(bridge_env::usdt_id()) == start_usdt + 250_000);

    env.scenario().next_tx(RECIPIENT);
    let a = env.scenario().take_from_sender<coin::Coin<MYUSD>>();
    let b = env.scenario().take_from_sender<coin::Coin<MYUSD>>();
    assert!(a.value() + b.value() == 1_250_000);
    env.scenario().return_to_sender(a);
    env.scenario().return_to_sender(b);
    env.destroy_env();
}

#[test, expected_failure(abort_code = bridge::treasury::EInsufficientRailReserve)]
fun test_withdraw_usdt_fails_when_only_usdc_reserve() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    env.create_bridge_default();
    let seq = env.bridge_to_myso_rail<MYUSD>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        1_000_000,
        bridge_env::usdc_id(),
    );
    assert!(env.claim_and_transfer_token<MYUSD>(chain_ids::eth_custom(), seq) == claimed());

    env.scenario().next_tx(RECIPIENT);
    let myusd = env.scenario().take_from_sender<coin::Coin<MYUSD>>();
    env.send_token_with_rail(
        RECIPIENT,
        chain_ids::eth_custom(),
        EVM_DEST,
        myusd,
        bridge_env::usdt_id(),
    );
    abort 0
}

#[test]
fun test_withdraw_usdc_succeeds_from_usdc_reserve() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    env.create_bridge_default();
    let start_usdc = env.rail_reserve(bridge_env::usdc_id());
    let seq = env.bridge_to_myso_rail<MYUSD>(
        chain_ids::eth_custom(),
        ETH_SENDER,
        RECIPIENT,
        1_000_000,
        bridge_env::usdc_id(),
    );
    assert!(env.claim_and_transfer_token<MYUSD>(chain_ids::eth_custom(), seq) == claimed());

    env.scenario().next_tx(RECIPIENT);
    let myusd = env.scenario().take_from_sender<coin::Coin<MYUSD>>();
    env.send_token_with_rail(
        RECIPIENT,
        chain_ids::eth_custom(),
        EVM_DEST,
        myusd,
        bridge_env::usdc_id(),
    );
    assert!(env.rail_reserve(bridge_env::usdc_id()) == start_usdc);
    env.destroy_env();
}

#[test, expected_failure(abort_code = bridge::treasury::ETokenRequiresRailId)]
fun test_send_token_myusd_without_rail_aborts() {
    let mut env = bridge_env::create_env(chain_ids::myso_custom());
    env.create_bridge_default();
    let myusd = env.get_myusd(100);
    env.send_token(RECIPIENT, chain_ids::eth_custom(), EVM_DEST, myusd);
    abort 0
}
