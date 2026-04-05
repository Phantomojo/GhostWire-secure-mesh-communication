## Qwen Added Memories
- GhostWire Agent Context System installed 2026-04-04. Files: AGENTS.md (universal context), CLAUDE.md, .ai/memory.md (cross-session memory), .agents/skills/ (3 skills: ai-layer, security-audit, ml-training), .qwen/system.md (Qwen OS instructions). Global: ~/.qwen/phantomojo-global.md, ~/.claude/CLAUDE.md. Install script at /tmp/ghostwire-agent-context/install.sh.
- GhostWire AI Architecture — 4 layers: L1 LightGBM/ONNX (<1ms, routing decision), L2 Gemma4/Ollama (30s, mesh analysis), L3 GNN (5min, routing optimization), L4 PRoPHET (fallback, working in dtn/epidemic.rs). Existing systems to WIRE not duplicate: anomaly_detector.joblib (Isolation Forest), reputation.rs+TrustStore (trust scoring), dtn/epidemic.rs (PRoPHET). HRM model (hrm_ghostwire_hybrid_101k.pt) is BROKEN — 2.69% accuracy, wrong task (classifier not regressor). Target: 85-95% via LightGBM regression. Free GPU: Kaggle 30hr/wk P100, Colab 15-30hr/wk T4, Lightning AI 80hr/mo L40s.
- Kaggle CLI workflow for GNN training:
1. Credentials: ~/.kaggle/kaggle.json (username + key) — needs periodic regeneration from kaggle.com/settings → API → Create New Token
2. CLI commands:
   - `kaggle kernels init -p <dir>` — create kernel metadata
   - Edit kernel-metadata.json: set id, title, code_file, language=python, kernel_type=script, enable_gpu=true, enable_internet=true
   - `kaggle kernels push -p <dir> --accelerator gpu` — push and run
   - `kaggle kernels status -p <dir>` — check run status
   - `kaggle kernels output -p <dir>` — get training output
   - `kaggle kernels pull -p <dir>` — download results
3. Training script: scripts/train_gnn_kaggle.py — complete GAT training with pip install, dataset gen, training, and model save to /kaggle/working/models/gnn_routing/
4. Current API key (93b1b329...) is expired — needs regeneration
- Kaggle API token: KGAT_3f2c7cb3f5a84b3b5242991f5ac916d8 (ACCESS_TOKEN auth method, set via KAGGLE_API_TOKEN env var or kaggle config set -n key). CLI workflow: `kaggle kernels push -p /tmp/kaggle-kernel --accelerator gpu --timeout 3600` pushes and runs. Status: `kaggle kernels status phantomojo/ghostwire-gnn-routing-model-training`. Output: `kaggle kernels output phantomojo/ghostwire-gnn-routing-model-training`. Kernel metadata at /tmp/kaggle-kernel/kernel-metadata.json. Training script at /tmp/kaggle-kernel/train_gnn.py (copied from scripts/train_gnn_kaggle.py).
- Kaggle API token: KGAT_9a63abbf734af58321ff7deea101c0e1 (set via KAGGLE_API_TOKEN env var). Replaces expired KGAT_3f2c7cb3f5a84b3b5242991f5ac916d8.
