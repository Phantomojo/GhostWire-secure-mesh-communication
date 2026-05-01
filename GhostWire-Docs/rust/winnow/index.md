Crate winnow Copy item path

<span class="sub-heading"><a href="../src/winnow/lib.rs.html#1-168" class="src">Source</a>
</span>

Expand description

> winnow, making parsing a breeze

`winnow` is a parser combinator library

Quick links:

-   [List of
    combinators](combinator/index.html "mod winnow::combinator")
-   \[Tutorial\]\[\_tutorial::chapter\_0\]
-   \[Special Topics\]\[\_topic\]
-   [Discussions](https://github.com/winnow-rs/winnow/discussions)
-   [CHANGELOG](https://github.com/winnow-rs/winnow/blob/v0.7.14/CHANGELOG.md)
    (includes major version migration guides)

## <a href="#aspirations" class="doc-anchor">§</a>Aspirations

`winnow` aims to be your “do everything” parser, much like people treat
regular expressions.

In roughly priority order:

1.  Support writing parser declaratively while not getting in the way of
    imperative-style parsing when needed, working as an open-ended
    toolbox rather than a close-ended framework.
2.  Flexible enough to be used for any application, including parsing
    strings, binary data, or separate \[lexing and parsing
    phases\]\[\_topic::lexing\]
3.  Zero-cost abstractions, making it easy to write high performance
    parsers
4.  Easy to use, making it trivial for one-off uses

In addition:

-   Resilient maintainership, including
    -   Willing to break compatibility rather than batching up breaking
        changes in large releases
    -   Leverage feature flags to keep one active branch
-   We will support the last 6 months of rust releases (MSRV, currently
    1.64.0)

See also \[Special Topic: Why winnow?\]\[crate::\_topic::why\]

## <a href="#example" class="doc-anchor">§</a>Example

Run

``` language-console
$ cargo add winnow
```

Then use it to parse:

``` {.rust .rust-example-rendered}
use winnow::combinator::seq;
use winnow::prelude::*;
use winnow::token::take_while;
use winnow::Result;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl std::str::FromStr for Color {
    // The error must be owned
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex_color.parse(s).map_err(|e| e.to_string())
    }
}

pub(crate) fn hex_color(input: &mut &str) -> Result<Color> {
    seq!(Color {
        _: '#',
        red: hex_primary,
        green: hex_primary,
        blue: hex_primary
    })
    .parse_next(input)
}

fn hex_primary(input: &mut &str) -> Result<u8> {
    take_while(2, |c: char| c.is_ascii_hexdigit())
        .try_map(|input| u8::from_str_radix(input, 16))
        .parse_next(input)
}
```

See also the \[Tutorial\]\[\_tutorial::chapter\_0\] and \[Special
Topics\]\[\_topic\]

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use error::`<a href="error/type.ModalResult.html" class="type"
title="type winnow::error::ModalResult"><code>ModalResult</code></a>`;`

`pub use error::`<a href="error/type.Result.html" class="type"
title="type winnow::error::Result"><code>Result</code></a>`;`

`pub use stream::`<a href="stream/struct.BStr.html" class="struct"
title="struct winnow::stream::BStr"><code>BStr</code></a>`;`

`pub use stream::`<a href="stream/struct.Bytes.html" class="struct"
title="struct winnow::stream::Bytes"><code>Bytes</code></a>`;`

`pub use stream::`<a href="stream/struct.LocatingSlice.html" class="struct"
title="struct winnow::stream::LocatingSlice"><code>LocatingSlice</code></a>`;`

`pub use stream::`<a href="stream/struct.Partial.html" class="struct"
title="struct winnow::stream::Partial"><code>Partial</code></a>`;`

`pub use stream::`<a href="stream/struct.Stateful.html" class="struct"
title="struct winnow::stream::Stateful"><code>Stateful</code></a>`;`

`pub use stream::`<a href="stream/type.Str.html" class="type"
title="type winnow::stream::Str"><code>Str</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="ascii/index.html" class="mod"
title="mod winnow::ascii">ascii</a>  
Character specific parsers and combinators

<a href="binary/index.html" class="mod"
title="mod winnow::binary">binary</a>  
Parsers recognizing numbers

<a href="combinator/index.html" class="mod"
title="mod winnow::combinator">combinator</a>  
List of parsers and combinators

<a href="error/index.html" class="mod"
title="mod winnow::error">error</a>  
Error management

<a href="prelude/index.html" class="mod"
title="mod winnow::prelude">prelude</a>  
Core concepts available for glob import

<a href="stream/index.html" class="mod"
title="mod winnow::stream">stream</a>  
Stream capability for combinators to parse

<a href="token/index.html" class="mod"
title="mod winnow::token">token</a>  
Parsers extracting tokens from the stream

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ModalParser.html" class="trait"
title="trait winnow::ModalParser">ModalParser</a>  
Trait alias for [`Parser`](trait.Parser.html "trait winnow::Parser") to
be used with
[`ModalResult`](error/type.ModalResult.html "type winnow::error::ModalResult")

<a href="trait.Parser.html" class="trait"
title="trait winnow::Parser">Parser</a>  
Core trait for parsing
