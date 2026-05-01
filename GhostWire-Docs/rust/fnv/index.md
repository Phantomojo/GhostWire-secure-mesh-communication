Crate fnv Copy item path

<span class="sub-heading"><a href="../src/fnv/lib.rs.html#1-367" class="src">Source</a>
</span>

Expand description

An implementation of the [Fowler–Noll–Vo hash
function](http://www.isthe.com/chongo/tech/comp/fnv/index.html).

## <a href="#about" class="doc-anchor">§</a>About

The FNV hash function is a custom `Hasher` implementation that is more
efficient for smaller hash keys.

[The Rust FAQ states
that](https://www.rust-lang.org/en-US/faq.html#why-are-rusts-hashmaps-slow)
while the default `Hasher` implementation, SipHash, is good in many
cases, it is notably slower than other algorithms with short keys, such
as when you have a map of integers to other values. In cases like these,
[FNV is demonstrably faster](https://cglab.ca/~abeinges/blah/hash-rs/).

Its disadvantages are that it performs badly on larger inputs, and
provides no protection against collision attacks, where a malicious user
can craft specific keys designed to slow a hasher down. Thus, it is
important to profile your program to ensure that you are using small
hash keys, and be certain that your program could not be exposed to
malicious inputs (including being a networked server).

The Rust compiler itself uses FNV, as it is not worried about
denial-of-service attacks, and can assume that its inputs are going to
be small—a perfect use case for FNV.

## <a href="#using-fnv-in-a-hashmap" class="doc-anchor">§</a>Using FNV in a `HashMap`

The `FnvHashMap` type alias is the easiest way to use the standard
library’s `HashMap` with FNV.

``` {.rust .rust-example-rendered}
use fnv::FnvHashMap;

let mut map = FnvHashMap::default();
map.insert(1, "one");
map.insert(2, "two");

map = FnvHashMap::with_capacity_and_hasher(10, Default::default());
map.insert(1, "one");
map.insert(2, "two");
```

Note, the standard library’s `HashMap::new` and `HashMap::with_capacity`
are only implemented for the `RandomState` hasher, so using `Default` to
get the hasher is the next best option.

## <a href="#using-fnv-in-a-hashset" class="doc-anchor">§</a>Using FNV in a `HashSet`

Similarly, `FnvHashSet` is a type alias for the standard library’s
`HashSet` with FNV.

``` {.rust .rust-example-rendered}
use fnv::FnvHashSet;

let mut set = FnvHashSet::default();
set.insert(1);
set.insert(2);

set = FnvHashSet::with_capacity_and_hasher(10, Default::default());
set.insert(1);
set.insert(2);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.FnvHasher.html" class="struct"
title="struct fnv::FnvHasher">FnvHasher</a>  
An implementation of the Fowler–Noll–Vo hash function.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.FnvBuildHasher.html" class="type"
title="type fnv::FnvBuildHasher">FnvBuildHasher</a>  
A builder for default FNV hashers.

<a href="type.FnvHashMap.html" class="type"
title="type fnv::FnvHashMap">FnvHashMap</a>  
A `HashMap` using a default FNV hasher.

<a href="type.FnvHashSet.html" class="type"
title="type fnv::FnvHashSet">FnvHashSet</a>  
A `HashSet` using a default FNV hasher.
