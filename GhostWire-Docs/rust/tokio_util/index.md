Crate tokio\_util Copy item path

<span class="sub-heading"><a href="../src/tokio_util/lib.rs.html#1-65" class="src">Source</a>
</span>

Expand description

Utilities for working with Tokio.

This crate is not versioned in lockstep with the core
[`tokio`](../tokio/index.html "mod tokio") crate. However, `tokio-util`
*will* respect Rust’s semantic versioning policy, especially with regard
to breaking changes.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../bytes/index.html" class="mod"
title="mod bytes"><code>bytes</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="codec/index.html" class="mod"
title="mod tokio_util::codec">codec</a>  
Adaptors from `AsyncRead`/`AsyncWrite` to Stream/Sink

<a href="compat/index.html" class="mod"
title="mod tokio_util::compat">compat</a>  
Compatibility between the `tokio::io` and `futures-io` versions of the
`AsyncRead` and `AsyncWrite` traits.

<a href="either/index.html" class="mod"
title="mod tokio_util::either">either</a>  
Module defining an Either type.

<a href="future/index.html" class="mod"
title="mod tokio_util::future">future</a>  
An extension trait for Futures that provides a variety of convenient
adapters.

<a href="io/index.html" class="mod" title="mod tokio_util::io">io</a>  
Helpers for IO related tasks.

<a href="sync/index.html" class="mod"
title="mod tokio_util::sync">sync</a>  
Synchronization primitives
