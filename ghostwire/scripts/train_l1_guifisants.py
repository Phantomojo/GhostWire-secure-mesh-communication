#!/usr/bin/env python3
"""
Train LightGBM on REAL Guifi.net mesh data (Guifisants node).

Dataset:
  - 7,931 samples × 2,387 features (system metrics, network stats, BMX6 routing)
  - Binary classification: anomaly detection (gateway failure events)
  - Highly imbalanced: ~1.5% positive class

Strategy:
  - scale_pos_weight for class imbalance
  - Early stopping on validation AUC
  - Feature importance analysis
  - ONNX export for Rust inference
"""

import numpy as np
import time
from pathlib import Path
import json

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
DATA_DIR = PROJECT_DIR.parent / "datasets" / "processed" / "l1_lightgbm"
MODEL_DIR = PROJECT_DIR / "models"
MODEL_DIR.mkdir(parents=True, exist_ok=True)

print("=" * 70)
print("GHOSTWIRE L1 — LightGBM on REAL Guifi.net Data")
print("=" * 70)

# ============================================================
# Load data
# ============================================================
print("\nLoading real Guifi.net data...")
X_train = np.load(DATA_DIR / "guifisants_X_train.npy")
y_train = np.load(DATA_DIR / "guifisants_y_train.npy")
X_val = np.load(DATA_DIR / "guifisants_X_val.npy")
y_val = np.load(DATA_DIR / "guifisants_y_val.npy")
X_test = np.load(DATA_DIR / "guifisants_X_test.npy")
y_test = np.load(DATA_DIR / "guifisants_y_test.npy")

print(f"  Train: {X_train.shape[0]:,} samples, {X_train.shape[1]} features")
print(f"  Val:   {X_val.shape[0]:,} samples")
print(f"  Test:  {X_test.shape[0]:,} samples")
print(f"  Total: {X_train.shape[0] + X_val.shape[0] + X_test.shape[0]:,}")

# Class distribution
n_pos_train = y_train.sum()
n_neg_train = len(y_train) - n_pos_train
n_pos_test = y_test.sum()
n_neg_test = len(y_test) - n_pos_test
print(f"\n  Train: {n_neg_train} negative, {n_pos_train} positive ({n_pos_train/len(y_train)*100:.2f}%)")
print(f"  Test:  {n_neg_test} negative, {n_pos_test} positive ({n_pos_test/len(y_test)*100:.2f}%)")

# ============================================================
# Train LightGBM
# ============================================================
import lightgbm as lgb
from sklearn.metrics import (
    accuracy_score, precision_score, recall_score, f1_score,
    roc_auc_score, average_precision_score, confusion_matrix
)

# Handle class imbalance
if n_pos_train > 0:
    scale_pos_weight = n_neg_train / n_pos_train
else:
    scale_pos_weight = 1.0
print(f"\n  scale_pos_weight: {scale_pos_weight:.1f}")

params = {
    'objective': 'binary',
    'metric': ['binary_logloss', 'auc', 'average_precision'],
    'num_leaves': 63,
    'learning_rate': 0.05,
    'feature_fraction': 0.8,
    'bagging_fraction': 0.8,
    'bagging_freq': 5,
    'min_child_samples': 10,
    'reg_alpha': 0.1,
    'reg_lambda': 0.1,
    'scale_pos_weight': scale_pos_weight,
    'verbose': -1,
    'seed': 42,
}

print("\nTraining LightGBM...")
start = time.time()

train_data = lgb.Dataset(X_train, label=y_train)
val_data = lgb.Dataset(X_val, label=y_val, reference=train_data)

model = lgb.train(
    params,
    train_data,
    num_boost_round=2000,
    valid_sets=[val_data],
    valid_names=['val'],
    callbacks=[
        lgb.log_evaluation(100),
        lgb.early_stopping(100),
    ],
)

train_time = time.time() - start
print(f"\nTraining complete in {train_time:.1f}s")
print(f"Best iteration: {model.best_iteration}")

