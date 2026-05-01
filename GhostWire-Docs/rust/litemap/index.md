Crate litemap Copy item path

<span class="sub-heading"><a href="../src/litemap/lib.rs.html#5-83" class="src">Source</a>
</span>

Expand description

# <a href="#litemap" class="doc-anchor">§</a>`litemap`

`litemap` is a crate providing
[`LiteMap`](struct.LiteMap.html "struct litemap::LiteMap"), a highly
simplistic “flat” key-value map based off of a single sorted vector.

The main goal of this crate is to provide a map that is good enough for
small sizes, and does not carry the binary size impact of
[`HashMap`](https://doc.rust-lang.org/1.93.1/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")
or [`BTreeMap`](alloc::collections::BTreeMap).

If binary size is not a concern,
[`std::collections::BTreeMap`](https://doc.rust-lang.org/1.93.1/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")
may be a better choice for your use case. It behaves very similarly to
[`LiteMap`](struct.LiteMap.html "struct litemap::LiteMap") for less than
12 elements, and upgrades itself gracefully for larger inputs.

## <a href="#performance-characteristics" class="doc-anchor">§</a>Performance characteristics

[`LiteMap`](struct.LiteMap.html "struct litemap::LiteMap") is a data
structure with similar characteristics as
[`std::collections::BTreeMap`](https://doc.rust-lang.org/1.93.1/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")
but with slightly different trade-offs as it’s implemented on top of a
flat storage (e.g. Vec).

-   [`LiteMap`](struct.LiteMap.html "struct litemap::LiteMap") iteration
    is generally faster than `BTreeMap` because of the flat storage.
-   [`LiteMap`](struct.LiteMap.html "struct litemap::LiteMap") can be
    pre-allocated whereas `BTreeMap` can’t.
-   [`LiteMap`](struct.LiteMap.html "struct litemap::LiteMap") has a
    smaller memory footprint than `BTreeMap` for small collections (&lt;
    20 items).
-   Lookup is `O(log(n))` like `BTreeMap`.
-   Insertion is generally `O(n)`, but optimized to `O(1)` if the new
    item sorts greater than the current items. In `BTreeMap` it’s
    `O(log(n))`.
-   Deletion is `O(n)` whereas `BTreeMap` is `O(log(n))`.
-   Bulk operations like `from_iter`, `extend` and deserialization have
    an optimized `O(n)` path for inputs that are ordered and
    `O(n*log(n))` complexity otherwise.

## <a href="#pluggable-backends" class="doc-anchor">§</a>Pluggable Backends

By default, [`LiteMap`](struct.LiteMap.html "struct litemap::LiteMap")
is backed by a [`Vec`](alloc::vec::Vec); however, it can be backed by
any appropriate random-access data store, giving that data store a
map-like interface. See the
[`store`](store/index.html "mod litemap::store") module for more
details.

## <a href="#const-construction" class="doc-anchor">§</a>Const construction

[`LiteMap`](struct.LiteMap.html "struct litemap::LiteMap") supports
const construction from any store that is const-constructible, such as a
static slice, via
[`LiteMap::from_sorted_store_unchecked()`](struct.LiteMap.html#method.from_sorted_store_unchecked "associated function litemap::LiteMap::from_sorted_store_unchecked").
This also makes
[`LiteMap`](struct.LiteMap.html "struct litemap::LiteMap") suitable for
use with \[`databake`\]. See
[`impl Bake for LiteMap`](./struct.LiteMap.html#impl-Bake-for-LiteMap%3CK,+V,+S%3E)
for more details.

# Modules<a href="#modules" class="anchor">§</a>

<a href="store/index.html" class="mod"
title="mod litemap::store">store</a>  
Traits for pluggable LiteMap backends.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.LiteMap.html" class="struct"
title="struct litemap::LiteMap">LiteMap</a>  
A simple “flat” map based on a sorted vector

<a href="struct.OccupiedEntry.html" class="struct"
title="struct litemap::OccupiedEntry">OccupiedEntry</a>  
A view into an occupied entry in a `LiteMap`.

<a href="struct.VacantEntry.html" class="struct"
title="struct litemap::VacantEntry">VacantEntry</a>  
A view into a vacant entry in a `LiteMap`.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Entry.html" class="enum"
title="enum litemap::Entry">Entry</a>  
An entry in a `LiteMap`, which may be either occupied or vacant.
