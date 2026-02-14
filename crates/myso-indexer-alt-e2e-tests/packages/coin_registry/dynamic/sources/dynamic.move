// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module dynamic::dynamic;

use myso::coin::{CoinCreationAdminCap};
use myso::coin_registry::{Self, CoinRegistry};

public struct Dynamic has key { id: UID }

entry fun new_currency(
    registry: &mut CoinRegistry,
    admin_cap: &CoinCreationAdminCap,
    ctx: &mut TxContext,
) {
    new_currency_impl(registry, admin_cap, ctx);
}

#[test_only]
entry fun new_currency_with_test_cap(
    registry: &mut CoinRegistry,
    ctx: &mut TxContext,
) {
    let admin_cap = myso::coin::create_coin_creation_admin_cap_for_testing(ctx);
    new_currency_impl(registry, &admin_cap, ctx);
}

fun new_currency_impl(
    registry: &mut CoinRegistry,
    admin_cap: &CoinCreationAdminCap,
    ctx: &mut TxContext,
) {
    let (mut init, mut treasury_cap) = coin_registry::new_currency<Dynamic>(
        registry,
        2,
        b"DYNAMIC".to_string(),
        b"Dynamic".to_string(),
        b"A fake dynamic coin for test purposes".to_string(),
        b"https://example.com/dynamic.png".to_string(),
        admin_cap,
        ctx,
    );

    let coin = treasury_cap.mint(1_000_000_000, ctx);
    init.make_supply_fixed(treasury_cap);

    let metadata_cap = init.finalize(ctx);
    transfer::public_transfer(coin, ctx.sender());
    transfer::public_transfer(metadata_cap, @0x0);
}
