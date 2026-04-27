// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! JSON helpers so large on-chain integer fields stay exact for JavaScript clients.

pub mod json_string_i64 {
    use serde::Serializer;

    pub fn serialize<S>(v: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&v.to_string())
    }
}
