Crate signature Copy item path

<span class="sub-heading"><a href="../src/signature/lib.rs.html#1-158" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-digital-signature-algorithms"
class="doc-anchor">§</a>[RustCrypto](https://github.com/RustCrypto/): Digital Signature Algorithms

[![crate](https://buildstats.info/crate/signature)](https://crates.io/crates/signature)
[![Docs](https://docs.rs/signature/badge.svg)](https://docs.rs/signature/)
[![Build
Status](https://github.com/RustCrypto/traits/actions/workflows/signature.yml/badge.svg)](https://github.com/RustCrypto/traits/actions/workflows/signature.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260048-signatures)

This crate contains traits which provide generic, object-safe APIs for
generating and verifying [digital
signatures](https://en.wikipedia.org/wiki/Digital_signature).

Used by the
[`dsa`](https://github.com/RustCrypto/signatures/tree/master/dsa),
[`ecdsa`](https://github.com/RustCrypto/signatures/tree/master/ecdsa),
[`ed25519`](https://github.com/RustCrypto/signatures/tree/master/ed25519),
and [`rsa`](https://github.com/RustCrypto/RSA) crates maintained by the
[RustCrypto](https://github.com/RustCrypto/) organization, as well as
[`ed25519-dalek`](https://github.com/dalek-cryptography/ed25519-dalek).

[Documentation](https://docs.rs/signature/)

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

Rust **1.60** or higher.

Minimum supported Rust version can be changed in the future, but it will
be done with a minor version bump.

## <a href="#semver-policy" class="doc-anchor">§</a>SemVer Policy

-   All on-by-default features of this library are covered by SemVer
-   MSRV is considered exempt from SemVer as noted above
-   The `derive` feature is stable and covered by SemVer
-   The off-by-default features `digest` and `rand_core` are unstable
    features which are also considered exempt from SemVer as they
    correspond to pre-1.0 crates which are still subject to changes.
    Breaking changes to these features will, like MSRV, be done with a
    minor version bump.

## <a href="#license" class="doc-anchor">§</a>License

Licensed under either of

-   [Apache License, Version
    2.0](http://www.apache.org/licenses/LICENSE-2.0)
-   [MIT license](http://opensource.org/licenses/MIT)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

# <a href="#design" class="doc-anchor">§</a>Design

This crate provides a common set of traits for signing and verifying
digital signatures intended to be implemented by libraries which produce
or contain implementations of digital signature algorithms, and used by
libraries which want to produce or verify digital signatures while
generically supporting any compatible backend.

## <a href="#goals" class="doc-anchor">§</a>Goals

The traits provided by this crate were designed with the following goals
in mind:

-   Provide an easy-to-use, misuse resistant API optimized for consumers
    (as opposed to implementers) of its traits.
-   Support common type-safe wrappers around “bag-of-bytes”
    representations which can be directly parsed from or written to the
    “wire”.
-   Expose a trait/object-safe API where signers/verifiers spanning
    multiple homogeneous provider implementations can be seamlessly
    leveraged together in the same logical “keyring” so long as they
    operate on the same underlying signature type.
-   Allow one provider type to potentially implement support (including
    being generic over) several signature types.
-   Keep signature algorithm customizations / “knobs” out-of-band from
    the signing/verification APIs, ideally pushing such concerns into
    the type system so that algorithm mismatches are caught as type
    errors.
-   Opaque error type which minimizes information leaked from
    cryptographic failures, as “rich” error types in these scenarios are
    often a source of sidechannel information for attackers (e.g.
    [BB’06](https://en.wikipedia.org/wiki/Daniel_Bleichenbacher))

## <a href="#implementation" class="doc-anchor">§</a>Implementation

To accomplish the above goals, the
[`Signer`](trait.Signer.html "trait signature::Signer") and
[`Verifier`](trait.Verifier.html "trait signature::Verifier") traits
provided by this are generic over a signature value, and use generic
parameters rather than associated types. Notably, they use such a
parameter for the return value, allowing it to be inferred by the type
checker based on the desired signature type.

## <a href="#alternatives-considered" class="doc-anchor">§</a>Alternatives considered

This crate is based on many years of exploration of how to encapsulate
digital signature systems in the most flexible, developer-friendly way.
During that time many design alternatives were explored, tradeoffs
compared, and ultimately the provided API was selected.

The tradeoffs made in this API have all been to improve simplicity,
ergonomics, type safety, and flexibility for *consumers* of the traits.
At times, this has come at a cost to implementers. Below are some
concerns we are cognizant of which were considered in the design of the
API:

-   “Bag-of-bytes” serialization precludes signature providers from
    using their own internal representation of a signature, which can be
    helpful for many reasons (e.g. advanced signature system features
    like batch verification).
-   Associated types, rather than generic parameters of traits, could
    allow more customization of the types used by a particular signature
    system, e.g. using custom error types.

It may still make sense to continue to explore the above tradeoffs, but
with a *new* set of traits which are intended to be
implementor-friendly, rather than consumer friendly. The existing
[`Signer`](trait.Signer.html "trait signature::Signer") and
[`Verifier`](trait.Verifier.html "trait signature::Verifier") traits
could have blanket impls for the “provider-friendly” traits. However, as
noted above this is a design space easily explored after stabilizing the
consumer-oriented traits, and thus we consider these more important.

That said, below are some caveats of trying to design such traits, and
why we haven’t actively pursued them:

-   Generics in the return position are already used to select which
    trait impl to use, i.e. for a particular signature algorithm/system.
    Avoiding a unified, concrete signature type adds another dimension
    to complexity and compiler errors, and in our experience makes them
    unsuitable for this sort of API. We believe such an API is the
    natural one for signature systems, reflecting the natural way they
    are written absent a trait.
-   Associated types preclude multiple implementations of the same
    trait. These parameters are common in signature systems, notably
    ones which support different serializations of a signature (e.g. raw
    vs ASN.1).
-   Digital signatures are almost always larger than the present
    32-entry trait impl limitation on array types, which complicates
    bounds for these types (particularly things like `From` or `Borrow`
    bounds).

## <a href="#unstable-features" class="doc-anchor">§</a>Unstable features

Despite being post-1.0, this crate includes off-by-default unstable
optional features, each of which depends on a pre-1.0 crate.

These features are considered exempt from SemVer. See the [SemVer
policy](#semver-policy) above for more information.

The following unstable features are presently supported:

-   `digest`: enables the
    [`DigestSigner`](trait.DigestSigner.html "trait signature::DigestSigner")
    and
    [`DigestVerifier`](trait.DigestVerifier.html "trait signature::DigestVerifier")
    traits which are based on the [`Digest`](https://docs.rs/digest/)
    trait from the [`digest`](https://docs.rs/digest/) crate. These
    traits are used for representing signature systems based on the
    [Fiat-Shamir
    heuristic](https://en.wikipedia.org/wiki/Fiat%E2%80%93Shamir_heuristic)
    which compute a random challenge value to sign by computing a
    cryptographically secure digest of the input message.
-   `rand_core`: enables the
    [`RandomizedSigner`](trait.RandomizedSigner.html "trait signature::RandomizedSigner")
    trait for signature systems which rely on a cryptographically secure
    random number generator for security.

NOTE: the [`async-signature`](https://docs.rs/async-signature) crate
contains experimental `async` support for
[`Signer`](trait.Signer.html "trait signature::Signer") and
[`DigestSigner`](trait.DigestSigner.html "trait signature::DigestSigner").

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../digest/index.html" class="mod"
title="mod digest"><code>digest</code></a>`;`

`pub use `<a href="https://rust-random.github.io/rand/rand_core/index.html"
class="mod" title="mod rand_core"><code>rand_core</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="hazmat/index.html" class="mod"
title="mod signature::hazmat">hazmat</a>  
Hazardous Materials: low-level APIs which can be insecure if misused.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct signature::Error">Error</a>  
Signature errors.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.DigestSigner.html" class="trait"
title="trait signature::DigestSigner">DigestSigner</a>  
Sign the given prehashed message
[`Digest`](../digest/digest/trait.Digest.html "trait digest::digest::Digest")
using `Self`.

<a href="trait.DigestVerifier.html" class="trait"
title="trait signature::DigestVerifier">DigestVerifier</a>  
Verify the provided signature for the given prehashed message
[`Digest`](../digest/digest/trait.Digest.html "trait digest::digest::Digest")
is authentic.

<a href="trait.Keypair.html" class="trait"
title="trait signature::Keypair">Keypair</a>  
Signing keypair with an associated verifying key.

<a href="trait.KeypairRef.html" class="trait"
title="trait signature::KeypairRef">KeypairRef</a>  
Signing keypair with an associated verifying key.

<a href="trait.PrehashSignature.html" class="trait"
title="trait signature::PrehashSignature">PrehashSignature</a>  
Marker trait for `Signature` types computable as `𝐒(𝐇(𝒎))` i.e. ones
which prehash a message to be signed as `𝐇(𝒎)`

<a href="trait.RandomizedDigestSigner.html" class="trait"
title="trait signature::RandomizedDigestSigner">RandomizedDigestSigner</a>  
Combination of
[`DigestSigner`](trait.DigestSigner.html "trait signature::DigestSigner")
and
[`RandomizedSigner`](trait.RandomizedSigner.html "trait signature::RandomizedSigner")
with support for computing a signature over a digest which requires
entropy from an RNG.

<a href="trait.RandomizedSigner.html" class="trait"
title="trait signature::RandomizedSigner">RandomizedSigner</a>  
Sign the given message using the provided external randomness source.

<a href="trait.SignatureEncoding.html" class="trait"
title="trait signature::SignatureEncoding">SignatureEncoding</a>  
Support for decoding/encoding signatures as bytes.

<a href="trait.Signer.html" class="trait"
title="trait signature::Signer">Signer</a>  
Sign the provided message bytestring using `Self` (e.g. a cryptographic
key or connection to an HSM), returning a digital signature.

<a href="trait.SignerMut.html" class="trait"
title="trait signature::SignerMut">SignerMut</a>  
Sign the provided message bytestring using `&mut Self` (e.g. an evolving
cryptographic key such as a stateful hash-based signature), returning a
digital signature.

<a href="trait.Verifier.html" class="trait"
title="trait signature::Verifier">Verifier</a>  
Verify the provided message bytestring using `Self` (e.g. a public key)

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type signature::Result">Result</a>  
Result type.
