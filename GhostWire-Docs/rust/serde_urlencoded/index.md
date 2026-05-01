Crate serde\_urlencoded Copy item path

<span class="sub-heading"><a href="../src/serde_urlencoded/lib.rs.html#1-12"
class="src">Source</a> </span>

Expand description

`x-www-form-urlencoded` meets Serde

# Modules<a href="#modules" class="anchor">§</a>

<a href="de/index.html" class="mod"
title="mod serde_urlencoded::de">de</a>  
Deserialization support for the `application/x-www-form-urlencoded`
format.

<a href="ser/index.html" class="mod"
title="mod serde_urlencoded::ser">ser</a>  
Serialization support for the `application/x-www-form-urlencoded`
format.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Deserializer.html" class="struct"
title="struct serde_urlencoded::Deserializer">Deserializer</a>  
A deserializer for the `application/x-www-form-urlencoded` format.

<a href="struct.Serializer.html" class="struct"
title="struct serde_urlencoded::Serializer">Serializer</a>  
A serializer for the `application/x-www-form-urlencoded` format.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.from_bytes.html" class="fn"
title="fn serde_urlencoded::from_bytes">from_bytes</a>  
Deserializes a `application/x-www-form-urlencoded` value from a `&[u8]`.

<a href="fn.from_reader.html" class="fn"
title="fn serde_urlencoded::from_reader">from_reader</a>  
Convenience function that reads all bytes from `reader` and deserializes
them with `from_bytes`.

<a href="fn.from_str.html" class="fn"
title="fn serde_urlencoded::from_str">from_str</a>  
Deserializes a `application/x-www-form-urlencoded` value from a `&str`.

<a href="fn.to_string.html" class="fn"
title="fn serde_urlencoded::to_string">to_string</a>  
Serializes a value into a `application/x-www-form-urlencoded` `String`
buffer.
