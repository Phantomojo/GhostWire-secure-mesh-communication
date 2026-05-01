Crate libp2p\_swarm Copy item path

<span class="sub-heading"><a href="../src/libp2p_swarm/lib.rs.html#21-2342" class="src">Source</a>
</span>

Expand description

High-level network manager.

A [`Swarm`](struct.Swarm.html "struct libp2p_swarm::Swarm") contains the
state of the network as a whole. The entire behaviour of a libp2p
network can be controlled through the `Swarm`. The `Swarm` struct
contains all active and pending connections to remotes and manages the
state of all the substreams that have been opened, and all the upgrades
that were built upon these substreams.

# <a href="#initializing-a-swarm" class="doc-anchor">§</a>Initializing a Swarm

Creating a `Swarm` requires three things:

1.  A network identity of the local node in form of a
    [`PeerId`](../libp2p_identity/peer_id/struct.PeerId.html "struct libp2p_identity::peer_id::PeerId").
2.  An implementation of the
    [`Transport`](../libp2p_core/transport/trait.Transport.html "trait libp2p_core::transport::Transport")
    trait. This is the type that will be used in order to reach nodes on
    the network based on their address. See the `transport` module for
    more information.
3.  An implementation of the
    [`NetworkBehaviour`](behaviour/trait.NetworkBehaviour.html "trait libp2p_swarm::behaviour::NetworkBehaviour")
    trait. This is a state machine that defines how the swarm should
    behave once it is connected to a node.

# <a href="#network-behaviour" class="doc-anchor">§</a>Network Behaviour

The
[`NetworkBehaviour`](behaviour/trait.NetworkBehaviour.html "trait libp2p_swarm::behaviour::NetworkBehaviour")
trait is implemented on types that indicate to the swarm how it should
behave. This includes which protocols are supported and which nodes to
try to connect to. It is the `NetworkBehaviour` that controls what
happens on the network. Multiple types that implement `NetworkBehaviour`
can be composed into a single behaviour.

# <a href="#protocols-handler" class="doc-anchor">§</a>Protocols Handler

