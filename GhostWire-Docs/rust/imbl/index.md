Crate imbl Copy item path

<span class="sub-heading"><a href="../src/imbl/lib.rs.html#5-482" class="src">Source</a>
</span>

Expand description

# <a href="#immutable-data-structures-for-rust" class="doc-anchor">§</a>Immutable Data Structures for Rust

This library implements several of the more commonly useful immutable
data structures for Rust.

## <a href="#what-are-immutable-data-structures" class="doc-anchor">§</a>What are immutable data structures?

Immutable data structures are data structures which can be copied and
modified efficiently without altering the original. The most
uncomplicated example of this is the venerable [cons
list](https://en.wikipedia.org/wiki/Cons#Lists). This crate offers a
selection of more modern and flexible data structures with similar
properties, tuned for the needs of Rust developers.

Briefly, the following data structures are provided:

-   [Vectors](type.Vector.html "type imbl::Vector") based on [RRB
    trees](https://infoscience.epfl.ch/record/213452/files/rrbvector.pdf)
-   [Hash
    maps](type.HashMap.html "type imbl::HashMap")/[sets](type.HashSet.html "type imbl::HashSet")
    based on [hash array mapped
    tries](https://en.wikipedia.org/wiki/Hash_array_mapped_trie)
-   [Ordered
    maps](type.OrdMap.html "type imbl::OrdMap")/[sets](type.OrdSet.html "type imbl::OrdSet")
    based on [B+trees](https://en.wikipedia.org/wiki/B%2B_tree)

## <a href="#why-would-i-want-this" class="doc-anchor">§</a>Why Would I Want This?

While immutable data structures can be a game changer for other
programming languages, the most obvious benefit - avoiding the
accidental mutation of data - is already handled so well by Rust’s type
system that it’s just not something a Rust programmer needs to worry
about even when using data structures that would send a conscientious
Clojure programmer into a panic.

Immutable data structures offer other benefits, though, some of which
are useful even in a language like Rust. The most prominent is
*structural sharing*, which means that if two data structures are mostly
copies of each other, most of the memory they take up will be shared
between them. This implies that making copies of an immutable data
structure is cheap: it’s really only a matter of copying a pointer and
increasing a reference counter, where in the case of
[`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")
you have to allocate the same amount of memory all over again and make a
copy of every element it contains. For immutable data structures, extra
memory isn’t allocated until you modify either the copy or the original,
and then only the memory needed to record the difference.

Another goal of this library has been the idea that you shouldn’t even
have to think about what data structure to use in any given situation,
until the point where you need to start worrying about optimisation -
which, in practice, often never comes. Beyond the shape of your data
(ie. whether to use a list or a map), it should be fine not to think too
carefully about data structures - you can just pick the one that has the
right shape and it should have acceptable performance characteristics
for every operation you might need. Specialised data structures will
always be faster at what they’ve been specialised for, but `imbl` aims
to provide the data structures which deliver the least chance of
accidentally using them for the wrong thing.

For instance,
[`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")
beats everything at memory usage, indexing and operations that happen at
the back of the list, but is terrible at insertion and removal, and gets
worse the closer to the front of the list you get.
[`VecDeque`](https://doc.rust-lang.org/1.93.1/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")
adds a little bit of complexity in order to make operations at the front
as efficient as operations at the back, but is still bad at insertion
and especially concatenation.
[`Vector`](type.Vector.html "type imbl::Vector") adds another bit of
complexity, and could never match
[`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")
at what it’s best at, but in return every operation you can throw at it
can be completed in a reasonable amount of time - even normally
expensive operations like copying and especially concatenation are
reasonably cheap when using a
[`Vector`](type.Vector.html "type imbl::Vector").

It should be noted, however, that because of its simplicity,
[`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")
actually beats [`Vector`](type.Vector.html "type imbl::Vector") even at
its strongest operations at small sizes, just because modern CPUs are
hyperoptimised for things like copying small chunks of contiguous
memory - you actually need to go past a certain size (usually in the
vicinity of several hundred elements) before you get to the point where
[`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")
isn’t always going to be the fastest choice.
[`Vector`](type.Vector.html "type imbl::Vector") attempts to overcome
this by actually just being an array at very small sizes, and being able
to switch efficiently to the full data structure when it grows large
enough. Thus, [`Vector`](type.Vector.html "type imbl::Vector") will
actually be equivalent to
[Vec](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")
until it grows past the size of a single chunk.

The maps - [`HashMap`](type.HashMap.html "type imbl::HashMap") and
[`OrdMap`](type.OrdMap.html "type imbl::OrdMap") - generally perform
similarly to their equivalents in the standard library, but tend to run
a bit slower on the basic operations
([`HashMap`](type.HashMap.html "type imbl::HashMap") is almost neck and
neck with its counterpart, while
[`OrdMap`](type.OrdMap.html "type imbl::OrdMap") currently tends to run
2-3x slower). On the other hand, they offer the cheap copy and
structural sharing between copies that you’d expect from immutable data
structures.

In conclusion, the aim of this library is to provide a safe default
choice for the most common kinds of data structures, allowing you to
defer careful thinking about the right data structure for the job until
you need to start looking for optimisations - and you may find,
especially for larger data sets, that immutable data structures are
still the right choice.

## <a href="#values" class="doc-anchor">§</a>Values

Because we need to make copies of shared nodes in these data structures
before updating them, the values you store in them must implement
[`Clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html "trait core::clone::Clone").
For primitive values that implement
[`Copy`](https://doc.rust-lang.org/1.93.1/core/marker/trait.Copy.html "trait core::marker::Copy"),
such as numbers, everything is fine: this is the case for which the data
structures are optimised, and performance is going to be great.

On the other hand, if you want to store values for which cloning is
expensive, or values that don’t implement
[`Clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html "trait core::clone::Clone"),
you need to wrap them in
[`Rc`](https://doc.rust-lang.org/1.93.1/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")
or
[`Arc`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc").
Thus, if you have a complex structure `BigBlobOfData` and you want to
store a list of them as a `Vector<BigBlobOfData>`, you should instead
use a `Vector<Rc<BigBlobOfData>>`, which is going to save you not only
the time spent cloning the big blobs of data, but also the memory spent
keeping multiple copies of it around, as
[`Rc`](https://doc.rust-lang.org/1.93.1/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")
keeps a single reference counted copy around instead.

If you’re storing smaller values that aren’t
[`Copy`](https://doc.rust-lang.org/1.93.1/core/marker/trait.Copy.html "trait core::marker::Copy")able,
you’ll need to exercise judgement: if your values are going to be very
cheap to clone, as would be the case for short
[`String`](https://doc.rust-lang.org/1.93.1/alloc/string/struct.String.html "struct alloc::string::String")
or small
[`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")s,
you’re probably better off storing them directly without wrapping them
in an
[`Rc`](https://doc.rust-lang.org/1.93.1/alloc/rc/struct.Rc.html "struct alloc::rc::Rc"),
because, like the
[`Rc`](https://doc.rust-lang.org/1.93.1/alloc/rc/struct.Rc.html "struct alloc::rc::Rc"),
you might actually lose more performance from the extra redirection of
wrapping them in an
[`Rc`](https://doc.rust-lang.org/1.93.1/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")
than you would from occasionally cloning them.

### <a href="#when-does-cloning-happen" class="doc-anchor">§</a>When does cloning happen?

So when will your values actually be cloned? The easy answer is only if
you
[`clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html#tymethod.clone "method core::clone::Clone::clone")
the data structure itself, and then only lazily as you change it. Values
are stored in tree nodes inside the data structure, each node of which
contains up to 64 values. When you
[`clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html#tymethod.clone "method core::clone::Clone::clone")
a data structure, nothing is actually copied - it’s just the reference
count on the root node that’s incremented, to indicate that it’s shared
between two data structures. It’s only when you actually modify one of
the shared data structures that nodes are cloned: when you make a change
somewhere in the tree, the node containing the change needs to be
cloned, and then its parent nodes need to be updated to contain the new
child node instead of the old version, and so they’re cloned as well.

We can call this “lazy” cloning - if you make two copies of a data
structure and you never change either of them, there’s never any need to
clone the data they contain. It’s only when you start making changes
that cloning starts to happen, and then only on the specific tree nodes
that are part of the change. Note that the implications of lazily
cloning the data structure extend to memory usage as well as the CPU
workload of copying the data around - cloning an immutable data
structure means both copies share the same allocated memory, until you
start making changes.

Most crucially, if you never clone the data structure, the data inside
it is also never cloned, and in this case it acts just like a mutable
data structure, with minimal performance differences (but still
non-zero, as we still have to check for shared nodes).

## <a href="#data-structures" class="doc-anchor">§</a>Data Structures

We’ll attempt to provide a comprehensive guide to the available data
structures below.

### <a href="#performance-notes" class="doc-anchor">§</a>Performance Notes

“Big O notation” is the standard way of talking about the time
complexity of data structure operations. If you’re not familiar with big
O notation, here’s a quick cheat sheet:

*O(1)* means an operation runs in constant time: it will take the same
time to complete regardless of the size of the data structure.

*O(n)* means an operation runs in linear time: if you double the size of
your data structure, the operation will take twice as long to complete;
if you quadruple the size, it will take four times as long, etc.

*O(log n)* means an operation runs in logarithmic time: for
*log<sub>2</sub>*, if you double the size of your data structure, the
operation will take one step longer to complete; if you quadruple the
size, it will need two steps more; and so on. However, the data
structures in this library generally run in *log<sub>64</sub>* time,
meaning you have to make your data structure 64 times bigger to need one
extra step, and 4096 times bigger to need two steps. This means that,
while they still count as O(log n), operations on all but really large
data sets will run at near enough to O(1) that you won’t usually notice.

*O(n log n)* is the most expensive operation you’ll see in this library:
it means that for every one of the *n* elements in your data structure,
you have to perform *log n* operations. In our case, as noted above,
this is often close enough to O(n) that it’s not usually as bad as it
sounds, but even O(n) isn’t cheap and the cost still increases
logarithmically, if slowly, as the size of your data increases. O(n log
n) basically means “are you sure you need to do this?”

*O(1)*\* means ‘amortised O(1),’ which means that an operation usually
runs in constant time but will occasionally be more expensive: for
instance,
[`Vector::push_back`](struct.GenericVector.html#method.push_back "method imbl::GenericVector::push_back"),
if called in sequence, will be O(1) most of the time but every 64th time
it will be O(log n), as it fills up its tail chunk and needs to insert
it into the tree. Please note that the O(1) with the asterisk attached
is not a common notation; it’s just a convention I’ve used in these docs
to save myself from having to type ‘amortised’ everywhere.

### <a href="#lists" class="doc-anchor">§</a>Lists

Lists are sequences of single elements which maintain the order in which
you inserted them. The only list in this library is
[`Vector`](type.Vector.html "type imbl::Vector"), which offers the best
all round performance characteristics: it’s pretty good at everything,
even if there’s always another kind of list that’s better at something.

| Type | Algorithm | Constraints | Order | Push | Pop | Split | Append | Lookup |
|----|----|----|----|----|----|----|----|----|
| [`Vector<A>`](type.Vector.html "type imbl::Vector") | [RRB tree](https://infoscience.epfl.ch/record/213452/files/rrbvector.pdf) | [`Clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html "trait core::clone::Clone") | insertion | O(1)\* | O(1)\* | O(log n) | O(log n) | O(log n) |

### <a href="#maps" class="doc-anchor">§</a>Maps

Maps are mappings of keys to values, where the most common read
operation is to find the value associated with a given key. Maps may or
may not have a defined order. Any given key can only occur once inside a
map, and setting a key to a different value will overwrite the previous
value.

| Type | Algorithm | Key Constraints | Order | Insert | Remove | Lookup |
|----|----|----|----|----|----|----|
| [`HashMap<K, V>`](type.HashMap.html "type imbl::HashMap") | [HAMT](https://en.wikipedia.org/wiki/Hash_array_mapped_trie) | [`Clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html "trait core::clone::Clone") + [`Hash`](https://doc.rust-lang.org/1.93.1/core/hash/trait.Hash.html "trait core::hash::Hash") + [`Eq`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.Eq.html "trait core::cmp::Eq") | undefined | O(log n) | O(log n) | O(log n) |
| [`OrdMap<K, V>`](type.OrdMap.html "type imbl::OrdMap") | [B+tree](https://en.wikipedia.org/wiki/B%2B_tree) | [`Clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html "trait core::clone::Clone") + [`Ord`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.Ord.html "trait core::cmp::Ord") | sorted | O(log n) | O(log n) | O(log n) |

### <a href="#sets" class="doc-anchor">§</a>Sets

Sets are collections of unique values, and may or may not have a defined
order. Their crucial property is that any given value can only exist
once in a given set.

| Type | Algorithm | Constraints | Order | Insert | Remove | Lookup |
|----|----|----|----|----|----|----|
| [`HashSet<A>`](type.HashSet.html "type imbl::HashSet") | [HAMT](https://en.wikipedia.org/wiki/Hash_array_mapped_trie) | [`Clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html "trait core::clone::Clone") + [`Hash`](https://doc.rust-lang.org/1.93.1/core/hash/trait.Hash.html "trait core::hash::Hash") + [`Eq`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.Eq.html "trait core::cmp::Eq") | undefined | O(log n) | O(log n) | O(log n) |
| [`OrdSet<A>`](type.OrdSet.html "type imbl::OrdSet") | [B+tree](https://en.wikipedia.org/wiki/B%2B_tree) | [`Clone`](https://doc.rust-lang.org/1.93.1/core/clone/trait.Clone.html "trait core::clone::Clone") + [`Ord`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.Ord.html "trait core::cmp::Ord") | sorted | O(log n) | O(log n) | O(log n) |

## <a href="#in-place-mutation" class="doc-anchor">§</a>In-place Mutation

All of these data structures support in-place copy-on-write mutation,
which means that if you’re the sole user of a data structure, you can
update it in place without taking the performance hit of making a copy
of the data structure before modifying it (this is about an order of
magnitude faster than immutable operations, almost as fast as
[`std::collections`](https://doc.rust-lang.org/1.93.1/std/collections/index.html "mod std::collections")’s
mutable data structures).

Thanks to
[`Rc`](https://doc.rust-lang.org/1.93.1/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")’s
reference counting, we are able to determine whether a node in a data
structure is being shared with other data structures, or whether it’s
safe to mutate it in place. When it’s shared, we’ll automatically make a
copy of the node before modifying it. The consequence of this is that
cloning a data structure becomes a lazy operation: the initial clone is
instant, and as you modify the cloned data structure it will clone
chunks only where you change them, so that if you change the entire
thing you will eventually have performed a full clone.

This also gives us a couple of other optimisations for free:
implementations of immutable data structures in other languages often
have the idea of local mutation, like Clojure’s transients or Haskell’s
`ST` monad - a managed scope where you can treat an immutable data
structure like a mutable one, gaining a considerable amount of
performance because you no longer need to copy your changed nodes for
every operation, just the first time you hit a node that’s sharing
structure. In Rust, we don’t need to think about this kind of managed
scope, it’s all taken care of behind the scenes because of our low level
access to the garbage collector (which, in our case, is just a simple
[`Rc`](https://doc.rust-lang.org/1.93.1/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")).

## <a href="#thread-safety" class="doc-anchor">§</a>Thread Safety

The data structures in `imbl` are thread safe by default using
[`Arc`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc").
However, `imbl` also supports `Rc` as the pointer type through the
[`archery`](../archery/index.html "mod archery") crate, just like
`im-rc` in the original `im` crate. If you prioritise speed over thread
safety, you can use
[`GenericVector<T, shared_pointer::RcK>`](struct.GenericVector.html "struct imbl::GenericVector")
that uses non-threadsafe but faster `Rc`, instead of the type alias
[`Vector`](type.Vector.html "type imbl::Vector"). You can also create
your own type alias for that. It can be done on other types too.

## <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

`imbl` comes with optional support for the following crates through
Cargo feature flags. You can enable them in your `Cargo.toml` file like
this:

``` language-no_compile
[dependencies]
imbl = { version = "*", features = ["proptest", "serde", "bincode"] }
```

| Feature | Description |
|----|----|
| [`proptest`](https://crates.io/crates/proptest) | Strategies for all `imbl` datatypes under a `proptest` namespace, eg. `imbl::vector::proptest::vector()` |
| [`quickcheck`](https://crates.io/crates/quickcheck) | [`quickcheck::Arbitrary`](https://docs.rs/quickcheck/latest/quickcheck/trait.Arbitrary.html) implementations for all `imbl` datatypes |
| [`rayon`](https://crates.io/crates/rayon) | parallel iterator implementations for [`Vector`](type.Vector.html "type imbl::Vector") |
| [`serde`](https://crates.io/crates/serde) | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) implementations for all `imbl` datatypes |
| [`bincode`](https://crates.io/crates/bincode) | [`Encode`](https://docs.rs/bincode/latest/bincode/enc/trait.Encode.html) and [`Decode`](https://docs.rs/bincode/latest/bincode/de/trait.Decode.html) implementations for all `imbl` datatypes |
| [`arbitrary`](https://crates.io/crates/arbitrary/) | [`arbitrary::Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) implementations for all `imbl` datatypes |
| [`triomphe`](https://crates.io/crates/triomphe/) | Use [`triomphe::Arc`](https://docs.rs/triomphe/latest/triomphe/struct.Arc.html) for the default shared pointer. This is a drop-in replacement for [`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) that is faster in some cases. |

# Modules<a href="#modules" class="anchor">§</a>

<a href="hashmap/index.html" class="mod"
title="mod imbl::hashmap">hashmap</a>  
An unordered map.

<a href="hashset/index.html" class="mod"
title="mod imbl::hashset">hashset</a>  
An unordered set.

<a href="iter/index.html" class="mod" title="mod imbl::iter">iter</a>  
Iterators over immutable data.

<a href="ordmap/index.html" class="mod"
title="mod imbl::ordmap">ordmap</a>  
An ordered map.

<a href="ordset/index.html" class="mod"
title="mod imbl::ordset">ordset</a>  
An ordered set.

<a href="shared_ptr/index.html" class="mod"
title="mod imbl::shared_ptr">shared_ptr</a>  
About shared pointers. Re-export the
[`archery`](https://docs.rs/archery/latest/) crate.

<a href="vector/index.html" class="mod"
title="mod imbl::vector">vector</a>  
A persistent vector.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.get_in.html" class="macro"
title="macro imbl::get_in">get_in</a>  
Get a value inside multiple levels of data structures.

<a href="macro.hashmap.html" class="macro"
title="macro imbl::hashmap">hashmap</a>  
Construct a hash map from a sequence of key/value pairs.

<a href="macro.hashset.html" class="macro"
title="macro imbl::hashset">hashset</a>  
Construct a set from a sequence of values.

<a href="macro.ordmap.html" class="macro"
title="macro imbl::ordmap">ordmap</a>  
Construct a map from a sequence of key/value pairs.

<a href="macro.ordset.html" class="macro"
title="macro imbl::ordset">ordset</a>  
Construct a set from a sequence of values.

<a href="macro.update_in.html" class="macro"
title="macro imbl::update_in">update_in</a>  
Update a value inside multiple levels of data structures.

<a href="macro.vector.html" class="macro"
title="macro imbl::vector">vector</a>  
Construct a vector from a sequence of elements.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.GenericHashMap.html" class="struct"
title="struct imbl::GenericHashMap">GenericHashMap</a>  
An unordered map.

<a href="struct.GenericHashSet.html" class="struct"
title="struct imbl::GenericHashSet">GenericHashSet</a>  
An unordered set.

<a href="struct.GenericOrdMap.html" class="struct"
title="struct imbl::GenericOrdMap">GenericOrdMap</a>  
An ordered map.

<a href="struct.GenericOrdSet.html" class="struct"
title="struct imbl::GenericOrdSet">GenericOrdSet</a>  
An ordered set.

<a href="struct.GenericVector.html" class="struct"
title="struct imbl::GenericVector">GenericVector</a>  
A persistent vector.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.HashMap.html" class="type"
title="type imbl::HashMap">HashMap</a>  
Type alias for
[`GenericHashMap`](struct.GenericHashMap.html "struct imbl::GenericHashMap")
that uses
[`std::hash::RandomState`](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.RandomState.html)
as the default hasher and
[`DefaultSharedPtr`](shared_ptr/type.DefaultSharedPtr.html "type imbl::shared_ptr::DefaultSharedPtr")
as the pointer type.

<a href="type.HashSet.html" class="type"
title="type imbl::HashSet">HashSet</a>  
Type alias for
[`GenericHashSet`](struct.GenericHashSet.html "struct imbl::GenericHashSet")
that uses
[`std::hash::RandomState`](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.RandomState.html)
as the default hasher and
[`DefaultSharedPtr`](shared_ptr/type.DefaultSharedPtr.html "type imbl::shared_ptr::DefaultSharedPtr")
as the pointer type.

<a href="type.OrdMap.html" class="type"
title="type imbl::OrdMap">OrdMap</a>  
Type alias for
[`GenericOrdMap`](struct.GenericOrdMap.html "struct imbl::GenericOrdMap")
that uses
[`DefaultSharedPtr`](shared_ptr/type.DefaultSharedPtr.html "type imbl::shared_ptr::DefaultSharedPtr")
as the pointer type.

<a href="type.OrdSet.html" class="type"
title="type imbl::OrdSet">OrdSet</a>  
Type alias for
[`GenericOrdSet`](struct.GenericOrdSet.html "struct imbl::GenericOrdSet")
that uses
[`DefaultSharedPtr`](shared_ptr/type.DefaultSharedPtr.html "type imbl::shared_ptr::DefaultSharedPtr")
as the pointer type.

<a href="type.Vector.html" class="type"
title="type imbl::Vector">Vector</a>  
Type alias for
[`GenericVector`](struct.GenericVector.html "struct imbl::GenericVector")
that uses
[`DefaultSharedPtr`](shared_ptr/type.DefaultSharedPtr.html "type imbl::shared_ptr::DefaultSharedPtr")
as the pointer type.
