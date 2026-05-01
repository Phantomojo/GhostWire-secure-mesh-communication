Crate simd\_adler32 Copy item path

<span class="sub-heading"><a href="../src/simd_adler32/lib.rs.html#1-321" class="src">Source</a>
</span>

Expand description

# <a href="#simd-adler32" class="doc-anchor">§</a>simd-adler32

A SIMD-accelerated Adler-32 hash algorithm implementation.

## <a href="#features" class="doc-anchor">§</a>Features

-   No dependencies
-   Support `no_std` (with `default-features = false`)
-   Runtime CPU feature detection (when `std` enabled)
-   Blazing fast performance on as many targets as possible (currently
    only x86 and x86\_64)
-   Default to scalar implementation when simd not available

## <a href="#quick-start" class="doc-anchor">§</a>Quick start

> Cargo.toml

``` language-toml
[dependencies]
simd-adler32 = "*"
```

> example.rs

``` {.rust .rust-example-rendered}
use simd_adler32::Adler32;

let mut adler = Adler32::new();
adler.write(b"rust is pretty cool, man");
let hash = adler.finish();

println!("{}", hash);
// 1921255656
```

## <a href="#feature-flags" class="doc-anchor">§</a>Feature flags

-   `std` - Enabled by default

Enables std support, see [CPU Feature Detection](#cpu-feature-detection)
for runtime detection support.

-   `nightly`

Enables nightly features required for avx512 support.

-   `const-generics` - Enabled by default

Enables const-generics support allowing for user-defined array hashing
by value. See
[`Adler32Hash`](trait.Adler32Hash.html "trait simd_adler32::Adler32Hash")
for details.

## <a href="#support" class="doc-anchor">§</a>Support

**CPU Features**

| impl | arch             | feature |
|------|------------------|---------|
| ✅   | `x86`, `x86_64`  | avx512  |
| ✅   | `x86`, `x86_64`  | avx2    |
| ✅   | `x86`, `x86_64`  | ssse3   |
| ✅   | `x86`, `x86_64`  | sse2    |
| 🚧   | `arm`, `aarch64` | neon    |
|      | `wasm32`         | simd128 |

**MSRV** `1.36.0`\*\*

Minimum supported rust version is tested before a new version is
published. \[\*\*\] Feature `const-generics` needs to disabled to build
on rustc versions `<1.51` which can be done by updating your dependency
definition to the following.

## <a href="#cpu-feature-detection" class="doc-anchor">§</a>CPU Feature Detection

simd-adler32 supports both runtime and compile time CPU feature
detection using the `std::is_x86_feature_detected` macro when the
`Adler32` struct is instantiated with the `new` fn.

Without `std` feature enabled simd-adler32 falls back to compile time
feature detection using `target-feature` or `target-cpu` flags supplied
to rustc. See
<https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html>
for more information.

Feature detection tries to use the fastest supported feature first.

# Modules<a href="#modules" class="anchor">§</a>

<a href="bufread/index.html" class="mod"
title="mod simd_adler32::bufread">bufread</a>  
BufRead-based hashing.

<a href="read/index.html" class="mod"
title="mod simd_adler32::read">read</a>  
Reader-based hashing.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Adler32.html" class="struct"
title="struct simd_adler32::Adler32">Adler32</a>  
An adler32 hash generator type.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Adler32Hash.html" class="trait"
title="trait simd_adler32::Adler32Hash">Adler32Hash</a>  
A Adler-32 hash-able type.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.adler32.html" class="fn"
title="fn simd_adler32::adler32">adler32</a>  
Compute Adler-32 hash on `Adler32Hash` type.
