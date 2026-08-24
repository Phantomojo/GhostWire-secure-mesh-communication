# GhostWire Agent Memory
Before making changes, read , , and . Keep this file current with architectural decisions, verified evidence, unresolved risks, and the exact next action.

Current state: transactional transport switching, identity-status compatibility, explicit unavailable peer-control contracts, deterministic route eligibility, deterministic GNN topology indexing, identity/social test-contract corrections, exact message timestamp semantics, mailbox envelope associated-data hardening, and legacy shell-updater removal are implemented or verified. The signed updater is now the only reachable update module; no repository-local  execution remains in .

Verified constraints: production PID 932431 remains on API 9097 / mesh 4001 / QUIC 8081. Isolated instances remain on 9397/4301/8381, 9497/4401/8481, and 9597/4501/8581. The aggregate test suite still requires isolation because an existing network-oriented test run does not complete. Clippy is not clean on the current dirty tree.

Next safe slice: build a red secure-wipe reproducer in an isolated , then implement the real persisted-message deletion path. Do not restart production, regenerate identity, or reset the dirty checkout.
