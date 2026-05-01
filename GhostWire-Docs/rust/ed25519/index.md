Crate ed25519 Copy item path

<span class="sub-heading"><a href="../src/ed25519/lib.rs.html#1-419" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-ed25519" class="doc-anchor">§</a>[RustCrypto](https://github.com/RustCrypto): Ed25519

[![crate](https://buildstats.info/crate/ed25519)](https://crates.io/crates/ed25519)
[![Docs](https://docs.rs/ed25519/badge.svg)](https://docs.rs/ed25519/)
[![Build
Status](https://github.com/RustCrypto/signatures/actions/workflows/ed25519.yml/badge.svg)](https://github.com/RustCrypto/signatures/actions/workflows/ed25519.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260048-signatures)

[Edwards Digital Signature Algorithm
(EdDSA)](https://en.wikipedia.org/wiki/EdDSA#Ed25519) over Curve25519 as
specified in [RFC 8032](https://tools.ietf.org/html/rfc8032).

[Documentation](https://docs.rs/ed25519/)

## <a href="#about" class="doc-anchor">§</a>About

This crate doesn’t contain an implementation of Ed25519, but instead
contains an
[`ed25519::Signature`](https://docs.rs/ed25519/latest/ed25519/struct.Signature.html)
type which other crates can use in conjunction with the
[`signature::Signer`](https://docs.rs/signature/latest/signature/trait.Signer.html)
and
[`signature::Verifier`](https://docs.rs/signature/latest/signature/trait.Verifier.html)
traits.

These traits allow crates which produce and consume Ed25519 signatures
to be written abstractly in such a way that different signer/verifier
providers can be plugged in, enabling support for using different
Ed25519 implementations, including HSMs or Cloud KMS services.

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

This crate requires **Rust 1.60** at a minimum.

Our policy is to allow MSRV to be raised in future released without that
qualifing as a SemVer-breaking change, but it will be accompanied by a
minor version bump, ensuring if you lock to a minor version MSRV will be
preserved for the default feature set.

## <a href="#semver-policy" class="doc-anchor">§</a>SemVer Policy

-   All on-by-default features of this library are covered by SemVer
-   MSRV is considered exempt from SemVer as noted above
-   The `pkcs8` module is exempted as it uses a pre-1.0 dependency,
    however, breaking changes to this module will be accompanied by a
    minor version bump.

## <a href="#license" class="doc-anchor">§</a>License

All crates licensed under either of

-   [Apache License, Version
    2.0](http://www.apache.org/licenses/LICENSE-2.0)
-   [MIT license](http://opensource.org/licenses/MIT)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

# <a
href="#using-ed25519-generically-over-algorithm-implementationsproviders"
class="doc-anchor">§</a>Using Ed25519 generically over algorithm implementations/providers

By using the `ed25519` crate, you can write code which signs and
verifies messages using the Ed25519 signature algorithm generically over
any supported Ed25519 implementation (see the next section for available
providers).

This allows consumers of your code to plug in whatever implementation
they want to use without having to add all potential Ed25519 libraries
you’d like to support as optional dependencies.

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use ed25519::signature::{Signer, Verifier};

pub struct HelloSigner<S>
where
    S: Signer<ed25519::Signature>
{
    pub signing_key: S
}

impl<S> HelloSigner<S>
where
    S: Signer<ed25519::Signature>
{
    pub fn sign(&self, person: &str) -> ed25519::Signature {
        // NOTE: use `try_sign` if you'd like to be able to handle
        // errors from external signing services/devices (e.g. HSM/KMS)
        // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
        self.signing_key.sign(format_message(person).as_bytes())
    }
}

pub struct HelloVerifier<V> {
    pub verifying_key: V
}

impl<V> HelloVerifier<V>
where
    V: Verifier<ed25519::Signature>
{
    pub fn verify(
        &self,
        person: &str,
        signature: &ed25519::Signature
    ) -> Result<(), ed25519::Error> {
        self.verifying_key.verify(format_message(person).as_bytes(), signature)
    }
}

fn format_message(person: &str) -> String {
    format!("Hello, {}!", person)
}
```

## <a href="#using-above-example-with-ed25519-dalek"
class="doc-anchor">§</a>Using above example with `ed25519-dalek`

The [`ed25519-dalek`](https://docs.rs/ed25519-dalek) crate natively
supports the
[`ed25519::Signature`](struct.Signature.html "struct ed25519::Signature")
type defined in this crate along with the
[`signature::Signer`](../signature/signer/trait.Signer.html "trait signature::signer::Signer")
and
[`signature::Verifier`](../signature/verifier/trait.Verifier.html "trait signature::verifier::Verifier")
traits.

Below is an example of how a hypothetical consumer of the code above can
instantiate and use the previously defined `HelloSigner` and
`HelloVerifier` types with
[`ed25519-dalek`](https://docs.rs/ed25519-dalek) as the
signing/verification provider:

*NOTE: requires [`ed25519-dalek`](https://docs.rs/ed25519-dalek) v2 or
newer for compatibility with `ed25519` v2.2+*.

``` {.rust .rust-example-rendered}
use ed25519_dalek::{Signer, Verifier, Signature};
use rand_core::OsRng; // Requires the `std` feature of `rand_core`

/// `HelloSigner` defined above instantiated with `ed25519-dalek` as
/// the signing provider.
pub type DalekHelloSigner = HelloSigner<ed25519_dalek::SigningKey>;

let signing_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
let signer = DalekHelloSigner { signing_key };
let person = "Joe"; // Message to sign
let signature = signer.sign(person);

/// `HelloVerifier` defined above instantiated with `ed25519-dalek`
/// as the signature verification provider.
pub type DalekHelloVerifier = HelloVerifier<ed25519_dalek::VerifyingKey>;

let verifying_key: ed25519_dalek::VerifyingKey = signer.signing_key.verifying_key();
let verifier = DalekHelloVerifier { verifying_key };
assert!(verifier.verify(person, &signature).is_ok());
```

## <a href="#using-above-example-with-ring-compat" class="doc-anchor">§</a>Using above example with `ring-compat`

The [`ring-compat`](https://docs.rs/ring-compat) crate provides wrappers
for [*ring*](https://github.com/briansmith/ring) which implement the
[`signature::Signer`](../signature/signer/trait.Signer.html "trait signature::signer::Signer")
and
[`signature::Verifier`](../signature/verifier/trait.Verifier.html "trait signature::verifier::Verifier")
traits for
[`ed25519::Signature`](struct.Signature.html "struct ed25519::Signature").

Below is an example of how a hypothetical consumer of the code above can
instantiate and use the previously defined `HelloSigner` and
`HelloVerifier` types with [`ring-compat`](https://docs.rs/ring-compat)
as the signing/verification provider:

``` {.rust .rust-example-rendered}
use ring_compat::signature::{
    ed25519::{Signature, SigningKey, VerifyingKey},
    Signer, Verifier
};
use rand_core::{OsRng, RngCore}; // Requires the `std` feature of `rand_core`

/// `HelloSigner` defined above instantiated with *ring* as
/// the signing provider.
pub type RingHelloSigner = HelloSigner<SigningKey>;

let mut ed25519_seed = [0u8; 32];
OsRng.fill_bytes(&mut ed25519_seed);

let signing_key = SigningKey::from_bytes(&ed25519_seed);
let verifying_key = signing_key.verifying_key();

let signer = RingHelloSigner { signing_key };
let person = "Joe"; // Message to sign
let signature = signer.sign(person);

/// `HelloVerifier` defined above instantiated with *ring*
/// as the signature verification provider.
pub type RingHelloVerifier = HelloVerifier<VerifyingKey>;

let verifier = RingHelloVerifier { verifying_key };
assert!(verifier.verify(person, &signature).is_ok());
```

# <a href="#available-ed25519-providers" class="doc-anchor">§</a>Available Ed25519 providers

The following libraries support the types/traits from the `ed25519`
crate:

-   [`ed25519-dalek`](https://docs.rs/ed25519-dalek) - mature pure Rust
    implementation of Ed25519
-   [`ring-compat`](https://docs.rs/ring-compat) - compatibility wrapper
    for [*ring*](https://github.com/briansmith/ring)
-   [`yubihsm`](https://github.com/iqlusioninc/yubihsm.rs/blob/develop/README.md) -
    host-side client library for YubiHSM2 devices from Yubico

# <a href="#features" class="doc-anchor">§</a>Features

The following features are presently supported:

-   `pkcs8`: support for decoding/encoding PKCS#8-formatted private keys
    using the \[`KeypairBytes`\] type.

-   `std` *(default)*: Enable `std` support in
    [`signature`](../signature/index.html "mod signature"), which
    currently only affects whether
    [`signature::Error`](struct.Error.html "struct ed25519::Error")
    implements `std::error::Error`.

-   `serde`: Implement `serde::Deserialize` and `serde::Serialize` for
    [`Signature`](struct.Signature.html "struct ed25519::Signature").
    Signatures are serialized as their bytes.

-   `serde_bytes`: Implement `serde_bytes::Deserialize` and
    `serde_bytes::Serialize` for
    [`Signature`](struct.Signature.html "struct ed25519::Signature").
    This enables more compact representations for formats with an
    efficient byte array representation. As per the `serde_bytes`
    documentation, this can most easily be realised using the
    `#[serde(with = "serde_bytes")]` annotation, e.g.:

    <a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
    ``` {.rust .rust-example-rendered}
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    struct SignatureAsBytes(#[serde(with = "serde_bytes")] Signature);
    ```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../signature/index.html" class="mod"
title="mod signature"><code>signature</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct ed25519::Error">Error</a>  
Signature errors.

<a href="struct.Signature.html" class="struct"
title="struct ed25519::Signature">Signature</a>  
Ed25519 signature.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.SignatureEncoding.html" class="trait"
title="trait ed25519::SignatureEncoding">SignatureEncoding</a>  
Support for decoding/encoding signatures as bytes.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ComponentBytes.html" class="type"
title="type ed25519::ComponentBytes">ComponentBytes</a>  
Size of an `R` or `s` component of an Ed25519 signature when serialized
as bytes.

<a href="type.SignatureBytes.html" class="type"
title="type ed25519::SignatureBytes">SignatureBytes</a>  
Ed25519 signature serialized as a byte array.
