[ghostwire](../index.html)

Module genesis\_constants Copy item path

<span class="sub-heading"><a href="../../src/ghostwire/genesis_constants.rs.html#1-321"
class="src">Source</a> </span>

Expand description

genesis\_constants.rs — Ghostwire network identity constants

Copyright (C) 2025-2026 Michael Irungu SPDX-License-Identifier:
AGPL-3.0-or-later

These constants were derived in a two-party commit-reveal ceremony
between the protocol author and an external party on 2026-03-17.

Derivation: genesis\_key = BLAKE3(“ghostwire genesis ceremony v1” \|\|
R\_mike \|\| R\_claude)

where R\_mike and R\_claude were independently generated entropy values,
each committed to via BLAKE3 hash before reveal.

The genesis\_key itself no longer exists. Neither input can be
reproduced. These derived constants are the only artifact of the
ceremony.

WHAT THIS MEANS:

-   A node compiled without these exact constants speaks a different
    protocol and is rejected at handshake.
-   Any attempt to reconstruct the ceremony inputs fails because one
    input is permanently gone (AI amnesia is architectural, not a
    promise).
-   The string domain labels that previously appeared in plaintext in
    the source code (“ghostwire sphinx v1” etc.) no longer exist. A
    reverse engineer finds opaque 32-byte arrays with no string context.
-   No single party holds the derivation path. This is not a secret key
    that can be seized, subpoenaed, or coerced out of anyone.
-   The network is blackboxed: even the original architect cannot
    reproduce these constants without the other party’s entropy. A new
    ceremony requires explicit authorization from the current protocol
    maintainers — reproduction without consent is null and void.

ALL 16 DOMAIN CONSTANTS ARE ACTIVE: Every domain constant defined below
is consumed by at least one production module. The
all\_domains\_are\_consumed() test verifies that no dead constants
accumulate. See the test for the complete mapping of domain → consumer.

DO NOT MODIFY THESE CONSTANTS. Changing any byte breaks compatibility
with every existing Ghostwire node. If a protocol upgrade is needed,
derive new constants via a new ceremony and bump the protocol version.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="fn.genesis_derive.html" class="fn"
title="fn ghostwire::genesis_constants::genesis_derive"><code>genesis_derive</code></a>` as derive_with_domain;`

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.CHANNEL_INVITE_PREFIX.html" class="constant"
title="constant ghostwire::genesis_constants::CHANNEL_INVITE_PREFIX">CHANNEL_INVITE_PREFIX</a>  
Ghost Channel invite prefix — replaces “gwc1:”

<a href="constant.CHANNEL_KDF_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::CHANNEL_KDF_DOMAIN">CHANNEL_KDF_DOMAIN</a>  
Ghost Channel PoW KDF domain — replaces “ghostwire channel pow v1”

<a href="constant.DHT_GENESIS_HASH.html" class="constant"
title="constant ghostwire::genesis_constants::DHT_GENESIS_HASH">DHT_GENESIS_HASH</a>  
DHT genesis bootstrap value — first trusted entry in the routing table

<a href="constant.FRAME_MAGIC.html" class="constant"
title="constant ghostwire::genesis_constants::FRAME_MAGIC">FRAME_MAGIC</a>  
Wire frame magic bytes — first 4 bytes of every Ghostwire frame.
Replaces b“GWF\x01“. Any frame not starting with these bytes is dropped.

<a href="constant.FRAME_VERSION.html" class="constant"
title="constant ghostwire::genesis_constants::FRAME_VERSION">FRAME_VERSION</a>  
Protocol frame version identifier

<a href="constant.HANDSHAKE_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::HANDSHAKE_DOMAIN">HANDSHAKE_DOMAIN</a>  
Handshake domain separator — used in version negotiation

<a href="constant.MAILBOX_KDF_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::MAILBOX_KDF_DOMAIN">MAILBOX_KDF_DOMAIN</a>  
Mailbox encryption domain — replaces “ghostwire mailbox v1”

<a href="constant.MIXING_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::MIXING_DOMAIN">MIXING_DOMAIN</a>  
Mixing delay seed domain — replaces “ghostwire mixing domain v1”

<a href="constant.ONION_HOP_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::ONION_HOP_DOMAIN">ONION_HOP_DOMAIN</a>  
Onion hop encryption domain — replaces “ghostwire onion hop v1”

<a href="constant.PAIRING_SESSION_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::PAIRING_SESSION_DOMAIN">PAIRING_SESSION_DOMAIN</a>  
Pairing session domain — replaces “ghostwire pairing session v1”

<a href="constant.POW_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::POW_DOMAIN">POW_DOMAIN</a>  
S/Kademlia NodeId PoW domain — replaces “ghostwire pow domain v1”

<a href="constant.RECOVERY_KDF_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::RECOVERY_KDF_DOMAIN">RECOVERY_KDF_DOMAIN</a>  
Recovery share encryption domain — replaces “ghostwire recovery share
v1”

<a href="constant.RENDEZVOUS_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::RENDEZVOUS_DOMAIN">RENDEZVOUS_DOMAIN</a>  
Rendezvous address domain — replaces “ghostwire rendezvous v1”

<a href="constant.SEALED_KDF_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::SEALED_KDF_DOMAIN">SEALED_KDF_DOMAIN</a>  
ECIES sealed message KDF domain — replaces “ghostwire sealed v1”

<a href="constant.SPHINX_ALPHA_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::SPHINX_ALPHA_DOMAIN">SPHINX_ALPHA_DOMAIN</a>  
Sphinx alpha re-randomisation domain — replaces “ghostwire sphinx alpha
v1”

<a href="constant.SPHINX_BLIND_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::SPHINX_BLIND_DOMAIN">SPHINX_BLIND_DOMAIN</a>  
Sphinx alpha blinding domain — replaces “ghostwire sphinx blind v1”

<a href="constant.SPHINX_KDF_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::SPHINX_KDF_DOMAIN">SPHINX_KDF_DOMAIN</a>  
Sphinx per-hop KDF domain — replaces “ghostwire sphinx v1” Used in
HopKeys::derive() for header/payload/mac/blind key derivation.

<a href="constant.SPHINX_MAC_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::SPHINX_MAC_DOMAIN">SPHINX_MAC_DOMAIN</a>  
Sphinx MAC computation domain — replaces “ghostwire sphinx mac v1”

<a href="constant.SPHINX_SHARED_DOMAIN.html" class="constant"
title="constant ghostwire::genesis_constants::SPHINX_SHARED_DOMAIN">SPHINX_SHARED_DOMAIN</a>  
Sphinx shared secret derivation domain — replaces “ghostwire sphinx
shared v1”

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.genesis_derive.html" class="fn"
title="fn ghostwire::genesis_constants::genesis_derive">genesis_derive</a>
