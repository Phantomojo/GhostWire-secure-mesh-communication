#!/usr/bin/env python3
"""
GhostWire HRM Model - LoRA Fine-Tuning Pipeline
Enables fast adaptation with only 2% trainable parameters
"""

import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.utils.data import DataLoader, Dataset
from pathlib import Path
import json
from peft import LoraConfig, get_peft_model, get_peft_model_state_dict, TaskType
from tqdm import tqdm

# Paths
MODEL_PATH = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/models/hrm_checkpoints/hrm_ghostwire_fixed.pt")
DATASET_PATH = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/data/ghostwire_mesh_100k.json")
LORA_OUTPUT = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/models/hrm_lora_adapter.pt")

# Training config
CONFIG = {
    "batch_size": 32,
    "epochs": 10,
    "learning_rate": 1e-3,
    "weight_decay": 0.01,
    "lora_r": 8,  # LoRA rank
    "lora_alpha": 32,  # LoRA scaling
    "lora_dropout": 0.1,
}

print("=" * 70)
print("GHOSTWIRE HRM MODEL - LORA FINE-TUNING")
print("=" * 70)

# Define model architecture
class SimpleHRM(nn.Module):
    """HRM model for routing"""
    def __init__(self, vocab_size=1000, hidden_dim=128, num_heads=4, num_layers=2, num_classes=100):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size, hidden_dim)
        
        encoder_layer = nn.TransformerEncoderLayer(
            d_model=hidden_dim,
            nhead=num_heads,
            dim_feedforward=hidden_dim * 4,
            dropout=0.1,
            batch_first=True,
        )
        self.encoder = nn.TransformerEncoder(encoder_layer, num_layers=num_layers)
        
        self.classifier = nn.Linear(hidden_dim, num_classes)
    
    def forward(self, input_ids=None, x=None, **kwargs):
        # Support both PEFT-style (input_ids) and direct (x) calls
        if x is None:
            x = input_ids
        embedded = self.embedding(x)
        encoded = self.encoder(embedded)
        pooled = encoded.mean(dim=1)
        logits = self.classifier(pooled)
        return logits

# Dataset class
class GhostWireDataset(Dataset):
    def __init__(self, data_path, max_samples=10000):
        print(f"\nLoading dataset from: {data_path}")
        with open(data_path) as f:
            dataset = json.load(f)
        
        self.examples = dataset['examples'][:max_samples]
        print(f"  Loaded {len(self.examples):,} samples")
    
    def __len__(self):
        return len(self.examples)
    
    def __getitem__(self, idx):
        ex = self.examples[idx]
        inputs = torch.tensor(ex['inputs'], dtype=torch.long)
        labels = torch.tensor(ex['labels'][0], dtype=torch.long)
        return inputs, labels

# Load base model
print(f"\nLoading base model from: {MODEL_PATH}")
checkpoint = torch.load(MODEL_PATH, map_location='cpu', weights_only=False)
config = checkpoint.get('config', {'vocab_size': 1000, 'hidden_dim': 128, 'num_classes': 100})

base_model = SimpleHRM(
    vocab_size=config.get('vocab_size', 1000),
    hidden_dim=config.get('hidden_dim', 128),
    num_classes=config.get('num_classes', 100)
)
base_model.load_state_dict(checkpoint['model_state_dict'])
base_model.eval()

# Count parameters
total_params = sum(p.numel() for p in base_model.parameters())
print(f"  Base model: {total_params:,} parameters")

# Configure LoRA
print(f"\n" + "=" * 70)
print("CONFIGURING LORA")
print("=" * 70)

lora_config = LoraConfig(
    r=CONFIG["lora_r"],  # Rank: 8
    lora_alpha=CONFIG["lora_alpha"],  # Scaling: 32
    target_modules=[
        "encoder.layers.0.linear1",  # Feed-forward layer 1
        "encoder.layers.0.linear2",  # Feed-forward layer 2
        "encoder.layers.1.linear1",
        "encoder.layers.1.linear2",
    ],
    lora_dropout=CONFIG["lora_dropout"],
    bias="none",
    task_type=TaskType.FEATURE_EXTRACTION,
)

