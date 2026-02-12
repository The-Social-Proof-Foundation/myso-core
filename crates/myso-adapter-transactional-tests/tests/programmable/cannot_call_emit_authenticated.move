// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

// tests cannot call event emitters with programmable transactions

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    public struct A has copy, drop, store {}
    public fun a(): A { A {} }
}

//# programmable
//> 0: test::m1::a();
//> myso::event::emit_authenticated<test::m1::A>(Result(0));

//# programmable
//> 0: test::m1::a();
// wrong type annotation did not matter in v1 of PTB execution
//> myso::event::emit_authenticated<bool>(Result(0));

//# programmable
//> 0: test::m1::a();
// function doesn't exist
//> myso::event::does_not_exist<test::m1::A>(Result(0));


