Crate arc\_swap Copy item path

<span class="sub-heading"><a href="../src/arc_swap/lib.rs.html#1-1328" class="src">Source</a>
</span>

Expand description

Making
[`Arc`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")
itself atomic

The [`ArcSwap`](type.ArcSwap.html "type arc_swap::ArcSwap") type is a
container for an `Arc` that can be changed atomically. Semantically, it
is similar to something like `Atomic<Arc<T>>` (if there was such a
thing) or `RwLock<Arc<T>>` (but without the need for the locking). It is
optimized for read-mostly scenarios, with consistent performance
characteristics.

# <a href="#motivation" class="doc-anchor">§</a>Motivation

There are many situations in which one might want to have some data
structure that is often read and seldom updated. Some examples might be
a configuration of a service, routing tables, snapshot of some data that
is renewed every few minutes, etc.

In all these cases one needs:

-   Being able to read the current value of the data structure, fast,
    often and concurrently from many threads.
-   Using the same version of the data structure over longer period of
    time ‒ a query should be answered by a consistent version of data, a
    packet should be routed either by an old or by a new version of the
    routing table but not by a combination, etc.
-   Perform an update without disrupting the processing.

The first idea would be to use
[`RwLock<T>`](https://doc.rust-lang.org/std/sync/struct.RwLock.html) and
keep a read-lock for the whole time of processing. Update would,
however, pause all processing until done.

Better option would be to have
[`RwLock<Arc<T>>`](https://doc.rust-lang.org/std/sync/struct.RwLock.html).
Then one would lock, clone the
[Arc](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")
and unlock. This suffers from CPU-level contention (on the lock and on
the reference count of the
[Arc](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc"))
which makes it relatively slow. Depending on the implementation, an
update may be blocked for arbitrary long time by a steady inflow of
readers.

``` {.rust .rust-example-rendered}
static ROUTING_TABLE: Lazy<RwLock<Arc<RoutingTable>>> = Lazy::new(|| {
    RwLock::new(Arc::new(RoutingTable))
});

fn process_packet(packet: Packet) {
    let table = Arc::clone(&ROUTING_TABLE.read().unwrap());
    table.route(packet);
}
```

The [ArcSwap](type.ArcSwap.html "type arc_swap::ArcSwap") can be used
instead, which solves the above problems and has better performance
characteristics than the
[RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html), both in
contended and non-contended scenarios.

``` {.rust .rust-example-rendered}
static ROUTING_TABLE: Lazy<ArcSwap<RoutingTable>> = Lazy::new(|| {
    ArcSwap::from_pointee(RoutingTable)
});

fn process_packet(packet: Packet) {
    let table = ROUTING_TABLE.load();
    table.route(packet);
}
```

# <a href="#crate-contents" class="doc-anchor">§</a>Crate contents

At the heart of the crate there are
[`ArcSwap`](type.ArcSwap.html "type arc_swap::ArcSwap") and
[`ArcSwapOption`](type.ArcSwapOption.html "type arc_swap::ArcSwapOption")
types, containers for an
[`Arc`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")
and
[`Option<Arc>`](https://doc.rust-lang.org/1.93.1/core/option/enum.Option.html "enum core::option::Option").

Technically, these are type aliases for partial instantiations of the
[`ArcSwapAny`](struct.ArcSwapAny.html "struct arc_swap::ArcSwapAny")
type. The
[`ArcSwapAny`](struct.ArcSwapAny.html "struct arc_swap::ArcSwapAny") is
more flexible and allows tweaking of many things (can store other things
than
[`Arc`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")s,
can configure the locking
[`Strategy`](strategy/trait.Strategy.html "trait arc_swap::strategy::Strategy")).
For details about the tweaking, see the documentation of the
[`strategy`](strategy/index.html "mod arc_swap::strategy") module and
the [`RefCnt`](trait.RefCnt.html "trait arc_swap::RefCnt") trait.

The [`cache`](cache/index.html "mod arc_swap::cache") module provides
means for speeding up read access of the contained data at the cost of
delayed reclamation.

The [`access`](access/index.html "mod arc_swap::access") module can be
used to do projections into the contained data to separate parts of
application from each other (eg. giving a component access to only its
own part of configuration while still having it reloaded as a whole).

# <a href="#before-using" class="doc-anchor">§</a>Before using

The data structure is a bit niche. Before using, please check the
[limitations and common
pitfalls](docs/limitations/index.html "mod arc_swap::docs::limitations")
and the [performance
characteristics](docs/performance/index.html "mod arc_swap::docs::performance"),
including choosing the right [read
operation](docs/performance/index.html#read-operations "mod arc_swap::docs::performance").

You can also get an inspiration about what’s possible in the [common
patterns](docs/patterns/index.html "mod arc_swap::docs::patterns")
section.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use std::sync::Arc;

use arc_swap::ArcSwap;
use crossbeam_utils::thread;

let config = ArcSwap::from(Arc::new(String::default()));
thread::scope(|scope| {
    scope.spawn(|_| {
        let new_conf = Arc::new("New configuration".to_owned());
        config.store(new_conf);
    });
    for _ in 0..10 {
        scope.spawn(|_| {
            loop {
                let cfg = config.load();
                if !cfg.is_empty() {
                    assert_eq!(**cfg, "New configuration");
                    return;
                }
            }
        });
    }
}).unwrap();
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::cache::`<a href="cache/struct.Cache.html" class="struct"
title="struct arc_swap::cache::Cache"><code>Cache</code></a>`;`

`pub use crate::strategy::`<a href="strategy/type.DefaultStrategy.html" class="type"
title="type arc_swap::strategy::DefaultStrategy"><code>DefaultStrategy</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="access/index.html" class="mod"
title="mod arc_swap::access">access</a>  
Abstracting over accessing parts of stored value.

<a href="cache/index.html" class="mod"
title="mod arc_swap::cache">cache</a>  
Caching handle into the
[ArcSwapAny](struct.ArcSwapAny.html "struct arc_swap::ArcSwapAny").

<a href="docs/index.html" class="mod"
title="mod arc_swap::docs">docs</a>  
Additional documentation.

<a href="strategy/index.html" class="mod"
title="mod arc_swap::strategy">strategy</a>  
Strategies for protecting the reference counts.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ArcSwapAny.html" class="struct"
title="struct arc_swap::ArcSwapAny">ArcSwapAny</a>  
An atomic storage for a reference counted smart pointer like
[`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) or
`Option<Arc>`.

<a href="struct.Guard.html" class="struct"
title="struct arc_swap::Guard">Guard</a>  
A temporary storage of the pointer.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsRaw.html" class="trait"
title="trait arc_swap::AsRaw">AsRaw</a>  
A trait describing things that can be turned into a raw pointer.

<a href="trait.RefCnt.html" class="trait"
title="trait arc_swap::RefCnt">RefCnt</a>  
A trait describing smart reference counted pointers.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ArcSwap.html" class="type"
title="type arc_swap::ArcSwap">ArcSwap</a>  
An atomic storage for `Arc`.

<a href="type.ArcSwapOption.html" class="type"
title="type arc_swap::ArcSwapOption">ArcSwapOption</a>  
An atomic storage for `Option<Arc>`.
