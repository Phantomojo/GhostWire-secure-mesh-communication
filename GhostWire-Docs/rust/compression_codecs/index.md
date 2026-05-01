Crate compression\_codecs Copy item path

<span class="sub-heading"><a href="../src/compression_codecs/lib.rs.html#1-234"
class="src">Source</a> </span>

Expand description

Adaptors for various compression algorithms.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::flate::`<a href="flate/struct.FlateDecoder.html" class="struct"
title="struct compression_codecs::flate::FlateDecoder"><code>FlateDecoder</code></a>`;`

`pub use self::flate::`<a href="flate/struct.FlateEncoder.html" class="struct"
title="struct compression_codecs::flate::FlateEncoder"><code>FlateEncoder</code></a>`;`

`pub use self::gzip::`<a href="gzip/struct.GzipDecoder.html" class="struct"
title="struct compression_codecs::gzip::GzipDecoder"><code>GzipDecoder</code></a>`;`

`pub use self::gzip::`<a href="gzip/struct.GzipEncoder.html" class="struct"
title="struct compression_codecs::gzip::GzipEncoder"><code>GzipEncoder</code></a>`;`

`pub use `<a href="../compression_core/index.html" class="mod"
title="mod compression_core"><code>compression_core</code></a>` as core;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="flate/index.html" class="mod"
title="mod compression_codecs::flate">flate</a>

<a href="gzip/index.html" class="mod"
title="mod compression_codecs::gzip">gzip</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Decode.html" class="trait"
title="trait compression_codecs::Decode">Decode</a>  
<a href="trait.DecodeV2.html" class="trait"
title="trait compression_codecs::DecodeV2">DecodeV2</a>  
version 2
[`Decode`](trait.Decode.html "trait compression_codecs::Decode") that is
trait object safe.

<a href="trait.DecodedSize.html" class="trait"
title="trait compression_codecs::DecodedSize">DecodedSize</a>  
<a href="trait.Encode.html" class="trait"
title="trait compression_codecs::Encode">Encode</a>  
<a href="trait.EncodeV2.html" class="trait"
title="trait compression_codecs::EncodeV2">EncodeV2</a>  
version 2 of
[`Encode`](trait.Encode.html "trait compression_codecs::Encode") that is
trait object safe.
