#!/usr/bin/env python3
"""
Train GhostWire Model on Hybrid Dataset (101K samples)
Combines synthetic + real WSN data
"""

import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.utils.data import DataLoader, Dataset
from pathlib import Path
import json
from tqdm import tqdm

# Paths
DATASET_PATH = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/data/ghostwire_mesh_hybrid_101k.json")
MODEL_OUTPUT = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/models/hrm_ghostwire_hybrid_101k.pt")
CHECKPOINT_DIR = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/models/hrm_checkpoints")

# Training config
CONFIG = {
    "batch_size": 64,
    "epochs": 30,
    "learning_rate": 1e-3,
    "weight_decay": 0.01,
    "vocab_size": 2048,
    "hidden_dim": 128,
    "num_heads": 4,
    "num_layers": 2,
    "num_classes": 200,
}

print("=" * 70)
print("GHOSTWIRE MODEL - TRAINING ON HYBRID DATASET (101K)")
print("=" * 70)

# Device selection
device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
print(f"\nUsing device: {device}")
if torch.cuda.is_available():
    print(f"  GPU: {torch.cuda.get_device_name(0)}")
    print(f"  Memory: {torch.cuda.get_device_properties(0).total_memory / 1024**3:.1f} GB")

# Dataset class
class GhostWireDataset(Dataset):
    def __init__(self, data_path, max_samples=None):
        print(f"\nLoading dataset from: {data_path}")
        with open(data_path) as f:
            dataset = json.load(f)
        
        self.examples = dataset['examples']
        if max_samples:
            self.examples = self.examples[:max_samples]
        
        print(f"  Loaded {len(self.examples):,} samples")
    
    def __len__(self):
        return len(self.examples)
    
    def __getitem__(self, idx):
        ex = self.examples[idx]
        inputs = torch.tensor(ex['inputs'], dtype=torch.long)
        # Clamp to valid range
        inputs = torch.clamp(inputs, min=0, max=CONFIG['vocab_size']-1)
        # Pad or truncate to fixed length (128)
        if len(inputs) < 128:
            inputs = F.pad(inputs, (0, 128 - len(inputs)), value=0)
        elif len(inputs) > 128:
            inputs = inputs[:128]
        labels = torch.tensor(ex['labels'][0], dtype=torch.long)
        return inputs, labels

# Model definition
class SimpleHRM(nn.Module):
    def __init__(self, vocab_size=2048, hidden_dim=128, num_heads=4, num_layers=2, num_classes=200):
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
    
    def forward(self, x):
        # Handle padding (-1 -> 0)
        x = torch.clamp(x, min=0)
        embedded = self.embedding(x)
        encoded = self.encoder(embedded)
        pooled = encoded.mean(dim=1)
        logits = self.classifier(pooled)
        return logits

# Create dataset and dataloader
print(f"\n" + "=" * 70)
print("LOADING DATASET")
print("=" * 70)

# Split into train/val (90/10)
dataset = GhostWireDataset(DATASET_PATH)
dataset_size = len(dataset)
train_size = int(0.9 * dataset_size)
val_size = dataset_size - train_size

train_dataset, val_dataset = torch.utils.data.random_split(dataset, [train_size, val_size])

print(f"\nTrain size: {train_size:,}")
print(f"Val size: {val_size:,}")

train_loader = DataLoader(train_dataset, batch_size=CONFIG['batch_size'], shuffle=True, num_workers=0)
val_loader = DataLoader(val_dataset, batch_size=CONFIG['batch_size'], shuffle=False, num_workers=0)

# Create model
print(f"\n" + "=" * 70)
print("CREATING MODEL")
print("=" * 70)

model = SimpleHRM(
    vocab_size=CONFIG['vocab_size'],
    hidden_dim=CONFIG['hidden_dim'],
    num_heads=CONFIG['num_heads'],
    num_layers=CONFIG['num_layers'],
    num_classes=CONFIG['num_classes'],
)

num_params = sum(p.numel() for p in model.parameters())
print(f"  Model parameters: {num_params:,}")
print(f"  Architecture: Transformer Encoder (2 layers, 4 heads)")

# Move model to device
model = model.to(device)

# Training setup
optimizer = torch.optim.AdamW(model.parameters(), lr=CONFIG['learning_rate'], weight_decay=CONFIG['weight_decay'])
scheduler = torch.optim.lr_scheduler.CosineAnnealingLR(optimizer, T_max=CONFIG['epochs'])
criterion = nn.CrossEntropyLoss()

