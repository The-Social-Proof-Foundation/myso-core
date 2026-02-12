// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests invalid wrapping of a parent object with children, in a single transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use myso::dynamic_object_field as ofield;

    public struct S has key, store {
        id: myso::object::UID,
    }

    public struct R has key {
        id: myso::object::UID,
        s: S,
    }

    public entry fun test_wrap(ctx: &mut TxContext) {
        let mut id = myso::object::new(ctx);
        let child = S { id: myso::object::new(ctx) };
        ofield::add(&mut id, 0u64, child);
        let parent = S { id };
        let r = R { id: myso::object::new(ctx), s: parent };
        myso::transfer::transfer(r, tx_context::sender(ctx))
    }
}

//# run test::m::test_wrap --sender A
