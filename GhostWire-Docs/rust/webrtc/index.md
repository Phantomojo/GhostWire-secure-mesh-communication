Crate webrtc Copy item path

<span class="sub-heading"><a href="../src/webrtc/lib.rs.html#1-44" class="src">Source</a>
</span>

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use error::`<a href="error/enum.Error.html" class="enum"
title="enum webrtc::error::Error"><code>Error</code></a>`;`

`pub use `<a href="../webrtc_data/index.html" class="mod"
title="mod webrtc_data"><code>data</code></a>`;`

`pub use `<a href="../webrtc_dtls/index.html" class="mod"
title="mod webrtc_dtls"><code>dtls</code></a>`;`

`pub use `<a href="../webrtc_ice/index.html" class="mod"
title="mod webrtc_ice"><code>ice</code></a>`;`

`pub use `<a href="../interceptor/index.html" class="mod"
title="mod interceptor"><code>interceptor</code></a>`;`

`pub use `<a href="../webrtc_mdns/index.html" class="mod"
title="mod webrtc_mdns"><code>mdns</code></a>`;`

`pub use `<a href="../webrtc_media/index.html" class="mod"
title="mod webrtc_media"><code>media</code></a>`;`

`pub use `<a href="../rtcp/index.html" class="mod"
title="mod rtcp"><code>rtcp</code></a>`;`

`pub use `<a href="../rtp/index.html" class="mod"
title="mod rtp"><code>rtp</code></a>`;`

`pub use `<a href="../webrtc_sctp/index.html" class="mod"
title="mod webrtc_sctp"><code>sctp</code></a>`;`

`pub use `<a href="../sdp/index.html" class="mod"
title="mod sdp"><code>sdp</code></a>`;`

`pub use `<a href="../webrtc_srtp/index.html" class="mod"
title="mod webrtc_srtp"><code>srtp</code></a>`;`

`pub use `<a href="../stun/index.html" class="mod"
title="mod stun"><code>stun</code></a>`;`

`pub use `<a href="../turn/index.html" class="mod"
title="mod turn"><code>turn</code></a>`;`

`pub use `<a href="../webrtc_util/index.html" class="mod"
title="mod webrtc_util"><code>util</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="api/index.html" class="mod" title="mod webrtc::api">api</a>

Set of constructors for WebRTC primitives. Subject to deprecation in
future.

<a href="data_channel/index.html" class="mod"
title="mod webrtc::data_channel">data_channel</a>

WebRTC DataChannel can be used for peer-to-peer transmitting arbitrary
binary data.

<a href="dtls_transport/index.html" class="mod"
title="mod webrtc::dtls_transport">dtls_transport</a>

<a href="error/index.html" class="mod"
title="mod webrtc::error">error</a>

[`Error`](error/enum.Error.html "enum webrtc::error::Error") enumerates
WebRTC problems,
[`error::OnErrorHdlrFn`](error/type.OnErrorHdlrFn.html "type webrtc::error::OnErrorHdlrFn")
defines type for callback-logger.

<a href="ice_transport/index.html" class="mod"
title="mod webrtc::ice_transport">ice_transport</a>

The utilities defining transport between peers. Contains
[`ice_transport::ice_server::RTCIceServer`](ice_transport/ice_server/struct.RTCIceServer.html "struct webrtc::ice_transport::ice_server::RTCIceServer")
struct which describes how peer does ICE (Interactive Connectivity
Establishment).

<a href="mux/index.html" class="mod" title="mod webrtc::mux">mux</a>

Module responsible for multiplexing data streams of different protocols
on one socket. Custom
[`mux::endpoint::Endpoint`](mux/endpoint/struct.Endpoint.html "struct webrtc::mux::endpoint::Endpoint")
with
[`mux::mux_func::MatchFunc`](mux/mux_func/type.MatchFunc.html "type webrtc::mux::mux_func::MatchFunc")
can be used for parsing your application-specific byte stream.

<a href="peer_connection/index.html" class="mod"
title="mod webrtc::peer_connection">peer_connection</a>

[`peer_connection::RTCPeerConnection`](peer_connection/struct.RTCPeerConnection.html "struct webrtc::peer_connection::RTCPeerConnection")
allows to establish connection between two peers given RTC
configuration. Its API is similar to one in JavaScript.

<a href="rtp_transceiver/index.html" class="mod"
title="mod webrtc::rtp_transceiver">rtp_transceiver</a>

<a href="sctp_transport/index.html" class="mod"
title="mod webrtc::sctp_transport">sctp_transport</a>

<a href="stats/index.html" class="mod"
title="mod webrtc::stats">stats</a>

Measuring connection statistics, such as amount of data transmitted or
round trip time.

<a href="track/index.html" class="mod"
title="mod webrtc::track">track</a>
