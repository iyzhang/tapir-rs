//! Common replication interfaces and types

use serde::{Deserialize, Serialize};

/// View number and operation sequence number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ViewStamp {
    /// View number
    pub view: u64,
    /// Operation sequence number
    pub opnum: u64,
}

impl ViewStamp {
    /// Create a new viewstamp
    pub fn new(view: u64, opnum: u64) -> Self {
        Self { view, opnum }
    }

    /// Create a viewstamp with view 0 and opnum 0
    pub fn zero() -> Self {
        Self { view: 0, opnum: 0 }
    }
}

impl std::fmt::Display for ViewStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.view, self.opnum)
    }
}

/// Common trait for replica clients
pub trait ReplicaClient: Send + Sync {
    // Methods will be added as we implement specific protocols
}

/// Common trait for replica servers
pub trait ReplicaServer: Send + Sync {
    // Methods will be added as we implement specific protocols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewstamp_ordering() {
        let vs1 = ViewStamp::new(0, 1);
        let vs2 = ViewStamp::new(0, 2);
        let vs3 = ViewStamp::new(1, 0);

        assert!(vs1 < vs2);
        assert!(vs2 < vs3);
        assert!(vs1 < vs3);
    }
}
