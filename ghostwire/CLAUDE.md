# GhostWire — Claude Code Configuration
# Extends AGENTS.md with Claude Code-specific features
# Note: AGENTS.md is the canonical source. This adds Claude-only directives.

@AGENTS.md

---

## Claude-Specific Directives

### Memory & Session Continuity

Claude Code: after completing any task that changes architecture, update `.ai/memory.md`
with what changed, why, and what the next logical step is. This survives session resets.

### Thinking Mode (Claude's extended thinking)

Enable extended thinking for:
- Root cause diagnosis on compile errors spanning multiple files
- Tradeoff decisions (e.g. Arc<RwLock> vs message passing for GemmaClient)
- Any change touching Core struct or module graph

Disable extended thinking for:
- Single-file mechanical fixes (import corrections, typo fixes)
- Running cargo commands and reporting output
- Reading file contents

### File Scope Rules

When editing, Claude must state which files it will touch before touching them.
Format: "Modifying: [file1], [file2]. Not touching: [file3]."

If a task requires touching more than 5 files, pause and confirm with user first.

### Import Convention

When adding new use statements in Rust files, group them:
```rust
// std
use std::sync::{Arc, RwLock};

// external crates
use tokio::time::Duration;
use tracing::info;

// internal modules  
use crate::core::ai::GemmaClient;
```

### Error Handling Pattern for AI Components

```rust
// Preferred pattern for optional AI components
match self.gemma_client.read().await.as_ref() {
    Some(client) => {
        client.analyze_mesh(&telemetry).await
            .map_err(|e| {
                tracing::warn!("Gemma analysis failed, falling back: {}", e);
                e
            })
            .ok() // graceful degradation — AI failure never crashes mesh
    }
    None => {
        tracing::debug!("Gemma client not initialized, using PRoPHET fallback");
        None
    }
}
```

### Slash Commands (Claude Code)

/ghostwire-status    — Run cargo check and report all errors with triage
/ghostwire-ai-audit  — Check all four AI layers for wiring completeness
/ghostwire-handoff   — Generate end-of-session handoff document

### Hooks

Before every file write: read the current file first.
After every cargo build: capture stderr and include in status report.
After every session: update .ai/memory.md.
