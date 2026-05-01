Crate axum\_core Copy item path

<span class="sub-heading"><a href="../src/axum_core/lib.rs.html#1-70" class="src">Source</a>
</span>

Expand description

Core types and traits for [`axum`](https://crates.io/crates/axum).

Libraries authors that want to provide
[`FromRequest`](extract/trait.FromRequest.html "trait axum_core::extract::FromRequest")
or
[`IntoResponse`](response/trait.IntoResponse.html "trait axum_core::response::IntoResponse")
implementations should depend on the
[`axum-core`](http://crates.io/crates/axum-core) crate, instead of
`axum` if possible.

# Modules<a href="#modules" class="anchor">§</a>

<a href="body/index.html" class="mod"
title="mod axum_core::body">body</a>  
HTTP body utilities.

<a href="extract/index.html" class="mod"
title="mod axum_core::extract">extract</a>  
Types and traits for extracting data from requests.

<a href="response/index.html" class="mod"
title="mod axum_core::response">response</a>  
Types and traits for generating responses.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct axum_core::Error">Error</a>  
Errors that can happen when using axum.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.RequestExt.html" class="trait"
title="trait axum_core::RequestExt">RequestExt</a>  
Extension trait that adds additional methods to
[`Request`](extract/type.Request.html "type axum_core::extract::Request").

<a href="trait.RequestPartsExt.html" class="trait"
title="trait axum_core::RequestPartsExt">RequestPartsExt</a>  
Extension trait that adds additional methods to
[`Parts`](../http/request/struct.Parts.html "struct http::request::Parts").

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BoxError.html" class="type"
title="type axum_core::BoxError">BoxError</a>  
Alias for a type-erased error type.
