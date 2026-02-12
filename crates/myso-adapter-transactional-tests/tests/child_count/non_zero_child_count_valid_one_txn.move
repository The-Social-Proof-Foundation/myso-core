// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests valid transfers of an object that has children
// all transfers done in a single transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use myso::dynamic_object_field as ofield;

    public struct S has key, store {
        id: myso::object::UID,
    }

    public struct R has key, store {
        id: myso::object::UID,
        s: S,
    }

    public entry fun share(ctx: &mut TxContext) {
        let mut id = myso::object::new(ctx);
        let child = S { id: myso::object::new(ctx) };
        ofield::add(&mut id, 0u64, child);
        myso::transfer::public_share_object(S { id })
    }

}

//
// Test share object allows non-zero child count
//

//# run test::m::share --sender A

//# view-object 2,1
