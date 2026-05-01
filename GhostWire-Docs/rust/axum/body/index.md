[axum](../index.html)

Module body Copy item path

<span class="sub-heading"><a href="../../src/axum/body/mod.rs.html#1-54" class="src">Source</a>
</span>

Expand description

HTTP body utilities.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use http_body::`<a href="../../http_body/trait.Body.html" class="trait"
title="trait http_body::Body"><code>Body</code></a>` as HttpBody;`

`pub use bytes::`<a href="../../bytes/bytes/struct.Bytes.html" class="struct"
title="struct bytes::bytes::Bytes"><code>Bytes</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Body.html" class="struct"
title="struct axum::body::Body">Body</a>  
The body type used in axum requests and responses.

<a href="struct.BodyDataStream.html" class="struct"
title="struct axum::body::BodyDataStream">BodyDataStream</a>  
A stream of data frames.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.to_bytes.html" class="fn"
title="fn axum::body::to_bytes">to_bytes</a>  
Converts [`Body`](struct.Body.html "struct axum::body::Body") into
[`Bytes`](../../bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")
and limits the maximum size of the body.
