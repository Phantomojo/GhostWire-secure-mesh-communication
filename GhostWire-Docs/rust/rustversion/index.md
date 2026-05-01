Crate rustversion Copy item path

<span class="sub-heading"><a href="../src/rustversion/lib.rs.html#1-282" class="src">Source</a>
</span>

Expand description

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/rustversion) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/rustversion) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/rustversion)

  

This crate provides macros for conditional compilation according to
rustc compiler version, analogous to
[`#[cfg(...)]`](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute)
and
[`#[cfg_attr(...)]`](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute).

  

# <a href="#selectors" class="doc-anchor">§</a>Selectors

-   **`#[rustversion::stable]`** —  
    True on any stable compiler.

-   **`#[rustversion::stable(1.34)]`** —  
    True on exactly the specified stable compiler.

-   **`#[rustversion::beta]`** —  
    True on any beta compiler.

-   **`#[rustversion::nightly]`** —  
    True on any nightly compiler or dev build.

-   **`#[rustversion::nightly(2025-01-01)]`** —  
    True on exactly one nightly.

-   **`#[rustversion::since(1.34)]`** —  
    True on that stable release and any later compiler, including beta
    and nightly.

-   **`#[rustversion::since(2025-01-01)]`** —  
    True on that nightly and all newer ones.

-   **`#[rustversion::before(`***version or date***`)]`** —  
    Negative of *\#\[rustversion::since(...)\]*.

-   **`#[rustversion::not(`***selector***`)]`** —  
    Negative of any selector; for example
    *\#\[rustversion::not(nightly)\]*.

-   **`#[rustversion::any(`***selectors...***`)]`** —  
    True if any of the comma-separated selectors is true; for example
    *\#\[rustversion::any(stable, beta)\]*.

-   **`#[rustversion::all(`***selectors...***`)]`** —  
    True if all of the comma-separated selectors are true; for example
    *\#\[rustversion::all(since(1.31), before(1.34))\]*.

-   **`#[rustversion::attr(`***selector***`, `***attribute***`)]`** —  
    For conditional inclusion of attributes; analogous to `cfg_attr`.

-   **`rustversion::cfg!(`***selector***`)`** —  
    An expression form of any of the above attributes; for example *if
    rustversion::cfg!(any(stable, beta)) { ... }*.

  

# <a href="#use-cases" class="doc-anchor">§</a>Use cases

Providing additional trait impls as types are stabilized in the standard
library without breaking compatibility with older compilers; in this
case Pin&lt;P&gt; stabilized in [Rust
1.33](https://blog.rust-lang.org/2019/02/28/Rust-1.33.0.html#pinning):

``` {.rust .rust-example-rendered}
#[rustversion::since(1.33)]
use std::pin::Pin;

#[rustversion::since(1.33)]
impl<P: MyTrait> MyTrait for Pin<P> {
    /* ... */
}
```

Similar but for language features; the ability to control alignment
greater than 1 of packed structs was stabilized in [Rust
1.33](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1330-2019-02-28).

``` {.rust .rust-example-rendered}
#[rustversion::attr(before(1.33), repr(packed))]
#[rustversion::attr(since(1.33), repr(packed(2)))]
struct Six(i16, i32);

fn main() {
    println!("{}", std::mem::align_of::<Six>());
}
```

Augmenting code with `const` as const impls are stabilized in the
standard library. This use of `const` as an attribute is recognized as a
special case by the rustversion::attr macro.

``` {.rust .rust-example-rendered}
use std::time::Duration;

#[rustversion::attr(since(1.32), const)]
fn duration_as_days(dur: Duration) -> u64 {
    dur.as_secs() / 60 / 60 / 24
}
```

Emitting Cargo cfg directives from a build script. Note that this
requires listing `rustversion` under `[build-dependencies]` in
Cargo.toml, not `[dependencies]`.

``` {.rust .rust-example-rendered}
// build.rs

fn main() {
    if rustversion::cfg!(since(1.36)) {
        println!("cargo:rustc-cfg=no_std");
    }
}
```

``` {.rust .rust-example-rendered}
// src/lib.rs

#![cfg_attr(no_std, no_std)]

#[cfg(no_std)]
extern crate alloc;
```

  

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.cfg.html" class="macro"
title="macro rustversion::cfg">cfg</a>

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.all.html" class="attr"
title="attr rustversion::all">all</a>

<a href="attr.any.html" class="attr"
title="attr rustversion::any">any</a>

<a href="attr.attr.html" class="attr"
title="attr rustversion::attr">attr</a>

<a href="attr.before.html" class="attr"
title="attr rustversion::before">before</a>

<a href="attr.beta.html" class="attr"
title="attr rustversion::beta">beta</a>

<a href="attr.nightly.html" class="attr"
title="attr rustversion::nightly">nightly</a>

<a href="attr.not.html" class="attr"
title="attr rustversion::not">not</a>

<a href="attr.since.html" class="attr"
title="attr rustversion::since">since</a>

<a href="attr.stable.html" class="attr"
title="attr rustversion::stable">stable</a>
