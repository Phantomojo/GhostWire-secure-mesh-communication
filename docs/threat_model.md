# GhostWire Threat Model

## Attackers
- State-level censors
- Malicious peers (Sybil, spam, DoS)
- Malicious bridges/federation relays
- Passive eavesdroppers (traffic analysis)

## Assets to Protect
- Message content (confidentiality)
- User identity (anonymity)
- Social graph (metadata privacy)
- Encryption keys (integrity, revocation)
- Node availability (DoS resilience)

## Risks
- Sybil attacks
- Peer discovery spam/DoS
- Key compromise, replay, or revocation failure
- Store-and-forward spam/flooding
- Malicious bridges/federation
- Traffic analysis

## Features by Phase
- **MVP:** Sybil caps, quotas, disaster triggers, local reputation, basic federation trust
- **Phase 2:** Traffic obfuscation, advanced bridge trust, signed score exchange, key revocation propagation
- **Phase 3:** Global reputation, dynamic plugin adapters, mobile/IoT wrappers, advanced cover traffic 