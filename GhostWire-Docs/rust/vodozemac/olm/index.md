[vodozemac](../index.html)

Module olm Copy item path

<span class="sub-heading"><a href="../../src/vodozemac/olm/mod.rs.html#16-114"
class="src">Source</a> </span>

Expand description

An implementation of the Olm double ratchet.

## <a href="#overview" class="doc-anchor">§</a>Overview

The core component of the crate is the
[`Account`](struct.Account.html "struct vodozemac::olm::Account"),
representing a single Olm participant. An Olm
[`Account`](struct.Account.html "struct vodozemac::olm::Account")
consists of a collection of key pairs, though often documentation will
shorten this to just “keys”. These are:

1.  An Ed25519 *signing key pair* representing the stable cryptographic
    identity of the participant (the participant’s “fingerprint”).
2.  A Curve25519 *sender key pair* (also sometimes called the *identity
    key pair*, somewhat confusingly).
3.  A number of one-time key pairs.
4.  A current and previous (if any) “fallback” key pair.

While the key in 1 is used for signing but not encryption, the keys in
2-4 participate in a triple Diffie-Hellman key exchange (3DH) with
another Olm participant, thereby establishing an Olm session on each
side of the communication channel. Ultimately, this session is used for
deriving the concrete encryption keys for a particular message.

