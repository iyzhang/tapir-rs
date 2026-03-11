//! Timeout management for transport layer

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::Result;
use crate::transport::Transport;

/// Timeout wrapper that automatically manages timer lifecycle
pub struct Timeout {
    transport: Arc<Mutex<dyn Transport>>,
    duration_ms: u64,
    timer_id: Option<u64>,
    active: bool,
}

impl Timeout {
    /// Create a new timeout
    pub fn new(transport: Arc<Mutex<dyn Transport>>, duration_ms: u64) -> Self {
        Self {
            transport,
            duration_ms,
            timer_id: None,
            active: false,
        }
    }

    /// Start the timeout
    pub async fn start(&mut self, callback: Box<dyn FnOnce() + Send>) -> Result<()> {
        if self.active {
            self.stop().await?;
        }

        let transport = self.transport.lock().await;
        let timer_id = transport.set_timer(self.duration_ms, callback).await?;
        drop(transport);

        self.timer_id = Some(timer_id);
        self.active = true;
        Ok(())
    }

    /// Reset the timeout with the same duration
    pub async fn reset(&mut self, callback: Box<dyn FnOnce() + Send>) -> Result<()> {
        self.start(callback).await
    }

    /// Stop the timeout
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(timer_id) = self.timer_id {
            let transport = self.transport.lock().await;
            transport.cancel_timer(timer_id).await?;
            drop(transport);
        }
        self.active = false;
        self.timer_id = None;
        Ok(())
    }

    /// Check if the timeout is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set a new timeout duration
    pub fn set_duration(&mut self, duration_ms: u64) {
        self.duration_ms = duration_ms;
    }
}

impl Drop for Timeout {
    fn drop(&mut self) {
        // Note: We can't call async stop() in Drop, so timers must be explicitly stopped
        // or will be cleaned up when the transport is dropped
        self.active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_lifecycle() {
        // Timeout lifecycle test
        // (Would need a mock transport implementation for full testing)
    }
}
