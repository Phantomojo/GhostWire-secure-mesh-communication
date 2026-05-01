Crate libp2p\_webrtc\_utils Copy item path

<span class="sub-heading"><a href="../src/libp2p_webrtc_utils/lib.rs.html#1-15"
class="src">Source</a> </span>

# Modules<a href="#modules" class="anchor">§</a>

<a href="noise/index.html" class="mod"
title="mod libp2p_webrtc_utils::noise">noise</a>

<a href="sdp/index.html" class="mod"
title="mod libp2p_webrtc_utils::sdp">sdp</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DropListener.html" class="struct"
title="struct libp2p_webrtc_utils::DropListener">DropListener</a>  
<a href="struct.Fingerprint.html" class="struct"
title="struct libp2p_webrtc_utils::Fingerprint">Fingerprint</a>  
A certificate fingerprint that is assumed to be created using the SHA256
hash algorithm.

<a href="struct.Stream.html" class="struct"
title="struct libp2p_webrtc_utils::Stream">Stream</a>  
A stream backed by a WebRTC data channel.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.MAX_MSG_LEN.html" class="constant"
title="constant libp2p_webrtc_utils::MAX_MSG_LEN">MAX_MSG_LEN</a>

Maximum length of a message.

<a href="constant.SHA256.html" class="constant"
title="constant libp2p_webrtc_utils::SHA256">SHA256</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.parse_webrtc_dial_addr.html" class="fn"
title="fn libp2p_webrtc_utils::parse_webrtc_dial_addr">parse_webrtc_dial_addr</a>  
Parse the given
[`Multiaddr`](../multiaddr/struct.Multiaddr.html "struct multiaddr::Multiaddr")
into a
[`SocketAddr`](https://doc.rust-lang.org/1.93.1/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")
and a
[`Fingerprint`](struct.Fingerprint.html "struct libp2p_webrtc_utils::Fingerprint")
for dialing.
