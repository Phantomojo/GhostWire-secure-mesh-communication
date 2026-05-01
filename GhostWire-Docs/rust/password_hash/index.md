Crate password\_hash Copy item path

<span class="sub-heading"><a href="../src/password_hash/lib.rs.html#1-389" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-password-hashing-traits" class="doc-anchor">§</a>RustCrypto: Password Hashing Traits

[![crate](https://buildstats.info/crate/password-hash)](https://crates.io/crates/password-hash)
[![Docs](https://docs.rs/password-hash/badge.svg)](https://docs.rs/password-hash/)
[![Build
Status](https://github.com/RustCrypto/traits/workflows/password-hash/badge.svg?branch=master&event=push)](https://github.com/RustCrypto/traits/actions?query=workflow:password-hash)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260046-password-hashes)

Traits which describe the functionality of [password hashing
algorithms](https://en.wikipedia.org/wiki/Cryptographic_hash_function#Password_verification).

[Documentation](https://docs.rs/password-hash/)

## <a href="#about" class="doc-anchor">§</a>About

Provides a `no_std`-friendly implementation of the [Password Hashing
Competition (PHC) string format
specification](https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md)
(a well-defined subset of the [Modular Crypt Format a.k.a.
MCF](https://passlib.readthedocs.io/en/stable/modular_crypt_format.html))
which works in conjunction with the traits this crate defines.

## <a href="#supported-crates" class="doc-anchor">§</a>Supported Crates

See
[RustCrypto/password-hashes](https://github.com/RustCrypto/password-hashes)
for algorithm implementations which use this crate for interoperability:

-   [`argon2`](https://docs.rs/argon2) - Argon2 memory hard key
    derivation function
-   [`pbkdf2`](https://docs.rs/pbkdf2) - Password-Based Key Derivation
    Function v2
-   [`scrypt`](https://docs.rs/scrypt) - scrypt key derivation function

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

Rust **1.60** or higher.

Minimum supported Rust version may be changed in the future, but it will
be accompanied by a minor version bump.

## <a href="#semver-policy" class="doc-anchor">§</a>SemVer Policy

-   All on-by-default features of this library are covered by SemVer
-   MSRV is considered exempt from SemVer as noted above

## <a href="#license" class="doc-anchor">§</a>License

Licensed under either of:

-   [Apache License, Version
    2.0](https://www.apache.org/licenses/LICENSE-2.0)
-   [MIT license](https://opensource.org/licenses/MIT)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

# <a href="#usage" class="doc-anchor">§</a>Usage

This crate represents password hashes using the
[`PasswordHash`](struct.PasswordHash.html "struct password_hash::PasswordHash")
type, which represents a parsed “PHC string” with the following format:

``` language-text
$<id>[$v=<version>][$<param>=<value>(,<param>=<value>)*][$<salt>[$<hash>]]
```

For more information, please see the documentation for
[`PasswordHash`](struct.PasswordHash.html "struct password_hash::PasswordHash").

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::errors::`<a href="errors/enum.Error.html" class="enum"
title="enum password_hash::errors::Error"><code>Error</code></a>`;`

`pub use crate::errors::`<a href="errors/type.Result.html" class="type"
title="type password_hash::errors::Result"><code>Result</code></a>`;`

`pub use `<a href="../rand_core/index.html" class="mod"
title="mod rand_core"><code>rand_core</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="errors/index.html" class="mod"
title="mod password_hash::errors">errors</a>  
Error types.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Ident.html" class="struct"
title="struct password_hash::Ident">Ident</a>  
Algorithm or parameter identifier.

<a href="struct.Output.html" class="struct"
title="struct password_hash::Output">Output</a>  
Output from password hashing functions, i.e. the “hash” or “digest” as
raw bytes.

<a href="struct.ParamsString.html" class="struct"
title="struct password_hash::ParamsString">ParamsString</a>  
Algorithm parameter string.

<a href="struct.PasswordHash.html" class="struct"
title="struct password_hash::PasswordHash">PasswordHash</a>  
Password hash.

<a href="struct.PasswordHashString.html" class="struct"
title="struct password_hash::PasswordHashString">PasswordHashString</a>  
Serialized
[`PasswordHash`](struct.PasswordHash.html "struct password_hash::PasswordHash").

<a href="struct.Salt.html" class="struct"
title="struct password_hash::Salt">Salt</a>  
Salt string.

<a href="struct.SaltString.html" class="struct"
title="struct password_hash::SaltString">SaltString</a>  
Owned stack-allocated equivalent of
[`Salt`](struct.Salt.html "struct password_hash::Salt").

<a href="struct.Value.html" class="struct"
title="struct password_hash::Value">Value</a>  
Algorithm parameter value string.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Encoding.html" class="enum"
title="enum password_hash::Encoding">Encoding</a>  
Base64 encoding variants.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.McfHasher.html" class="trait"
title="trait password_hash::McfHasher">McfHasher</a>  
Trait for password hashing algorithms which support the legacy [Modular
Crypt Format
(MCF)](https://passlib.readthedocs.io/en/stable/modular_crypt_format.html).

<a href="trait.PasswordHasher.html" class="trait"
title="trait password_hash::PasswordHasher">PasswordHasher</a>  
Trait for password hashing functions.

<a href="trait.PasswordVerifier.html" class="trait"
title="trait password_hash::PasswordVerifier">PasswordVerifier</a>  
Trait for password verification.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Decimal.html" class="type"
title="type password_hash::Decimal">Decimal</a>  
Type used to represent decimal (i.e. integer) values.
