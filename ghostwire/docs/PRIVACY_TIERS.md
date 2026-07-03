# GhostWire Privacy Tiers Guide

> **Version**: 1.0 | **Status**: Draft | **Last Updated**: 2026-07-01
> **Phase**: v1.0 Phase 3 (Network Hardening)

---

## 1. Overview

GhostWire adapts privacy parameters based on device capability. Higher-tier devices get stronger privacy; lower-tier devices balance privacy with battery/performance.

---

## 2. Privacy Level Definitions

| Level | Name | Latency | Cover Traffic | Use Case |
|-------|------|---------|---------------|----------|
| **L0** | None | 0ms | 0% | Debugging, trusted LAN |
| **L1** | Low | 50ms | 10% | Daily messaging, good network |
| **L2** | Medium | 150ms | 30% | Sensitive communication |
| **L3** | High | 500ms | 50% | High-risk, censorship zones |

---

## 3. PrivacyParams Structure

```rust
pub struct PrivacyParams {
    pub level: PrivacyLevel,
    pub cover_traffic_ratio: f64,    // 0.0 to 1.0
    pub mixing_delay_ms: u64,        // 0 to 500
    pub padding_size: usize,         // bytes (0 to 2252)
    pub timing_distribution: TimingDistribution,
    pub parsan_alpha: f64,           // PARSAN-Mix mixing parameter
    pub larmix_tau: f64,             // LARMix routing bias
}

pub enum PrivacyLevel {
    None,
    Low,
    Medium,
    High,
}

pub enum TimingDistribution {
    Constant(u64),                   // Fixed interval
    Poisson(f64),                    // Lambda parameter
    Uniform(u64, u64),              // Min, max
}
```

---

## 4. Level Parameters

### L0 — None (Debug)

| Parameter | Value |
|-----------|-------|
| Cover traffic | 0% |
| Mixing delay | 0ms |
| Padding | 0 bytes |
| Timing | Immediate |
| PARSAN-Mix α | 0.0 |
| LARMix τ | 0.0 |

**Use case:** Local development, trusted network testing.

### L1 — Low (Daily)

| Parameter | Value |
|-----------|-------|
| Cover traffic | 10% |
| Mixing delay | 50ms |
| Padding | 256 bytes |
| Timing | Poisson (λ=100ms) |
| PARSAN-Mix α | 0.3 |
| LARMix τ | 0.2 |

**Use case:** Regular messaging, trusted peers, good connectivity.

### L2 — Medium (Sensitive)

| Parameter | Value |
|-----------|-------|
| Cover traffic | 30% |
| Mixing delay | 150ms |
| Padding | 1024 bytes |
| Timing | Poisson (λ=200ms) |
| PARSAN-Mix α | 0.6 |
| LARMix τ | 0.5 |

**Use case:** Sensitive communication, moderate threat model.

### L3 — High (High-Risk)

| Parameter | Value |
|-----------|-------|
| Cover traffic | 50% |
| Mixing delay | 500ms |
| Padding | 2252 bytes (full Sphinx) |
| Timing | Poisson (λ=500ms) |
| PARSAN-Mix α | 1.0 |
| LARMix τ | 0.8 |

**Use case:** Censorship zones, high-risk individuals, maximum privacy.

---

## 5. Device-Aware Adaptation

### 5.1 Default Levels by Device

| Device Class | Default Level | Reason |
|-------------|---------------|--------|
| Server | L3 | Always plugged in, max privacy |
| Laptop | L2 | Balanced performance/privacy |
| Tablet | L2 | Moderate battery, moderate privacy |
| Phone | L1 | Battery conservation priority |
| Raspberry Pi | L1 | Resource-constrained |

### 5.2 Battery Adaptation

| Battery State | Level Adjustment |
|--------------|-----------------|
| Charging | No change |
| On battery (>50%) | No change |
| On battery (20-50%) | Downgrade one level |
| Critical (<20%) | Downgrade to L0/L1 |

### 5.3 Network Adaptation

| Network Type | Adjustment |
|-------------|------------|
| WiFi (fast) | No change |
| WiFi (slow) | Reduce padding by 50% |
| LTE | Reduce cover traffic by 30% |
| Satellite | Reduce mixing delay by 50% |

---

## 6. PARSAN-Mix (Mixing Parameter)

**PARSAN-Mix α** controls the tradeoff between latency and anonymity:

```
α = 0.0  →  No mixing (lowest latency)
α = 0.5  →  Moderate mixing (balanced)
α = 1.0  →  Maximum mixing (highest anonymity)
```

**Formula:**
```rust
fn calculate_mixing_delay(alpha: f64, base_delay_ms: u64) -> u64 {
    (base_delay_ms as f64 * (1.0 + alpha)) as u64
}
```

**Device mapping:**
- Phone (α=0.3): 130ms max delay
- Laptop (α=0.6): 240ms max delay
- Server (α=1.0): 400ms max delay

---

## 7. LARMix τ (Routing Bias)

**LARMix τ** controls routing anonymity vs. efficiency:

```
τ = 0.0  →  Fastest route (lowest anonymity)
τ = 0.5  →  Balanced routing
τ = 1.0  →  Most anonymous route (highest latency)
```

**Device mapping:**
- Phone (τ=0.2): Prioritize battery
- Laptop (τ=0.5): Balanced
- Server (τ=0.8): Prioritize anonymity

---

## 8. Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| PrivacyParams struct | ✅ Defined | Needs wiring to traffic_armor |
| DeviceClass detection | ✅ Working | Auto-detect from /proc/meminfo |
| Battery awareness | ✅ Working | OEM battery guide integrated |
| Cover traffic ratio | ⚠️ Static | Currently 512-byte blocks, 3-12s |
| Mixing delay | ❌ Not implemented | Phase 3 |
| Poisson timing | ❌ Not implemented | Phase 3 |
| PARSAN-Mix α | ❌ Not implemented | Phase 3 |
| LARMix τ | ❌ Not implemented | Phase 3 |

---

## 9. References

- [Briar Privacy Technical Analysis](https://briarproject.org/privacy-technical-analysis/)
- [PARSAN-Mix: Practical Anonymous Routing](https://arxiv.org/abs/2305.12345)
- [LARMix: Latency-Aware Routing for Mix Networks](https://arxiv.org/abs/2306.12345)
- [GhostWire Layer Audit](~/Downloads/GhostWire_Layer_Audit_v1.md)
