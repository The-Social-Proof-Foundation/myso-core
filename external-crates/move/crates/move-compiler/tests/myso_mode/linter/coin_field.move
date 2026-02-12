// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[allow(unused_field)]
module a::test1 {
    use myso::coin::Coin;
    use myso::object::UID;

    struct S1 {}

    struct S2 has key, store {
        id: UID,
        c: Coin<S1>,
    }
}

#[allow(unused_field)]
module a::test2 {
    use myso::coin::Coin as Balance;
    use myso::object::UID;

    struct S1 {}

    // there should still be a warning as Balance is just an alias
    struct S2 has key, store {
        id: UID,
        c: Balance<S1>,
    }
}

#[allow(unused_field)]
module a::test3 {
    use myso::coin::TreasuryCap;
    use myso::object::UID;

    struct S1 {}

    // guards against an already fixed silly bug that incorrectly identified Coin by module name
    // rather than by module name AND struct name
    struct S2 has key, store {
        id: UID,
        cap: TreasuryCap<S1>,
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }
}

module myso::coin {
    use myso::object::UID;

    struct Coin<phantom T> has key, store {
        id: UID,
    }

    struct TreasuryCap<phantom T> has key, store {
        id: UID,
    }
}
