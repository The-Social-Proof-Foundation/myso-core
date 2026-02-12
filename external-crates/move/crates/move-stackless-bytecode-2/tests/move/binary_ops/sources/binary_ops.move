// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module bin_ops::bin_ops {

    public fun add(a: u64, b: u64): u64 {
        a + b
    }

    public fun sub(a: u64, b: u64): u64 {
        a - b
    }

    public fun mul(a: u64, b: u64): u64 {
        a * b
    }

    public fun div(a: u64, b: u64): u64 {
        a / b
    }

    public fun less_than(a: u64, b: u64): bool {
        a < b
    }

    public fun greater_than(a: u64, b: u64): bool {
        a > b
    }

}