#!/usr/bin/env python3
"""
GhostWire Data Preprocessing Pipeline
Prepares all datasets for AI model training
"""

import pandas as pd
import numpy as np
from pathlib import Path
import sys

# Configuration
DATA_ROOT = Path(__file__).parent.parent / "data"
OUTPUT_DIR = DATA_ROOT / "processed"
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

print("=" * 70)
print("GHOSTWIRE DATA PREPROCESSING PIPELINE")
print("=" * 70)

def preprocess_nslkdd():
    """Preprocess NSL-KDD dataset"""
    print("\n[1/4] Processing NSL-KDD...")
    
    nslkdd_dir = DATA_ROOT / "nsl-kdd"
    if not nslkdd_dir.exists():
        print("  ⚠ NSL-KDD not found, skipping...")
        return False
    
    for split in ["KDDTrain+", "KDDTest+"]:
        file_path = nslkdd_dir / f"{split}.txt"
        if not file_path.exists():
            continue
        
        # NSL-KDD has no header - need to provide column names
        column_names = [
            'duration', 'protocol_type', 'service', 'flag', 'src_bytes',
            'dst_bytes', 'land', 'wrong_fragment', 'urgent', 'hot',
            'num_failed_logins', 'logged_in', 'num_compromised', 'root_shell',
            'su_attempted', 'num_root', 'num_file_creations', 'num_shells',
            'num_access_files', 'num_outbound_cmds', 'is_host_login',
            'is_guest_login', 'count', 'srv_count', 'serror_rate', 'srv_serror_rate',
            'rerror_rate', 'srv_rerror_rate', 'same_srv_rate', 'diff_srv_rate',
            'srv_diff_host_rate', 'dst_host_count', 'dst_host_srv_count',
            'dst_host_same_srv_rate', 'dst_host_diff_srv_rate',
            'dst_host_same_src_port_rate', 'dst_host_srv_diff_host_rate',
            'dst_host_serror_rate', 'dst_host_srv_serror_rate',
            'dst_host_rerror_rate', 'dst_host_srv_rerror_rate',
            'attack', 'level'  # Last two columns
        ]
            
        df = pd.read_csv(file_path, header=None, names=column_names)
        
        # Encode categorical features
        for col in ['protocol_type', 'service', 'flag']:
            df[col] = df[col].astype('category').cat.codes
        
        # Binary classification (normal vs attack)
        df['is_attack'] = (df['attack'] != 'normal').astype(int)
        
        # Save
        output_file = OUTPUT_DIR / f"nslkdd_{split.lower()}.parquet"
        df.to_parquet(output_file, index=False)
        print(f"  ✓ {split}: {len(df):,} samples, {df['is_attack'].sum():,} attacks ({100*df['is_attack'].mean():.1f}%)")
    
    return True

def preprocess_network_traffic():
    """Preprocess Network Traffic dataset"""
    print("\n[2/4] Processing Network Traffic...")
    
    # Files are in data/ root, not subdirectory
    traffic_dir = DATA_ROOT
    
    csv_file = traffic_dir / "output1.csv"
    if csv_file.exists():
        df = pd.read_csv(csv_file)
        
        # Handle missing values
        df = df.fillna(0)
        
        # Encode categorical
        for col in df.select_dtypes(['object']).columns:
            df[col] = df[col].astype('category').cat.codes
        
        # Save
        output_file = OUTPUT_DIR / "network_traffic.parquet"
        df.to_parquet(output_file, index=False)
        print(f"  ✓ Saved: {len(df):,} samples")
        return True
    else:
        print("  ⚠ output1.csv not found")
        return False

