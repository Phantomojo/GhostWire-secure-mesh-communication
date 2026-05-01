Crate vodozemac Copy item path

<span class="sub-heading"><a href="../src/vodozemac/lib.rs.html#15-353" class="src">Source</a>
</span>

Expand description

A Rust implementation of Olm and Megolm

vodozemac is a Rust reimplementation of
[libolm](https://gitlab.matrix.org/matrix-org/olm), a cryptographic
library used for end-to-end encryption in [Matrix](https://matrix.org).
At its core, it is an implementation of the Olm and Megolm cryptographic
ratchets, along with a high-level API to easily establish cryptographic
communication channels employing those ratchets with other parties. It
also implements some other miscellaneous cryptographic functionality
which is useful for building Matrix clients, such as
[SAS](https://spec.matrix.org/v1.2/client-server-api/#short-authentication-string-sas-verification).

# <a href="#olm" class="doc-anchor">§</a>Olm

Olm is an implementation of the [Double Ratchet
algorithm](https://whispersystems.org/docs/specifications/doubleratchet/),
very similar to that employed by the Signal Protocol. It allows the
establishment of a 1-to-1 private communication channel, with perfect
forward secrecy and self-healing properties.

A detailed technical specification can be found at
<https://gitlab.matrix.org/matrix-org/olm/-/blob/master/docs/olm.md>.

For more information on using vodozemac for Olm, see the
[`olm`](olm/index.html "mod vodozemac::olm") module.

# <a href="#megolm" class="doc-anchor">§</a>Megolm

Megolm is an AES-based single ratchet for group conversations with a
large number of participants, where using Olm would be cost prohibitive
because it would imply encrypting each message individually for each
participant. Megolm sidesteps this by encrypting messages with a
symmetric ratchet, shared once with each participant and then reused for
a sequence of messages before rotating.

This is a trade-off in which we lose Olm’s self-healing properties,
because someone in possession of a Megolm session at a particular state
can derive all future states. However, if the attacker is only able to
obtain the session in a ratcheted state, they cannot use it to decrypt
messages encrypted with an earlier state. This preserves forward
secrecy.

A detailed technical specification can be found at
<https://gitlab.matrix.org/matrix-org/olm/-/blob/master/docs/megolm.md>.

For more information on using vodozemac for Megolm, see the
[`megolm`](megolm/index.html "mod vodozemac::megolm") module.

# <a href="#features" class="doc-anchor">§</a>Features

## <a href="#supported" class="doc-anchor">§</a>Supported

-   [Olm](https://matrix-org.github.io/vodozemac/vodozemac/olm/index.html)
-   [Megolm](https://matrix-org.github.io/vodozemac/vodozemac/megolm/index.html)
-   [libolm pickle format](#legacy-pickles) (read-only)
-   [Modern pickle format](#modern-pickles)
-   [SAS (Short Authentication
    Strings)](https://matrix-org.github.io/vodozemac/vodozemac/sas/index.html)

## <a href="#unsupported" class="doc-anchor">§</a>Unsupported

-   Creating asymmetric [server-side message key
    backups](https://spec.matrix.org/v1.2/client-server-api/#server-side-key-backups),
    since they are slated to be replaced with symmetric backups.

## <a href="#planned" class="doc-anchor">§</a>Planned

-   Symmetric [server-side message key
    backups](https://github.com/uhoreg/matrix-doc/blob/symmetric-backups/proposals/3270-symmetric-megolm-backup.md)
-   Importing asymmetric [server-side message key
    backups](https://spec.matrix.org/v1.2/client-server-api/#server-side-key-backups),
    for compatibility with existing backups created by libolm.

# <a href="#feature-flags" class="doc-anchor">§</a>Feature flags

## <a href="#low-level-api" class="doc-anchor">§</a>Low-level API

Feature: `low-level-api` (default: off)

Vodozemac exposes some lower-level structs and functions that are only
useful in very advanced use cases. These should *not* be needed by the
vast majority of users.

Extreme care must be taken when using such APIs, as incorrect usage can
lead to broken sessions.

# <a href="#pickling" class="doc-anchor">§</a>Pickling

vodozemac supports serializing its entire internal state into a form a
“pickle”. The state can subsequently be restored from such a pickle
(“unpickled”) in order to continue operation. This is used to support
some Matrix features like device dehydration.

## <a href="#legacy-pickles" class="doc-anchor">§</a>Legacy pickles

The legacy pickle format is a simple binary format used by libolm.
Implemented for interoperability with current clients which are using
libolm. Only *unpickling* is supported.

## <a href="#modern-pickles" class="doc-anchor">§</a>Modern pickles

The crate also implements a modern pickling mechanism using
[Serde](https://serde.rs/). The exact serialization format is not
mandated nor specified by this crate, but you can serialize to and
deserialize from any format supported by Serde.

The following structs support pickling:

-   [`olm::Account`](olm/struct.Account.html "struct vodozemac::olm::Account")
-   [`olm::Session`](olm/struct.Session.html "struct vodozemac::olm::Session")
-   [`megolm::GroupSession`](megolm/struct.GroupSession.html "struct vodozemac::megolm::GroupSession")
-   [`megolm::InboundGroupSession`](megolm/struct.InboundGroupSession.html "struct vodozemac::megolm::InboundGroupSession")

For example, the following will print out the JSON representing the
serialized `Account` and will leave no new copies of the account’s
secrets in memory:

``` {.rust .rust-example-rendered}
use anyhow::Result;
use vodozemac::olm::{Account, AccountPickle};

const PICKLE_KEY: [u8; 32] = [0u8; 32];

fn main() -> Result<()>{
    let mut account = Account::new();

    account.generate_one_time_keys(10);
    account.generate_fallback_key();

    let pickle = account.pickle().encrypt(&PICKLE_KEY);

    let account2: Account = AccountPickle::from_encrypted(&pickle, &PICKLE_KEY)?.into();

    assert_eq!(account.identity_keys(), account2.identity_keys());

    Ok(())
}
```

You can unpickle a pickle-able struct directly from its serialized form:

``` {.rust .rust-example-rendered}
    let mut json_str = serde_json::to_string(&some_account.pickle())?;
    // This will produce an account which is identical to `some_account`.
    let account: Account = serde_json::from_str::<AccountPickle>(&json_str)?.into();

    json_str.zeroize();
```

However, the pickle-able structs do not implement `serde::Serialize`
themselves. If you want to serialize to a format other than JSON, you
should instead call the `.pickle()` method to obtain a special
serializable struct. This struct *does* implement `Serialize` and can
therefore be serialized into any format supported by `serde`. To get
back to the original struct from such as serializable struct, just call
`.unpickle()`.

``` {.rust .rust-example-rendered}
use anyhow::Result;
use vodozemac::olm::Account;

fn main() -> Result<()> {
    let account = Account::new();
    let account: Account = account.pickle().into();  // this is identity

    Ok(())
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="ecies/index.html" class="mod"
title="mod vodozemac::ecies">ecies</a>  
Implementation of an integrated encryption scheme.

<a href="megolm/index.html" class="mod"
title="mod vodozemac::megolm">megolm</a>  
An implementation of the Megolm ratchet.

<a href="olm/index.html" class="mod" title="mod vodozemac::olm">olm</a>  
An implementation of the Olm double ratchet.

<a href="pk_encryption/index.html" class="mod"
title="mod vodozemac::pk_encryption">pk_encryption</a>  
☣️ Compat support for libolm’s PkEncryption and PkDecryption

<a href="sas/index.html" class="mod" title="mod vodozemac::sas">sas</a>  
User-friendly key verification using short authentication strings (SAS).

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Curve25519PublicKey.html" class="struct"
title="struct vodozemac::Curve25519PublicKey">Curve25519PublicKey</a>  
Struct representing a Curve25519 public key.

<a href="struct.Curve25519SecretKey.html" class="struct"
title="struct vodozemac::Curve25519SecretKey">Curve25519SecretKey</a>  
Struct representing a Curve25519 secret key.

<a href="struct.Ed25519Keypair.html" class="struct"
title="struct vodozemac::Ed25519Keypair">Ed25519Keypair</a>  
A struct collecting both a public, and a secret, Ed25519 key.

<a href="struct.Ed25519PublicKey.html" class="struct"
title="struct vodozemac::Ed25519PublicKey">Ed25519PublicKey</a>  
An Ed25519 public key, used to verify digital signatures.

<a href="struct.Ed25519SecretKey.html" class="struct"
title="struct vodozemac::Ed25519SecretKey">Ed25519SecretKey</a>  
An Ed25519 secret key, used to create digital signatures.

<a href="struct.Ed25519Signature.html" class="struct"
title="struct vodozemac::Ed25519Signature">Ed25519Signature</a>  
An Ed25519 digital signature, can be used to verify the authenticity of
a message.

<a href="struct.KeyId.html" class="struct"
title="struct vodozemac::KeyId">KeyId</a>  
A unique identifier for a one-time
[`Curve25519PublicKey`](struct.Curve25519PublicKey.html "struct vodozemac::Curve25519PublicKey").

<a href="struct.ProtoBufDecodeError.html" class="struct"
title="struct vodozemac::ProtoBufDecodeError">ProtoBufDecodeError</a>  
A Protobuf message decoding error.

<a href="struct.SharedSecret.html" class="struct"
title="struct vodozemac::SharedSecret">SharedSecret</a>  
The result of a Diffie-Hellman key exchange.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Base64DecodeError.html" class="enum"
title="enum vodozemac::Base64DecodeError">Base64DecodeError</a>  
Errors that can occur while decoding.

<a href="enum.DecodeError.html" class="enum"
title="enum vodozemac::DecodeError">DecodeError</a>  
Error type describing the different ways message decoding can fail.

<a href="enum.DehydratedDeviceError.html" class="enum"
title="enum vodozemac::DehydratedDeviceError">DehydratedDeviceError</a>  
Error type describing the various ways dehydrated devices can fail to be
decoded.

<a href="enum.KeyError.html" class="enum"
title="enum vodozemac::KeyError">KeyError</a>  
Error type for failures that may occur when decoding or using a
cryptographic key.

<a href="enum.LibolmPickleError.html" class="enum"
title="enum vodozemac::LibolmPickleError">LibolmPickleError</a>  
Error type describing the various ways libolm pickles can fail to be
decoded.

<a href="enum.PickleError.html" class="enum"
title="enum vodozemac::PickleError">PickleError</a>  
Error type describing the various ways Vodozemac pickles can fail to be
decoded.

<a href="enum.SignatureError.html" class="enum"
title="enum vodozemac::SignatureError">SignatureError</a>  
Error type describing signature verification failures.

# Statics<a href="#statics" class="anchor">§</a>

<a href="static.VERSION.html" class="static"
title="static vodozemac::VERSION">VERSION</a>  
The version of vodozemac that is being used.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.base64_decode.html" class="fn"
title="fn vodozemac::base64_decode">base64_decode</a>  
Decode the input as base64 with no padding.

<a href="fn.base64_encode.html" class="fn"
title="fn vodozemac::base64_encode">base64_encode</a>  
Encode the input as base64 with no padding.
