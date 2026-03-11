//! Configuration management for TAPIR
//!
//! This module handles replica group configuration, including the number
//! and addresses of replicas in the group.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::SocketAddr;

use crate::error::{Error, Result};

/// Address of a replica in the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReplicaAddress {
    /// Hostname or IP address
    pub host: String,
    /// Port number
    pub port: u16,
}

impl ReplicaAddress {
    /// Create a new replica address
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    /// Convert to a socket address
    pub fn to_socket_addr(&self) -> Result<SocketAddr> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(|e| Error::Configuration(format!("Invalid address: {}", e)))
    }
}

impl fmt::Display for ReplicaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

/// Configuration for a replica group
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Configuration {
    /// Number of replicas
    pub n: usize,
    /// Number of failures tolerated
    pub f: usize,
    /// List of replica addresses
    replicas: Vec<ReplicaAddress>,
    /// Optional multicast address
    multicast_address: Option<ReplicaAddress>,
}

impl Configuration {
    /// Create a new configuration
    pub fn new(n: usize, f: usize, replicas: Vec<ReplicaAddress>) -> Result<Self> {
        if replicas.len() != n {
            return Err(Error::Configuration(format!(
                "Number of replicas ({}) does not match n ({})",
                replicas.len(),
                n
            )));
        }

        if n <= 2 * f {
            return Err(Error::Configuration(format!(
                "Invalid configuration: n ({}) must be > 2*f ({})",
                n,
                2 * f
            )));
        }

        Ok(Self {
            n,
            f,
            replicas,
            multicast_address: None,
        })
    }

    /// Create a configuration with multicast support
    pub fn with_multicast(
        n: usize,
        f: usize,
        replicas: Vec<ReplicaAddress>,
        multicast: ReplicaAddress,
    ) -> Result<Self> {
        let mut config = Self::new(n, f, replicas)?;
        config.multicast_address = Some(multicast);
        Ok(config)
    }

    /// Get a replica address by index
    pub fn replica(&self, idx: usize) -> Option<&ReplicaAddress> {
        self.replicas.get(idx)
    }

    /// Get the multicast address if configured
    pub fn multicast(&self) -> Option<&ReplicaAddress> {
        self.multicast_address.as_ref()
    }

    /// Get the leader index for a given view
    pub fn get_leader_index(&self, view: u64) -> usize {
        (view as usize) % self.n
    }

    /// Calculate quorum size (n - f)
    pub fn quorum_size(&self) -> usize {
        self.n - self.f
    }

    /// Calculate fast quorum size (n - floor(f/2))
    pub fn fast_quorum_size(&self) -> usize {
        self.n - (self.f / 2)
    }

    /// Get all replica addresses
    pub fn all_replicas(&self) -> &[ReplicaAddress] {
        &self.replicas
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_creation() {
        let replicas = vec![
            ReplicaAddress::new("localhost".to_string(), 8000),
            ReplicaAddress::new("localhost".to_string(), 8001),
            ReplicaAddress::new("localhost".to_string(), 8002),
        ];

        let config = Configuration::new(3, 1, replicas).unwrap();
        assert_eq!(config.n, 3);
        assert_eq!(config.f, 1);
        assert_eq!(config.quorum_size(), 2);
        // fast_quorum_size = n - floor(f/2) = 3 - 0 = 3
        assert_eq!(config.fast_quorum_size(), 3);
    }

    #[test]
    fn test_leader_selection() {
        let replicas = vec![
            ReplicaAddress::new("localhost".to_string(), 8000),
            ReplicaAddress::new("localhost".to_string(), 8001),
            ReplicaAddress::new("localhost".to_string(), 8002),
        ];

        let config = Configuration::new(3, 1, replicas).unwrap();
        assert_eq!(config.get_leader_index(0), 0);
        assert_eq!(config.get_leader_index(1), 1);
        assert_eq!(config.get_leader_index(2), 2);
        assert_eq!(config.get_leader_index(3), 0);
    }

    #[test]
    fn test_invalid_configuration() {
        let replicas = vec![
            ReplicaAddress::new("localhost".to_string(), 8000),
            ReplicaAddress::new("localhost".to_string(), 8001),
        ];

        // n must be > 2*f, so n=2, f=1 is invalid
        let result = Configuration::new(2, 1, replicas);
        assert!(result.is_err());
    }
}
