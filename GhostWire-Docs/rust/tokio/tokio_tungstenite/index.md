Crate tokio\_tungstenite Copy item path

<span class="sub-heading"><a href="../src/tokio_tungstenite/lib.rs.html#1-441"
class="src">Source</a> </span>

Expand description

Async WebSocket usage.

This library is an implementation of WebSocket handshakes and streams.
It is based on the crate which implements all required WebSocket
protocol logic. So this crate basically just brings tokio support /
tokio integration to it.

Each WebSocket stream implements the required `Stream` and `Sink`
traits, so the socket is just a stream of messages coming in and going
out.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../tungstenite/index.html" class="mod"
title="mod tungstenite"><code>tungstenite</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.WebSocketStream.html" class="struct"
title="struct tokio_tungstenite::WebSocketStream">WebSocketStream</a>  
A wrapper around an underlying raw stream which implements the WebSocket
protocol.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Connector.html" class="enum"
title="enum tokio_tungstenite::Connector">Connector</a>  
A connector that can be used when establishing connections, allowing to
control whether `native-tls` or `rustls` is used to create a TLS
connection. Or TLS can be disabled with the `Plain` variant.

<a href="enum.MaybeTlsStream.html" class="enum"
title="enum tokio_tungstenite::MaybeTlsStream">MaybeTlsStream</a>  
A stream that might be protected with TLS.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.accept_async.html" class="fn"
title="fn tokio_tungstenite::accept_async">accept_async</a>  
Accepts a new WebSocket connection with the provided stream.

<a href="fn.accept_async_with_config.html" class="fn"
title="fn tokio_tungstenite::accept_async_with_config">accept_async_with_config</a>  
The same as `accept_async()` but the one can specify a websocket
configuration. Please refer to `accept_async()` for more details.

<a href="fn.accept_hdr_async.html" class="fn"
title="fn tokio_tungstenite::accept_hdr_async">accept_hdr_async</a>  
Accepts a new WebSocket connection with the provided stream.

<a href="fn.accept_hdr_async_with_config.html" class="fn"
title="fn tokio_tungstenite::accept_hdr_async_with_config">accept_hdr_async_with_config</a>  
The same as `accept_hdr_async()` but the one can specify a websocket
configuration. Please refer to `accept_hdr_async()` for more details.

<a href="fn.client_async.html" class="fn"
title="fn tokio_tungstenite::client_async">client_async</a>  
Creates a WebSocket handshake from a request and a stream. For
convenience, the user may call this with a url string, a URL, or a
`Request`. Calling with `Request` allows the user to add a WebSocket
protocol or other custom headers.

<a href="fn.client_async_with_config.html" class="fn"
title="fn tokio_tungstenite::client_async_with_config">client_async_with_config</a>  
The same as `client_async()` but the one can specify a websocket
configuration. Please refer to `client_async()` for more details.

<a href="fn.connect_async.html" class="fn"
title="fn tokio_tungstenite::connect_async">connect_async</a>  
Connect to a given URL.

<a href="fn.connect_async_with_config.html" class="fn"
title="fn tokio_tungstenite::connect_async_with_config">connect_async_with_config</a>  
The same as `connect_async()` but the one can specify a websocket
configuration. Please refer to `connect_async()` for more details.
`disable_nagle` specifies if the Nagle’s algorithm must be disabled,
i.e. `set_nodelay(true)`. If you don’t know what the Nagle’s algorithm
is, better leave it set to `false`.
