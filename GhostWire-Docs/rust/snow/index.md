Crate snow Copy item path

<span class="sub-heading"><a href="../src/snow/lib.rs.html#1-89" class="src">Source</a>
</span>

Expand description

The `snow` crate is a straightforward, Hard To Fuck Up™ Noise Protocol
implementation.

Read the [Noise Protocol Framework
Spec](https://noiseprotocol.org/noise.html) for more information.

The typical usage flow is to use
[`Builder`](struct.Builder.html "struct snow::Builder") to construct a
[`HandshakeState`](struct.HandshakeState.html "struct snow::HandshakeState"),
where you will complete the handshake phase and convert into either a
[`TransportState`](struct.TransportState.html "struct snow::TransportState")
or
[`StatelessTransportState`](struct.StatelessTransportState.html "struct snow::StatelessTransportState").

# <a href="#examples" class="doc-anchor">§</a>Examples

See `examples/simple.rs` for a more complete TCP client/server example
with static keys.

``` {.rust .rust-example-rendered}
static PATTERN: &'static str = "Noise_NN_25519_ChaChaPoly_BLAKE2s";

let mut initiator = snow::Builder::new(PATTERN.parse()?)
    .build_initiator()?;
let mut responder = snow::Builder::new(PATTERN.parse()?)
    .build_responder()?;

let (mut read_buf, mut first_msg, mut second_msg) =
    ([0u8; 1024], [0u8; 1024], [0u8; 1024]);

// -> e
let len = initiator.write_message(&[], &mut first_msg)?;

// responder processes the first message...
responder.read_message(&first_msg[..len], &mut read_buf)?;

// <- e, ee
let len = responder.write_message(&[], &mut second_msg)?;

// initiator processes the response...
initiator.read_message(&second_msg[..len], &mut read_buf)?;

// NN handshake complete, transition into transport mode.
let initiator = initiator.into_transport_mode();
let responder = responder.into_transport_mode();
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::error::`<a href="error/enum.Error.html" class="enum"
title="enum snow::error::Error"><code>Error</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="error/index.html" class="mod" title="mod snow::error">error</a>  
All error types used by Snow operations.

<a href="params/index.html" class="mod"
title="mod snow::params">params</a>  
All structures related to Noise parameter definitions (cryptographic
primitive choices, protocol patterns/names)

<a href="resolvers/index.html" class="mod"
title="mod snow::resolvers">resolvers</a>  
The wrappers around the default collection of cryptography and entropy
providers.

<a href="types/index.html" class="mod" title="mod snow::types">types</a>  
The traits for cryptographic implementations that can be used by Noise.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Builder.html" class="struct"
title="struct snow::Builder">Builder</a>  
Generates a
[`HandshakeState`](struct.HandshakeState.html "struct snow::HandshakeState")
and also validates that all the prerequisites for the given parameters
are satisfied.

<a href="struct.HandshakeState.html" class="struct"
title="struct snow::HandshakeState">HandshakeState</a>  
A state machine encompassing the handshake phase of a Noise session.

<a href="struct.Keypair.html" class="struct"
title="struct snow::Keypair">Keypair</a>  
A keypair object returned by
[`Builder::generate_keypair()`](struct.Builder.html#method.generate_keypair "method snow::Builder::generate_keypair")

<a href="struct.StatelessTransportState.html" class="struct"
title="struct snow::StatelessTransportState">StatelessTransportState</a>  
A state machine encompassing the transport phase of a Noise session,
using the two `CipherState`s (for sending and receiving) that were
spawned from the `SymmetricState`’s `Split()` method, called after a
handshake has been finished.

<a href="struct.TransportState.html" class="struct"
title="struct snow::TransportState">TransportState</a>  
A state machine encompassing the transport phase of a Noise session,
using the two `CipherState`s (for sending and receiving) that were
spawned from the `SymmetricState`’s `Split()` method, called after a
handshake has been finished.
