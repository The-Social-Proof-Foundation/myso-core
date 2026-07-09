// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Prometheus registry (`spot_oracle` prefix).

use prometheus::{IntCounter, IntCounterVec, IntGauge, IntGaugeVec, Opts, Registry};

pub struct OracleMetrics {
    pub registry: Registry,
    pub reviews_total: IntCounterVec,
    pub resolver_latency_seconds: IntCounter,
    pub chain_tx_total: IntCounterVec,
    pub queue_depth: IntGaugeVec,
    pub rss_wake_total: IntCounter,
    pub checkpoint_ingest_total: IntCounterVec,
    pub checkpoint_lag: IntGauge,
    pub posts_filtered_enable_spot: IntCounter,
    pub discovery_client_errors: IntCounter,
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

        let checkpoint_ingest_total = IntCounterVec::new(
            Opts::new("checkpoint_ingest_total", "Checkpoint ingest outcomes"),
            &["result"],
        )
        .expect("checkpoint_ingest_total metric");
        registry
            .register(Box::new(checkpoint_ingest_total.clone()))
            .expect("register checkpoint_ingest_total");

        let checkpoint_lag = IntGauge::new("checkpoint_lag", "Last processed checkpoint sequence")
            .expect("checkpoint_lag metric");
        registry
            .register(Box::new(checkpoint_lag.clone()))
            .expect("register checkpoint_lag");

        let posts_filtered_enable_spot = IntCounter::new(
            "posts_filtered_enable_spot",
            "Posts observed with enable_spot=false",
        )
        .expect("posts_filtered_enable_spot metric");
        registry
            .register(Box::new(posts_filtered_enable_spot.clone()))
            .expect("register posts_filtered_enable_spot");

        let discovery_client_errors = IntCounter::new(
            "discovery_client_errors_total",
            "Discovery client request failures",
        )
        .expect("discovery_client_errors metric");
        registry
            .register(Box::new(discovery_client_errors.clone()))
            .expect("register discovery_client_errors");

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
            checkpoint_ingest_total,
            checkpoint_lag,
            posts_filtered_enable_spot,
            discovery_client_errors,
        }
    }
}
