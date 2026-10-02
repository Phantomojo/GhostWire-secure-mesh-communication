//! Fail-first guard for H1: the config file path must be recorded at load
//! time so PUT /api/settings can persist config keys back to the file the
//! process actually loaded from.
//!
//! The source-level assertion exists first because the `loaded_path` field
//! does not exist yet — a behavioural test referencing it would not compile,
//! and a compile error is not a failing test. Behavioural round-trip tests
//! are appended after the field lands.

use ghostwire::config::Config;
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
fn load_from_file_records_the_path_it_loaded_from() {
    let config_src = src("src/config.rs");

    // Both load() and load_from_path() must funnel through load_from_file so
    // one recording site covers every entry point.
    let load_start = config_src
        .find("pub fn load()")
        .expect("Config::load not found");
    let load_body = &config_src[load_start..config_src.find("pub fn load_from_path").expect("load_from_path not found")];
    assert!(
        load_body.contains("load_from_file"),
        "Config::load must resolve files through load_from_file, otherwise \
         the recording site does not cover the default entry point"
    );

    let start = config_src
        .find("fn load_from_file")
        .expect("load_from_file not found");
    let body = &config_src[start..];
    let end = body
        .find("\n    fn from_env")
        .unwrap_or_else(|| body.find("\n    pub fn save").unwrap_or(body.len()));
    let body = &body[..end];

    assert!(
        body.contains("loaded_path"),
        "load_from_file must record loaded_path so later saves write back to \
         the file the process loaded from (H1 config persistence)"
    );
}

#[test]
fn load_from_path_records_the_path_it_loaded_from() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("ghostwire.toml");
    fs::write(&path, "[server]\nport = 9090\n").expect("write config");
    let path_str = path.to_str().expect("utf-8 temp path");

    let cfg = Config::load_from_path(path_str).expect("load_from_path");

    assert_eq!(cfg.loaded_path.as_deref(), Some(path.as_path()));
    assert_eq!(cfg.server.port, 9090);
}

#[test]
fn save_to_recorded_path_roundtrips_without_serializing_the_path_itself() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("ghostwire.toml");
    fs::write(&path, "[server]\nport = 9090\n").expect("write config");
    let path_str = path.to_str().expect("utf-8 temp path");

    let mut cfg = Config::load_from_path(path_str).expect("load_from_path");
    cfg.server.port = 9999;
    let recorded = cfg.loaded_path.clone().expect("path must be recorded");
    cfg.save(&recorded.to_string_lossy()).expect("save");

    let saved = fs::read_to_string(&path).expect("read back");
    assert!(
        saved.contains("port = 9999"),
        "saved file must contain the new port, got:\n{saved}"
    );
    assert!(
        !saved.contains("loaded_path"),
        "serde(skip) must keep load provenance out of the config file, got:\n{saved}"
    );

    let reloaded = Config::load_from_path(path_str).expect("reload");
    assert_eq!(reloaded.server.port, 9999);
    assert_eq!(reloaded.loaded_path.as_deref(), Some(path.as_path()));
}
