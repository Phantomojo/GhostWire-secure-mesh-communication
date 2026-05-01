Crate trust\_store Copy item path

<span class="sub-heading"><a href="../src/trust_store/lib.rs.html#1-450" class="src">Source</a>
</span>

Expand description

# <a href="#trust-store" class="doc-anchor">§</a>trust-store

Web of Trust with TOFU (Trust-On-First-Use) key verification and social
graph attestation for P2P networks.

## <a href="#features" class="doc-anchor">§</a>Features

-   **TOFU key pinning** — pin a peer’s public key on first contact
-   **Key change detection** — flag potential MITM attacks
-   **Social graph attestation** — peers vouch for each other’s keys
-   **Computed trust scores** — bubble trust through the graph with
    decay
-   **Ed25519 signature verification** — cryptographically verify
    attestations

## <a href="#quick-start" class="doc-anchor">§</a>Quick Start

``` {.rust .rust-example-rendered}
use trust_store::TrustStore;

let store = TrustStore::new();
let key = vec![0x01u8; 32];

// First contact — TOFU pin
let is_new = store.verify_or_pin("peer1", &key).await.unwrap();
assert!(is_new);

// Second contact with same key — trusted
let is_new = store.verify_or_pin("peer1", &key).await.unwrap();
assert!(!is_new);
```

## <a href="#no-pki-no-ca-no-blockchain" class="doc-anchor">§</a>No PKI, No CA, No Blockchain

Pure peer attestation — like SSH known\_hosts + web of trust combined.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Attestation.html" class="struct"
title="struct trust_store::Attestation">Attestation</a>  
A vouching attestation from one peer about another

<a href="struct.PeerRecord.html" class="struct"
title="struct trust_store::PeerRecord">PeerRecord</a>  
Our record of a known peer

<a href="struct.TrustStore.html" class="struct"
title="struct trust_store::TrustStore">TrustStore</a>  
The trust store — manages all known peer records

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.TrustError.html" class="enum"
title="enum trust_store::TrustError">TrustError</a>  
Errors that can occur during trust store operations.

<a href="enum.TrustLevel.html" class="enum"
title="enum trust_store::TrustLevel">TrustLevel</a>  
Trust level for a known peer

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.TrustResult.html" class="type"
title="type trust_store::TrustResult">TrustResult</a>  
Result type for trust store operations
