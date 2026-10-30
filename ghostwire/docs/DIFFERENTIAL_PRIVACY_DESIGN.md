# Differential Privacy for GhostWire Telemetry

## Purpose

Make GhostWire's "privacy" claim auditable by adding formal (ε,δ)-DP bounds
to aggregate reporting (peer counts, transport stats, health metrics).

## Current State

GhostWire's lenses claim privacy but there's no formal ε-bound on aggregate
reporting. Peer counts, transport stats, and health metrics are reported
without any privacy guarantee. If GhostWire ever reports fleet stats or runs
a directory, this is a gap.

## Design

### Local Differential Privacy (LDP)

Each node perturbs its own data before transmission. The aggregator
combines perturbed values. Privacy guarantee: no single node's data can be
inferred from the aggregate.

### Mechanism: Randomized Response

For binary metrics (e.g., "is this peer online?"):
```
P(report true | actual true) = p
P(report true | actual false) = q
```

Privacy parameter: ε = ln(p(1-q) / (q(1-p)))

### Mechanism: Laplace Mechanism

For numeric metrics (e.g., peer count):
```
reported_value = actual_value + Laplace(0, sensitivity/ε)
```

Sensitivity = max change in output from one person's data.

### Privacy Budget

- Starting budget: ε = 1.0 per reporting period
- Each metric query consumes ε_i from the budget
- Total consumption ≤ ε (composition theorem)
- Budget resets per reporting period (e.g., daily)

### MIC-DP for Correlated Features

When features are correlated (e.g., peer count and transport stats),
standard DP undercounts the privacy loss. MIC-DP (Mutual Information
Constraint DP) accounts for correlations:
```
ε_total = Σ ε_i + Σ I(X_i; X_j)  (mutual information terms)
```

## Implementation Plan

### Phase 1: LDP Library
1. Create `src/core/differential_privacy.rs`
2. Implement randomized response for binary metrics
3. Implement Laplace mechanism for numeric metrics
4. Add privacy budget tracking

### Phase 2: Telemetry Integration
1. Add DP to mesh health reporting (peer counts, transport stats)
2. Add DP to system status (uptime, latency)
3. Add privacy budget to config

### Phase 3: Audit & Verification
1. Add tests for DP guarantees
2. Add privacy budget dashboard
3. Document ε values for each metric

## References

- Dwork 2006: "Differential Privacy"
- Dwork & Roth 2014: "The Algorithmic Foundations of Differential Privacy"
- Local DP: Erlingsson et al. 2014 (RAPPOR)
- MIC-DP: (correlated features)
EOF
echo "created"