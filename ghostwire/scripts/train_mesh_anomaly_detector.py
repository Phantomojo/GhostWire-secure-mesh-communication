#!/usr/bin/env python3
"""
Train Isolation Forest on REAL Guifi.net mesh data (Guifisants node).

Replaces the NSL-KDD intrusion detection model with a mesh-domain
anomaly detector. Uses the same 2,387-feature schema as the L1 LightGBM
but with unservised anomaly detection (no labels needed).

This provides a complementary detection method:
  - L1 LightGBM: supervised, detects known gateway failure patterns
  - L1 Isolation Forest: unsupervised, detects novel anomaly types
"""

import numpy as np
import time
import json
from pathlib import Path
from sklearn.ensemble import IsolationForest
from sklearn.metrics import roc_auc_score, average_precision_score

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
DATA_DIR = PROJECT_DIR.parent / "datasets" / "processed" / "l1_lightgbm"
MODEL_DIR = PROJECT_DIR / "models"
MODEL_DIR.mkdir(parents=True, exist_ok=True)

print("=" * 70)
print("GHOSTWIRE — Isolation Forest on REAL Guifi.net Mesh Data")
print("=" * 70)

# ============================================================
# Load data
# ============================================================
print("\nLoading real Guifi.net data...")
X_train = np.load(DATA_DIR / "guifisants_X_train.npy")
y_train = np.load(DATA_DIR / "guifisants_y_train.npy")
X_test = np.load(DATA_DIR / "guifisants_X_test.npy")
y_test = np.load(DATA_DIR / "guifisants_y_test.npy")

print(f"  Train: {X_train.shape[0]:,} samples, {X_train.shape[1]} features")
print(f"  Test:  {X_test.shape[0]:,} samples")
print(f"  Labels: {y_train.sum()} train anomalies, {y_test.sum()} test anomalies")

# ============================================================
# Train Isolation Forest
# ============================================================
print(f"\n{'=' * 70}")
print("TRAINING ISOLATION FOREST")
print(f"{'=' * 70}")

# Use contamination rate from training data
contamination = y_train.sum() / len(y_train)
print(f"  Contamination rate: {contamination:.4f}")

start = time.time()
model = IsolationForest(
    n_estimators=200,
    max_samples=min(8192, len(X_train)),
    contamination=contamination,
    max_features=min(256, X_train.shape[1]),
    random_state=42,
    n_jobs=-1,
    verbose=1,
)

model.fit(X_train)
train_time = time.time() - start
print(f"\nTraining complete in {train_time:.1f}s")

# ============================================================
# Evaluation
# ============================================================
print(f"\n{'=' * 70}")
print("EVALUATION")
print(f"{'=' * 70}")

# Score function returns negative for anomalies, positive for normal
train_scores = model.decision_function(X_train)
test_scores = model.decision_function(X_test)

# Convert to anomaly scores (higher = more anomalous)
train_anomaly_scores = -train_scores
test_anomaly_scores = -test_scores

# ROC-AUC
test_auc = roc_auc_score(y_test, test_anomaly_scores)
test_ap = average_precision_score(y_test, test_anomaly_scores)

# Predictions (-1 for anomaly, 1 for normal)
test_preds = model.predict(X_test)
test_preds_binary = (test_preds == -1).astype(int)

from sklearn.metrics import accuracy_score, f1_score, confusion_matrix
test_acc = accuracy_score(y_test, test_preds_binary)
test_f1 = f1_score(y_test, test_preds_binary, zero_division=0)
cm = confusion_matrix(y_test, test_preds_binary)

print(f"\nTest Results:")
print(f"  ROC-AUC:     {test_auc:.4f}")
print(f"  Avg Prec:    {test_ap:.4f}")
print(f"  Accuracy:    {test_acc:.4f}")
print(f"  F1:          {test_f1:.4f}")
print(f"  Confusion:   {cm.tolist()}")

