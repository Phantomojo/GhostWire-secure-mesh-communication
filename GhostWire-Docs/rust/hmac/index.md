Crate hmac Copy item path

<span class="sub-heading"><a href="../src/hmac/lib.rs.html#1-131" class="src">Source</a>
</span>

Expand description

Generic implementation of Hash-based Message Authentication Code (HMAC).

To use it you will need a cryptographic hash function implementation
which implements the [`digest`](https://docs.rs/digest) crate traits.
You can find compatible crates (e.g. [`sha2`](https://docs.rs/sha2)) in
the [`RustCrypto/hashes`](https://github.com/RustCrypto/hashes)
repository.

This crate provides two HMAC implementation
[`Hmac`](type.Hmac.html "type hmac::Hmac") and
[`SimpleHmac`](struct.SimpleHmac.html "struct hmac::SimpleHmac"). The
first one is a buffered wrapper around block-level
[`HmacCore`](struct.HmacCore.html "struct hmac::HmacCore"). Internally
it uses efficient state representation, but works only with hash
functions which expose block-level API and consume blocks eagerly (e.g.
it will not work with the BLAKE2 family of hash functions). On the other
hand, [`SimpleHmac`](struct.SimpleHmac.html "struct hmac::SimpleHmac")
is a bit less efficient memory-wise, but works with all hash functions
which implement the [`Digest`](https://docs.rs/digest) trait.

# <a href="#examples" class="doc-anchor">§</a>Examples

Let us demonstrate how to use HMAC using the SHA-256 hash function.

In the following examples [`Hmac`](type.Hmac.html "type hmac::Hmac") is
interchangeable with
[`SimpleHmac`](struct.SimpleHmac.html "struct hmac::SimpleHmac").

To get authentication code:

``` {.rust .rust-example-rendered}
use sha2::Sha256;
use hmac::{Hmac, Mac};
use hex_literal::hex;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
    .expect("HMAC can take key of any size");
mac.update(b"input message");

// `result` has type `CtOutput` which is a thin wrapper around array of
// bytes for providing constant time equality check
let result = mac.finalize();
// To get underlying array use `into_bytes`, but be careful, since
// incorrect use of the code value may permit timing attacks which defeats
// the security provided by the `CtOutput`
let code_bytes = result.into_bytes();
let expected = hex!("
    97d2a569059bbcd8ead4444ff99071f4
    c01d005bcefe0d3567e1be628e5fdcd9
");
assert_eq!(code_bytes[..], expected[..]);
```

To verify the message:

``` {.rust .rust-example-rendered}
let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
    .expect("HMAC can take key of any size");

mac.update(b"input message");

let code_bytes = hex!("
    97d2a569059bbcd8ead4444ff99071f4
    c01d005bcefe0d3567e1be628e5fdcd9
");
// `verify_slice` will return `Ok(())` if code is correct, `Err(MacError)` otherwise
mac.verify_slice(&code_bytes[..]).unwrap();
```

# <a href="#block-and-input-sizes" class="doc-anchor">§</a>Block and input sizes

Usually it is assumed that block size is larger than output size. Due to
the generic nature of the implementation, this edge case must be handled
as well to remove potential panic. This is done by truncating hash
output to the hash block size if needed.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../digest/index.html" class="mod"
title="mod digest"><code>digest</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.HmacCore.html" class="struct"
title="struct hmac::HmacCore">HmacCore</a>  
Generic core HMAC instance, which operates over blocks.

<a href="struct.SimpleHmac.html" class="struct"
title="struct hmac::SimpleHmac">SimpleHmac</a>  
Simplified HMAC instance able to operate over hash functions which do
not expose block-level API and hash functions which process blocks
lazily (e.g. BLAKE2).

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Mac.html" class="trait" title="trait hmac::Mac">Mac</a>  
Convenience wrapper trait covering functionality of Message
Authentication algorithms.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Hmac.html" class="type" title="type hmac::Hmac">Hmac</a>  
Generic HMAC instance.
