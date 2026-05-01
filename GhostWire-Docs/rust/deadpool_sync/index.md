Crate deadpool\_sync Copy item path

<span class="sub-heading"><a href="../src/deadpool_sync/lib.rs.html#1-202" class="src">Source</a>
</span>

Expand description

# <a
href="#deadpool-for-synchroneous-code-latest-version-unsafe-forbidden-rust-175"
class="doc-anchor">§</a>Deadpool for synchroneous code [![Latest Version](https://img.shields.io/crates/v/deadpool-sync.svg)](https://crates.io/crates/deadpool-sync) ![Unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg "Unsafe forbidden") [![Rust 1.75+](https://img.shields.io/badge/rustc-1.75+-lightgray.svg "Rust 1.75+")](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)

Deadpool is a dead simple async pool for connections and objects of any
type.

This crates provides helpers for writing pools for objects that don’t
support async and need to be run inside a thread.

**Note:** This crate is intended for making the development of
`deadpool-*` crates easier. Other libraries and binary projects normally
should not use this directly and use some provided reexports by the
crates using it.

## <a href="#features" class="doc-anchor">§</a>Features

| Feature | Description | Extra dependencies | Default |
|----|----|----|----|
| `tracing` | Enable support for [tracing](https://github.com/tokio-rs/tracing) by propagating Spans in the `interact()` calls. Enable this if you use the `tracing` crate and you want to get useful traces from within `interact()` calls. | `tracing` | no |

## <a href="#license" class="doc-anchor">§</a>License

Licensed under either of

-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    <http://www.apache.org/licenses/LICENSE-2.0)>
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or
    <http://opensource.org/licenses/MIT)>

at your option.

# Modules<a href="#modules" class="anchor">§</a>

<a href="reexports/index.html" class="mod"
title="mod deadpool_sync::reexports">reexports</a>  
This module contains all things that should be reexported by backend
implementations in order to avoid direct dependencies on the `deadpool`
crate itself.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.SyncGuard.html" class="struct"
title="struct deadpool_sync::SyncGuard">SyncGuard</a>  
This guard is returned when calling `SyncWrapper::lock` or
`SyncWrapper::try_lock`. This is basicly just a wrapper around a
`MutexGuard` but hides some implementation details.

<a href="struct.SyncWrapper.html" class="struct"
title="struct deadpool_sync::SyncWrapper">SyncWrapper</a>  
Wrapper for objects which only provides blocking functions that need to
be called on a separate thread.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.InteractError.html" class="enum"
title="enum deadpool_sync::InteractError">InteractError</a>  
Possible errors returned when
[`SyncWrapper::interact()`](struct.SyncWrapper.html#method.interact "method deadpool_sync::SyncWrapper::interact")
fails.
