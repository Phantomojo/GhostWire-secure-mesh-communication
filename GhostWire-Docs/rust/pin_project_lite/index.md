Crate pin\_project\_lite Copy item path

<span class="sub-heading"><a href="../src/pin_project_lite/lib.rs.html#3-1766"
class="src">Source</a> </span>

Expand description

A lightweight version of
[pin-project](https://github.com/taiki-e/pin-project) written with
declarative macros.

## <a href="#usage" class="doc-anchor">§</a>Usage

Add this to your `Cargo.toml`:

``` language-toml
[dependencies]
pin-project-lite = "0.2"
```

## <a href="#examples" class="doc-anchor">§</a>Examples

[`pin_project!`](macro.pin_project.html "macro pin_project_lite::pin_project")
macro creates a projection type covering all the fields of struct.

``` {.rust .rust-example-rendered}
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    struct Struct<T, U> {
        #[pin]
        pinned: T,
        unpinned: U,
    }
}

impl<T, U> Struct<T, U> {
    fn method(self: Pin<&mut Self>) {
        let this = self.project();
        let _: Pin<&mut T> = this.pinned; // Pinned reference to the field
        let _: &mut U = this.unpinned; // Normal reference to the field
    }
}
```

To use
[`pin_project!`](macro.pin_project.html "macro pin_project_lite::pin_project")
on enums, you need to name the projection type returned from the method.

``` {.rust .rust-example-rendered}
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    #[project = EnumProj]
    enum Enum<T, U> {
        Variant { #[pin] pinned: T, unpinned: U },
    }
}

impl<T, U> Enum<T, U> {
    fn method(self: Pin<&mut Self>) {
        match self.project() {
            EnumProj::Variant { pinned, unpinned } => {
                let _: Pin<&mut T> = pinned;
                let _: &mut U = unpinned;
            }
        }
    }
}
```

## <a href="#pin-project-vs-pin-project-lite" class="doc-anchor">§</a>[pin-project](https://github.com/taiki-e/pin-project) vs pin-project-lite

Here are some similarities and differences compared to
[pin-project](https://github.com/taiki-e/pin-project).

### <a href="#similar-safety" class="doc-anchor">§</a>Similar: Safety

pin-project-lite guarantees safety in much the same way as
[pin-project](https://github.com/taiki-e/pin-project). Both are
completely safe unless you write other unsafe code.

### <a href="#different-minimal-design" class="doc-anchor">§</a>Different: Minimal design

This library does not tackle as expansive of a range of use cases as
[pin-project](https://github.com/taiki-e/pin-project) does. If your use
case is not already covered, please use
[pin-project](https://github.com/taiki-e/pin-project).

### <a href="#different-no-proc-macro-related-dependencies"
class="doc-anchor">§</a>Different: No proc-macro related dependencies

This is the **only** reason to use this crate. However, **if you already
have proc-macro related dependencies in your crate’s dependency graph,
there is no benefit from using this crate.** (Note: There is almost no
difference in the amount of code generated between
[pin-project](https://github.com/taiki-e/pin-project) and
pin-project-lite.)

### <a href="#different-no-useful-error-messages" class="doc-anchor">§</a>Different: No useful error messages

This macro does not handle any invalid input. So error messages are not
to be useful in most cases. If you do need useful error messages, then
upon error you can pass the same input to
[pin-project](https://github.com/taiki-e/pin-project) to receive a
helpful description of the compile error.

### <a href="#different-no-support-for-custom-unpin-implementation"
class="doc-anchor">§</a>Different: No support for custom Unpin implementation

pin-project supports this by
[`UnsafeUnpin`](https://docs.rs/pin-project/latest/pin_project/attr.pin_project.html#unsafeunpin).
(`!Unpin` is supported by both
[pin-project](https://docs.rs/pin-project/latest/pin_project/attr.pin_project.html#unpin)
and
[pin-project-lite](macro.pin_project.html#unpin "macro pin_project_lite::pin_project").)

### <a href="#different-no-support-for-tuple-structs-and-tuple-variants"
class="doc-anchor">§</a>Different: No support for tuple structs and tuple variants

pin-project supports this.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.pin_project.html" class="macro"
title="macro pin_project_lite::pin_project">pin_project</a>  
A macro that creates a projection type covering all the fields of
struct.
