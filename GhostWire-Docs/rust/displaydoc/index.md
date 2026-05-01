Crate displaydoc Copy item path

<span class="sub-heading"><a href="../src/displaydoc/lib.rs.html#1-186" class="src">Source</a>
</span>

Expand description

This library provides a convenient derive macro for the standard
library’s
[`core::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
trait.

``` language-toml
[dependencies]
displaydoc = "0.2"
```

*Compiler support: requires rustc 1.56+*

  

## <a href="#example" class="doc-anchor">§</a>Example

*Demonstration alongside the
[`Error`](https://doc.rust-lang.org/1.93.1/core/error/trait.Error.html "trait core::error::Error")
derive macro from
[`thiserror`](https://docs.rs/thiserror/1.0.25/thiserror/index.html), to
propagate source locations from
[`io::Error`](https://doc.rust-lang.org/1.93.1/std/io/error/struct.Error.html "struct std::io::error::Error")
with the `#[source]` attribute:*

``` {.rust .rust-example-rendered}
use std::io;
use displaydoc::Display;
use thiserror::Error;

#[derive(Display, Error, Debug)]
pub enum DataStoreError {
    /// data store disconnected
    Disconnect(#[source] io::Error),
    /// the data for key `{0}` is not available
    Redaction(String),
    /// invalid header (expected {expected:?}, found {found:?})
    InvalidHeader {
        expected: String,
        found: String,
    },
    /// unknown data store error
    Unknown,
}

let error = DataStoreError::Redaction("CLASSIFIED CONTENT".to_string());
assert!("the data for key `CLASSIFIED CONTENT` is not available" == &format!("{}", error));
```

*Note that although
[`io::Error`](https://doc.rust-lang.org/1.93.1/std/io/error/struct.Error.html "struct std::io::error::Error")
implements `Display`, we do not add it to the generated message for
`DataStoreError::Disconnect`, since it is already made available via
`#[source]`. See further context on avoiding duplication in error
reports at the rust blog
[here](https://github.com/yaahc/blog.rust-lang.org/blob/master/posts/inside-rust/2021-05-15-What-the-error-handling-project-group-is-working-towards.md#duplicate-information-issue).*

  

## <a href="#details" class="doc-anchor">§</a>Details

-   A `fmt::Display` impl is generated for your enum if you provide a
    docstring comment on each variant as shown above in the example. The
    `Display` derive macro supports a shorthand for interpolating fields
    from the error:
    -   `/// {var}` ⟶ `write!("{}", self.var)`
    -   `/// {0}` ⟶ `write!("{}", self.0)`
    -   `/// {var:?}` ⟶ `write!("{:?}", self.var)`
    -   `/// {0:?}` ⟶ `write!("{:?}", self.0)`
-   This also works with structs and [generic
    types](derive.Display.html#generic-type-parameters "derive displaydoc::Display"):

``` {.rust .rust-example-rendered}
/// oh no, an error: {0}
#[derive(Display)]
pub struct Error<E>(pub E);

let error: Error<&str> = Error("muahaha i am an error");
assert!("oh no, an error: muahaha i am an error" == &format!("{}", error));
```

-   Two optional attributes can be added to your types next to the
    derive:

    -   `#[ignore_extra_doc_attributes]` makes the macro ignore any doc
        comment attributes (or `///` lines) after the first. Multi-line
        comments using `///` are otherwise treated as an error, so use
        this attribute or consider switching to block doc comments
        (`/** */`).

    -   `#[prefix_enum_doc_attributes]` combines the doc comment message
        on your enum itself with the messages for each variant, in the
        format “enum: variant”. When added to an enum, the doc comment
        on the enum becomes mandatory. When added to any other type, it
        has no effect.

-   In case you want to have an independent doc comment, the
    `#[displaydoc("...")` atrribute may be used on the variant or struct
    to override it.

  

## <a href="#faq" class="doc-anchor">§</a>FAQ

1.  **Is this crate `no_std` compatible?**

    -   Yes! This crate implements the
        [`core::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
        trait, not the
        [`std::fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display")
        trait, so it should work in `std` and `no_std` environments.
        Just add `default-features = false`.

2.  **Does this crate work with `Path` and `PathBuf` via the `Display`
    trait?**

    -   Yuuup. This crate uses @dtolnay’s [autoref specialization
        technique](https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md)
        to add a special trait for types to get the display impl. It
        then specializes for `Path` and `PathBuf`, and when either of
        these types are found, it calls `self.display()` to get a
        `std::path::Display<'_>` type which can be used with the
        `Display` format specifier!

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.Display.html" class="derive"
title="derive displaydoc::Display">Display</a>  
[Custom `#[derive(...)]`
macro](https://doc.rust-lang.org/edition-guide/rust-2018/macros/custom-derive.html)
for implementing
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display")
via doc comment attributes.
