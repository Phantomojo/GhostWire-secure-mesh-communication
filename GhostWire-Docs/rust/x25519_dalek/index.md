Crate x25519\_dalek Copy item path

<span class="sub-heading"><a href="../src/x25519_dalek/lib.rs.html#17-33" class="src">Source</a>
</span>

Expand description

# <a href="#x25519-dalek----ci" class="doc-anchor">§</a>x25519-dalek [![](https://img.shields.io/crates/v/x25519-dalek.svg)](https://crates.io/crates/x25519-dalek) [![](https://docs.rs/x25519-dalek/badge.svg)](https://docs.rs/x25519-dalek) [![CI](https://github.com/dalek-cryptography/curve25519-dalek/actions/workflows/x25519-dalek.yml/badge.svg?branch=main)](https://github.com/dalek-cryptography/curve25519-dalek/actions/workflows/x25519-dalek.yml)

A pure-Rust implementation of x25519 elliptic curve Diffie-Hellman key
exchange, with curve operations provided by
[curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek).

This crate provides two levels of API: a bare byte-oriented `x25519`
function which matches the function specified in
[RFC7748](https://tools.ietf.org/html/rfc7748), as well as a
higher-level Rust API for static and ephemeral Diffie-Hellman.

## <a href="#examples" class="doc-anchor">§</a>Examples

[<img
src="https://raw.githubusercontent.com/dalek-cryptography/x25519-dalek/master/res/bubblesort-zines-secret-messages-cover.jpeg"
style="float: right; width: auto; height: 300px;" />](https://shop.bubblesort.io)

Alice and Bob are two adorable kittens who have lost their mittens, and
they wish to be able to send secret messages to each other to coordinate
finding them, otherwise—if their caretaker cat finds out—they will
surely be called naughty kittens and be given no pie!

But the two kittens are quite clever. Even though their paws are still
too big and the rest of them is 90% fuzziness, these clever kittens have
been studying up on modern public key cryptography and have learned a
nifty trick called *elliptic curve Diffie-Hellman key exchange*. With
the right incantations, the kittens will be able to secretly organise to
find their mittens, and then spend the rest of the afternoon nomming
some yummy pie!

First, Alice uses `EphemeralSecret::random()` and then
`PublicKey::from()` to produce her secret and public keys:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use x25519_dalek::{EphemeralSecret, PublicKey};

let alice_secret = EphemeralSecret::random();
let alice_public = PublicKey::from(&alice_secret);
```

Bob does the same:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
let bob_secret = EphemeralSecret::random();
let bob_public = PublicKey::from(&bob_secret);
```

Alice meows across the room, telling `alice_public` to Bob, and Bob
loudly meows `bob_public` back to Alice. Alice now computes her shared
secret with Bob by doing:

``` {.rust .rust-example-rendered}
let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
```

Similarly, Bob computes a shared secret by doing:

``` {.rust .rust-example-rendered}
let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);
```

These secrets are the same:

``` {.rust .rust-example-rendered}
assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());
```

Voilà! Alice and Bob can now use their shared secret to encrypt their
meows, for example, by using it to generate a key and nonce for an
authenticated-encryption cipher.

This example used the ephemeral DH API, which ensures that secret keys
cannot be reused; Alice and Bob could instead use the static DH API and
load a long-term secret key.

# <a href="#installation" class="doc-anchor">§</a>Installation

To install, add the following to your project’s `Cargo.toml`:

``` language-toml
[dependencies]
x25519-dalek = "2"
```

# <a href="#msrv" class="doc-anchor">§</a>MSRV

Current MSRV is 1.60.

# <a href="#documentation" class="doc-anchor">§</a>Documentation

Documentation is available [here](https://docs.rs/x25519-dalek).

# <a href="#performance-and-backend-selection" class="doc-anchor">§</a>Performance and backend selection

Performance is a secondary goal behind correctness, safety, and clarity,
but we aim to be competitive with other implementations. To this end, we
allow users to choose their *backend*, i.e., the underlying
implementation of elliptic curve and scalar arithmetic. Different
backends have different use cases. For example, if you demand formally
verified code, you want to use the `fiat` backend (as it was generated
from [Fiat Crypto](https://github.com/mit-plv/fiat-crypto)).

Further instructions and details regarding backends can be found in the
[curve25519-dalek
docs](https://github.com/dalek-cryptography/curve25519-dalek#backends).

# <a href="#note" class="doc-anchor">§</a>Note

This code matches the [RFC7748](https://tools.ietf.org/html/rfc7748)
test vectors. The elliptic curve operations are provided by
`curve25519-dalek`, which makes a best-effort attempt to prevent
software side-channels.

“Secret Messages” cover image and
[zine](https://shop.bubblesort.io/products/secret-messages-zine)
copyright © Amy Wibowo ([@sailorhg](https://twitter.com/sailorhg))

# <a href="#see-also" class="doc-anchor">§</a>See also

-   [crypto\_box](https://github.com/RustCrypto/nacl-compat/tree/master/crypto_box):
    pure Rust public-key authenticated encryption compatible with the
    NaCl family of encryption libraries (libsodium, TweetNaCl) which
    uses `x25519-dalek` for key agreement

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.EphemeralSecret.html" class="struct"
title="struct x25519_dalek::EphemeralSecret">EphemeralSecret</a>  
A short-lived Diffie-Hellman secret key that can only be used to compute
a single
[`SharedSecret`](struct.SharedSecret.html "struct x25519_dalek::SharedSecret").

<a href="struct.PublicKey.html" class="struct"
title="struct x25519_dalek::PublicKey">PublicKey</a>  
A Diffie-Hellman public key

<a href="struct.ReusableSecret.html" class="struct"
title="struct x25519_dalek::ReusableSecret">ReusableSecret</a>  
A Diffie-Hellman secret key which may be used more than once, but is
purposefully not serialiseable in order to discourage key-reuse. This is
implemented to facilitate protocols such as Noise (e.g. Noise IK key
usage, etc.) and X3DH which require an “ephemeral” key to conduct the
Diffie-Hellman operation multiple times throughout the protocol, while
the protocol run at a higher level is only conducted once per key.

<a href="struct.SharedSecret.html" class="struct"
title="struct x25519_dalek::SharedSecret">SharedSecret</a>  
The result of a Diffie-Hellman key exchange.

<a href="struct.StaticSecret.html" class="struct"
title="struct x25519_dalek::StaticSecret">StaticSecret</a>  
A Diffie-Hellman secret key that can be used to compute multiple
[`SharedSecret`](struct.SharedSecret.html "struct x25519_dalek::SharedSecret")s.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.X25519_BASEPOINT_BYTES.html" class="constant"
title="constant x25519_dalek::X25519_BASEPOINT_BYTES">X25519_BASEPOINT_BYTES</a>  
The X25519 basepoint, for use with the bare, byte-oriented x25519
function. This is provided for people who cannot use the typed DH API
for some reason.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.x25519.html" class="fn"
title="fn x25519_dalek::x25519">x25519</a>  
The bare, byte-oriented x25519 function, exactly as specified in
RFC7748.
