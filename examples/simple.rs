//! Simple example showing basic TAPIR-RS usage
//!
//! This example demonstrates:
//! - Creating a configuration
//! - Creating a TAPIR client
//! - Running a simple transaction

use tapir_rs::{
    config::{Configuration, ReplicaAddress},
    store::{tapir::TapirClient, TransactionStore},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("TAPIR-RS Simple Example");
    println!("========================\n");

    // Create a configuration for 3 replicas with f=1 fault tolerance
    println!("Creating configuration with 3 replicas...");
    let replicas = vec![
        ReplicaAddress::new("localhost".to_string(), 8000),
        ReplicaAddress::new("localhost".to_string(), 8001),
        ReplicaAddress::new("localhost".to_string(), 8002),
    ];

    let config = Configuration::new(3, 1, replicas)?;
    println!("Configuration created:");
    println!("  - Replicas: {}", config.n);
    println!("  - Fault tolerance: {}", config.f);
    println!("  - Quorum size: {}", config.quorum_size());
    println!("  - Fast quorum size: {}", config.fast_quorum_size());
    println!();

    // Create a TAPIR client
    println!("Creating TAPIR client...");
    let mut client = TapirClient::new();
    println!("Client created successfully\n");

    // Begin a transaction
    println!("Beginning transaction...");
    let txn_id = client.begin().await?;
    println!("Transaction started with ID: {}\n", txn_id);

    // Perform some operations
    println!("Performing transaction operations...");
    client
        .put(txn_id, b"key1".to_vec(), b"value1".to_vec())
        .await?;
    println!("  - PUT key1 = value1");

    client
        .put(txn_id, b"key2".to_vec(), b"value2".to_vec())
        .await?;
    println!("  - PUT key2 = value2");
    println!();

    // Commit the transaction
    println!("Committing transaction...");
    let committed = client.commit(txn_id).await?;
    if committed {
        println!("Transaction {} committed successfully!", txn_id);
    } else {
        println!("Transaction {} failed to commit", txn_id);
    }

    println!("\nExample completed successfully!");
    Ok(())
}
