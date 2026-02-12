# Translation Guide: C++ TAPIR to Rust

This document explains how the original C++ TAPIR implementation has been translated to Rust.

## Overview

The original TAPIR implementation (UWSysLab/tapir) is written in C++ and uses:
- Custom memory management
- Protobuf for serialization
- libevent for networking
- Manual callback-based async

The Rust translation modernizes these choices while maintaining the core protocol semantics.

## Key Architectural Changes

### 1. Memory Management

**C++:**
```cpp
class Transport {
    virtual ~Transport() {}
    // Manual memory management
};
```

**Rust:**
```rust
pub trait Transport: Send + Sync {
    // Automatic memory management via ownership
}
```

- Rust's ownership system replaces manual memory management
- `Arc<T>` and `Arc<Mutex<T>>` replace shared pointers
- Lifetimes ensure safety without garbage collection

### 2. Asynchronous Operations

**C++:**
```cpp
typedef std::function<void (void)> timer_callback_t;
virtual int Timer(uint64_t ms, timer_callback_t cb) = 0;
```

**Rust:**
```rust
async fn set_timer(&self, duration_ms: u64, 
                   callback: Box<dyn FnOnce() + Send>) -> Result<u64>;
```

- Async/await replaces callback-based code
- Tokio runtime provides event loop functionality
- `async_trait` enables async methods in traits

### 3. Serialization

**C++:**
```cpp
#include <google/protobuf/message.h>
typedef ::google::protobuf::Message Message;
```

**Rust:**
```rust
use serde::{Serialize, Deserialize};
use bincode;
```

- Serde provides compile-time serialization
- Bincode offers efficient binary encoding
- Type safety without code generation

### 4. Error Handling

**C++:**
```cpp
// Often uses exceptions or return codes
bool SendMessage(TransportReceiver *src, 
                const TransportAddress &dst,
                const Message &m);
```

**Rust:**
```rust
async fn send_message(&self, dst: &TransportAddress, 
                      msg: &dyn Message) -> Result<()>;
```

- `Result<T, E>` type for explicit error handling
- No exceptions or null pointers
- Pattern matching for error recovery

## Module Mapping

### Original C++ Structure
```
tapir/
├── lib/                    # Transport layer
│   ├── configuration.*
│   ├── transport.*
│   ├── udptransport.*
│   └── tcptransport.*
├── replication/
│   ├── common/
│   ├── ir/                # Inconsistent Replication
│   └── vr/                # Viewstamped Replication
└── store/
    ├── common/
    ├── tapirstore/
    ├── strongstore/
    └── weakstore/
```

### Rust Translation
```
tapir-rs/
├── src/
│   ├── config.rs          # Configuration
│   ├── transport/         # Transport layer
│   │   ├── address.rs
│   │   ├── message.rs
│   │   └── timeout.rs
│   ├── replication/       # Replication protocols
│   │   ├── common.rs
│   │   ├── ir.rs
│   │   └── vr.rs
│   └── store/             # Storage implementations
│       ├── common.rs
│       └── tapir.rs
└── examples/              # Example applications
```

## Type Mappings

| C++ Type | Rust Equivalent | Notes |
|----------|----------------|-------|
| `uint64_t` | `u64` | Unsigned 64-bit integer |
| `string` | `String` or `&str` | UTF-8 string |
| `std::vector<T>` | `Vec<T>` | Dynamic array |
| `std::map<K,V>` | `HashMap<K,V>` | Hash map |
| `std::function<>` | `Box<dyn Fn()>` | Function object |
| `std::shared_ptr<T>` | `Arc<T>` | Shared ownership |
| `std::mutex` | `Mutex<T>` or `RwLock<T>` | Synchronization |

## Protocol Translation

### Configuration

**C++ (configuration.h):**
```cpp
class Configuration {
public:
    int n;  // number of replicas
    int f;  // failures tolerated
    int QuorumSize() const { return n - f; }
};
```

**Rust (config.rs):**
```rust
pub struct Configuration {
    pub n: usize,
    pub f: usize,
    // ...
}

impl Configuration {
    pub fn quorum_size(&self) -> usize {
        self.n - self.f
    }
}
```

### ViewStamp

**C++ (viewstamp.h):**
```cpp
typedef uint64_t view_t;
typedef uint64_t opnum_t;

struct viewstamp_t {
    view_t view;
    opnum_t opnum;
};
```

**Rust (replication/common.rs):**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, 
         PartialOrd, Ord, Serialize, Deserialize)]
pub struct ViewStamp {
    pub view: u64,
    pub opnum: u64,
}
```

Benefits:
- Automatic comparison operators via `derive`
- Serialization support built-in
- Type safety prevents mixing view/opnum

## Testing Approach

### C++ Testing
- Uses Google Test framework
- Manual setup/teardown
- Synchronous tests

### Rust Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_creation() {
        // Test code
    }

    #[tokio::test]
    async fn test_async_operation() {
        // Async test code
    }
}
```

Benefits:
- Built into language with `cargo test`
- Async tests via `tokio::test`
- Property testing via `proptest` (future)

## Implementation Status

### Completed
- ✅ Basic project structure
- ✅ Error handling framework
- ✅ Configuration management
- ✅ Transport abstractions
- ✅ Replication type definitions
- ✅ Storage interfaces

### In Progress
- 🔄 Network transport implementations
- 🔄 Full IR protocol
- 🔄 Full VR protocol
- 🔄 Complete TAPIR store

### Planned
- 📋 Lock server
- 📋 Time server
- 📋 Benchmarking framework
- 📋 Example applications

## Performance Considerations

### Zero-Cost Abstractions
Rust's traits compile to static dispatch by default, with no runtime overhead.

### Memory Layout
```rust
#[repr(C)]  // Can match C layout if needed
struct Message {
    // Predictable layout
}
```

### Async Runtime
Tokio provides:
- Efficient task scheduling
- Lock-free data structures
- Zero-copy I/O where possible

## Future Work

1. **Complete Network Layer**: Implement UDP and TCP transports using Tokio's networking
2. **Protocol Implementation**: Full IR and VR consensus protocols
3. **Performance Testing**: Benchmark against original C++ implementation
4. **Documentation**: Comprehensive API docs and protocol guides
5. **Examples**: Real-world usage examples and tutorials

## References

- [Original TAPIR Paper](http://dl.acm.org/authorize?N93281)
- [UWSysLab/tapir](https://github.com/UWSysLab/tapir) - Original C++ implementation
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/tokio/tutorial)
