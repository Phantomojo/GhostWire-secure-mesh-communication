Crate deadpool\_runtime Copy item path

<span class="sub-heading"><a href="../src/deadpool_runtime/lib.rs.html#1-130"
class="src">Source</a> </span>

Expand description

# <a
href="#deadpool-runtime-abstraction-latest-version-unsafe-forbidden-rust-175"
class="doc-anchor">§</a>Deadpool runtime abstraction [![Latest Version](https://img.shields.io/crates/v/deadpool-runtime.svg)](https://crates.io/crates/deadpool-runtime) ![Unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg "Unsafe forbidden") [![Rust 1.75+](https://img.shields.io/badge/rustc-1.75+-lightgray.svg "Rust 1.75+")](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)

Deadpool is a dead simple async pool for connections and objects of any
type.

This crate provides a simple `Runtime` enum that can be used to target
multiple runtimes. This crate avoids boxed futures and and only
implements things actually needed by the `deadpool` crates.

**Note:** This crate is intended for making the development of
`deadpool-*` crates easier. Other libraries and binary projects normally
should not use this directly and use some provided reexports by the
crates using it.

## <a href="#features" class="doc-anchor">§</a>Features

| Feature | Description | Extra dependencies | Default |
|----|----|----|----|
| `tokio_1` | Enable support for [tokio](https://crates.io/crates/tokio) crate | `tokio/time`, `tokio/rt` | no |
| `async-std_1` | Enable support for [async-std](https://crates.io/crates/config) crate | `async-std` | no |

## <a href="#license" class="doc-anchor">§</a>License

Licensed under either of

-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    <http://www.apache.org/licenses/LICENSE-2.0)>
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or
    <http://opensource.org/licenses/MIT)>

at your option.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Runtime.html" class="enum"
title="enum deadpool_runtime::Runtime">Runtime</a>  
Enumeration for picking a runtime implementation.

<a href="enum.SpawnBlockingError.html" class="enum"
title="enum deadpool_runtime::SpawnBlockingError">SpawnBlockingError</a>  
Error of spawning a task on a thread where blocking is acceptable.
