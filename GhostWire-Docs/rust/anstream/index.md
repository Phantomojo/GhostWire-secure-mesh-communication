Crate anstream Copy item path

<span class="sub-heading"><a href="../src/anstream/lib.rs.html#1-90" class="src">Source</a>
</span>

Expand description

**Auto-adapting [`stdout`](fn.stdout.html "fn anstream::stdout") /
[`stderr`](fn.stderr.html "fn anstream::stderr") streams**

*A portmanteau of “ansi stream”*

[`AutoStream`](struct.AutoStream.html "struct anstream::AutoStream")
always accepts [ANSI escape
codes](https://en.wikipedia.org/wiki/ANSI_escape_code), [adapting to the
user’s terminal’s
capabilities](struct.AutoStream.html "struct anstream::AutoStream").

Benefits

-   Allows the caller to not be concerned with the terminal’s
    capabilities
-   Semver safe way of passing styled text between crates as ANSI escape
    codes offer more compatibility than most crate APIs.

Available styling crates:

-   [anstyle](https://docs.rs/anstyle) for minimal runtime styling,
    designed to go in public APIs
-   [owo-colors](https://docs.rs/owo-colors) for feature-rich runtime
    styling
-   [color-print](https://docs.rs/color-print) for feature-rich
    compile-time styling

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use anstream::println;
use owo_colors::OwoColorize as _;

// Foreground colors
println!("My number is {:#x}!", 10.green());
// Background colors
println!("My number is not {}!", 4.on_red());
```

And this will correctly handle piping to a file, etc

# Modules<a href="#modules" class="anchor">§</a>

<a href="adapter/index.html" class="mod"
title="mod anstream::adapter">adapter</a>  
Gracefully degrade styled output

<a href="stream/index.html" class="mod"
title="mod anstream::stream">stream</a>  
Higher-level traits to describe writeable streams

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.eprint.html" class="macro"
title="macro anstream::eprint">eprint</a>  
Prints to [`stderr`](fn.stderr.html "fn anstream::stderr").

<a href="macro.eprintln.html" class="macro"
title="macro anstream::eprintln">eprintln</a>  
Prints to [`stderr`](fn.stderr.html "fn anstream::stderr"), with a
newline.

<a href="macro.panic.html" class="macro"
title="macro anstream::panic">panic</a>  
Panics the current thread.

<a href="macro.print.html" class="macro"
title="macro anstream::print">print</a>  
Prints to [`stdout`](fn.stdout.html "fn anstream::stdout").

<a href="macro.println.html" class="macro"
title="macro anstream::println">println</a>  
Prints to [`stdout`](fn.stdout.html "fn anstream::stdout"), with a
newline.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AutoStream.html" class="struct"
title="struct anstream::AutoStream">AutoStream</a>  
[`std::io::Write`](https://doc.rust-lang.org/1.93.1/std/io/trait.Write.html "trait std::io::Write")
that adapts ANSI escape codes to the underlying `Write`s capabilities

<a href="struct.StripStream.html" class="struct"
title="struct anstream::StripStream">StripStream</a>  
Only pass printable data to the inner `Write`

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ColorChoice.html" class="enum"
title="enum anstream::ColorChoice">ColorChoice</a>  
Selection for overriding color output Selection for overriding color
output

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.stderr.html" class="fn"
title="fn anstream::stderr">stderr</a>  
Create an ANSI escape code compatible stderr

<a href="fn.stdout.html" class="fn"
title="fn anstream::stdout">stdout</a>  
Create an ANSI escape code compatible stdout

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Stderr.html" class="type"
title="type anstream::Stderr">Stderr</a>  
An adaptive wrapper around the global standard error stream of the
current process

<a href="type.Stdout.html" class="type"
title="type anstream::Stdout">Stdout</a>  
An adaptive wrapper around the global standard output stream of the
current process
