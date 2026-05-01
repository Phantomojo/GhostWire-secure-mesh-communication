Crate itoa Copy item path

<span class="sub-heading"><a href="../src/itoa/lib.rs.html#1-459" class="src">Source</a>
</span>

Expand description

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/itoa) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/itoa) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/itoa)

  

This crate provides a fast conversion of integer primitives to decimal
strings. The implementation comes straight from
[libcore](https://github.com/rust-lang/rust/blob/1.92.0/library/core/src/fmt/num.rs#L190-L253)
but avoids the performance penalty of going through
[`core::fmt::Formatter`](https://doc.rust-lang.org/1.93.1/core/fmt/struct.Formatter.html "struct core::fmt::Formatter").

See also [`zmij`](https://github.com/dtolnay/zmij) for printing floating
point primitives.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
fn main() {
    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(128u64);
    assert_eq!(printed, "128");
}
```

# <a href="#performance" class="doc-anchor">§</a>Performance

The [itoa-benchmark](https://github.com/dtolnay/itoa-benchmark) compares
this library and other Rust integer formatting implementations across a
range of integer sizes. The vertical axis in this chart shows
nanoseconds taken by a single execution of
`itoa::Buffer::new().format(value)` so a lower result indicates a faster
library.

![performance](https://raw.githubusercontent.com/dtolnay/itoa/master/itoa-benchmark.png)

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Buffer.html" class="struct"
title="struct itoa::Buffer">Buffer</a>  
A correctly sized stack allocation for the formatted integer to be
written into.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Integer.html" class="trait"
title="trait itoa::Integer">Integer</a>  
An integer that can be written into an
[`itoa::Buffer`](struct.Buffer.html "struct itoa::Buffer").
