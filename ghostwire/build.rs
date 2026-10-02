// build.rs — compile-time configuration for Ghostwire
//
// Injects the release signing key (if provided) as a compile-time env var.
// Set GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX in your CI/release environment.
// Never commit the actual key here.
//
// Also builds the web UI. `web.rs` uses `rust_embed` over `webui/dist/`, and
// that directory is gitignored because it is generated output. Nothing built it:
// `build.rs` never invoked the frontend toolchain, so a clean clone compiled
// successfully and produced a binary serving an EMPTY UI — the API answered, and
// the browser got a blank page with no way to tell that from a broken server.
//
// The frontend source IS tracked (webui/src, package.json, lockfile), so the
// build is reproducible. This function makes it automatic, and fails the build
// loudly if the UI cannot be produced rather than shipping a blank one.
//
// Skipped when GHOSTWIRE_SKIP_WEBUI_BUILD is set, so a backend-only change does
// not pay for an npm install, and so an environment without Node can still
// build the library.

use std::path::{Path, PathBuf};
use std::process::Command;

fn build_webui() {
    if std::env::var_os("GHOSTWIRE_SKIP_WEBUI_BUILD").is_some() {
        println!(
            "cargo:warning=GHOSTWIRE_SKIP_WEBUI_BUILD set — web UI not built. \
             The binary will serve an empty UI unless webui/dist/ is populated."
        );
        return;
    }

    let webui = Path::new("../webui");
    let dist = webui.join("dist");
    let manifest = webui.join("package.json");

    if !manifest.exists() {
        println!(
            "cargo:warning=webui/package.json not found — skipping UI build. \
             This binary will serve an empty UI."
        );
        return;
    }

    // Nothing to do if the bundle is already present and newer than every source
    // file; this keeps incremental Rust builds from re-running npm each time.
    if dist.join("index.html").exists() {
        let bundle_time = dist.join("index.html").metadata().and_then(|m| m.modified());
        let source_time = newest_modified(webui.join("src"));
        if let (Ok(bundle), Some(source)) = (bundle_time, source_time) {
            if bundle >= source {
                println!("cargo:warning=webui/dist is up to date — not rebuilding");
                println!("cargo:rerun-if-changed=webui/dist");
                return;
            }
        }
    }

    let pkg_mgr = if command_exists("pnpm") {
        "pnpm"
    } else if command_exists("npm") {
        "npm"
    } else {
        println!(
            "cargo:warning=Neither pnpm nor npm found — cannot build the web UI. \
             cargo build will produce a binary that serves an empty UI. \
             Install Node, or pre-build webui/dist/, or set \
             GHOSTWIRE_SKIP_WEBUI_BUILD=1 to silence this."
        );
        return;
    };

    println!("cargo:warning=Building the web UI with {pkg_mgr}...");

    // Bind each Command: the builder chain and `status()` both borrow it, so an
    // inline `Command::new(..).arg(..).status()` drops the temporary while it is
    // still in use (E0716).
    let mut install = Command::new(pkg_mgr);
    install.arg("install").arg("--frozen-lockfile").current_dir(&webui);
    if !install.status().map(|s| s.success()).unwrap_or(false) {
        // A lockfile mismatch must not silently produce a stale bundle.
        let mut retry = Command::new(pkg_mgr);
        retry.arg("install").current_dir(&webui);
        if !retry.status().map(|s| s.success()).unwrap_or(false) {
            panic!(
                "web UI dependency install failed. The binary would serve an empty UI. \
                 Fix the install, or set GHOSTWIRE_SKIP_WEBUI_BUILD=1 to build without the UI."
            );
        }
    }

    let mut build = Command::new(pkg_mgr);
    build.arg("run").arg("build").current_dir(&webui);
    let status = build.status();
    match status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Web UI built successfully");
        }
        _ => panic!(
            "web UI build failed. Refusing to produce a binary that serves an empty UI. \
             Set GHOSTWIRE_SKIP_WEBUI_BUILD=1 to build without it."
        ),
    }

    println!("cargo:rerun-if-changed=webui/src");
    println!("cargo:rerun-if-changed=webui/package.json");
}

fn command_exists(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Modification time of the most recently changed file under `dir`, recursively.
///
/// Returns `None` when the directory is absent or unreadable, which makes the
/// caller rebuild rather than silently trust a possibly-stale bundle.
fn newest_modified(dir: PathBuf) -> Option<std::time::SystemTime> {
    let mut newest: Option<std::time::SystemTime> = None;
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        let modified = if path.is_dir() {
            newest_modified(path)
        } else {
            entry.metadata().and_then(|m| m.modified()).ok()
        };
        if let Some(t) = modified {
            if newest.map(|n| t > n).unwrap_or(true) {
                newest = Some(t);
            }
        }
    }
    newest
}

fn main() {
    build_webui();

    if std::env::var_os("GHOSTWIRE_RELEASE_SIGNING_KEY_HEX").is_some() {
        println!(
            "cargo:warning=GHOSTWIRE_RELEASE_SIGNING_KEY_HEX is deprecated and ignored; provide the public GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX instead"
        );
    }
    println!("cargo:rerun-if-env-changed=GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX");
    println!("cargo:rerun-if-env-changed=GHOSTWIRE_BINARY_HASH");
    println!("cargo:rerun-if-env-changed=GHOSTWIRE_ASSET_MANIFEST_HASH");
    println!("cargo:rerun-if-env-changed=GHOSTWIRE_GIT_HEAD");

    if let Ok(key_hex) = std::env::var("GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX") {
        if key_hex.len() == 64 && key_hex.chars().all(|c| c.is_ascii_hexdigit()) {
            println!(
                "cargo:rustc-env=GHOSTWIRE_RELEASE_VERIFYING_KEY_BYTES={}",
                key_hex
            );
            println!(
                "cargo:warning=Release public verification key injected from GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX"
            );
        } else {
            println!(
                "cargo:warning=GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX is set but invalid (need 64 hex chars). Updates disabled."
            );
        }
    } else {
        println!(
            "cargo:warning=GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX not set. Automatic updates will be disabled at runtime."
        );
    }

    if let Ok(hash_hex) = std::env::var("GHOSTWIRE_BINARY_HASH") {
        if hash_hex.len() == 64 && hash_hex.chars().all(|c| c.is_ascii_hexdigit()) {
            println!("cargo:rustc-env=GHOSTWIRE_BINARY_HASH={}", hash_hex);
            println!("cargo:warning=Binary integrity hash embedded from GHOSTWIRE_BINARY_HASH");
        } else {
            println!(
                "cargo:warning=GHOSTWIRE_BINARY_HASH is set but invalid (need 64 hex chars). Integrity check disabled."
            );
        }
    } else {
        println!(
            "cargo:warning=GHOSTWIRE_BINARY_HASH not set. Binary integrity check disabled at runtime."
        );
    }
    for name in ["GHOSTWIRE_ASSET_MANIFEST_HASH", "GHOSTWIRE_GIT_HEAD"] {
        if let Ok(value) = std::env::var(name) {
            if !value.is_empty() && value.chars().all(|c| c.is_ascii_graphic()) {
                println!("cargo:rustc-env={name}={value}");
            }
        }
    }
}
