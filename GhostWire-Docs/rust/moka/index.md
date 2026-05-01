Crate moka Copy item path

<span class="sub-heading"><a href="../src/moka/lib.rs.html#1-305" class="src">Source</a>
</span>

Expand description

Moka is a fast, concurrent cache library for Rust. Moka is inspired by
the [Caffeine](https://github.com/ben-manes/caffeine) library for Java.

Moka provides in-memory concurrent cache implementations on top of hash
maps. They support full concurrency of retrievals and a high expected
concurrency for updates. They utilize a lock-free concurrent hash table
as the central key-value storage.

All cache implementations perform a best-effort bounding of the map
using an entry replacement algorithm to determine which entries to evict
when the capacity is exceeded.

# <a href="#features" class="doc-anchor">§</a>Features

-   Thread-safe, highly concurrent in-memory cache implementations:
    -   Synchronous caches that can be shared across OS threads.
    -   An asynchronous (futures aware) cache.
-   A cache can be bounded by one of the followings:
    -   The maximum number of entries.
    -   The total weighted size of entries. (Size aware eviction)
-   Maintains near optimal hit ratio by using an entry replacement
    algorithms inspired by Caffeine:
    -   Admission to a cache is controlled by the Least Frequently Used
        (LFU) policy.
    -   Eviction from a cache is controlled by the Least Recently Used
        (LRU) policy.
    -   [More details and some benchmark results are available
        here](https://github.com/moka-rs/moka/wiki#admission-and-eviction-policies).
-   Supports expiration policies:
    -   Time to live.
    -   Time to idle.
    -   Per-entry variable expiration.
-   Supports eviction listener, a callback function that will be called
    when an entry is removed from the cache.

## <a href="#cache-policies" class="doc-anchor">§</a>Cache Policies

When a cache is full, it has to select and evict existing entries to
make some room. A cache policy is a strategy to determine which entry to
evict.

The choice of the cache policy may have a significant impact on the
performance of the cache. Because the time for cache misses is usually
much greater than the time for cache hits, the miss rate (number of
misses per second) has a significant impact on the performance.

Moka provides the following policies:

-   TinyLFU
-   LRU

### <a href="#tinylfu" class="doc-anchor">§</a>TinyLFU

TinyLFU is the default policy of the cache, and will be suitable for
most workloads.

TinyLFU is a combination of the LRU eviction policy and the LFU
admission policy. LRU stands for Least Recently Used, which is very
popular in many cache systems. LFU stands for Least Frequently Used.

![The lifecycle of cached entries with
TinyLFU](https://github.com/moka-rs/moka/wiki/images/benchmarks/moka-tiny-lfu.png)

With TinyLFU policy, the cache will admit a new entry based on its
popularity. If the key of the entry is popular, it will be admitted to
the cache. Otherwise, it will be rejected.

The popularity of the key is estimated by the historic popularity
estimator called LFU filter. It is a modified Count-Min Sketch, and it
can estimate the frequency of keys with a very low memory footprint
(thus the name “tiny”). Note that it tracks not only the keys currently
in the cache, but all hit and missed keys.

Once the entry is admitted to the cache, it will be evicted based on the
LRU policy. It evicts the least recently used entry from the cache.

TinyLFU will be suitable for most workloads, such as database, search,
and analytics.

### <a href="#lru" class="doc-anchor">§</a>LRU

LRU stands for Least Recently Used.

With LRU policy, the cache will evict the least recently used entry. It
is a simple policy and has been used in many cache systems.

LRU will be suitable for recency-biased workloads, such as job queues
and event streams.

# <a href="#examples" class="doc-anchor">§</a>Examples

See the following document:

-   Thread-safe, synchronous caches:
    -   [`sync::Cache`](./sync/struct.Cache.html)
    -   [`sync::SegmentedCache`](./sync/struct.SegmentedCache.html)
-   An asynchronous (futures aware) cache:
    -   [`future::Cache`](./future/struct.Cache.html) (Requires “future”
        feature)

**NOTE:** The following caches have been moved to a separate crate
called “[mini-moka](https://crates.io/crates/mini-moka)”.

-   Non concurrent cache for single threaded applications:
    -   `moka::unsync::Cache` →
        [`mini_moka::unsync::Cache`](https://docs.rs/mini-moka/latest/mini_moka/unsync/struct.Cache.html)
-   A simple, thread-safe, synchronous cache:
    -   `moka::dash::Cache` →
        [`mini_moka::sync::Cache`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.Cache.html)

# <a href="#minimum-supported-rust-versions" class="doc-anchor">§</a>Minimum Supported Rust Versions

This crate’s minimum supported Rust versions (MSRV) are the followings:

| Feature  |             MSRV             |
|:---------|:----------------------------:|
| `future` | Rust 1.71.1 (August 3, 2023) |
| `sync`   | Rust 1.71.1 (August 3, 2023) |

It will keep a rolling MSRV policy of at least 6 months. If the default
features with a mandatory features (`future` or `sync`) are enabled,
MSRV will be updated conservatively. When using other features, MSRV
might be updated more frequently, up to the latest stable.

In both cases, increasing MSRV is *not* considered a semver-breaking
change.

# <a href="#implementation-details" class="doc-anchor">§</a>Implementation Details

## <a href="#concurrency" class="doc-anchor">§</a>Concurrency

The entry replacement algorithms are kept eventually consistent with the
concurrent hash table. While updates to the cache are immediately
applied to the hash table, recording of reads and writes may not be
immediately reflected on the cache policy’s data structures.

These cache policy structures are guarded by a lock and operations are
applied in batches to avoid lock contention.

Recap:

-   The concurrent hash table in the cache is *strong consistent*:
    -   It is a lock-free data structure and immediately applies
        updates.
    -   It is guaranteed that the inserted entry will become visible
        immediately to all threads.
-   The cache policy’s data structures are *eventually consistent*:
    -   They are guarded by a lock and operations are applied in
        batches.
    -   An example of eventual consistency: the `entry_count` method may
        return an outdated value.

### <a href="#bounded-channels" class="doc-anchor">§</a>Bounded Channels

In order to hold the recordings of reads and writes until they are
applied to the cache policy’s data structures, the cache uses two
bounded channels, one for reads and the other for writes. Bounded means
that a channel have a maximum number of elements that can be stored.

These channels are drained when one of the following conditions is met:

-   The numbers of read or write recordings reach to the configured
    amounts.
    -   It is currently hard-coded to 64.
-   Or, the certain time past from the last draining.
    -   It is currently hard-coded to 300 milliseconds.

Cache does not have a dedicated thread for draining. Instead, it is done
by a user thread. When user code calls certain cache methods, such as
`get`, `get_with`, `insert`, and `run_pending_tasks`, the cache checks
if the above condition is met, and if so, it will start draining as a
part of the method call and apply the recordings to the cache policy’s
data structures. See [the Maintenance Tasks section](#maintenance-tasks)
for more details of applying the recordings.

### <a href="#when-a-bounded-channels-is-full" class="doc-anchor">§</a>When a Bounded Channels is Full

Under heavy concurrent operations from clients, draining may not be able
to catch up and the bounded channels can become full. In this case, the
cache will do one of the followings:

-   For the read channel, recordings of new reads will be discarded, so
    that retrievals will never be blocked. This behavior may have some
    impact to the hit rate of the cache.
-   For the write channel, updates from clients to the cache will be
    blocked until the draining task catches up.

## <a href="#maintenance-tasks" class="doc-anchor">§</a>Maintenance Tasks

When draining the read and write recordings from the channels, the cache
will do the following maintenance tasks:

1.  Determine whether to admit an entry to the cache or not, based on
    its popularity.
    -   If not, the entry is removed from the internal concurrent hash
        table.
2.  Apply the recording of cache reads and writes to the internal data
    structures for the cache policies, such as the LFU filter, LRU
    queues, and hierarchical timer wheels.
    -   The hierarchical timer wheels are used for the per-entry
        expiration policy.
3.  When cache’s max capacity is exceeded, remove least recently used
    (LRU) entries.
4.  Remove expired entries.
5.  Find and remove the entries that have been invalidated by the
    `invalidate_all` or `invalidate_entries_if` methods.
6.  Deliver removal notifications to the eviction listener. (Call the
    eviction listener closure with the information about the evicted
    entry)

The following cache method calls may trigger the maintenance tasks:

-   All cache write methods: `insert`, `get_with`, `invalidate`, etc.,
    except for `invalidate_all` and `invalidate_entries_if`.
-   Some of the cache read methods: `get`
-   `run_pending_tasks` method, which executes the pending maintenance
    tasks explicitly.

Except `run_pending_tasks` method, the maintenance tasks are executed
lazily when one of the conditions in the [Bounded
Channels](#bounded-channels) section is met.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use policy::`<a href="policy/trait.Expiry.html" class="trait"
title="trait moka::policy::Expiry"><code>Expiry</code></a>`;`

`pub use policy::`<a href="policy/struct.Policy.html" class="struct"
title="struct moka::policy::Policy"><code>Policy</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="notification/index.html" class="mod"
title="mod moka::notification">notification</a>  
Common data types for notifications.

<a href="ops/index.html" class="mod" title="mod moka::ops">ops</a>  
Cache operations.

<a href="policy/index.html" class="mod"
title="mod moka::policy">policy</a>  
<a href="sync/index.html" class="mod" title="mod moka::sync">sync</a>  
Provides thread-safe, concurrent cache implementations.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Entry.html" class="struct"
title="struct moka::Entry">Entry</a>  
A snapshot of a single entry in the cache.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.PredicateError.html" class="enum"
title="enum moka::PredicateError">PredicateError</a>  
The error type for the functionalities around
[`Cache::invalidate_entries_if`](./sync/struct.Cache.html#method.invalidate_entries_if)
method.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Equivalent.html" class="trait"
title="trait moka::Equivalent">Equivalent</a>  
Key equivalence trait.
