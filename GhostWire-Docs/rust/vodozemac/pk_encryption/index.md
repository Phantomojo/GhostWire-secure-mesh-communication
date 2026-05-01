[vodozemac](../index.html)

Module pk\_encryption Copy item path

<span class="sub-heading"><a href="../../src/vodozemac/pk_encryption.rs.html#15-478"
class="src">Source</a> </span>

Expand description

☣️ Compat support for libolm’s PkEncryption and PkDecryption

This implements the `m.megolm_backup.v1.curve25519-aes-sha2` described
in the Matrix
[spec](https://spec.matrix.org/v1.11/client-server-api/#backup-algorithm-mmegolm_backupv1curve25519-aes-sha2).
This is a hybrid encryption scheme utilizing Curve25519 and AES-CBC.
X25519 ECDH is performed between an ephemeral key pair and a long-lived
backup key pair to establish a shared secret, from which symmetric
encryption and message authentication (MAC) keys are derived.

**WARNING**: Please note the algorithm contains a critical flaw and does
not provide authentication of the ciphertext.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use anyhow::Result;
use vodozemac::pk_encryption::{PkDecryption, PkEncryption};

fn main() -> Result<()> {
    let plaintext = b"It's a secret to everybody";

    let decryption = PkDecryption::new();
    let encryption = PkEncryption::from_key(decryption.public_key());

    let message = encryption.encrypt(plaintext);
    let decrypted = decryption.decrypt(&message)?;

    assert_eq!(decrypted.as_slice(), plaintext);

    Ok(())
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Message.html" class="struct"
title="struct vodozemac::pk_encryption::Message">Message</a>  
A message that was encrypted using a
[`PkEncryption`](struct.PkEncryption.html "struct vodozemac::pk_encryption::PkEncryption")
object.

<a href="struct.PkDecryption.html" class="struct"
title="struct vodozemac::pk_encryption::PkDecryption">PkDecryption</a>  
The decryption component of the PkEncryption support.

<a href="struct.PkEncryption.html" class="struct"
title="struct vodozemac::pk_encryption::PkEncryption">PkEncryption</a>  
The encryption component of PkEncryption support.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum vodozemac::pk_encryption::Error">Error</a>  
An error type describing failures which can happen during the decryption
step.

<a href="enum.MessageDecodeError.html" class="enum"
title="enum vodozemac::pk_encryption::MessageDecodeError">MessageDecodeError</a>  
An error type describing failures which can happen during the decoding
of an encrypted
[`Message`](struct.Message.html "struct vodozemac::pk_encryption::Message").
