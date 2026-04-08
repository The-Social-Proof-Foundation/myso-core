//! Quorum-based transport implementation.
//!
//! This module provides a custom `Transport` implementation that sends requests
//! to multiple RPC providers and requires quorum consensus before returning a response.
//!
//! When only a single provider is configured (with quorum=1), the transport
//! operates in a zero-overhead passthrough mode — no quorum checking, no health
//! recording, no background tasks, no lock contention.

use crate::config::MultiProviderConfig;
use crate::error::MultiProviderError;
use crate::health::{HealthStatus, ProviderHealth, ProviderHealthCheckResult};

use alloy_json_rpc::{Id, Request, RequestPacket, ResponsePacket};
use alloy_transport::{TransportError, TransportErrorKind, TransportFut};
use alloy_transport_http::{reqwest::Client, Http};

use futures::future::join_all;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::sync::Notify;
use tokio::time::interval;
use tower::Service;
use tracing::{debug, error, info, warn};
use url::Url;

struct ProviderTransport {
    transport: Http<Client>,
    url: String,
    health: ProviderHealth,
}

impl ProviderTransport {
    fn new(transport: Http<Client>, url: String) -> Self {
        Self {
            transport,
            url,
            health: ProviderHealth::new(),
        }
    }
}

struct QuorumTransportState {
    providers: Vec<ProviderTransport>,
    ranked_indices: Vec<usize>,
}

impl QuorumTransportState {
    fn new(providers: Vec<ProviderTransport>) -> Self {
        let ranked_indices: Vec<usize> = (0..providers.len()).collect();
        Self {
            providers,
            ranked_indices,
        }
    }

    fn rerank(&mut self) {
        self.ranked_indices.sort_by(|&a, &b| {
            let health_a = &self.providers[a].health;
            let health_b = &self.providers[b].health;

            match health_a.status.priority().cmp(&health_b.status.priority()) {
                std::cmp::Ordering::Equal => {
                    // Then by latency (lower is better)
                    let latency_a = health_a.latency.unwrap_or(Duration::MAX);
                    let latency_b = health_b.latency.unwrap_or(Duration::MAX);
                    latency_a.cmp(&latency_b)
                }
                other => other,
            }
        });

        debug!(
            "Reranked providers: {:?}",
            self.ranked_indices
                .iter()
                .map(|&i| (&self.providers[i].url, self.providers[i].health.status))
                .collect::<Vec<_>>()
        );
    }

    fn get_ranked_provider_indices(&self, skip: usize, count: usize) -> Vec<usize> {
        self.ranked_indices
            .iter()
            .skip(skip)
            .take(count)
            .filter(|&&i| self.providers[i].health.status.is_usable())
            .copied()
            .collect()
    }
}

/// Internal representation: single-provider passthrough vs multi-provider quorum.
#[derive(Clone)]
enum TransportMode {
    /// Single provider — direct passthrough with no quorum, health tracking, or locking overhead.
    Single {
        transport: Http<Client>,
        url: String,
        timeout: Duration,
    },
    /// Multiple providers with quorum consensus and health ranking.
    Quorum {
        state: Arc<RwLock<QuorumTransportState>>,
    },
}

#[derive(Clone)]
pub struct QuorumTransport {
    mode: TransportMode,
    config: MultiProviderConfig,
    shutdown: Arc<Notify>,
}

impl QuorumTransport {
    pub fn new(config: MultiProviderConfig) -> Result<Self, MultiProviderError> {
        config.validate()?;

        let client = Client::new();
        let mut providers = Vec::with_capacity(config.rpc_urls.len());

        for url_str in &config.rpc_urls {
            let url: Url = url_str.parse().map_err(|e: url::ParseError| {
                MultiProviderError::InitializationFailed {
                    url: url_str.clone(),
                    reason: e.to_string(),
                }
            })?;

            let transport = Http::with_client(client.clone(), url);
            providers.push(ProviderTransport::new(transport, url_str.clone()));
        }

        let mode = if providers.len() == 1 && config.quorum == 1 {
            // Single-provider fast path: no quorum state, no locks, no health tracking.
            let single = providers.pop().expect("checked len == 1");
            TransportMode::Single {
                transport: single.transport,
                url: single.url,
                timeout: config.request_timeout,
            }
        } else {
            TransportMode::Quorum {
                state: Arc::new(RwLock::new(QuorumTransportState::new(providers))),
            }
        };

        let shutdown = Arc::new(Notify::new());

        Ok(Self {
            mode,
            config,
            shutdown,
        })
    }

    pub fn from_urls(urls: Vec<String>, quorum: usize) -> Result<Self, MultiProviderError> {
        let config = MultiProviderConfig::new(urls, quorum);
        Self::new(config)
    }

    /// Run a one-shot health check on all providers.
    /// No-op in single-provider mode.
    pub async fn run_health_check(&self) {
        match &self.mode {
            TransportMode::Single { .. } => { /* no-op: nothing to rank or compare */ }
            TransportMode::Quorum { state } => {
                run_health_check_internal(state, &self.config).await;
            }
        }
    }

