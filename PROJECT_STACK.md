# GhostWire Project Technology Stack

This document lists all languages, tools, and dependencies used in the GhostWire project.

## Languages
- **Rust**: Core backend, mesh networking, and crypto.
- **TypeScript / JavaScript**: Frontend (React) and Tauri integration.
- **Python**: ML model training, data processing, and auxiliary scripts.
- **C / C++**: ESP32 firmware development (ESP-IDF).
- **SQL**: Database queries (via `sled` or external DBs).
- **Markdown**: Documentation.
- **Shell (Bash)**: Automation scripts.

## Core Frameworks & Libraries

### Backend (Rust)
- **Networking**: `libp2p` (P2P), `tokio` (Async runtime), `axum` (Web server).
- **Cryptography**: `vodozemac` (Olm/Megolm), `ring`, `ed25519-dalek`, `x25519-dalek`, `aes-gcm`, `chacha20poly1305`, `pqcrypto-mlkem` (Post-quantum).
- **Database**: `sled` (Embedded key-value store).
- **AI/ML**: `ort` (ONNX Runtime), `ndarray`.
- **UI**: `ratatui` (Terminal UI).
- **Error/Logging**: `anyhow`, `thiserror`, `tracing`, `sentry`.
- **Bluetooth**: `bluer`, `btleplug`.
- **Onion Routing**: `sphinx-rs`.

### Frontend (React/TypeScript)
- **Framework**: `React 19`.
- **Runtime**: `Tauri 2.0` (Cross-platform desktop/mobile).
- **Build Tool**: `Vite`.
- **Styling**: `Tailwind CSS`, `Framer Motion` (Animations).
- **Icons**: `react-icons`.
- **HTTP Client**: `Axios`.

### Embedded (ESP32)
- **Framework**: `ESP-IDF`.

### AI/ML Ecosystem
- **Training**: `PyTorch`, `torch-geometric`, `networkx`.
- **Inference**: `ONNX Runtime`.
- **Local LLMs**: `Ollama` (running `Gemma 4`).

## Infrastructure & Dev Tools
- **Containerization**: `Docker`, `docker-compose`.
- **CI/CD**: `Travis CI`, `GitHub Actions`.
- **Pre-commit**: `pre-commit` hooks for linting.
- **Monitoring**: `Datadog` (OpenTelemetry), `Sentry`.
- **Scraping/Stealth**: `Obscura` (Headless browser).
- **Testing**: `Vitest`, `Playwright` (Frontend), `tokio-test`, `mockall` (Rust).

## Project Specific Libraries (Internal)
- `sphinx-rs`: Sphinx onion routing.
- `ghostwire-dtn`: Delay-Tolerant Networking.
- `hlc-rs`: Hybrid Logical Clocks.
- `trust-store`: Web of Trust / TOFU.
