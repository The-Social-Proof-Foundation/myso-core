// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Prometheus metrics registry — extend with myso-indexer-alt-metrics in production.

pub fn registry_prefix() -> &'static str {
    "discovery"
}
