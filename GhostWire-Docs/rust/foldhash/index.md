Crate foldhash Copy item path

<span class="sub-heading"><a href="../src/foldhash/lib.rs.html#1-350" class="src">Source</a>
</span>

Expand description

This crate provides foldhash, a fast, non-cryptographic, minimally
DoS-resistant hashing algorithm designed for computational uses such as
hashmaps, bloom filters, count sketching, etc.

When should you **not** use foldhash:

-   You are afraid of people studying your long-running program’s
    behavior to reverse engineer its internal random state and using
    this knowledge to create many colliding inputs for computational
    complexity attacks.

-   You expect foldhash to have a consistent output across versions or
    platforms, such as for persistent file formats or communication
    protocols.

-   You are relying on foldhash’s properties for any kind of security.
    Foldhash is **not appropriate for any cryptographic purpose**.

Foldhash has two variants, one optimized for speed which is ideal for
data structures such as hash maps and bloom filters, and one optimized
for statistical quality which is ideal for algorithms such as
[HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog) and
[MinHash](https://en.wikipedia.org/wiki/MinHash).

Foldhash can be used in a `#![no_std]` environment by disabling its
default `"std"` feature.

# <a href="#usage" class="doc-anchor">§</a>Usage

The easiest way to use this crate with the standard library
\[`HashMap`\] or \[`HashSet`\] is to import them from `foldhash`
instead, along with the extension traits to make \[`HashMap::new`\] and
\[`HashMap::with_capacity`\] work out-of-the-box:

``` {.rust .rust-example-rendered}
use foldhash::{HashMap, HashMapExt};

let mut hm = HashMap::new();
hm.insert(42, "hello");
```

You can also avoid the convenience types and do it manually by
initializing a
[`RandomState`](fast/struct.RandomState.html "struct foldhash::fast::RandomState"),
for example if you are using a different hash map implementation like
[`hashbrown`](https://docs.rs/hashbrown/):

``` {.rust .rust-example-rendered}
use hashbrown::HashMap;
use foldhash::fast::RandomState;

let mut hm = HashMap::with_hasher(RandomState::default());
hm.insert("foo", "bar");
```

The above methods are the recommended way to use foldhash, which will
automatically generate a randomly generated hasher instance for you. If
you absolutely must have determinism you can use
[`FixedState`](fast/struct.FixedState.html "struct foldhash::fast::FixedState")
instead, but note that this makes you trivially vulnerable to HashDoS
attacks and might lead to quadratic runtime when moving data from one
hashmap/set into another:

``` {.rust .rust-example-rendered}
use std::collections::HashSet;
use foldhash::fast::FixedState;

let mut hm = HashSet::with_hasher(FixedState::with_seed(42));
hm.insert([1, 10, 100]);
```

If you rely on statistical properties of the hash for the correctness of
your algorithm, such as in
[HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog), it is
suggested to use the
[`RandomState`](quality/struct.RandomState.html "struct foldhash::quality::RandomState")
or
[`FixedState`](quality/struct.FixedState.html "struct foldhash::quality::FixedState")
from the [`quality`](quality/index.html "mod foldhash::quality") module
instead of the [`fast`](fast/index.html "mod foldhash::fast") module.
The latter is optimized purely for speed in hash tables and has known
statistical imperfections.

Finally, you can also directly use the
[`RandomState`](quality/struct.RandomState.html "struct foldhash::quality::RandomState")
or
[`FixedState`](quality/struct.FixedState.html "struct foldhash::quality::FixedState")
to manually hash items using the [`BuildHasher`](std::hash::BuildHasher)
trait:

``` {.rust .rust-example-rendered}
use std::hash::BuildHasher;
use foldhash::quality::RandomState;

let random_state = RandomState::default();
let hash = random_state.hash_one("hello world");
```

## <a href="#seeding" class="doc-anchor">§</a>Seeding

Foldhash relies on a single 8-byte per-hasher seed which should be
ideally be different from each instance to instance, and also a larger
[`SharedSeed`](struct.SharedSeed.html "struct foldhash::SharedSeed")
which may be shared by many different instances.

To reduce overhead, this
[`SharedSeed`](struct.SharedSeed.html "struct foldhash::SharedSeed") is
typically initialized once and stored. To prevent each hashmap
unnecessarily containing a reference to this value there are three kinds
of
[`BuildHasher`](https://doc.rust-lang.org/1.93.1/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher")s
foldhash provides (both for
[`fast`](fast/index.html "mod foldhash::fast") and
[`quality`](quality/index.html "mod foldhash::quality")):

1.  [`RandomState`](fast/struct.RandomState.html "struct foldhash::fast::RandomState"),
    which always generates a random per-hasher seed and implicitly
    stores a reference to
    [`SharedSeed::global_random`](struct.SharedSeed.html#method.global_random "associated function foldhash::SharedSeed::global_random").
2.  [`FixedState`](fast/struct.FixedState.html "struct foldhash::fast::FixedState"),
    which by default uses a fixed per-hasher seed and implicitly stores
    a reference to
    [`SharedSeed::global_fixed`](struct.SharedSeed.html#method.global_fixed "associated function foldhash::SharedSeed::global_fixed").
3.  [`SeedableRandomState`](fast/struct.SeedableRandomState.html "struct foldhash::fast::SeedableRandomState"),
    which works like
    [`RandomState`](fast/struct.RandomState.html "struct foldhash::fast::RandomState")
    by default but can be seeded in any manner. This state must include
    an explicit reference to a
    [`SharedSeed`](struct.SharedSeed.html "struct foldhash::SharedSeed"),
    and thus this struct is 16 bytes as opposed to just 8 bytes for the
    previous two.

## <a href="#features" class="doc-anchor">§</a>Features

This crate has the following features:

-   `nightly`, this feature improves string hashing performance slightly
    using the nightly-only Rust feature
    [`hasher_prefixfree_extras`](https://github.com/rust-lang/rust/issues/96762),
-   `std`, this enabled-by-default feature offers convenient aliases for
    `std` containers, but can be turned off for `#![no_std]` crates.

# Modules<a href="#modules" class="anchor">§</a>

<a href="fast/index.html" class="mod"
title="mod foldhash::fast">fast</a>  
The foldhash implementation optimized for speed.

<a href="quality/index.html" class="mod"
title="mod foldhash::quality">quality</a>  
The foldhash implementation optimized for quality.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.SharedSeed.html" class="struct"
title="struct foldhash::SharedSeed">SharedSeed</a>  
A random seed intended to be shared by many different foldhash
instances.
