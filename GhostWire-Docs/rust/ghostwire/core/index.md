[ghostwire](../index.html)

Module core Copy item path

<span class="sub-heading"><a href="../../src/ghostwire/core/mod.rs.html#3-1863"
class="src">Source</a> </span>

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="security_v2/index.html" class="mod"
title="mod ghostwire::core::security_v2"><code>security_v2</code></a>` as security;`

`pub use encryption::`<a href="encryption/type.Encryption.html" class="type"
title="type ghostwire::core::encryption::Encryption"><code>Encryption</code></a>`;`

`pub use identity::`<a href="identity/core_identity/struct.Identity.html" class="struct"
title="struct ghostwire::core::identity::core_identity::Identity"><code>Identity</code></a>`;`

`pub use message::`<a href="message/struct.Message.html" class="struct"
title="struct ghostwire::core::message::Message"><code>Message</code></a>`;`

`pub use security::`<a href="security_v2/struct.AuditLogger.html" class="struct"
title="struct ghostwire::core::security_v2::AuditLogger"><code>AuditLogger</code></a>`;`

`pub use security::`<a href="security_v2/struct.FailedAttemptTracker.html" class="struct"
title="struct ghostwire::core::security_v2::FailedAttemptTracker"><code>FailedAttemptTracker</code></a>`;`

`pub use security::`<a href="security_v2/struct.SecurityAuditRecord.html" class="struct"
title="struct ghostwire::core::security_v2::SecurityAuditRecord"><code>SecurityAuditRecord</code></a>`;`

`pub use security::`<a href="security_v2/struct.SecurityConfig.html" class="struct"
title="struct ghostwire::core::security_v2::SecurityConfig"><code>SecurityConfig</code></a>`;`

`pub use security::`<a href="security_v2/struct.SecurityManager.html" class="struct"
title="struct ghostwire::core::security_v2::SecurityManager"><code>SecurityManager</code></a>`;`

`pub use security::`<a href="security_v2/struct.SecurityStats.html" class="struct"
title="struct ghostwire::core::security_v2::SecurityStats"><code>SecurityStats</code></a>`;`

`pub use mesh::`<a href="mesh/struct.MeshManager.html" class="struct"
title="struct ghostwire::core::mesh::MeshManager"><code>MeshManager</code></a>`;`

`pub use mesh::`<a href="mesh/struct.MeshNode.html" class="struct"
title="struct ghostwire::core::mesh::MeshNode"><code>MeshNode</code></a>`;`

`pub use mesh::`<a href="mesh/struct.MeshStats.html" class="struct"
title="struct ghostwire::core::mesh::MeshStats"><code>MeshStats</code></a>`;`

`pub use mesh::`<a href="mesh/struct.MeshTopology.html" class="struct"
title="struct ghostwire::core::mesh::MeshTopology"><code>MeshTopology</code></a>`;`

`pub use reticulum::`<a href="reticulum/struct.ReticulumManager.html" class="struct"
title="struct ghostwire::core::reticulum::ReticulumManager"><code>ReticulumManager</code></a>`;`

`pub use reticulum::`<a href="reticulum/struct.ReticulumStats.html" class="struct"
title="struct ghostwire::core::reticulum::ReticulumStats"><code>ReticulumStats</code></a>`;`

`pub use reticulum::`<a href="reticulum/struct.ReticulumTopology.html" class="struct"
title="struct ghostwire::core::reticulum::ReticulumTopology"><code>ReticulumTopology</code></a>`;`

`pub use stealth_tcp::`<a href="stealth_tcp/struct.ConnectionStats.html" class="struct"
title="struct ghostwire::core::stealth_tcp::ConnectionStats"><code>ConnectionStats</code></a>`;`

`pub use stealth_tcp::`<a href="stealth_tcp/struct.StealthTCPProvider.html" class="struct"
title="struct ghostwire::core::stealth_tcp::StealthTCPProvider"><code>StealthTCPProvider</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="ai/index.html" class="mod"
title="mod ghostwire::core::ai">ai</a>  
<a href="armor/index.html" class="mod"
title="mod ghostwire::core::armor">armor</a>  
<a href="binary_hardening/index.html" class="mod"
title="mod ghostwire::core::binary_hardening">binary_hardening</a>  
binary\_hardening.rs — Anti-reverse-engineering measures

