// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Delivers task market events from `automation_event_outbox` to the automation engine.
//!
//! The indexer queues an event in the same transaction that applies it, so a crash can never lose
//! one. This loop drains the table in order, attaches the counterparty's current reputation, posts
//! each event to `POST {AUTOMATION_EVENTS_URL}/internal/automation/events` and stamps it sent. The
//! engine deduplicates on the event's `deduplication_key`, which is what makes redelivery after a
//! crash or a timed-out response harmless. It also queues the time-driven events (an offer past its
//! deadline, a delivery nobody settled) that no chain event announces.
//!
//! It runs only when `AUTOMATION_EVENTS_URL` and a secret are configured, so a deployment without
//! the engine is unaffected, and a down engine only delays delivery: indexing never waits on it.

use std::time::{Duration, Instant};

use anyhow::Result;
use myso_indexer_alt_social_schema::models::{platform_event, retry_delay_secs, AUTOMATION_FAMILY_TASK};
use reqwest::StatusCode;

use crate::reader::Reader;

const DEFAULT_POLL_SECS: u64 = 2;
const DEFAULT_BATCH: i64 = 50;
/// How long a leased row stays invisible to other forwarders if this one dies mid-delivery.
const LEASE_SECS: i64 = 60;
const DUE_SCAN_EVERY: Duration = Duration::from_secs(30);
const DUE_BATCH: i64 = 200;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
/// After this many failed deliveries a row is retried once a day instead of backing off further.
const SLOW_RETRY_AFTER_ATTEMPTS: i32 = 50;
const SLOW_RETRY_SECS: i64 = 86_400;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwarderConfig {
    pub events_url: String,
    pub secret: String,
    pub poll_interval: Duration,
    pub batch: i64,
}

impl ForwarderConfig {
    /// `None` unless both an engine URL and a secret are set. Blank values count as unset.
    pub fn from_lookup(lookup: &dyn Fn(&str) -> Option<String>) -> Option<Self> {
        let get = |key: &str| lookup(key).map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
        let events_url = get("AUTOMATION_EVENTS_URL")?;
        let secret = get("AUTOMATION_EVENTS_SECRET").or_else(|| get("INTERNAL_SYNC_SECRET"))?;
        let poll_secs = get("AUTOMATION_FORWARDER_POLL_SECS")
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(DEFAULT_POLL_SECS);
        let batch = get("AUTOMATION_FORWARDER_BATCH")
            .and_then(|v| v.parse::<i64>().ok())
            .filter(|v| (1..=500).contains(v))
            .unwrap_or(DEFAULT_BATCH);
        Some(Self {
            events_url,
            secret,
            poll_interval: Duration::from_secs(poll_secs),
            batch,
        })
    }

    pub fn from_env() -> Option<Self> {
        Self::from_lookup(&|key| std::env::var(key).ok())
    }

