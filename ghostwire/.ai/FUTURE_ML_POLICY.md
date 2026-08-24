# GhostWire Future-ML Policy

This policy is normative for any ML, AI, statistical, GNN, Gemma, ONNX, LightGBM, anomaly-detection, or adaptive-routing work in GhostWire.

## Boundary

ML is advisory. Deterministic code owns cryptographic verification, authorization, revocation, parser validity, interface lifecycle, route eligibility, privacy policy, resource budgets, and fail-closed behavior. A model may forecast link quality, mobility, congestion, recovery time, energy cost, or anomalies; it may rank only routes that the supervisor has already verified as eligible.

A model must never decide that a peer is trusted, a key is valid, a packet is safe, a user is authorized, an interface is ready, a privacy rule may be bypassed, or content may be decrypted or disclosed.

## Input rules

Inputs must use a versioned schema with provenance, sample age, confidence, state, privacy class, route/profile epoch, and a hard deadline. Do not send raw message content, private keys, session tokens, exact locations, unredacted relationship graphs, or unnecessary stable peer identifiers. Treat local and remote telemetry as attacker-influenced.

Unknown values remain unknown. Estimated values are labeled and receive a confidence penalty. Stale, malformed, out-of-range, NaN, or privacy-disallowed features cause model abstention or deterministic fallback.

## Output rules

Outputs must be structured, bounded, versioned, expiry-limited, and independently revalidated. Every recommendation must identify the model version, decision ID, confidence, calibration state, evidence features, and deterministic constraints that accepted or rejected it. A model cannot expand the candidate set or bypass the supervisor.

Model failure must be operationally boring: timeout, crash, schema mismatch, low confidence, stale model, unavailable runtime, resource exhaustion, and invalid output all fall back to deterministic policy without route corruption.

## Lifecycle gates

A model is not production-ready because it has good benchmark accuracy. It requires a signed bundle containing the model, feature extractor, scaler, schema, calibration metadata, resource profile, provenance, version, hash, and rollback information. It must pass deterministic replay, privacy review, resource profiling, adversarial feature tests, calibration tests, environment-shift tests, shadow-mode comparison, rollback tests, and bounded canary deployment.

Shadow mode has no authority. Canary mode is bounded, reversible, operator-disableable, and automatically expires. Every live advisory decision must be explainable as a recommendation accepted by deterministic constraints.

## Threat model

Evaluate metric poisoning, Sybil influence, Eclipse steering, replay, distribution shift, model supply-chain compromise, feature privacy leakage, inference resource denial of service, synchronized feedback loops, operator overtrust, and cross-layer privilege escalation. Human review is not a substitute for structural authority separation.

## Research discipline

For each external source, record URL, date, assumptions, exact claim, GhostWire applicability, and what the source does not prove. Do not convert simulation results into production evidence. Compare Omen, EliteKali, Wi-Fi, Tailscale, QUIC, libp2p, RNS, BLE, and low-bandwidth conditions separately.

## Operator contract

Web and TUI must show model version, advisory/shadow/canary state, selected deterministic route, eligible alternatives, measured versus estimated metrics, confidence, sample age, policy constraints, cooldown, fallback reason, and a model-disable control. Never display “AI chose this” without the deterministic explanation.
