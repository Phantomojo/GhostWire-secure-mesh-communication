Crate matrixmultiply Copy item path

<span class="sub-heading"><a href="../src/matrixmultiply/lib.rs.html#8-188" class="src">Source</a>
</span>

Expand description

General matrix multiplication for f32, f64, and complex matrices.
Operates on matrices with general layout (they can use arbitrary row and
column stride).

This crate uses the same macro/microkernel approach to matrix
multiplication as the [BLIS](https://github.com/flame/blis) project.

We presently provide a few good microkernels, portable and for x86-64
and AArch64 NEON, and only one operation: the general matrix-matrix
multiplication (“gemm”).

## <a href="#matrix-representation" class="doc-anchor">§</a>Matrix Representation

**matrixmultiply** supports matrices with general stride, so a matrix is
passed using a pointer and four integers:

-   `a: *const f32`, pointer to the first element in the matrix
-   `m: usize`, number of rows
-   `k: usize`, number of columns
-   `rsa: isize`, row stride
-   `csa: isize`, column stride

In this example, A is a m by k matrix. `a` is a pointer to the element
at index *0, 0*.

The *row stride* is the pointer offset (in number of elements) to the
element on the next row. It’s the distance from element *i, j* to *i +
1, j*.

The *column stride* is the pointer offset (in number of elements) to the
element in the next column. It’s the distance from element *i, j* to *i,
j + 1*.

For example for a contiguous matrix, row major strides are *rsa=k,
csa=1* and column major strides are *rsa=1, csa=m*.

Strides can be negative or even zero, but for a mutable matrix elements
may not alias each other.

## <a href="#portability-and-performance" class="doc-anchor">§</a>Portability and Performance

-   The default kernels are written in portable Rust and available on
    all targets. These may depend on autovectorization to perform well.

-   *x86* and *x86-64* features can be detected at runtime by default or
    compile time (if enabled), and the following kernel variants are
    implemented:

    -   `fma`
    -   `avx`
    -   `sse2`

-   *aarch64* features can be detected at runtime by default or compile
    time (if enabled), and the following kernel variants are
    implemented:

    -   `neon`

## <a href="#features" class="doc-anchor">§</a>Features

### <a href="#std" class="doc-anchor">§</a>`std`

`std` is enabled by default.

This crate can be used without the standard library (`#![no_std]`) by
disabling the default `std` feature. To do so, use this in your
`Cargo.toml`:

``` language-toml
matrixmultiply = { version = "0.3", default-features = false }
```

Runtime CPU feature detection is available **only** when `std` is
enabled. Without the `std` feature, the crate uses special CPU features
only if they are enabled at compile time. (To enable CPU features at
compile time, pass the relevant
[`target-cpu`](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu)
or
[`target-feature`](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-feature)
option to `rustc`.)

### <a href="#threading" class="doc-anchor">§</a>`threading`

`threading` is an optional crate feature

Threading enables multithreading for the operations. The environment
variable `MATMUL_NUM_THREADS` decides how many threads are used at
maximum. At the moment 1-4 are supported and the default is the number
of physical cpus (as detected by `num_cpus`).

### <a href="#cgemm" class="doc-anchor">§</a>`cgemm`

`cgemm` is an optional crate feature.

It enables the `cgemm` and `zgemm` methods for complex matrix
multiplication. This is an **experimental feature** and not yet as
performant as the float kernels on x86.

The complex representation we use is `[f64; 2]`.

### <a href="#constconf" class="doc-anchor">§</a>`constconf`

`constconf` is an optional feature. When enabled, cache-sensitive
parameters of the gemm implementations can be tweaked *at compile time*
by defining the following variables:

-   `MATMUL_SGEMM_MC` (And so on, for S, D, C, ZGEMM and with NC, KC or
    MC).

## <a href="#other-notes" class="doc-anchor">§</a>Other Notes

The functions in this crate are thread safe, as long as the destination
matrix is distinct.

## <a href="#rust-version" class="doc-anchor">§</a>Rust Version

This version requires Rust 1.41.1 or later; the crate follows a
carefully considered upgrade policy, where updating the minimum Rust
version is not a breaking change.

Some features are enabled with later versions: from Rust 1.61 AArch64
NEON support.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.CGemmOption.html" class="enum"
title="enum matrixmultiply::CGemmOption">CGemmOption</a>  
cgemm/zgemm per-operand options

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.cgemm.html" class="fn"
title="fn matrixmultiply::cgemm">cgemm</a><sup>⚠</sup>  
General matrix multiplication (complex f32)

<a href="fn.dgemm.html" class="fn"
title="fn matrixmultiply::dgemm">dgemm</a><sup>⚠</sup>  
General matrix multiplication (f64)

<a href="fn.sgemm.html" class="fn"
title="fn matrixmultiply::sgemm">sgemm</a><sup>⚠</sup>  
General matrix multiplication (f32)

<a href="fn.zgemm.html" class="fn"
title="fn matrixmultiply::zgemm">zgemm</a><sup>⚠</sup>  
General matrix multiplication (complex f64)
