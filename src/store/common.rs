//! Common storage interfaces and types

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Transaction ID
pub type TransactionId = u64;

/// Key type for storage
pub type Key = Vec<u8>;

/// Value type for storage
pub type Value = Vec<u8>;

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Transaction is in progress
    InProgress,
    /// Transaction committed successfully
    Committed,
    /// Transaction was aborted
    Aborted,
}

/// Read/write operation in a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    /// Read operation
    Read { key: Key },
    /// Write operation
    Write { key: Key, value: Value },
}

/// Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique transaction ID
    pub id: TransactionId,
    /// Operations in this transaction
    pub operations: Vec<Operation>,
    /// Transaction status
    pub status: TransactionStatus,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(id: TransactionId) -> Self {
        Self {
            id,
            operations: Vec::new(),
            status: TransactionStatus::InProgress,
        }
    }

    /// Add a read operation
    pub fn read(&mut self, key: Key) {
        self.operations.push(Operation::Read { key });
    }

    /// Add a write operation
    pub fn write(&mut self, key: Key, value: Value) {
        self.operations.push(Operation::Write { key, value });
    }
}

/// Common trait for transactional storage
#[async_trait]
pub trait TransactionStore: Send + Sync {
    /// Begin a new transaction
    async fn begin(&mut self) -> Result<TransactionId>;

    /// Get a value by key
    async fn get(&self, txn_id: TransactionId, key: &Key) -> Result<Option<Value>>;

    /// Put a key-value pair
    async fn put(&mut self, txn_id: TransactionId, key: Key, value: Value) -> Result<()>;

    /// Commit a transaction
    async fn commit(&mut self, txn_id: TransactionId) -> Result<bool>;

    /// Abort a transaction
    async fn abort(&mut self, txn_id: TransactionId) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let mut txn = Transaction::new(42);
        assert_eq!(txn.id, 42);
        assert_eq!(txn.status, TransactionStatus::InProgress);

        txn.read(b"key1".to_vec());
        txn.write(b"key2".to_vec(), b"value2".to_vec());
        assert_eq!(txn.operations.len(), 2);
    }
}
