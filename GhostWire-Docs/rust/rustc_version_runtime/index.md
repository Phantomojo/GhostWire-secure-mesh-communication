Crate rustc\_version\_runtime Copy item path

<span class="sub-heading"><a href="../src/rustc_version_runtime/lib.rs.html#1-58"
class="src">Source</a> </span>

Expand description

Simple library for getting the version information of a `rustc` compiler
in runtime.

The goal of this crate is to provide debug information to developers,
not to branch application logic based on compiler version. Please, don’t
do that.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
extern crate rustc_version_runtime;

println!("This was compiled using {:?}", rustc_version_runtime::version());
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Version.html" class="struct"
title="struct rustc_version_runtime::Version">Version</a>  
**SemVer version** as defined by <https://semver.org>.

<a href="struct.VersionMeta.html" class="struct"
title="struct rustc_version_runtime::VersionMeta">VersionMeta</a>  
Rustc version plus metadata like git short hash and build date.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Channel.html" class="enum"
title="enum rustc_version_runtime::Channel">Channel</a>  
Release channel of the compiler.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.version.html" class="fn"
title="fn rustc_version_runtime::version">version</a>  
Returns the `rustc` SemVer version.

<a href="fn.version_matches.html" class="fn"
title="fn rustc_version_runtime::version_matches">version_matches</a>  
Check wether the `rustc` version matches the given SemVer version
requirement.

<a href="fn.version_meta.html" class="fn"
title="fn rustc_version_runtime::version_meta">version_meta</a>  
Returns the `rustc` SemVer version and additional metadata like the git
short hash and build date.
