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
    pub checkpoint_ingest_total: IntCounterVec,
    pub checkpoint_lag: IntGauge,
    pub source_fetch_errors: IntCounter,
    pub event_provider_sync_total: IntCounterVec,
    pub scheduled_events_active: IntGauge,
    pub event_match_total: IntCounterVec,
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

        let source_fetch_errors = IntCounter::new(
            "source_fetch_errors_total",
            "Trusted-source direct HTTP fetch failures",
        )
        .expect("source_fetch_errors metric");
        registry
            .register(Box::new(source_fetch_errors.clone()))
            .expect("register source_fetch_errors");

        let event_provider_sync_total = IntCounterVec::new(
            Opts::new("event_provider_sync_total", "Event provider sync outcomes"),
            &["provider", "status"],
        )
        .expect("event_provider_sync_total metric");
        registry
            .register(Box::new(event_provider_sync_total.clone()))
            .expect("register event_provider_sync_total");

        let scheduled_events_active = IntGauge::new(
            "scheduled_events_active",
            "Active scheduled events in registry",
        )
        .expect("scheduled_events_active metric");
        registry
            .register(Box::new(scheduled_events_active.clone()))
            .expect("register scheduled_events_active");

        let event_match_total = IntCounterVec::new(
            Opts::new("event_match_total", "Scheduled event matches at review time"),
            &["category"],
        )
        .expect("event_match_total metric");
        registry
            .register(Box::new(event_match_total.clone()))
            .expect("register event_match_total");

        let uptime = myso_indexer_alt_metrics::uptime(env!("CARGO_PKG_VERSION"))
            .expect("uptime metric");
        registry.register(uptime).expect("register uptime");

        Self {
            registry,
            reviews_total,
            resolver_latency_seconds,
            chain_tx_total,
            queue_depth,
            checkpoint_ingest_total,
            checkpoint_lag,
            source_fetch_errors,
            event_provider_sync_total,
            scheduled_events_active,
            event_match_total,
        }
    }
}
