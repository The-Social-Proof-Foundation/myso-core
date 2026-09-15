// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Bridged USDT type owned by the Bridge package so `MyUsdPeg` can hold `Balance<USDT>`.
module bridge::usdt;

public struct USDT has drop {}

const DECIMAL: u8 = 6;

public fun decimals(): u8 {
    DECIMAL
}

#[test_only]
public fun create_bridge_token(
    ctx: &mut TxContext,
): (myso::package::UpgradeCap, myso::coin::TreasuryCap<USDT>, myso::coin::CoinMetadata<USDT>) {
    use std::ascii;
    use std::type_name;
    use myso::address;
    use myso::coin;
    use myso::hex;
    use myso::package::test_publish;
    let admin = coin::create_coin_creation_admin_cap_for_testing(ctx);
    let (treasury_cap, metadata) = coin::create_currency_with_admin<USDT>(
        DECIMAL,
        b"usdt",
        b"usdt",
        b"bridge usdt token",
        option::none(),
        &admin,
        ctx,
    );
    std::unit_test::destroy(admin);
    let type_name = type_name::with_defining_ids<USDT>();
    let address_bytes = hex::decode(ascii::into_bytes(type_name::address_string(&type_name)));
    let coin_id = address::from_bytes(address_bytes).to_id();
    (test_publish(coin_id, ctx), treasury_cap, metadata)
}

#[test_only]
public fun mint_for_testing(amount: u64, ctx: &mut TxContext): myso::coin::Coin<USDT> {
    myso::coin::mint_for_testing<USDT>(amount, ctx)
}
