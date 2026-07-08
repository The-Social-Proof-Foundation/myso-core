// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Prometheus registry (`spot_oracle` prefix). Counters/histograms are wired up
//! alongside the modules that emit them; the foundation exposes uptime + queue
//! depth so `/metrics` is non-empty from boot.

use prometheus::{IntCounter, IntCounterVec, IntGaugeVec, Opts, Registry};

pub struct OracleMetrics {
    pub registry: Registry,
    pub reviews_total: IntCounterVec,
    pub resolver_latency_seconds: IntCounter,
    pub chain_tx_total: IntCounterVec,
    pub queue_depth: IntGaugeVec,
    pub rss_wake_total: IntCounter,
}

impl OracleMetrics {
    pub fn build() -> Self {
        let registry = Registry::new_custom(Some("spot_oracle".to_string()), None)
            .expect("Failed to create Prometheus registry.");

        let reviews_total = IntCounterVec::new(
            Opts::new("reviews_total", "Claim reviews by decision"),
            &["decision", "reason"],
        )
        .expect("reviews_total metric");
        registry
            .register(Box::new(reviews_total.clone()))
            .expect("register reviews_total");

        let resolver_latency_seconds = IntCounter::new(
            "resolver_latency_seconds_total",
            "Accumulated resolver wall-clock seconds",
        )
        .expect("resolver_latency metric");
        registry
            .register(Box::new(resolver_latency_seconds.clone()))
            .expect("register resolver_latency");

        let chain_tx_total = IntCounterVec::new(
            Opts::new("chain_tx_total", "Chain transaction attempts by kind and status"),
            &["kind", "status"],
        )
        .expect("chain_tx_total metric");
        registry
            .register(Box::new(chain_tx_total.clone()))
            .expect("register chain_tx_total");

        let queue_depth = IntGaugeVec::new(
            Opts::new("queue_depth", "spot_jobs queue depth by status"),
            &["status"],
        )
        .expect("queue_depth metric");
        registry
            .register(Box::new(queue_depth.clone()))
            .expect("register queue_depth");

        let rss_wake_total = IntCounter::new(
            "rss_wake_total",
            "RSS watcher wake events that enqueued resolver jobs",
        )
        .expect("rss_wake_total metric");
        registry
            .register(Box::new(rss_wake_total.clone()))
            .expect("register rss_wake_total");

        let uptime = myso_indexer_alt_metrics::uptime(env!("CARGO_PKG_VERSION"))
            .expect("uptime metric");
        registry.register(uptime).expect("register uptime");

        Self {
            registry,
            reviews_total,
            resolver_latency_seconds,
            chain_tx_total,
            queue_depth,
            rss_wake_total,
        }
    }
}
