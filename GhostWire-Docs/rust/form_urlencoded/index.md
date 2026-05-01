Crate form\_urlencoded Copy item path

<span class="sub-heading"><a href="../src/form_urlencoded/lib.rs.html#9-430"
class="src">Source</a> </span>

Expand description

Parser and serializer for the [`application/x-www-form-urlencoded`
syntax](http://url.spec.whatwg.org/#application/x-www-form-urlencoded),
as used by HTML forms.

Converts between a string (such as an URL’s query string) and a sequence
of (name, value) pairs.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ByteSerialize.html" class="struct"
title="struct form_urlencoded::ByteSerialize">ByteSerialize</a>  
Return value of `byte_serialize()`.

<a href="struct.Parse.html" class="struct"
title="struct form_urlencoded::Parse">Parse</a>  
The return type of `parse()`.

<a href="struct.ParseIntoOwned.html" class="struct"
title="struct form_urlencoded::ParseIntoOwned">ParseIntoOwned</a>  
Like `Parse`, but yields pairs of `String` instead of pairs of
`Cow<str>`.

<a href="struct.Serializer.html" class="struct"
title="struct form_urlencoded::Serializer">Serializer</a>  
The [`application/x-www-form-urlencoded`
serializer](https://url.spec.whatwg.org/#concept-urlencoded-serializer).

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Target.html" class="trait"
title="trait form_urlencoded::Target">Target</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.byte_serialize.html" class="fn"
title="fn form_urlencoded::byte_serialize">byte_serialize</a>  
The [`application/x-www-form-urlencoded` byte
serializer](https://url.spec.whatwg.org/#concept-urlencoded-byte-serializer).

<a href="fn.parse.html" class="fn"
title="fn form_urlencoded::parse">parse</a>  
Convert a byte string in the `application/x-www-form-urlencoded` syntax
into a iterator of (name, value) pairs.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.EncodingOverride.html" class="type"
title="type form_urlencoded::EncodingOverride">EncodingOverride</a>
