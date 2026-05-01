Crate zmij Copy item path

<span class="sub-heading"><a href="../src/zmij/lib.rs.html#1-1454" class="src">Source</a>
</span>

Expand description

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/zmij) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/zmij) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/zmij)

  

A double-to-string conversion algorithm based on
[Schubfach](https://fmt.dev/papers/Schubfach4.pdf) and
[yy](https://github.com/ibireme/c_numconv_benchmark/blob/master/vendor/yy_double/yy_double.c).

This Rust implementation is a line-by-line port of Victor Zverovich’s
implementation in C++, <https://github.com/vitaut/zmij>.

  

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
fn main() {
    let mut buffer = zmij::Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");
}
```

  

## <a href="#performance" class="doc-anchor">§</a>Performance

The [dtoa-benchmark](https://github.com/dtolnay/dtoa-benchmark) compares
this library and other Rust floating point formatting implementations
across a range of precisions. The vertical axis in this chart shows
nanoseconds taken by a single execution of
`zmij::Buffer::new().format_finite(value)` so a lower result indicates a
faster library.

![performance](https://raw.githubusercontent.com/dtolnay/zmij/master/dtoa-benchmark.png)

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Buffer.html" class="struct"
title="struct zmij::Buffer">Buffer</a>  
Safe API for formatting floating point numbers to text.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Float.html" class="trait"
title="trait zmij::Float">Float</a>  
A floating point number, f32 or f64, that can be written into a
[`zmij::Buffer`](struct.Buffer.html "struct zmij::Buffer").
