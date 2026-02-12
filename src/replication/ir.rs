//! Inconsistent Replication (IR) protocol
//!
//! IR is a high-performance replication protocol that provides no ordering
//! guarantees. It's designed to work with transaction protocols like TAPIR
//! that enforce consistency at a higher layer.

/// IR protocol client
#[derive(Default)]
pub struct IrClient {
    // Implementation will be added
}

impl IrClient {
    pub fn new() -> Self {
        Self::default()
    }
}

/// IR protocol server/replica
#[derive(Default)]
pub struct IrReplica {
    // Implementation will be added
}

impl IrReplica {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_creation() {
        let _client = IrClient::new();
        let _replica = IrReplica::new();
    }
}
