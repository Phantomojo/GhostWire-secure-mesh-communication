Crate libp2p\_core Copy item path

<span class="sub-heading"><a href="../src/libp2p_core/lib.rs.html#21-77" class="src">Source</a>
</span>

Expand description

Transports, upgrades, multiplexing and node handling of *libp2p*.

The main concepts of libp2p-core are:

-   The
    [`Transport`](transport/trait.Transport.html "trait libp2p_core::transport::Transport")
    trait defines how to reach a remote node or listen for incoming
    remote connections. See the
    [`transport`](transport/index.html "mod libp2p_core::transport")
    module.
-   The
    [`StreamMuxer`](muxing/trait.StreamMuxer.html "trait libp2p_core::muxing::StreamMuxer")
    trait is implemented on structs that hold a connection to a remote
    and can subdivide this connection into multiple substreams. See the
    [`muxing`](muxing/index.html "mod libp2p_core::muxing") module.
-   The
    [`UpgradeInfo`](upgrade/trait.UpgradeInfo.html "trait libp2p_core::upgrade::UpgradeInfo"),
    [`InboundUpgrade`](upgrade/trait.InboundUpgrade.html "trait libp2p_core::upgrade::InboundUpgrade")
    and
    [`OutboundUpgrade`](upgrade/trait.OutboundUpgrade.html "trait libp2p_core::upgrade::OutboundUpgrade")
    traits define how to upgrade each individual substream to use a
    protocol. See the `upgrade` module.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use connection::`<a href="connection/enum.ConnectedPoint.html" class="enum"
title="enum libp2p_core::connection::ConnectedPoint"><code>ConnectedPoint</code></a>`;`

`pub use connection::`<a href="connection/enum.Endpoint.html" class="enum"
title="enum libp2p_core::connection::Endpoint"><code>Endpoint</code></a>`;`

`pub use muxing::`<a href="muxing/trait.StreamMuxer.html" class="trait"
title="trait libp2p_core::muxing::StreamMuxer"><code>StreamMuxer</code></a>`;`

`pub use peer_record::`<a href="peer_record/struct.PeerRecord.html" class="struct"
title="struct libp2p_core::peer_record::PeerRecord"><code>PeerRecord</code></a>`;`

`pub use signed_envelope::`<a href="signed_envelope/struct.SignedEnvelope.html" class="struct"
title="struct libp2p_core::signed_envelope::SignedEnvelope"><code>SignedEnvelope</code></a>`;`

`pub use transport::`<a href="transport/trait.Transport.html" class="trait"
title="trait libp2p_core::transport::Transport"><code>Transport</code></a>`;`

`pub use upgrade::`<a href="upgrade/trait.InboundUpgrade.html" class="trait"
title="trait libp2p_core::upgrade::InboundUpgrade"><code>InboundUpgrade</code></a>`;`

`pub use upgrade::`<a href="upgrade/trait.OutboundUpgrade.html" class="trait"
title="trait libp2p_core::upgrade::OutboundUpgrade"><code>OutboundUpgrade</code></a>`;`

`pub use upgrade::`<a href="upgrade/trait.UpgradeInfo.html" class="trait"
title="trait libp2p_core::upgrade::UpgradeInfo"><code>UpgradeInfo</code></a>`;`

`pub use `<a href="../multiaddr/index.html" class="mod"
title="mod multiaddr"><code>multiaddr</code></a>`;`

`pub use `<a href="../multihash/index.html" class="mod"
title="mod multihash"><code>multihash</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="connection/index.html" class="mod"
title="mod libp2p_core::connection">connection</a>

<a href="either/index.html" class="mod"
title="mod libp2p_core::either">either</a>

<a href="muxing/index.html" class="mod"
title="mod libp2p_core::muxing">muxing</a>

Muxing is the process of splitting a connection into multiple
substreams.

<a href="peer_record/index.html" class="mod"
title="mod libp2p_core::peer_record">peer_record</a>

<a href="signed_envelope/index.html" class="mod"
title="mod libp2p_core::signed_envelope">signed_envelope</a>

<a href="transport/index.html" class="mod"
title="mod libp2p_core::transport">transport</a>

Connection-oriented communication channels.

<a href="upgrade/index.html" class="mod"
title="mod libp2p_core::upgrade">upgrade</a>

Contains everything related to upgrading a connection or a substream to
use a protocol.

<a href="util/index.html" class="mod"
title="mod libp2p_core::util">util</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DecodeError.html" class="struct"
title="struct libp2p_core::DecodeError">DecodeError</a>  
<a href="struct.Multiaddr.html" class="struct"
title="struct libp2p_core::Multiaddr">Multiaddr</a>  
Representation of a Multiaddr.

<a href="struct.PeerId.html" class="struct"
title="struct libp2p_core::PeerId">PeerId</a>  
Identifier of a peer of the network.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Negotiated.html" class="type"
title="type libp2p_core::Negotiated">Negotiated</a>
