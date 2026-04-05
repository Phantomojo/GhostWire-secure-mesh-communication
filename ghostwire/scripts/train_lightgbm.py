#!/usr/bin/env python3
"""
Train LightGBM routing model for GhostWire mesh network.

Input: 7 numerical features → delivery_probability (regression)
Output: ONNX model for Rust inference via ort crate

This replaces the broken HRM transformer (2.69% accuracy).
Expected accuracy: 85-95% (RMSE < 0.15).
"""

import numpy as np
import pandas as pd
from pathlib import Path
import json
import time

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
DATA_DIR = PROJECT_DIR / "data" / "lightgbm-routing"
MODEL_DIR = PROJECT_DIR / "models"

FEATURES = [
    'rssi',
    'latency_ms',
    'packet_loss',
    'battery_level',
    'hop_count',
    'encounter_frequency',
    'time_since_last_seen',
]
TARGET = 'delivery_probability'


def load_data():
    """Load and split dataset."""
    csv_path = DATA_DIR / "routing_dataset.csv"
    if not csv_path.exists():
        print(f"❌ Dataset not found: {csv_path}")
        print("Run: python3 scripts/generate_lightgbm_dataset.py first")
        return None

    df = pd.read_csv(csv_path)
    print(f"Loaded {len(df):,} samples, {len(FEATURES)} features")

    # Train/val/test split: 80/10/10
    n = len(df)
    df = df.sample(frac=1, random_state=42).reset_index(drop=True)
    train_end = int(0.8 * n)
    val_end = int(0.9 * n)

    train_df = df.iloc[:train_end]
    val_df = df.iloc[train_end:val_end]
    test_df = df.iloc[val_end:]

    print(f"Train: {len(train_df):,} | Val: {len(val_df):,} | Test: {len(test_df):,}")
    return train_df, val_df, test_df


def train_lightgbm(train_df, val_df):
    """Train LightGBM regression model."""
    import lightgbm as lgb
    from sklearn.metrics import mean_squared_error, mean_absolute_error, r2_score

    X_train = train_df[FEATURES].values
    y_train = train_df[TARGET].values
    X_val = val_df[FEATURES].values
    y_val = val_df[TARGET].values

    params = {
        'objective': 'regression',
        'metric': 'rmse',
        'num_leaves': 63,
        'learning_rate': 0.05,
        'feature_fraction': 0.9,
        'bagging_fraction': 0.8,
        'bagging_freq': 5,
        'min_child_samples': 20,
        'reg_alpha': 0.1,
        'reg_lambda': 0.1,
        'verbose': -1,
        'seed': 42,
    }

    train_data = lgb.Dataset(X_train, label=y_train, feature_name=FEATURES)
    val_data = lgb.Dataset(X_val, label=y_val, feature_name=FEATURES, reference=train_data)

    print("\nTraining LightGBM...")
    start = time.time()

    model = lgb.train(
        params,
        train_data,
        num_boost_round=1000,
        valid_sets=[val_data],
        callbacks=[
            lgb.log_evaluation(100),
            lgb.early_stopping(50),
        ],
    )

    train_time = time.time() - start
    print(f"\nTraining complete in {train_time:.1f}s")
    print(f"Best iteration: {model.best_iteration}")

    # Validation metrics
    val_preds = model.predict(X_val)
    val_rmse = np.sqrt(mean_squared_error(y_val, val_preds))
    val_mae = mean_absolute_error(y_val, val_preds)
    val_r2 = r2_score(y_val, val_preds)

    print(f"\n{'=' * 50}")
    print(f"VALIDATION METRICS")
    print(f"{'=' * 50}")
    print(f"RMSE:  {val_rmse:.4f}  (target: < 0.15)")
    print(f"MAE:   {val_mae:.4f}")
    print(f"R²:    {val_r2:.4f}  (target: > 0.85)")

    # Feature importance
    importance = model.feature_importance(importance_type='gain')
    print(f"\nFeature importance (gain):")
    for feat, imp in sorted(zip(FEATURES, importance), key=lambda x: -x[1]):
        print(f"  {feat:25s} {imp:8.1f}")

    return model, val_rmse, val_r2


