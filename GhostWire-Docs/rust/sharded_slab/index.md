Crate sharded\_slab Copy item path

<span class="sub-heading"><a href="../src/sharded_slab/lib.rs.html#1-1106" class="src">Source</a>
</span>

Expand description

A lock-free concurrent slab.

Slabs provide pre-allocated storage for many instances of a single data
type. When a large number of values of a single type are required, this
can be more efficient than allocating each item individually. Since the
allocated items are the same size, memory fragmentation is reduced, and
creating and removing new items can be very cheap.

This crate implements a lock-free concurrent slab, indexed by `usize`s.

## <a href="#usage" class="doc-anchor">§</a>Usage

First, add this to your `Cargo.toml`:

``` language-toml
sharded-slab = "0.1.1"
```

This crate provides two types, [`Slab`](https://crates.io/crates/loom)
and [`Pool`](struct.Pool.html "struct sharded_slab::Pool"), which
provide slightly different APIs for using a sharded slab.

[`Slab`](https://crates.io/crates/loom) implements a slab for *storing*
small types, sharing them between threads, and accessing them by index.
New entries are allocated by
[inserting](struct.Slab.html#method.insert "method sharded_slab::Slab::insert")
data, moving it in by value. Similarly, entries may be deallocated by
[taking](struct.Slab.html#method.take "method sharded_slab::Slab::take")
from the slab, moving the value out. This API is similar to a
`Vec<Option<T>>`, but allowing lock-free concurrent insertion and
removal.

In contrast, the [`Pool`](struct.Pool.html "struct sharded_slab::Pool")
type provides an [object
pool](https://en.wikipedia.org/wiki/Object_pool_pattern) style API for
*reusing storage*. Rather than constructing values and moving them into
the pool, as with [`Slab`](https://crates.io/crates/loom), [allocating
an
entry](struct.Pool.html#method.create "method sharded_slab::Pool::create")
from the pool takes a closure that’s provided with a mutable reference
to initialize the entry in place. When entries are deallocated, they are
[cleared](trait.Clear.html "trait sharded_slab::Clear") in place. Types
which own a heap allocation can be cleared by dropping any *data* they
store, but retaining any previously-allocated capacity. This means that
a [`Pool`](struct.Pool.html "struct sharded_slab::Pool") may be used to
reuse a set of existing heap allocations, reducing allocator load.

# <a href="#examples" class="doc-anchor">§</a>Examples

Inserting an item into the slab, returning an index:

``` {.rust .rust-example-rendered}
let slab = Slab::new();

let key = slab.insert("hello world").unwrap();
assert_eq!(slab.get(key).unwrap(), "hello world");
```

To share a slab across threads, it may be wrapped in an `Arc`:

``` {.rust .rust-example-rendered}
use std::sync::Arc;
let slab = Arc::new(Slab::new());

let slab2 = slab.clone();
let thread2 = std::thread::spawn(move || {
    let key = slab2.insert("hello from thread two").unwrap();
    assert_eq!(slab2.get(key).unwrap(), "hello from thread two");
    key
});

let key1 = slab.insert("hello from thread one").unwrap();
assert_eq!(slab.get(key1).unwrap(), "hello from thread one");

// Wait for thread 2 to complete.
let key2 = thread2.join().unwrap();

// The item inserted by thread 2 remains in the slab.
assert_eq!(slab.get(key2).unwrap(), "hello from thread two");
```

If items in the slab must be mutated, a `Mutex` or `RwLock` may be used
for each item, providing granular locking of items rather than of the
slab:

``` {.rust .rust-example-rendered}
use std::sync::{Arc, Mutex};
let slab = Arc::new(Slab::new());

let key = slab.insert(Mutex::new(String::from("hello world"))).unwrap();

let slab2 = slab.clone();
let thread2 = std::thread::spawn(move || {
    let hello = slab2.get(key).expect("item missing");
    let mut hello = hello.lock().expect("mutex poisoned");
    *hello = String::from("hello everyone!");
});

thread2.join().unwrap();

let hello = slab.get(key).expect("item missing");
let mut hello = hello.lock().expect("mutex poisoned");
assert_eq!(hello.as_str(), "hello everyone!");
```

# <a href="#configuration" class="doc-anchor">§</a>Configuration

For performance reasons, several values used by the slab are calculated
as constants. In order to allow users to tune the slab’s parameters, we
provide a [`Config`](trait.Config.html) trait which defines these
parameters as associated `consts`. The `Slab` type is generic over a
`C: Config` parameter.

# <a href="#comparison-with-similar-crates" class="doc-anchor">§</a>Comparison with Similar Crates

-   [`slab`](https://crates.io/crates/loom): Carl Lerche’s `slab` crate
    provides a slab implementation with a similar API, implemented by
    storing all data in a single vector.

    Unlike `sharded_slab`, inserting and removing elements from the slab
    requires mutable access. This means that if the slab is accessed
    concurrently by multiple threads, it is necessary for it to be
    protected by a `Mutex` or `RwLock`. Items may not be inserted or
    removed (or accessed, if a `Mutex` is used) concurrently, even when
    they are unrelated. In many cases, the lock can become a significant
    bottleneck. On the other hand, this crate allows separate indices in
    the slab to be accessed, inserted, and removed concurrently without
    requiring a global lock. Therefore, when the slab is shared across
    multiple threads, this crate offers significantly better performance
    than `slab`.

    However, the lock free slab introduces some additional
    constant-factor overhead. This means that in use-cases where a slab
    is *not* shared by multiple threads and locking is not required,
    this crate will likely offer slightly worse performance.

    In summary: `sharded-slab` offers significantly improved performance
    in concurrent use-cases, while `slab` should be preferred in
    single-threaded use-cases.

# <a href="#safety-and-correctness" class="doc-anchor">§</a>Safety and Correctness

Most implementations of lock-free data structures in Rust require some
amount of unsafe code, and this crate is not an exception. In order to
catch potential bugs in this unsafe code, we make use of
[`loom`](https://crates.io/crates/loom), a permutation-testing tool for
concurrent Rust programs. All `unsafe` blocks this crate occur in
accesses to `loom` `UnsafeCell`s. This means that when those accesses
occur in this crate’s tests, `loom` will assert that they are valid
under the C11 memory model across multiple permutations of concurrent
executions of those tests.

In order to guard against the [ABA
problem](https://en.wikipedia.org/wiki/ABA_problem), this crate makes
use of *generational indices*. Each slot in the slab tracks a generation
counter which is incremented every time a value is inserted into that
slot, and the indices returned by
[`Slab::insert`](struct.Slab.html#method.insert) include the generation
of the slot when the value was inserted, packed into the high-order bits
of the index. This ensures that if a value is inserted, removed, and a
new value is inserted into the same slot in the slab, the key returned
by the first call to `insert` will not map to the new value.

Since a fixed number of bits are set aside to use for storing the
generation counter, the counter will wrap around after being incremented
a number of times. To avoid situations where a returned index lives long
enough to see the generation counter wrap around to the same value, it
is good to be fairly generous when configuring the allocation of index
bits.

# <a href="#performance" class="doc-anchor">§</a>Performance

These graphs were produced by
[benchmarks](https://github.com/hawkw/sharded-slab/blob/master/benches/bench.rs)
of the sharded slab implementation, using the
[`criterion`](https://crates.io/crates/criterion) crate.

The first shows the results of a benchmark where an increasing number of
items are inserted and then removed into a slab concurrently by five
threads. It compares the performance of the sharded slab implementation
with a `RwLock<slab::Slab>`:

<img
src="https://user-images.githubusercontent.com/2796466/66078398-cd6c9f80-e516-11e9-9923-0ed6292e8498.png"
width="1124" alt="Screen Shot 2019-10-01 at 5 09 49 PM" />

The second graph shows the results of a benchmark where an increasing
number of items are inserted and then removed by a *single* thread. It
compares the performance of the sharded slab implementation with an
`RwLock<slab::Slab>` and a `mut slab::Slab`.

<img
src="https://user-images.githubusercontent.com/2796466/66078469-f0974f00-e516-11e9-95b5-f65f0aa7e494.png"
width="925" alt="Screen Shot 2019-10-01 at 5 13 45 PM" />

These benchmarks demonstrate that, while the sharded approach introduces
a small constant-factor overhead, it offers significantly better
performance across concurrent accesses.

# <a href="#implementation-notes" class="doc-anchor">§</a>Implementation Notes

See [this
page](implementation/index.html "mod sharded_slab::implementation") for
details on this crate’s design and implementation.

# Modules<a href="#modules" class="anchor">§</a>

<a href="implementation/index.html" class="mod"
title="mod sharded_slab::implementation">implementation</a>  
Notes on `sharded-slab`’s implementation and design.

<a href="pool/index.html" class="mod"
title="mod sharded_slab::pool">pool</a>  
A lock-free concurrent object pool.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DefaultConfig.html" class="struct"
title="struct sharded_slab::DefaultConfig">DefaultConfig</a>  
Default slab configuration values.

<a href="struct.Entry.html" class="struct"
title="struct sharded_slab::Entry">Entry</a>  
A handle that allows access to an occupied entry in a
[`Slab`](struct.Slab.html "struct sharded_slab::Slab").

<a href="struct.OwnedEntry.html" class="struct"
title="struct sharded_slab::OwnedEntry">OwnedEntry</a>  
An owned reference to an occupied entry in a
[`Slab`](struct.Slab.html "struct sharded_slab::Slab").

<a href="struct.Pool.html" class="struct"
title="struct sharded_slab::Pool">Pool</a>  
A lock-free concurrent object pool.

<a href="struct.Slab.html" class="struct"
title="struct sharded_slab::Slab">Slab</a>  
A sharded slab.

<a href="struct.UniqueIter.html" class="struct"
title="struct sharded_slab::UniqueIter">UniqueIter</a>  
An exclusive fused iterator over the items in a
[`Slab`](struct.Slab.html "struct sharded_slab::Slab").

<a href="struct.VacantEntry.html" class="struct"
title="struct sharded_slab::VacantEntry">VacantEntry</a>  
A handle to a vacant entry in a
[`Slab`](struct.Slab.html "struct sharded_slab::Slab").

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Clear.html" class="trait"
title="trait sharded_slab::Clear">Clear</a>  
Trait implemented by types which can be cleared in place, retaining any
allocated memory.

<a href="trait.Config.html" class="trait"
title="trait sharded_slab::Config">Config</a>  
Configuration parameters which can be overridden to tune the behavior of
a slab.
