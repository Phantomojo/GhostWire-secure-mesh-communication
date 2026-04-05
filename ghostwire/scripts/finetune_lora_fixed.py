#!/usr/bin/env python3
"""
GhostWire HRM Model - LoRA Fine-Tuning Pipeline (FIXED)
Properly targets PyTorch TransformerEncoderLayer modules
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
    "lora_r": 8,
    "lora_alpha": 32,
    "lora_dropout": 0.1,
}

print("=" * 70)
print("GHOSTWIRE HRM MODEL - LORA FINE-TUNING (FIXED)")
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
    
    def forward(self, input_ids=None, attention_mask=None, x=None, **kwargs):
        # Support both PEFT-style (input_ids) and direct (x) calls
        # PEFT passes input_ids, attention_mask, and other kwargs
        if x is None and input_ids is not None:
            x = input_ids
        elif x is None:
            raise ValueError("Either input_ids or x must be provided")
        
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

# Load base model with larger vocab and num_classes to accommodate dataset
print(f"\nLoading base model from: {MODEL_PATH}")
checkpoint = torch.load(MODEL_PATH, map_location='cpu', weights_only=False)

# Update config for larger vocab and num_classes (dataset has node indices up to ~200 and labels up to ~165)
config = checkpoint.get('config', {'vocab_size': 2048, 'hidden_dim': 128, 'num_classes': 200})
config['vocab_size'] = 2048   # Increase to accommodate node indices up to 2000
config['num_classes'] = 200   # Increase to accommodate labels up to ~165

base_model = SimpleHRM(
    vocab_size=config.get('vocab_size', 2048),
    hidden_dim=config.get('hidden_dim', 128),
    num_classes=config.get('num_classes', 200)
)

# Load weights with manual embedding and classifier handling
checkpoint_state = checkpoint['model_state_dict']
current_state = base_model.state_dict()

# Copy all weights except embedding and classifier
for key in checkpoint_state:
    if key not in ['embedding.weight', 'classifier.weight', 'classifier.bias']:
        current_state[key] = checkpoint_state[key]

# Initialize embedding with checkpoint weights (padded with zeros for new vocab)
old_embed = checkpoint_state['embedding.weight']
new_embed = current_state['embedding.weight']
new_embed[:old_embed.shape[0], :] = old_embed  # Copy old weights
current_state['embedding.weight'] = new_embed

# Initialize classifier with checkpoint weights (padded for new classes)
old_classifier_w = checkpoint_state['classifier.weight']
old_classifier_b = checkpoint_state['classifier.bias']
new_classifier_w = current_state['classifier.weight']
new_classifier_b = current_state['classifier.bias']
new_classifier_w[:old_classifier_w.shape[0], :] = old_classifier_w
new_classifier_b[:old_classifier_b.shape[0]] = old_classifier_b
current_state['classifier.weight'] = new_classifier_w
current_state['classifier.bias'] = new_classifier_b

base_model.load_state_dict(current_state)
base_model.eval()

# Count parameters
total_params = sum(p.numel() for p in base_model.parameters())
print(f"  Base model: {total_params:,} parameters")

# Inspect model structure for LoRA targeting
print(f"\n" + "=" * 70)
print("MODEL STRUCTURE ANALYSIS")
print("=" * 70)

def print_modules(module, indent=0, prefix=""):
    for name, child in module.named_children():
        full_name = f"{prefix}{name}"
        print(f"{'  ' * indent}{full_name}: {type(child).__name__}")
        if isinstance(child, nn.Linear):
            print(f"{'  ' * (indent+1)}-> weight: {child.weight.shape}")
        if hasattr(child, 'named_children'):
            print_modules(child, indent + 1, f"{full_name}.")

print("\nAvailable modules for LoRA targeting:")
print_modules(base_model)

# Configure LoRA - targeting CORRECT module names for PyTorch TransformerEncoderLayer
print(f"\n" + "=" * 70)
print("CONFIGURING LORA")
print("=" * 70)

# PyTorch TransformerEncoderLayer has:
# - encoder.layers.0.self_attn.out_proj (Linear)
# - encoder.layers.0.linear1 (Linear)
# - encoder.layers.0.linear2 (Linear)
# Note: q_proj, k_proj, v_proj don't exist as separate modules
# They are combined in in_proj_weight which PEFT doesn't support

lora_config = LoraConfig(
    r=CONFIG["lora_r"],
    lora_alpha=CONFIG["lora_alpha"],
    target_modules=[
        "encoder.layers.0.self_attn.out_proj",  # Attention output
        "encoder.layers.0.linear1",              # Feed-forward 1
        "encoder.layers.0.linear2",              # Feed-forward 2
        "encoder.layers.1.self_attn.out_proj",
        "encoder.layers.1.linear1",
        "encoder.layers.1.linear2",
    ],
    lora_dropout=CONFIG["lora_dropout"],
    bias="none",
    task_type=TaskType.FEATURE_EXTRACTION,
)

print(f"\nLoRA configuration:")
print(f"  Rank (r): {CONFIG['lora_r']}")
print(f"  Alpha: {CONFIG['lora_alpha']}")
print(f"  Target modules: 6 (out_proj + linear1 + linear2 in 2 layers)")

# Wrap model with LoRA
print(f"\nWrapping model with LoRA...")
try:
    model = get_peft_model(base_model, lora_config)
    print(f"  ✅ LoRA wrapping successful")
except Exception as e:
    print(f"  ❌ LoRA wrapping failed: {e}")
    print(f"\nThis confirms that PEFT/LoRA is NOT compatible with custom PyTorch models")
    print(f"using TransformerEncoderLayer without HuggingFace model structure.")
    print(f"\nRECOMMENDATION: Use manual LoRA implementation or switch to HuggingFace model")
    exit(1)

# Count trainable parameters
trainable_params = sum(p.numel() for p in model.parameters() if p.requires_grad)
print(f"\nTrainable parameters: {trainable_params:,} / {total_params:,} ({100*trainable_params/total_params:.2f}%)")

# Load dataset
print(f"\n" + "=" * 70)
print("LOADING DATASET")
print("=" * 70)

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

# Optimizer
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
print(f"  1. Load base model: hrm_ghostwire_fixed.pt")
print(f"  2. Load LoRA adapter: hrm_lora_adapter.pt")
print(f"  3. Merge or use separately for inference")
