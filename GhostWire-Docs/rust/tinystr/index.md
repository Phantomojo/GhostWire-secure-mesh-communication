Crate tinystr Copy item path

<span class="sub-heading"><a href="../src/tinystr/lib.rs.html#5-114" class="src">Source</a>
</span>

Expand description

`tinystr` is a utility crate of the [`ICU4X`](../icu/index.html)
project.

It includes
[`TinyAsciiStr`](struct.TinyAsciiStr.html "struct tinystr::TinyAsciiStr"),
a core API for representing small ASCII-only bounded length strings.

It is optimized for operations on strings of size 8 or smaller. When use
cases involve comparison and conversion of strings for
lowercase/uppercase/titlecase, or checking
numeric/alphabetic/alphanumeric, `TinyAsciiStr` is the edge performance
library.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use tinystr::TinyAsciiStr;

let s1: TinyAsciiStr<4> = "tEsT".parse().expect("Failed to parse.");

assert_eq!(s1, "tEsT");
assert_eq!(s1.to_ascii_uppercase(), "TEST");
assert_eq!(s1.to_ascii_lowercase(), "test");
assert_eq!(s1.to_ascii_titlecase(), "Test");
assert!(s1.is_ascii_alphanumeric());
assert!(!s1.is_ascii_numeric());

let s2 = TinyAsciiStr::<8>::try_from_raw(*b"New York")
    .expect("Failed to parse.");

assert_eq!(s2, "New York");
assert_eq!(s2.to_ascii_uppercase(), "NEW YORK");
assert_eq!(s2.to_ascii_lowercase(), "new york");
assert_eq!(s2.to_ascii_titlecase(), "New york");
assert!(!s2.is_ascii_alphanumeric());
```

# <a href="#details" class="doc-anchor">§</a>Details

When strings are of size 8 or smaller, the struct transforms the strings
as `u32`/`u64` and uses bitmasking to provide basic string manipulation
operations:

-   `is_ascii_numeric`
-   `is_ascii_alphabetic`
-   `is_ascii_alphanumeric`
-   `to_ascii_lowercase`
-   `to_ascii_uppercase`
-   `to_ascii_titlecase`
-   `PartialEq`

`TinyAsciiStr` will fall back to `u8` character manipulation for strings
of length greater than 8.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.tinystr.html" class="macro"
title="macro tinystr::tinystr">tinystr</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.TinyAsciiStr.html" class="struct"
title="struct tinystr::TinyAsciiStr">TinyAsciiStr</a>  
<a href="struct.UnvalidatedTinyAsciiStr.html" class="struct"
title="struct tinystr::UnvalidatedTinyAsciiStr">UnvalidatedTinyAsciiStr</a>  
A fixed-length bytes array that is expected to be an ASCII string but
does not enforce that invariant.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ParseError.html" class="enum"
title="enum tinystr::ParseError">ParseError</a>

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.TinyStr4.html" class="type"
title="type tinystr::TinyStr4">TinyStr4</a>  
These are temporary compatability reexports that will be removed in a
future version.

<a href="type.TinyStr8.html" class="type"
title="type tinystr::TinyStr8">TinyStr8</a>  
These are temporary compatability reexports that will be removed in a
future version.

<a href="type.TinyStr16.html" class="type"
title="type tinystr::TinyStr16">TinyStr16</a>  
These are temporary compatability reexports that will be removed in a
future version.
