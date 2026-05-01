Crate ring Copy item path

<span class="sub-heading"><a href="../src/ring/lib.rs.html#15-177" class="src">Source</a>
</span>

Expand description

# <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

Feature

Description

`alloc (default)`

Enable features that require use of the heap, RSA in particular.

`less-safe-getrandom-custom-or-rdrand`

Treat user-provided ("custom") and RDRAND-based `getrandom`
implementations as secure random number generators (see `SecureRandom`).
This feature only works with `os = "none"` targets. See
[`register_custom_getrandom`](https://docs.rs/getrandom/0.2.10/getrandom/macro.register_custom_getrandom.html)
and [RDRAND on
x86](https://docs.rs/getrandom/0.2.10/getrandom/#rdrand-on-x86) for
additional details.

`less-safe-getrandom-espidf`

Treat getrandom as a secure random number generator (see `SecureRandom`)
on the esp-idf target. While the esp-idf target does have hardware RNG,
it is beyond the scope of ring to ensure its configuration. This feature
allows ring to build on esp-idf despite the likelihood that RNG is not
secure. This feature only works with `os = espidf` targets. See
[](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/random.html)

`std`

Enable features that use libstd, in particular `std::error::Error`
integration. Implies \`alloc\`.

`wasm32_unknown_unknown_js`

When this feature is enabled, for the wasm32-unknown-unknown target, Web
APIs will be used to implement features like \`ring::rand\` that require
an operating environment of some kind. This has no effect for any other
target. This enables the \`getrandom\` crate's \`js\` feature.

# Modules<a href="#modules" class="anchor">§</a>

<a href="aead/index.html" class="mod" title="mod ring::aead">aead</a>  
Authenticated Encryption with Associated Data (AEAD).

<a href="agreement/index.html" class="mod"
title="mod ring::agreement">agreement</a>  
Key Agreement: ECDH, including X25519.

<a href="digest/index.html" class="mod"
title="mod ring::digest">digest</a>  
SHA-2 and the legacy SHA-1 digest algorithm.

<a href="error/index.html" class="mod" title="mod ring::error">error</a>  
Error reporting.

<a href="hkdf/index.html" class="mod" title="mod ring::hkdf">hkdf</a>  
HMAC-based Extract-and-Expand Key Derivation Function.

<a href="hmac/index.html" class="mod" title="mod ring::hmac">hmac</a>  
HMAC is specified in [RFC 2104](https://tools.ietf.org/html/rfc2104).

<a href="io/index.html" class="mod" title="mod ring::io">io</a>  
Serialization and deserialization.

<a href="pbkdf2/index.html" class="mod"
title="mod ring::pbkdf2">pbkdf2</a>  
PBKDF2 derivation and verification.

<a href="pkcs8/index.html" class="mod" title="mod ring::pkcs8">pkcs8</a>  
PKCS#8 is specified in [RFC 5958](https://tools.ietf.org/html/rfc5958).

<a href="rand/index.html" class="mod" title="mod ring::rand">rand</a>  
Cryptographic pseudo-random number generation.

<a href="rsa/index.html" class="mod" title="mod ring::rsa">rsa</a>  
RSA.

<a href="signature/index.html" class="mod"
title="mod ring::signature">signature</a>  
Public key signatures: signing and verification.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.test_file.html" class="macro"
title="macro ring::test_file">test_file</a><span class="stab deprecated" title="">Deprecated</span>  
References a test input file.
