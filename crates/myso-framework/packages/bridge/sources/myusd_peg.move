// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Atomic USDC/USDT ↔ MyUSD peg. The relay never holds either coin.
module bridge::myusd_peg;

use std::type_name;

use bridge::bridge::{Self, Bridge};
use bridge::myusd::{Self, MYUSD};
use bridge::usdc::{Self, USDC};
use bridge::usdt::{Self, USDT};
use myso::balance::{Self, Balance};
use myso::clock::Clock;
use myso::coin::{Self, Coin, TreasuryCap};
use myso::event;

const EUnsupportedRail: u64 = 0;
const EZeroAmount: u64 = 1;
const ERailDisabled: u64 = 2;
const EPegPaused: u64 = 3;
const EInsufficientReserve: u64 = 4;
const EDecimalMismatch: u64 = 5;
const ENotAdmin: u64 = 6;
const EAlreadyInitialized: u64 = 7;
const EInvalidPolicy: u64 = 8;

const TOKEN_ID_USDC: u8 = 3;
const TOKEN_ID_USDT: u8 = 4;
const STABLE_DECIMALS: u8 = 6;

public struct MyUsdPeg has key {
    id: UID,
    myusd_treasury: TreasuryCap<MYUSD>,
    usdc_reserve: Balance<USDC>,
    usdt_reserve: Balance<USDT>,
    usdc_enabled: bool,
    usdt_enabled: bool,
    usdc_issued: u64,
    usdt_issued: u64,
    usdc_redeemed: u64,
    usdt_redeemed: u64,
    paused: bool,
    admin: address,
}

public struct ConversionAdminCap has key, store {
    id: UID,
}

public struct StableClaimConvertedToMyUsd has copy, drop {
    message_key_source_chain: u8,
    message_key_seq: u64,
    source_chain: u8,
    bridge_seq_num: u64,
    rail_token_id: u8,
    input_amount: u64,
    myusd_amount: u64,
    recipient: address,
    peg_id: ID,
    usdc_reserve: u64,
    usdt_reserve: u64,
    usdc_issued: u64,
    usdt_issued: u64,
}

public struct StableRedeemedToRail has copy, drop {
    rail_token_id: u8,
    myusd_amount: u64,
    rail_amount: u64,
    sender: address,
    peg_id: ID,
    usdc_reserve: u64,
    usdt_reserve: u64,
}

/// Create the shared peg. `treasury_cap` must be the sole MyUSD mint authority.
public fun create(treasury_cap: TreasuryCap<MYUSD>, ctx: &mut TxContext): ConversionAdminCap {
    assert!(coin::total_supply(&treasury_cap) == 0, EAlreadyInitialized);
    assert!(myusd::decimals() == STABLE_DECIMALS, EDecimalMismatch);
    assert!(usdc::decimals() == STABLE_DECIMALS, EDecimalMismatch);
    assert!(usdt::decimals() == STABLE_DECIMALS, EDecimalMismatch);

    let admin = tx_context::sender(ctx);
    let peg = MyUsdPeg {
        id: object::new(ctx),
        myusd_treasury: treasury_cap,
        usdc_reserve: balance::zero(),
        usdt_reserve: balance::zero(),
        usdc_enabled: false,
        usdt_enabled: false,
        usdc_issued: 0,
        usdt_issued: 0,
        usdc_redeemed: 0,
        usdt_redeemed: 0,
        paused: false,
        admin,
    };
    transfer::share_object(peg);
    ConversionAdminCap { id: object::new(ctx) }
}

public fun set_paused(peg: &mut MyUsdPeg, cap: &ConversionAdminCap, paused: bool, ctx: &TxContext) {
    assert_admin(peg, cap, ctx);
    peg.paused = paused;
}

public fun set_rail_enabled(
    peg: &mut MyUsdPeg,
    cap: &ConversionAdminCap,
    token_id: u8,
    enabled: bool,
    ctx: &TxContext,
) {
    assert_admin(peg, cap, ctx);
    if (token_id == TOKEN_ID_USDC) {
        peg.usdc_enabled = enabled;
    } else {
        assert!(token_id == TOKEN_ID_USDT, EUnsupportedRail);
        peg.usdt_enabled = enabled;
    };
}