<a href="blockstore/index.html" class="mod"
title="mod ghostwire::core::blockstore">blockstore</a>  
blockstore.rs — Content-addressed block storage with Bitswap-inspired
exchange

<a href="bluetooth/index.html" class="mod"
title="mod ghostwire::core::bluetooth">bluetooth</a>  
<a href="briar/index.html" class="mod"
title="mod ghostwire::core::briar">briar</a>  
<a href="channels/index.html" class="mod"
title="mod ghostwire::core::channels">channels</a>  
<a href="constants/index.html" class="mod"
title="mod ghostwire::core::constants">constants</a>  
GhostWire Core Constants

<a href="crypto_machine/index.html" class="mod"
title="mod ghostwire::core::crypto_machine">crypto_machine</a>  
crypto\_machine.rs — Cryptographic state machine using vodozemac

<a href="crypto_processor/index.html" class="mod"
title="mod ghostwire::core::crypto_processor">crypto_processor</a>  
CryptoProcessor: The I/O driver for the Sans I/O Crypto Machine. This
component handles side effects: networking, storage, and timers.

<a href="db/index.html" class="mod"
title="mod ghostwire::core::db">db</a>  
<a href="dht_signatures/index.html" class="mod"
title="mod ghostwire::core::dht_signatures">dht_signatures</a>  
dht\_signatures.rs — Ed25519 signatures on DHT records

<a href="disaster/index.html" class="mod"
title="mod ghostwire::core::disaster">disaster</a>  
<a href="dtn/index.html" class="mod"
title="mod ghostwire::core::dtn">dtn</a>  
<a href="encrypted_store/index.html" class="mod"
title="mod ghostwire::core::encrypted_store">encrypted_store</a>  
encrypted\_store.rs — Encrypted, authenticated key-value storage

<a href="encryption/index.html" class="mod"
title="mod ghostwire::core::encryption">encryption</a>  
Encryption: key revocation, rotation announcements, CRLs. Next:
integrate with key manager and web API.

<a href="identity/index.html" class="mod"
title="mod ghostwire::core::identity">identity</a>  
<a href="identity_pairing/index.html" class="mod"
title="mod ghostwire::core::identity_pairing">identity_pairing</a>  
<a href="knowledge/index.html" class="mod"
title="mod ghostwire::core::knowledge">knowledge</a>  
<a href="mailbox/index.html" class="mod"
title="mod ghostwire::core::mailbox">mailbox</a>  
mailbox.rs — Store-and-forward delivery for offline peers

<a href="mesh/index.html" class="mod"
title="mod ghostwire::core::mesh">mesh</a>  
<a href="mesh_health/index.html" class="mod"
title="mod ghostwire::core::mesh_health">mesh_health</a>  
mesh\_health.rs — Live mesh health dashboard

<a href="message/index.html" class="mod"
title="mod ghostwire::core::message">message</a>  
Message Module - Core Communication Structure

<a href="micro_optimizations/index.html" class="mod"
title="mod ghostwire::core::micro_optimizations">micro_optimizations</a>  
micro\_optimizations.rs — Performance improvements for the core hot
paths

<a href="monitor/index.html" class="mod"
title="mod ghostwire::core::monitor">monitor</a>  
<a href="network/index.html" class="mod"
title="mod ghostwire::core::network">network</a>  
<a href="obfs4_transport/index.html" class="mod"
title="mod ghostwire::core::obfs4_transport">obfs4_transport</a>  
Obfs4 Transport Helper for GhostWire

<a href="oem_battery_guide/index.html" class="mod"
title="mod ghostwire::core::oem_battery_guide">oem_battery_guide</a>  
oem\_battery\_guide.rs — Per-OEM battery optimization guidance

