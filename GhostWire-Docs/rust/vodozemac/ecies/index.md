[vodozemac](../index.html)

Module ecies Copy item path

<span class="sub-heading"><a href="../../src/vodozemac/ecies/mod.rs.html#15-746"
class="src">Source</a> </span>

Expand description

Implementation of an integrated encryption scheme.

This module implements
[ECIES](https://en.wikipedia.org/wiki/Integrated_Encryption_Scheme), the
elliptic curve variant of the Integrated Encryption Scheme. This is a
hybrid encryption scheme, using elliptic curve Diffie-Hellman for shared
secret establishment and a symmetric algorithm for encryption of
individual messages. It is instantiated with X25519 (Curve25519-based
Diffie-Hellman), HMAC-SHA256 as the KDF and ChaCha20-Poly1305 for
symmetric encryption.

ECIES allows a party (the initiator) to establish a communication
channel toward another party (the recipient) given knowledge of only its
public key. We assume that this key was obtained in a secure way. This
implies that the initiator side is able to tell for sure whether there
is an active MITM attack in progress once the channel is established.

On the other hand, the initiator’s key pair is ephemeral and generated
anew for each new channel. This implies the initiator must send their
ephemeral public key to the recipient *unauthenticated* so that the
recipient can complete the channel establishment on its end. From this
it follows that the recipient has no way of knowing who is contacting
them, allowing for active MITM attacks on the recipient side.

In order to close this vector, an out-of-band confirmation is required
to be sent from the initiator device to the recipient device, after
which the channel is considered *secure*. The module provides the
[`CheckCode`](struct.CheckCode.html "struct vodozemac::ecies::CheckCode")
facility which can be used for this purpose.

Throughout this document, we use a naming convention which designates
the device initiating an ECIES channel as device S, while the device on
the other side (towards which the channel is opened) is designated
device G.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use vodozemac::ecies::{Ecies, InboundCreationResult, OutboundCreationResult};

let plaintext = b"It's a secret to everybody";

let alice = Ecies::new();
let bob = Ecies::new();

let OutboundCreationResult { ecies: mut alice, message } = alice
    .establish_outbound_channel(bob.public_key(), plaintext)?;

let InboundCreationResult { mut ecies, message } = bob
    .establish_inbound_channel(&message)
    .expect("We should be able to create an inbound channel");

assert_eq!(
    message, plaintext,
    "The decrypted plaintext should match our initial plaintext"
);

// We now exchange the check code out-of-band and compare it.
if alice.check_code() != ecies.check_code() {
    panic!("The check code must match; possible active MITM attack in progress");
}

let message = ecies.encrypt(b"Another plaintext");
let decrypted = alice.decrypt(&message)?;

assert_eq!(decrypted, b"Another plaintext");
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CheckCode.html" class="struct"
title="struct vodozemac::ecies::CheckCode">CheckCode</a>  
A check code that can be used to confirm that two
[`EstablishedEcies`](struct.EstablishedEcies.html "struct vodozemac::ecies::EstablishedEcies")
objects share the same secret. This is supposed to be shared out-of-band
to protect against active MITM attacks.

<a href="struct.Ecies.html" class="struct"
title="struct vodozemac::ecies::Ecies">Ecies</a>  
An unestablished ECIES session.

<a href="struct.EstablishedEcies.html" class="struct"
title="struct vodozemac::ecies::EstablishedEcies">EstablishedEcies</a>  
An established ECIES session.

<a href="struct.InboundCreationResult.html" class="struct"
title="struct vodozemac::ecies::InboundCreationResult">InboundCreationResult</a>  
The result of an inbound ECIES channel establishment.

<a href="struct.InitialMessage.html" class="struct"
title="struct vodozemac::ecies::InitialMessage">InitialMessage</a>  
The initial message, sent by the ECIES channel establisher.

<a href="struct.Message.html" class="struct"
title="struct vodozemac::ecies::Message">Message</a>  
An encrypted message a
[`EstablishedEcies`](struct.EstablishedEcies.html "struct vodozemac::ecies::EstablishedEcies")
channel has sent.

<a href="struct.OutboundCreationResult.html" class="struct"
title="struct vodozemac::ecies::OutboundCreationResult">OutboundCreationResult</a>  
The result of an outbound ECIES channel establishment.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum vodozemac::ecies::Error">Error</a>  
The Error type for the ECIES submodule.

<a href="enum.MessageDecodeError.html" class="enum"
title="enum vodozemac::ecies::MessageDecodeError">MessageDecodeError</a>  
The error type for the ECIES message decoding failures.
