//! Honesty tests for `src/cli/commands.rs` (P1 CLI audit: "6 structs, ZERO
//! refs; 5 of 6 print success, do nothing").
//!
//! The module was never wired into `cli/mod.rs`'s own `Commands` enum, yet it
//! contained handlers that print "Whisper sent", "Trust score set" and return
//! `Ok(())` having done nothing. Dead fake-success code is the same honesty
//! defect as a live one — anyone wiring these structs in later would ship the
//! lie wholesale. These tests pin that the dead handlers stay deleted.
//!
//! Source-level assertions (comment-stripped, tests/persistence_honesty.rs
//! pattern): the defect is which symbols exist, not observable at runtime
//! because the module has no callers — that is the defect.

use std::fs;

/// Read a source file relative to the crate root, with comments stripped.
fn src(path: &str) -> String {
    let root = env!("CARGO_MANIFEST_DIR");
    let raw = fs::read_to_string(format!("{root}/{path}"))
        .unwrap_or_else(|e| panic!("could not read {path}: {e}"));
    raw.lines()
        .map(|l| match l.find("//") {
            Some(i) => &l[..i],
            None => l,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn cli_commands_contains_no_dead_command_structs() {
    let body = src("src/cli/commands.rs");
    for name in ["Whisper", "Cloak", "Drop", "Fetch", "Peers", "Trust"] {
        let pattern = format!("pub struct {name}");
        assert!(
            !body.contains(&pattern),
            "src/cli/commands.rs still defines `{pattern}`. The struct has zero \
             references anywhere in src/ or tests/ and its execute() printed \
             fake success — it must stay deleted, or be wired into \
             cli/mod.rs' Commands enum by the cli/mod.rs owner with a real \
             implementation."
        );
    }
}

#[test]
fn cli_commands_contains_no_fake_success_prints() {
    let body = src("src/cli/commands.rs");
    assert!(
        !body.contains("println!") && !body.contains("Ok(())"),
        "src/cli/commands.rs reintroduced output-only handlers: any `println!` \
         + `Ok(())` pair here would report success for work the code never \
         performs (the P1 CLI audit finding)."
    );
}