    /// Spawn a background task that periodically health-checks all providers.
    /// No-op in single-provider mode (no task is spawned).
    pub fn start_health_check_task(&self) {
        let state = match &self.mode {
            TransportMode::Single { .. } => return,
            TransportMode::Quorum { state } => Arc::clone(state),
        };
        let config = self.config.clone();
        let shutdown = Arc::clone(&self.shutdown);

        tokio::spawn(async move {
            let mut health_interval = interval(config.health_check_interval);

            loop {
                tokio::select! {
                    _ = health_interval.tick() => {
                        run_health_check_internal(&state, &config).await;
                    }
                    _ = shutdown.notified() => {
                        info!("Health check task shutting down");
                        break;
                    }
                }
            }
        });
    }

    pub fn shutdown(&self) {
        self.shutdown.notify_waiters();
    }

    pub fn get_provider_health(&self) -> Vec<(String, HealthStatus)> {
        match &self.mode {
            TransportMode::Single { url, .. } => {
                vec![(url.clone(), HealthStatus::Healthy)]
            }
            TransportMode::Quorum { state } => {
                let s = state.read();
                s.providers
                    .iter()
                    .map(|p| (p.url.clone(), p.health.status))
                    .collect()
            }
        }
    }

    pub fn healthy_provider_count(&self) -> usize {
        match &self.mode {
            TransportMode::Single { .. } => 1,
            TransportMode::Quorum { state } => {
                let s = state.read();
                s.providers
                    .iter()
                    .filter(|p| p.health.status == HealthStatus::Healthy)
                    .count()
            }
        }
    }

    pub fn provider_count(&self) -> usize {
        match &self.mode {
            TransportMode::Single { .. } => 1,
            TransportMode::Quorum { state } => state.read().providers.len(),
        }
    }

    pub fn quorum(&self) -> usize {
        self.config.quorum
    }

    pub fn config(&self) -> &MultiProviderConfig {
        &self.config
    }

    /// Returns `true` if this transport is operating in single-provider
    /// passthrough mode (no quorum overhead).
    pub fn is_single_provider(&self) -> bool {
        matches!(self.mode, TransportMode::Single { .. })
    }
}

impl Service<RequestPacket> for QuorumTransport {
    type Response = ResponsePacket;
    type Error = TransportError;
    type Future = TransportFut<'static>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // We're always ready to accept requests
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: RequestPacket) -> Self::Future {
        match &self.mode {
            TransportMode::Single {
                transport, timeout, ..
            } => {
                // Fast path: direct passthrough, no quorum/health overhead.
                let mut transport = transport.clone();
                let timeout = *timeout;
                Box::pin(async move {
                    tokio::time::timeout(timeout, transport.call(req))
                        .await
                        .map_err(|_| {
                            TransportErrorKind::custom(MultiProviderError::QuorumNotReached {
                                required: 1,
                                received: 0,
                                total: 1,
                            })
                        })?
                })
            }
            TransportMode::Quorum { state } => {
                let state = Arc::clone(state);
                let quorum = self.config.quorum;
                let timeout = self.config.request_timeout;
                Box::pin(
                    async move { execute_with_quorum(state, req, quorum, timeout).await },
                )
            }
        }
    }
}

async fn execute_with_quorum(
    state: Arc<RwLock<QuorumTransportState>>,
    req: RequestPacket,
    quorum: usize,
    timeout: Duration,
) -> Result<ResponsePacket, TransportError> {
    let mut all_responses: Vec<(usize, Result<ResponsePacket, String>)> = Vec::new();
    let mut providers_tried = 0;

    loop {
        let provider_indices: Vec<usize> = {
            let s = state.read();
            s.get_ranked_provider_indices(providers_tried, quorum)
        };

        if provider_indices.is_empty() {
            break;
        }

        let transports: Vec<(usize, Http<Client>, String)> = {
            let s = state.read();
            provider_indices
                .iter()
                .map(|&i| {
                    let p = &s.providers[i];
                    (i, p.transport.clone(), p.url.clone())
                })
                .collect()
        };

        let futures: Vec<_> = transports
            .into_iter()
            .map(|(idx, mut transport, url)| {
                let req = req.clone();
                async move {
                    debug!("Sending request to provider {}", url);
                    let result = tokio::time::timeout(timeout, transport.call(req))
                        .await
                        .map_err(|_| "timeout".to_string())
                        .and_then(|r| r.map_err(|e| e.to_string()));

                    (idx, result)
                }
            })
            .collect();

        let batch_responses = join_all(futures).await;
        let batch_len = batch_responses.len();

        {
            let mut s = state.write();
            for (idx, result) in &batch_responses {
                s.providers[*idx].health.record_request(result.is_ok());
            }
        }

        all_responses.extend(batch_responses);
        providers_tried += batch_len;

        if let Some(response) = check_quorum_json(&all_responses, quorum) {
            debug!(
                "Quorum reached with {} providers after trying {}",
                quorum, providers_tried
            );
            return Ok(response);
        }

        debug!(
            "Quorum not yet reached, tried {} providers",
            providers_tried
        );
    }

    let successful = all_responses.iter().filter(|(_, r)| r.is_ok()).count();

    error!(
        "Quorum not reached: needed {} matching responses, got {} successful out of {} total",
        quorum,
        successful,
        all_responses.len()
    );

    Err(TransportErrorKind::custom(
        MultiProviderError::QuorumNotReached {
            required: quorum,
            received: successful,
            total: all_responses.len(),
        },
    ))
}

