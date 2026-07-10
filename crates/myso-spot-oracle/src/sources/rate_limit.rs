// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Per-source rate limiting for external trusted-source fetches.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const MIN_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Debug)]
struct SourceGate {
    last_fetch: Option<Instant>,
}

#[derive(Debug, Default)]
pub struct RateLimiter {
    gates: Mutex<HashMap<String, SourceGate>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn wait_duration(&self, source_id: &str) -> Duration {
        let mut gates = self.gates.lock().expect("rate limiter lock");
        let gate = gates
            .entry(source_id.to_string())
            .or_insert_with(|| SourceGate { last_fetch: None });
        if let Some(last) = gate.last_fetch {
            let elapsed = last.elapsed();
            if elapsed < MIN_INTERVAL {
                return MIN_INTERVAL - elapsed;
            }
        }
        Duration::ZERO
    }

    pub fn record_fetch(&self, source_id: &str) {
        let mut gates = self.gates.lock().expect("rate limiter lock");
        gates
            .entry(source_id.to_string())
            .or_insert_with(|| SourceGate { last_fetch: None })
            .last_fetch = Some(Instant::now());
    }

    pub async fn throttle(&self, source_id: &str) {
        let wait = self.wait_duration(source_id);
        if !wait.is_zero() {
            tokio::time::sleep(wait).await;
        }
        self.record_fetch(source_id);
    }
}
