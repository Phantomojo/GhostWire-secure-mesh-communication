Crate rustc\_version Copy item path

<span class="sub-heading"><a href="../src/rustc_version/lib.rs.html#9-424" class="src">Source</a>
</span>

Expand description

Simple library for getting the version information of a `rustc`
compiler.

This can be used by build scripts or other tools dealing with Rust
sources to make decisions based on the version of the compiler.

It calls `$RUSTC --version -v` and parses the output, falling back to
`rustc` if `$RUSTC` is not set.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
// This could be a cargo build script

use rustc_version::{version, version_meta, Channel, Version};

// Assert we haven't travelled back in time
assert!(version().unwrap().major >= 1);

// Set cfg flags depending on release channel
match version_meta().unwrap().channel {
    Channel::Stable => {
        println!("cargo:rustc-cfg=RUSTC_IS_STABLE");
    }
    Channel::Beta => {
        println!("cargo:rustc-cfg=RUSTC_IS_BETA");
    }
    Channel::Nightly => {
        println!("cargo:rustc-cfg=RUSTC_IS_NIGHTLY");
    }
    Channel::Dev => {
        println!("cargo:rustc-cfg=RUSTC_IS_DEV");
    }
}

// Check for a minimum version
if version().unwrap() >= Version::parse("1.4.0").unwrap() {
    println!("cargo:rustc-cfg=compiler_has_important_bugfix");
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.LlvmVersion.html" class="struct"
title="struct rustc_version::LlvmVersion">LlvmVersion</a>  
LLVM version

<a href="struct.Version.html" class="struct"
title="struct rustc_version::Version">Version</a>  
**SemVer version** as defined by <https://semver.org>.

<a href="struct.VersionMeta.html" class="struct"
title="struct rustc_version::VersionMeta">VersionMeta</a>  
Rustc version plus metadata like git short hash and build date.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Channel.html" class="enum"
title="enum rustc_version::Channel">Channel</a>  
Release channel of the compiler.

<a href="enum.Error.html" class="enum"
title="enum rustc_version::Error">Error</a>  
The error type for this crate.

<a href="enum.LlvmVersionParseError.html" class="enum"
title="enum rustc_version::LlvmVersionParseError">LlvmVersionParseError</a>  
LLVM Version Parse Error

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.version.html" class="fn"
title="fn rustc_version::version">version</a>  
Returns the `rustc` SemVer version.

<a href="fn.version_meta.html" class="fn"
title="fn rustc_version::version_meta">version_meta</a>  
Returns the `rustc` SemVer version and additional metadata like the git
short hash and build date.

<a href="fn.version_meta_for.html" class="fn"
title="fn rustc_version::version_meta_for">version_meta_for</a>  
Parses a “rustc -vV” output string and returns the SemVer version and
additional metadata like the git short hash and build date.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type rustc_version::Result">Result</a>  
The result type for this crate.