# Training loop
print(f"\n" + "=" * 70)
print("STARTING TRAINING")
print("=" * 70)
print(f"  Epochs: {CONFIG['epochs']}")
print(f"  Batch size: {CONFIG['batch_size']}")
print(f"  Learning rate: {CONFIG['learning_rate']}")
print(f"  Steps per epoch: {len(train_loader)}")

CHECKPOINT_DIR.mkdir(parents=True, exist_ok=True)

best_val_loss = float('inf')

for epoch in range(CONFIG['epochs']):
    # Training
    model.train()
    total_train_loss = 0
    num_batches = 0
    
    pbar = tqdm(train_loader, desc=f"Epoch {epoch+1}/{CONFIG['epochs']} [Train]")
    for inputs, labels in pbar:
        inputs = inputs.to(device)
        labels = labels.to(device)
        optimizer.zero_grad()
        outputs = model(inputs)
        loss = criterion(outputs, labels)
        loss.backward()
        optimizer.step()
        
        total_train_loss += loss.item()
        num_batches += 1
        
        pbar.set_postfix({"loss": f"{loss.item():.4f}"})
    
    avg_train_loss = total_train_loss / num_batches
    
    # Validation
    model.eval()
    total_val_loss = 0
    correct = 0
    total = 0
    top3_correct = 0
    
    with torch.no_grad():
        for inputs, labels in val_loader:
            inputs = inputs.to(device)
            labels = labels.to(device)
            outputs = model(inputs)
            loss = criterion(outputs, labels)
            total_val_loss += loss.item()
            
            # Accuracy
            preds = outputs.argmax(dim=1)
            correct += (preds == labels).sum().item()
            total += labels.size(0)
            
            # Top-3 accuracy
            top3 = outputs.topk(3, dim=1).indices
            top3_correct += (top3 == labels.unsqueeze(1)).any(dim=1).sum().item()
    
    avg_val_loss = total_val_loss / len(val_loader)
    train_acc = 100 * correct / total
    top3_acc = 100 * top3_correct / total
    
    # Learning rate scheduling
    scheduler.step()
    
    print(f"\nEpoch {epoch+1}/{CONFIG['epochs']}:")
    print(f"  Train Loss: {avg_train_loss:.4f}")
    print(f"  Val Loss: {avg_val_loss:.4f}")
    print(f"  Val Top-1 Acc: {train_acc:.2f}%")
    print(f"  Val Top-3 Acc: {top3_acc:.2f}%")
    
    # Save best model
    if avg_val_loss < best_val_loss:
        best_val_loss = avg_val_loss
        checkpoint_path = CHECKPOINT_DIR / f"checkpoint_hybrid_best.pt"
        torch.save({
            'epoch': epoch + 1,
            'model_state_dict': model.state_dict(),
            'optimizer_state_dict': optimizer.state_dict(),
            'train_loss': avg_train_loss,
            'val_loss': avg_val_loss,
            'val_acc': train_acc,
            'val_top3_acc': top3_acc,
            'config': CONFIG,
        }, checkpoint_path)
        print(f"  ✅ Best model saved (val_loss: {best_val_loss:.4f})")
    
    # Save checkpoint every 10 epochs
    if (epoch + 1) % 10 == 0:
        checkpoint_path = CHECKPOINT_DIR / f"checkpoint_hybrid_epoch_{epoch+1}.pt"
        torch.save({
            'epoch': epoch + 1,
            'model_state_dict': model.state_dict(),
            'optimizer_state_dict': optimizer.state_dict(),
            'train_loss': avg_train_loss,
            'val_loss': avg_val_loss,
            'val_acc': train_acc,
            'val_top3_acc': top3_acc,
            'config': CONFIG,
        }, checkpoint_path)
        print(f"  ✅ Checkpoint saved: epoch_{epoch+1}")

# Final save
print(f"\n" + "=" * 70)
print("TRAINING COMPLETE")
print("=" * 70)

torch.save({
    'epoch': CONFIG['epochs'],
    'model_state_dict': model.state_dict(),
    'optimizer_state_dict': optimizer.state_dict(),
    'config': CONFIG,
}, MODEL_OUTPUT)

model_size_mb = MODEL_OUTPUT.stat().st_size / 1024 / 1024
print(f"\nFinal model saved to: {MODEL_OUTPUT}")
print(f"  Size: {model_size_mb:.1f} MB")
print(f"\nNext step: Benchmark accuracy on test set")
