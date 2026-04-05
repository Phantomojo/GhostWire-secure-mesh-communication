#!/usr/bin/env python3
"""
GhostWire Dataset - Create Proper Train/Val/Test Splits
Splits 100K dataset into 80/10/10 with stratification
"""

import json
import random
from pathlib import Path

# Paths
INPUT_PATH = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/data/ghostwire_mesh_100k.json")
OUTPUT_DIR = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/data/splits")

print("=" * 70)
print("GHOSTWIRE DATASET - TRAIN/VAL/TEST SPLITS")
print("=" * 70)

# Load dataset
print(f"\nLoading dataset from: {INPUT_PATH}")
with open(INPUT_PATH) as f:
    dataset = json.load(f)

examples = dataset['examples']
print(f"  Total samples: {len(examples):,}")

# Analyze label distribution
label_counts = {}
for ex in examples:
    label = ex['labels'][0]
    label_counts[label] = label_counts.get(label, 0) + 1

print(f"  Unique labels: {len(label_counts)}")
print(f"  Label range: [{min(label_counts.keys())}, {max(label_counts.keys())}]")

# Split with stratification by path_length (proxy for difficulty)
print(f"\nCreating stratified splits...")

# Group by path_length for stratification
groups = {}
for ex in examples:
    path_len = ex['metadata']['path_length']
    if path_len not in groups:
        groups[path_len] = []
    groups[path_len].append(ex)

# Split each group
train_examples = []
val_examples = []
test_examples = []

for path_len, group_examples in groups.items():
    random.shuffle(group_examples)
    n = len(group_examples)
    
    # 80/10/10 split
    train_end = int(n * 0.8)
    val_end = int(n * 0.9)
    
    train_examples.extend(group_examples[:train_end])
    val_examples.extend(group_examples[train_end:val_end])
    test_examples.extend(group_examples[val_end:])

# Shuffle splits
random.shuffle(train_examples)
random.shuffle(val_examples)
random.shuffle(test_examples)

print(f"\nSplit sizes:")
print(f"  Train: {len(train_examples):,} ({100*len(train_examples)/len(examples):.1f}%)")
print(f"  Val:   {len(val_examples):,} ({100*len(val_examples)/len(examples):.1f}%)")
print(f"  Test:  {len(test_examples):,} ({100*len(test_examples)/len(examples):.1f}%)")

# Save splits
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

splits = {
    'train': {
        'examples': train_examples,
        'metadata': {
            'num_samples': len(train_examples),
            'purpose': 'Training',
            'split_ratio': 0.8
        }
    },
    'val': {
        'examples': val_examples,
        'metadata': {
            'num_samples': len(val_examples),
            'purpose': 'Validation (hyperparameter tuning)',
            'split_ratio': 0.1
        }
    },
    'test': {
        'examples': test_examples,
        'metadata': {
            'num_samples': len(test_examples),
            'purpose': 'Testing (final evaluation)',
            'split_ratio': 0.1
        }
    }
}

for split_name, split_data in splits.items():
    output_path = OUTPUT_DIR / f"ghostwire_mesh_{split_name}.json"
    print(f"\nSaving {split_name} split to: {output_path}")
    with open(output_path, 'w') as f:
        json.dump(split_data, f)
    
    file_size_mb = output_path.stat().st_size / 1024 / 1024
    print(f"  Size: {file_size_mb:.1f} MB")

# Summary
print(f"\n" + "=" * 70)
print("SPLITS COMPLETE")
print("=" * 70)
print(f"\nFiles created:")
print(f"  {OUTPUT_DIR / 'ghostwire_mesh_train.json'} - {len(train_examples):,} samples")
print(f"  {OUTPUT_DIR / 'ghostwire_mesh_val.json'} - {len(val_examples):,} samples")
print(f"  {OUTPUT_DIR / 'ghostwire_mesh_test.json'} - {len(test_examples):,} samples")
print(f"\nUsage:")
print(f"  - Train model on train split")
print(f"  - Tune hyperparameters on val split")
print(f"  - Evaluate final accuracy on test split")
