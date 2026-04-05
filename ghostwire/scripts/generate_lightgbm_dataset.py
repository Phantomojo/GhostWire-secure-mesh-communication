#!/usr/bin/env python3
"""
Generate synthetic mesh routing dataset for LightGBM training.

Input: 7 numerical features per link
  - rssi: signal strength (-100 to 0 dBm)
  - latency_ms: round-trip latency (1-500 ms)
  - packet_loss: loss rate (0.0-1.0)
  - battery_level: node battery (0.0-1.0)
  - hop_count: hops to destination (1-15)
  - encounter_frequency: encounters/hour (0.0-10.0)
  - time_since_last_seen: seconds since last contact (0-3600)

Output: delivery_probability (0.0-1.0) computed from realistic mesh physics.

This is the CORRECT task: regression (7 floats → 1 probability).
Not classification. Not token prediction.
"""

import numpy as np
import pandas as pd
from pathlib import Path
import json

OUTPUT_DIR = Path(__file__).parent.parent / "data" / "lightgbm-routing"
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

np.random.seed(42)

def compute_delivery_probability(rssi, latency_ms, packet_loss, battery_level,
                                  hop_count, encounter_frequency, time_since_last_seen):
    """
    Compute realistic delivery probability from network features.
    Based on PRoPHET-inspired model with realistic mesh physics.
    """
    # RSSI component: -100..0 → 0..1, sigmoid-weighted
    rssi_norm = (rssi + 100) / 100.0  # 0..1
    rssi_score = 1.0 / (1.0 + np.exp(-10 * (rssi_norm - 0.5)))

    # Latency component: lower is better, exponential decay
    latency_score = np.exp(-latency_ms / 200.0)

    # Packet loss: direct inverse
    loss_score = 1.0 - packet_loss

    # Battery: linear, but critical below 20%
    battery_score = battery_level if battery_level > 0.2 else battery_level * 0.5

    # Hop count: exponential decay (each hop adds failure risk)
    hop_score = np.exp(-0.15 * hop_count)

    # Encounter frequency: higher = more reliable
    encounter_score = 1.0 - np.exp(-0.5 * encounter_frequency)

    # Recency: exponential decay
    recency_score = np.exp(-time_since_last_seen / 1800.0)  # 30min half-life

    # Weighted combination (weights sum to 1.0)
    prob = (
        0.25 * rssi_score +
        0.20 * latency_score +
        0.20 * loss_score +
        0.10 * battery_score +
        0.10 * hop_score +
        0.08 * encounter_score +
        0.07 * recency_score
    )

    # Add realistic noise (2% std — reduced from 5% for cleaner signal)
    noise = np.random.normal(0, 0.02)
    prob = np.clip(prob + noise, 0.0, 1.0)

    return prob


def generate_dataset(n_samples=100_000):
    """Generate synthetic mesh routing dataset."""
    print(f"Generating {n_samples:,} samples...")

    # Realistic feature distributions
    rssi = np.random.normal(-60, 20, n_samples).clip(-100, 0)
    latency_ms = np.random.exponential(50, n_samples).clip(1, 500)
    packet_loss = np.random.beta(1, 10, n_samples)  # Skewed toward low loss
    battery_level = np.random.uniform(0, 1, n_samples)
    hop_count = np.random.geometric(0.3, n_samples).clip(1, 15)
    encounter_frequency = np.random.exponential(2, n_samples).clip(0, 10)
    time_since_last_seen = np.random.exponential(600, n_samples).clip(0, 3600)

    # Compute target
    delivery_probability = np.array([
        compute_delivery_probability(r, l, p, b, h, e, t)
        for r, l, p, b, h, e, t in zip(
            rssi, latency_ms, packet_loss, battery_level,
            hop_count, encounter_frequency, time_since_last_seen
        )
    ])

    df = pd.DataFrame({
        'rssi': rssi.astype(np.float32),
        'latency_ms': latency_ms.astype(np.float32),
        'packet_loss': packet_loss.astype(np.float32),
        'battery_level': battery_level.astype(np.float32),
        'hop_count': hop_count.astype(np.float32),
        'encounter_frequency': encounter_frequency.astype(np.float32),
        'time_since_last_seen': time_since_last_seen.astype(np.float32),
        'delivery_probability': delivery_probability.astype(np.float32),
    })

    return df


def main():
    print("=" * 70)
    print("GHOSTWIRE LIGHTGBM ROUTING DATASET GENERATOR")
    print("=" * 70)

    # Generate main dataset
    df = generate_dataset(100_000)

    # Save as CSV
    csv_path = OUTPUT_DIR / "routing_dataset.csv"
    df.to_csv(csv_path, index=False)
    print(f"\nSaved CSV: {csv_path}")

    # Save feature metadata
    metadata = {
        "n_samples": len(df),
        "features": [
            "rssi", "latency_ms", "packet_loss", "battery_level",
            "hop_count", "encounter_frequency", "time_since_last_seen"
        ],
        "target": "delivery_probability",
        "feature_ranges": {
            "rssi": {"min": -100, "max": 0, "unit": "dBm"},
            "latency_ms": {"min": 1, "max": 500, "unit": "ms"},
            "packet_loss": {"min": 0.0, "max": 1.0, "unit": "ratio"},
            "battery_level": {"min": 0.0, "max": 1.0, "unit": "ratio"},
            "hop_count": {"min": 1, "max": 15, "unit": "count"},
            "encounter_frequency": {"min": 0.0, "max": 10.0, "unit": "encounters/hour"},
            "time_since_last_seen": {"min": 0, "max": 3600, "unit": "seconds"},
        },
        "target_range": {"min": 0.0, "max": 1.0, "unit": "probability"},
    }

    meta_path = OUTPUT_DIR / "metadata.json"
    with open(meta_path, 'w') as f:
        json.dump(metadata, f, indent=2)
    print(f"Saved metadata: {meta_path}")

    # Statistics
    print(f"\n{'=' * 70}")
    print(f"DATASET STATISTICS")
    print(f"{'=' * 70}")
    print(f"Samples: {len(df):,}")
    print(f"Features: 7")
    print(f"Target: delivery_probability")
    print(f"\nTarget distribution:")
    print(f"  Min:  {df['delivery_probability'].min():.4f}")
    print(f"  Max:  {df['delivery_probability'].max():.4f}")
    print(f"  Mean: {df['delivery_probability'].mean():.4f}")
    print(f"  Std:  {df['delivery_probability'].std():.4f}")
    print(f"  Median: {df['delivery_probability'].median():.4f}")

    print(f"\nFeature statistics:")
    print(df.describe().to_string())

    csv_size = csv_path.stat().st_size / 1024 / 1024
    print(f"\nCSV size: {csv_size:.2f} MB")

    print(f"\n✅ Dataset ready for LightGBM training!")
    print(f"\nNext: Run train_lightgbm.py")


if __name__ == "__main__":
    main()
