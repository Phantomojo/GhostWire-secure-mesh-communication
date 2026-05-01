Crate const\_panic Copy item path

<span class="sub-heading"><a href="../src/const_panic/lib.rs.html#1-345" class="src">Source</a>
</span>

Expand description

For panicking with formatting in const contexts.

This library exists because the panic macro was stabilized for use in
const contexts in Rust 1.57.0, without formatting support.

All of the types that implement the
[`PanicFmt`](fmt/trait.PanicFmt.html "trait const_panic::fmt::PanicFmt")
trait can be formatted in panics.

# <a href="#examples" class="doc-anchor">§</a>Examples

-   [Basic](#basic)
-   [Custom Types](#custom-types)

### <a href="#basic" class="doc-anchor">§</a>Basic

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
use const_panic::concat_assert;

const FOO: u32 = 10;
const BAR: u32 = 0;
const _: () = assert_non_zero(FOO, BAR);

#[track_caller]
const fn assert_non_zero(foo: u32, bar: u32) {
    concat_assert!{
        foo != 0 && bar != 0,
        "\nneither foo nor bar can be zero!\nfoo: ", foo, "\nbar: ", bar
    }
}
```

The above code fails to compile with this error:

``` language-text
error[E0080]: evaluation of constant value failed
 --> src/lib.rs:20:15
  |
8 | const _: () = assert_non_zero(FOO, BAR);
  |               ^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
neither foo nor bar can be zero!
foo: 10
bar: 0', src/lib.rs:8:15
```

When called at runtime

<a href="#" class="tooltip" title="This example panics">ⓘ</a>

``` {.rust .rust-example-rendered}
use const_panic::concat_assert;

assert_non_zero(10, 0);

#[track_caller]
const fn assert_non_zero(foo: u32, bar: u32) {
    concat_assert!{
        foo != 0 && bar != 0,
        "\nneither foo nor bar can be zero!\nfoo: ", foo, "\nbar: ", bar
    }
}
```

it prints this:

``` language-text
thread 'main' panicked at '
neither foo nor bar can be zero!
foo: 10
bar: 0', src/lib.rs:6:1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### <a href="#custom-types" class="doc-anchor">§</a>Custom types

Panic formatting for custom types can be done in these ways (in
increasing order of verbosity):

-   Using the [`PanicFmt` derive](derive@crate::PanicFmt) macro
    (requires the opt-in `"derive"` feature)
-   Using the [`impl_panicfmt`](crate::impl_panicfmt) macro (requires
    the default-enabled `"non_basic"` feature)
-   Using the [`flatten_panicvals`](crate::flatten_panicvals) macro
    (requires the default-enabled `"non_basic"` feature)
-   Manually implementing the
    [`PanicFmt`](fmt/trait.PanicFmt.html "trait const_panic::fmt::PanicFmt")
    trait as described in its docs.

This example uses the [`PanicFmt` derive](derive@crate::PanicFmt)
approach.

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
use const_panic::{PanicFmt, concat_panic};

const LAST: u8 = {
    Foo{
        x: &[],
        y: Bar(false, true),
        z: Qux::Left(23),
    }.pop().1
};

impl Foo<'_> {
    /// Pops the last element
    ///
    /// # Panics
    ///
    /// Panics if `self.x` is empty
    #[track_caller]
    const fn pop(mut self) -> (Self, u8) {
        if let [rem @ .., last] = self.x {
            self.x = rem;
            (self, *last)
        } else {
            concat_panic!(
                "\nexpected a non-empty Foo, found: \n",
                // uses alternative Debug formatting for `self`,
                // otherwise this would use regular Debug formatting.
                alt_debug: self
            )
        }
    }
}

#[derive(PanicFmt)]
struct Foo<'a> {
    x: &'a [u8],
    y: Bar,
    z: Qux,
}

#[derive(PanicFmt)]
struct Bar(bool, bool);

#[derive(PanicFmt)]
enum Qux {
    Up,
    Down { x: u32, y: u32 },
    Left(u64),
}
```

The above code fails to compile with this error:

``` language-text
error[E0080]: evaluation of constant value failed
  --> src/lib.rs:57:5
   |
7  | /     Foo{
8  | |         x: &[],
9  | |         y: Bar(false, true),
10 | |         z: Qux::Left(23),
11 | |     }.pop().1
   | |___________^ the evaluated program panicked at '
