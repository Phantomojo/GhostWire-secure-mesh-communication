# GhostWire GNN Training — Lightning AI Studio Setup
# Free GPU: L40s 48GB, 80 hours/month
# No credit card required

---

## Step 1: Create Account

1. Go to https://lightning.ai/
2. Click "Sign Up" — use GitHub or email
3. No credit card needed

## Step 2: Create a Studio

1. Click "Create Studio" (or "New Studio")
2. Configure:
   - **Name**: `ghostwire-gnn-training`
   - **Hardware**: GPU → **L40s (48 GB)**
   - **Image**: PyTorch 2.x (default)
   - **Disk**: 50 GB (default is fine)
3. Click "Create" — takes ~2 minutes to spin up

## Step 3: Upload Training Script

In the Studio terminal:

```bash
# Clone the GhostWire repo (or upload files directly)
cd /home/user
git clone https://github.com/YOUR_USERNAME/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication/ghostwire

# OR if repo is private, upload files manually:
# Drag these files into the Studio file browser:
#   scripts/train_gnn_routing.py
#   scripts/gnn_routing.py (original, for reference)
```

## Step 4: Install Dependencies

```bash
# Install PyTorch Geometric with CUDA support
pip install torch-geometric
pip install pyg-lib torch-scatter torch-sparse torch-cluster -f https://data.pyg.org/whl/torch-2.1.0+cu121.html

# Other dependencies (should already be installed in PyTorch image)
pip install numpy scipy

# Verify GPU is available
python3 -c "import torch; print(f'CUDA: {torch.cuda.is_available()}, GPU: {torch.cuda.get_device_name(0)}')"
```

## Step 5: Run Training

```bash
cd /home/user/GhostWire-secure-mesh-communication/ghostwire

# Full training (10K samples, 100 epochs, early stopping)
python3 scripts/train_gnn_routing.py \
    --epochs 100 \
    --batch-size 32 \
    --hidden-dim 128 \
    --num-heads 4 \
    --num-layers 3 \
    --lr 1e-3 \
    --num-samples 10000 \
    --patience 15

# Quick test (1K samples, 10 epochs)
python3 scripts/train_gnn_routing.py \
    --epochs 10 \
    --batch-size 16 \
    --num-samples 1000 \
    --patience 5
```

## Step 6: Download Trained Model

After training completes, download the model files:

```bash
# Check output
ls -la models/gnn_routing/

# Download via Studio file browser:
#   models/gnn_routing/gnn_routing_best.pt
#   models/gnn_routing/gnn_routing.onnx
#   models/gnn_routing/training_history.json

# OR use rsync/scp from your local machine:
rsync -avz user@<studio-ip>:/home/user/GhostWire-secure-mesh-communication/ghostwire/models/gnn_routing/ ./models/gnn_routing/
```

## Step 7: Wire into Rust

Once the ONNX model is downloaded:

```bash
# Copy to main models directory
cp models/gnn_routing/gnn_routing.onnx models/gnn_routing.onnx

# The OnnxInferenceEngine can load it:
# OnnxInferenceEngine::new("models/gnn_routing.onnx", "gnn_routing")
```

---

## Cost Tracking

- **Free tier**: 80 hours/month on L40s (48GB VRAM)
- **Training time**: ~5-15 minutes for 10K samples, 100 epochs
- **Cost per run**: ~0.1-0.25 hours
- **Runs per month**: 300-800 (plenty for experimentation)

## Troubleshooting

### PyTorch Geometric installation fails
```bash
# Try pre-built wheels
pip install torch-geometric pyg-lib torch-scatter torch-sparse torch-cluster -f https://data.pyg.org/whl/torch-2.1.0+cu121.html

# Or install from source (slower)
pip install torch-geometric
```

### CUDA out of memory
```bash
# Reduce batch size
python3 scripts/train_gnn_routing.py --batch-size 16 --hidden-dim 64
```

### Training too slow
```bash
# Verify GPU is being used
python3 -c "import torch; print(torch.cuda.is_available())"
# Should print: True
```
