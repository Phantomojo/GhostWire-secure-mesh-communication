Crate substring Copy item path

<span class="sub-heading"><a href="../src/substring/lib.rs.html#1-140" class="src">Source</a>
</span>

Expand description

Substring method for string types.

This crate provides a
[`substring()`](trait.Substring.html#tymethod.substring) method on Rust
string types. The method takes a start and end character index and
returns a string slice of the characters within that range.

The method is provided via the [`Substring`](trait.Substring.html) trait
which is implemented on the
[`str`](https://doc.rust-lang.org/std/primitive.str.html) primitive.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use substring::Substring;

// Works on a string slice.
assert_eq!("foobar".substring(2,5), "oba");

// Also works on a String.
assert_eq!("foobar".to_string().substring(1,6), "oobar");
```

As Rust strings are UTF-8 encoded, the algorithm for finding a character
substring is `O(n)`, where `n` is the byte length of the string. This is
due to characters not being of predictible byte lengths.

# <a href="#note" class="doc-anchor">§</a>Note

The indexing of substrings is based on [*Unicode Scalar
Value*](http://www.unicode.org/glossary/#unicode_scalar_value). As such,
substrings may not always match your intuition:

``` {.rust .rust-example-rendered}
use substring::Substring;

assert_eq!("ã".substring(0, 1), "a");  // As opposed to "ã".
assert_eq!("ã".substring(1, 2), "\u{0303}")
```

The above example occurs because “ã” is technically made up of two UTF-8
scalar values.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Substring.html" class="trait"
title="trait substring::Substring">Substring</a>  
Provides a [`substring()`](trait.Substring.html#tymethod.substring)
method.
