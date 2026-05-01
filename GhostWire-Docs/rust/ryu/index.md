Crate ryu Copy item path

<span class="sub-heading"><a href="../src/ryu/lib.rs.html#1-134" class="src">Source</a>
</span>

Expand description

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/ryu) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/ryu) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/ryu)

  

Pure Rust implementation of Ryū, an algorithm to quickly convert
floating point numbers to decimal strings.

The PLDI’18 paper [*Ryū: fast float-to-string
conversion*](https://dl.acm.org/citation.cfm?id=3192369) by Ulf Adams
includes a complete correctness proof of the algorithm. The paper is
available under the creative commons CC-BY-SA license.

This Rust implementation is a line-by-line port of Ulf Adams’
implementation in C, <https://github.com/ulfjack/ryu>.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
fn main() {
    let mut buffer = ryu::Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");
}
```

## <a href="#performance" class="doc-anchor">§</a>Performance

The [dtoa-benchmark](https://github.com/dtolnay/dtoa-benchmark) compares
this library and other Rust floating point formatting implementations
across a range of precisions. The vertical axis in this chart shows
nanoseconds taken by a single execution of
`ryu::Buffer::new().format_finite(value)` so a lower result indicates a
faster library.

![performance](https://raw.githubusercontent.com/dtolnay/ryu/master/dtoa-benchmark.png)

You can run upstream’s benchmarks with:

``` language-console
$ git clone https://github.com/ulfjack/ryu c-ryu
$ cd c-ryu
$ bazel run -c opt //ryu/benchmark
```

And the same benchmark against our implementation with:

``` language-console
$ git clone https://github.com/dtolnay/ryu rust-ryu
$ cd rust-ryu
$ cargo run --example upstream_benchmark --release
```

These benchmarks measure the average time to print a 32-bit float and
average time to print a 64-bit float, where the inputs are distributed
as uniform random bit patterns 32 and 64 bits wide.

The upstream C code, the unsafe direct Rust port, and the safe pretty
Rust API all perform the same, taking around 21 nanoseconds to format a
32-bit float and 31 nanoseconds to format a 64-bit float.

There is also a Rust-specific benchmark comparing this implementation to
the standard library which you can run with:

``` language-console
$ cargo bench
```

The benchmark shows Ryū approximately 2-5x faster than the standard
library across a range of f32 and f64 inputs. Measurements are in
nanoseconds per iteration; smaller is better.

## <a href="#formatting" class="doc-anchor">§</a>Formatting

This library tends to produce more human-readable output than the
standard library’s to\_string, which never uses scientific notation.
Here are two examples:

-   *ryu:* 1.23e40, *std:* 12300000000000000000000000000000000000000
-   *ryu:* 1.23e-40, *std:* 0.000000000000000000000000000000000000000123

Both libraries print short decimals such as 0.0000123 without scientific
notation.

# Modules<a href="#modules" class="anchor">§</a>

<a href="raw/index.html" class="mod" title="mod ryu::raw">raw</a>  
Unsafe functions that mirror the API of the C implementation of Ryū.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Buffer.html" class="struct"
title="struct ryu::Buffer">Buffer</a>  
Safe API for formatting floating point numbers to text.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Float.html" class="trait"
title="trait ryu::Float">Float</a>  
A floating point number, f32 or f64, that can be written into a
[`ryu::Buffer`](struct.Buffer.html "struct ryu::Buffer").
