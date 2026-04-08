use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MultiProviderConfig {
    pub rpc_urls: Vec<String>,
    pub quorum: usize,
    pub health_check_interval: Duration,
    pub max_block_delta: u64,
    pub max_healthy_latency: Duration,
    pub request_timeout: Duration,
    pub start_health_check_on_init: bool,
}

impl Default for MultiProviderConfig {
    fn default() -> Self {
        Self {
            rpc_urls: Vec::new(),
            quorum: 1,
            health_check_interval: Duration::from_secs(5 * 60), // 5 minutes
            max_block_delta: 5,
            max_healthy_latency: Duration::from_secs(5),
            request_timeout: Duration::from_secs(30),
            start_health_check_on_init: true,
        }
    }
}

impl MultiProviderConfig {
    pub fn new(rpc_urls: Vec<String>, quorum: usize) -> Self {
        Self {
            rpc_urls,
            quorum,
            ..Default::default()
        }
    }

    pub fn with_health_check_interval(mut self, interval: Duration) -> Self {
        self.health_check_interval = interval;
        self
    }

    pub fn with_max_block_delta(mut self, delta: u64) -> Self {
        self.max_block_delta = delta;
        self
    }

    pub fn with_max_healthy_latency(mut self, latency: Duration) -> Self {
        self.max_healthy_latency = latency;
        self
    }

    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    pub fn with_start_health_check_on_init(mut self, start: bool) -> Self {
        self.start_health_check_on_init = start;
        self
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        if self.rpc_urls.is_empty() {
            return Err(crate::error::MultiProviderError::InvalidConfig(
                "at least one RPC URL is required".into(),
            ));
        }

        if self.quorum == 0 {
            return Err(crate::error::MultiProviderError::InvalidConfig(
                "quorum must be at least 1".into(),
            ));
        }

        if self.quorum > self.rpc_urls.len() {
            return Err(crate::error::MultiProviderError::InvalidConfig(format!(
                "quorum ({}) cannot exceed number of providers ({})",
                self.quorum,
                self.rpc_urls.len()
            )));
        }

        Ok(())
    }
}
