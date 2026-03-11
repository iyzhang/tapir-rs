//! # TAPIR-RS
//!
//! A Rust implementation of TAPIR (Transaction Application Protocol for Inconsistent Replication).
//!
//! TAPIR is a protocol for linearizable distributed transactions built using replication with
//! no consistency guarantees. By enforcing consistency only at the transaction layer, TAPIR
//! eliminates coordination at the replication layer.
//!
//! ## Modules
//!
//! - `transport`: Network transport layer (UDP, TCP, simulation)
//! - `replication`: Replication protocols (IR, VR)
//! - `store`: Storage layer implementations
//! - `config`: Configuration management
//! - `error`: Error types and handling

pub mod config;
pub mod error;
pub mod replication;
pub mod store;
pub mod transport;

// Re-export commonly used types
pub use config::Configuration;
pub use error::{Error, Result};
