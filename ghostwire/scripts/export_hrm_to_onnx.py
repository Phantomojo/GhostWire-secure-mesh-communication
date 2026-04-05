#!/usr/bin/env python3
"""
Export GhostWire HRM Model to ONNX
For Rust integration
"""

import torch
import onnx
from pathlib import Path

# Model paths
MODEL_PATH = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/models/hrm_checkpoints/hrm_ghostwire_fixed.pt")
ONNX_PATH = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/models/hrm_ghostwire_fixed.onnx")

print("=" * 70)
print("GHOSTWIRE HRM MODEL - ONNX EXPORT")
print("=" * 70)

# Load checkpoint
print(f"\nLoading model from: {MODEL_PATH}")
if not MODEL_PATH.exists():
    print(f"❌ Model file not found: {MODEL_PATH}")
    print("\nPlease download from Kaggle:")
    print("  1. Go to: https://www.kaggle.com/code/phantomojo/ghostwire-hrm-fixed-t4-training")
    print("  2. Click Output tab")
    print("  3. Download hrm_ghostwire_fixed.pt")
    print(f"  4. Save to: {MODEL_PATH.parent}")
    exit(1)

checkpoint = torch.load(MODEL_PATH, map_location='cpu')
print(f"✅ Checkpoint loaded")
print(f"   Epoch: {checkpoint.get('epoch', 'N/A')}")
print(f"   Config: {checkpoint.get('config', 'N/A')}")

# Define the model architecture (must match training)
class SimpleHRM(torch.nn.Module):
    """HRM-inspired model for routing"""
    def __init__(self, vocab_size=1000, hidden_dim=128, num_heads=4, num_layers=2, num_classes=100):
        super().__init__()
        self.embedding = torch.nn.Embedding(vocab_size, hidden_dim)
        
        encoder_layer = torch.nn.TransformerEncoderLayer(
            d_model=hidden_dim,
            nhead=num_heads,
            dim_feedforward=hidden_dim * 4,
            dropout=0.1,
            batch_first=True,
        )
        self.encoder = torch.nn.TransformerEncoder(encoder_layer, num_layers=num_layers)
        
        self.classifier = torch.nn.Linear(hidden_dim, num_classes)
    
    def forward(self, x):
        # x: [batch, seq_len]
        embedded = self.embedding(x)  # [batch, seq_len, hidden]
        encoded = self.encoder(embedded)  # [batch, seq_len, hidden]
        pooled = encoded.mean(dim=1)  # [batch, hidden]
        logits = self.classifier(pooled)  # [batch, num_classes]
        return logits

# Create model and load weights
print("\nCreating model architecture...")
config = checkpoint.get('config', {'vocab_size': 1000, 'hidden_dim': 128, 'num_classes': 100})
model = SimpleHRM(
    vocab_size=config.get('vocab_size', 1000),
    hidden_dim=config.get('hidden_dim', 128),
    num_classes=config.get('num_classes', 100)
)

# Load state dict
model.load_state_dict(checkpoint['model_state_dict'])
print(f"✅ Weights loaded")

# Set to eval mode
model.eval()

# Create dummy input for export
batch_size = 1
seq_len = 64
dummy_input = torch.randint(0, 1000, (batch_size, seq_len), dtype=torch.long)

print(f"\nExporting to ONNX...")
print(f"   Input shape: {dummy_input.shape}")
print(f"   Input type: {dummy_input.dtype}")

# Export to ONNX
torch.onnx.export(
    model,
    dummy_input,
    ONNX_PATH,
    export_params=True,
    opset_version=14,
    do_constant_folding=True,
    input_names=['input_tokens'],
    output_names=['routing_logits'],
    dynamic_axes={
        'input_tokens': {0: 'batch_size', 1: 'sequence_length'},
        'routing_logits': {0: 'batch_size'}
    }
)

print(f"\n✅ ONNX export complete!")
print(f"   Model saved to: {ONNX_PATH}")
print(f"   Model size: {ONNX_PATH.stat().st_size / 1024 / 1024:.1f} MB")

# Verify ONNX model
print(f"\nVerifying ONNX model...")
onnx_model = onnx.load(ONNX_PATH)
onnx.checker.check_model(onnx_model)
print(f"✅ ONNX model verified!")

# Test inference
print(f"\nTesting inference...")
with torch.no_grad():
    output = model(dummy_input)
    print(f"   Output shape: {output.shape}")
    print(f"   Output range: [{output.min():.4f}, {output.max():.4f}]")

print(f"\n" + "=" * 70)
print("EXPORT COMPLETE")
print("=" * 70)
print(f"\nNext steps:")
print(f"1. Copy ONNX model to Rust project")
print(f"2. Add ort crate to Cargo.toml")
print(f"3. Use ONNX model for routing decisions")
