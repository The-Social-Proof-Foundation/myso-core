// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<MYSO> is the token used to pay for gas in MySo.
/// It has 9 decimals, and the smallest unit (10^-9) is called "mist".
module myso::myso;

use myso::balance::Balance;
use myso::coin;

const EAlreadyMinted: u64 = 0;
/// Sender is not @0x0 the system address.
const ENotSystemAddress: u64 = 1;

#[allow(unused_const)]
/// The amount of Mist per MySo token based on the fact that mist is
/// 10^-9 of a MySo token
const MIST_PER_MYSO: u64 = 1_000_000_000;

#[allow(unused_const)]
/// The total supply of MySo denominated in whole MySo tokens (10 Billion)
const TOTAL_SUPPLY_MYSO: u64 = 10_000_000_000;

/// The total supply of MySo denominated in Mist (10 Billion * 10^9)
const TOTAL_SUPPLY_MIST: u64 = 10_000_000_000_000_000_000;

/// Name of the coin
public struct MYSO has drop {}

#[allow(unused_function, deprecated_usage)]
/// Register the `MYSO` Coin to acquire its `Supply`.
/// This should be called only once during genesis creation.
fun new(ctx: &mut TxContext): Balance<MYSO> {
    assert!(ctx.sender() == @0x0, ENotSystemAddress);
    assert!(ctx.epoch() == 0, EAlreadyMinted);

    let (treasury, metadata) = coin::create_currency(
        MYSO {},
        9,
        b"MySo",
        b"MySocial",
        b"The official native token of MySocial blockchain.",
        option::none(),
        ctx,
    );
    transfer::public_freeze_object(metadata);
    let mut supply = treasury.treasury_into_supply();
    let total_myso = supply.increase_supply(TOTAL_SUPPLY_MIST);
    supply.destroy_supply();
    total_myso
}

#[allow(lint(public_entry))]
public entry fun transfer(c: coin::Coin<MYSO>, recipient: address) {
    transfer::public_transfer(c, recipient)
}
