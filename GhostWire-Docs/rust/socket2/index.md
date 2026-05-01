Crate socket2 Copy item path

<span class="sub-heading"><a href="../src/socket2/lib.rs.html#9-717" class="src">Source</a>
</span>

Expand description

Utilities for creating and using sockets.

The goal of this crate is to create and use a socket using advanced
configuration options (those that are not available in the types in the
standard library) without using any unsafe code.

This crate provides as direct as possible access to the system’s
functionality for sockets, this means little effort to provide
cross-platform utilities. It is up to the user to know how to use
sockets when using this crate. *If you don’t know how to create a socket
using libc/system calls then this crate is not for you*. Most, if not
all, functions directly relate to the equivalent system call with no
error handling applied, so no handling errors such as
[`EINTR`](https://doc.rust-lang.org/1.93.1/std/io/error/enum.ErrorKind.html#variant.Interrupted "variant std::io::error::ErrorKind::Interrupted").
As a result using this crate can be a little wordy, but it should give
you maximal flexibility over configuration of sockets.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use std::net::{SocketAddr, TcpListener};
use socket2::{Socket, Domain, Type};

// Create a TCP listener bound to two addresses.
let socket = Socket::new(Domain::IPV6, Type::STREAM, None)?;

socket.set_only_v6(false)?;
let address: SocketAddr = "[::1]:12345".parse().unwrap();
socket.bind(&address.into())?;
socket.listen(128)?;

let listener: TcpListener = socket.into();
// ...
```

## <a href="#features" class="doc-anchor">§</a>Features

This crate has a single feature `all`, which enables all functions even
ones that are not available on all OSs.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CcidEndpoints.html" class="struct"
title="struct socket2::CcidEndpoints">CcidEndpoints</a>  
See
[`Socket::dccp_available_ccids`](struct.Socket.html#method.dccp_available_ccids "method socket2::Socket::dccp_available_ccids").

<a href="struct.Domain.html" class="struct"
title="struct socket2::Domain">Domain</a>  
Specification of the communication domain for a socket.

<a href="struct.MaybeUninitSlice.html" class="struct"
title="struct socket2::MaybeUninitSlice">MaybeUninitSlice</a>  
A version of
[`IoSliceMut`](https://doc.rust-lang.org/1.93.1/std/io/struct.IoSliceMut.html "struct std::io::IoSliceMut")
that allows the buffer to be uninitialised.

<a href="struct.MsgHdr.html" class="struct"
title="struct socket2::MsgHdr">MsgHdr</a>  
Configuration of a `sendmsg(2)` system call.

<a href="struct.MsgHdrMut.html" class="struct"
title="struct socket2::MsgHdrMut">MsgHdrMut</a>  
Configuration of a `recvmsg(2)` system call.

<a href="struct.Protocol.html" class="struct"
title="struct socket2::Protocol">Protocol</a>  
Protocol specification used for creating sockets via `Socket::new`.

<a href="struct.RecvFlags.html" class="struct"
title="struct socket2::RecvFlags">RecvFlags</a>  
Flags for incoming messages.

<a href="struct.SockAddr.html" class="struct"
title="struct socket2::SockAddr">SockAddr</a>  
The address of a socket.

<a href="struct.SockAddrStorage.html" class="struct"
title="struct socket2::SockAddrStorage">SockAddrStorage</a>  
Rust version of the
[`sockaddr_storage`](../libc/unix/linux_like/struct.sockaddr_storage.html "struct libc::unix::linux_like::sockaddr_storage")
type.

<a href="struct.SockFilter.html" class="struct"
title="struct socket2::SockFilter">SockFilter</a>  
Berkeley Packet Filter (BPF).

<a href="struct.SockRef.html" class="struct"
title="struct socket2::SockRef">SockRef</a>  
A reference to a [`Socket`](struct.Socket.html "struct socket2::Socket")
that can be used to configure socket types other than the `Socket` type
itself.

<a href="struct.Socket.html" class="struct"
title="struct socket2::Socket">Socket</a>  
Owned wrapper around a system socket.

<a href="struct.TcpKeepalive.html" class="struct"
title="struct socket2::TcpKeepalive">TcpKeepalive</a>  
Configures a socket’s TCP keepalive parameters.

<a href="struct.Type.html" class="struct"
title="struct socket2::Type">Type</a>  
Specification of communication semantics on a socket.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.InterfaceIndexOrAddress.html" class="enum"
title="enum socket2::InterfaceIndexOrAddress">InterfaceIndexOrAddress</a>  
A local interface specified by its index or an address assigned to it.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.sa_family_t.html" class="type"
title="type socket2::sa_family_t">sa_family_t</a>  
The integer type for the `ss_family` field on this platform.

<a href="type.socklen_t.html" class="type"
title="type socket2::socklen_t">socklen_t</a>  
The integer type used with `getsockname` on this platform.
