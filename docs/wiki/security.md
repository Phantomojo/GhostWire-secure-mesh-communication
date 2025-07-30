# Security & Privacy

GhostWire is built with security and privacy as core principles. This page details the security architecture, features, and best practices.

---

## Encryption
- **End-to-End Encryption:** All messages are encrypted using industry-standard cryptography (AES-256-GCM, X25519, HMAC-SHA256).
- **Perfect Forward Secrecy:** Ephemeral keys and session rotation.
- **Hybrid Crypto:** Support for post-quantum algorithms (planned).

---

## Key Management
- **Ed25519/X25519:** For identity and session keys.
- **Ephemeral Identities:** Temporary keys for privacy.
- **Key Rotation:** Automated and manual rotation supported.
- **Secure Storage:** Keys stored encrypted on disk or in secure enclaves.

---

## Trust & Defense Modules
- **Sybil Defense:** Prevents fake node attacks using reputation, proof-of-work, and social trust.
- **Quotas:** Rate limits to prevent spam and abuse.
- **Blacklists:** Block malicious nodes or addresses.
- **Disaster Triggers:** Emergency wipe, panic button, and rapid shutdown.
- **Reputation:** Nodes earn trust over time; bad actors are penalized.
- **Federation:** Multiple networks can interconnect with trust boundaries.
- **Traffic Obfuscation:** Cover traffic, timing randomization, and packet padding to resist surveillance.

---

## Threat Model
- **Adversaries:** Censors, eavesdroppers, Sybil attackers, spammers, malicious insiders.
- **Goals:** Confidentiality, integrity, availability, plausible deniability.
- **Mitigations:** Strong crypto, modular trust, rapid response to threats.

---

## Best Practices
- Always use the latest release.
- Rotate keys regularly.
- Enable all security modules.
- Report vulnerabilities via [GitHub Security](https://github.com/phantomojo/GhostWire-secure-mesh-communication/security/advisories). 