use thiserror::Error;

/// Comprehensive error handling for the DTaP core engine.
#[derive(Debug, Error)]
pub enum DTaPError {
    /// Network-related errors (timeouts, DNS failures, etc.).
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// Errors from the voucher provider API (non-2xx responses, invalid payloads).
    #[error("Provider API error: {0}")]
    ProviderError(String),

    /// Failed to verify the user's asset lock on the ledger.
    #[error("Ledger verification failed: {0}")]
    LedgerVerificationError(String),

    /// Failed to deserialize a response from an external service.
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Any other unexpected error.
    #[error("Unexpected error: {0}")]
    Other(String),
}

/// Convenience type for functions that return DTaPError.
pub type Result<T> = std::result::Result<T, DTaPError>;