Olm sessions are represented by the
[`Session`](struct.Session.html "struct vodozemac::olm::Session")
struct. Such a session is created by calling
[`Account::create_outbound_session`](struct.Account.html#method.create_outbound_session "method vodozemac::olm::Account::create_outbound_session")
on one of the participating accounts, passing it the Curve25519 sender
key and one Curve25519 one-time key of the other side. The protocol is
asynchronous, so the participant can start sending messages to the other
side even before the other side has created a session, producing
so-called pre-key messages (see
[`PreKeyMessage`](struct.PreKeyMessage.html "struct vodozemac::olm::PreKeyMessage")).

Once the other participant receives such a pre-key message, they can
create their own matching session by calling
[`Account::create_inbound_session`](struct.Account.html#method.create_inbound_session "method vodozemac::olm::Account::create_inbound_session")
and passing it the pre-key message they received and the Curve25519
sender key of the other side. This completes the establishment of the
Olm communication channel.

``` {.rust .rust-example-rendered}
use anyhow::Result;
use vodozemac::olm::{Account, InboundCreationResult, OlmMessage, SessionConfig};

fn main() -> Result<()> {
    let alice = Account::new();
    let mut bob = Account::new();

    bob.generate_one_time_keys(1);
    let bob_otk = *bob.one_time_keys().values().next().unwrap();

    let mut alice_session = alice
        .create_outbound_session(SessionConfig::version_2(), bob.curve25519_key(), bob_otk);

    bob.mark_keys_as_published();

    let message = "Keep it between us, OK?";
    let alice_msg = alice_session.encrypt(message);

    if let OlmMessage::PreKey(m) = alice_msg.clone() {
        let result = bob.create_inbound_session(alice.curve25519_key(), &m)?;

        let mut bob_session = result.session;
        let what_bob_received = result.plaintext;

        assert_eq!(alice_session.session_id(), bob_session.session_id());

        assert_eq!(message.as_bytes(), what_bob_received);

        let bob_reply = "Yes. Take this, it's dangerous out there!";
        let bob_encrypted_reply = bob_session.encrypt(bob_reply).into();

        let what_alice_received = alice_session
            .decrypt(&bob_encrypted_reply)?;
        assert_eq!(what_alice_received, bob_reply.as_bytes());
    }

    Ok(())
}
```

## <a href="#sending-messages" class="doc-anchor">§</a>Sending messages

To encrypt a message, just call
[`Session::encrypt()`](struct.Session.html#method.encrypt "method vodozemac::olm::Session::encrypt").
This will either produce an
[`OlmMessage::PreKey`](enum.OlmMessage.html#variant.PreKey "variant vodozemac::olm::OlmMessage::PreKey")
or
[`OlmMessage::Normal`](enum.OlmMessage.html#variant.Normal "variant vodozemac::olm::OlmMessage::Normal")
depending on whether the session is fully established. A session is
fully established once you receive (and decrypt) at least one message
from the other side.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Account.html" class="struct"
title="struct vodozemac::olm::Account">Account</a>  
An Olm [`Account`](struct.Account.html "struct vodozemac::olm::Account")
manages all cryptographic keys used on a device.

<a href="struct.AccountPickle.html" class="struct"
title="struct vodozemac::olm::AccountPickle">AccountPickle</a>  
A format suitable for serialization which implements
[`serde::Serialize`](../../serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize")
and
[`serde::Deserialize`](../../serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize").
Obtainable by calling
[`Account::pickle`](struct.Account.html#method.pickle "method vodozemac::olm::Account::pickle").

<a href="struct.IdentityKeys.html" class="struct"
title="struct vodozemac::olm::IdentityKeys">IdentityKeys</a>  
Struct holding the two public identity keys of an
[`Account`](struct.Account.html "struct vodozemac::olm::Account").

<a href="struct.InboundCreationResult.html" class="struct"
title="struct vodozemac::olm::InboundCreationResult">InboundCreationResult</a>  
Return type for the creation of inbound
[`Session`](struct.Session.html "struct vodozemac::olm::Session")
objects.

<a href="struct.Message.html" class="struct"
title="struct vodozemac::olm::Message">Message</a>  
An encrypted Olm message.

<a href="struct.OneTimeKeyGenerationResult.html" class="struct"
title="struct vodozemac::olm::OneTimeKeyGenerationResult">OneTimeKeyGenerationResult</a>  
The result type for the one-time key generation operation.

<a href="struct.PreKeyMessage.html" class="struct"
title="struct vodozemac::olm::PreKeyMessage">PreKeyMessage</a>  
An encrypted Olm pre-key message.

<a href="struct.RatchetPublicKey.html" class="struct"
title="struct vodozemac::olm::RatchetPublicKey">RatchetPublicKey</a>  
The public part of a \[`RatchetKey`\].

<a href="struct.Session.html" class="struct"
title="struct vodozemac::olm::Session">Session</a>  
An Olm session represents one end of an encrypted communication channel
between two participants.

<a href="struct.SessionConfig.html" class="struct"
title="struct vodozemac::olm::SessionConfig">SessionConfig</a>  
A struct to configure how Olm sessions should work under the hood.
Currently only the MAC truncation behaviour can be configured.

<a href="struct.SessionKeys.html" class="struct"
title="struct vodozemac::olm::SessionKeys">SessionKeys</a>  
The set of keys that were used to establish the Olm Session,

<a href="struct.SessionPickle.html" class="struct"
title="struct vodozemac::olm::SessionPickle">SessionPickle</a>  
A format suitable for serialization which implements
[`serde::Serialize`](../../serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize")
and
[`serde::Deserialize`](../../serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize").
Obtainable by calling
[`Session::pickle`](struct.Session.html#method.pickle "method vodozemac::olm::Session::pickle").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DecryptionError.html" class="enum"
title="enum vodozemac::olm::DecryptionError">DecryptionError</a>  
Error type for Olm-based decryption failures.

<a href="enum.MessageType.html" class="enum"
title="enum vodozemac::olm::MessageType">MessageType</a>  
An enum over the two supported message types.

<a href="enum.OlmMessage.html" class="enum"
title="enum vodozemac::olm::OlmMessage">OlmMessage</a>  
Enum over the different Olm message types.

<a href="enum.SessionCreationError.html" class="enum"
title="enum vodozemac::olm::SessionCreationError">SessionCreationError</a>  
Error describing failure modes when creating a Olm
[`Session`](struct.Session.html "struct vodozemac::olm::Session") from
an incoming Olm message.
