//! Storage layer implementations for TAPIR
//!
//! This module provides different storage implementations:
//! - TAPIR Store: Distributed transactional storage using IR
//! - Strong Store: 2PC-based transactional storage with OCC and locking
//! - Weak Store: Eventually consistent storage

pub mod common;
pub mod tapir;

pub use common::{Transaction, TransactionStore};
