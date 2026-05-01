Crate tungstenite Copy item path

<span class="sub-heading"><a href="../src/tungstenite/lib.rs.html#1-48" class="src">Source</a>
</span>

Expand description

Lightweight, flexible WebSockets for Rust.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::error::`<a href="error/enum.Error.html" class="enum"
title="enum tungstenite::error::Error"><code>Error</code></a>`;`

`pub use crate::error::`<a href="error/type.Result.html" class="type"
title="type tungstenite::error::Result"><code>Result</code></a>`;`

`pub use crate::protocol::`<a href="protocol/enum.Message.html" class="enum"
title="enum tungstenite::protocol::Message"><code>Message</code></a>`;`

`pub use crate::protocol::`<a href="protocol/struct.WebSocket.html" class="struct"
title="struct tungstenite::protocol::WebSocket"><code>WebSocket</code></a>`;`

`pub use crate::client::`<a href="client/fn.client.html" class="fn"
title="fn tungstenite::client::client"><code>client</code></a>`;`

`pub use crate::client::`<a href="client/fn.connect.html" class="fn"
title="fn tungstenite::client::connect"><code>connect</code></a>`;`

`pub use crate::client::`<a href="client/struct.ClientRequestBuilder.html" class="struct"
title="struct tungstenite::client::ClientRequestBuilder"><code>ClientRequestBuilder</code></a>`;`

`pub use crate::handshake::client::`<a href="handshake/client/struct.ClientHandshake.html" class="struct"
title="struct tungstenite::handshake::client::ClientHandshake"><code>ClientHandshake</code></a>`;`

`pub use crate::handshake::server::`<a href="handshake/server/struct.ServerHandshake.html" class="struct"
title="struct tungstenite::handshake::server::ServerHandshake"><code>ServerHandshake</code></a>`;`

`pub use crate::handshake::`<a href="handshake/enum.HandshakeError.html" class="enum"
title="enum tungstenite::handshake::HandshakeError"><code>HandshakeError</code></a>`;`

`pub use `<a href="../http/index.html" class="mod"
title="mod http"><code>http</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="buffer/index.html" class="mod"
title="mod tungstenite::buffer">buffer</a>  
A buffer for reading data from the network.

<a href="client/index.html" class="mod"
title="mod tungstenite::client">client</a>  
Methods to connect to a WebSocket as a client.

<a href="error/index.html" class="mod"
title="mod tungstenite::error">error</a>  
Error handling.

<a href="handshake/index.html" class="mod"
title="mod tungstenite::handshake">handshake</a>  
WebSocket handshake control.

<a href="protocol/index.html" class="mod"
title="mod tungstenite::protocol">protocol</a>  
Generic WebSocket message stream.

<a href="stream/index.html" class="mod"
title="mod tungstenite::stream">stream</a>  
Convenience wrapper for streams to switch between plain TCP and TLS at
runtime.

<a href="util/index.html" class="mod"
title="mod tungstenite::util">util</a>  
Helper traits to ease non-blocking handling.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.accept.html" class="fn"
title="fn tungstenite::accept">accept</a>  
Accept the given Stream as a WebSocket.

<a href="fn.accept_hdr.html" class="fn"
title="fn tungstenite::accept_hdr">accept_hdr</a>  
Accept the given Stream as a WebSocket.

<a href="fn.accept_hdr_with_config.html" class="fn"
title="fn tungstenite::accept_hdr_with_config">accept_hdr_with_config</a>  
Accept the given Stream as a WebSocket.

<a href="fn.accept_with_config.html" class="fn"
title="fn tungstenite::accept_with_config">accept_with_config</a>  
Accept the given Stream as a WebSocket.
