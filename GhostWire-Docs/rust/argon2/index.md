Crate argon2 Copy item path

<span class="sub-heading"><a href="../src/argon2/lib.rs.html#1-696" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-argon2" class="doc-anchor">§</a>RustCrypto: Argon2

[![crate](https://buildstats.info/crate/argon2)](https://crates.io/crates/argon2)
[![Docs](https://docs.rs/argon2/badge.svg)](https://docs.rs/argon2/)
[![Build
Status](https://github.com/RustCrypto/password-hashes/workflows/argon2/badge.svg?branch=master&event=push)](https://github.com/RustCrypto/password-hashes/actions?query=workflow%3Aargon2)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260046-password-hashes)

Pure Rust implementation of the
[Argon2](https://en.wikipedia.org/wiki/Argon2) password hashing
function.

[Documentation](https://docs.rs/argon2/)

# <a href="#about" class="doc-anchor">§</a>About

Argon2 is a memory-hard [key derivation
function](https://en.wikipedia.org/wiki/Key_derivation_function) chosen
as the winner of the [Password Hashing
Competition](https://www.password-hashing.net/) in July 2015.

It implements the following three algorithmic variants:

-   **Argon2d**: maximizes resistance to GPU cracking attacks
-   **Argon2i**: optimized to resist side-channel attacks
-   **Argon2id**: (default) hybrid version combining both Argon2i and
    Argon2d

Support is provided for embedded (i.e. `no_std`) environments, including
ones without `alloc` support.

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

Rust **1.65** or higher.

Minimum supported Rust version can be changed in the future, but it will
be done with a minor version bump.

## <a href="#semver-policy" class="doc-anchor">§</a>SemVer Policy

-   All on-by-default features of this library are covered by SemVer
-   MSRV is considered exempt from SemVer as noted above

## <a href="#license" class="doc-anchor">§</a>License

Licensed under either of:

-   [Apache License, Version
    2.0](http://www.apache.org/licenses/LICENSE-2.0)
-   [MIT license](http://opensource.org/licenses/MIT)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

## <a href="#usage" class="doc-anchor">§</a>Usage

### <a href="#password-hashing" class="doc-anchor">§</a>Password Hashing

This API hashes a password to a “PHC string” suitable for the purposes
of password-based authentication. Do not use this API to derive
cryptographic keys: see the “key derivation” usage example below.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

let password = b"hunter42"; // Bad password; don't actually use!
let salt = SaltString::generate(&mut OsRng);

// Argon2 with default params (Argon2id v19)
let argon2 = Argon2::default();

// Hash password to PHC string ($argon2id$v=19$...)
let password_hash = argon2.hash_password(password, &salt)?.to_string();

// Verify password against PHC string.
//
// NOTE: hash params from `parsed_hash` are used instead of what is configured in the
// `Argon2` instance.
let parsed_hash = PasswordHash::new(&password_hash)?;
assert!(Argon2::default().verify_password(password, &parsed_hash).is_ok());
```

### <a href="#key-derivation" class="doc-anchor">§</a>Key Derivation

This API is useful for transforming a password into cryptographic keys
for e.g. password-based encryption.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use argon2::Argon2;

let password = b"hunter42"; // Bad password; don't actually use!
let salt = b"example salt"; // Salt should be unique per password

let mut output_key_material = [0u8; 32]; // Can be any desired size
Argon2::default().hash_password_into(password, salt, &mut output_key_material)?;
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../password_hash/index.html" class="mod"
title="mod password_hash"><code>password_hash</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Argon2.html" class="struct"
title="struct argon2::Argon2">Argon2</a>  
Argon2 context.

<a href="struct.AssociatedData.html" class="struct"
title="struct argon2::AssociatedData">AssociatedData</a>  
Associated data

<a href="struct.Block.html" class="struct"
title="struct argon2::Block">Block</a>  
Structure for the (1 KiB) memory block implemented as 128 64-bit words.

<a href="struct.KeyId.html" class="struct"
title="struct argon2::KeyId">KeyId</a>  
Key identifier

<a href="struct.Params.html" class="struct"
title="struct argon2::Params">Params</a>  
Argon2 password hash parameters.

<a href="struct.ParamsBuilder.html" class="struct"
title="struct argon2::ParamsBuilder">ParamsBuilder</a>  
Builder for Argon2
[`Params`](struct.Params.html "struct argon2::Params").

<a href="struct.PasswordHash.html" class="struct"
title="struct argon2::PasswordHash">PasswordHash</a>  
Password hash.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Algorithm.html" class="enum"
title="enum argon2::Algorithm">Algorithm</a>  
Argon2 primitive type: variants of the algorithm.

<a href="enum.Error.html" class="enum"
title="enum argon2::Error">Error</a>  
Error type.

<a href="enum.Version.html" class="enum"
title="enum argon2::Version">Version</a>  
Version of the algorithm.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ARGON2D_IDENT.html" class="constant"
title="constant argon2::ARGON2D_IDENT">ARGON2D_IDENT</a>  
Argon2d algorithm identifier

<a href="constant.ARGON2ID_IDENT.html" class="constant"
title="constant argon2::ARGON2ID_IDENT">ARGON2ID_IDENT</a>  
Argon2id algorithm identifier

<a href="constant.ARGON2I_IDENT.html" class="constant"
title="constant argon2::ARGON2I_IDENT">ARGON2I_IDENT</a>  
Argon2i algorithm identifier

<a href="constant.MAX_PWD_LEN.html" class="constant"
title="constant argon2::MAX_PWD_LEN">MAX_PWD_LEN</a>  
Maximum password length in bytes.

<a href="constant.MAX_SALT_LEN.html" class="constant"
title="constant argon2::MAX_SALT_LEN">MAX_SALT_LEN</a>  
Maximum salt length in bytes.

<a href="constant.MAX_SECRET_LEN.html" class="constant"
title="constant argon2::MAX_SECRET_LEN">MAX_SECRET_LEN</a>  
Maximum secret key length in bytes.

<a href="constant.MIN_SALT_LEN.html" class="constant"
title="constant argon2::MIN_SALT_LEN">MIN_SALT_LEN</a>  
Minimum salt length in bytes.

<a href="constant.RECOMMENDED_SALT_LEN.html" class="constant"
title="constant argon2::RECOMMENDED_SALT_LEN">RECOMMENDED_SALT_LEN</a>  
Recommended salt length for password hashing in bytes.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.PasswordHasher.html" class="trait"
title="trait argon2::PasswordHasher">PasswordHasher</a>  
Trait for password hashing functions.

<a href="trait.PasswordVerifier.html" class="trait"
title="trait argon2::PasswordVerifier">PasswordVerifier</a>  
Trait for password verification.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type argon2::Result">Result</a>  
Result with argon2’s [`Error`](enum.Error.html "enum argon2::Error")
type.
