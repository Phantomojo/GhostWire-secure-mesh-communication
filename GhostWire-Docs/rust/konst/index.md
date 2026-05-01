Crate konst Copy item path

<span class="sub-heading"><a href="../src/konst/lib.rs.html#1-438" class="src">Source</a>
</span>

Expand description

Const equivalents of std functions and const parsing.

# <a href="#features" class="doc-anchor">§</a>Features

This crate provides:

-   Const fn equivalents of standard library functions and methods.

-   [`destructure`](./macro.destructure.html) macro to allow
    destructuring types in const without getting “cannot drop in const”
    errors.

-   Compile-time parsing through the
    [`Parser`](./parsing/struct.Parser.html) type, and
    [`parser_method`](macro.parser_method.html) macro.

# <a href="#examples" class="doc-anchor">§</a>Examples

### <a href="#parsing-an-enum" class="doc-anchor">§</a>Parsing an enum

This example demonstrates how you can parse a simple enum from an
environment variable, at compile-time.

``` {.rust .rust-example-rendered}
use konst::{
    eq_str,
    option,
    result::unwrap_ctx,
};

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

impl Direction {
    const fn try_parse(input: &str) -> Result<Self, ParseDirectionError> {
        // As of Rust 1.65.0, string patterns don't work in const contexts
        match () {
            _ if eq_str(input, "forward") => Ok(Direction::Forward),
            _ if eq_str(input, "backward") => Ok(Direction::Backward),
            _ if eq_str(input, "left") => Ok(Direction::Left),
            _ if eq_str(input, "right") => Ok(Direction::Right),
            _ => Err(ParseDirectionError),
        }
    }
}

const CHOICE: &str = option::unwrap_or!(option_env!("chosen-direction"), "forward");

const DIRECTION: Direction = unwrap_ctx!(Direction::try_parse(CHOICE));

fn main() {
    match DIRECTION {
        Direction::Forward => assert_eq!(CHOICE, "forward"),
        Direction::Backward => assert_eq!(CHOICE, "backward"),
        Direction::Left => assert_eq!(CHOICE, "left"),
        Direction::Right => assert_eq!(CHOICE, "right"),
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseDirectionError;

use std::fmt::{self, Display};

impl Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Failed to parse a Direction")
    }
}

impl ParseDirectionError {
    const fn panic(&self) -> ! {
        panic!("failed to parse a Direction")
    }
}
```

### <a href="#parsing-csv" class="doc-anchor">§</a>Parsing CSV

This example demonstrates how CSV can be parsed into integers.

This example requires the `"parsing"` and `"iter"` features (both are
enabled by default).

``` {.rust .rust-example-rendered}
use konst::{
    primitive::parse_u64,
    result::unwrap_ctx,
    iter, string,
};

const CSV: &str = "3, 8, 13, 21, 34";

static PARSED: [u64; 5] = iter::collect_const!(u64 =>
    string::split(CSV, ","),
        map(string::trim),
        map(|s| unwrap_ctx!(parse_u64(s))),
);

assert_eq!(PARSED, [3, 8, 13, 21, 34]);
```

### <a href="#parsing-a-struct" class="doc-anchor">§</a>Parsing a struct

This example demonstrates how a key-value pair format can be parsed into
a struct.

This requires the `"parsing_proc"` feature (enabled by default).

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use konst::{
    parsing::{Parser, ParseValueResult},
    eq_str,
    for_range, parser_method, try_, unwrap_ctx,
};

const PARSED: Struct = {
    // You can also parse strings from environment variables, or from an `include_str!(....)`
    let input = "\
        colors = red, blue, green, blue
        amount = 1000
        repeating = circle
        name = bob smith
    ";
     
    unwrap_ctx!(parse_struct(Parser::new(input))).0
};

