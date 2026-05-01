[libp2p](../index.html)

Crate core Copy item path

<span class="sub-heading"><a href="../../src/libp2p_core/lib.rs.html#21-77" class="src">Source</a>
</span>

Expand description

Transports, upgrades, multiplexing and node handling of *libp2p*.

The main concepts of libp2p-core are:

-   The [`Transport`](../trait.Transport.html "trait libp2p::Transport")
    trait defines how to reach a remote node or listen for incoming
    remote connections. See the
    [`transport`](transport/index.html "mod libp2p::core::transport")
    module.
-   The
    [`StreamMuxer`](trait.StreamMuxer.html "trait libp2p::core::StreamMuxer")
    trait is implemented on structs that hold a connection to a remote
    and can subdivide this connection into multiple substreams. See the
    [`muxing`](muxing/index.html "mod libp2p::core::muxing") module.
-   The
    [`UpgradeInfo`](trait.UpgradeInfo.html "trait libp2p::core::UpgradeInfo"),
    [`InboundUpgrade`](../trait.InboundUpgrade.html "trait libp2p::InboundUpgrade")
    and
    [`OutboundUpgrade`](../trait.OutboundUpgrade.html "trait libp2p::OutboundUpgrade")
    traits define how to upgrade each individual substream to use a
    protocol. See the `upgrade` module.

# Modules<a href="#modules" class="anchor">§</a>

<a href="connection/index.html" class="mod"
title="mod libp2p::core::connection">connection</a>

<a href="either/index.html" class="mod"
title="mod libp2p::core::either">either</a>

<a href="multiaddr/index.html" class="mod"
title="mod libp2p::core::multiaddr">multiaddr</a>

Implementation of [multiaddr](https://github.com/multiformats/multiaddr)
in Rust.

<a href="muxing/index.html" class="mod"
title="mod libp2p::core::muxing">muxing</a>

Muxing is the process of splitting a connection into multiple
substreams.

<a href="peer_record/index.html" class="mod"
title="mod libp2p::core::peer_record">peer_record</a>

<a href="signed_envelope/index.html" class="mod"
title="mod libp2p::core::signed_envelope">signed_envelope</a>

<a href="transport/index.html" class="mod"
title="mod libp2p::core::transport">transport</a>

Connection-oriented communication channels.

<a href="upgrade/index.html" class="mod"
title="mod libp2p::core::upgrade">upgrade</a>

Contains everything related to upgrading a connection or a substream to
use a protocol.

<a href="util/index.html" class="mod"
title="mod libp2p::core::util">util</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DecodeError.html" class="struct"
title="struct libp2p::core::DecodeError">DecodeError</a>  
<a href="struct.Multiaddr.html" class="struct"
title="struct libp2p::core::Multiaddr">Multiaddr</a>  
Representation of a Multiaddr.

<a href="struct.PeerId.html" class="struct"
title="struct libp2p::core::PeerId">PeerId</a>  
Identifier of a peer of the network.

<a href="struct.PeerRecord.html" class="struct"
title="struct libp2p::core::PeerRecord">PeerRecord</a>  
Represents a peer routing record.

<a href="struct.SignedEnvelope.html" class="struct"
title="struct libp2p::core::SignedEnvelope">SignedEnvelope</a>  
A signed envelope contains an arbitrary byte string payload, a signature
of the payload, and the public key that can be used to verify the
signature.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ConnectedPoint.html" class="enum"
title="enum libp2p::core::ConnectedPoint">ConnectedPoint</a>  
The endpoint roles associated with an established peer-to-peer
connection.

<a href="enum.Endpoint.html" class="enum"
title="enum libp2p::core::Endpoint">Endpoint</a>  
The endpoint roles associated with a peer-to-peer communication channel.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.InboundUpgrade.html" class="trait"
title="trait libp2p::core::InboundUpgrade">InboundUpgrade</a>  
Possible upgrade on an inbound connection or substream.

<a href="trait.OutboundUpgrade.html" class="trait"
title="trait libp2p::core::OutboundUpgrade">OutboundUpgrade</a>  
Possible upgrade on an outbound connection or substream.

<a href="trait.StreamMuxer.html" class="trait"
title="trait libp2p::core::StreamMuxer">StreamMuxer</a>  
Provides multiplexing for a connection by allowing users to open
substreams.

<a href="trait.Transport.html" class="trait"
title="trait libp2p::core::Transport">Transport</a>  
A transport provides connection-oriented communication between two peers
through ordered streams of data (i.e. connections).

<a href="trait.UpgradeInfo.html" class="trait"
title="trait libp2p::core::UpgradeInfo">UpgradeInfo</a>  
Common trait for upgrades that can be applied on inbound substreams,
outbound substreams, or both.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Negotiated.html" class="type"
title="type libp2p::core::Negotiated">Negotiated</a>
