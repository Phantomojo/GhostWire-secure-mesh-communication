Crate aead Copy item path

<span class="sub-heading"><a href="../src/aead/lib.rs.html#1-591" class="src">Source</a>
</span>

Expand description

[Authenticated Encryption with Associated
Data](https://en.wikipedia.org/wiki/Authenticated_encryption) (AEAD)
traits

This crate provides an abstract interface for AEAD ciphers, which
guarantee both confidentiality and integrity, even from a powerful
attacker who is able to execute [chosen-ciphertext
attacks](https://en.wikipedia.org/wiki/Chosen-ciphertext_attack). The
resulting security property, [ciphertext
indistinguishability](https://en.wikipedia.org/wiki/Ciphertext_indistinguishability),
is considered a basic requirement for modern cryptographic
implementations.

See [RustCrypto/AEADs](https://github.com/RustCrypto/AEADs) for cipher
implementations which use this trait.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../generic_array/index.html" class="mod"
title="mod generic_array"><code>generic_array</code></a>`;`

`pub use crypto_common::`<a href="../rand_core/index.html" class="mod"
title="mod rand_core"><code>rand_core</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="consts/index.html" class="mod"
title="mod aead::consts">consts</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct aead::Error">Error</a>  
Error type.

<a href="struct.OsRng.html" class="struct"
title="struct aead::OsRng">OsRng</a>  
A random number generator that retrieves randomness from the operating
system.

<a href="struct.Payload.html" class="struct"
title="struct aead::Payload">Payload</a>  
AEAD payloads (message + AAD).

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Aead.html" class="trait"
title="trait aead::Aead">Aead</a>  
Authenticated Encryption with Associated Data (AEAD) algorithm.

<a href="trait.AeadCore.html" class="trait"
title="trait aead::AeadCore">AeadCore</a>  
Authenticated Encryption with Associated Data (AEAD) algorithm core
trait.

<a href="trait.AeadInPlace.html" class="trait"
title="trait aead::AeadInPlace">AeadInPlace</a>  
In-place stateless AEAD trait.

<a href="trait.AeadMut.html" class="trait"
title="trait aead::AeadMut">AeadMut</a>  
Stateful Authenticated Encryption with Associated Data algorithm.

<a href="trait.AeadMutInPlace.html" class="trait"
title="trait aead::AeadMutInPlace">AeadMutInPlace</a>  
In-place stateful AEAD trait.

<a href="trait.Buffer.html" class="trait"
title="trait aead::Buffer">Buffer</a>  
In-place encryption/decryption byte buffers.

<a href="trait.KeyInit.html" class="trait"
title="trait aead::KeyInit">KeyInit</a>  
Types which can be initialized from key.

<a href="trait.KeySizeUser.html" class="trait"
title="trait aead::KeySizeUser">KeySizeUser</a>  
Types which use key for initialization.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Key.html" class="type" title="type aead::Key">Key</a>  
Key used by
[`KeySizeUser`](trait.KeySizeUser.html "trait aead::KeySizeUser")
implementors.

<a href="type.Nonce.html" class="type"
title="type aead::Nonce">Nonce</a>  
Nonce: single-use value for ensuring ciphertexts are unique

<a href="type.Result.html" class="type"
title="type aead::Result">Result</a>  
Result type alias with
[`Error`](struct.Error.html "struct aead::Error").

<a href="type.Tag.html" class="type" title="type aead::Tag">Tag</a>  
Tag: authentication code which ensures ciphertexts are authentic
