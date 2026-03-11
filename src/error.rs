//! Error types for TAPIR-RS

use thiserror::Error;

/// Result type alias for TAPIR operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for TAPIR
#[derive(Error, Debug)]
pub enum Error {
    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Network transport errors
    #[error("Transport error: {0}")]
    Transport(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Replication errors
    #[error("Replication error: {0}")]
    Replication(String),

    /// Storage errors
    #[error("Storage error: {0}")]
    Storage(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Timeout errors
    #[error("Timeout")]
    Timeout,

    /// Invalid state errors
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Not found errors
    #[error("Not found: {0}")]
    NotFound(String),
}