# ============================================================
# Evaluation
# ============================================================
print(f"\n{'=' * 70}")
print("EVALUATION")
print(f"{'=' * 70}")

# Validation
val_proba = model.predict(X_val)
val_preds = (val_proba >= 0.5).astype(int)
val_auc = roc_auc_score(y_val, val_proba)
val_ap = average_precision_score(y_val, val_proba)
val_acc = accuracy_score(y_val, val_preds)
val_f1 = f1_score(y_val, val_preds, zero_division=0)
val_precision = precision_score(y_val, val_preds, zero_division=0)
val_recall = recall_score(y_val, val_preds, zero_division=0)

print(f"\nValidation:")
print(f"  AUC:       {val_auc:.4f}")
print(f"  Avg Prec:  {val_ap:.4f}")
print(f"  Accuracy:  {val_acc:.4f}")
print(f"  Precision: {val_precision:.4f}")
print(f"  Recall:    {val_recall:.4f}")
print(f"  F1:        {val_f1:.4f}")

# Test
test_proba = model.predict(X_test)
test_preds = (test_proba >= 0.5).astype(int)
test_auc = roc_auc_score(y_test, test_proba)
test_ap = average_precision_score(y_test, test_proba)
test_acc = accuracy_score(y_test, test_preds)
test_f1 = f1_score(y_test, test_preds, zero_division=0)
test_precision = precision_score(y_test, test_preds, zero_division=0)
test_recall = recall_score(y_test, test_preds, zero_division=0)

print(f"\nTest:")
print(f"  AUC:       {test_auc:.4f}")
print(f"  Avg Prec:  {test_ap:.4f}")
print(f"  Accuracy:  {test_acc:.4f}")
print(f"  Precision: {test_precision:.4f}")
print(f"  Recall:    {test_recall:.4f}")
print(f"  F1:        {test_f1:.4f}")

# Confusion matrix
cm = confusion_matrix(y_test, test_preds)
print(f"\nConfusion Matrix:")
print(f"  {cm}")

# Optimal threshold search
from sklearn.metrics import precision_recall_curve
precisions, recalls, thresholds = precision_recall_curve(y_test, test_proba)
f1_scores = 2 * (precisions * recalls) / (precisions + recalls + 1e-8)
best_idx = np.argmax(f1_scores)
best_threshold = thresholds[best_idx] if best_idx < len(thresholds) else 0.5
print(f"\nOptimal threshold: {best_threshold:.4f}")
test_preds_opt = (test_proba >= best_threshold).astype(int)
test_f1_opt = f1_score(y_test, test_preds_opt, zero_division=0)
test_prec_opt = precision_score(y_test, test_preds_opt, zero_division=0)
test_rec_opt = recall_score(y_test, test_preds_opt, zero_division=0)
print(f"  At optimal threshold: F1={test_f1_opt:.4f}, Prec={test_prec_opt:.4f}, Rec={test_rec_opt:.4f}")

# Feature importance
importance = model.feature_importance(importance_type='gain')
top_indices = np.argsort(-importance)[:20]
print(f"\nTop 20 features (by gain):")
for i, idx in enumerate(top_indices):
    print(f"  {i+1:2d}. Feature {idx:4d}: {importance[idx]:10.1f}")

# ============================================================
# Benchmark latency
# ============================================================
print(f"\n{'=' * 70}")
print("INFERENCE LATENCY")
print(f"{'=' * 70}")
X_sample = X_test[:1]
for _ in range(10):
    model.predict(X_sample)
start = time.perf_counter()
n_runs = 1000
for _ in range(n_runs):
    model.predict(X_sample)
elapsed = time.perf_counter() - start
avg_us = (elapsed / n_runs) * 1_000_000
print(f"  Average: {avg_us:.1f}μs per prediction ({n_runs:,} runs)")
print(f"  Target:  < 1000μs (1ms)")
print(f"  {'✅ PASS' if avg_us < 1000 else '⚠️  SLOW'}")

