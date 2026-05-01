Crate tokio\_rustls Copy item path

<span class="sub-heading"><a href="../src/tokio_rustls/lib.rs.html#1-231" class="src">Source</a>
</span>

Expand description

Asynchronous TLS/SSL streams for Tokio using
[Rustls](https://github.com/rustls/rustls).

# <a href="#why-do-i-need-to-call-poll_flush" class="doc-anchor">§</a>Why do I need to call `poll_flush`?

Most TLS implementations will have an internal buffer to improve
throughput, and rustls is no exception.

When we write data to `TlsStream`, we always write rustls buffer first,
then take out rustls encrypted data packet, and write it to data channel
(like TcpStream). When data channel is pending, some data may remain in
rustls buffer.

`tokio-rustls` To keep it simple and correct,
[TlsStream](enum.TlsStream.html "enum tokio_rustls::TlsStream") will
behave like `BufWriter`. For `TlsStream<TcpStream>`, this means that
data written by `poll_write` is not guaranteed to be written to
`TcpStream`. You must call `poll_flush` to ensure that it is written to
`TcpStream`.

You should call `poll_flush` at the appropriate time, such as when a
period of `poll_write` write is complete and there is no more data to
write.

## <a href="#why-dont-we-write-during-poll_read" class="doc-anchor">§</a>Why don’t we write during `poll_read`?

We did this in the early days of `tokio-rustls`, but it caused some
bugs. We can solve these bugs through some solutions, but this will
cause performance degradation (reverse false wakeup).

And reverse write will also prevent us implement full duplex in the
future.

see <https://github.com/tokio-rs/tls/issues/40>

## <a href="#why-cant-we-handle-it-like-native-tls"
class="doc-anchor">§</a>Why can’t we handle it like `native-tls`?

When data channel returns to pending, `native-tls` will falsely report
the number of bytes it consumes. This means that if data written by
`poll_write` is not actually written to data channel, it will not return
`Ready`. Thus avoiding the call of `poll_flush`.

but which does not conform to convention of `AsyncWrite` trait. This
means that if you give inconsistent data in two `poll_write`, it may
cause unexpected behavior.

see <https://github.com/tokio-rs/tls/issues/41>

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use client::`<a href="client/struct.Connect.html" class="struct"
title="struct tokio_rustls::client::Connect"><code>Connect</code></a>`;`

`pub use client::`<a href="client/struct.FallibleConnect.html" class="struct"
title="struct tokio_rustls::client::FallibleConnect"><code>FallibleConnect</code></a>`;`

`pub use client::`<a href="client/struct.TlsConnector.html" class="struct"
title="struct tokio_rustls::client::TlsConnector"><code>TlsConnector</code></a>`;`

`pub use client::`<a href="client/struct.TlsConnectorWithAlpn.html" class="struct"
title="struct tokio_rustls::client::TlsConnectorWithAlpn"><code>TlsConnectorWithAlpn</code></a>`;`

`pub use server::`<a href="server/struct.Accept.html" class="struct"
title="struct tokio_rustls::server::Accept"><code>Accept</code></a>`;`

`pub use server::`<a href="server/struct.FallibleAccept.html" class="struct"
title="struct tokio_rustls::server::FallibleAccept"><code>FallibleAccept</code></a>`;`

`pub use server::`<a href="server/struct.LazyConfigAcceptor.html" class="struct"
title="struct tokio_rustls::server::LazyConfigAcceptor"><code>LazyConfigAcceptor</code></a>`;`

`pub use server::`<a href="server/struct.StartHandshake.html" class="struct"
title="struct tokio_rustls::server::StartHandshake"><code>StartHandshake</code></a>`;`

`pub use server::`<a href="server/struct.TlsAcceptor.html" class="struct"
title="struct tokio_rustls::server::TlsAcceptor"><code>TlsAcceptor</code></a>`;`

`pub use `<a href="../rustls/index.html" class="mod"
title="mod rustls"><code>rustls</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="client/index.html" class="mod"
title="mod tokio_rustls::client">client</a>

<a href="server/index.html" class="mod"
title="mod tokio_rustls::server">server</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.TlsStream.html" class="enum"
title="enum tokio_rustls::TlsStream">TlsStream</a>  
Unified TLS stream type
