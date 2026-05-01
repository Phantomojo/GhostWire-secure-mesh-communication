Crate hlc\_rs Copy item path

<span class="sub-heading"><a href="../src/hlc_rs/lib.rs.html#1-381" class="src">Source</a>
</span>

Expand description

# <a href="#hlc-rs" class="doc-anchor">§</a>hlc-rs

Hybrid Logical Clocks (HLC) for causally correct distributed event
ordering.

HLC combines physical time (NTP) with logical counters to provide
causally correct ordering without requiring perfectly synced clocks.

## <a href="#algorithm" class="doc-anchor">§</a>Algorithm

-   **On local event**: advance physical time, reset or increment
    counter
-   **On remote event**: merge with remote HLC, advance counter
-   **Comparison**: `(physical_time, logical_counter, node_id)` gives
    total order

## <a href="#quick-start" class="doc-anchor">§</a>Quick Start

``` {.rust .rust-example-rendered}
use hlc_rs::{HybridLogicalClock, HlcTimestamp};

let hlc_a = HybridLogicalClock::new(1);
let hlc_b = HybridLogicalClock::new(2);

// A sends a message
let ts_a = hlc_a.send();

// B receives A's message
let ts_b = hlc_b.receive(&ts_a);

// Causal order: A's send < B's receive
assert!(ts_a < ts_b);
```

## <a href="#features" class="doc-anchor">§</a>Features

-   **Thread-safe** — atomic CAS operations for concurrent access
-   **Clock manipulation detection** — hybrid SystemTime + Instant
    prevents backwards clock manipulation
-   **Message reorder buffer** — holds out-of-order messages and
    delivers in HLC order
-   **Zero dependencies** (except serde for serialization)

## <a href="#reference" class="doc-anchor">§</a>Reference

-   Kulkarni, S., et al. “Hybrid Logical Clocks.”

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.HlcTimestamp.html" class="struct"
title="struct hlc_rs::HlcTimestamp">HlcTimestamp</a>  
Hybrid Logical Clock timestamp.

<a href="struct.HybridLogicalClock.html" class="struct"
title="struct hlc_rs::HybridLogicalClock">HybridLogicalClock</a>  
Hybrid Logical Clock state for a single node.

<a href="struct.MessageReorderBuffer.html" class="struct"
title="struct hlc_rs::MessageReorderBuffer">MessageReorderBuffer</a>  
Message reorder buffer.
