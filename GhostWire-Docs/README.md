# GhostWire Knowledge Base

This is a comprehensive, local documentation knowledge base for the GhostWire project. It is designed to be easily readable by both human developers and local AI agents (like Ollama/Gemma 4).

## Structure

- **`rust/`**: Core backend documentation.
    - `libp2p/`: P2P and mesh networking.
    - `tokio/`: Async runtime and synchronization.
    - `axum/`: Web framework and routing.
    - `vodozemac/`: E2E encryption (Olm/Megolm).
    - `sled/`: Embedded database.
    - `sphinx-rs/`: Onion routing.
    - `cryptography/`: Primitives (ring, dalek, etc.).
- **`frontend/`**: Web UI and Tauri documentation.
- **`python-ml/`**: Machine Learning and AI components.
- **`embedded/`**: ESP32 and firmware development.
- **`infrastructure/`**: Docker, scripts, and CI/CD.

## Updating the Knowledge Base

To keep this knowledge base up-to-date:
1.  Use `read_url_content` or `obscura` to fetch the latest docs from `docs.rs` or official sites.
2.  Convert to clean Markdown.
3.  Add a version/date header to each file.
4.  Ensure code examples are preserved.

## GhostWire Usage Notes

Each major section contains a `common-patterns.md` or `ghostwire-usage.md` file with specific implementation advice for the GhostWire architecture.
