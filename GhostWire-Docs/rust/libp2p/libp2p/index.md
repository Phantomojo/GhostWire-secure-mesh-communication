Crate libp2p Copy item path

<span class="sub-heading"><a href="../src/libp2p/lib.rs.html#21-169" class="src">Source</a>
</span>

Expand description

libp2p is a modular peer-to-peer networking framework.

To learn more about the general libp2p multi-language framework visit
<https://libp2p.io>.

To get started with this libp2p implementation in Rust, please take a
look at the [`tutorials`](tutorials/index.html "mod libp2p::tutorials").
Further examples can be found in the
[examples](https://github.com/libp2p/rust-libp2p/tree/master/examples)
directory.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../bytes/index.html" class="mod"
title="mod bytes"><code>bytes</code></a>`;`

`pub use `<a href="../futures/index.html" class="mod"
title="mod futures"><code>futures</code></a>`;`

`pub use `<a href="../libp2p_identity/index.html" class="mod"
title="mod libp2p_identity"><code>libp2p_identity</code></a>` as identity;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="allow_block_list/index.html" class="mod"
title="mod libp2p::allow_block_list">allow_block_list</a>  
A libp2p module for managing allow and blocks lists to peers.

<a href="autonat/index.html" class="mod"
title="mod libp2p::autonat">autonat</a>  
<a href="connection_limits/index.html" class="mod"
title="mod libp2p::connection_limits">connection_limits</a>  
<a href="core/index.html" class="mod" title="mod libp2p::core">core</a>  
Transports, upgrades, multiplexing and node handling of *libp2p*.

<a href="dcutr/index.html" class="mod"
title="mod libp2p::dcutr">dcutr</a>  
Implementation of the [libp2p Direct Connection Upgrade through Relay
specification](https://github.com/libp2p/specs/blob/master/relay/DCUtR.md).

<a href="dns/index.html" class="mod" title="mod libp2p::dns">dns</a>  
[DNS name
resolution](https://github.com/libp2p/specs/blob/master/addressing/README.md#ip-and-name-resolution)

<a href="gossipsub/index.html" class="mod"
title="mod libp2p::gossipsub">gossipsub</a>  
Implementation of the
[Gossipsub](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/README.md)
protocol.

<a href="identify/index.html" class="mod"
title="mod libp2p::identify">identify</a>  
Implementation of the
[Identify](https://github.com/libp2p/specs/blob/master/identify/README.md)
protocol.

<a href="kad/index.html" class="mod" title="mod libp2p::kad">kad</a>  
Implementation of the libp2p-specific
[Kademlia](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)
protocol.

<a href="mdns/index.html" class="mod" title="mod libp2p::mdns">mdns</a>  
Implementation of the libp2p-specific
[mDNS](https://github.com/libp2p/specs/blob/master/discovery/mdns.md)
protocol.

<a href="multiaddr/index.html" class="mod"
title="mod libp2p::multiaddr">multiaddr</a>  
Implementation of [multiaddr](https://github.com/multiformats/multiaddr)
in Rust.

<a href="multihash/index.html" class="mod"
title="mod libp2p::multihash">multihash</a>  
Bare-minimum multihash data structure.

<a href="noise/index.html" class="mod"
title="mod libp2p::noise">noise</a>  
[Noise protocol framework](http://noiseprotocol.org/) support for
libp2p.

<a href="ping/index.html" class="mod" title="mod libp2p::ping">ping</a>  
This module implements the `/ipfs/ping/1.0.0` protocol.

<a href="relay/index.html" class="mod"
title="mod libp2p::relay">relay</a>  
Implementation of libp2p [circuit
relay](https://github.com/libp2p/specs/blob/master/relay/README.md)
protocol.

<a href="request_response/index.html" class="mod"
title="mod libp2p::request_response">request_response</a>  
Generic request/response protocols.

<a href="swarm/index.html" class="mod"
title="mod libp2p::swarm">swarm</a>  
High-level network manager.

<a href="tcp/index.html" class="mod" title="mod libp2p::tcp">tcp</a>  
Implementation of the libp2p
[`libp2p_core::Transport`](trait.Transport.html "trait libp2p::Transport")
trait for TCP/IP.

<a href="tutorials/index.html" class="mod"
title="mod libp2p::tutorials">tutorials</a>  
Rust-libp2p Tutorials to get started with.

<a href="yamux/index.html" class="mod"
title="mod libp2p::yamux">yamux</a>  
Implementation of the
[Yamux](https://github.com/hashicorp/yamux/blob/master/spec.md)
multiplexing protocol for libp2p.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.build_multiaddr.html" class="macro"
title="macro libp2p::build_multiaddr">build_multiaddr</a>  
Easy way for a user to create a `Multiaddr`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BehaviourBuilderError.html" class="struct"
title="struct libp2p::BehaviourBuilderError">BehaviourBuilderError</a>

<a href="struct.Multiaddr.html" class="struct"
title="struct libp2p::Multiaddr">Multiaddr</a>

Representation of a Multiaddr.

<a href="struct.PeerId.html" class="struct"
title="struct libp2p::PeerId">PeerId</a>

Identifier of a peer of the network.

<a href="struct.Stream.html" class="struct"
title="struct libp2p::Stream">Stream</a>

<a href="struct.StreamProtocol.html" class="struct"
title="struct libp2p::StreamProtocol">StreamProtocol</a>

Identifies a protocol for a stream.

<a href="struct.Swarm.html" class="struct"
title="struct libp2p::Swarm">Swarm</a>

Contains the state of the network, plus the way it should behave.

<a href="struct.SwarmBuilder.html" class="struct"
title="struct libp2p::SwarmBuilder">SwarmBuilder</a>

Build a [`Swarm`](struct.Swarm.html "struct libp2p::Swarm") by combining
an identity, a set of
[`Transport`](trait.Transport.html "trait libp2p::Transport")s and a
[`NetworkBehaviour`](swarm/trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour").

<a href="struct.TransportBuilderError.html" class="struct"
title="struct libp2p::TransportBuilderError">TransportBuilderError</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.TransportError.html" class="enum"
title="enum libp2p::TransportError">TransportError</a>  
An error during
[dialing](trait.Transport.html#tymethod.dial "method libp2p::Transport::dial")
or
[listening](trait.Transport.html#tymethod.listen_on "method libp2p::Transport::listen_on")
on a [`Transport`](trait.Transport.html "trait libp2p::Transport").

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.InboundUpgrade.html" class="trait"
title="trait libp2p::InboundUpgrade">InboundUpgrade</a>  
Possible upgrade on an inbound connection or substream.

<a href="trait.OutboundUpgrade.html" class="trait"
title="trait libp2p::OutboundUpgrade">OutboundUpgrade</a>  
Possible upgrade on an outbound connection or substream.

<a href="trait.Transport.html" class="trait"
title="trait libp2p::Transport">Transport</a>  
A transport provides connection-oriented communication between two peers
through ordered streams of data (i.e. connections).
