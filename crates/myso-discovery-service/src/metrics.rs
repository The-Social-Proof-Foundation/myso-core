// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Prometheus registry (`discovery` prefix).

use prometheus::{IntCounter, IntCounterVec, IntGaugeVec, Opts, Registry};

pub struct DiscoveryMetrics {
    pub registry: Registry,
    pub assets_upserted_total: IntCounter,
    pub embed_jobs_total: IntCounterVec,
    pub queue_depth: IntGaugeVec,
    pub source_poll_total: IntCounterVec,
    pub cache_hits_total: IntCounter,
    pub cache_misses_total: IntCounter,
    pub refresh_total: IntCounter,
}

impl DiscoveryMetrics {
    pub fn build() -> Self {
        let registry = Registry::new_custom(Some("discovery".to_string()), None)
            .expect("Failed to create Prometheus registry.");

        let assets_upserted_total = IntCounter::new(
            "assets_upserted_total",
            "Discovery assets upserted from source polls",
        )
        .expect("assets_upserted_total metric");
        registry
            .register(Box::new(assets_upserted_total.clone()))
            .expect("register assets_upserted_total");

        let embed_jobs_total = IntCounterVec::new(
            Opts::new("embed_jobs_total", "Embed jobs by terminal status"),
            &["status"],
        )
        .expect("embed_jobs_total metric");
        registry
            .register(Box::new(embed_jobs_total.clone()))
            .expect("register embed_jobs_total");

        let queue_depth = IntGaugeVec::new(
            Opts::new("queue_depth", "discovery_jobs queue depth by status"),
            &["status"],
        )
        .expect("queue_depth metric");
        registry
            .register(Box::new(queue_depth.clone()))
            .expect("register queue_depth");

        let source_poll_total = IntCounterVec::new(
            Opts::new("source_poll_total", "Source poll outcomes"),
            &["source_id", "result"],
        )
        .expect("source_poll_total metric");
        registry
            .register(Box::new(source_poll_total.clone()))
            .expect("register source_poll_total");

        let cache_hits_total = IntCounter::new("cache_hits_total", "Factual cache hits on /v1/*")
            .expect("cache_hits_total metric");
        registry
            .register(Box::new(cache_hits_total.clone()))
            .expect("register cache_hits_total");

        let cache_misses_total =
            IntCounter::new("cache_misses_total", "Factual cache misses on /v1/*")
                .expect("cache_misses_total metric");
        registry
            .register(Box::new(cache_misses_total.clone()))
            .expect("register cache_misses_total");

        let refresh_total =
            IntCounter::new("refresh_total", "Forced source refresh via /v1/refresh")
                .expect("refresh_total metric");
        registry
            .register(Box::new(refresh_total.clone()))
            .expect("register refresh_total");

        let uptime = myso_indexer_alt_metrics::uptime(env!("CARGO_PKG_VERSION"))
            .expect("uptime metric");
        registry.register(uptime).expect("register uptime");

        Self {
            registry,
            assets_upserted_total,
            embed_jobs_total,
            queue_depth,
            source_poll_total,
            cache_hits_total,
            cache_misses_total,
            refresh_total,
        }
    }
}