fn main(){
    assert_eq!(
        PARSED,
        Struct{
            name: "bob smith",
            amount: 1000,
            repeating: Shape::Circle,
            colors: [Color::Red, Color::Blue, Color::Green, Color::Blue],
        }
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct<'a> {
    pub name: &'a str,
    pub amount: usize,
    pub repeating: Shape,
    pub colors: [Color; 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shape {
    Circle,
    Square,
    Line,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

pub const fn parse_struct(mut parser: Parser<'_>) -> ParseValueResult<'_, Struct<'_>> {
    let mut name = "<none>";
    let mut amount = 0;
    let mut repeating = Shape::Circle;
    let mut colors = [Color::Red; 4];
     
    parser = parser.trim_end();
    if !parser.is_empty() {
        loop {
            let mut prev_parser = parser.trim_start();

            parser = try_!(parser.find_skip('='));

            parser_method!{prev_parser, strip_prefix;
                "name" => (name, parser) = try_!(parser.trim_start().split_keep('\n')),
                "amount" => (amount, parser) = try_!(parser.trim_start().parse_usize()),
                "repeating" => (repeating, parser) = try_!(parse_shape(parser.trim_start())),
                "colors" => (colors, parser) = try_!(parse_colors(parser.trim_start())),
                _ => {
                    let err = &"could not parse Struct field name";
                    return Err(prev_parser.into_other_error(err));
                }
            }

            if parser.is_empty() {
                break
            }
            parser = try_!(parser.strip_prefix("\n"));
        }
    }

    Ok((Struct{name, amount, repeating, colors}, parser))
}

pub const fn parse_shape(mut parser: Parser<'_>) -> ParseValueResult<'_, Shape> {
    let shape = parser_method!{parser, strip_prefix;
        "circle" => Shape::Circle,
        "square" => Shape::Square,
        "line" => Shape::Line,
        _ => return Err(parser.into_other_error(&"could not parse Shape"))
    };
    Ok((shape, parser))
}

pub const fn parse_colors<const LEN: usize>(
    mut parser: Parser<'_>,
) -> ParseValueResult<'_, [Color; LEN]> {
    let mut colors = [Color::Red; LEN];

    for_range!{i in 0..LEN =>
        (colors[i], parser) = try_!(parse_color(parser.trim_start()));
         
        match parser.strip_prefix(",") {
            Ok(next) => parser = next,
            Err(_) if i == LEN - 1 => {}
            Err(e) => return Err(e),
        }
    }

    Ok((colors, parser))
}

pub const fn parse_color(mut parser: Parser<'_>) -> ParseValueResult<'_, Color> {
    let color = parser_method!{parser, strip_prefix;
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
        _ => return Err(parser.into_other_error(&"could not parse Color"))
    };
    Ok((color, parser))
}

```

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo features

These are the features of these crates:

-   `"iter"`(enabled by default): Enables all iteration items, including
    macros/functions that take/return iterators,

-   `"cmp"`(enabled by default): Enables all comparison functions and
    macros, the string equality and ordering comparison functions don’t
    require this feature.

-   `"parsing_proc"`(enabled by default): Enables the `"parsing"`
    feature, compiles the `konst_proc_macros` dependency, and enables
    the [`parser_method`](macro.parser_method.html) macro. You can use
    this feature instead of `"parsing"` if the slightly longer compile
    times aren’t a problem.

-   `"parsing"`(enabled by default): Enables the
    [`parsing`](./parsing/index.html) module (for parsing from `&str`
    and `&[u8]`), the `primitive::parse_*` functions, `try_rebind`, and
    `rebind_if_ok` macros.

-   `"alloc"`: Enables items that use types from the
    [`alloc`](https://doc.rust-lang.org/alloc/) crate, including `Vec`
    and `String`.

### <a href="#rust-release-related" class="doc-anchor">§</a>Rust release related

None of thse features are enabled by default.

-   `"rust_latest_stable"`: enables the latest `"rust_1_*"` feature.
    Only recommendable if you can update the Rust compiler every stable
    release.

-   `"rust_1_83"`: Enables const functions that take mutable references,
    `array::{from_fn_, map_}` macros, and
    [`destructure`](./macro.destructure.html) macro.

# <a href="#no-std-support" class="doc-anchor">§</a>No-std support

`konst` is `#![no_std]`, it can be used anywhere Rust can be used.

# <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

`konst` requires Rust 1.65.0.

Features that require newer versions of Rust, or the nightly compiler,
need to be explicitly enabled with crate features.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::parsing::`<a href="parsing/struct.Parser.html" class="struct"
title="struct konst::parsing::Parser"><code>Parser</code></a>`;`

`pub use crate::string::`<a href="string/fn.cmp_str.html" class="fn"
title="fn konst::string::cmp_str"><code>cmp_str</code></a>`;`

`pub use crate::string::`<a href="string/fn.eq_str.html" class="fn"
title="fn konst::string::eq_str"><code>eq_str</code></a>`;`

`pub use crate::result::`<a href="result/macro.unwrap_ctx.html" class="macro"
title="macro konst::result::unwrap_ctx"><code>unwrap_ctx</code></a>`;`

`pub use crate::string::`<a href="string/fn.cmp_option_str.html" class="fn"
title="fn konst::string::cmp_option_str"><code>cmp_option_str</code></a>`;`

`pub use crate::string::`<a href="string/fn.eq_option_str.html" class="fn"
title="fn konst::string::eq_option_str"><code>eq_option_str</code></a>`;`

`pub use ::`<a href="../const_panic/index.html" class="mod"
title="mod const_panic"><code>const_panic</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="array/index.html" class="mod"
title="mod konst::array">array</a>  
Const equivalents of array functions.

<a href="chr/index.html" class="mod" title="mod konst::chr">chr</a>  
Const equivalents of `char` functions.

<a href="cmp/index.html" class="mod" title="mod konst::cmp">cmp</a>  
Comparisong-related items.

<a href="ffi/index.html" class="mod" title="mod konst::ffi">ffi</a>  
const equivalents of `core::ffi` functions

<a href="iter/index.html" class="mod" title="mod konst::iter">iter</a>  
Const equivalent of iterators with a specific `next` function signature.

<a href="manually_drop/index.html" class="mod"
title="mod konst::manually_drop">manually_drop</a>  
Const fn equivalents of
[`ManuallyDrop<T>`](https://doc.rust-lang.org/1.93.1/core/mem/manually_drop/struct.ManuallyDrop.html "struct core::mem::manually_drop::ManuallyDrop")
methods.

<a href="maybe_uninit/index.html" class="mod"
title="mod konst::maybe_uninit">maybe_uninit</a>  
Const fn equivalents of
[`MaybeUninit<T>`](https://doc.rust-lang.org/core/mem/union.MaybeUninit.html)
methods.

<a href="nonzero/index.html" class="mod"
title="mod konst::nonzero">nonzero</a>  
`const fn` equivalents of `NonZero*` methods.

<a href="option/index.html" class="mod"
title="mod konst::option">option</a>  
`const` equivalents of `Option` methods.

<a href="other/index.html" class="mod"
title="mod konst::other">other</a>  
`const fn` equivalents of methods from miscelaneous standard library
types.

<a href="parsing/index.html" class="mod"
title="mod konst::parsing">parsing</a>  
Parsing using `const fn` methods.

<a href="polymorphism/index.html" class="mod"
title="mod konst::polymorphism">polymorphism</a>  
Miscelaneous items used for emulating polymorphism without trait
methods.

<a href="primitive/index.html" class="mod"
title="mod konst::primitive">primitive</a>  
`const fn` equivalents of primitive type methods.

<a href="ptr/index.html" class="mod" title="mod konst::ptr">ptr</a>  
Const equivalents of raw pointer and
[`NonNull`](https://doc.rust-lang.org/1.93.1/core/ptr/non_null/struct.NonNull.html "struct core::ptr::non_null::NonNull")
methods.

<a href="range/index.html" class="mod"
title="mod konst::range">range</a>  
`const fn` equivalents of range methods.

<a href="result/index.html" class="mod"
title="mod konst::result">result</a>  
`const` equivalents of `Result` methods.

<a href="slice/index.html" class="mod"
title="mod konst::slice">slice</a>  
`const fn` equivalents of slice methods.

<a href="string/index.html" class="mod"
title="mod konst::string">string</a>  
`const fn` equivalents of `str` methods.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.assertc_eq.html" class="macro"
title="macro konst::assertc_eq">assertc_eq</a>  
For asserting that two values are equal.

<a href="macro.assertc_ne.html" class="macro"
title="macro konst::assertc_ne">assertc_ne</a>  
For asserting that two values are unequal.

<a href="macro.coerce_to_cmp.html" class="macro"
title="macro konst::coerce_to_cmp">coerce_to_cmp</a>  
Coerces `reference` to a type that has a `const_eq` or `const_cmp`
method.

<a href="macro.const_cmp.html" class="macro"
title="macro konst::const_cmp">const_cmp</a>  
Compares two values for ordering.

<a href="macro.const_cmp_for.html" class="macro"
title="macro konst::const_cmp_for">const_cmp_for</a>  
Compares two standard library types for ordering, that can’t be compared
with [`const_cmp`](macro.const_cmp.html).

<a href="macro.const_eq.html" class="macro"
title="macro konst::const_eq">const_eq</a>  
Compares two values for equality.

<a href="macro.const_eq_for.html" class="macro"
title="macro konst::const_eq_for">const_eq_for</a>  
Compares two standard library types for equality, that can’t be compared
with [`const_eq`](macro.const_eq.html).

<a href="macro.for_range.html" class="macro"
title="macro konst::for_range">for_range</a>  
For loop over a range

<a href="macro.impl_cmp.html" class="macro"
title="macro konst::impl_cmp">impl_cmp</a>  
For implementing const comparison semi-manually.

<a href="macro.konst.html" class="macro"
title="macro konst::konst">konst</a>  
Emulates the [inline const
feature](https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html#inline-const-expressions)(`const{ ... }`)
in pre-1.79 versions.

<a href="macro.max.html" class="macro" title="macro konst::max">max</a>  
Const equivalent of
[`std::cmp::max`](https://doc.rust-lang.org/1.93.1/core/cmp/fn.max.html "fn core::cmp::max")

<a href="macro.max_by.html" class="macro"
title="macro konst::max_by">max_by</a>  
Const equivalent of
[`std::cmp::max_by`](https://doc.rust-lang.org/1.93.1/core/cmp/fn.max_by.html "fn core::cmp::max_by")

<a href="macro.max_by_key.html" class="macro"
title="macro konst::max_by_key">max_by_key</a>  
Const equivalent of
[`std::cmp::max_by_key`](https://doc.rust-lang.org/1.93.1/core/cmp/fn.max_by_key.html "fn core::cmp::max_by_key")

<a href="macro.min.html" class="macro" title="macro konst::min">min</a>  
Const equivalent of
[`std::cmp::min`](https://doc.rust-lang.org/1.93.1/core/cmp/fn.min.html "fn core::cmp::min")

<a href="macro.min_by.html" class="macro"
title="macro konst::min_by">min_by</a>  
Const equivalent of
[`std::cmp::min_by`](https://doc.rust-lang.org/1.93.1/core/cmp/fn.min_by.html "fn core::cmp::min_by")

<a href="macro.min_by_key.html" class="macro"
title="macro konst::min_by_key">min_by_key</a>  
Const equivalent of
[`std::cmp::min_by_key`](https://doc.rust-lang.org/1.93.1/core/cmp/fn.min_by_key.html "fn core::cmp::min_by_key")

<a href="macro.parse_with.html" class="macro"
title="macro konst::parse_with">parse_with</a>  
Parses a type that impls [`HasParser`](./parsing/trait.HasParser.html)
with the passed in [`Parser`](./parsing/struct.Parser.html).

<a href="macro.rebind_if_ok.html" class="macro"
title="macro konst::rebind_if_ok">rebind_if_ok</a>  
Like an `if let Ok`, but also reassigns variables with the value in the
`Ok` variant.

<a href="macro.try_.html" class="macro"
title="macro konst::try_">try_</a>  
`?`-like macro, which allows optionally mapping errors.

<a href="macro.try_equal.html" class="macro"
title="macro konst::try_equal">try_equal</a>  
Evaluates to `$ord` if it is `Ordering::Equal`, otherwise returns it
from the enclosing function.

<a href="macro.try_opt.html" class="macro"
title="macro konst::try_opt">try_opt</a>  
`?`-like macro for `Option`s.

<a href="macro.try_rebind.html" class="macro"
title="macro konst::try_rebind">try_rebind</a>  
Like the `?` operator, but also reassigns variables with the value in
the `Ok` variant.
