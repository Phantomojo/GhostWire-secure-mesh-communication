[vodozemac](../index.html)

Module sas Copy item path

<span class="sub-heading"><a href="../../src/vodozemac/sas.rs.html#15-661" class="src">Source</a>
</span>

Expand description

User-friendly key verification using short authentication strings (SAS).

The verification process is heavily inspired by Phil Zimmermann’s
[ZRTP](https://tools.ietf.org/html/rfc6189#section-4.4.1) key agreement
handshake. A core part of key agreement in
[ZRTP](https://tools.ietf.org/html/rfc6189#section-4.4.1) is the *hash
commitment*: the party that begins the key sharing process sends a
*hash* of their part of the Diffie-Hellman exchange but does not send
the part itself exchange until they had received the other party’s part.

The verification process can be used to verify the Ed25519 identity key
of an
[`Account`](../olm/struct.Account.html "struct vodozemac::olm::Account").

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use vodozemac::sas::Sas;
let alice = Sas::new();
let bob = Sas::new();

let bob_public_key = bob.public_key();

let bob = bob.diffie_hellman(alice.public_key())?;
let alice = alice.diffie_hellman(bob_public_key)?;

let alice_bytes = alice.bytes("AGREED_INFO");
let bob_bytes = bob.bytes("AGREED_INFO");

let alice_emojis = alice_bytes.emoji_indices();
let bob_emojis = bob_bytes.emoji_indices();

assert_eq!(alice_emojis, bob_emojis);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.EstablishedSas.html" class="struct"
title="struct vodozemac::sas::EstablishedSas">EstablishedSas</a>  
A struct representing a short auth string verification object where the
shared secret has been established.

<a href="struct.InvalidCount.html" class="struct"
title="struct vodozemac::sas::InvalidCount">InvalidCount</a>  
Error type for the case when we try to generate too many SAS bytes.

<a href="struct.Mac.html" class="struct"
title="struct vodozemac::sas::Mac">Mac</a>  
The output type for the SAS MAC calculation.

<a href="struct.Sas.html" class="struct"
title="struct vodozemac::sas::Sas">Sas</a>  
A struct representing a short auth string verification object.

<a href="struct.SasBytes.html" class="struct"
title="struct vodozemac::sas::SasBytes">SasBytes</a>  
Bytes generated from an shared secret that can be used as the short auth
string.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.SasError.html" class="enum"
title="enum vodozemac::sas::SasError">SasError</a>  
Error type describing failures that can happen during the key
verification.