/// Enable conversion policy on the bridge at `(boundary_source_chain, boundary_seq)`.
public fun enable_conversion_policy(
    bridge: &mut Bridge,
    peg: &MyUsdPeg,
    cap: &ConversionAdminCap,
    token_id: u8,
    boundary_source_chain: u8,
    boundary_seq: u64,
    ctx: &TxContext,
) {
    assert_admin(peg, cap, ctx);
    assert!(token_id == TOKEN_ID_USDC || token_id == TOKEN_ID_USDT, EUnsupportedRail);
    bridge::set_token_claim_policy(
        bridge,
        token_id,
        bridge::claim_policy_convert_to_myusd(),
        boundary_source_chain,
        boundary_seq,
        ctx,
    );
}

public fun usdc_reserve(peg: &MyUsdPeg): u64 { balance::value(&peg.usdc_reserve) }

public fun usdt_reserve(peg: &MyUsdPeg): u64 { balance::value(&peg.usdt_reserve) }

public fun usdc_issued(peg: &MyUsdPeg): u64 { peg.usdc_issued }

public fun usdt_issued(peg: &MyUsdPeg): u64 { peg.usdt_issued }

public fun usdc_redeemed(peg: &MyUsdPeg): u64 { peg.usdc_redeemed }

public fun usdt_redeemed(peg: &MyUsdPeg): u64 { peg.usdt_redeemed }

public fun usdc_enabled(peg: &MyUsdPeg): bool { peg.usdc_enabled }

public fun usdt_enabled(peg: &MyUsdPeg): bool { peg.usdt_enabled }

public fun is_paused(peg: &MyUsdPeg): bool { peg.paused }

public fun converted_recipient(event: &StableClaimConvertedToMyUsd): address {
    event.recipient
}

public fun converted_input_amount(event: &StableClaimConvertedToMyUsd): u64 {
    event.input_amount
}

public fun converted_myusd_amount(event: &StableClaimConvertedToMyUsd): u64 {
    event.myusd_amount
}

public fun myusd_total_supply(peg: &MyUsdPeg): u64 {
    coin::total_supply(&peg.myusd_treasury)
}

/// Generic dispatcher. `T` must be `USDC` or `USDT`; each path uses a typed reserve field.
public fun claim_stable_into_myusd<T>(
    bridge: &mut Bridge,
    peg: &mut MyUsdPeg,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &mut TxContext,
) {
    if (is_usdc<T>()) {
        claim_usdc_into_myusd(bridge, peg, clock, source_chain, bridge_seq_num, ctx);
    } else {
        assert!(is_usdt<T>(), EUnsupportedRail);
        claim_usdt_into_myusd(bridge, peg, clock, source_chain, bridge_seq_num, ctx);
    };
}

public fun redeem_myusd_and_send_to_evm<T>(
    bridge: &mut Bridge,
    peg: &mut MyUsdPeg,
    myusd: Coin<MYUSD>,
    target_chain: u8,
    target_address: vector<u8>,
    ctx: &mut TxContext,
) {
    if (is_usdc<T>()) {
        let coin_t = redeem_myusd_to_rail_usdc(peg, myusd, ctx);
        bridge::send_token(bridge, target_chain, target_address, coin_t, ctx);
    } else {
        assert!(is_usdt<T>(), EUnsupportedRail);
        let coin_t = redeem_myusd_to_rail_usdt(peg, myusd, ctx);
        bridge::send_token(bridge, target_chain, target_address, coin_t, ctx);
    };
}

fun claim_usdc_into_myusd(
    bridge: &mut Bridge,
    peg: &mut MyUsdPeg,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &mut TxContext,
) {
    assert_can_convert(peg, TOKEN_ID_USDC, bridge, source_chain, bridge_seq_num);
    let (maybe_token, recipient) = bridge::claim_token_internal<USDC>(
        bridge,
        clock,
        source_chain,
        bridge_seq_num,
        ctx,
    );
    if (maybe_token.is_none()) {
        maybe_token.destroy_none();
        return
    };
    let coin_t = maybe_token.destroy_some();
    let amount = finish_usdc_deposit(peg, coin_t, recipient, source_chain, bridge_seq_num, ctx);
    let _ = amount;
}

