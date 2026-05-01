Crate pin\_project Copy item path

<span class="sub-heading"><a href="../src/pin_project/lib.rs.html#3-350" class="src">Source</a>
</span>

Expand description

A crate for safe and ergonomic
[pin-projection](https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning).

## <a href="#usage" class="doc-anchor">§</a>Usage

Add this to your `Cargo.toml`:

``` language-toml
[dependencies]
pin-project = "1"
```

## <a href="#examples" class="doc-anchor">§</a>Examples

[`#[pin_project]`](attr.pin_project.html "attr pin_project::pin_project")
attribute creates projection types covering all the fields of struct or
enum.

``` {.rust .rust-example-rendered}
use std::pin::Pin;

use pin_project::pin_project;

#[pin_project]
struct Struct<T, U> {
    #[pin]
    pinned: T,
    unpinned: U,
}

impl<T, U> Struct<T, U> {
    fn method(self: Pin<&mut Self>) {
        let this = self.project();
        let _: Pin<&mut T> = this.pinned; // Pinned reference to the field
        let _: &mut U = this.unpinned; // Normal reference to the field
    }
}
```

[*code like this will be
generated*](https://github.com/taiki-e/pin-project/blob/HEAD/examples/struct-default-expanded.rs)

To use `#[pin_project]` on enums, you need to name the projection type
returned from the method.

``` {.rust .rust-example-rendered}
use std::pin::Pin;

use pin_project::pin_project;

#[pin_project(project = EnumProj)]
enum Enum<T, U> {
    Pinned(#[pin] T),
    Unpinned(U),
}

impl<T, U> Enum<T, U> {
    fn method(self: Pin<&mut Self>) {
        match self.project() {
            EnumProj::Pinned(x) => {
                let _: Pin<&mut T> = x;
            }
            EnumProj::Unpinned(y) => {
                let _: &mut U = y;
            }
        }
    }
}
```

[*code like this will be
generated*](https://github.com/taiki-e/pin-project/blob/HEAD/examples/enum-default-expanded.rs)

See
[`#[pin_project]`](attr.pin_project.html "attr pin_project::pin_project")
attribute for more details, and see
[examples](https://github.com/taiki-e/pin-project/blob/HEAD/examples/README.md)
directory for more examples and generated code.

## <a href="#related-projects" class="doc-anchor">§</a>Related Projects

-   [pin-project-lite](https://github.com/taiki-e/pin-project-lite): A
    lightweight version of pin-project written with declarative macros.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.UnsafeUnpin.html" class="trait"
title="trait pin_project::UnsafeUnpin">UnsafeUnpin</a>  
A trait used for custom implementations of
[`Unpin`](https://doc.rust-lang.org/1.93.1/core/marker/trait.Unpin.html "trait core::marker::Unpin").

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.pin_project.html" class="attr"
title="attr pin_project::pin_project">pin_project</a>  
An attribute that creates projection types covering all the fields of
struct or enum.

<a href="attr.pinned_drop.html" class="attr"
title="attr pin_project::pinned_drop">pinned_drop</a>  
An attribute used for custom implementations of \[`Drop`\].
