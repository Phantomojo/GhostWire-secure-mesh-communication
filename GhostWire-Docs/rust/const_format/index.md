Crate const\_format Copy item path

<span class="sub-heading"><a href="../src/const_format/lib.rs.html#1-509" class="src">Source</a>
</span>

Expand description

Compile-time string formatting.

This crate provides types and macros for formatting strings at
compile-time.

# <a href="#rust-versions" class="doc-anchor">§</a>Rust versions

There are some features that require a variety of Rust versions, the
sections below describe the features that are available for each
version.

### <a href="#rust-1600" class="doc-anchor">§</a>Rust 1.60.0

These macros are available in Rust 1.60.0:

-   [`concatcp`](./macro.concatcp.html): Concatenates `integers`,
    `bool`, `char`, and `&str` constants into a `&'static str` constant.

-   [`formatcp`](./macro.formatcp.html):
    [`format`](https://doc.rust-lang.org/std/macro.format.html)-like
    formatting which takes `integers`, `bool`, `char`, and `&str`
    constants, and emits a `&'static str` constant.

-   [`str_get`](./macro.str_get.html): Indexes a `&'static str`
    constant, returning `None` when the index is out of bounds.

-   [`str_index`](./macro.str_index.html): Indexes a `&'static str`
    constant.

-   [`str_repeat`](./macro.str_repeat.html): Creates a `&'static str` by
    repeating a `&'static str` constant `times` times.

-   [`str_splice`](./macro.str_splice.html): Replaces a substring in a
    `&'static str` constant.

-   [`map_ascii_case`](./macro.map_ascii_case.html): Converts a
    `&'static str` constant to a different casing style, determined by a
    [`Case`](./enum.Case.html) argument.

-   [`str_replace`](./macro.str_replace.html): Replaces all the
    instances of a pattern in a `&'static str` constant with another
    `&'static str` constant.

The `"assertcp"` feature enables the
[`assertcp`](./macro.assertcp.html),
[`assertcp_eq`](./macro.assertcp_eq.html), and
[`assertcp_ne`](./macro.assertcp_ne.html) macros. These macros are like
the standard library assert macros, but evaluated at compile-time, with
the limitation that they can only have primitive types as arguments
(just like [`concatcp`](./macro.concatcp.html) and
[`formatcp`](./macro.formatcp.html)).

### <a href="#rust-1640" class="doc-anchor">§</a>Rust 1.64.0

The `"rust_1_64"` feature enables these macros:

-   [`str_split`](./macro.str_split.html): splits a string constant

### <a href="#rust-1830" class="doc-anchor">§</a>Rust 1.83.0

By enabling the “fmt” feature, you can use a
[`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html)-like API.

This requires Rust 1.83.0, because it uses mutable references in const
fn.

All the other features of this crate are implemented on top of the
[`const_format::fmt`](./fmt/index.html) API:

-   [`concatc`](./macro.concatc.html): Concatenates many standard
    library and user defined types into a `&'static str` constant.

-   [`formatc`](./macro.formatc.html):
    [`format`](https://doc.rust-lang.org/std/macro.format.html)-like
    macro that can format many standard library and user defined types
    into a `&'static str` constant.

-   [`writec`](./macro.writec.html):
    [`write`](https://doc.rust-lang.org/std/macro.write.html)-like macro
    that can format many standard library and user defined types into a
    type that implements
    [`WriteMarker`](./marker_traits/trait.WriteMarker.html).

The `"derive"` feature enables the
[`ConstDebug`](./derive.ConstDebug.html) macro, and the `"fmt"`
feature.  
[`ConstDebug`](./derive.ConstDebug.html) derives the
[`FormatMarker`](./marker_traits/trait.FormatMarker.html) trait, and
implements an inherent `const_debug_fmt` method for compile-time debug
formatting.

The `"assertc"` feature enables the [`assertc`](./macro.assertc.html),
[`assertc_eq`](./macro.assertc_eq.html),
[`assertc_ne`](./macro.assertc_ne.html) macros, and the `"fmt"`
feature.  
These macros are like the standard library assert macros, but evaluated
at compile-time.

# <a href="#examples" class="doc-anchor">§</a>Examples

### <a href="#concatenation-of-primitive-types" class="doc-anchor">§</a>Concatenation of primitive types

``` {.rust .rust-example-rendered}
use const_format::concatcp;

const NAME: &str = "Bob";
const FOO: &str = concatcp!(NAME, ", age ", 21u8,"!");

assert_eq!(FOO, "Bob, age 21!");
```

### <a href="#formatting-primitive-types" class="doc-anchor">§</a>Formatting primitive types

``` {.rust .rust-example-rendered}
use const_format::formatcp;

const NAME: &str = "John";

const FOO: &str = formatcp!("{NAME}, age {}!", compute_age(NAME));

assert_eq!(FOO, "John, age 24!");
```

### <a href="#formatting-custom-types" class="doc-anchor">§</a>Formatting custom types

This example demonstrates how you can use the
[`ConstDebug`](./derive.ConstDebug.html) derive macro, and then format
the type into a `&'static str` constant.

This example requires the `"derive"` feature and Rust 1.83.0, because it
uses `&mut` in a const context.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use const_format::{ConstDebug, formatc};

#[derive(ConstDebug)]
struct Message{
    ip: [Octet; 4],
    value: &'static str,
}

#[derive(ConstDebug)]
struct Octet(u8);

const MSG: Message = Message{
    ip: [Octet(127), Octet(0), Octet(0), Octet(1)],
    value: "Hello, World!",
};

const FOO: &str = formatc!("{:?}", MSG);

assert_eq!(
    FOO,
    "Message { ip: [Octet(127), Octet(0), Octet(0), Octet(1)], value: \"Hello, World!\" }"
);
```

### <a href="#formatted-const-assertions" class="doc-anchor">§</a>Formatted const assertions

This example demonstrates how you can use the
[`assertcp_ne`](./macro.assertcp_ne.html) macro to do compile-time
inequality assertions with formatted error messages.

This requires the `"assertcp"` feature.

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use const_format::assertcp_ne;

macro_rules! check_valid_pizza{
    ($user:expr, $topping:expr) => {
        assertcp_ne!(
            $topping,
            "pineapple",
            "You can't put pineapple on pizza, {}",
            $user,
        );
    }
}

check_valid_pizza!("John", "salami");
check_valid_pizza!("Dave", "sausage");
check_valid_pizza!("Bob", "pineapple");
```

This is the compiler output:

``` language-text
error[E0080]: evaluation of constant value failed
  --> src/lib.rs:178:27
   |
20 | check_valid_pizza!("Bob", "pineapple");
   |                           ^^^^^^^^^^^ the evaluated program panicked at '
assertion failed: `(left != right)`
 left: `"pineapple"`
right: `"pineapple"`
You can't put pineapple on pizza, Bob
', src/lib.rs:20:27
```

# <a href="#limitations" class="doc-anchor">§</a>Limitations

All of the macros from `const_format` have these limitations:

-   The formatting macros that expand to `&'static str`s can only use
    constants from concrete types, so while a `Type::<u8>::FOO` argument
    would be fine, `Type::<T>::FOO` would not be (`T` being a type
    parameter).

-   Integer arguments must have a type inferrable from context, [more
    details in the Integer arguments section](#integer-args).

-   They cannot be used places that take string literals. So
    `#[doc = "foobar"]` cannot be replaced with
    `#[doc = concatcp!("foo", "bar") ]`.

<span id="integer-args"></span>

### <a href="#integer-arguments" class="doc-anchor">§</a>Integer arguments

Integer arguments must have a type inferrable from context. so if you
only pass an integer literal it must have a suffix.

Example of what does compile:

``` {.rust .rust-example-rendered}
const N: u32 = 1;
assert_eq!(const_format::concatcp!(N + 1, 2 + N), "23");

assert_eq!(const_format::concatcp!(2u32, 2 + 1u8, 3u8 + 1), "234");
```

Example of what does not compile:

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
assert_eq!(const_format::concatcp!(1 + 1, 2 + 1), "23");
```

# <a href="#renaming-crate" class="doc-anchor">§</a>Renaming crate

All function-like macros from `const_format` can be used when the crate
is renamed.

The [`ConstDebug`](./derive.ConstDebug.html) derive macro has the
`#[cdeb(crate = "foo::bar")]` attribute to tell it where to find the
`const_format` crate.

Example of renaming the `const_format` crate in the Cargo.toml file:

``` language-toml
[dependencies]
cfmt = {version = "0.*", package = "const_format"}
```

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo features

-   `"fmt"`: Enables the
    [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html)-like API
    and `"rust_1_83"` feature, requires Rust 1.83.0 because it uses
    mutable references in const fn.  
    This feature includes the
    [`formatc`](./macro.formatc.html)/[`writec`](./macro.writec.html)
    formatting macros.

-   `"derive"`: requires Rust 1.83.0, implies the `"fmt"` feature,
    provides the [`ConstDebug`](./derive.ConstDebug.html) derive macro
    to format user-defined types at compile-time.  
    This implicitly uses the `syn` crate, so clean compiles take a bit
    longer than without the feature.

-   `"assertc"`: requires Rust 1.83.0, implies the `"fmt"` feature,
    enables the [`assertc`](./macro.assertc.html),
    [`assertc_eq`](./macro.assertc_eq.html), and
    [`assertc_ne`](./macro.assertc_ne.html) assertion macros.  
    This feature was previously named `"assert"`, but it was renamed to
    avoid confusion with the `"assertcp"` feature.

-   `"assertcp"`: Enables the [`assertcp`](./macro.assertcp.html),
    [`assertcp_eq`](./macro.assertcp_eq.html), and
    [`assertcp_ne`](./macro.assertcp_ne.html) assertion macros.

-   `"rust_1_64"`: Enables the [`str_split`](./macro.str_split.html)
    macro. Allows the `as_bytes_alt` methods and `slice_up_to_len_alt`
    methods to run in constant time, rather than linear time
    (proportional to the truncated part of the slice).

-   `"rust_1_83"`: Enables the `"rust_1_64"` feature and makes macros
    that evaluate to a value compatible with [inline const
    patterns](https://doc.rust-lang.org/1.83.0/unstable-book/language-features/inline-const-pat.html).

# <a href="#no-std-support" class="doc-anchor">§</a>No-std support

`const_format` is unconditionally `#![no_std]`, it can be used anywhere
Rust can be used.

# <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

`const_format` requires Rust 1.60.0.

Features that require newer versions of Rust, or the nightly compiler,
need to be explicitly enabled with cargo features.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.concatcp.html" class="macro"
title="macro const_format::concatcp">concatcp</a>  
Concatenates constants of primitive types into a `&'static str`.

<a href="macro.formatcp.html" class="macro"
title="macro const_format::formatcp">formatcp</a>  
Formats constants of primitive types into a `&'static str`

<a href="macro.map_ascii_case.html" class="macro"
title="macro const_format::map_ascii_case">map_ascii_case</a>  
Converts the casing style of a `&'static str` constant, ignoring
non-ascii unicode characters.

<a href="macro.str_get.html" class="macro"
title="macro const_format::str_get">str_get</a>  
Indexes a `&'static str` constant, returning `None` when the index is
not on a character boundary.

<a href="macro.str_index.html" class="macro"
title="macro const_format::str_index">str_index</a>  
Indexes a `&'static str` constant.

<a href="macro.str_repeat.html" class="macro"
title="macro const_format::str_repeat">str_repeat</a>  
Creates a `&'static str` by repeating a `&'static str` constant `times`
times

<a href="macro.str_replace.html" class="macro"
title="macro const_format::str_replace">str_replace</a>  
Replaces all the instances of `$pattern` in `$input` (a `&'static str`
constant) with `$replace_with` (a `&'static str` constant).

<a href="macro.str_splice.html" class="macro"
title="macro const_format::str_splice">str_splice</a>  
Replaces a substring in a `&'static str` constant. Returns both the new
resulting `&'static str`, and the replaced substring.

<a href="macro.str_splice_out.html" class="macro"
title="macro const_format::str_splice_out">str_splice_out</a>  
Alternative version of [`str_splice`](./macro.str_splice.html) which
only returns the string with the applied replacement.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.SplicedStr.html" class="struct"
title="struct const_format::SplicedStr">SplicedStr</a>  
The return value of [`str_splice`](./macro.str_splice.html)

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Case.html" class="enum"
title="enum const_format::Case">Case</a>  
The casing style of a string.
