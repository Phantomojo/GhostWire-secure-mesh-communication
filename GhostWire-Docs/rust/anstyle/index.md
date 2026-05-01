Crate anstyle Copy item path

<span class="sub-heading"><a href="../src/anstyle/lib.rs.html#1-70" class="src">Source</a>
</span>

Expand description

ANSI Text Styling

*A portmanteau of “ansi style”*

`anstyle` provides core types describing [ANSI styling escape
codes](https://en.wikipedia.org/wiki/ANSI_escape_code) for
interoperability between crates.

Example use cases:

-   An argument parser allowing callers to define the colors used in the
    help-output without putting the text formatting crate in the public
    API
-   A style description parser that can work with any text formatting
    crate

Priorities:

1.  API stability
2.  Low compile-time and binary-size overhead
3.  `const` friendly API for callers to statically define their
    stylesheet

For integration with text styling crate, see:

-   [anstyle-ansi-term](https://docs.rs/anstyle-ansi-term)
-   [anstyle-crossterm](https://docs.rs/anstyle-crossterm)
-   [anstyle-owo-colors](https://docs.rs/anstyle-owo-colors)
-   [anstyle-termcolor](https://docs.rs/anstyle-termcolor)
-   [anstyle-yansi](https://docs.rs/anstyle-yansi)

User-styling parsers:

-   [anstyle-git](https://docs.rs/anstyle-git): Parse Git style
    descriptions
-   [anstyle-ls](https://docs.rs/anstyle-ls): Parse `LS_COLORS` style
    descriptions

Convert to other formats

-   [anstream](https://docs.rs/anstream): A simple cross platform
    library for writing colored text to a terminal
-   [anstyle-roff](https://docs.rs/anstyle-roff): For converting to ROFF
-   [anstyle-syntect](https://docs.rs/anstyle-syntect): For working with
    syntax highlighting

Utilities

-   [anstyle-lossy](https://docs.rs/anstyle-lossy): Convert between
    `anstyle::Color` types
-   [anstyle-parse](https://docs.rs/anstyle-parse): Parsing ANSI Style
    Escapes
-   [anstyle-wincon](https://docs.rs/anstyle-wincon): Styling legacy
    Microsoft terminals

# <a href="#examples" class="doc-anchor">§</a>Examples

The core type is [`Style`](struct.Style.html "struct anstyle::Style"):

``` {.rust .rust-example-rendered}
let style = anstyle::Style::new().bold();
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Ansi256Color.html" class="struct"
title="struct anstyle::Ansi256Color">Ansi256Color</a>  
256 (8-bit) color support

<a href="struct.EffectIter.html" class="struct"
title="struct anstyle::EffectIter">EffectIter</a>  
Enumerate each enabled value in
[`Effects`](struct.Effects.html "struct anstyle::Effects")

<a href="struct.Effects.html" class="struct"
title="struct anstyle::Effects">Effects</a>  
A set of text effects

<a href="struct.Reset.html" class="struct"
title="struct anstyle::Reset">Reset</a>  
Reset terminal formatting

<a href="struct.RgbColor.html" class="struct"
title="struct anstyle::RgbColor">RgbColor</a>  
24-bit ANSI RGB color codes

<a href="struct.Style.html" class="struct"
title="struct anstyle::Style">Style</a>  
ANSI Text styling

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AnsiColor.html" class="enum"
title="enum anstyle::AnsiColor">AnsiColor</a>  
Available 4-bit ANSI color palette codes

<a href="enum.Color.html" class="enum"
title="enum anstyle::Color">Color</a>  
Any ANSI color code scheme
