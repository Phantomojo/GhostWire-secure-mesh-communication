# BBR-Style Congestion Control — Design for GhostWire Transport Layer

## Purpose

Replace AIMD (1s→30s exponential backoff) with BBR-style congestion control.
BBR models bottleneck bandwidth × min RTT (BDP) and paces to fill the pipe
without persistent queue buildup. Smoother, less timing-leaky.

## Current State

GhostWire uses AIMD (Additive Increase, Multiplicative Decrease) for
retransmission:
- Backoff: 1s → 30s exponential
- RTT smoothing: exponential moving average
- Relevant to ReliabilityMode::Anonymous (fixed intervals to avoid timing leakage)

## BBR Model

BBR (Bottleneck Bandwidth and RTT) estimates:
1. **Bottleneck bandwidth (BtlBw)**: max delivery rate over a window
2. **Minimum RTT (RTprop)**: min RTT over a window
3. **BDP (Bandwidth-Delay Product)**: BtlBw × RTprop

Pacing rate = BtlBw (or slightly less for safety)
Congestion window = BDP (or slightly more)

### States
- **Startup**: exponential growth until bandwidth plateaus
- **Drain**: reduce inflight to drain queue
- **ProbeBW**: steady state, probe for more bandwidth
- **ProbeRTT**: periodically reduce to measure min RTT

### Advantages over AIMD
1. **No persistent queue**: AIMD fills queues; BBR drains them
2. **Fairer**: BBR doesn't compete unfairly with loss-based flows
3. **Less timing-leaky**: smoother pacing, harder to fingerprint
4. **Better on lossy links**: doesn't over-react to random loss

## Implementation Plan

### Phase 1: BBR Estimator
1. Create `src/core/bbr.rs` with BtlBw and RTprop estimation
2. Sliding window for max delivery rate (10s window)
3. Sliding window for min RTT (10s window)
4. BDP calculation

### Phase 2: Pacing
1. Add pacing rate to transport layer
2. Pacing timer between sends
3. Congestion window tracking

### Phase 3: State Machine
1. Implement Startup/Drain/ProbeBW/ProbeRTT states
2. State transitions based on bandwidth/RTT changes
3. ProbeRTT every 10s (reduce to 4 packets for 200ms)

### Phase 4: Integration
1. Replace AIMD in retransmission module
2. Wire into transport selector (use BBR metrics for quality)
3. Add BBR metrics to mesh health (for cockpit-truth)

### Anonymous Mode Consideration
In ReliabilityMode::Anonymous, BBR's smooth pacing is actually beneficial
(fixed intervals to avoid timing leakage). BBR's natural pacing is smoother
than AIMD's sawtooth, making it harder to fingerprint.

## References

- Cardwell et al. 2016: "BBR: Congestion-Based Congestion Control"
- IETF draft-ietf-ccwg-bbr
- Stanford CS244: BBR analysis
- Wi-Fi 6 BBRv3 study (arXiv 2512.18259)
EOF
echo "created"