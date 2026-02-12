// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests the invalid creation and deletion of a parent object

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    public struct S has key, store {
        id: myso::object::UID,
    }

    public entry fun t(ctx: &mut TxContext) {
        let mut parent = myso::object::new(ctx);
        let child = S { id: myso::object::new(ctx) };
        myso::dynamic_object_field::add(&mut parent, 0u64, child);
        myso::object::delete(parent);
    }
}

//# run test::m::t --sender A
