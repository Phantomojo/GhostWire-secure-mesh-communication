Crate indexmap Copy item path

<span class="sub-heading"><a href="../src/indexmap/lib.rs.html#1-283" class="src">Source</a>
</span>

Expand description

[`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
is a hash table where the iteration order of the key-value pairs is
independent of the hash values of the keys.

[`IndexSet`](set/struct.IndexSet.html "struct indexmap::set::IndexSet")
is a corresponding hash set using the same implementation and with
similar properties.

### <a href="#highlights" class="doc-anchor">§</a>Highlights

[`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
and
[`IndexSet`](set/struct.IndexSet.html "struct indexmap::set::IndexSet")
are drop-in compatible with the std `HashMap` and `HashSet`, but they
also have some features of note:

-   The ordering semantics (see their documentation for details)
-   Sorting methods and the
    [`.pop()`](map/struct.IndexMap.html#method.pop "method indexmap::map::IndexMap::pop")
    methods.
-   The
    [`Equivalent`](trait.Equivalent.html "trait indexmap::Equivalent")
    trait, which offers more flexible equality definitions between
    borrowed and owned versions of keys.
-   The
    [`MutableKeys`](map/trait.MutableKeys.html "trait indexmap::map::MutableKeys")
    trait, which gives opt-in mutable access to map keys, and
    [`MutableValues`](set/trait.MutableValues.html "trait indexmap::set::MutableValues")
    for sets.

### <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

To reduce the amount of compiled code in the crate by default, certain
features are gated behind [feature
flags](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section).
These allow you to opt in to (or out of) functionality. Below is a list
of the features available in this crate.

-   `std`: Enables features which require the Rust standard library. For
    more information see the section on
    [`no_std`](#no-standard-library-targets).
-   `rayon`: Enables parallel iteration and other parallel methods.
-   `serde`: Adds implementations for
    [`Serialize`](../serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize")
    and
    [`Deserialize`](../serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")
    to
    [`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
    and
    [`IndexSet`](set/struct.IndexSet.html "struct indexmap::set::IndexSet").
    Alternative implementations for (de)serializing
    [`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
    as an ordered sequence are available in the
    [`map::serde_seq`](map/serde_seq/index.html "mod indexmap::map::serde_seq")
    module.
-   `arbitrary`: Adds implementations for the
    [`arbitrary::Arbitrary`](%60::arbitrary::Arbitrary%60) trait to
    [`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
    and
    [`IndexSet`](set/struct.IndexSet.html "struct indexmap::set::IndexSet").
-   `quickcheck`: Adds implementations for the
    [`quickcheck::Arbitrary`](%60::quickcheck::Arbitrary%60) trait to
    [`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
    and
    [`IndexSet`](set/struct.IndexSet.html "struct indexmap::set::IndexSet").
-   `borsh` (**deprecated**): Adds implementations for
    [`BorshSerialize`](%60::borsh::BorshSerialize%60) and
    [`BorshDeserialize`](%60::borsh::BorshDeserialize%60) to
    [`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
    and
    [`IndexSet`](set/struct.IndexSet.html "struct indexmap::set::IndexSet").
    Due to a cyclic dependency that arose between
    [`borsh`](%60::borsh%60) and `indexmap`, `borsh v1.5.6` added an
    `indexmap` feature that should be used instead of enabling the
    feature here.

*Note: only the `std` feature is enabled by default.*

### <a href="#alternate-hashers" class="doc-anchor">§</a>Alternate Hashers

[`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
and
[`IndexSet`](set/struct.IndexSet.html "struct indexmap::set::IndexSet")
have a default hasher type
[`S = RandomState`](https://doc.rust-lang.org/1.93.1/std/hash/random/struct.RandomState.html "struct std::hash::random::RandomState"),
just like the standard `HashMap` and `HashSet`, which is resistant to
HashDoS attacks but not the most performant. Type aliases can make it
easier to use alternate hashers:

``` {.rust .rust-example-rendered}
use fnv::FnvBuildHasher;
use indexmap::{IndexMap, IndexSet};

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;
type FnvIndexSet<T> = IndexSet<T, FnvBuildHasher>;

let std: IndexSet<i32> = (0..100).collect();
let fnv: FnvIndexSet<i32> = (0..100).collect();
assert_eq!(std, fnv);
```

### <a href="#rust-version" class="doc-anchor">§</a>Rust Version

This version of indexmap requires Rust 1.82 or later.

The indexmap 2.x release series will use a carefully considered version
upgrade policy, where in a later 2.x version, we will raise the minimum
required Rust version.

## <a href="#no-standard-library-targets" class="doc-anchor">§</a>No Standard Library Targets

This crate supports being built without `std`, requiring `alloc`
instead. This is chosen by disabling the default “std” cargo feature, by
adding `default-features = false` to your dependency specification.

-   Creating maps and sets using
    [`new`](map/struct.IndexMap.html#method.new "associated function indexmap::map::IndexMap::new")
    and
    [`with_capacity`](map/struct.IndexMap.html#method.with_capacity "associated function indexmap::map::IndexMap::with_capacity")
    is unavailable without `std`. Use methods
    [`IndexMap::default`](map/struct.IndexMap.html#method.default "associated function indexmap::map::IndexMap::default"),
    [`with_hasher`](map/struct.IndexMap.html#method.with_hasher "associated function indexmap::map::IndexMap::with_hasher"),
    [`with_capacity_and_hasher`](map/struct.IndexMap.html#method.with_capacity_and_hasher "associated function indexmap::map::IndexMap::with_capacity_and_hasher")
    instead. A no-std compatible hasher will be needed as well, for
    example from the crate `twox-hash`.
-   Macros [`indexmap!`](macro.indexmap.html "macro indexmap::indexmap")
    and [`indexset!`](macro.indexset.html "macro indexmap::indexset")
    are unavailable without `std`. Use the macros
    [`indexmap_with_default!`](macro.indexmap_with_default.html "macro indexmap::indexmap_with_default")
    and
    [`indexset_with_default!`](macro.indexset_with_default.html "macro indexmap::indexset_with_default")
    instead.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::map::`<a href="map/struct.IndexMap.html" class="struct"
title="struct indexmap::map::IndexMap"><code>IndexMap</code></a>`;`

`pub use crate::set::`<a href="set/struct.IndexSet.html" class="struct"
title="struct indexmap::set::IndexSet"><code>IndexSet</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="map/index.html" class="mod" title="mod indexmap::map">map</a>  
[`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
is a hash table where the iteration order of the key-value pairs is
independent of the hash values of the keys.

<a href="set/index.html" class="mod" title="mod indexmap::set">set</a>  
A hash set implemented using
[`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.indexmap.html" class="macro"
title="macro indexmap::indexmap">indexmap</a>  
Create an
[`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
from a list of key-value pairs

<a href="macro.indexmap_with_default.html" class="macro"
title="macro indexmap::indexmap_with_default">indexmap_with_default</a>  
Create an
[`IndexMap`](map/struct.IndexMap.html "struct indexmap::map::IndexMap")
from a list of key-value pairs and a
[`BuildHasherDefault`](https://doc.rust-lang.org/1.93.1/core/hash/struct.BuildHasherDefault.html "struct core::hash::BuildHasherDefault")-wrapped
custom hasher.

<a href="macro.indexset.html" class="macro"
title="macro indexmap::indexset">indexset</a>  
Create an
[`IndexSet`](set/struct.IndexSet.html "struct indexmap::set::IndexSet")
from a list of values

<a href="macro.indexset_with_default.html" class="macro"
title="macro indexmap::indexset_with_default">indexset_with_default</a>  
Create an
[`IndexSet`](set/struct.IndexSet.html "struct indexmap::set::IndexSet")
from a list of values and a
[`BuildHasherDefault`](https://doc.rust-lang.org/1.93.1/core/hash/struct.BuildHasherDefault.html "struct core::hash::BuildHasherDefault")-wrapped
custom hasher.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.TryReserveError.html" class="struct"
title="struct indexmap::TryReserveError">TryReserveError</a>  
The error type for
[`try_reserve`](map/struct.IndexMap.html#method.try_reserve "method indexmap::map::IndexMap::try_reserve")
methods.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.GetDisjointMutError.html" class="enum"
title="enum indexmap::GetDisjointMutError">GetDisjointMutError</a>  
The error type returned by
[`get_disjoint_indices_mut`](map/struct.IndexMap.html#method.get_disjoint_indices_mut "method indexmap::map::IndexMap::get_disjoint_indices_mut").

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Equivalent.html" class="trait"
title="trait indexmap::Equivalent">Equivalent</a>  
Key equivalence trait.
