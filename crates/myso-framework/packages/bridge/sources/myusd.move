// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Platform MyUSD type owned by the Bridge package so `MyUsdPeg` can hold `TreasuryCap<MYUSD>`.
module bridge::myusd;

public struct MYUSD has drop {}

const DECIMAL: u8 = 6;

public fun decimals(): u8 {
    DECIMAL
}

#[test_only]
public fun create_treasury_cap_for_testing(ctx: &mut TxContext): myso::coin::TreasuryCap<MYUSD> {
    myso::coin::create_treasury_cap_for_testing<MYUSD>(ctx)
}

#[test_only]
public fun mint_for_testing(amount: u64, ctx: &mut TxContext): myso::coin::Coin<MYUSD> {
    myso::coin::mint_for_testing<MYUSD>(amount, ctx)
}
