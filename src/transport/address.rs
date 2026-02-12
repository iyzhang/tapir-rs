//! Transport address abstraction

use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::SocketAddr;

/// Abstract transport address
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportAddress {
    /// Socket address (IP + port)
    Socket(SocketAddr),
    /// Simulated address for testing
    Simulated(u64),
}

impl TransportAddress {
    /// Create a new socket address
    pub fn new_socket(addr: SocketAddr) -> Self {
        Self::Socket(addr)
    }

    /// Create a new simulated address
    pub fn new_simulated(id: u64) -> Self {
        Self::Simulated(id)
    }

    /// Get the socket address if this is a socket transport
    pub fn as_socket(&self) -> Option<&SocketAddr> {
        match self {
            Self::Socket(addr) => Some(addr),
            _ => None,
        }
    }

    /// Get the simulated ID if this is a simulated transport
    pub fn as_simulated(&self) -> Option<u64> {
        match self {
            Self::Simulated(id) => Some(*id),
            _ => None,
        }
    }
}

impl fmt::Display for TransportAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Socket(addr) => write!(f, "{}", addr),
            Self::Simulated(id) => write!(f, "sim:{}", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_socket_address() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let transport_addr = TransportAddress::new_socket(addr);

        assert!(transport_addr.as_socket().is_some());
        assert_eq!(transport_addr.as_socket().unwrap(), &addr);
        assert!(transport_addr.as_simulated().is_none());
    }

    #[test]
    fn test_simulated_address() {
        let transport_addr = TransportAddress::new_simulated(42);

        assert!(transport_addr.as_simulated().is_some());
        assert_eq!(transport_addr.as_simulated().unwrap(), 42);
        assert!(transport_addr.as_socket().is_none());
    }
}