# Train results
train_auc = roc_auc_score(y_train, train_anomaly_scores)
train_ap = average_precision_score(y_train, train_anomaly_scores)
train_preds = model.predict(X_train)
train_preds_binary = (train_preds == -1).astype(int)
train_f1 = f1_score(y_train, train_preds_binary, zero_division=0)
print(f"\nTrain Results:")
print(f"  ROC-AUC:     {train_auc:.4f}")
print(f"  F1:          {train_f1:.4f}")

# ============================================================
# Benchmark latency
# ============================================================
print(f"\n{'=' * 70}")
print("INFERENCE LATENCY")
print(f"{'=' * 70}")
X_sample = X_test[:1]
for _ in range(10):
    model.decision_function(X_sample)
start = time.perf_counter()
n_runs = 1000
for _ in range(n_runs):
    model.decision_function(X_sample)
elapsed = time.perf_counter() - start
avg_us = (elapsed / n_runs) * 1_000_000
print(f"  Average: {avg_us:.1f}μs per prediction ({n_runs:,} runs)")
print(f"  Target:  < 1000μs (1ms)")
print(f"  {'✅ PASS' if avg_us < 1000 else '⚠️  SLOW'}")

# ============================================================
# Export to JSON for Rust
# ============================================================
print(f"\n{'=' * 70}")
print("EXPORT TO JSON (Rust-compatible)")
print(f"{'=' * 70}")

# Extract tree structures
trees = []
for estimator in model.estimators_:
    tree = estimator.tree_
    tree_data = {
        'children_left': tree.children_left.tolist(),
        'children_right': tree.children_right.tolist(),
        'feature': tree.feature.tolist(),
        'threshold': tree.threshold.tolist(),
        'value': tree.value[:,:,0].tolist(),  # [node, 1] -> [node]
        'n_node_samples': tree.n_node_samples.tolist(),
    }
    trees.append(tree_data)

export_data = {
    'n_estimators': model.n_estimators,
    'n_features': model.n_features_in_,
    'max_samples': int(model.max_samples_),
    'contamination': float(model.contamination),
    'offset': float(model.offset_),
    'trees': trees,
    'feature_importances': [0.0] * model.n_features_in_,  # Placeholder — IF doesn't have direct feature importance
}

json_path = MODEL_DIR / "mesh_anomaly_detector.json"
with open(json_path, 'w') as f:
    json.dump(export_data, f)
size_kb = json_path.stat().st_size / 1024
print(f"  Saved: {json_path} ({size_kb:.1f} KB)")

# Also save metadata
metadata = {
    "dataset": "guifisants_real_mesh",
    "source": "Guifi.net Guifisants node",
    "model_type": "IsolationForest",
    "n_estimators": model.n_estimators,
    "n_features": model.n_features_in_,
    "max_samples": int(model.max_samples_),
    "max_features": int(model.max_features),
    "contamination": float(model.contamination),
    "metrics": {
        "train_auc": float(train_auc),
        "train_f1": float(train_f1),
        "test_auc": float(test_auc),
        "test_ap": float(test_ap),
        "test_f1": float(test_f1),
        "test_accuracy": float(test_acc),
    },
    "training": {
        "time_seconds": float(train_time),
        "inference_latency_us": float(avg_us),
    },
    "layer": "L1 - Unsupervised anomaly detection (complementary to LightGBM)",
}

meta_path = MODEL_DIR / "mesh_anomaly_detector_metadata.json"
with open(meta_path, 'w') as f:
    json.dump(metadata, f, indent=2)
print(f"  Metadata: {meta_path}")

# ============================================================
# Summary
# ============================================================
print(f"\n{'=' * 70}")
print("TRAINING COMPLETE")
print(f"{'=' * 70}")
print(f"  Dataset: Guifi.net Guifisants (real mesh data)")
print(f"  Features: {X_train.shape[1]}")
print(f"  Test AUC: {test_auc:.4f}")
print(f"  Test F1:  {test_f1:.4f}")
print(f"  Inference: {avg_us:.1f}μs")
print(f"\nReplaces: NSL-KDD intrusion detection model (wrong domain)")
print(f"Next: Update Rust to load mesh_anomaly_detector.json")
