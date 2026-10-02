// Intentionally empty: this module's six clap structs (Whisper, Cloak, Drop,
// Fetch, Peers, Trust) had zero references anywhere in src/ or tests/, and
// five of six printed fake success before returning Ok(()). Deleted rather
// than wired — wiring belongs to the cli/mod.rs owner. Guarded by
// tests/cli_honesty.rs; do not re-add unwired command stubs here.
