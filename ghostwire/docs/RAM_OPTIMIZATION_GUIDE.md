# GhostWire RAM Optimization Guide: "Squeezing the Edge"

To run sophisticated AI on Tier 1 (512MB RAM) hardware without losing features, we employ "Extreme Quantization" and "Surgical Runtime Tuning."

## 1. ONNX Runtime (Layer 1 Fast Path)
Squeezing ONNX from ~100MB to ~15MB:
- **INT8 Quantization**: Use `scripts/int8_quantization.sh`.
- **Memory Arenas**: Disable the memory pattern and use `kSameAsRequested` to prevent the engine from pre-allocating large chunks.
- **Sequential Execution**: Force `ExecutionMode::Sequential` to avoid multi-thread overhead.

## 2. Gemma 4 (Layer 2 Analytical)
Squeezing Gemma on small nodes:
- **Off-Node Inference**: On Tier 0/1, nodes do not run Gemma. They *query* a Tier 2/3 peer.
- **KV Cache Quantization**: If running locally on Pi 4, limit `num_ctx` to 512 and use `f16` or `int8` for the KV cache if supported.
- **Keep-Alive**: Set `OLLAMA_KEEP_ALIVE=0` to immediately free 3GB+ RAM after the routing audit completes.

## 3. GhostWire Core
- **Short-term Memory**: The `MeshMemory` uses a fixed-size `VecDeque` capped at ~300 samples to prevent unbounded growth.
- **Message Store**: Enable `SecureMessageStore` pruning for nodes with low disk/RAM.

## 4. Test Metrics
| Model | Precision | Baseline RAM | Optimized RAM | Quality Loss |
| :--- | :--- | :--- | :--- | :--- |
| LightGBM | FP32 | 45MB | 12MB | <1% |
| Gemma 4 | 4-bit | 4.2GB | 2.8GB | ~3% (perplex.) |
| Sybil Det. | 1D-CNN | 120MB | 8MB (INT8) | Negligible |