fun claim_usdt_into_myusd(
    bridge: &mut Bridge,
    peg: &mut MyUsdPeg,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &mut TxContext,
) {
    assert_can_convert(peg, TOKEN_ID_USDT, bridge, source_chain, bridge_seq_num);
    let (maybe_token, recipient) = bridge::claim_token_internal<USDT>(
        bridge,
        clock,
        source_chain,
        bridge_seq_num,
        ctx,
    );
    if (maybe_token.is_none()) {
        maybe_token.destroy_none();
        return
    };
    let coin_t = maybe_token.destroy_some();
    finish_usdt_deposit(peg, coin_t, recipient, source_chain, bridge_seq_num, ctx);
}

fun finish_usdc_deposit(
    peg: &mut MyUsdPeg,
    coin_t: Coin<USDC>,
    recipient: address,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &mut TxContext,
): u64 {
    let amount = coin::value(&coin_t);
    assert!(amount > 0, EZeroAmount);
    balance::join(&mut peg.usdc_reserve, coin::into_balance(coin_t));
    peg.usdc_issued = peg.usdc_issued + amount;
    pay_myusd(peg, amount, recipient, TOKEN_ID_USDC, source_chain, bridge_seq_num, ctx);
    amount
}

fun finish_usdt_deposit(
    peg: &mut MyUsdPeg,
    coin_t: Coin<USDT>,
    recipient: address,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &mut TxContext,
) {
    let amount = coin::value(&coin_t);
    assert!(amount > 0, EZeroAmount);
    balance::join(&mut peg.usdt_reserve, coin::into_balance(coin_t));
    peg.usdt_issued = peg.usdt_issued + amount;
    pay_myusd(peg, amount, recipient, TOKEN_ID_USDT, source_chain, bridge_seq_num, ctx);
}

fun pay_myusd(
    peg: &mut MyUsdPeg,
    amount: u64,
    recipient: address,
    rail_token_id: u8,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &mut TxContext,
) {
    let myusd = coin::mint(&mut peg.myusd_treasury, amount, ctx);
    transfer::public_transfer(myusd, recipient);
    event::emit(StableClaimConvertedToMyUsd {
        message_key_source_chain: source_chain,
        message_key_seq: bridge_seq_num,
        source_chain,
        bridge_seq_num,
        rail_token_id,
        input_amount: amount,
        myusd_amount: amount,
        recipient,
        peg_id: object::id(peg),
        usdc_reserve: balance::value(&peg.usdc_reserve),
        usdt_reserve: balance::value(&peg.usdt_reserve),
        usdc_issued: peg.usdc_issued,
        usdt_issued: peg.usdt_issued,
    });
}

public(package) fun redeem_myusd_to_rail_usdc(
    peg: &mut MyUsdPeg,
    myusd: Coin<MYUSD>,
    ctx: &mut TxContext,
): Coin<USDC> {
    let amount = prepare_redeem(peg, &myusd, TOKEN_ID_USDC);
    assert!(balance::value(&peg.usdc_reserve) >= amount, EInsufficientReserve);
    coin::burn(&mut peg.myusd_treasury, myusd);
    peg.usdc_redeemed = peg.usdc_redeemed + amount;
    emit_redeem(peg, TOKEN_ID_USDC, amount, ctx);
    coin::from_balance(balance::split(&mut peg.usdc_reserve, amount), ctx)
}

public(package) fun redeem_myusd_to_rail_usdt(
    peg: &mut MyUsdPeg,
    myusd: Coin<MYUSD>,
    ctx: &mut TxContext,
): Coin<USDT> {
    let amount = prepare_redeem(peg, &myusd, TOKEN_ID_USDT);
    assert!(balance::value(&peg.usdt_reserve) >= amount, EInsufficientReserve);
    coin::burn(&mut peg.myusd_treasury, myusd);
    peg.usdt_redeemed = peg.usdt_redeemed + amount;
    emit_redeem(peg, TOKEN_ID_USDT, amount, ctx);
    coin::from_balance(balance::split(&mut peg.usdt_reserve, amount), ctx)
}