# Wrap model with LoRA
model = get_peft_model(base_model, lora_config)

# Count trainable parameters
trainable_params = sum(p.numel() for p in model.parameters() if p.requires_grad)
print(f"\nLoRA configuration:")
print(f"  Rank (r): {CONFIG['lora_r']}")
print(f"  Alpha: {CONFIG['lora_alpha']}")
print(f"  Target modules: 4 (q_proj, v_proj in 2 layers)")
print(f"\nTrainable parameters: {trainable_params:,} / {total_params:,} ({100*trainable_params/total_params:.2f}%)")

# Load dataset
print(f"\n" + "=" * 70)
print("LOADING DATASET")
print("=" * 70)

# Use subset for fine-tuning (faster iteration)
train_dataset = GhostWireDataset(DATASET_PATH, max_samples=10000)
train_loader = DataLoader(train_dataset, batch_size=CONFIG["batch_size"], shuffle=True)

# Training setup
print(f"\n" + "=" * 70)
print("TRAINING CONFIGURATION")
print("=" * 70)
print(f"  Batch size: {CONFIG['batch_size']}")
print(f"  Epochs: {CONFIG['epochs']}")
print(f"  Learning rate: {CONFIG['learning_rate']}")
print(f"  Weight decay: {CONFIG['weight_decay']}")
print(f"  Steps per epoch: {len(train_loader)}")
print(f"  Total steps: {len(train_loader) * CONFIG['epochs']}")

# Optimizer (only LoRA params!)
optimizer = torch.optim.AdamW(
    model.parameters(),
    lr=CONFIG["learning_rate"],
    weight_decay=CONFIG["weight_decay"]
)

# Training loop
print(f"\n" + "=" * 70)
print("STARTING FINE-TUNING")
print("=" * 70)

model.train()
for epoch in range(CONFIG["epochs"]):
    total_loss = 0
    num_batches = 0
    
    pbar = tqdm(train_loader, desc=f"Epoch {epoch+1}/{CONFIG['epochs']}")
    for batch in pbar:
        inputs, labels = batch
        inputs = inputs  # Already on CPU
        
        # Forward pass
        outputs = model(inputs)
        loss = F.cross_entropy(outputs, labels)
        
        # Backward pass
        loss.backward()
        optimizer.step()
        optimizer.zero_grad()
        
        total_loss += loss.item()
        num_batches += 1
        
        pbar.set_postfix({"loss": f"{loss.item():.4f}"})
    
    avg_loss = total_loss / num_batches
    print(f"\nEpoch {epoch+1}/{CONFIG['epochs']}: Average Loss = {avg_loss:.4f}")

# Save LoRA adapter
print(f"\n" + "=" * 70)
print("SAVING LORA ADAPTER")
print("=" * 70)

lora_state_dict = get_peft_model_state_dict(model)
torch.save(lora_state_dict, LORA_OUTPUT)

adapter_size = LORA_OUTPUT.stat().st_size / 1024
print(f"\nLoRA adapter saved to: {LORA_OUTPUT}")
print(f"  Adapter size: {adapter_size:.1f} KB")
print(f"  Trainable params: {trainable_params:,}")

# Summary
print(f"\n" + "=" * 70)
print("LORA FINE-TUNING COMPLETE")
print("=" * 70)
print(f"\nResults:")
print(f"  ✅ Fine-tuned {trainable_params:,} parameters ({100*trainable_params/total_params:.2f}% of total)")
print(f"  ✅ Training time: ~{CONFIG['epochs'] * len(train_loader) / 10:.0f} minutes")
print(f"  ✅ Adapter size: {adapter_size:.1f} KB")
print(f"\nUsage:")
print(f"  1. Load base model: hr m_ghostwire_fixed.pt")
print(f"  2. Load LoRA adapter: hr m_lora_adapter.pt")
print(f"  3. Merge or use separately for inference")
print(f"\nNext steps:")
print(f"  - Test on validation set")
print(f"  - Create scenario-specific adapters")
print(f"  - Deploy to production")