def evaluate_test(model, test_df):
    """Evaluate on held-out test set."""
    from sklearn.metrics import mean_squared_error, mean_absolute_error, r2_score

    X_test = test_df[FEATURES].values
    y_test = test_df[TARGET].values

    preds = model.predict(X_test)
    rmse = np.sqrt(mean_squared_error(y_test, preds))
    mae = mean_absolute_error(y_test, preds)
    r2 = r2_score(y_test, preds)

    print(f"\n{'=' * 50}")
    print(f"TEST SET METRICS")
    print(f"{'=' * 50}")
    print(f"RMSE:  {rmse:.4f}")
    print(f"MAE:   {mae:.4f}")
    print(f"R²:    {r2:.4f}")

    # Prediction range check
    print(f"\nPrediction range: [{preds.min():.4f}, {preds.max():.4f}]")
    out_of_range = ((preds < 0) | (preds > 1)).sum()
    if out_of_range > 0:
        print(f"⚠️  {out_of_range} predictions out of [0,1] range ({out_of_range/len(preds)*100:.1f}%)")
    else:
        print(f"✅ All predictions in [0,1] range")

    return rmse, r2


def benchmark_latency(model, test_df, n_runs=1000):
    """Benchmark inference latency."""
    X = test_df[FEATURES].values[:1]  # Single sample

    # Warmup
    for _ in range(10):
        model.predict(X)

    # Benchmark
    start = time.perf_counter()
    for _ in range(n_runs):
        model.predict(X)
    elapsed = time.perf_counter() - start

    avg_us = (elapsed / n_runs) * 1_000_000
    print(f"\n{'=' * 50}")
    print(f"INFERENCE LATENCY")
    print(f"{'=' * 50}")
    print(f"Average: {avg_us:.1f}μs per prediction ({n_runs:,} runs)")
    print(f"Target:  < 1000μs (1ms)")
    if avg_us < 1000:
        print(f"✅ PASS")
    else:
        print(f"⚠️  SLOW — may need model simplification")


def export_to_onnx(model):
    """Export LightGBM model to ONNX format."""
    try:
        from onnxmltools import convert_lightgbm
        from onnxmltools.convert.common.data_types import FloatTensorType
        import onnx
    except ImportError:
        print("\n⚠️  onnxmltools not installed. Skipping ONNX export.")
        print("Install: pip install onnxmltools")
        return False

    print(f"\n{'=' * 50}")
    print(f"ONNX EXPORT")
    print(f"{'=' * 50}")

    initial_type = [('float_input', FloatTensorType([None, len(FEATURES)]))]

    try:
        onnx_model = convert_lightgbm(
            model,
            initial_types=initial_type,
            target_opset=15,
        )

        onnx.checker.check_model(onnx_model)
        print("✅ ONNX validation passed")

        onnx_path = MODEL_DIR / "ghostwire_routing_lightgbm.onnx"
        with open(onnx_path, 'wb') as f:
            f.write(onnx_model.SerializeToString())

        size_kb = onnx_path.stat().st_size / 1024
        print(f"Saved: {onnx_path} ({size_kb:.1f} KB)")
        print(f"(vs 7.9MB for the broken HRM model)")
        return True

    except Exception as e:
        print(f"⚠️  ONNX export failed: {e}")
        print("Model saved as LightGBM text format instead.")

        # Save as LightGBM text format
        txt_path = MODEL_DIR / "ghostwire_routing_lightgbm.txt"
        model.save_model(txt_path)
        size_kb = txt_path.stat().st_size / 1024
        print(f"Saved: {txt_path} ({size_kb:.1f} KB)")
        return False


def main():
    print("=" * 70)
    print("GHOSTWIRE LIGHTGBM ROUTING MODEL TRAINER")
    print("=" * 70)

    # Load data
    result = load_data()
    if result is None:
        return
    train_df, val_df, test_df = result

    # Train
    model, val_rmse, val_r2 = train_lightgbm(train_df, val_df)

    # Validate
    if val_rmse > 0.15:
        print(f"\n⚠️  RMSE {val_rmse:.4f} > 0.15 target — model may need tuning")
    if val_r2 < 0.85:
        print(f"\n⚠️  R² {val_r2:.4f} < 0.85 target — model may need tuning")

    # Test evaluation
    test_rmse, test_r2 = evaluate_test(model, test_df)

    # Benchmark
    benchmark_latency(model, test_df)

    # Export
    onnx_ok = export_to_onnx(model)

    # Summary
    print(f"\n{'=' * 70}")
    print(f"TRAINING COMPLETE")
    print(f"{'=' * 70}")
    print(f"Test RMSE: {test_rmse:.4f}  |  Test R²: {test_r2:.4f}")
    print(f"ONNX export: {'✅' if onnx_ok else '⚠️  (text format)'}")
    print(f"\nCurrent HRM accuracy: 2.69%")
    print(f"New LightGBM accuracy: ~{(1 - test_rmse) * 100:.1f}% (RMSE proxy)")
    print(f"\nNext: Wire ONNX model into Rust onnx_inference.rs")


if __name__ == "__main__":
    main()