fun prepare_redeem(peg: &MyUsdPeg, myusd: &Coin<MYUSD>, token_id: u8): u64 {
    assert!(!peg.paused, EPegPaused);
    assert!(rail_enabled(peg, token_id), ERailDisabled);
    let amount = coin::value(myusd);
    assert!(amount > 0, EZeroAmount);
    amount
}

fun emit_redeem(peg: &MyUsdPeg, token_id: u8, amount: u64, ctx: &TxContext) {
    event::emit(StableRedeemedToRail {
        rail_token_id: token_id,
        myusd_amount: amount,
        rail_amount: amount,
        sender: tx_context::sender(ctx),
        peg_id: object::id(peg),
        usdc_reserve: balance::value(&peg.usdc_reserve),
        usdt_reserve: balance::value(&peg.usdt_reserve),
    });
}

fun assert_can_convert(
    peg: &MyUsdPeg,
    token_id: u8,
    bridge: &Bridge,
    source_chain: u8,
    bridge_seq_num: u64,
) {
    assert!(!peg.paused, EPegPaused);
    assert!(rail_enabled(peg, token_id), ERailDisabled);
    let policy = bridge::effective_claim_policy(bridge, token_id, source_chain, bridge_seq_num);
    assert!(policy == bridge::claim_policy_convert_to_myusd(), EInvalidPolicy);
}

fun rail_enabled(peg: &MyUsdPeg, token_id: u8): bool {
    if (token_id == TOKEN_ID_USDC) {
        peg.usdc_enabled
    } else {
        peg.usdt_enabled
    }
}

fun is_usdc<T>(): bool {
    type_name::with_defining_ids<T>() == type_name::with_defining_ids<USDC>()
}

fun is_usdt<T>(): bool {
    type_name::with_defining_ids<T>() == type_name::with_defining_ids<USDT>()
}

fun assert_admin(peg: &MyUsdPeg, _cap: &ConversionAdminCap, ctx: &TxContext) {
    assert!(tx_context::sender(ctx) == peg.admin, ENotAdmin);
}

#[test_only]
public fun create_for_testing(
    treasury_cap: TreasuryCap<MYUSD>,
    ctx: &mut TxContext,
): (MyUsdPeg, ConversionAdminCap) {
    assert!(coin::total_supply(&treasury_cap) == 0, EAlreadyInitialized);
    let admin = tx_context::sender(ctx);
    let peg = MyUsdPeg {
        id: object::new(ctx),
        myusd_treasury: treasury_cap,
        usdc_reserve: balance::zero(),
        usdt_reserve: balance::zero(),
        usdc_enabled: false,
        usdt_enabled: false,
        usdc_issued: 0,
        usdt_issued: 0,
        usdc_redeemed: 0,
        usdt_redeemed: 0,
        paused: false,
        admin,
    };
    (peg, ConversionAdminCap { id: object::new(ctx) })
}

#[test_only]
public fun share_for_testing(peg: MyUsdPeg) {
    transfer::share_object(peg)
}

#[test_only]
public fun assert_backing_invariant(peg: &MyUsdPeg) {
    let circulating = coin::total_supply(&peg.myusd_treasury);
    let reserved = balance::value(&peg.usdc_reserve) + balance::value(&peg.usdt_reserve);
    assert!(circulating == reserved, 0);
}

#[test_only]
public fun seed_usdc_reserve(peg: &mut MyUsdPeg, coin: Coin<USDC>) {
    let amount = coin::value(&coin);
    balance::join(&mut peg.usdc_reserve, coin::into_balance(coin));
    peg.usdc_issued = peg.usdc_issued + amount;
}

#[test_only]
public fun seed_usdt_reserve(peg: &mut MyUsdPeg, coin: Coin<USDT>) {
    let amount = coin::value(&coin);
    balance::join(&mut peg.usdt_reserve, coin::into_balance(coin));
    peg.usdt_issued = peg.usdt_issued + amount;
}
