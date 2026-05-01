Crate httparse Copy item path

<span class="sub-heading"><a href="../src/httparse/lib.rs.html#1-2798" class="src">Source</a>
</span>

Expand description

# <a href="#httparse" class="doc-anchor">§</a>httparse

A push library for parsing HTTP/1.x requests and responses.

The focus is on speed and safety. Unsafe code is used to keep parsing
fast, but unsafety is contained in a submodule, with invariants
enforced. The parsing internals use an `Iterator` instead of direct
indexing, while skipping bounds checks.

With Rust 1.27.0 or later, support for SIMD is enabled automatically. If
building an executable to be run on multiple platforms, and thus not
passing `target_feature` or `target_cpu` flags to the compiler, runtime
detection can still detect SSE4.2 or AVX2 support to provide massive
wins.

If compiling for a specific target, remembering to include
`-C target_cpu=native` allows the detection to become compile time
checks, making it *even* faster.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Header.html" class="struct"
title="struct httparse::Header">Header</a>  
Represents a parsed header.

<a href="struct.InvalidChunkSize.html" class="struct"
title="struct httparse::InvalidChunkSize">InvalidChunkSize</a>  
An error in parsing a chunk size.

<a href="struct.ParserConfig.html" class="struct"
title="struct httparse::ParserConfig">ParserConfig</a>  
Parser configuration.

<a href="struct.Request.html" class="struct"
title="struct httparse::Request">Request</a>  
A parsed Request.

<a href="struct.Response.html" class="struct"
title="struct httparse::Response">Response</a>  
A parsed Response.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum httparse::Error">Error</a>  
An error in parsing.

<a href="enum.Status.html" class="enum"
title="enum httparse::Status">Status</a>  
The result of a successful parse pass.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.EMPTY_HEADER.html" class="constant"
title="constant httparse::EMPTY_HEADER">EMPTY_HEADER</a>  
An empty header, useful for constructing a `Header` array to pass in for
parsing.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.parse_chunk_size.html" class="fn"
title="fn httparse::parse_chunk_size">parse_chunk_size</a>  
Parse a buffer of bytes as a chunk size.

<a href="fn.parse_headers.html" class="fn"
title="fn httparse::parse_headers">parse_headers</a>  
Parse a buffer of bytes as headers.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type httparse::Result">Result</a>  
A Result of any parsing action.
