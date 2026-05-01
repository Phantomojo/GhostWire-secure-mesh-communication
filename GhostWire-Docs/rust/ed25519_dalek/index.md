Crate ed25519\_dalek Copy item path

<span class="sub-heading"><a href="../src/ed25519_dalek/lib.rs.html#10-294" class="src">Source</a>
</span>

Expand description

A Rust implementation of ed25519 key generation, signing, and
verification.

# <a href="#example" class="doc-anchor">§</a>Example

Creating an ed25519 signature on a message is simple.

First, we need to generate a `SigningKey`, which includes both public
and secret halves of an asymmetric key. To do so, we need a
cryptographically secure pseudorandom number generator (CSPRNG). For
this example, we’ll use the operating system’s builtin PRNG:

``` {.rust .rust-example-rendered}
// $ cargo add ed25519_dalek --features rand_core
use rand::rngs::OsRng;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;

let mut csprng = OsRng;
let signing_key: SigningKey = SigningKey::generate(&mut csprng);
```

We can now use this `signing_key` to sign a message:

``` {.rust .rust-example-rendered}
use ed25519_dalek::{Signature, Signer};
let message: &[u8] = b"This is a test of the tsunami alert system.";
let signature: Signature = signing_key.sign(message);
```

As well as to verify that this is, indeed, a valid signature on that
`message`:

``` {.rust .rust-example-rendered}
use ed25519_dalek::Verifier;
assert!(signing_key.verify(message, &signature).is_ok());
```

Anyone else, given the `public` half of the `signing_key` can also
easily verify this signature:

``` {.rust .rust-example-rendered}
use ed25519_dalek::{VerifyingKey, Verifier};

let verifying_key: VerifyingKey = signing_key.verifying_key();
assert!(verifying_key.verify(message, &signature).is_ok());
```

## <a href="#serialisation" class="doc-anchor">§</a>Serialisation

`VerifyingKey`s, `SecretKey`s, `SigningKey`s, and `Signature`s can be
serialised into byte-arrays by calling `.to_bytes()`. It’s perfectly
acceptable and safe to transfer and/or store those bytes. (Of course,
never transfer your secret key to anyone else, since they will only need
the public key to verify your signatures!)

``` {.rust .rust-example-rendered}
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, KEYPAIR_LENGTH, SIGNATURE_LENGTH};

let verifying_key_bytes: [u8; PUBLIC_KEY_LENGTH] = signing_key.verifying_key().to_bytes();
let secret_key_bytes: [u8; SECRET_KEY_LENGTH] = signing_key.to_bytes();
let signing_key_bytes:    [u8; KEYPAIR_LENGTH]    = signing_key.to_keypair_bytes();
let signature_bytes:  [u8; SIGNATURE_LENGTH]  = signature.to_bytes();
```

And similarly, decoded from bytes with `::from_bytes()`:

``` {.rust .rust-example-rendered}
let verifying_key: VerifyingKey = VerifyingKey::from_bytes(&verifying_key_bytes)?;
let signing_key: SigningKey = SigningKey::from_bytes(&signing_key_bytes);
let signature: Signature = Signature::try_from(&signature_bytes[..])?;
```

### <a href="#pkcs8-key-encoding" class="doc-anchor">§</a>PKCS#8 Key Encoding

PKCS#8 is a private key format with support for multiple algorithms. It
can be encoded as binary (DER) or text (PEM).

You can recognize PEM-encoded PKCS#8 keys by the following:

``` language-text
-----BEGIN PRIVATE KEY-----
```

To use PKCS#8, you need to enable the `pkcs8` crate feature.

The following traits can be used to decode/encode
[`SigningKey`](struct.SigningKey.html "struct ed25519_dalek::SigningKey")
and
[`VerifyingKey`](struct.VerifyingKey.html "struct ed25519_dalek::VerifyingKey")
as PKCS#8. Note that \[`pkcs8`\] is re-exported from the toplevel of the
crate:

-   \[`pkcs8::DecodePrivateKey`\]: decode private keys from PKCS#8
-   \[`pkcs8::EncodePrivateKey`\]: encode private keys to PKCS#8
-   \[`pkcs8::DecodePublicKey`\]: decode public keys from PKCS#8
-   \[`pkcs8::EncodePublicKey`\]: encode public keys to PKCS#8

#### <a href="#example-1" class="doc-anchor">§</a>Example

NOTE: this requires the `pem` crate feature.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use ed25519_dalek::{VerifyingKey, pkcs8::DecodePublicKey};

