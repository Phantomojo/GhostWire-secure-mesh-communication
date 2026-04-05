#!/usr/bin/env python3
"""
GhostWire Anomaly Detector Training
Trains Isolation Forest model on network intrusion detection data
Exports to ONNX for Rust integration
"""

import pandas as pd
import numpy as np
from pathlib import Path
from sklearn.ensemble import IsolationForest
from sklearn.metrics import roc_auc_score, classification_report, confusion_matrix
import joblib
import json

# Configuration
DATA_DIR = Path(__file__).parent.parent / "data" / "processed"
MODEL_DIR = Path(__file__).parent.parent / "models"
MODEL_DIR.mkdir(parents=True, exist_ok=True)

print("=" * 70)
print("GHOSTWIRE ANOMALY DETECTOR TRAINING")
print("=" * 70)

# Load datasets
print("\nLoading datasets...")

datasets = {}
for parquet_file in DATA_DIR.glob("*.parquet"):
    if parquet_file.stem.startswith("nslkdd"):
        print(f"  Loading {parquet_file.name}...")
        df = pd.read_parquet(parquet_file)
        datasets[parquet_file.stem] = df

if not datasets:
    print("  ⚠ No datasets found! Run preprocess_all_datasets.py first")
    exit(1)

# Combine training data
train_data = datasets.get('nslkdd_kddtrain+', None)
if train_data is None:
    print("  ⚠ Training data not found!")
    exit(1)

# Extract features
feature_cols = [
    c for c in train_data.columns 
    if c not in ['attack', 'attack_category', 'is_attack', 'label'] 
    and train_data[c].dtype in ['float64', 'int64']
]

print(f"\nUsing {len(feature_cols)} features:")
print(f"  {feature_cols[:10]}...")  # Show first 10

X_train = train_data[feature_cols].fillna(0).values
y_train = train_data['is_attack'].values

# Add test data if available
if 'nslkdd_kddtest+' in datasets:
    test_data = datasets['nslkdd_kddtest+']
    X_test = test_data[feature_cols].fillna(0).values
    y_test = test_data['is_attack'].values
    print(f"\nTest set: {len(X_test):,} samples")
else:
    X_test, y_test = None, None

print(f"\nTraining set: {len(X_train):,} samples")
print(f"Attacks: {y_train.sum():,} ({100*y_train.mean():.1f}%)")

# Train Isolation Forest
print("\n" + "=" * 70)
print("TRAINING ISOLATION FOREST")
print("=" * 70)

model = IsolationForest(
    n_estimators=200,          # Number of trees
    contamination=0.1,         # Expected proportion of outliers
    max_samples='auto',        # Number of samples to draw
    max_features=1.0,          # Number of features to draw
    bootstrap=False,           # Sample with replacement
    n_jobs=-1,                 # Use all CPU cores
    random_state=42,           # Reproducibility
    verbose=1                  # Show progress
)

print("\nFitting model...")
model.fit(X_train)

# Evaluate
print("\n" + "=" * 70)
print("EVALUATION")
print("=" * 70)

# Training set evaluation
y_pred_train = model.predict(X_train)
y_scores_train = -model.score_samples(X_train)  # Higher = more anomalous

print(f"\nTraining Set:")
print(f"  ROC-AUC: {roc_auc_score(y_train, y_scores_train):.4f}")
print(f"\nClassification Report:")
# Get unique labels for proper report
unique_labels = sorted(np.unique(y_pred_train))
print(classification_report(y_train, y_pred_train, labels=unique_labels))

# Test set evaluation
if X_test is not None:
    y_pred_test = model.predict(X_test)
    y_scores_test = -model.score_samples(X_test)
    
    print(f"\nTest Set:")
    print(f"  ROC-AUC: {roc_auc_score(y_test, y_scores_test):.4f}")
    print(f"\nClassification Report:")
    unique_labels_test = sorted(np.unique(y_pred_test))
    print(classification_report(y_test, y_pred_test, labels=unique_labels_test))
    
    # Confusion matrix
    cm = confusion_matrix(y_test, y_pred_test)
    print(f"\nConfusion Matrix:")
    print(f"                Predicted")
    print(f"                Normal  Attack")
    print(f"  Actual Normal   {cm[0,0]:6d}  {cm[0,1]:6d}")
    print(f"  Actual Attack   {cm[1,0]:6d}  {cm[1,1]:6d}")

# Save model
print("\n" + "=" * 70)
print("SAVING MODEL")
print("=" * 70)

# Joblib format (Python)
joblib_path = MODEL_DIR / "anomaly_detector.joblib"
joblib.dump(model, joblib_path)
print(f"  ✓ Saved: {joblib_path}")
print(f"    Size: {joblib_path.stat().st_size / 1024 / 1024:.2f} MB")

# Export to ONNX (for Rust integration)
try:
    from skl2onnx import convert_sklearn
    from skl2onnx.common.data_types import FloatTensorType
    
    print("\nExporting to ONNX...")
    
    initial_type = [('float_input', FloatTensorType([None, len(feature_cols)]))]
    onnx_model = convert_sklearn(
        model, 
        initial_types=initial_type,
        target_opset=15
    )
    
    onnx_path = MODEL_DIR / "anomaly_detector.onnx"
    with open(onnx_path, "wb") as f:
        f.write(onnx_model.SerializeToString())
    
    print(f"  ✓ Saved: {onnx_path}")
    print(f"    Size: {onnx_path.stat().st_size / 1024 / 1024:.2f} MB")
    
except Exception as e:
    print(f"\n  ⚠ ONNX export failed: {e}")
    print("    Model saved in joblib format (can be used directly)")

# Save metadata
metadata = {
    'model_type': 'IsolationForest',
    'feature_count': len(feature_cols),
    'feature_names': feature_cols,
    'training_samples': len(X_train),
    'attack_ratio': float(y_train.mean()),
    'n_estimators': 200,
    'contamination': 0.1,
    'roc_auc': float(roc_auc_score(y_train, y_scores_train)),
}

metadata_path = MODEL_DIR / "anomaly_detector_metadata.json"
with open(metadata_path, 'w') as f:
    json.dump(metadata, f, indent=2)
print(f"  ✓ Saved: {metadata_path}")

print("\n" + "=" * 70)
print("✅ TRAINING COMPLETE")
print("=" * 70)
print(f"\nModel files:")
print(f"  - {joblib_path}")
print(f"  - {onnx_path if onnx_path.exists() else 'ONNX not exported'}")
print(f"  - {metadata_path}")
print(f"\nReady for integration into GhostWire!")
