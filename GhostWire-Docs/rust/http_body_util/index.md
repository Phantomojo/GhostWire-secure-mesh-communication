Crate http\_body\_util Copy item path

<span class="sub-heading"><a href="../src/http_body_util/lib.rs.html#1-147" class="src">Source</a>
</span>

Expand description

Utilities for
[`http_body::Body`](../http_body/trait.Body.html "trait http_body::Body").

[`BodyExt`](trait.BodyExt.html "trait http_body_util::BodyExt") adds
extensions to the common trait.

[`Empty`](struct.Empty.html "struct http_body_util::Empty") and
[`Full`](struct.Full.html "struct http_body_util::Full") provide simple
implementations.

# Modules<a href="#modules" class="anchor">§</a>

<a href="combinators/index.html" class="mod"
title="mod http_body_util::combinators">combinators</a>  
Combinators for the `Body` trait.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BodyDataStream.html" class="struct"
title="struct http_body_util::BodyDataStream">BodyDataStream</a>  
A data stream created from a
[`Body`](../http_body/trait.Body.html "trait http_body::Body").

<a href="struct.BodyStream.html" class="struct"
title="struct http_body_util::BodyStream">BodyStream</a>  
A stream created from a
[`Body`](../http_body/trait.Body.html "trait http_body::Body").

<a href="struct.Collected.html" class="struct"
title="struct http_body_util::Collected">Collected</a>  
A collected body produced by
[`BodyExt::collect`](trait.BodyExt.html#method.collect "method http_body_util::BodyExt::collect")
which collects all the DATA frames and trailers.

<a href="struct.Empty.html" class="struct"
title="struct http_body_util::Empty">Empty</a>  
A body that is always empty.

<a href="struct.Full.html" class="struct"
title="struct http_body_util::Full">Full</a>  
A body that consists of a single chunk.

<a href="struct.LengthLimitError.html" class="struct"
title="struct http_body_util::LengthLimitError">LengthLimitError</a>  
An error returned when body length exceeds the configured limit.

<a href="struct.Limited.html" class="struct"
title="struct http_body_util::Limited">Limited</a>  
A length limited body.

<a href="struct.StreamBody.html" class="struct"
title="struct http_body_util::StreamBody">StreamBody</a>  
A body created from a
[`Stream`](../futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Either.html" class="enum"
title="enum http_body_util::Either">Either</a>  
Sum type with two cases:
[`Left`](enum.Either.html#variant.Left "variant http_body_util::Either::Left")
and
[`Right`](enum.Either.html#variant.Right "variant http_body_util::Either::Right"),
used if a body can be one of two distinct types.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BodyExt.html" class="trait"
title="trait http_body_util::BodyExt">BodyExt</a>  
An extension trait for
[`http_body::Body`](../http_body/trait.Body.html "trait http_body::Body")
adding various combinators and adapters
