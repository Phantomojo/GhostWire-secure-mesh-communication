// build.rs — compile-time configuration for Ghostwire
//
// Injects the release signing key (if provided) as a compile-time env var.
// Set GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX in your CI/release environment.
// Never commit the actual key here.

fn main() {
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
