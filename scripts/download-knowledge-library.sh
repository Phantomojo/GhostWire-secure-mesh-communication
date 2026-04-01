#!/bin/bash
# Download offline knowledge library for GhostWire development
# This script fetches essential documentation, books, whitepapers, and RFCs

set -e

echo "📚 GhostWire Knowledge Library Download"
echo "======================================="
echo ""

LIBRARY_DIR=".qwen/library"

# Create directory structure
echo "Creating directory structure..."
mkdir -p "$LIBRARY_DIR"/{books/{rust,cryptography,distributed_systems,security,opsec},whitepapers/{libp2p,privacy,mesh,protocol},rfcs,cheat-sheets,links}

# ============= RUST DOCUMENTATION =============
echo ""
echo "🦀 Downloading Rust Documentation..."

# The Rust Book (official)
echo "  - The Rust Programming Language..."
curl -sL "https://doc.rust-lang.org/book/second-edition/print.pdf" \
  -o "$LIBRARY_DIR/books/rust/The_Rust_Programming_Language.pdf" 2>/dev/null || \
  echo "    ⚠️  Book download failed (may need manual download)"

# Rust By Example
echo "  - Rust By Example..."
curl -sL "https://doc.rust-lang.org/rust-by-example/print.html" \
  -o "$LIBRARY_DIR/books/rust/Rust_By_Example.html" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# Tokio Tutorial
echo "  - Tokio Tutorial..."
curl -sL "https://tokio.rs/tokio/tutorial" \
  -o "$LIBRARY_DIR/books/rust/Tokio_Tutorial.html" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# ============= CRYPTOGRAPHY DOCUMENTATION =============
echo ""
echo "🔐 Downloading Cryptography Documentation..."

# NIST PQC Standards
echo "  - NIST FIPS 203 (ML-KEM/Kyber)..."
curl -sL "https://csrc.nist.gov/publications/detail/fips/203/final" \
  -o "$LIBRARY_DIR/books/cryptography/NIST_FIPS_203_ML-KEM.pdf" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# Real-World Cryptography (sample chapters)
echo "  - Cryptography resources..."
cat > "$LIBRARY_DIR/books/cryptography/README.md" << 'EOF'
# Cryptography Books

## Recommended Reading

1. **Real-World Cryptography** by David Wong
   - https://www.manning.com/books/real-world-cryptography
   - Purchase for full PDF

2. **Cryptography Engineering** by Ferguson, Schneier, Kohno
   - Classic text on practical crypto

3. **Serious Cryptography** by Jean-Philippe Aumasson
   - Modern cryptographic implementations

## Free Resources

