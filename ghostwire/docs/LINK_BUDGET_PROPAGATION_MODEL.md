# Link Budget & Propagation Model — GhostWire Radio Transports

## Purpose

Ground GhostWire's "meshing" and "range" claims in checkable physics. Without
a propagation model, transport design (BLE, WiFi, RNS, LoRa-adjacent) is
ungrounded. This document provides the formulas and assumption sets needed.

## 1. Link Budget (Fundamental)

```
P_RX = P_TX + G_sys - L_sys - L_channel - M
```

Where:
- `P_TX` = transmit power (dBm)
- `G_sys` = system gain (antenna gains, dB)
- `L_sys` = system losses (cable, connector, dB)
- `L_channel` = channel path loss (dB)
- `M` = fading margin (dB)

A link is **link-limited** when `P_RX < S_r` (receiver sensitivity).
A link is **interference-limited** when noise floor / other transmitters dominate.

### Receiver Sensitivity
```
S_r = SNR_min + N_0 + BW
```
Where:
- `SNR_min` = minimum SNR for correct demodulation (modulation-dependent)
- `N_0` = noise floor = NF + kT (dBm/Hz)
- `BW` = bandwidth (Hz, in dB)

### Shannon Capacity
```
C = B * log2(1 + SNR)
```
Where:
- `C` = channel capacity (bps)
- `B` = bandwidth (Hz)
- `SNR` = signal-to-noise ratio (linear)

## 2. Path Loss Models

### Free-Space (Friis)
```
L_fs = 32.45 + 20*log10(d_km) + 20*log10(f_MHz)
```
Inverse-square law. Valid for clear line-of-sight, no obstacles.

### 2-Ray Ground Reflection
```
L_2ray = 120 - 20*log10(h_TX * h_RX) + 40*log10(d)
```
Fourth-power loss beyond critical distance. Low antennas kill range.

Critical distance: `d_c = 4 * h_TX * h_RX / λ` (wavelength)

### Log-Normal Shadowing
```
L_total = L_model + X_σ
```
Where `X_σ ~ N(0, σ)` dB, σ ≈ 3-8 dB depending on environment.

### Okumura-Hata (Urban, 800-2000 MHz, 20m-5km)
```
L = 69.55 + 26.16*log10(f) - 13.82*log10(h_b) - a(h_m)
    + (44.9 - 6.55*log10(h_b))*log10(d)
```
Where `h_b` = base station antenna height, `h_m` = mobile antenna height.

## 3. Thermal Noise & Sensitivity

```
N_0 = NF + kT  (dBm/Hz)
```
Where:
- `NF` = noise figure (dB, receiver-dependent)
- `k` = Boltzmann constant (1.38e-23 J/K)
- `T` = temperature (K, ~290K room temp)

Noise floor at 290K, 1 Hz: -174 dBm/Hz

### LoRa Spreading Factor Tradeoff
Higher SF = lower data rate but better SNR margin:
- SF7: ~5.5 kbps, SNR threshold ~-7.5 dB
- SF12: ~293 bps, SNR threshold ~-20 dB

Dynamic coding rate = automated SF selection based on link conditions.

## 4. Fading & Interference

### Fading Margin
Absorbs multipath, Doppler, and interference into a single margin `M`.
For mobile nodes (phones, laptops), `M` ≈ 10-20 dB.

### Doppler Shift
```
f_d = (v/c) * f_c * cos(θ)
```
Where `v` = relative velocity, `c` = speed of light, `f_c` = carrier frequency.

## 5. GhostWire Transport Assumptions

### BLE (Nordic UART Service)
- Frequency: 2.4 GHz
- TX power: 0 dBm (typical)
- Antenna gain: 0 dBi (chip antenna)
- Receiver sensitivity: -90 dBm
- Environment: indoor, σ = 6 dB
- Range: ~10-30m (with 2-ray model)

### WiFi (802.11n/ac)
- Frequency: 2.4/5 GHz
- TX power: 20 dBm
- Antenna gain: 2 dBi
- Receiver sensitivity: -80 dBm
- Environment: indoor, σ = 8 dB
- Range: ~30-100m

### RNS (AutoInterface, multicast UDP)
- Frequency: 2.4 GHz (WiFi) or 868/915 MHz (LoRa-adjacent)
- TX power: varies by hardware
- Range: depends on modulation and environment

## 6. Dynamic Coding Rate for Transport Selector

The transport selector should output not just transport choice but also
coding rate (for radio transports):

```
coding_rate = f(link_quality, snr_margin, battery_level, message_priority)
```

- High link quality + high battery → high coding rate (fast)
- Low link quality + low battery → low coding rate (robust)
- Emergency messages → minimum coding rate (maximum reliability)

## 7. Implementation Recommendations

1. **Add `link_budget` module** to `src/core/` with the formulas above
2. **Add `propagation_model` field** to RNS interface config (environment type)
3. **Add `coding_rate` output** to transport selector
4. **Add link quality estimation** to mesh health (RSSI/SNR tracking)
5. **Add range estimation** to peer discovery (based on link budget)

## 8. References

- Semtech AN1200.22: LoRa link budget
- Roesler 2023: LoRa opportunistic routing
- Shannon 1948: A Mathematical Theory of Communication
- Okumura-Hata: urban propagation model
- Friis 1946: free-space transmission formula
EOF
echo "created"