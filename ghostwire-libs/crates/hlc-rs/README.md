# hlc-rs

[![Crates.io](https://img.shields.io/crates/v/hlc-rs.svg)](https://crates.io/crates/hlc-rs)
[![Documentation](https://docs.rs/hlc-rs/badge.svg)](https://docs.rs/hlc-rs)
[![License](https://img.shields.io/crates/l/hlc-rs)](https://github.com/Phantomojo/GhostWire-secure-mesh-communication)

Hybrid Logical Clocks (HLC) for causally correct distributed event ordering.

## Features

- **Thread-safe** — atomic CAS operations for concurrent access
- **Clock manipulation detection** — hybrid SystemTime + Instant prevents backwards clock manipulation
- **Message reorder buffer** — holds out-of-order messages and delivers in HLC order
- **Zero dependencies** (except serde for serialization)

## Installation

```toml
[dependencies]
hlc-rs = "0.1"
```

## Quick Start

```rust
use hlc_rs::{HybridLogicalClock, HlcTimestamp};

let hlc_a = HybridLogicalClock::new(1);
let hlc_b = HybridLogicalClock::new(2);

let ts_a = hlc_a.send();
let ts_b = hlc_b.receive(&ts_a);
assert!(ts_a < ts_b);
```

## Reference

- Kulkarni, S., et al. "Hybrid Logical Clocks."

## License

Licensed under either of Apache License 2.0 or MIT license.
