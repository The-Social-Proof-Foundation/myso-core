use alloy_json_rpc::RpcError;
use alloy_transport::TransportErrorKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MultiProviderError {
    #[error("quorum not reached: needed {required} matching responses, got {received} out of {total} providers")]
    QuorumNotReached {
        required: usize,
        received: usize,
        total: usize,
    },

    #[error("transport error: {0}")]
    Transport(#[from] alloy_transport::TransportError),

    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("failed to initialize provider for {url}: {reason}")]
    InitializationFailed { url: String, reason: String },
}

impl MultiProviderError {
    pub fn into_rpc_error(self) -> RpcError<TransportErrorKind> {
        TransportErrorKind::custom(self)
    }
}

pub type Result<T> = std::result::Result<T, MultiProviderError>;
