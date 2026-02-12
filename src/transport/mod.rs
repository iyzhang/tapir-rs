//! Network transport layer for TAPIR
//!
//! This module provides abstractions for network communication between
//! nodes in the distributed system. It supports multiple transport
//! implementations including UDP, TCP, and simulation-based transports.

use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;

use crate::config::Configuration;
use crate::error::Result;

pub mod address;
pub mod message;
pub mod timeout;

pub use address::TransportAddress;
pub use message::Message;
pub use timeout::Timeout;

/// Receiver trait for handling incoming messages
#[async_trait]
pub trait TransportReceiver: Send + Sync {
    /// Called when a message is received
    async fn receive_message(
        &mut self,
        remote: &TransportAddress,
        msg_type: &str,
        data: Bytes,
    ) -> Result<()>;

    /// Get the address this receiver is bound to
    fn get_address(&self) -> Option<&TransportAddress>;

    /// Set the address for this receiver
    fn set_address(&mut self, addr: TransportAddress);
}

/// Transport trait for sending messages and managing timers
#[async_trait]
pub trait Transport: Send + Sync {
    /// Register a receiver with the transport
    async fn register(
        &mut self,
        receiver: Arc<tokio::sync::Mutex<dyn TransportReceiver>>,
        config: &Configuration,
        replica_idx: usize,
    ) -> Result<()>;

    /// Send a message to a specific address
    async fn send_message(&self, dst: &TransportAddress, msg: &dyn Message) -> Result<()>;

    /// Send a message to a specific replica
    async fn send_message_to_replica(&self, replica_idx: usize, msg: &dyn Message) -> Result<()>;

    /// Broadcast a message to all replicas
    async fn send_message_to_all(&self, msg: &dyn Message) -> Result<()>;

    /// Set a timer that fires after the specified duration
    async fn set_timer(&self, duration_ms: u64, callback: Box<dyn FnOnce() + Send>)
        -> Result<u64>;

    /// Cancel a timer by ID
    async fn cancel_timer(&self, timer_id: u64) -> Result<bool>;

    /// Cancel all active timers
    async fn cancel_all_timers(&self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_structure() {
        // Basic module structure test
    }
}
