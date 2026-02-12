//! TAPIR Store implementation
//!
//! The TAPIR store provides distributed transactional storage using
//! Inconsistent Replication (IR) for high performance.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::{Error, Result};
use crate::store::common::{Key, Transaction, TransactionId, TransactionStatus, TransactionStore, Value};

/// TAPIR store client
pub struct TapirClient {
    next_txn_id: Arc<RwLock<TransactionId>>,
    transactions: Arc<RwLock<HashMap<TransactionId, Transaction>>>,
}

impl TapirClient {
    /// Create a new TAPIR client
    pub fn new() -> Self {
        Self {
            next_txn_id: Arc::new(RwLock::new(1)),
            transactions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for TapirClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TransactionStore for TapirClient {
    async fn begin(&mut self) -> Result<TransactionId> {
        let mut next_id = self.next_txn_id.write().await;
        let txn_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let txn = Transaction::new(txn_id);
        self.transactions.write().await.insert(txn_id, txn);

        Ok(txn_id)
    }

    async fn get(&self, txn_id: TransactionId, _key: &Key) -> Result<Option<Value>> {
        // Check if transaction exists
        let txns = self.transactions.read().await;
        if !txns.contains_key(&txn_id) {
            return Err(Error::NotFound(format!("Transaction {} not found", txn_id)));
        }
        drop(txns);

        // In a full implementation, this would read from replicas
        // For now, return None (key not found)
        Ok(None)
    }

    async fn put(&mut self, txn_id: TransactionId, key: Key, value: Value) -> Result<()> {
        let mut txns = self.transactions.write().await;
        let txn = txns
            .get_mut(&txn_id)
            .ok_or_else(|| Error::NotFound(format!("Transaction {} not found", txn_id)))?;

        if txn.status != TransactionStatus::InProgress {
            return Err(Error::InvalidState(format!(
                "Transaction {} is not in progress",
                txn_id
            )));
        }

        txn.write(key, value);
        Ok(())
    }

    async fn commit(&mut self, txn_id: TransactionId) -> Result<bool> {
        let mut txns = self.transactions.write().await;
        let txn = txns
            .get_mut(&txn_id)
            .ok_or_else(|| Error::NotFound(format!("Transaction {} not found", txn_id)))?;

        if txn.status != TransactionStatus::InProgress {
            return Err(Error::InvalidState(format!(
                "Transaction {} is not in progress",
                txn_id
            )));
        }

        // In a full implementation, this would coordinate with replicas
        // For now, just mark as committed
        txn.status = TransactionStatus::Committed;
        Ok(true)
    }

    async fn abort(&mut self, txn_id: TransactionId) -> Result<()> {
        let mut txns = self.transactions.write().await;
        let txn = txns
            .get_mut(&txn_id)
            .ok_or_else(|| Error::NotFound(format!("Transaction {} not found", txn_id)))?;

        txn.status = TransactionStatus::Aborted;
        Ok(())
    }
}

/// TAPIR store replica
pub struct TapirReplica {
    replica_id: usize,
    storage: Arc<RwLock<HashMap<Key, Value>>>,
}

impl TapirReplica {
    /// Create a new TAPIR replica
    pub fn new(replica_id: usize) -> Self {
        Self {
            replica_id,
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get the replica ID
    pub fn id(&self) -> usize {
        self.replica_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tapir_client_lifecycle() {
        let mut client = TapirClient::new();

        // Begin transaction
        let txn_id = client.begin().await.unwrap();
        assert_eq!(txn_id, 1);

        // Put operation
        client
            .put(txn_id, b"key1".to_vec(), b"value1".to_vec())
            .await
            .unwrap();

        // Commit
        let result = client.commit(txn_id).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_tapir_replica_creation() {
        let replica = TapirReplica::new(0);
        assert_eq!(replica.id(), 0);
    }
}
