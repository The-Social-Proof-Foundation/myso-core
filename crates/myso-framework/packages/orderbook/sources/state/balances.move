// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// `Balances` represents the three assets that make up a pool: base, quote, and
/// myso. Whenever funds are moved, they are moved in the form of `Balances`.
module orderbook::balances;

use orderbook::math;

// === Structs ===
public struct Balances has copy, drop, store {
    base: u64,
    quote: u64,
    myso: u64,
}

// === Public-Package Functions ===
public(package) fun empty(): Balances {
    Balances { base: 0, quote: 0, myso: 0 }
}

public(package) fun new(base: u64, quote: u64, myso: u64): Balances {
    Balances { base: base, quote: quote, myso: myso }
}

public(package) fun reset(balances: &mut Balances): Balances {
    let old = *balances;
    balances.base = 0;
    balances.quote = 0;
    balances.myso = 0;

    old
}

public(package) fun add_balances(balances: &mut Balances, other: Balances) {
    balances.base = balances.base + other.base;
    balances.quote = balances.quote + other.quote;
    balances.myso = balances.myso + other.myso;
}

public(package) fun add_base(balances: &mut Balances, base: u64) {
    balances.base = balances.base + base;
}

public(package) fun add_quote(balances: &mut Balances, quote: u64) {
    balances.quote = balances.quote + quote;
}

public(package) fun add_myso(balances: &mut Balances, myso: u64) {
    balances.myso = balances.myso + myso;
}

public(package) fun base(balances: &Balances): u64 {
    balances.base
}

public(package) fun quote(balances: &Balances): u64 {
    balances.quote
}

public(package) fun myso(balances: &Balances): u64 {
    balances.myso
}

public(package) fun mul(balances: &mut Balances, factor: u64) {
    balances.base = math::mul(balances.base, factor);
    balances.quote = math::mul(balances.quote, factor);
    balances.myso = math::mul(balances.myso, factor);
}

public(package) fun non_zero_value(balances: &Balances): u64 {
    if (balances.base > 0) {
        balances.base
    } else if (balances.quote > 0) {
        balances.quote
    } else {
        balances.myso
    }
}
