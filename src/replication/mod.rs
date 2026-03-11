//! Replication protocols for TAPIR
//!
//! This module implements various replication protocols including:
//! - Inconsistent Replication (IR): High-performance unordered replication
//! - Viewstamped Replication (VR): Ordered replication for strong consistency

pub mod common;
pub mod ir;
pub mod vr;

pub use common::{ReplicaClient, ReplicaServer, ViewStamp};
