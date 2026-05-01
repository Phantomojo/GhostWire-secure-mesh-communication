Crate fxhash Copy item path

<span class="sub-heading"><a href="../src/fxhash/lib.rs.html#11-324" class="src">Source</a>
</span>

Expand description

# <a href="#fx-hash" class="doc-anchor">§</a>Fx Hash

This hashing algorithm was extracted from the Rustc compiler. This is
the same hashing algoirthm used for some internal operations in FireFox.
The strength of this algorithm is in hashing 8 bytes at a time on 64-bit
platforms, where the FNV algorithm works on one byte at a time.

## <a href="#disclaimer" class="doc-anchor">§</a>Disclaimer

It is **not a cryptographically secure** hash, so it is strongly
recommended that you do not use this hash for cryptographic purproses.
Furthermore, this hashing algorithm was not designed to prevent any
attacks for determining collisions which could be used to potentially
cause quadratic behavior in `HashMap`s. So it is not recommended to
expose this hash in places where collissions or DDOS attacks may be a
concern.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.FxHasher.html" class="struct"
title="struct fxhash::FxHasher">FxHasher</a>  
This hashing algorithm was extracted from the Rustc compiler. This is
the same hashing algoirthm used for some internal operations in FireFox.
The strength of this algorithm is in hashing 8 bytes at a time on 64-bit
platforms, where the FNV algorithm works on one byte at a time.

<a href="struct.FxHasher32.html" class="struct"
title="struct fxhash::FxHasher32">FxHasher32</a>  
This hashing algorithm was extracted from the Rustc compiler. This is
the same hashing algoirthm used for some internal operations in FireFox.
The strength of this algorithm is in hashing 4 bytes at a time on any
platform, where the FNV algorithm works on one byte at a time.

<a href="struct.FxHasher64.html" class="struct"
title="struct fxhash::FxHasher64">FxHasher64</a>  
This hashing algorithm was extracted from the Rustc compiler. This is
the same hashing algoirthm used for some internal operations in FireFox.
The strength of this algorithm is in hashing 8 bytes at a time on any
platform, where the FNV algorithm works on one byte at a time.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.hash.html" class="fn" title="fn fxhash::hash">hash</a>  
A convenience function for when you need a quick usize hash.

<a href="fn.hash32.html" class="fn" title="fn fxhash::hash32">hash32</a>  
A convenience function for when you need a quick 32-bit hash.

<a href="fn.hash64.html" class="fn" title="fn fxhash::hash64">hash64</a>  
A convenience function for when you need a quick 64-bit hash.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.FxBuildHasher.html" class="type"
title="type fxhash::FxBuildHasher">FxBuildHasher</a>  
A builder for default Fx hashers.

<a href="type.FxHashMap.html" class="type"
title="type fxhash::FxHashMap">FxHashMap</a>  
A `HashMap` using a default Fx hasher.

<a href="type.FxHashSet.html" class="type"
title="type fxhash::FxHashSet">FxHashSet</a>  
A `HashSet` using a default Fx hasher.
