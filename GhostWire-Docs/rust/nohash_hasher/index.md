Crate nohash\_hasher Copy item path

<span class="sub-heading"><a href="../src/nohash_hasher/lib.rs.html#11-426" class="src">Source</a>
</span>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.NoHashHasher.html" class="struct"
title="struct nohash_hasher::NoHashHasher">NoHashHasher</a>  
For an enabled type `T`, a `NoHashHasher<T>` implements
`std::hash::Hasher` and uses the value set by one of the
`write_{u8, u16, u32, u64, usize, i8, i16, i32, i64, isize}` methods as
its hash output.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.IsEnabled.html" class="trait"
title="trait nohash_hasher::IsEnabled">IsEnabled</a>  
Types which are safe to use with `NoHashHasher`.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BuildNoHashHasher.html" class="type"
title="type nohash_hasher::BuildNoHashHasher">BuildNoHashHasher</a>  
An alias for `BuildHasherDefault` for use with `NoHashHasher`.

<a href="type.IntMap.html" class="type"
title="type nohash_hasher::IntMap">IntMap</a>  
A `HashMap` with an integer domain, using `NoHashHasher` to perform no
hashing at all.

<a href="type.IntSet.html" class="type"
title="type nohash_hasher::IntSet">IntSet</a>  
A `HashSet` of integers, using `NoHashHasher` to perform no hashing at
all.
