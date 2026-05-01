[libp2p](../index.html)

Crate swarm Copy item path

<span class="sub-heading"><a href="../../src/libp2p_swarm/lib.rs.html#21-2342"
class="src">Source</a> </span>

Expand description

High-level network manager.

A [`Swarm`](../struct.Swarm.html "struct libp2p::Swarm") contains the
state of the network as a whole. The entire behaviour of a libp2p
network can be controlled through the `Swarm`. The `Swarm` struct
contains all active and pending connections to remotes and manages the
state of all the substreams that have been opened, and all the upgrades
that were built upon these substreams.

# <a href="#initializing-a-swarm" class="doc-anchor">§</a>Initializing a Swarm

Creating a `Swarm` requires three things:

1.  A network identity of the local node in form of a
    [`PeerId`](../struct.PeerId.html "struct libp2p::PeerId").
2.  An implementation of the
    [`Transport`](../trait.Transport.html "trait libp2p::Transport")
    trait. This is the type that will be used in order to reach nodes on
    the network based on their address. See the `transport` module for
    more information.
3.  An implementation of the
    [`NetworkBehaviour`](trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
    trait. This is a state machine that defines how the swarm should
    behave once it is connected to a node.

# <a href="#network-behaviour" class="doc-anchor">§</a>Network Behaviour

The
[`NetworkBehaviour`](trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
trait is implemented on types that indicate to the swarm how it should
behave. This includes which protocols are supported and which nodes to
try to connect to. It is the `NetworkBehaviour` that controls what
happens on the network. Multiple types that implement `NetworkBehaviour`
can be composed into a single behaviour.

# <a href="#protocols-handler" class="doc-anchor">§</a>Protocols Handler

The
[`ConnectionHandler`](trait.ConnectionHandler.html "trait libp2p::swarm::ConnectionHandler")
trait defines how each active connection to a remote should behave: how
to handle incoming substreams, which protocols are supported, when to
open a new outbound substream, etc.

# Modules<a href="#modules" class="anchor">§</a>

<a href="behaviour/index.html" class="mod"
title="mod libp2p::swarm::behaviour">behaviour</a>  
<a href="dial_opts/index.html" class="mod"
title="mod libp2p::swarm::dial_opts">dial_opts</a>  
<a href="dummy/index.html" class="mod"
title="mod libp2p::swarm::dummy">dummy</a>  
<a href="handler/index.html" class="mod"
title="mod libp2p::swarm::handler">handler</a>  
Once a connection to a remote peer is established, a
[`ConnectionHandler`](trait.ConnectionHandler.html "trait libp2p::swarm::ConnectionHandler")
negotiates and handles one or more specific protocols on the connection.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AddressChange.html" class="struct"
title="struct libp2p::swarm::AddressChange">AddressChange</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour that the
[`ConnectedPoint`](../core/enum.ConnectedPoint.html "enum libp2p::core::ConnectedPoint")
of an existing connection has changed.

<a href="struct.Config.html" class="struct"
title="struct libp2p::swarm::Config">Config</a>  
<a href="struct.ConnectionClosed.html" class="struct"
title="struct libp2p::swarm::ConnectionClosed">ConnectionClosed</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour about a closed connection to a peer.

<a href="struct.ConnectionCounters.html" class="struct"
title="struct libp2p::swarm::ConnectionCounters">ConnectionCounters</a>  
Network connection information.

<a href="struct.ConnectionDenied.html" class="struct"
title="struct libp2p::swarm::ConnectionDenied">ConnectionDenied</a>  
A connection was denied.

<a href="struct.ConnectionHandlerSelect.html" class="struct"
title="struct libp2p::swarm::ConnectionHandlerSelect">ConnectionHandlerSelect</a>  
Implementation of
[`ConnectionHandler`](trait.ConnectionHandler.html "trait libp2p::swarm::ConnectionHandler")
that combines two protocols into one.

<a href="struct.ConnectionId.html" class="struct"
title="struct libp2p::swarm::ConnectionId">ConnectionId</a>  
Connection identifier.

<a href="struct.DialFailure.html" class="struct"
title="struct libp2p::swarm::DialFailure">DialFailure</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour that the dial to a known or unknown
node failed.

<a href="struct.ExpiredListenAddr.html" class="struct"
title="struct libp2p::swarm::ExpiredListenAddr">ExpiredListenAddr</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour that a multiaddr we were listening on
has expired, which means that we are no longer listening on it.

<a href="struct.ExternalAddrExpired.html" class="struct"
title="struct libp2p::swarm::ExternalAddrExpired">ExternalAddrExpired</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour that an external address was removed.

<a href="struct.ExternalAddresses.html" class="struct"
title="struct libp2p::swarm::ExternalAddresses">ExternalAddresses</a>  
Utility struct for tracking the external addresses of a
[`Swarm`](../struct.Swarm.html "struct libp2p::Swarm").

<a href="struct.InvalidProtocol.html" class="struct"
title="struct libp2p::swarm::InvalidProtocol">InvalidProtocol</a>  
<a href="struct.ListenAddresses.html" class="struct"
title="struct libp2p::swarm::ListenAddresses">ListenAddresses</a>  
Utility struct for tracking the addresses a
[`Swarm`](../struct.Swarm.html "struct libp2p::Swarm") is listening on.

<a href="struct.ListenFailure.html" class="struct"
title="struct libp2p::swarm::ListenFailure">ListenFailure</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour that an error happened on an incoming
connection during its initial handshake.

<a href="struct.ListenOpts.html" class="struct"
title="struct libp2p::swarm::ListenOpts">ListenOpts</a>  
<a href="struct.ListenerClosed.html" class="struct"
title="struct libp2p::swarm::ListenerClosed">ListenerClosed</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour that a listener closed.

<a href="struct.ListenerError.html" class="struct"
title="struct libp2p::swarm::ListenerError">ListenerError</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour that a listener experienced an error.

<a href="struct.NetworkInfo.html" class="struct"
title="struct libp2p::swarm::NetworkInfo">NetworkInfo</a>  
Information about the connections obtained by
[`Swarm::network_info()`](../struct.Swarm.html#method.network_info "method libp2p::Swarm::network_info").

<a href="struct.NewExternalAddrCandidate.html" class="struct"
title="struct libp2p::swarm::NewExternalAddrCandidate">NewExternalAddrCandidate</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour about a new candidate for an external
address for us.

<a href="struct.NewExternalAddrOfPeer.html" class="struct"
title="struct libp2p::swarm::NewExternalAddrOfPeer">NewExternalAddrOfPeer</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour that a new external address for a
remote peer was detected.

<a href="struct.NewListenAddr.html" class="struct"
title="struct libp2p::swarm::NewListenAddr">NewListenAddr</a>  
[`FromSwarm`](enum.FromSwarm.html "enum libp2p::swarm::FromSwarm")
variant that informs the behaviour that we have started listening on a
new multiaddr.

<a href="struct.OneShotHandler.html" class="struct"
title="struct libp2p::swarm::OneShotHandler">OneShotHandler</a>  
A
[`ConnectionHandler`](trait.ConnectionHandler.html "trait libp2p::swarm::ConnectionHandler")
that opens a new substream for each request.

<a href="struct.OneShotHandlerConfig.html" class="struct"
title="struct libp2p::swarm::OneShotHandlerConfig">OneShotHandlerConfig</a>  
Configuration parameters for the `OneShotHandler`

<a href="struct.PeerAddresses.html" class="struct"
title="struct libp2p::swarm::PeerAddresses">PeerAddresses</a>  
Struct for tracking peers’ external addresses of the
[`Swarm`](../struct.Swarm.html "struct libp2p::Swarm").

<a href="struct.Stream.html" class="struct"
title="struct libp2p::swarm::Stream">Stream</a>  
<a href="struct.StreamProtocol.html" class="struct"
title="struct libp2p::swarm::StreamProtocol">StreamProtocol</a>  
Identifies a protocol for a stream.

<a href="struct.SubstreamProtocol.html" class="struct"
title="struct libp2p::swarm::SubstreamProtocol">SubstreamProtocol</a>  
Configuration of inbound or outbound substream protocol(s) for a
[`ConnectionHandler`](trait.ConnectionHandler.html "trait libp2p::swarm::ConnectionHandler").

<a href="struct.SupportedProtocols.html" class="struct"
title="struct libp2p::swarm::SupportedProtocols">SupportedProtocols</a>  
<a href="struct.Swarm.html" class="struct"
title="struct libp2p::swarm::Swarm">Swarm</a>  
Contains the state of the network, plus the way it should behave.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.CloseConnection.html" class="enum"
title="enum libp2p::swarm::CloseConnection">CloseConnection</a>  
The options which connections to close.

<a href="enum.ConnectionError.html" class="enum"
title="enum libp2p::swarm::ConnectionError">ConnectionError</a>  
Errors that can occur in the context of an established `Connection`.

<a href="enum.ConnectionHandlerEvent.html" class="enum"
title="enum libp2p::swarm::ConnectionHandlerEvent">ConnectionHandlerEvent</a>  
Event produced by a handler.

<a href="enum.DialError.html" class="enum"
title="enum libp2p::swarm::DialError">DialError</a>  
Possible errors when trying to establish or upgrade an outbound
connection.

<a href="enum.FromSwarm.html" class="enum"
title="enum libp2p::swarm::FromSwarm">FromSwarm</a>  
Enumeration with the list of the possible events to pass to
[`on_swarm_event`](trait.NetworkBehaviour.html#tymethod.on_swarm_event "method libp2p::swarm::NetworkBehaviour::on_swarm_event").

<a href="enum.ListenError.html" class="enum"
title="enum libp2p::swarm::ListenError">ListenError</a>  
Possible errors when upgrading an inbound connection.

<a href="enum.NotifyHandler.html" class="enum"
title="enum libp2p::swarm::NotifyHandler">NotifyHandler</a>  
The options w.r.t. which connection handler to notify of an event.

<a href="enum.StreamUpgradeError.html" class="enum"
title="enum libp2p::swarm::StreamUpgradeError">StreamUpgradeError</a>  
Error that can happen on an outbound substream opening attempt.

<a href="enum.SwarmEvent.html" class="enum"
title="enum libp2p::swarm::SwarmEvent">SwarmEvent</a>  
Event generated by the `Swarm`.

<a href="enum.ToSwarm.html" class="enum"
title="enum libp2p::swarm::ToSwarm">ToSwarm</a>  
A command issued from a
[`NetworkBehaviour`](trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
for the [`Swarm`](../struct.Swarm.html "struct libp2p::Swarm").

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ConnectionHandler.html" class="trait"
title="trait libp2p::swarm::ConnectionHandler">ConnectionHandler</a>  
A handler for a set of protocols used on a connection with a remote.

<a href="trait.Executor.html" class="trait"
title="trait libp2p::swarm::Executor">Executor</a>  
Implemented on objects that can run a `Future` in the background.

<a href="trait.NetworkBehaviour.html" class="trait"
title="trait libp2p::swarm::NetworkBehaviour">NetworkBehaviour</a>  
A
[`NetworkBehaviour`](trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
defines the behaviour of the local node on the network.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.THandler.html" class="type"
title="type libp2p::swarm::THandler">THandler</a>  
[`ConnectionHandler`](trait.ConnectionHandler.html "trait libp2p::swarm::ConnectionHandler")
of the
[`NetworkBehaviour`](trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
for all the protocols the
[`NetworkBehaviour`](trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
supports.

<a href="type.THandlerInEvent.html" class="type"
title="type libp2p::swarm::THandlerInEvent">THandlerInEvent</a>  
Custom event that can be received by the
[`ConnectionHandler`](trait.ConnectionHandler.html "trait libp2p::swarm::ConnectionHandler")
of the
[`NetworkBehaviour`](trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour").

<a href="type.THandlerOutEvent.html" class="type"
title="type libp2p::swarm::THandlerOutEvent">THandlerOutEvent</a>  
Custom event that can be produced by the
[`ConnectionHandler`](trait.ConnectionHandler.html "trait libp2p::swarm::ConnectionHandler")
of the
[`NetworkBehaviour`](trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour").

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.NetworkBehaviour.html" class="derive"
title="derive libp2p::swarm::NetworkBehaviour">NetworkBehaviour</a>  
Generates a delegating `NetworkBehaviour` implementation for the struct
this is used for. See the trait documentation for better description.
