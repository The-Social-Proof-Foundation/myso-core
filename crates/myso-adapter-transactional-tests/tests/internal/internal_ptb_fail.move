// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

// Case: try to call internal in PTB, fail

//# init --addresses test=0x0 --accounts A

//# publish
module test::custom_type {
    public struct CustomType {}
}

//# programmable --sender A
//> 0: std::internal::permit<test::custom_type::CustomType>();
