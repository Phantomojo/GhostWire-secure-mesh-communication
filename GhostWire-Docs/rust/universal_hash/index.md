Crate universal\_hash Copy item path

<span class="sub-heading"><a href="../src/universal_hash/lib.rs.html#1-190" class="src">Source</a>
</span>

Expand description

Traits for [Universal Hash
Functions](https://en.wikipedia.org/wiki/Universal_hashing).

# <a href="#about-universal-hashes" class="doc-anchor">§</a>About universal hashes

Universal hash functions provide a “universal family” of possible hash
functions where a given member of a family is selected by a key.

They are well suited to the purpose of “one time authenticators” for a
sequence of bytestring inputs, as their construction has a number of
desirable properties such as pairwise independence as well as
amenability to efficient implementations, particularly when implemented
using SIMD instructions.

When combined with a cipher, such as in Galois/Counter Mode (GCM) or the
Salsa20 family AEAD constructions, they can provide the core
functionality for a Message Authentication Code (MAC).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crypto_common;`

`pub use crypto_common::`<a href="../generic_array/index.html" class="mod"
title="mod generic_array"><code>generic_array</code></a>`;`

`pub use crypto_common::`<a href="../typenum/index.html" class="mod"
title="mod typenum"><code>typenum</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="consts/index.html" class="mod"
title="mod universal_hash::consts">consts</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct universal_hash::Error">Error</a>  
Error type used by the
[`UniversalHash::verify`](trait.UniversalHash.html#method.verify "method universal_hash::UniversalHash::verify")
method to indicate that UHF output is not equal the expected value.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.KeyInit.html" class="trait"
title="trait universal_hash::KeyInit">KeyInit</a>  
Types which can be initialized from key.

<a href="trait.Reset.html" class="trait"
title="trait universal_hash::Reset">Reset</a>  
Resettable types.

<a href="trait.UhfBackend.html" class="trait"
title="trait universal_hash::UhfBackend">UhfBackend</a>  
Trait implemented by UHF backends.

<a href="trait.UhfClosure.html" class="trait"
title="trait universal_hash::UhfClosure">UhfClosure</a>  
Trait for
[`UhfBackend`](trait.UhfBackend.html "trait universal_hash::UhfBackend")
users.

<a href="trait.UniversalHash.html" class="trait"
title="trait universal_hash::UniversalHash">UniversalHash</a>  
The
[`UniversalHash`](trait.UniversalHash.html "trait universal_hash::UniversalHash")
trait defines a generic interface for universal hash functions.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Block.html" class="type"
title="type universal_hash::Block">Block</a>  
Block on which \[`BlockSizeUser`\] implementors operate.

<a href="type.Key.html" class="type"
title="type universal_hash::Key">Key</a>  
Key used by \[`KeySizeUser`\] implementors.

<a href="type.ParBlocks.html" class="type"
title="type universal_hash::ParBlocks">ParBlocks</a>  
Parallel blocks on which \[`ParBlocksSizeUser`\] implementors operate.