<a href="persistence_manager/index.html" class="mod"
title="mod ghostwire::core::persistence_manager">persistence_manager</a>  
<a href="qr/index.html" class="mod"
title="mod ghostwire::core::qr">qr</a>  
<a href="recovery_shamir/index.html" class="mod"
title="mod ghostwire::core::recovery_shamir">recovery_shamir</a>  
recovery.rs — Shamir Secret Sharing for identity key recovery

<a href="reputation/index.html" class="mod"
title="mod ghostwire::core::reputation">reputation</a>  
reputation.rs — Web of trust and TOFU key verification

<a href="reticulum/index.html" class="mod"
title="mod ghostwire::core::reticulum">reticulum</a>  
<a href="routing/index.html" class="mod"
title="mod ghostwire::core::routing">routing</a>  
<a href="security_v2/index.html" class="mod"
title="mod ghostwire::core::security_v2">security_v2</a>  
Security module: Sybil defense, quotas, blacklists. Next: integrate with
peer discovery, store, and reputation.

<a href="sim_transport/index.html" class="mod"
title="mod ghostwire::core::sim_transport">sim_transport</a>  
<a href="social/index.html" class="mod"
title="mod ghostwire::core::social">social</a>  
<a href="stealth_tcp/index.html" class="mod"
title="mod ghostwire::core::stealth_tcp">stealth_tcp</a>  
<a href="store/index.html" class="mod"
title="mod ghostwire::core::store">store</a>  
Secure message storage with encryption at rest.

<a href="traffic_armor/index.html" class="mod"
title="mod ghostwire::core::traffic_armor">traffic_armor</a>  
traffic\_armor.rs — Traffic analysis resistance

<a href="transport/index.html" class="mod"
title="mod ghostwire::core::transport">transport</a>  
<a href="transport_ext/index.html" class="mod"
title="mod ghostwire::core::transport_ext">transport_ext</a>  
<a href="transport_manager/index.html" class="mod"
title="mod ghostwire::core::transport_manager">transport_manager</a>  
<a href="transport_pluggable/index.html" class="mod"
title="mod ghostwire::core::transport_pluggable">transport_pluggable</a>  
<a href="tutorial/index.html" class="mod"
title="mod ghostwire::core::tutorial">tutorial</a>  
<a href="typed_crypto/index.html" class="mod"
title="mod ghostwire::core::typed_crypto">typed_crypto</a>  
typed\_crypto.rs — Veilid-inspired typed cryptographic keys

<a href="updater/index.html" class="mod"
title="mod ghostwire::core::updater">updater</a>  
<a href="updater_fixed/index.html" class="mod"
title="mod ghostwire::core::updater_fixed">updater_fixed</a>  
updater\_fixed.rs — Secure updater replacing the RCE-vulnerable shell
script caller

<a href="username_discovery/index.html" class="mod"
title="mod ghostwire::core::username_discovery">username_discovery</a>  
DHT-based Username Discovery System Allows nodes to publish and lookup
username → PeerID mappings across the internet

<a href="wire_protocol/index.html" class="mod"
title="mod ghostwire::core::wire_protocol">wire_protocol</a>  
wire\_protocol.rs — Compact binary wire format for constrained networks

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AdapterRegistry.html" class="struct"
title="struct ghostwire::core::AdapterRegistry">AdapterRegistry</a>

<a href="struct.Core.html" class="struct"
title="struct ghostwire::core::Core">Core</a>

Core application state and management

<a href="struct.MatrixAdapter.html" class="struct"
title="struct ghostwire::core::MatrixAdapter">MatrixAdapter</a>

<a href="struct.MeshtasticAdapter.html" class="struct"
title="struct ghostwire::core::MeshtasticAdapter">MeshtasticAdapter</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AdapterKind.html" class="enum"
title="enum ghostwire::core::AdapterKind">AdapterKind</a>  
<a href="enum.CoreEvent.html" class="enum"
title="enum ghostwire::core::CoreEvent">CoreEvent</a>  
System-wide events for real-time reactivity
