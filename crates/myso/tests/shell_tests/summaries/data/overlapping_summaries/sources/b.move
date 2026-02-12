// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module overlapping_summaries::b;

#[allow(unused)]
public struct Y {
    x: child_pkg::a::X,
    y: other_child::a::X,
}

public fun g() { }
