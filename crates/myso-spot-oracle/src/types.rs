// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Primitives shared by the review (claim interpretation) and resolver
//! (deterministic execution) layers. Kept in one neutral module so neither
//! layer depends on the other purely to reuse an enum.

use serde::{Deserialize, Serialize};

/// Numeric/string comparison operator used both in extracted claims and in the
/// immutable `ResolverSpec`. Canonicalized to one of these forms before compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOp {
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,
}

/// Executor kind emitted by the Resolver Compiler. The scheduler + resolver engine
/// branch on this enum only — never on adapter identity or claim text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverKind {
    PriceThreshold,
    EventOccurrence,
    ReleasePublished,
    CustomHttp,
}