- [Cryptobook (University of Washington)](https://cryptobook.cs.washington.edu/)
- [Practical Cryptography for Developers](https://cryptobook.nakov.com/)
EOF

# ============= LIBP2P WHITEPAPERS =============
echo ""
echo "🌐 Downloading libp2p Documentation..."

# Kademlia Paper
echo "  - Kademlia DHT Paper..."
curl -sL "https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf" \
  -o "$LIBRARY_DIR/whitepapers/libp2p/Kademlia_DHT.pdf" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# libp2p Specs
echo "  - libp2p Specifications..."
if command -v git &> /dev/null; then
    git clone --depth 1 https://github.com/libp2p/specs.git \
      "$LIBRARY_DIR/whitepapers/libp2p/specs" 2>/dev/null || \
      echo "    ⚠️  Git clone failed"
fi

# Gossipsub Spec
echo "  - Gossipsub Specification..."
curl -sL "https://raw.githubusercontent.com/libp2p/specs/master/pubsub/gossipsub/gossipsub-v1.1.md" \
  -o "$LIBRARY_DIR/whitepapers/libp2p/Gossipsub_Spec.md" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# Noise Protocol
echo "  - Noise Protocol Framework..."
curl -sL "https://noiseprotocol.org/noise.html" \
  -o "$LIBRARY_DIR/whitepapers/libp2p/Noise_Protocol.html" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# ============= PRIVACY & STEALTH =============
echo ""
echo "👻 Downloading Privacy/Stealth Documentation..."

# obfs4 Spec
echo "  - obfs4 Specification..."
curl -sL "https://gitlab.com/yawning/obfs4/-/raw/master/README.md" \
  -o "$LIBRARY_DIR/whitepapers/privacy/obfs4_Spec.md" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# Tor Design Paper
echo "  - Tor Design Paper..."
curl -sL "https://svn.torproject.org/svn/projects/design-paper/tor-design.pdf" \
  -o "$LIBRARY_DIR/whitepapers/privacy/Tor_Design.pdf" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# Briar Protocol
echo "  - Briar Protocol..."
curl -sL "https://briarproject.org/protocol/" \
  -o "$LIBRARY_DIR/whitepapers/privacy/Briar_Protocol.html" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# ============= MESH NETWORKING =============
echo ""
echo "📡 Downloading Mesh Networking Documentation..."

# Meshtastic Protocol
echo "  - Meshtastic Protocol..."
curl -sL "https://meshtastic.org/docs/protocol/" \
  -o "$LIBRARY_DIR/whitepapers/mesh/Meshtastic_Protocol.html" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# Reticulum Network
echo "  - Reticulum Network..."
curl -sL "https://reticulum.network/" \
  -o "$LIBRARY_DIR/whitepapers/mesh/Reticulum_Network.html" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# ============= RFCs =============
echo ""
echo "📜 Downloading RFCs..."

# Ed25519
echo "  - RFC 8032 (Ed25519)..."
curl -sL "https://www.rfc-editor.org/rfc/pdfrfc/rfc8032.txt.pdf" \
  -o "$LIBRARY_DIR/rfcs/RFC_8032_Ed25519.pdf" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# X25519/Curves
echo "  - RFC 7748 (Elliptic Curves)..."
curl -sL "https://www.rfc-editor.org/rfc/pdfrfc/rfc7748.txt.pdf" \
  -o "$LIBRARY_DIR/rfcs/RFC_7748_Curves.pdf" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# TLS 1.3
echo "  - RFC 8446 (TLS 1.3)..."
curl -sL "https://www.rfc-editor.org/rfc/pdfrfc/rfc8446.txt.pdf" \
  -o "$LIBRARY_DIR/rfcs/RFC_8446_TLS_1.3.pdf" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# HPBS
echo "  - RFC 9180 (HPBS)..."
curl -sL "https://www.rfc-editor.org/rfc/pdfrfc/rfc9180.txt.pdf" \
  -o "$LIBRARY_DIR/rfcs/RFC_9180_HPBS.pdf" 2>/dev/null || \
  echo "    ⚠️  Download failed"

# ============= CHEAT SHEETS =============
echo ""
echo "📋 Creating Cheat Sheets..."

cat > "$LIBRARY_DIR/cheat-sheets/rust-cheat-sheet.md" << 'EOF'
# Rust Cheat Sheet

## Ownership
- Each value has one owner
- When owner goes out of scope, value is dropped
- Borrow: `&T` (immutable), `&mut T` (mutable)

## Common Patterns
```rust
// Result handling
let value = result?;  // Propagate error
let value = result.unwrap_or(default);  // Default on error

// Option handling
let value = option.unwrap_or(default);
let value = option.map(|x| x * 2);

// Async
async fn fetch() -> Result<Data> { ... }
let data = fetch().await?;
```

## Clippy Lints
```bash
cargo clippy -- -W clippy::pedantic
cargo clippy --fix
```
EOF

cat > "$LIBRARY_DIR/cheat-sheets/libp2p-cheat-sheet.md" << 'EOF'
# libp2p Cheat Sheet

## Peer ID
- Format: Blake2b hash of public key
- Length: 32 bytes (base58 encoded)

## Multiaddr Format
```
/ip4/192.168.1.1/tcp/4001/p2p/QmPeerID
/ip4/10.0.0.1/udp/4001/quic/p2p/QmPeerID
```

## Kademlia
- Distance: XOR metric
- K-Buckets: Organized by distance
- Replication Factor: 20
- Alpha: 3 (parallel lookups)

## Gossipsub
- D: 6 (mesh degree)
- D_lo: 4, D_hi: 12
- Heartbeat: 1 second
EOF

cat > "$LIBRARY_DIR/cheat-sheets/crypto-cheat-sheet.md" << 'EOF'
# Cryptography Cheat Sheet

## Algorithms in GhostWire
- **Ed25519**: Digital signatures
- **X25519**: Key exchange (Diffie-Hellman)
- **AES-256-GCM**: Symmetric encryption
- **ChaCha20-Poly1305**: AEAD encryption
- **ML-KEM (Kyber)**: Post-quantum KEM
- **Argon2id**: Password hashing

## Key Sizes
- Ed25519: 32 bytes (secret), 32 bytes (public)
- X25519: 32 bytes (secret), 32 bytes (public)
- AES-256: 32 bytes
- ChaCha20: 32 bytes
- Nonce (GCM): 12 bytes

## Best Practices
- Always use CSPRNG (OsRng)
- Zeroize keys after use
- Never reuse nonces
- Use AEAD modes only
EOF

# ============= LINKS COLLECTION =============
echo ""
echo "🔗 Creating Links Collection..."

cat > "$LIBRARY_DIR/links/official-docs.md" << 'EOF'
# Official Documentation Links

## Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust Crypto](https://github.com/RustCrypto)

## libp2p
- [libp2p Docs](https://docs.libp2p.io/)
- [libp2p Specs](https://github.com/libp2p/specs)
- [Rust libp2p](https://github.com/libp2p/rust-libp2p)

## Cryptography
- [dalek cryptography](https://dalek.rs/)
- [NIST PQC](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [Noise Protocol](https://noiseprotocol.org/)

## GhostWire
- [Architecture Docs](../../docs/ARCHITECTURE.md)
- [Security Model](../../docs/SECURITY.md)
- [API Docs](../../docs/api/DEEP_DIVE.md)
EOF

cat > "$LIBRARY_DIR/links/communities.md" << 'EOF'
# Developer Communities

## Rust
- r/rust (Reddit)
- users.rust-lang.org
- Rust Discord
- Rust Users Forum

## libp2p
- libp2p Discord
- IPFS Discord
- Protocol Labs Research

## Cryptography
- r/cryptography (Reddit)
- IACR (International Association for Cryptologic Research)
- Cryptography Stack Exchange

## Privacy/Security
- r/privacy (Reddit)
- r/netsec (Reddit)
- Hacker News
- Lobsters
EOF

# ============= SUMMARY =============
echo ""
echo "=========================================="
echo "✅ Knowledge Library Download Complete!"
echo "=========================================="
echo ""
echo "Library location: $LIBRARY_DIR"
echo ""
echo "Contents:"
echo "  📚 Books:      $LIBRARY_DIR/books/"
echo "  📄 Whitepapers: $LIBRARY_DIR/whitepapers/"
echo "  📜 RFCs:       $LIBRARY_DIR/rfcs/"
echo "  📋 Cheat Sheets: $LIBRARY_DIR/cheat-sheets/"
echo "  🔗 Links:      $LIBRARY_DIR/links/"
echo ""
echo "Note: Some downloads may have failed due to"
echo "      direct PDF links. Check README files"
echo "      for manual download instructions."
echo ""
echo "Next: Review and add additional resources"
echo "      from global knowledge sources."
