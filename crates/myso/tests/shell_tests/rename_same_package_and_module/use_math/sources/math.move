// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module use_math::both;

public fun foo() {
  math_a::math::a();
  math_b::math::a();
}
