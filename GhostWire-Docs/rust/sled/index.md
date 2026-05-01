Crate sled Copy item path

<span class="sub-heading"><a href="../src/sled/lib.rs.html#1-527" class="src">Source</a>
</span>

Expand description

`sled` is a high-performance embedded database with an API that is
similar to a `BTreeMap<[u8], [u8]>`, but with several additional
capabilities for assisting creators of stateful systems.

It is fully thread-safe, and all operations are atomic. Multiple `Tree`s
with isolated keyspaces are supported with the
[`Db::open_tree`](struct.Db.html#method.open_tree) method.

ACID transactions involving reads and writes to multiple items are
supported with the
[`Tree::transaction`](struct.Tree.html#method.transaction) method.
Transactions may also operate over multiple `Tree`s (see
[`Tree::transaction`](struct.Tree.html#method.transaction) docs for more
info).

Users may also subscribe to updates on individual `Tree`s by using the
[`Tree::watch_prefix`](struct.Tree.html#method.watch_prefix) method,
which returns a blocking `Iterator` over updates to keys that begin with
the provided prefix. You may supply an empty prefix to subscribe to
everything.

[Merge operators](https://github.com/spacejam/sled/wiki/merge-operators)
(aka read-modify-write operators) are supported. A merge operator is a
function that specifies how new data can be merged into an existing
value without requiring both a read and a write. Using the
[`Tree::merge`](struct.Tree.html#method.merge) method, you may “push”
data to a `Tree` value and have the provided merge operator combine it
with the existing value, if there was one. They are set on a per-`Tree`
basis, and essentially allow any sort of data structure to be built
using merges as an atomic high-level operation.

`sled` is built by experienced database engineers who think users should
spend less time tuning and working against high-friction APIs. Expect
significant ergonomic and performance improvements over time. Most
surprises are bugs, so please [let us
know](mailto:t@jujit.su?subject=sled%20sucks!!!) if something is high
friction.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
let db: sled::Db = sled::open("my_db").unwrap();

// insert and get
db.insert(b"yo!", b"v1");
assert_eq!(&db.get(b"yo!").unwrap().unwrap(), b"v1");

// Atomic compare-and-swap.
db.compare_and_swap(
    b"yo!",      // key
    Some(b"v1"), // old value, None for not present
    Some(b"v2"), // new value, None for delete
)
.unwrap();

// Iterates over key-value pairs, starting at the given key.
let scan_key: &[u8] = b"a non-present key before yo!";
let mut iter = db.range(scan_key..);
assert_eq!(&iter.next().unwrap().unwrap().0, b"yo!");
assert_eq!(iter.next(), None);

db.remove(b"yo!");
assert_eq!(db.get(b"yo!"), Ok(None));

let other_tree: sled::Tree = db.open_tree(b"cool db facts").unwrap();
other_tree.insert(
    b"k1",
    &b"a Db acts like a Tree due to implementing Deref<Target = Tree>"[..]
).unwrap();
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::transaction::`<a href="transaction/trait.Transactional.html" class="trait"
title="trait sled::transaction::Transactional"><code>Transactional</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="transaction/index.html" class="mod"
title="mod sled::transaction">transaction</a>  
Fully serializable (ACID) multi-`Tree` transactions

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Batch.html" class="struct"
title="struct sled::Batch">Batch</a>  
A batch of updates that will be applied atomically to the Tree.

<a href="struct.CompareAndSwapError.html" class="struct"
title="struct sled::CompareAndSwapError">CompareAndSwapError</a>  
Compare and swap error.

<a href="struct.Config.html" class="struct"
title="struct sled::Config">Config</a>  
Top-level configuration for the system.

<a href="struct.Db.html" class="struct" title="struct sled::Db">Db</a>  
The `sled` embedded database! Implements `Deref<Target = sled::Tree>` to
refer to a default keyspace / namespace / bucket.

<a href="struct.IVec.html" class="struct"
title="struct sled::IVec">IVec</a>  
A buffer that may either be inline or remote and protected by an Arc

<a href="struct.Iter.html" class="struct"
title="struct sled::Iter">Iter</a>  
An iterator over keys and values in a `Tree`.

<a href="struct.Subscriber.html" class="struct"
title="struct sled::Subscriber">Subscriber</a>  
A subscriber listening on a specified prefix

<a href="struct.Tree.html" class="struct"
title="struct sled::Tree">Tree</a>  
A flash-sympathetic persistent lock-free B+ tree.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum sled::Error">Error</a>  
An Error type encapsulating various issues that may come up in the
operation of a `Db`.

<a href="enum.Event.html" class="enum"
title="enum sled::Event">Event</a>  
An event that happened to a key that a subscriber is interested in.

<a href="enum.Mode.html" class="enum" title="enum sled::Mode">Mode</a>  
The high-level database mode, according to the trade-offs of the RUM
conjecture.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.MergeOperator.html" class="trait"
title="trait sled::MergeOperator">MergeOperator</a>  
A function that may be configured on a particular shared `Tree` that
will be applied as a kind of read-modify-write operator to any values
that are written using the `Tree::merge` method.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.open.html" class="fn" title="fn sled::open">open</a>  
Opens a `Db` with a default configuration at the specified path. This
will create a new storage directory at the specified path if it does not
already exist. You can use the `Db::was_recovered` method to determine
if your database was recovered from a previous instance. You can use
`Config::create_new` if you want to increase the chances that the
database will be freshly created.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type sled::Result">Result</a>  
The top-level result type for dealing with fallible operations. The
errors tend to be fail-stop, and nested results are used in cases where
the outer fail-stop error can have try `?` used on it, exposing the
inner operation that is expected to fail under normal operation. The
philosophy behind this is detailed [on the sled
blog](https://sled.rs/errors).