def preprocess_ids2025():
    """Preprocess IDS 2025 dataset"""
    print("\n[3/4] Processing IDS 2025...")
    
    # Files are in data/ root, not subdirectory
    ids_dir = DATA_ROOT
    
    import glob
    csv_files = list(ids_dir.glob("*.csv"))
    
    # Filter for IDS 2025 files (attack type CSVs)
    ids_files = [f for f in csv_files if 'Midterm' not in str(f) and 'output' not in str(f)]
    
    if not ids_files:
        print("  ⚠ No IDS 2025 CSV files found")
        return False
    
    print(f"  Found {len(ids_files)} IDS 2025 CSV files...")
    dfs = []
    for file in ids_files[:5]:  # Limit to first 5 for speed
        print(f"    Loading {file.name}...")
        try:
            df = pd.read_csv(file)
            dfs.append(df)
        except Exception as e:
            print(f"    ⚠ Error loading {file.name}: {e}")
    
    if not dfs:
        print("  ⚠ No data loaded")
        return False
    
    df = pd.concat(dfs, ignore_index=True)
    print(f"  Combined: {len(df):,} samples")
    
    # Handle missing values
    df = df.fillna(0)
    
    # Encode categorical columns
    for col in df.select_dtypes(['object']).columns:
        if col != 'label' and col != 'attack_type':
            df[col] = df[col].astype('category').cat.codes
    
    # Create binary classification
    if 'label' in df.columns:
        df['is_attack'] = (df['label'] != 'normal').astype(int)
    elif 'attack_type' in df.columns:
        df['is_attack'] = (df['attack_type'] != 'normal').astype(int)
    else:
        # No label column - create dummy
        df['is_attack'] = 0
    
    # Save
    output_file = OUTPUT_DIR / "ids2025.parquet"
    df.to_parquet(output_file, index=False)
    print(f"  ✓ Saved: {len(df):,} samples")
    
    if 'is_attack' in df.columns:
        print(f"    Attacks: {df['is_attack'].sum():,} ({100*df['is_attack'].mean():.1f}%)")
    
    return True

def create_feature_summary():
    """Create summary of all preprocessed features"""
    print("\n[4/4] Creating feature summary...")
    
    summary = {
        'datasets': [],
        'total_samples': 0,
        'total_attacks': 0,
    }
    
    for parquet_file in OUTPUT_DIR.glob("*.parquet"):
        df = pd.read_parquet(parquet_file)
        
        feature_cols = [c for c in df.columns if c not in ['attack', 'attack_category', 'is_attack', 'label']]
        
        dataset_info = {
            'name': parquet_file.stem,
            'samples': len(df),
            'features': len(feature_cols),
            'attacks': int(df['is_attack'].sum()) if 'is_attack' in df.columns else 0,
        }
        
        summary['datasets'].append(dataset_info)
        summary['total_samples'] += len(df)
        summary['total_attacks'] += dataset_info['attacks']
        
        print(f"  ✓ {dataset_info['name']}: {dataset_info['samples']:,} samples, {dataset_info['features']} features")
    
    # Save summary
    import json
    summary_file = OUTPUT_DIR / "dataset_summary.json"
    with open(summary_file, 'w') as f:
        json.dump(summary, f, indent=2)
    
    print(f"\n{'='*70}")
    print(f"TOTAL: {summary['total_samples']:,} samples, {summary['total_attacks']:,} attacks")
    print(f"{'='*70}")
    
    return summary

def main():
    """Run complete preprocessing pipeline"""
    
    results = {
        'nslkdd': preprocess_nslkdd(),
        'network_traffic': preprocess_network_traffic(),
        'ids2025': preprocess_ids2025(),
    }
    
    summary = create_feature_summary()
    
    print("\n" + "=" * 70)
    print("PREPROCESSING COMPLETE")
    print("=" * 70)
    
    success_count = sum(results.values())
    print(f"Datasets processed: {success_count}/3")
    print(f"Output directory: {OUTPUT_DIR.absolute()}")
    print(f"Files created: {len(list(OUTPUT_DIR.glob('*.parquet')))}")
    
    if success_count == 0:
        print("\n⚠ WARNING: No datasets were processed!")
        print("  Check that data files exist in data/ directory")
        sys.exit(1)
    
    print("\n✅ Ready for model training!")
    return summary

if __name__ == "__main__":
    main()
