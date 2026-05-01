Crate netlink\_sys Copy item path

<span class="sub-heading"><a href="../src/netlink_sys/lib.rs.html#3-38" class="src">Source</a>
</span>

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::async_socket_ext::`<a href="async_socket_ext/trait.AsyncSocketExt.html" class="trait"
title="trait netlink_sys::async_socket_ext::AsyncSocketExt"><code>AsyncSocketExt</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="async_socket_ext/index.html" class="mod"
title="mod netlink_sys::async_socket_ext">async_socket_ext</a>

<a href="constants/index.html" class="mod"
title="mod netlink_sys::constants">constants</a>

This module provides a lot of netlink constants for various protocol. As
we add support for the various protocols, these constants will be moved
to their own crate.

<a href="protocols/index.html" class="mod"
title="mod netlink_sys::protocols">protocols</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Socket.html" class="struct"
title="struct netlink_sys::Socket">Socket</a>  
A netlink socket.

<a href="struct.SocketAddr.html" class="struct"
title="struct netlink_sys::SocketAddr">SocketAddr</a>  
The address of a netlink socket

<a href="struct.TokioSocket.html" class="struct"
title="struct netlink_sys::TokioSocket">TokioSocket</a>  
An I/O object representing a Netlink socket.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsyncSocket.html" class="trait"
title="trait netlink_sys::AsyncSocket">AsyncSocket</a>  
Trait to support different async backends
