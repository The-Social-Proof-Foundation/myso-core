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

/// Serialises `Option<i64>` as a JSON string for JS integer safety, or `null` when `None`.
pub mod json_string_opt_i64 {
    use serde::Serializer;

    pub fn serialize<S>(v: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match v {
            Some(n) => serializer.serialize_str(&n.to_string()),
            None => serializer.serialize_none(),
        }
    }
}
