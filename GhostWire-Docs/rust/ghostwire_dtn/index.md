Crate ghostwire\_dtn Copy item path

<span class="sub-heading"><a href="../src/ghostwire_dtn/lib.rs.html#1-777" class="src">Source</a>
</span>

Expand description

# <a href="#ghostwire-dtn" class="doc-anchor">§</a>ghostwire-dtn

Delay-Tolerant Networking (DTN) with Spray-and-Wait + PRoPHET epidemic
routing.

## <a href="#features" class="doc-anchor">§</a>Features

-   **Spray-and-Wait** (Spyropoulos et al., 2005) — efficient bundle
    distribution
-   **PRoPHET** (Lindgren et al., 2003) — delivery predictability with
    time decay and transitivity
-   **Bloom filter sync** — efficient set reconciliation during
    Bluetooth encounters
-   **Bundle encryption** — X25519 ECDH + HKDF + XChaCha20-Poly1305
-   **Bounded buffer** — 50MB max with LRU eviction by delivery
    probability
-   **GNN-enhanced routing** — optional ML-based forwarding decisions

## <a href="#quick-start" class="doc-anchor">§</a>Quick Start

``` {.rust .rust-example-rendered}
use ghostwire_dtn::{DtnStore, DtnBundle, DtnSyncHandshake, SyncBloom};

let store = DtnStore::new([0x01u8; 32]);

// Create a bundle
let bundle = DtnBundle::new(vec![1, 2, 3, 4], [0x02u8; 32], None);
let cid = bundle.cid;

// Store it
let stored = store.store(bundle).await;
assert!(stored);

// Retrieve it
let retrieved = store.get(&cid).await;
assert!(retrieved.is_some());
```

## <a href="#protocol" class="doc-anchor">§</a>Protocol

When two nodes come within range:

1.  Exchange `DtnSyncHandshake` messages (bloom filter + delivery
    scores)
2.  Each node computes which bundles to forward (`on_encounter`)
3.  Bundles are transferred
4.  Delivery tables are updated with transitivity

## <a href="#reference" class="doc-anchor">§</a>Reference

-   Spyropoulos, T., et al. “Spray and Wait: An Efficient Routing Scheme
    for Intermittently Connected Mobile Networks.”
-   Lindgren, A., et al. “Probabilistic Routing in Intermittently
    Connected Networks.”

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DeliveryTable.html" class="struct"
title="struct ghostwire_dtn::DeliveryTable">DeliveryTable</a>  
PRoPHET delivery predictability table

<a href="struct.DtnBundle.html" class="struct"
title="struct ghostwire_dtn::DtnBundle">DtnBundle</a>  
A DTN bundle wrapping encrypted payload bytes

<a href="struct.DtnStore.html" class="struct"
title="struct ghostwire_dtn::DtnStore">DtnStore</a>  
The GhostDTN store: bundles + delivery table

<a href="struct.DtnSyncHandshake.html" class="struct"
title="struct ghostwire_dtn::DtnSyncHandshake">DtnSyncHandshake</a>  
DTN sync handshake message

<a href="struct.EncryptedBundlePayload.html" class="struct"
title="struct ghostwire_dtn::EncryptedBundlePayload">EncryptedBundlePayload</a>  
Encrypted DTN transport payload

<a href="struct.SyncBloom.html" class="struct"
title="struct ghostwire_dtn::SyncBloom">SyncBloom</a>  
Compact bloom filter for advertising which CIDs we hold

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.BUNDLE_TTL_SECS.html" class="constant"
title="constant ghostwire_dtn::BUNDLE_TTL_SECS">BUNDLE_TTL_SECS</a>  
Default TTL: 72 hours in seconds

<a href="constant.SPRAY_L.html" class="constant"
title="constant ghostwire_dtn::SPRAY_L">SPRAY_L</a>  
Default spray copies (L=6 from literature)
