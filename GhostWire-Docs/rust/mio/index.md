Crate mio Copy item path

<span class="sub-heading"><a href="../src/mio/lib.rs.html#1-285" class="src">Source</a>
</span>

Expand description

Mio is a fast, low-level I/O library for Rust focusing on non-blocking
APIs and event notification for building high performance I/O apps with
as little overhead as possible over the OS abstractions.

# <a href="#usage" class="doc-anchor">§</a>Usage

Using Mio starts by creating a [`Poll`](../mio/struct.Poll.html), which
reads events from the OS and puts them into
[`Events`](../mio/event/struct.Events.html). You can handle I/O events
from the OS with it.

For more detail, see [`Poll`](../mio/struct.Poll.html).

## <a href="#examples" class="doc-anchor">§</a>Examples

Examples can be found in the `examples` directory of the source code, or
[on GitHub](https://github.com/tokio-rs/mio/tree/master/examples).

## <a href="#guide" class="doc-anchor">§</a>Guide

A getting started guide is available in the
[`guide`](guide/index.html "mod mio::guide") module.

## <a href="#available-features" class="doc-anchor">§</a>Available features

The available features are described in the
[`features`](features/index.html "mod mio::features") module.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use event::`<a href="event/struct.Events.html" class="struct"
title="struct mio::event::Events"><code>Events</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="event/index.html" class="mod" title="mod mio::event">event</a>  
Readiness event types and utilities.

<a href="features/index.html" class="mod"
title="mod mio::features">features</a>  
Mio’s optional features.

<a href="guide/index.html" class="mod" title="mod mio::guide">guide</a>  
Getting started guide.

<a href="net/index.html" class="mod" title="mod mio::net">net</a>  
Networking primitives.

<a href="unix/index.html" class="mod" title="mod mio::unix">unix</a>  
Unix only extensions.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Interest.html" class="struct"
title="struct mio::Interest">Interest</a>  
Interest used in registering.

<a href="struct.Poll.html" class="struct"
title="struct mio::Poll">Poll</a>  
Polls for readiness events on all registered values.

<a href="struct.Registry.html" class="struct"
title="struct mio::Registry">Registry</a>  
Registers I/O resources.

<a href="struct.Token.html" class="struct"
title="struct mio::Token">Token</a>  
Associates readiness events with
[`event::Source`](./event/trait.Source.html)s.

<a href="struct.Waker.html" class="struct"
title="struct mio::Waker">Waker</a>  
Waker allows cross-thread waking of [`Poll`](struct.Poll.html).