    pub fn endpoint(&self) -> String {
        format!("{}/internal/automation/events", self.events_url.trim_end_matches('/'))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Delivery {
    Delivered,
    Retry(String),
}

/// 200 means the engine already had the event, 202 that it queued it. Anything else is retried.
pub fn classify(status: StatusCode) -> Delivery {
    match status {
        StatusCode::OK | StatusCode::ACCEPTED => Delivery::Delivered,
        other => Delivery::Retry(format!("automation engine answered {other}")),
    }
}

pub fn retry_in_secs(attempts: i32) -> i64 {
    if attempts >= SLOW_RETRY_AFTER_ATTEMPTS {
        SLOW_RETRY_SECS
    } else {
        retry_delay_secs(attempts)
    }
}

/// Lease, enrich and deliver one batch. Returns how many rows were leased.
async fn deliver_batch(
    reader: &Reader,
    client: &reqwest::Client,
    config: &ForwarderConfig,
) -> Result<usize> {
    let rows = reader.claim_automation_events(config.batch, LEASE_SECS).await?;
    let leased = rows.len();
    let endpoint = config.endpoint();

    for row in rows {
        let outcome = async {
            let mut payload = row.payload.clone();
            if row.event_family == AUTOMATION_FAMILY_TASK {
                reader.automation_enrichment(&payload).await?.apply(&mut payload);
            }
            let event = platform_event(&row.identity(), &payload);
            let response = client
                .post(&endpoint)
                .header("x-internal-sync-secret", &config.secret)
                .json(&event)
                .send()
                .await?;
            anyhow::Ok(classify(response.status()))
        }
        .await;

        let delivery = outcome.unwrap_or_else(|error| Delivery::Retry(error.to_string()));
        match delivery {
            Delivery::Delivered => reader.mark_automation_event_sent(row.id).await?,
            Delivery::Retry(reason) => {
                tracing::warn!(id = row.id, attempts = row.attempts, %reason, "automation event not delivered");
                reader
                    .mark_automation_event_failed(row.id, &reason, retry_in_secs(row.attempts))
                    .await?;
            }
        }
    }
    Ok(leased)
}

/// Runs until the process exits.
pub async fn run(reader: Reader, config: ForwarderConfig) {
    let client = match reqwest::Client::builder().timeout(REQUEST_TIMEOUT).build() {
        Ok(client) => client,
        Err(error) => {
            tracing::error!(%error, "automation forwarder could not build its HTTP client; not running");
            return;
        }
    };
    tracing::info!(endpoint = %config.endpoint(), "automation event forwarder started");

    let mut last_scan: Option<Instant> = None;
    loop {
        if last_scan.is_none_or(|at| at.elapsed() >= DUE_SCAN_EVERY) {
            last_scan = Some(Instant::now());
            let now_ms = chrono::Utc::now().timestamp_millis();
            match reader.enqueue_due_automation_events(now_ms, DUE_BATCH).await {
                Ok(0) => {}
                Ok(queued) => tracing::info!(queued, "queued time-driven task events"),
                Err(error) => tracing::warn!(%error, "scanning for due task events failed"),
            }
        }
        match deliver_batch(&reader, &client, &config).await {
            // A full batch means more is waiting.
            Ok(leased) if leased as i64 >= config.batch => continue,
            Ok(_) => {}
            Err(error) => tracing::warn!(%error, "automation event delivery pass failed"),
        }
        tokio::time::sleep(config.poll_interval).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn lookup(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        move |key: &str| map.get(key).cloned()
    }

    #[test]
    fn the_forwarder_is_off_unless_an_engine_and_a_secret_are_configured() {
        assert_eq!(ForwarderConfig::from_lookup(&lookup(&[])), None);
        assert_eq!(
            ForwarderConfig::from_lookup(&lookup(&[("AUTOMATION_EVENTS_URL", "http://engine:8010")])),
            None,
            "a URL without a secret would post unauthenticated events"
        );
        assert_eq!(
            ForwarderConfig::from_lookup(&lookup(&[("INTERNAL_SYNC_SECRET", "s3cret")])),
            None
        );
        assert_eq!(
            ForwarderConfig::from_lookup(&lookup(&[
                ("AUTOMATION_EVENTS_URL", "  "),
                ("INTERNAL_SYNC_SECRET", "s3cret"),
            ])),
            None,
            "blank counts as unset"
        );
    }

    #[test]
    fn the_events_secret_falls_back_to_the_shared_secret() {
        let shared = ForwarderConfig::from_lookup(&lookup(&[
            ("AUTOMATION_EVENTS_URL", "http://engine:8010"),
            ("INTERNAL_SYNC_SECRET", "shared"),
        ]))
        .unwrap();
        assert_eq!(shared.secret, "shared");

        let dedicated = ForwarderConfig::from_lookup(&lookup(&[
            ("AUTOMATION_EVENTS_URL", "http://engine:8010"),
            ("INTERNAL_SYNC_SECRET", "shared"),
            ("AUTOMATION_EVENTS_SECRET", "dedicated"),
        ]))
        .unwrap();
        assert_eq!(dedicated.secret, "dedicated");
    }

    #[test]
    fn tuning_values_are_bounded_and_default_when_invalid() {
        let base = [
            ("AUTOMATION_EVENTS_URL", "http://engine:8010/"),
            ("AUTOMATION_EVENTS_SECRET", "s"),
        ];
        let default = ForwarderConfig::from_lookup(&lookup(&base)).unwrap();
        assert_eq!(default.poll_interval, Duration::from_secs(2));
        assert_eq!(default.batch, 50);

        let mut tuned = base.to_vec();
        tuned.extend([("AUTOMATION_FORWARDER_POLL_SECS", "7"), ("AUTOMATION_FORWARDER_BATCH", "200")]);
        let tuned = ForwarderConfig::from_lookup(&lookup(&tuned)).unwrap();
        assert_eq!(tuned.poll_interval, Duration::from_secs(7));
        assert_eq!(tuned.batch, 200);

        let mut bad = base.to_vec();
        bad.extend([("AUTOMATION_FORWARDER_POLL_SECS", "0"), ("AUTOMATION_FORWARDER_BATCH", "100000")]);
        let bad = ForwarderConfig::from_lookup(&lookup(&bad)).unwrap();
        assert_eq!(bad.poll_interval, Duration::from_secs(2));
        assert_eq!(bad.batch, 50);
    }

    #[test]
    fn the_endpoint_tolerates_a_trailing_slash() {
        let config = ForwarderConfig::from_lookup(&lookup(&[
            ("AUTOMATION_EVENTS_URL", "http://engine:8010/"),
            ("AUTOMATION_EVENTS_SECRET", "s"),
        ]))
        .unwrap();
        assert_eq!(config.endpoint(), "http://engine:8010/internal/automation/events");
    }

    #[test]
    fn only_an_accepted_or_already_known_event_counts_as_delivered() {
        assert_eq!(classify(StatusCode::ACCEPTED), Delivery::Delivered);
        assert_eq!(classify(StatusCode::OK), Delivery::Delivered);
        for status in [
            StatusCode::UNAUTHORIZED,
            StatusCode::BAD_REQUEST,
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::BAD_GATEWAY,
            StatusCode::NO_CONTENT,
        ] {
            assert!(matches!(classify(status), Delivery::Retry(_)), "{status}");
        }
    }

    #[test]
    fn a_stuck_event_backs_off_then_settles_to_a_daily_retry() {
        assert_eq!(retry_in_secs(1), 5);
        assert_eq!(retry_in_secs(8), 600);
        assert_eq!(retry_in_secs(49), 600);
        assert_eq!(retry_in_secs(50), 86_400);
        assert_eq!(retry_in_secs(1_000), 86_400);
    }
}
