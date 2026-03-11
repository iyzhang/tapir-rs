//! Message abstraction for transport layer

use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Trait for messages that can be sent over the transport
pub trait Message: Send + Sync {
    /// Get the message type identifier
    fn msg_type(&self) -> &str;

    /// Serialize the message to bytes
    fn to_bytes(&self) -> Result<Bytes>;

    /// Deserialize from bytes
    fn from_bytes(data: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

/// Basic message wrapper for serializable types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicMessage<T: Serialize> {
    msg_type: String,
    payload: T,
}

impl<T: Serialize> BasicMessage<T> {
    pub fn new(msg_type: String, payload: T) -> Self {
        Self { msg_type, payload }
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }
}

impl<T> Message for BasicMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync,
{
    fn msg_type(&self) -> &str {
        &self.msg_type
    }

    fn to_bytes(&self) -> Result<Bytes> {
        bincode::serialize(self)
            .map(Bytes::from)
            .map_err(|e| crate::error::Error::Serialization(e.to_string()))
    }

    fn from_bytes(data: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        bincode::deserialize(data).map_err(|e| crate::error::Error::Serialization(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestPayload {
        id: u64,
        data: String,
    }

    #[test]
    fn test_message_serialization() {
        let payload = TestPayload {
            id: 42,
            data: "test".to_string(),
        };

        let msg = BasicMessage::new("test_msg".to_string(), payload.clone());
        assert_eq!(msg.msg_type(), "test_msg");

        let bytes = msg.to_bytes().unwrap();
        let deserialized: BasicMessage<TestPayload> = BasicMessage::from_bytes(&bytes).unwrap();

        assert_eq!(deserialized.payload(), &payload);
    }
}
