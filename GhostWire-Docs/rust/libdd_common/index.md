Crate libdd\_common Copy item path

<span class="sub-heading"><a href="../src/libdd_common/lib.rs.html#3-403" class="src">Source</a>
</span>

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../const_format/index.html" class="mod"
title="mod const_format"><code>const_format</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="azure_app_services/index.html" class="mod"
title="mod libdd_common::azure_app_services">azure_app_services</a>

<a href="cc_utils/index.html" class="mod"
title="mod libdd_common::cc_utils">cc_utils</a>

<a href="config/index.html" class="mod"
title="mod libdd_common::config">config</a>

<a href="connector/index.html" class="mod"
title="mod libdd_common::connector">connector</a>

<a href="cstr/index.html" class="mod"
title="mod libdd_common::cstr">cstr</a>

<a href="entity_id/index.html" class="mod"
title="mod libdd_common::entity_id">entity_id</a>

Extract the entity id, container id and external env

<a href="error/index.html" class="mod"
title="mod libdd_common::error">error</a>

<a href="header/index.html" class="mod"
title="mod libdd_common::header">header</a>

<a href="http_common/index.html" class="mod"
title="mod libdd_common::http_common">http_common</a>

<a href="rate_limiter/index.html" class="mod"
title="mod libdd_common::rate_limiter">rate_limiter</a>

<a href="tag/index.html" class="mod"
title="mod libdd_common::tag">tag</a>

<a href="threading/index.html" class="mod"
title="mod libdd_common::threading">threading</a>

<a href="timeout/index.html" class="mod"
title="mod libdd_common::timeout">timeout</a>

<a href="unix_utils/index.html" class="mod"
title="mod libdd_common::unix_utils">unix_utils</a>

<a href="worker/index.html" class="mod"
title="mod libdd_common::worker">worker</a>

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.cstr.html" class="macro"
title="macro libdd_common::cstr">cstr</a>  
<a href="macro.cstr_u8.html" class="macro"
title="macro libdd_common::cstr_u8">cstr_u8</a>  
<a href="macro.tag.html" class="macro"
title="macro libdd_common::tag">tag</a>  
Creates a tag from a key and value known at compile-time, and fails to
compile if it’s known to be invalid (it may still emit an invalid tag,
not all tag validation is currently done client-side). If the key or
value aren’t known at compile-time, then use
[Tag::new](tag/struct.Tag.html#method.new "associated function libdd_common::tag::Tag::new").

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Endpoint.html" class="struct"
title="struct libdd_common::Endpoint">Endpoint</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Connect.html" class="trait"
title="trait libdd_common::Connect">Connect</a>  
<a href="trait.MutexExt.html" class="trait"
title="trait libdd_common::MutexExt">MutexExt</a>  
Extension trait for `Mutex` to provide a method that acquires a lock,
panicking if the lock is poisoned.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode_uri_path_in_authority.html" class="fn"
title="fn libdd_common::decode_uri_path_in_authority">decode_uri_path_in_authority</a>  
<a href="fn.parse_uri.html" class="fn"
title="fn libdd_common::parse_uri">parse_uri</a>  
TODO: we should properly handle malformed urls

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.GenericHttpClient.html" class="type"
title="type libdd_common::GenericHttpClient">GenericHttpClient</a>

<a href="type.HttpClient.html" class="type"
title="type libdd_common::HttpClient">HttpClient</a>

<a href="type.HttpRequestBuilder.html" class="type"
title="type libdd_common::HttpRequestBuilder">HttpRequestBuilder</a>

<a href="type.HttpResponse.html" class="type"
title="type libdd_common::HttpResponse">HttpResponse</a>
