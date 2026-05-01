Crate anyhow Copy item path

<span class="sub-heading"><a href="../src/anyhow/lib.rs.html#1-728" class="src">Source</a>
</span>

Expand description

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/anyhow) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/anyhow) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/anyhow)

  

This library provides
[`anyhow::Error`](struct.Error.html "struct anyhow::Error"), a trait
object based error type for easy idiomatic error handling in Rust
applications.

  

# <a href="#details" class="doc-anchor">§</a>Details

-   Use `Result<T, anyhow::Error>`, or equivalently `anyhow::Result<T>`,
    as the return type of any fallible function.

    Within the function, use `?` to easily propagate any error that
    implements the
    [`std::error::Error`](https://doc.rust-lang.org/1.93.1/core/error/trait.Error.html "trait core::error::Error")
    trait.

    ``` {.rust .rust-example-rendered}
    use anyhow::Result;

    fn get_cluster_info() -> Result<ClusterMap> {
        let config = std::fs::read_to_string("cluster.json")?;
        let map: ClusterMap = serde_json::from_str(&config)?;
        Ok(map)
    }
    ```

-   Attach context to help the person troubleshooting the error
    understand where things went wrong. A low-level error like “No such
    file or directory” can be annoying to debug without more context
    about what higher level step the application was in the middle of.

    ``` {.rust .rust-example-rendered}
    use anyhow::{Context, Result};

    fn main() -> Result<()> {
        ...
        it.detach().context("Failed to detach the important thing")?;

        let content = std::fs::read(path)
            .with_context(|| format!("Failed to read instrs from {}", path))?;
        ...
    }
    ```

    ``` language-console
    Error: Failed to read instrs from ./path/to/instrs.json

    Caused by:
        No such file or directory (os error 2)
    ```

-   Downcasting is supported and can be by value, by shared reference,
    or by mutable reference as needed.

    ``` {.rust .rust-example-rendered}
    // If the error was caused by redaction, then return a
    // tombstone instead of the content.
    match root_cause.downcast_ref::<DataStoreError>() {
        Some(DataStoreError::Censored(_)) => Ok(Poll::Ready(REDACTED_CONTENT)),
        None => Err(error),
    }
    ```

-   If using Rust ≥ 1.65, a backtrace is captured and printed with the
    error if the underlying error type does not already provide its own.
    In order to see backtraces, they must be enabled through the
    environment variables described in
    [`std::backtrace`](https://doc.rust-lang.org/1.93.1/std/backtrace/index.html#environment-variables "mod std::backtrace"):

    -   If you want panics and errors to both have backtraces, set
        `RUST_BACKTRACE=1`;
    -   If you want only errors to have backtraces, set
        `RUST_LIB_BACKTRACE=1`;
    -   If you want only panics to have backtraces, set
        `RUST_BACKTRACE=1` and `RUST_LIB_BACKTRACE=0`.

-   Anyhow works with any error type that has an impl of
    `std::error::Error`, including ones defined in your crate. We do not
    bundle a `derive(Error)` macro but you can write the impls yourself
    or use a standalone macro like
    [thiserror](https://github.com/dtolnay/thiserror).

    ``` {.rust .rust-example-rendered}
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum FormatError {
        #[error("Invalid header (expected {expected:?}, got {found:?})")]
        InvalidHeader {
            expected: String,
            found: String,
        },
        #[error("Missing attribute: {0}")]
        MissingAttribute(String),
    }
    ```

-   One-off error messages can be constructed using the `anyhow!` macro,
    which supports string interpolation and produces an `anyhow::Error`.

    ``` {.rust .rust-example-rendered}
    return Err(anyhow!("Missing attribute: {}", missing));
    ```

    A `bail!` macro is provided as a shorthand for the same early
    return.

    ``` {.rust .rust-example-rendered}
    bail!("Missing attribute: {}", missing);
    ```

  

# <a href="#no-std-support" class="doc-anchor">§</a>No-std support

In no\_std mode, almost all of the same API is available and works the
same way. To depend on Anyhow in no\_std mode, disable our default
enabled “std” feature in Cargo.toml. A global allocator is required.

``` language-toml
[dependencies]
anyhow = { version = "1.0", default-features = false }
```

With versions of Rust older than 1.81, no\_std mode may require an
additional `.map_err(Error::msg)` when working with a non-Anyhow error
type inside a function that returns Anyhow’s error type, as the trait
that `?`-based error conversions are defined by is only available in std
in those old versions.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="macro.anyhow.html" class="macro"
title="macro anyhow::anyhow"><code>anyhow</code></a>` as format_err;`

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.anyhow.html" class="macro"
title="macro anyhow::anyhow">anyhow</a>  
Construct an ad-hoc error from a string or existing non-`anyhow` error
value.

<a href="macro.bail.html" class="macro"
title="macro anyhow::bail">bail</a>  
Return early with an error.

<a href="macro.ensure.html" class="macro"
title="macro anyhow::ensure">ensure</a>  
Return early with an error if a condition is not satisfied.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Chain.html" class="struct"
title="struct anyhow::Chain">Chain</a>  
Iterator of a chain of source errors.

<a href="struct.Error.html" class="struct"
title="struct anyhow::Error">Error</a>  
The `Error` type, a wrapper around a dynamic error type.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Context.html" class="trait"
title="trait anyhow::Context">Context</a>  
Provides the `context` method for `Result`.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.Ok.html" class="fn" title="fn anyhow::Ok">Ok</a>  
Equivalent to `Ok::<_, anyhow::Error>(value)`.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type anyhow::Result">Result</a>  
`Result<T, Error>`
