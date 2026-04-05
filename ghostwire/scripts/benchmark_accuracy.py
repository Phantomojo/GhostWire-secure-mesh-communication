#!/usr/bin/env python3
"""
GhostWire Model - Accuracy Benchmark
Properly handles model loading and evaluation
"""

import torch
import torch.nn.functional as F
import json
from pathlib import Path
from tqdm import tqdm

print("=" * 70)
print("GHOSTWIRE MODEL - ACCURACY BENCHMARK")
print("=" * 70)

# Define model
class SimpleHRM(torch.nn.Module):
    def __init__(self, vocab_size=1000, hidden_dim=128, num_heads=4, num_layers=2, num_classes=100):
        super().__init__()
        self.embedding = torch.nn.Embedding(vocab_size, hidden_dim)
        encoder_layer = torch.nn.TransformerEncoderLayer(
            d_model=hidden_dim, nhead=num_heads, dim_feedforward=hidden_dim*4,
            dropout=0.1, batch_first=True,
        )
        self.encoder = torch.nn.TransformerEncoder(encoder_layer, num_layers=num_layers)
        self.classifier = torch.nn.Linear(hidden_dim, num_classes)
    
    def forward(self, x):
        # Handle padding (-1 -> 0)
        x = torch.clamp(x, min=0)
        embedded = self.embedding(x)
        encoded = self.encoder(embedded)
        pooled = encoded.mean(dim=1)
        return self.classifier(pooled)

# Load checkpoint
model_path = Path("models/hrm_checkpoints/hrm_ghostwire_fixed.pt")
checkpoint = torch.load(model_path, map_location='cpu', weights_only=False)
orig_config = checkpoint.get('config', {'vocab_size': 1000, 'hidden_dim': 128, 'num_classes': 100})

print(f"\nOriginal config: {orig_config}")

# Create model with ORIGINAL config
model = SimpleHRM(
    vocab_size=orig_config.get('vocab_size', 1000),
    hidden_dim=orig_config.get('hidden_dim', 128),
    num_classes=orig_config.get('num_classes', 100)
)

# Load weights
model.load_state_dict(checkpoint['model_state_dict'])
model.eval()

total_params = sum(p.numel() for p in model.parameters())
print(f"✅ Model loaded: {total_params:,} parameters")

# Load test set
test_path = Path("data/splits/ghostwire_mesh_test.json")
with open(test_path) as f:
    test_data = json.load(f)

# Check data range
max_val = 0
for ex in test_data['examples'][:100]:
    for val in ex['inputs']:
        if val > max_val:
            max_val = val

print(f"✅ Test set loaded: {len(test_data['examples']):,} samples")
print(f"   Max input value: {max_val}")
print(f"   Model vocab size: {orig_config.get('vocab_size', 1000)}")

if max_val >= orig_config.get('vocab_size', 1000):
    print(f"\n⚠️ WARNING: Test data has values >= vocab size!")
    print(f"   Clipping values to max {orig_config.get('vocab_size', 1000) - 1}")

# Benchmark
correct = 0
total = 0
top3_correct = 0
top5_correct = 0

print(f"\nRunning accuracy benchmark...")
vocab_size = orig_config.get('vocab_size', 1000)

for ex in tqdm(test_data['examples'][:1000]):
    inputs = torch.tensor([ex['inputs']], dtype=torch.long)
    label = ex['labels'][0]
    
    # Clip to valid range
    inputs = torch.clamp(inputs, min=0, max=vocab_size-1)
    
    with torch.no_grad():
        outputs = model(inputs)
    
    # Top-1 accuracy
    pred = outputs.argmax(dim=1).item()
    if pred == label:
        correct += 1
    
    # Top-3 accuracy
    top3 = outputs.topk(3, dim=1).indices[0].tolist()
    if label in top3:
        top3_correct += 1
    
    # Top-5 accuracy
    top5 = outputs.topk(5, dim=1).indices[0].tolist()
    if label in top5:
        top5_correct += 1
    
    total += 1

print(f"\n" + "=" * 70)
print("BENCHMARK RESULTS")
print("=" * 70)
print(f"Test samples: {total:,}")
print(f"Top-1 Accuracy: {100*correct/total:.2f}%")
print(f"Top-3 Accuracy: {100*top3_correct/total:.2f}%")
print(f"Top-5 Accuracy: {100*top5_correct/total:.2f}%")
print(f"\nBaseline (random): {100/orig_config.get('num_classes', 100):.2f}%")
print(f"Improvement over baseline: +{100*correct/total - 100/orig_config.get('num_classes', 100):.2f} percentage points")
