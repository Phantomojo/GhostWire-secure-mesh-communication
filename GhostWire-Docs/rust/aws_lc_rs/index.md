Crate aws\_lc\_rs Copy item path

<span class="sub-heading"><a href="../src/aws_lc_rs/lib.rs.html#5-371" class="src">Source</a>
</span>

Expand description

A [*ring*](https://github.com/briansmith/ring)-compatible crypto library
using the cryptographic operations provided by
[*AWS-LC*](https://github.com/aws/aws-lc). It uses either the
auto-generated [*aws-lc-sys*](https://crates.io/crates/aws-lc-sys) or
[*aws-lc-fips-sys*](https://crates.io/crates/aws-lc-fips-sys) Foreign
Function Interface (FFI) crates found in this repository for invoking
*AWS-LC*.

# <a href="#build" class="doc-anchor">§</a>Build

`aws-lc-rs` is available through
[crates.io](https://crates.io/crates/aws-lc-rs). It can be added to your
project in the [standard
way](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
using `Cargo.toml`:

``` language-toml
[dependencies]
aws-lc-rs = "1"
```

Consuming projects will need a C/C++ compiler to build.

**Non-FIPS builds (default):**

-   CMake is **never** required
-   Bindgen is **never** required (pre-generated bindings are provided)
-   Go is **never** required

**FIPS builds:** Require **CMake**, **Go**, and potentially **bindgen**
depending on the target platform.

See our [User Guide](https://aws.github.io/aws-lc-rs/) for guidance on
installing build requirements.

# <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

#### <a href="#alloc-default" class="doc-anchor">§</a>alloc (default)

Allows implementation to allocate values of arbitrary size. (The meaning
of this feature differs from the “alloc” feature of *ring*.) Currently,
this is required by the `io::writer` module.

#### <a href="#ring-io-default" class="doc-anchor">§</a>ring-io (default)

Enable feature to access the `io` module.

#### <a href="#ring-sig-verify-default" class="doc-anchor">§</a>ring-sig-verify (default)

Enable feature to preserve compatibility with ring’s
`signature::VerificationAlgorithm::verify` function. This adds a
requirement on `untrusted = "0.7.1"`.

#### <a href="#fips" class="doc-anchor">§</a>fips

Enable this feature to have aws-lc-rs use the
[*aws-lc-fips-sys*](https://crates.io/crates/aws-lc-fips-sys) crate for
the cryptographic implementations. The aws-lc-fips-sys crate provides
bindings to the latest version of the AWS-LC-FIPS module that has
completed FIPS validation testing by an accredited lab and has been
submitted to NIST for certification. This will continue to be the case
as we periodically submit new versions of the AWS-LC-FIPS module to NIST
for certification. Currently, aws-lc-fips-sys binds to [AWS-LC-FIPS
3.0.x](https://github.com/aws/aws-lc/tree/fips-2024-09-27).

Consult with your local FIPS compliance team to determine the version of
AWS-LC-FIPS module that you require. Consumers needing to remain on a
previous version of the AWS-LC-FIPS module should pin to specific
versions of aws-lc-rs to avoid automatically being upgraded to a newer
module version. (See [cargo’s
documentation](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
on how to specify dependency versions.)

| AWS-LC-FIPS module | aws-lc-rs  |
|--------------------|------------|
| 2.0.x              | &lt;1.12.0 |
| 3.0.x              | *latest*   |

Refer to the [NIST Cryptographic Module Validation Program’s Modules In
Progress
List](https://csrc.nist.gov/Projects/cryptographic-module-validation-program/modules-in-process/Modules-In-Process-List)
for the latest status of the static or dynamic AWS-LC Cryptographic
Module. Please see the [FIPS.md in the aws-lc
repository](https://github.com/aws/aws-lc/blob/main/crypto/fipsmodule/FIPS.md)
for relevant security policies and information on supported operating
environments. We will also update our release notes and documentation to
reflect any changes in FIPS certification status.

#### <a href="#non-fips" class="doc-anchor">§</a>non-fips

Enable this feature to guarantee that the non-FIPS
[*aws-lc-sys*](https://crates.io/crates/aws-lc-sys) crate is used for
cryptographic implementations. This feature is mutually exclusive with
the `fips` feature - enabling both will result in a compile-time error.
Use this feature when you need a compile-time guarantee that your build
is using the non-FIPS cryptographic module.

#### <a href="#asan" class="doc-anchor">§</a>asan

Performs an “address sanitizer” build. This can be used to help detect
memory leaks. See the [“Address Sanitizer”
section](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html#addresssanitizer)
of the [Rust Unstable
Book](https://doc.rust-lang.org/beta/unstable-book/).

#### <a href="#bindgen" class="doc-anchor">§</a>bindgen

Causes `aws-lc-sys` or `aws-lc-fips-sys` to generates fresh bindings for
AWS-LC instead of using the pre-generated bindings. This feature
requires `libclang` to be installed. See the
[requirements](https://rust-lang.github.io/rust-bindgen/requirements.html)
for [rust-bindgen](https://github.com/rust-lang/rust-bindgen)

#### <a href="#prebuilt-nasm" class="doc-anchor">§</a>prebuilt-nasm

Enables the use of crate provided prebuilt NASM objects under certain
conditions. This only affects builds for Windows x86-64 platforms. This
feature is ignored if the “fips” feature is also enabled.

Use of prebuilt NASM objects is prevented if either of the following
conditions are true:

-   The NASM assembler is detected in the build environment
-   `AWS_LC_SYS_PREBUILT_NASM` environment variable is set with a value
    of `0`

Be aware that [features are
additive](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification);
by enabling this feature, it is enabled for all crates within the same
build.

#### <a href="#dev-tests-only" class="doc-anchor">§</a>dev-tests-only

Enables the `rand::unsealed` module, which re-exports the normally
sealed `SecureRandom` trait. This allows consumers to provide their own
implementations of `SecureRandom` (e.g., a deterministic RNG) for
testing purposes. When enabled, a `mut_fill` method is also available on
`SecureRandom`.

This feature is restricted to **dev/debug profile builds only** —
attempting to use it in a release build will result in a compile-time
error.

It can be enabled in two ways:

-   **Feature flag:** `cargo test --features dev-tests-only`
-   **Environment variable:** `AWS_LC_RS_DEV_TESTS_ONLY=1 cargo test`

**⚠️ Warning:** This feature is intended **only** for development and
testing. It must not be used in production builds. The `rand::unsealed`
module and `mut_fill` method are not part of the stable public API and
may change without notice.

# <a href="#use-of-prebuilt-nasm-objects" class="doc-anchor">§</a>Use of prebuilt NASM objects

Prebuilt NASM objects are **only** applicable to Windows x86-64
platforms. They are **never** used on any other platform (Linux, macOS,
etc.).

For Windows x86 and x86-64, NASM is required for assembly code
compilation. On these platforms, we recommend that you install [the NASM
assembler](https://www.nasm.us/). **If NASM is detected in the build
environment, it is always used** to compile the assembly files. Prebuilt
NASM objects are only used as a fallback.

If a NASM assembler is not available, and the “fips” feature is not
enabled, then the build fails unless one of the following conditions are
true:

-   You are building for `x86-64` and either:
    -   The `AWS_LC_SYS_PREBUILT_NASM` environment variable is found and
        has a value of “1”; OR
    -   `AWS_LC_SYS_PREBUILT_NASM` is *not found* in the environment AND
        the “prebuilt-nasm” feature has been enabled.

If the above cases apply, then the crate provided prebuilt NASM objects
will be used for the build. To prevent usage of prebuilt NASM objects,
install NASM in the build environment and/or set the variable
`AWS_LC_SYS_PREBUILT_NASM` to `0` in the build environment to prevent
their use.

## <a href="#about-prebuilt-nasm-objects" class="doc-anchor">§</a>About prebuilt NASM objects

Prebuilt NASM objects are generated using automation similar to the
crate provided pregenerated bindings. See the repository’s [GitHub
workflow
configuration](https://github.com/aws/aws-lc-rs/blob/main/.github/workflows/sys-bindings-generator.yml)
for more information. The prebuilt NASM objects are checked into the
repository and are [available for
inspection](https://github.com/aws/aws-lc-rs/tree/main/aws-lc-sys/builder/prebuilt-nasm).
For each PR submitted, [CI
verifies](https://github.com/aws/aws-lc-rs/blob/main/.github/workflows/tests.yml)
that the NASM objects newly built from source match the NASM objects
currently in the repository.

# <a href="#ring-compatibility" class="doc-anchor">§</a>*ring*-compatibility

Although this library attempts to be fully compatible with *ring*
(v0.16.x), there are a few places where our behavior is observably
different.

-   Our implementation requires the `std` library. We currently do not
    support a
    [`#![no_std]`](https://docs.rust-embedded.org/book/intro/no-std.html)
    build.
-   `aws-lc-rs` supports the platforms supported by `aws-lc-sys` and
    AWS-LC. See the [Platform
    Support](https://aws.github.io/aws-lc-rs/platform_support.html) page
    in our User Guide.
-   `Ed25519KeyPair::from_pkcs8` and
    `Ed25519KeyPair::from_pkcs8_maybe_unchecked` both support parsing of
    v1 or v2 PKCS#8 documents. If a v2 encoded key is provided to either
    function, public key component, if present, will be verified to
    match the one derived from the encoded private key.

# <a href="#post-quantum-cryptography" class="doc-anchor">§</a>Post-Quantum Cryptography

Details on the post-quantum algorithms supported by aws-lc-rs can be
found at
[PQREADME](https://github.com/aws/aws-lc/tree/main/crypto/fipsmodule/PQREADME.md).

# <a href="#motivation" class="doc-anchor">§</a>Motivation

Rust developers increasingly need to deploy applications that meet US
and Canadian government cryptographic requirements. We evaluated how to
deliver FIPS validated cryptography in idiomatic and performant Rust,
built around our AWS-LC offering. We found that the popular ring (v0.16)
library fulfilled much of the cryptographic needs in the Rust community,
but it did not meet the needs of developers with FIPS requirements. Our
intention is to contribute a drop-in replacement for ring that provides
FIPS support and is compatible with the ring API. Rust developers with
prescribed cryptographic requirements can seamlessly integrate aws-lc-rs
into their applications and deploy them into AWS Regions.

# Modules<a href="#modules" class="anchor">§</a>

<a href="aead/index.html" class="mod"
title="mod aws_lc_rs::aead">aead</a>  
Authenticated Encryption with Associated Data (AEAD).

<a href="agreement/index.html" class="mod"
title="mod aws_lc_rs::agreement">agreement</a>  
Key Agreement: ECDH, including X25519.

<a href="cipher/index.html" class="mod"
title="mod aws_lc_rs::cipher">cipher</a>  
Block and Stream Ciphers for Encryption and Decryption.

<a href="cmac/index.html" class="mod"
title="mod aws_lc_rs::cmac">cmac</a>  
CMAC is specified in [RFC 4493](https://tools.ietf.org/html/rfc4493) and
[NIST SP
800-38B](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-38b.pdf).

<a href="constant_time/index.html" class="mod"
title="mod aws_lc_rs::constant_time">constant_time</a>  
Constant-time operations.

<a href="digest/index.html" class="mod"
title="mod aws_lc_rs::digest">digest</a>  
SHA-2 and the legacy SHA-1 digest algorithm.

<a href="encoding/index.html" class="mod"
title="mod aws_lc_rs::encoding">encoding</a>  
Serialization formats

<a href="error/index.html" class="mod"
title="mod aws_lc_rs::error">error</a>  
Error reporting.

<a href="hkdf/index.html" class="mod"
title="mod aws_lc_rs::hkdf">hkdf</a>  
HMAC-based Extract-and-Expand Key Derivation Function.

<a href="hmac/index.html" class="mod"
title="mod aws_lc_rs::hmac">hmac</a>  
HMAC is specified in [RFC 2104](https://tools.ietf.org/html/rfc2104).

<a href="iv/index.html" class="mod" title="mod aws_lc_rs::iv">iv</a>  
Initialization Vector (IV) cryptographic primitives

<a href="kdf/index.html" class="mod" title="mod aws_lc_rs::kdf">kdf</a>  
Key-based Key Derivation Function (KBKDF) in Counter Mode

<a href="kem/index.html" class="mod" title="mod aws_lc_rs::kem">kem</a>  
Key-Encapsulation Mechanisms (KEMs), including support for Kyber Round 3
Submission.

<a href="key_wrap/index.html" class="mod"
title="mod aws_lc_rs::key_wrap">key_wrap</a>  
Key Wrap Algorithms.

<a href="pbkdf2/index.html" class="mod"
title="mod aws_lc_rs::pbkdf2">pbkdf2</a>  
PBKDF2 derivation and verification.

<a href="pkcs8/index.html" class="mod"
title="mod aws_lc_rs::pkcs8">pkcs8</a>  
PKCS#8 is specified in [RFC 5208](https://tools.ietf.org/html/rfc5208.).

<a href="rand/index.html" class="mod"
title="mod aws_lc_rs::rand">rand</a>  
Cryptographic pseudo-random number generation.

<a href="rsa/index.html" class="mod" title="mod aws_lc_rs::rsa">rsa</a>  
RSA Signature and Encryption Support.

<a href="signature/index.html" class="mod"
title="mod aws_lc_rs::signature">signature</a>  
Public key signatures: signing and verification.

<a href="tls_prf/index.html" class="mod"
title="mod aws_lc_rs::tls_prf">tls_prf</a>  
TLS 1.2 PRF API’s for usage in [RFC
5246](https://www.rfc-editor.org/rfc/rfc5246) and [RFC
7627](https://www.rfc-editor.org/rfc/rfc7627).

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.test_file.html" class="macro"
title="macro aws_lc_rs::test_file">test_file</a>  
References a test input file.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.init.html" class="fn" title="fn aws_lc_rs::init">init</a>  
Initialize the *AWS-LC* library. (This should generally not be needed.)

<a href="fn.try_fips_cpu_jitter_entropy.html" class="fn"
title="fn aws_lc_rs::try_fips_cpu_jitter_entropy">try_fips_cpu_jitter_entropy</a>  
Indicates whether the underlying implementation is FIPS.

<a href="fn.try_fips_mode.html" class="fn"
title="fn aws_lc_rs::try_fips_mode">try_fips_mode</a>  
Indicates whether the underlying implementation is FIPS.
