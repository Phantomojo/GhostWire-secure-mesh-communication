[tokio](../index.html)

Module net Copy item path

<span class="sub-heading"><a href="../../src/tokio/net/mod.rs.html#1-64" class="src">Source</a>
</span>

Expand description

TCP/UDP/Unix bindings for `tokio`.

This module contains the TCP/UDP/Unix networking types, similar to the
standard library, which can be used to implement networking protocols.

# <a href="#organization" class="doc-anchor">§</a>Organization

-   [`TcpListener`](struct.TcpListener.html "struct tokio::net::TcpListener")
    and
    [`TcpStream`](struct.TcpStream.html "struct tokio::net::TcpStream")
    provide functionality for communication over TCP
-   [`UdpSocket`](struct.UdpSocket.html "struct tokio::net::UdpSocket")
    provides functionality for communication over UDP
-   [`UnixListener`](struct.UnixListener.html "struct tokio::net::UnixListener")
    and
    [`UnixStream`](struct.UnixStream.html "struct tokio::net::UnixStream")
    provide functionality for communication over a Unix Domain Stream
    Socket **(available on Unix only)**
-   [`UnixDatagram`](struct.UnixDatagram.html "struct tokio::net::UnixDatagram")
    provides functionality for communication over Unix Domain Datagram
    Socket **(available on Unix only)**
-   [`tokio::net::unix::pipe`](unix/pipe/index.html "mod tokio::net::unix::pipe")
    for FIFO pipes **(available on Unix only)**
-   [`tokio::net::windows::named_pipe`](windows::named_pipe) for Named
    Pipes **(available on Windows only)**

For IO resources not available in `tokio::net`, you can use
[`AsyncFd`](../io/unix/struct.AsyncFd.html "struct tokio::io::unix::AsyncFd").

# Modules<a href="#modules" class="anchor">§</a>

<a href="tcp/index.html" class="mod" title="mod tokio::net::tcp">tcp</a>  
TCP utility types.

<a href="unix/index.html" class="mod"
title="mod tokio::net::unix">unix</a>  
Unix specific network types.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.TcpListener.html" class="struct"
title="struct tokio::net::TcpListener">TcpListener</a>  
A TCP socket server, listening for connections.

<a href="struct.TcpSocket.html" class="struct"
title="struct tokio::net::TcpSocket">TcpSocket</a>  
A TCP socket that has not yet been converted to a `TcpStream` or
`TcpListener`.

<a href="struct.TcpStream.html" class="struct"
title="struct tokio::net::TcpStream">TcpStream</a>  
A TCP stream between a local and a remote socket.

<a href="struct.UdpSocket.html" class="struct"
title="struct tokio::net::UdpSocket">UdpSocket</a>  
A UDP socket.

<a href="struct.UnixDatagram.html" class="struct"
title="struct tokio::net::UnixDatagram">UnixDatagram</a>  
An I/O object representing a Unix datagram socket.

<a href="struct.UnixListener.html" class="struct"
title="struct tokio::net::UnixListener">UnixListener</a>  
A Unix socket which can accept connections from other Unix sockets.

<a href="struct.UnixSocket.html" class="struct"
title="struct tokio::net::UnixSocket">UnixSocket</a>  
A Unix socket that has not yet been converted to a
[`UnixStream`](struct.UnixStream.html "struct tokio::net::UnixStream"),
[`UnixDatagram`](struct.UnixDatagram.html "struct tokio::net::UnixDatagram"),
or
[`UnixListener`](struct.UnixListener.html "struct tokio::net::UnixListener").

<a href="struct.UnixStream.html" class="struct"
title="struct tokio::net::UnixStream">UnixStream</a>  
A structure representing a connected Unix socket.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ToSocketAddrs.html" class="trait"
title="trait tokio::net::ToSocketAddrs">ToSocketAddrs</a>  
Converts or resolves without blocking to one or more `SocketAddr`
values.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.lookup_host.html" class="fn"
title="fn tokio::net::lookup_host">lookup_host</a>  
Performs a DNS resolution.
