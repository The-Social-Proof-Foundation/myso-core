#[test_only]
module messaging::paid_escrow_settlement_tests;

use myso::coin::{Self, Coin};
use myso::myso::MYSO;
use myso::test_scenario as ts;
use messaging::paid_escrow_settlement as fees;
use std::unit_test::assert_eq;

const ALICE: address = @0xA11CE;

#[test]
fun distribute_matches_social_bps() {
    let mut s = ts::begin(ALICE);
    s.next_tx(ALICE);
    let coin_in = coin::mint_for_testing<MYSO>(10_000, s.ctx());
    let totals = fees::distribute_escrow_to_recipients(coin_in, @0x1, @0x2, @0x3, s.ctx());
    assert_eq!(fees::total_amount(&totals), 10_000);
    assert_eq!(fees::platform_fee(&totals), 250);
    assert_eq!(fees::treasury_fee(&totals), 250);
    assert_eq!(fees::net_amount(&totals), 9_500);
    s.end();
}

const ECOSYSTEM: address = @0xEC05;
const RECIPIENT: address = @0xB0B;

#[test]
fun distribute_no_platform_routes_both_fees_to_ecosystem() {
    let mut s = ts::begin(ALICE);
    s.next_tx(ALICE);
    let coin_in = coin::mint_for_testing<MYSO>(10_000, s.ctx());
    let totals = fees::distribute_escrow_to_recipients(
        coin_in,
        fees::no_platform_fee_recipient(),
        ECOSYSTEM,
        RECIPIENT,
        s.ctx(),
    );
    assert_eq!(fees::total_amount(&totals), 10_000);
    assert_eq!(fees::platform_fee(&totals), 250);
    assert_eq!(fees::treasury_fee(&totals), 250);
    assert_eq!(fees::net_amount(&totals), 9_500);

    s.next_tx(ECOSYSTEM);
    let ecosystem_coin = s.take_from_sender<Coin<MYSO>>();
    assert_eq!(coin::value(&ecosystem_coin), 500);
    coin::burn_for_testing(ecosystem_coin);

    s.next_tx(RECIPIENT);
    let recipient_coin = s.take_from_sender<Coin<MYSO>>();
    assert_eq!(coin::value(&recipient_coin), 9_500);
    coin::burn_for_testing(recipient_coin);

    s.end();
}
