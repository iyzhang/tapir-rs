# Contributing to TAPIR-RS

Thank you for your interest in contributing to TAPIR-RS! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.70 or later (edition 2021)
- Cargo (comes with Rust)

### Building the Project

```bash
# Clone the repository
git clone https://github.com/iyzhang/tapir-rs.git
cd tapir-rs

# Build the project
cargo build

# Build with optimizations
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### Running Examples

```bash
# List available examples
ls examples/

# Run an example
cargo run --example simple
```

## Project Structure

- `src/lib.rs` - Library root and module exports
- `src/error.rs` - Error types and handling
- `src/config.rs` - Configuration management
- `src/transport/` - Network transport layer
  - `address.rs` - Transport address abstraction
  - `message.rs` - Message serialization
  - `timeout.rs` - Timeout management
- `src/replication/` - Replication protocols
  - `common.rs` - Common replication types (ViewStamp)
  - `ir.rs` - Inconsistent Replication protocol
  - `vr.rs` - Viewstamped Replication protocol
- `src/store/` - Storage layer
  - `common.rs` - Common storage interfaces
  - `tapir.rs` - TAPIR store implementation
- `examples/` - Example applications

## Coding Guidelines

### Style

- Follow standard Rust formatting (use `cargo fmt`)
- Run `cargo clippy` to catch common issues
- Write documentation for public APIs
- Add tests for new functionality

### Testing

- Unit tests should be in the same file as the code they test
- Integration tests should be in the `tests/` directory
- Async tests should use `#[tokio::test]`

### Error Handling

- Use the `Result<T>` type alias from `crate::error`
- Return appropriate error variants from the `Error` enum
- Provide meaningful error messages

### Async Code

- Use Tokio for all async operations
- Prefer `async/await` over manual futures
- Use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for shared state

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run formatting (`cargo fmt`)
6. Run clippy (`cargo clippy`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to your branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## License

By contributing to TAPIR-RS, you agree that your contributions will be licensed under the MIT License.
