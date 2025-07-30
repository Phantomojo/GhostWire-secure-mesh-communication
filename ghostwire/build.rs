use std::env;

fn main() {
    // Set build-time configuration
    println!("cargo:rerun-if-changed=build.rs");
    
    // Enable optimizations for release builds
    if env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-env=RUSTFLAGS=-C target-cpu=native -C target-feature=+crt-static");
    }
    
    // Set feature flags based on environment
    if env::var("CARGO_FEATURE_MINIMAL").is_ok() {
        println!("cargo:rustc-cfg=minimal_build");
    }
    
    // Enable LTO for release builds
    if env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-env=RUSTFLAGS=-C lto=fat");
    }
    
    // Set panic strategy to abort for smaller binaries
    if env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-env=RUSTFLAGS=-C panic=abort");
    }
} 