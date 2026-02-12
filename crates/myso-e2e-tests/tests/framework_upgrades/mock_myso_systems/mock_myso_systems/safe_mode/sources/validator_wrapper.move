// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module myso_system::validator_wrapper {
    use myso::versioned::Versioned;

    public struct ValidatorWrapper has store {
        inner: Versioned
    }
}