fn check_quorum_json(
    responses: &[(usize, Result<ResponsePacket, String>)],
    required: usize,
) -> Option<ResponsePacket> {
    let mut groups: HashMap<String, (ResponsePacket, usize)> = HashMap::new();

    for (_, result) in responses {
        if let Ok(response) = result {
            let key = match response {
                ResponsePacket::Single(ref resp) => resp
                    .payload
                    .as_success()
                    .map(|v| v.get().to_string())
                    .unwrap_or_else(|| {
                        resp.payload
                            .as_error()
                            .map(|e| format!("error:{}", e.code))
                            .unwrap_or_default()
                    }),
                ResponsePacket::Batch(ref resps) => resps
                    .iter()
                    .map(|r| {
                        r.payload
                            .as_success()
                            .map(|v| v.get().to_string())
                            .unwrap_or_default()
                    })
                    .collect::<Vec<_>>()
                    .join("|"),
            };

            if !key.is_empty() {
                groups
                    .entry(key)
                    .and_modify(|(_, count)| *count += 1)
                    .or_insert_with(|| (response.clone(), 1));
            }
        }
    }

    for (_, (response, count)) in groups {
        if count >= required {
            return Some(response);
        }
    }

    None
}

async fn run_health_check_internal(
    state: &Arc<RwLock<QuorumTransportState>>,
    config: &MultiProviderConfig,
) {
    info!("Running health check on {} providers", {
        state.read().providers.len()
    });

    // Clone transports for health check
    let providers: Vec<(usize, Http<Client>, String)> = {
        let s = state.read();
        s.providers
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.transport.clone(), p.url.clone()))
            .collect()
    };

    let futures: Vec<_> = providers
        .into_iter()
        .map(|(idx, mut transport, url)| async move {
            let start = Instant::now();

            let request: Request<()> = Request::new("eth_blockNumber", Id::Number(1), ());
            let serialized = request.try_into().expect("valid request");
            let packet = RequestPacket::Single(serialized);

            let result = tokio::time::timeout(config.request_timeout, transport.call(packet)).await;
            let latency = start.elapsed();

            let check_result = match result {
                Ok(Ok(ResponsePacket::Single(response))) => {
                    let block_number = response.payload.as_success().and_then(|v| {
                        let s = v.get();
                        // Parse hex string like "0x1234"
                        let s = s.trim_matches('"');
                        u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()
                    });

                    ProviderHealthCheckResult {
                        success: block_number.is_some(),
                        block_number,
                        latency,
                        error: if block_number.is_none() {
                            Some("failed to parse block number".to_string())
                        } else {
                            None
                        },
                    }
                }
                Ok(Ok(_)) => ProviderHealthCheckResult {
                    success: false,
                    block_number: None,
                    latency,
                    error: Some("unexpected batch response".to_string()),
                },
                Ok(Err(e)) => ProviderHealthCheckResult {
                    success: false,
                    block_number: None,
                    latency,
                    error: Some(e.to_string()),
                },
                Err(_) => ProviderHealthCheckResult {
                    success: false,
                    block_number: None,
                    latency,
                    error: Some("timeout".to_string()),
                },
            };

            debug!(
                "Health check for {}: success={}, block={:?}, latency={:?}",
                url, check_result.success, check_result.block_number, check_result.latency
            );

            (idx, check_result)
        })
        .collect();

    let results = join_all(futures).await;

    let max_block = results
        .iter()
        .filter_map(|(_, r)| r.block_number)
        .max()
        .unwrap_or(0);

    {
        let mut s = state.write();
        for (idx, result) in results {
            let provider = &mut s.providers[idx];

            if result.success {
                provider.health.update_success(
                    result.block_number.unwrap_or(0),
                    result.latency,
                    max_block,
                    config.max_block_delta,
                    config.max_healthy_latency,
                );
                info!(
                    "Provider {} is {:?} (block: {}, latency: {:?})",
                    provider.url,
                    provider.health.status,
                    result.block_number.unwrap_or(0),
                    result.latency
                );
            } else {
                provider.health.update_failure();
                warn!(
                    "Provider {} health check failed: {:?}",
                    provider.url, result.error
                );
            }
        }

        s.rerank();
    }
}
