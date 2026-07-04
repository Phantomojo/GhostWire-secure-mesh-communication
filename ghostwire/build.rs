// build.rs — compile-time configuration for Ghostwire
//
// Injects the release signing key (if provided) as a compile-time env var.
// Set GHOSTWIRE_RELEASE_SIGNING_KEY_HEX in your CI/release environment.
// Never commit the actual key here.

fn main() {
    // Tell cargo to re-run this if the env var changes
    println!("cargo:rerun-if-env-changed=GHOSTWIRE_RELEASE_SIGNING_KEY_HEX");

    if let Ok(key_hex) = std::env::var("GHOSTWIRE_RELEASE_SIGNING_KEY_HEX") {
        // Validate it's 64 hex chars (32 bytes)
        if key_hex.len() == 64 && key_hex.chars().all(|c| c.is_ascii_hexdigit()) {
            println!("cargo:rustc-env=GHOSTWIRE_RELEASE_KEY_BYTES={}", key_hex);
            println!(
                "cargo:warning=Release signing key injected from GHOSTWIRE_RELEASE_SIGNING_KEY_HEX"
            );
        } else {
            println!("cargo:warning=GHOSTWIRE_RELEASE_SIGNING_KEY_HEX is set but invalid (need 64 hex chars). Updates disabled.");
        }
    } else {
        println!("cargo:warning=GHOSTWIRE_RELEASE_SIGNING_KEY_HEX not set. Automatic updates will be disabled at runtime.");
    }
}
