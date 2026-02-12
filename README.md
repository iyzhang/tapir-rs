# TAPIR-RS

A Rust implementation of TAPIR (Transaction Application Protocol for Inconsistent Replication).

## About

TAPIR is a protocol for linearizable distributed transactions built using replication with no consistency guarantees. By enforcing consistency only at the transaction layer, TAPIR eliminates coordination at the replication layer, enabling better latency and throughput compared to traditional systems.

This is a Rust translation of the original C++ implementation from [UWSysLab/tapir](https://github.com/UWSysLab/tapir), which was used for the SOSP 2015 paper: ["Building Consistent Transactions with Inconsistent Replication."](http://dl.acm.org/authorize?N93281)

## Features

- **Inconsistent Replication (IR)**: High-performance, unordered replication protocol
- **Viewstamped Replication (VR)**: Ordered replication protocol for strong consistency
- **TAPIR Store**: Distributed transactional storage using IR
- **Strong Store**: 2PC-based transactional storage with OCC and strict 2PL
- **Lock Server**: Distributed lock service for IR
- **Network Transports**: UDP, TCP, and simulation-based networking

## Project Structure

```
tapir-rs/
├── src/
│   ├── lib.rs              # Library root
│   ├── transport/          # Network transport layer (UDP, TCP, simulation)
│   ├── replication/        # Replication protocols (IR, VR)
│   ├── store/              # Storage implementations
│   │   ├── common/         # Common storage interfaces
│   │   ├── tapir/          # TAPIR store
│   │   ├── strong/         # Strong store (2PC)
│   │   └── weak/           # Weak store (eventual consistency)
│   ├── lockserver/         # Lock server
│   └── timeserver/         # Time server
├── benches/                # Performance benchmarks
└── examples/               # Example applications
```

## Building

```bash
cargo build --release
```

## Running Tests

```bash
cargo test
```

## Benchmarks

```bash
cargo bench
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Original TAPIR copyright (c) 2019 Irene Zhang, Dan Ports, Naveen Kr. Sharma  
Rust translation copyright (c) 2026 TAPIR-RS Contributors

## References

- Original TAPIR Paper: [Building Consistent Transactions with Inconsistent Replication](http://dl.acm.org/authorize?N93281)
- Original C++ Implementation: [UWSysLab/tapir](https://github.com/UWSysLab/tapir)
