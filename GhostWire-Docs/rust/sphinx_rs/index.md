Crate sphinx\_rs Copy item path

<span class="sub-heading"><a href="../src/sphinx_rs/lib.rs.html#1-958" class="src">Source</a>
</span>

Expand description

# <a href="#sphinx-rs" class="doc-anchor">§</a>sphinx-rs

A production-grade implementation of the
[Sphinx](https://www.freehaven.net/anonbib/cache/sphinx.pdf) onion
routing protocol (Danezis & Goldberg, 2009) with Single-Use Reply Blocks
(SURBs).

## <a href="#core-guarantee-bitwise-unlinkability" class="doc-anchor">§</a>Core Guarantee: Bitwise Unlinkability

An adversary who sees a Sphinx packet enter a node AND a packet leave
cannot determine they are the same message, even when controlling both
neighbours.

## <a href="#protocol-overview" class="doc-anchor">§</a>Protocol Overview

Sphinx uses a **layered header** construction:

1.  The sender builds the header **backwards** (last hop → first hop)
2.  Each hop’s routing info is prepended to a growing buffer
3.  The entire buffer is XOR-encrypted with a stream cipher derived from
    the hop’s shared secret
4.  A MAC is computed over the encrypted buffer for the next hop to
    verify

At each hop:

1.  Verify the MAC (packet integrity)
2.  Append zeros to the buffer (restoring the pre-shift size)
3.  XOR-decrypt to reveal the routing block + inner encrypted layers
4.  Parse the routing block (next\_hop\_id + next\_hop\_HMAC)
5.  Re-randomize the ephemeral key (unlinkability)
6.  Forward the inner packet to the next hop

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../curve25519_dalek/index.html" class="mod"
title="mod curve25519_dalek"><code>curve25519_dalek</code></a>`;`

`pub use `<a href="../x25519_dalek/index.html" class="mod"
title="mod x25519_dalek"><code>x25519_dalek</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.SphinxConfig.html" class="struct"
title="struct sphinx_rs::SphinxConfig">SphinxConfig</a>  
Domain constants for Sphinx key derivation.

<a href="struct.SphinxPacket.html" class="struct"
title="struct sphinx_rs::SphinxPacket">SphinxPacket</a>  
A Sphinx packet on the wire.

<a href="struct.Surb.html" class="struct"
title="struct sphinx_rs::Surb">Surb</a>  
SURB: Single-Use Reply Block.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.SphinxError.html" class="enum"
title="enum sphinx_rs::SphinxError">SphinxError</a>  
Errors that can occur during Sphinx packet operations.

<a href="enum.SphinxProcessResult.html" class="enum"
title="enum sphinx_rs::SphinxProcessResult">SphinxProcessResult</a>  
Result of processing a Sphinx packet at a hop.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.DELIVER_MARKER.html" class="constant"
title="constant sphinx_rs::DELIVER_MARKER">DELIVER_MARKER</a>  
Marker for the final hop (placed in the next\_hop\_id field)

<a href="constant.MAC_SIZE.html" class="constant"
title="constant sphinx_rs::MAC_SIZE">MAC_SIZE</a>  
Size of the HMAC (BLAKE3 output)

<a href="constant.ROUTING_BLOCK_SIZE.html" class="constant"
title="constant sphinx_rs::ROUTING_BLOCK_SIZE">ROUTING_BLOCK_SIZE</a>  
Size of each hop’s routing block in the header (fixed size for Sphinx
correctness) Layout: \[next\_hop\_id\_len(1) \| next\_hop\_id(variable,
max 63) \| zeros \| next\_hop\_HMAC(32)\]

<a href="constant.SPHINX_HEADER_SIZE.html" class="constant"
title="constant sphinx_rs::SPHINX_HEADER_SIZE">SPHINX_HEADER_SIZE</a>  
Fixed-size header buffer (matches Sphinx spec)

<a href="constant.SPHINX_MAX_HOPS.html" class="constant"
title="constant sphinx_rs::SPHINX_MAX_HOPS">SPHINX_MAX_HOPS</a>  
Maximum route length

<a href="constant.SPHINX_MIN_HOPS.html" class="constant"
title="constant sphinx_rs::SPHINX_MIN_HOPS">SPHINX_MIN_HOPS</a>  
Minimum route length for real anonymity. 1 hop is just encryption; 2 is
the minimum for testability; **production should use ≥ 3 hops**.

<a href="constant.SPHINX_PAYLOAD_SIZE.html" class="constant"
title="constant sphinx_rs::SPHINX_PAYLOAD_SIZE">SPHINX_PAYLOAD_SIZE</a>  
Fixed-size payload

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.create_sphinx_packet.html" class="fn"
title="fn sphinx_rs::create_sphinx_packet">create_sphinx_packet</a>  
Create a multi-hop Sphinx packet.

<a href="fn.create_surb.html" class="fn"
title="fn sphinx_rs::create_surb">create_surb</a>  
Create a Single-Use Reply Block (SURB) for anonymous replies.

<a href="fn.decrypt_surb_reply.html" class="fn"
title="fn sphinx_rs::decrypt_surb_reply">decrypt_surb_reply</a>  
Decrypt an incoming SURB reply.

<a href="fn.process_sphinx_packet.html" class="fn"
title="fn sphinx_rs::process_sphinx_packet">process_sphinx_packet</a>  
Process a Sphinx packet at a hop.

<a href="fn.public_key_from_secret.html" class="fn"
title="fn sphinx_rs::public_key_from_secret">public_key_from_secret</a>  
Generate a public key from a raw secret scalar (NO clamping).

<a href="fn.use_surb.html" class="fn"
title="fn sphinx_rs::use_surb">use_surb</a>  
Use a SURB to send an anonymous reply.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.HopPublicKey.html" class="type"
title="type sphinx_rs::HopPublicKey">HopPublicKey</a>  
Type alias for X25519 public keys (32 bytes)

<a href="type.SphinxResult.html" class="type"
title="type sphinx_rs::SphinxResult">SphinxResult</a>  
Result type for Sphinx operations
