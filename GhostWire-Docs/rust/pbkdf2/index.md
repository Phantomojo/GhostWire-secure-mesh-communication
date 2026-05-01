Crate pbkdf2 Copy item path

<span class="sub-heading"><a href="../src/pbkdf2/lib.rs.html#1-282" class="src">Source</a>
</span>

Expand description

This crate implements the PBKDF2 key derivation function as specified in
[RFC 2898](https://tools.ietf.org/html/rfc2898).

# <a href="#examples" class="doc-anchor">§</a>Examples

PBKDF2 is defined in terms of a keyed pseudo-random function (PRF). Most
commonly HMAC is used as this PRF. In such cases you can use
[`pbkdf2_hmac`](fn.pbkdf2_hmac.html "fn pbkdf2::pbkdf2_hmac") and
[`pbkdf2_hmac_array`](fn.pbkdf2_hmac_array.html "fn pbkdf2::pbkdf2_hmac_array")
functions. The former accepts a byte slice which gets filled with
generated key, while the former returns an array with generated key of
requested length.

``` {.rust .rust-example-rendered}
use hex_literal::hex;
use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array};
use sha2::Sha256;

let password = b"password";
let salt = b"salt";
// number of iterations
let n = 600_000;
// Expected value of generated key
let expected = hex!("669cfe52482116fda1aa2cbe409b2f56c8e45637");

let mut key1 = [0u8; 20];
pbkdf2_hmac::<Sha256>(password, salt, n, &mut key1);
assert_eq!(key1, expected);

let key2 = pbkdf2_hmac_array::<Sha256, 20>(password, salt, n);
assert_eq!(key2, expected);
```

If you want to use a different PRF, then you can use
[`pbkdf2`](fn.pbkdf2.html "fn pbkdf2::pbkdf2") and
[`pbkdf2_array`](fn.pbkdf2_array.html "fn pbkdf2::pbkdf2_array")
functions.

This crates also provides the high-level password-hashing API through
the [`Pbkdf2`](struct.Pbkdf2.html "struct pbkdf2::Pbkdf2") struct and
traits defined in the
[`password-hash`](../password_hash/index.html "mod password_hash")
crate.

Add the following to your crate’s `Cargo.toml` to import it:

``` language-toml
[dependencies]
pbkdf2 = { version = "0.12", features = ["simple"] }
rand_core = { version = "0.6", features = ["std"] }
```

The following example demonstrates the high-level password hashing API:

``` {.rust .rust-example-rendered}
use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};

let password = b"hunter42"; // Bad password; don't actually use!
let salt = SaltString::generate(&mut OsRng);

// Hash password to PHC string ($pbkdf2-sha256$...)
let password_hash = Pbkdf2.hash_password(password, &salt)?.to_string();

// Verify password against PHC string
let parsed_hash = PasswordHash::new(&password_hash)?;
assert!(Pbkdf2.verify_password(password, &parsed_hash).is_ok());
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../password_hash/index.html" class="mod"
title="mod password_hash"><code>password_hash</code></a>`;`

`pub use `<a href="../hmac/index.html" class="mod"
title="mod hmac"><code>hmac</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Params.html" class="struct"
title="struct pbkdf2::Params">Params</a>  
PBKDF2 params

<a href="struct.Pbkdf2.html" class="struct"
title="struct pbkdf2::Pbkdf2">Pbkdf2</a>  
PBKDF2 type for use with
[`PasswordHasher`](../password_hash/traits/trait.PasswordHasher.html "trait password_hash::traits::PasswordHasher").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Algorithm.html" class="enum"
title="enum pbkdf2::Algorithm">Algorithm</a>  
PBKDF2 variants.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.pbkdf2.html" class="fn" title="fn pbkdf2::pbkdf2">pbkdf2</a>  
Generic implementation of PBKDF2 algorithm which accepts an arbitrary
keyed PRF.

<a href="fn.pbkdf2_array.html" class="fn"
title="fn pbkdf2::pbkdf2_array">pbkdf2_array</a>  
A variant of the [`pbkdf2`](fn.pbkdf2.html "fn pbkdf2::pbkdf2") function
which returns an array instead of filling an input slice.

<a href="fn.pbkdf2_hmac.html" class="fn"
title="fn pbkdf2::pbkdf2_hmac">pbkdf2_hmac</a>  
A variant of the [`pbkdf2`](fn.pbkdf2.html "fn pbkdf2::pbkdf2") function
which uses HMAC for PRF. It’s generic over (eager) hash functions.

<a href="fn.pbkdf2_hmac_array.html" class="fn"
title="fn pbkdf2::pbkdf2_hmac_array">pbkdf2_hmac_array</a>  
A variant of the
[`pbkdf2_hmac`](fn.pbkdf2_hmac.html "fn pbkdf2::pbkdf2_hmac") function
which returns an array instead of filling an input slice.
