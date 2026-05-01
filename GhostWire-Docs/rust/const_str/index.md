Crate const\_str Copy item path

<span class="sub-heading"><a href="../src/const_str/lib.rs.html#1-125" class="src">Source</a>
</span>

Expand description

Compile-time string operations

MSRV: Rust 1.61.0

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.concat.html" class="macro"
title="macro const_str::concat">concat</a>  
Concatenates values into a string slice.

<a href="macro.contains.html" class="macro"
title="macro const_str::contains">contains</a>  
Returns
[`true`](https://doc.rust-lang.org/1.93.1/core/primitive.bool.html "primitive bool")
if the given pattern matches a sub-slice of this string slice.

<a href="macro.convert_ascii_case.html" class="macro"
title="macro const_str::convert_ascii_case">convert_ascii_case</a>  
Converts a string slice to a specified case. Non-ascii characters are
not affected.

<a href="macro.encode.html" class="macro"
title="macro const_str::encode">encode</a>  
Encode a string slice with a specified encoding.

<a href="macro.encode_z.html" class="macro"
title="macro const_str::encode_z">encode_z</a>  
Encode a string slice with a specified encoding and append a nul
character.

<a href="macro.ends_with.html" class="macro"
title="macro const_str::ends_with">ends_with</a>  
Returns
[`true`](https://doc.rust-lang.org/1.93.1/core/primitive.bool.html "primitive bool")
if the given pattern matches a suffix of this string slice.

<a href="macro.equal.html" class="macro"
title="macro const_str::equal">equal</a>  
Checks that two strings are equal.

<a href="macro.from_utf8.html" class="macro"
title="macro const_str::from_utf8">from_utf8</a>  
Converts a byte string to a string slice

<a href="macro.hex_bytes.html" class="macro"
title="macro const_str::hex_bytes">hex_bytes</a>  
Converts hexadecimal string slices to a byte array.

<a href="macro.parse.html" class="macro"
title="macro const_str::parse">parse</a>  
Parse a value from a string slice.

<a href="macro.repeat.html" class="macro"
title="macro const_str::repeat">repeat</a>  
Creates a new string slice by repeating a string slice n times.

<a href="macro.replace.html" class="macro"
title="macro const_str::replace">replace</a>  
Replaces all matches of a pattern with another string slice.

<a href="macro.split.html" class="macro"
title="macro const_str::split">split</a>  
Returns an array of substrings of a string slice, separated by
characters matched by a pattern.

<a href="macro.starts_with.html" class="macro"
title="macro const_str::starts_with">starts_with</a>  
Returns
[`true`](https://doc.rust-lang.org/1.93.1/core/primitive.bool.html "primitive bool")
if the given pattern matches a prefix of this string slice.

<a href="macro.to_byte_array.html" class="macro"
title="macro const_str::to_byte_array">to_byte_array</a>  
Converts a string slice or a byte string to a byte array.

<a href="macro.to_char_array.html" class="macro"
title="macro const_str::to_char_array">to_char_array</a>  
Converts a string slice into an array of its characters.

<a href="macro.to_str.html" class="macro"
title="macro const_str::to_str">to_str</a>  
Converts a value to a string slice.