The
[`ConnectionHandler`](handler/trait.ConnectionHandler.html "trait libp2p_swarm::handler::ConnectionHandler")
trait defines how each active connection to a remote should behave: how
to handle incoming substreams, which protocols are supported, when to
open a new outbound substream, etc.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use behaviour::`<a href="behaviour/struct.AddressChange.html" class="struct"
title="struct libp2p_swarm::behaviour::AddressChange"><code>AddressChange</code></a>`;`

`pub use behaviour::`<a href="behaviour/enum.CloseConnection.html" class="enum"
title="enum libp2p_swarm::behaviour::CloseConnection"><code>CloseConnection</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.ConnectionClosed.html" class="struct"
title="struct libp2p_swarm::behaviour::ConnectionClosed"><code>ConnectionClosed</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.DialFailure.html" class="struct"
title="struct libp2p_swarm::behaviour::DialFailure"><code>DialFailure</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.ExpiredListenAddr.html" class="struct"
title="struct libp2p_swarm::behaviour::ExpiredListenAddr"><code>ExpiredListenAddr</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.ExternalAddrExpired.html" class="struct"
title="struct libp2p_swarm::behaviour::ExternalAddrExpired"><code>ExternalAddrExpired</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.ExternalAddresses.html" class="struct"
title="struct libp2p_swarm::behaviour::ExternalAddresses"><code>ExternalAddresses</code></a>`;`

`pub use behaviour::`<a href="behaviour/enum.FromSwarm.html" class="enum"
title="enum libp2p_swarm::behaviour::FromSwarm"><code>FromSwarm</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.ListenAddresses.html" class="struct"
title="struct libp2p_swarm::behaviour::ListenAddresses"><code>ListenAddresses</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.ListenFailure.html" class="struct"
title="struct libp2p_swarm::behaviour::ListenFailure"><code>ListenFailure</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.ListenerClosed.html" class="struct"
title="struct libp2p_swarm::behaviour::ListenerClosed"><code>ListenerClosed</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.ListenerError.html" class="struct"
title="struct libp2p_swarm::behaviour::ListenerError"><code>ListenerError</code></a>`;`

`pub use behaviour::`<a href="behaviour/trait.NetworkBehaviour.html" class="trait"
title="trait libp2p_swarm::behaviour::NetworkBehaviour"><code>NetworkBehaviour</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.NewExternalAddrCandidate.html" class="struct"
title="struct libp2p_swarm::behaviour::NewExternalAddrCandidate"><code>NewExternalAddrCandidate</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.NewExternalAddrOfPeer.html" class="struct"
title="struct libp2p_swarm::behaviour::NewExternalAddrOfPeer"><code>NewExternalAddrOfPeer</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.NewListenAddr.html" class="struct"
title="struct libp2p_swarm::behaviour::NewListenAddr"><code>NewListenAddr</code></a>`;`

`pub use behaviour::`<a href="behaviour/enum.NotifyHandler.html" class="enum"
title="enum libp2p_swarm::behaviour::NotifyHandler"><code>NotifyHandler</code></a>`;`

`pub use behaviour::`<a href="behaviour/struct.PeerAddresses.html" class="struct"
title="struct libp2p_swarm::behaviour::PeerAddresses"><code>PeerAddresses</code></a>`;`

`pub use behaviour::`<a href="behaviour/enum.ToSwarm.html" class="enum"
title="enum libp2p_swarm::behaviour::ToSwarm"><code>ToSwarm</code></a>`;`

`pub use handler::`<a href="handler/trait.ConnectionHandler.html" class="trait"
title="trait libp2p_swarm::handler::ConnectionHandler"><code>ConnectionHandler</code></a>`;`

`pub use handler::`<a href="handler/enum.ConnectionHandlerEvent.html" class="enum"
title="enum libp2p_swarm::handler::ConnectionHandlerEvent"><code>ConnectionHandlerEvent</code></a>`;`

`pub use handler::`<a href="handler/struct.ConnectionHandlerSelect.html" class="struct"
title="struct libp2p_swarm::handler::ConnectionHandlerSelect"><code>ConnectionHandlerSelect</code></a>`;`

`pub use handler::`<a href="handler/struct.OneShotHandler.html" class="struct"
title="struct libp2p_swarm::handler::OneShotHandler"><code>OneShotHandler</code></a>`;`

`pub use handler::`<a href="handler/struct.OneShotHandlerConfig.html" class="struct"
title="struct libp2p_swarm::handler::OneShotHandlerConfig"><code>OneShotHandlerConfig</code></a>`;`

`pub use handler::`<a href="handler/enum.StreamUpgradeError.html" class="enum"
title="enum libp2p_swarm::handler::StreamUpgradeError"><code>StreamUpgradeError</code></a>`;`

`pub use handler::`<a href="handler/struct.SubstreamProtocol.html" class="struct"
title="struct libp2p_swarm::handler::SubstreamProtocol"><code>SubstreamProtocol</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="behaviour/index.html" class="mod"
title="mod libp2p_swarm::behaviour">behaviour</a>  
<a href="dial_opts/index.html" class="mod"
title="mod libp2p_swarm::dial_opts">dial_opts</a>  
<a href="dummy/index.html" class="mod"
title="mod libp2p_swarm::dummy">dummy</a>  
<a href="handler/index.html" class="mod"
title="mod libp2p_swarm::handler">handler</a>  
Once a connection to a remote peer is established, a
[`ConnectionHandler`](handler/trait.ConnectionHandler.html "trait libp2p_swarm::handler::ConnectionHandler")
negotiates and handles one or more specific protocols on the connection.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Config.html" class="struct"
title="struct libp2p_swarm::Config">Config</a>  
<a href="struct.ConnectionCounters.html" class="struct"
title="struct libp2p_swarm::ConnectionCounters">ConnectionCounters</a>  
Network connection information.

<a href="struct.ConnectionDenied.html" class="struct"
title="struct libp2p_swarm::ConnectionDenied">ConnectionDenied</a>  
A connection was denied.

<a href="struct.ConnectionId.html" class="struct"
title="struct libp2p_swarm::ConnectionId">ConnectionId</a>  
Connection identifier.

<a href="struct.InvalidProtocol.html" class="struct"
title="struct libp2p_swarm::InvalidProtocol">InvalidProtocol</a>  
<a href="struct.ListenOpts.html" class="struct"
title="struct libp2p_swarm::ListenOpts">ListenOpts</a>  
<a href="struct.NetworkInfo.html" class="struct"
title="struct libp2p_swarm::NetworkInfo">NetworkInfo</a>  
Information about the connections obtained by
[`Swarm::network_info()`](struct.Swarm.html#method.network_info "method libp2p_swarm::Swarm::network_info").

<a href="struct.Stream.html" class="struct"
title="struct libp2p_swarm::Stream">Stream</a>  
<a href="struct.StreamProtocol.html" class="struct"
title="struct libp2p_swarm::StreamProtocol">StreamProtocol</a>  
Identifies a protocol for a stream.

<a href="struct.SupportedProtocols.html" class="struct"
title="struct libp2p_swarm::SupportedProtocols">SupportedProtocols</a>  
<a href="struct.Swarm.html" class="struct"
title="struct libp2p_swarm::Swarm">Swarm</a>  
Contains the state of the network, plus the way it should behave.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ConnectionError.html" class="enum"
title="enum libp2p_swarm::ConnectionError">ConnectionError</a>  
Errors that can occur in the context of an established `Connection`.

<a href="enum.DialError.html" class="enum"
title="enum libp2p_swarm::DialError">DialError</a>  
Possible errors when trying to establish or upgrade an outbound
connection.

<a href="enum.ListenError.html" class="enum"
title="enum libp2p_swarm::ListenError">ListenError</a>  
Possible errors when upgrading an inbound connection.

<a href="enum.SwarmEvent.html" class="enum"
title="enum libp2p_swarm::SwarmEvent">SwarmEvent</a>  
Event generated by the `Swarm`.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Executor.html" class="trait"
title="trait libp2p_swarm::Executor">Executor</a>  
Implemented on objects that can run a `Future` in the background.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.THandler.html" class="type"
title="type libp2p_swarm::THandler">THandler</a>  
[`ConnectionHandler`](handler/trait.ConnectionHandler.html "trait libp2p_swarm::handler::ConnectionHandler")
of the
[`NetworkBehaviour`](behaviour/trait.NetworkBehaviour.html "trait libp2p_swarm::behaviour::NetworkBehaviour")
for all the protocols the
[`NetworkBehaviour`](behaviour/trait.NetworkBehaviour.html "trait libp2p_swarm::behaviour::NetworkBehaviour")
supports.

<a href="type.THandlerInEvent.html" class="type"
title="type libp2p_swarm::THandlerInEvent">THandlerInEvent</a>  
Custom event that can be received by the
[`ConnectionHandler`](handler/trait.ConnectionHandler.html "trait libp2p_swarm::handler::ConnectionHandler")
of the
[`NetworkBehaviour`](behaviour/trait.NetworkBehaviour.html "trait libp2p_swarm::behaviour::NetworkBehaviour").

<a href="type.THandlerOutEvent.html" class="type"
title="type libp2p_swarm::THandlerOutEvent">THandlerOutEvent</a>  
Custom event that can be produced by the
[`ConnectionHandler`](handler/trait.ConnectionHandler.html "trait libp2p_swarm::handler::ConnectionHandler")
of the
[`NetworkBehaviour`](behaviour/trait.NetworkBehaviour.html "trait libp2p_swarm::behaviour::NetworkBehaviour").

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.NetworkBehaviour.html" class="derive"
title="derive libp2p_swarm::NetworkBehaviour">NetworkBehaviour</a>  
Generates a delegating `NetworkBehaviour` implementation for the struct
this is used for. See the trait documentation for better description.
