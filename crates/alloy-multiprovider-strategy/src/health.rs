use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HealthStatus {
    Healthy = 0,
    HighLatency = 1,
    Behind = 2,
    Unreachable = 3,
    Unknown = 4,
}

impl HealthStatus {
    pub fn is_usable(&self) -> bool {
        !matches!(self, HealthStatus::Unreachable)
    }

    pub fn priority(&self) -> u8 {
        match self {
            HealthStatus::Healthy => 0,
            HealthStatus::HighLatency => 1,
            HealthStatus::Behind => 2,
            HealthStatus::Unknown => 3,
            HealthStatus::Unreachable => 4,
        }
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "healthy"),
            HealthStatus::HighLatency => write!(f, "high_latency"),
            HealthStatus::Behind => write!(f, "behind"),
            HealthStatus::Unreachable => write!(f, "unreachable"),
            HealthStatus::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProviderHealth {
    pub status: HealthStatus,
    pub block_number: Option<u64>,
    pub latency: Option<Duration>,
    pub last_check: Option<Instant>,
    pub consecutive_failures: u32,
    pub total_requests: u64,
    pub successful_requests: u64,
}

impl Default for ProviderHealth {
    fn default() -> Self {
        Self {
            status: HealthStatus::Unknown,
            block_number: None,
            latency: None,
            last_check: None,
            consecutive_failures: 0,
            total_requests: 0,
            successful_requests: 0,
        }
    }
}

impl ProviderHealth {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_success(
        &mut self,
        block_number: u64,
        latency: Duration,
        max_block: u64,
        max_block_delta: u64,
        max_healthy_latency: Duration,
    ) {
        self.block_number = Some(block_number);
        self.latency = Some(latency);
        self.last_check = Some(Instant::now());
        self.consecutive_failures = 0;

        let is_behind = max_block.saturating_sub(block_number) > max_block_delta;
        let is_high_latency = latency > max_healthy_latency;

        self.status = match (is_behind, is_high_latency) {
            (true, _) => HealthStatus::Behind,
            (false, true) => HealthStatus::HighLatency,
            (false, false) => HealthStatus::Healthy,
        };
    }

    pub fn update_failure(&mut self) {
        self.last_check = Some(Instant::now());
        self.consecutive_failures += 1;

        if self.consecutive_failures >= 3 {
            self.status = HealthStatus::Unreachable;
        }
    }

    pub fn record_request(&mut self, success: bool) {
        self.total_requests += 1;
        if success {
            self.successful_requests += 1;
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 100.0;
        }
        (self.successful_requests as f64 / self.total_requests as f64) * 100.0
    }
}

/// Result of a single provider health check
#[derive(Debug)]
pub struct ProviderHealthCheckResult {
    pub success: bool,
    pub block_number: Option<u64>,
    pub latency: Duration,
    pub error: Option<String>,
}
