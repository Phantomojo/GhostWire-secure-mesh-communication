# GhostWire Project Doctrine

This file is part of the canonical GhostWire codebase. It is mandatory reading for any agent or contributor making architectural, security, transport, identity, API, UI, storage, or intelligence changes.

## Mission

GhostWire is a privacy-preserving, resilient hybrid mesh communication platform. The objective is not to maximize the number of visible features. The objective is to make every exposed capability truthful, bounded, encrypted where required, privacy-minimized, operationally reversible, and understandable from the cockpit.

> Simple outside, sophisticated inside. The user should not need to understand the fuselage to fly the rocket, but the fuselage must never pretend that an unconnected, simulated, unauthorized, or unimplemented subsystem is operational.

## Mandatory decision method

Before changing code, state the exact claim being changed, the subsystem that has authority to make that claim, the evidence supporting it, the privacy class of the data involved, and the failure behavior. Label state as `measured`, `verified`, `configured`, `inferred`, `estimated`, `simulated`, `unknown`, or `stale`. Never turn `unknown` into a plausible default without marking the value as estimated.

Trace every state transition. Define its predecessor, evidence, successor, expiry, demotion, revocation, rollback, and operator-visible representation. A state that can only move upward is presumed incomplete until expiry and revocation are explained.

Treat the control plane as an attack surface. Check identity, authorization, discovery, transport readiness, routing, storage, API serialization, UI/TUI controls, telemetry, and actual side effects together. A success-shaped response without a real side effect is a correctness and security defect. A UI control is not proof that the backend capability exists.

Think beyond the immediate symptom. Ask what happens under a locked identity, stale route, malicious peer, malformed frame, replay, corrupted disk, queue exhaustion, process crash, mobility, clock skew, simultaneous route switching, model failure, and partial command completion. Search for second- and third-order effects, especially feedback loops and metadata leaks.

Prefer reversible changes: isolated `GHOSTWIRE_HOME`, distinct ports, timestamped backups, atomic writes, transactional route changes, shadow mode, explicit feature gates, and deterministic rollback. Never regenerate a production identity to make a test pass. Never reset the dirty canonical checkout.

## Authority boundaries

Discovery reports observations. It does not grant trust. Cryptographic verification proves control of a key. It does not grant application authorization. A transport reports lifecycle and path health. It does not grant permission to publish, relay, administer, or decrypt. The deterministic policy layer owns authorization, privacy, resource budgets, cryptographic validity, and fail-closed behavior. Future ML may recommend among already-eligible choices; it may never make an invalid interface valid or an unauthorized peer trusted.

## Cockpit truth contract

Web, TUI, CLI, REST, and WebSocket surfaces must use authoritative backend state and distinguish at least `implemented`, `available`, `configured`, `registered`, `started`, `ready`, `verified`, `deferred`, `degraded`, `failed`, `disabled`, `simulated`, and `unknown`. Controls for unwired features must be disabled or return explicit unavailable errors. Frontend hiding is not authorization; redaction must occur server-side.

## Privacy contract

Minimize metadata, not only plaintext. Sender and recipient identifiers, DIDs, usernames, timestamps, hops, vector clocks, presence, discovery provenance, route choice, retries, mailbox timing, addresses, and topology can leak despite ciphertext. Separate routable metadata from private metadata. Use coarse states, quantized time, aggregates, retention limits, scoped diagnostics, route-local pseudonyms, and explicit operator authorization for exact topology or timing.

Do not feed raw message content, private keys, session tokens, exact location, unredacted relationship graphs, or unnecessary stable identifiers into future models. Local inference is preferred when it avoids exporting sensitive telemetry.

## Verification standard

Do not claim a fix from compilation alone. Build a tight red/green feedback loop at the failing seam, rerun the exact reproducer after the change, run the relevant tests, then run the aggregate suite. Record the command and observed result. Distinguish source verification, unit verification, isolated runtime verification, and two-machine verification.

## Agent handoff rule

Every meaningful work block ends with a concise status record containing: verified work, root cause of issues, deferred work, modified files, production/isolated process state, exact next command, and any identity or migration risk. Another agent must be able to continue without reconstructing hidden reasoning.