expected a non-empty Foo, found:
Foo {
    x: [],
    y: Bar(
        false,
        true,
    ),
    z: Left(
        23,
    ),
}', src/lib.rs:11:7
```

# <a href="#limitations" class="doc-anchor">§</a>Limitations

Arguments to the formatting/panicking macros must have a fully inferred
concrete type, because `const_panic` macros use duck typing to call
methods on those arguments.

One effect of that limitation is that you will have to pass suffixed
integer literals (eg: `100u8`) when those integers aren’t inferred to be
a concrete type.

### <a href="#panic-message-length" class="doc-anchor">§</a>Panic message length

The panic message can only be up to
[`MAX_PANIC_MSG_LEN`](constant.MAX_PANIC_MSG_LEN.html "constant const_panic::MAX_PANIC_MSG_LEN")
long, after which it is truncated.

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo features

-   `"non_basic"`(enabled by default): Enables support for formatting
    structs, enums, and arrays.  
    Without this feature, you can effectively only format primitive
    types (custom types can manually implement formatting with more
    difficulty).

-   `"rust_latest_stable"`(disabled by default): Enables all the
    `"rust_1_*"` features.

-   `"rust_1_64"`(disabled by default): Enables formatting of additional
    items that require Rust 1.64.0 to do so.

-   `"rust_1_82"`(disabled by default): Enables formatting of additional
    items that require Rust 1.82.0 to do so.

-   `"rust_1_88"`(disabled by default): Enables formatting of additional
    items that require Rust 1.88.0 to do so.

-   `"derive"`(disabled by default): Enables the [`PanicFmt`
    derive](derive@crate::PanicFmt) macro.

# <a href="#plans" class="doc-anchor">§</a>Plans

None for now

# <a href="#no-std-support" class="doc-anchor">§</a>No-std support

`const_panic` is `#![no_std]`, it can be used anywhere Rust can be used.

# <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

This requires Rust 1.57.0, because it uses the `panic` macro in a const
context.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::fmt::`<a href="fmt/struct.FmtArg.html" class="struct"
title="struct const_panic::fmt::FmtArg"><code>FmtArg</code></a>`;`

`pub use crate::fmt::`<a href="fmt/struct.IsCustomType.html" class="struct"
title="struct const_panic::fmt::IsCustomType"><code>IsCustomType</code></a>`;`

`pub use crate::fmt::`<a href="fmt/trait.PanicFmt.html" class="trait"
title="trait const_panic::fmt::PanicFmt"><code>PanicFmt</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="fmt/index.html" class="mod"
title="mod const_panic::fmt">fmt</a>  
Formatting-related items

<a href="utils/index.html" class="mod"
title="mod const_panic::utils">utils</a>  
Utility functions

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.coerce_fmt.html" class="macro"
title="macro const_panic::coerce_fmt">coerce_fmt</a>  
Coerces `$reff` to a type that has a `to_panicvals` method, which is
expected to return a `[PanicVal<'_>; LEN]`.

<a href="macro.concat_assert.html" class="macro"
title="macro const_panic::concat_assert">concat_assert</a>  
Asserts that `$condition` is true.

<a href="macro.concat_panic.html" class="macro"
title="macro const_panic::concat_panic">concat_panic</a>  
Panics with the concanenation of the arguments.

<a href="macro.unwrap_err.html" class="macro"
title="macro const_panic::unwrap_err">unwrap_err</a>  
Gets the value in the `Err` variant.

<a href="macro.unwrap_ok.html" class="macro"
title="macro const_panic::unwrap_ok">unwrap_ok</a>  
Gets the value in the `Ok` variant.

<a href="macro.unwrap_some.html" class="macro"
title="macro const_panic::unwrap_some">unwrap_some</a>  
Gets the value in the `Some` variant.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.PanicVal.html" class="struct"
title="struct const_panic::PanicVal">PanicVal</a>  
An opaque enum of the values that this crate knows how to format, along
with some formatting metadata.

<a href="struct.StdWrapper.html" class="struct"
title="struct const_panic::StdWrapper">StdWrapper</a>  
A wrapper type used to define methods for std types.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.MAX_PANIC_MSG_LEN.html" class="constant"
title="constant const_panic::MAX_PANIC_MSG_LEN">MAX_PANIC_MSG_LEN</a>  
The maximum length of panic messages (in bytes), after which the message
is truncated.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.concat_panic.html" class="fn"
title="fn const_panic::concat_panic">concat_panic</a>  
Panics by concatenating the argument slice.
