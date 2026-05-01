Crate libp2p\_tcp Copy item path

<span class="sub-heading"><a href="../src/libp2p_tcp/lib.rs.html#21-1156" class="src">Source</a>
</span>

Expand description

Implementation of the libp2p
[`libp2p_core::Transport`](../libp2p_core/transport/trait.Transport.html "trait libp2p_core::transport::Transport")
trait for TCP/IP.

# <a href="#usage" class="doc-anchor">§</a>Usage

This crate provides
[`tokio::Transport`](tokio/type.Transport.html "type libp2p_tcp::tokio::Transport")
which implement the
[`libp2p_core::Transport`](../libp2p_core/transport/trait.Transport.html "trait libp2p_core::transport::Transport")
trait for use as a transport with `libp2p-core` or `libp2p-swarm`.

# Modules<a href="#modules" class="anchor">§</a>

<a href="tokio/index.html" class="mod"
title="mod libp2p_tcp::tokio">tokio</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Config.html" class="struct"
title="struct libp2p_tcp::Config">Config</a>  
The configuration for a TCP/IP transport capability for libp2p.

<a href="struct.Transport.html" class="struct"
title="struct libp2p_tcp::Transport">Transport</a>  
An abstract
[`libp2p_core::Transport`](../libp2p_core/transport/trait.Transport.html "trait libp2p_core::transport::Transport")
implementation.
