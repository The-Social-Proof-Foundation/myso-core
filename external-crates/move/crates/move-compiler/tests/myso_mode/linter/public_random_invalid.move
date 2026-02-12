// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[allow(lint(public_entry))]
module a::test {
    use myso::random::{Random, RandomGenerator};

    public fun not_allowed1(_x: u64, _r: &Random) {}

    public fun not_allowed2(_rg: &RandomGenerator, _x: u64) {}

    public fun not_allowed3(_r: &Random, _rg: &RandomGenerator, _x: u64) {}

    public entry fun not_allowed4(_x: u64, _r: &Random, _y: u64) {}
}

module myso::object {
    struct UID has store {
        id: address,
    }
}

module myso::random {
    use myso::object::UID;

    struct Random has key { id: UID }
    struct RandomGenerator has drop {}
}
