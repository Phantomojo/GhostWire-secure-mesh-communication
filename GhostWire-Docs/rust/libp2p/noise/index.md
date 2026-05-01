[libp2p](../index.html)

Crate noise Copy item path

<span class="sub-heading"><a href="../../src/libp2p_noise/lib.rs.html#21-266"
class="src">Source</a> </span>

Expand description

[Noise protocol framework](http://noiseprotocol.org/) support for
libp2p.

> **Note**: This crate is still experimental and subject to major
> breaking changes both on the API and the wire protocol.

This crate provides `libp2p_core::InboundUpgrade` and
`libp2p_core::OutboundUpgrade` implementations for various noise
handshake patterns (currently `IK`, `IX`, and `XX`) over a particular
choice of Diffie–Hellman key agreement (currently only X25519).

> **Note**: Only the `XX` handshake pattern is currently guaranteed to
> provide interoperability with other libp2p implementations.

All upgrades produce as output a pair, consisting of the remote’s static
public key and a `NoiseOutput` which represents the established
cryptographic session with the remote, implementing
`futures::io::AsyncRead` and `futures::io::AsyncWrite`.

# <a href="#usage" class="doc-anchor">§</a>Usage

Example:

``` {.rust .rust-example-rendered}
use libp2p_core::{transport::MemoryTransport, upgrade, Transport};
use libp2p_identity as identity;
use libp2p_noise as noise;

let id_keys = identity::Keypair::generate_ed25519();
let noise = noise::Config::new(&id_keys).unwrap();
let builder = MemoryTransport::default()
    .upgrade(upgrade::Version::V1)
    .authenticate(noise);
// let transport = builder.multiplex(...);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Config.html" class="struct"
title="struct libp2p::noise::Config">Config</a>  
The configuration for the noise handshake.

<a href="struct.DecodeError.html" class="struct"
title="struct libp2p::noise::DecodeError">DecodeError</a>  
<a href="struct.Output.html" class="struct"
title="struct libp2p::noise::Output">Output</a>  
A noise session to a remote.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum libp2p::noise::Error">Error</a>  
libp2p\_noise error type.
