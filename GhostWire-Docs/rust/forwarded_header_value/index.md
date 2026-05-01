Crate forwarded\_header\_value Copy item path

<span class="sub-heading"><a href="../src/forwarded_header_value/lib.rs.html#1-503"
class="src">Source</a> </span>

Expand description

This module implements parsing for the Forwarded header as defined by
RFC 7239 and RFC 7230. You should generally interact with the
[`ForwardedHeaderValue`](struct.ForwardedHeaderValue.html "struct forwarded_header_value::ForwardedHeaderValue")
struct.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ForwardedHeaderValue.html" class="struct"
title="struct forwarded_header_value::ForwardedHeaderValue">ForwardedHeaderValue</a>  
This structure represents the contents of the Forwarded header

<a href="struct.ForwardedHeaderValueIterator.html" class="struct"
title="struct forwarded_header_value::ForwardedHeaderValueIterator">ForwardedHeaderValueIterator</a>  
Iterator over stanzas in a ForwardedHeaderValue

<a href="struct.ForwardedStanza.html" class="struct"
title="struct forwarded_header_value::ForwardedStanza">ForwardedStanza</a>  
A single forwarded-for line; there may be a sequence of these in a
Forwarded header.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ForwardedHeaderValueParseError.html" class="enum"
title="enum forwarded_header_value::ForwardedHeaderValueParseError">ForwardedHeaderValueParseError</a>  
Errors that can occur while parsing a ForwardedHeaderValue

<a href="enum.Identifier.html" class="enum"
title="enum forwarded_header_value::Identifier">Identifier</a>  
Remote identifier. This can be an IP:port pair, a bare IP, an
underscore-prefixed “obfuscated string”, or unknown.

<a href="enum.Protocol.html" class="enum"
title="enum forwarded_header_value::Protocol">Protocol</a>  
HTTP protocol from the remote host