let pem = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAGb9ECWmEzf6FQbrBZ9w7lshQhqowtrbLDFw4rXAxZuE=
-----END PUBLIC KEY-----";

let verifying_key = VerifyingKey::from_public_key_pem(pem)
    .expect("invalid public key PEM");
```

### <a href="#using-serde" class="doc-anchor">§</a>Using Serde

If you prefer the bytes to be wrapped in another serialisation format,
all types additionally come with built-in [serde](https://serde.rs)
support by building `ed25519-dalek` via:

``` language-bash
$ cargo build --features="serde"
```

They can be then serialised into any of the wire formats which serde
supports. For example, using
[bincode](https://github.com/TyOverby/bincode):

``` {.rust .rust-example-rendered}
use bincode::serialize;

let encoded_verifying_key: Vec<u8> = serialize(&verifying_key).unwrap();
let encoded_signature: Vec<u8> = serialize(&signature).unwrap();
```

After sending the `encoded_verifying_key` and `encoded_signature`, the
recipient may deserialise them and verify:

``` {.rust .rust-example-rendered}
use bincode::deserialize;

let message: &[u8] = b"This is a test of the tsunami alert system.";
let decoded_verifying_key: VerifyingKey = deserialize(&encoded_verifying_key).unwrap();
let decoded_signature: Signature = deserialize(&encoded_signature).unwrap();

let verified: bool = decoded_verifying_key.verify(&message, &decoded_signature).is_ok();

assert!(verified);
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../ed25519/index.html" class="mod"
title="mod ed25519"><code>ed25519</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="hazmat/index.html" class="mod"
title="mod ed25519_dalek::hazmat">hazmat</a>  
Low-level interfaces to ed25519 functions

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Signature.html" class="struct"
title="struct ed25519_dalek::Signature">Signature</a>  
Ed25519 signature.

<a href="struct.SigningKey.html" class="struct"
title="struct ed25519_dalek::SigningKey">SigningKey</a>  
ed25519 signing key which can be used to produce signatures.

<a href="struct.StreamVerifier.html" class="struct"
title="struct ed25519_dalek::StreamVerifier">StreamVerifier</a>  
An IUF verifier for ed25519.

<a href="struct.VerifyingKey.html" class="struct"
title="struct ed25519_dalek::VerifyingKey">VerifyingKey</a>  
An ed25519 public key.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.EXPANDED_SECRET_KEY_LENGTH.html" class="constant"
title="constant ed25519_dalek::EXPANDED_SECRET_KEY_LENGTH">EXPANDED_SECRET_KEY_LENGTH</a>  
The length of an “expanded” ed25519 key, `ExpandedSecretKey`, in bytes.

<a href="constant.KEYPAIR_LENGTH.html" class="constant"
title="constant ed25519_dalek::KEYPAIR_LENGTH">KEYPAIR_LENGTH</a>  
The length of an ed25519 `Keypair`, in bytes.

<a href="constant.PUBLIC_KEY_LENGTH.html" class="constant"
title="constant ed25519_dalek::PUBLIC_KEY_LENGTH">PUBLIC_KEY_LENGTH</a>  
The length of an ed25519 `PublicKey`, in bytes.

<a href="constant.SECRET_KEY_LENGTH.html" class="constant"
title="constant ed25519_dalek::SECRET_KEY_LENGTH">SECRET_KEY_LENGTH</a>  
The length of a ed25519 `SecretKey`, in bytes.

<a href="constant.SIGNATURE_LENGTH.html" class="constant"
title="constant ed25519_dalek::SIGNATURE_LENGTH">SIGNATURE_LENGTH</a>  
The length of a ed25519 `Signature`, in bytes.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Signer.html" class="trait"
title="trait ed25519_dalek::Signer">Signer</a>  
Sign the provided message bytestring using `Self` (e.g. a cryptographic
key or connection to an HSM), returning a digital signature.

<a href="trait.Verifier.html" class="trait"
title="trait ed25519_dalek::Verifier">Verifier</a>  
Verify the provided message bytestring using `Self` (e.g. a public key)

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.SecretKey.html" class="type"
title="type ed25519_dalek::SecretKey">SecretKey</a>  
ed25519 secret key as defined in [RFC8032 §
5.1.5](https://www.rfc-editor.org/rfc/rfc8032#section-5.1.5):

<a href="type.SignatureError.html" class="type"
title="type ed25519_dalek::SignatureError">SignatureError</a>  
Errors which may occur while processing signatures and keypairs.
