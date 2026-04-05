#!/usr/bin/env python3
"""
GhostWire Setup Verification Script
Run this after installing dependencies to verify everything works.
"""

import sys
import os
from pathlib import Path

def check(name, condition, detail=""):
    status = "✅" if condition else "❌"
    print(f"  {status} {name}")
    if detail and not condition:
        print(f"     → {detail}")
    return condition

def main():
    print("=" * 50)
    print("GHOSTWIRE SETUP VERIFICATION")
    print("=" * 50)
    
    all_ok = True
    
    # Python version
    print("\n[Python]")
    all_ok &= check("Python 3.10+", sys.version_info >= (3, 10),
                   f"Found {sys.version_info.major}.{sys.version_info.minor}")
    
    # Core libraries
    print("\n[Core Libraries]")
    try:
        import numpy
        check("numpy", True, f"v{numpy.__version__}")
    except ImportError:
        all_ok &= check("numpy", False, "pip install numpy")
    
    try:
        import pandas
        check("pandas", True, f"v{pandas.__version__}")
    except ImportError:
        all_ok &= check("pandas", False, "pip install pandas")
    
    try:
        import sklearn
        check("scikit-learn", True, f"v{sklearn.__version__}")
    except ImportError:
        all_ok &= check("scikit-learn", False, "pip install scikit-learn")
    
    try:
        import torch
        check("pytorch", True, f"v{torch.__version__}")
    except ImportError:
        all_ok &= check("pytorch", False, "pip install torch")
    
    try:
        import torch_geometric
        check("pytorch-geometric", True)
    except ImportError:
        all_ok &= check("pytorch-geometric", False, "pip install torch-geometric")
    
    try:
        import onnxruntime
        check("onnxruntime", True)
    except ImportError:
        all_ok &= check("onnxruntime", False, "pip install onnxruntime")
    
    try:
        import networkx
        check("networkx", True)
    except ImportError:
        all_ok &= check("networkx", False, "pip install networkx")
    
    try:
        import matplotlib
        check("matplotlib", True)
    except ImportError:
        all_ok &= check("matplotlib", False, "pip install matplotlib")
    
    # Data files
    print("\n[Data Files]")
    base = Path(__file__).parent.parent / "datasets" / "processed"
    
    l1_dir = base / "l1_lightgbm"
    all_ok &= check("L1 LightGBM data", l1_dir.exists(),
                   "Run datasets/prepare_v2.py first")
    if l1_dir.exists():
        all_ok &= check("  X_train.npy", (l1_dir / "guifisants_X_train.npy").exists())
        all_ok &= check("  scaler.pkl", (l1_dir / "guifisants_scaler.pkl").exists())
    
    l3_dir = base / "l3_gnn"
    all_ok &= check("L3 GNN data", l3_dir.exists(),
                   "Run datasets/prepare_v2.py first")
    if l3_dir.exists():
        adj_files = list(l3_dir.glob("*_adj.npz"))
        all_ok &= check(f"  Adjacency matrices", len(adj_files) > 0,
                       f"Found {len(adj_files)} files")
    
    anomaly_dir = base / "anomaly_detection"
    all_ok &= check("Anomaly detection data", anomaly_dir.exists())
    if anomaly_dir.exists():
        all_ok &= check("  Full parquet", (anomaly_dir / "guifisants_full.parquet").exists())
    
    # Models
    print("\n[Models]")
    models_dir = Path(__file__).parent.parent / "models"
    all_ok &= check("Models directory", models_dir.exists())
    
    # Summary
    print("\n" + "=" * 50)
    if all_ok:
        print("✅ ALL CHECKS PASSED — You're ready to go!")
    else:
        print("⚠️  SOME CHECKS FAILED — Fix the issues above")
        print("   Share errors in the WhatsApp group if stuck")
    print("=" * 50)
    
    return 0 if all_ok else 1

if __name__ == "__main__":
    sys.exit(main())
