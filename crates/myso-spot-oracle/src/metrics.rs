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
    pub deadline_inference_total: IntCounterVec,
    pub deadline_rejection_total: IntCounterVec,
    pub claim_match_total: IntCounterVec,
    pub knowledge_sync_total: IntCounterVec,
    pub observation_ingest_total: IntCounterVec,
    pub dedup_linked_total: IntCounter,
    pub dedup_created_total: IntCounter,
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
            Opts::new(
                "chain_tx_total",
                "Chain transaction attempts by kind and status",
            ),
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
            Opts::new(
                "event_match_total",
                "Scheduled event matches at review time",
            ),
            &["category"],
        )
        .expect("event_match_total metric");
        registry
            .register(Box::new(event_match_total.clone()))
            .expect("register event_match_total");

        let deadline_inference_total = IntCounterVec::new(
            Opts::new(
                "deadline_inference_total",
                "Context-aware deadline inferences by source",
            ),
            &["source"],
        )
        .expect("deadline_inference_total metric");
        registry
            .register(Box::new(deadline_inference_total.clone()))
            .expect("register deadline_inference_total");

        let deadline_rejection_total = IntCounterVec::new(
            Opts::new(
                "deadline_rejection_total",
                "Review rejections from deadline validation",
            ),
            &["reason"],
        )
        .expect("deadline_rejection_total metric");
        registry
            .register(Box::new(deadline_rejection_total.clone()))
            .expect("register deadline_rejection_total");

        let claim_match_total = IntCounterVec::new(
            Opts::new(
                "claim_match_total",
                "Claim matcher outcomes by domain and tier",
            ),
            &["domain", "tier"],
        )
        .expect("claim_match_total metric");
        registry
            .register(Box::new(claim_match_total.clone()))
            .expect("register claim_match_total");

        let knowledge_sync_total = IntCounterVec::new(
            Opts::new(
                "knowledge_sync_total",
                "Knowledge provider sync by object type",
            ),
            &["provider", "object_type"],
        )
        .expect("knowledge_sync_total metric");
        registry
            .register(Box::new(knowledge_sync_total.clone()))
            .expect("register knowledge_sync_total");

        let observation_ingest_total = IntCounterVec::new(
            Opts::new("observation_ingest_total", "Metric observations ingested"),
            &["metric", "domain"],
        )
        .expect("observation_ingest_total metric");
        registry
            .register(Box::new(observation_ingest_total.clone()))
            .expect("register observation_ingest_total");

        let dedup_linked_total = IntCounter::new(
            "dedup_linked_total",
            "Posts linked to an existing market via dedup cascade",
        )
        .expect("dedup_linked_total metric");
        registry
            .register(Box::new(dedup_linked_total.clone()))
            .expect("register dedup_linked_total");

        let dedup_created_total = IntCounter::new(
            "dedup_created_total",
            "New markets created after dedup miss",
        )
        .expect("dedup_created_total metric");
        registry
            .register(Box::new(dedup_created_total.clone()))
            .expect("register dedup_created_total");

        let uptime =
            myso_indexer_alt_metrics::uptime(env!("CARGO_PKG_VERSION")).expect("uptime metric");
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
            deadline_inference_total,
            deadline_rejection_total,
            claim_match_total,
            knowledge_sync_total,
            observation_ingest_total,
            dedup_linked_total,
            dedup_created_total,
        }
    }
}
