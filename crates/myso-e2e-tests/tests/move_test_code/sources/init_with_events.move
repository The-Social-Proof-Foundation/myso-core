// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module move_test_code::init_with_event;

public struct Event has copy, drop {}

fun init(_ctx: &mut TxContext) {
    myso::event::emit(Event {});
}