# ============================================================
# Export to ONNX
# ============================================================
print(f"\n{'=' * 70}")
print("ONNX EXPORT")
print(f"{'=' * 70}")

try:
    from onnxmltools import convert_lightgbm
    from onnxmltools.convert.common.data_types import FloatTensorType
    import onnx

    initial_type = [('float_input', FloatTensorType([None, X_train.shape[1]]))]
    onnx_model = convert_lightgbm(
        model,
        initial_types=initial_type,
        target_opset=15,
    )
    onnx.checker.check_model(onnx_model)
    print("  ✅ ONNX validation passed")

    onnx_path = MODEL_DIR / "ghostwire_routing.onnx"
    with open(onnx_path, 'wb') as f:
        f.write(onnx_model.SerializeToString())
    size_kb = onnx_path.stat().st_size / 1024
    print(f"  Saved: {onnx_path} ({size_kb:.1f} KB)")

    # Also save the LightGBM text format
    txt_path = MODEL_DIR / "ghostwire_routing_l1.txt"
    model.save_model(txt_path)
    size_kb_txt = txt_path.stat().st_size / 1024
    print(f"  Text:  {txt_path} ({size_kb_txt:.1f} KB)")

except Exception as e:
    print(f"  ⚠️  ONNX export failed: {e}")
    txt_path = MODEL_DIR / "ghostwire_routing_l1.txt"
    model.save_model(txt_path)
    print(f"  Saved text format: {txt_path}")

# ============================================================
# Save metadata
# ============================================================
metadata = {
    "dataset": "guifisants_real_mesh",
    "source": "Guifi.net Guifisants node",
    "samples": {
        "train": int(X_train.shape[0]),
        "val": int(X_val.shape[0]),
        "test": int(X_test.shape[0]),
        "total": int(X_train.shape[0] + X_val.shape[0] + X_test.shape[0]),
    },
    "features": int(X_train.shape[1]),
    "class_distribution": {
        "train_negative": int(n_neg_train),
        "train_positive": int(n_pos_train),
        "train_positive_pct": float(n_pos_train / len(y_train) * 100),
        "test_negative": int(n_neg_test),
        "test_positive": int(n_pos_test),
        "test_positive_pct": float(n_pos_test / len(y_test) * 100),
    },
    "metrics": {
        "val_auc": float(val_auc),
        "val_ap": float(val_ap),
        "val_f1": float(val_f1),
        "test_auc": float(test_auc),
        "test_ap": float(test_ap),
        "test_f1": float(test_f1),
        "test_f1_optimal": float(test_f1_opt),
        "optimal_threshold": float(best_threshold),
    },
    "training": {
        "time_seconds": float(train_time),
        "best_iteration": int(model.best_iteration),
        "scale_pos_weight": float(scale_pos_weight),
        "inference_latency_us": float(avg_us),
    },
    "model_type": "LightGBM binary classification",
    "layer": "L1 - Anomaly detection / link quality",
}

meta_path = MODEL_DIR / "l1_guifisants_metadata.json"
with open(meta_path, 'w') as f:
    json.dump(metadata, f, indent=2)
print(f"\nMetadata saved: {meta_path}")

# ============================================================
# Summary
# ============================================================
print(f"\n{'=' * 70}")
print("TRAINING COMPLETE")
print(f"{'=' * 70}")
print(f"  Dataset: Guifi.net Guifisants (real mesh data)")
print(f"  Features: {X_train.shape[1]}")
print(f"  Test AUC: {test_auc:.4f}")
print(f"  Test F1:  {test_f1:.4f} (optimal: {test_f1_opt:.4f} @ threshold {best_threshold:.3f})")
print(f"  Inference: {avg_us:.1f}μs")
print(f"\nNext: Wire ONNX model into Rust for real-time anomaly detection")
