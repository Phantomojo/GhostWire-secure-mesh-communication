//! The daemon must forward `--config`, `--profile` and `--mesh-size` to its
//! child process.
//!
//! `start_daemon` re-executes this same binary to run the daemon in the
//! background. It builds the child's argv explicitly, so anything it does not
//! repeat is silently replaced by that child's own default. Three settings were
//! dropped that way:
//!
//!   * `--config` — an operator pointing at a custom config file got the default
//!     one. Every setting in that file was ignored, including
//!     `reticulum_enabled` and `hybrid_mode`.
//!   * `--profile` — same.
//!   * `--mesh-size` — the CLI printed "Mesh size: large" while gossipsub ran
//!     with mesh_n=2, because main.rs set GHOSTWIRE_MESH_SIZE and the child
//!     overwrote it with the clap default on the way back through main().
//!
//! This is source-level in the repo's `persistence_honesty` style: the argv
//! construction is a side-effecting function that spawns a process, so the
//! behavioural test is that the three flags appear in the built argv. A run of
//! `ghostwire --config custom.toml start` plus a check of the child's
//! `/proc/<pid>/cmdline` would be stronger, but it needs a spawn and a sleep.

use std::fs;

/// Extract the argv literal `start_daemon` builds for the child process.
fn child_argv_source() -> String {
    fs::read_to_string("src/daemon.rs").expect("src/daemon.rs must be readable")
}

#[test]
fn daemon_forwards_config_to_the_child() {
    let src = child_argv_source();
    let start = src
        .find("pub fn start_daemon")
        .expect("start_daemon must exist in daemon.rs");
    let body = &src[start..];

    assert!(
        body.contains(r#"cmd.arg("--config").arg(config)"#),
        "start_daemon must forward --config to the child. Without it, \
         `ghostwire --config custom.toml start` silently runs the daemon against \
         the DEFAULT config, so reticulum_enabled and hybrid_mode in that file \
         are ignored."
    );
}

#[test]
fn daemon_forwards_profile_to_the_child() {
    let src = child_argv_source();
    let start = src.find("pub fn start_daemon").unwrap();
    let body = &src[start..];

    assert!(
        body.contains(r#"cmd.arg("--profile").arg(profile)"#),
        "start_daemon must forward --profile to the child."
    );
}

#[test]
fn daemon_forwards_mesh_size_to_the_child() {
    let src = child_argv_source();
    let start = src.find("pub fn start_daemon").unwrap();
    let body = &src[start..];

    assert!(
        body.contains(r#"cmd.arg("--mesh-size").arg(mesh_size)"#),
        "start_daemon must forward --mesh-size to the child. Otherwise the CLI \
         prints 'Mesh size: large' while gossipsub runs with mesh_n=2."
    );
}

#[test]
fn daemon_args_persist_config_across_save_and_load() {
    // `ghostwire tui` reattaches via load_saved_args(). If the three settings
    // were not persisted there, a TUI attach would silently lose them even
    // after the argv fix.
    let src = child_argv_source();

    assert!(
        src.contains("pub config: Option<String>"),
        "SavedDaemonArgs must persist the config path."
    );
    assert!(
        src.contains("config: args.config.clone()"),
        "save_daemon_args must write the config path."
    );
    assert!(
        src.contains("config: saved.config"),
        "load_saved_args must restore the config path."
    );
}

#[test]
fn main_populates_the_forwarded_fields() {
    // The other half: main.rs must actually fill them in. A forward that nobody
    // populates is the same bug wearing a different hat.
    let src = fs::read_to_string("src/main.rs").expect("src/main.rs must be readable");

    assert!(
        src.contains("config: args.config.clone()"),
        "main.rs must pass --config into DaemonArgs."
    );
    assert!(
        src.contains("mesh_size: Some(args.mesh_size.clone())"),
        "main.rs must pass --mesh-size into DaemonArgs."
    );
}